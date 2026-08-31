# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/ssooidc`
- model: `aws-models/sso-oidc.json`
- package: `aws-sdk-ssooidc`
- library: `aws_sdk_ssooidc`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_create_token`: `CreateToken`
- `op_create_token_with_iam`: `CreateTokenWithIAM`
- `op_register_client`: `RegisterClient`
- `op_start_device_authorization`: `StartDeviceAuthorization`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/client/create_token_with_iam.rs`
- `src/client/customize.rs`
- `src/client/customize/internal.rs`
- `src/client/start_device_authorization.rs`
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
- `src/observability_feature.rs`
- `src/operation.rs`
- `src/operation/create_token.rs`
- `src/operation/create_token/_create_token_input.rs`
- `src/operation/create_token/builders.rs`
- `src/operation/create_token_with_iam.rs`
- `src/operation/create_token_with_iam/_create_token_with_iam_input.rs`
- `src/operation/create_token_with_iam/_create_token_with_iam_output.rs`
- `src/operation/create_token_with_iam/builders.rs`
- `src/operation/register_client.rs`
- `src/operation/register_client/_register_client_input.rs`
- `src/operation/register_client/builders.rs`
- `src/operation/start_device_authorization.rs`
- `src/operation/start_device_authorization/_start_device_authorization_input.rs`
- `src/operation/start_device_authorization/builders.rs`
- `src/primitives/sealed_enum_unknown.rs`
- `src/protocol_serde.rs`
- `src/protocol_serde/shape_access_denied_exception.rs`
- `src/protocol_serde/shape_authorization_pending_exception.rs`
- `src/protocol_serde/shape_aws_additional_details.rs`
- `src/protocol_serde/shape_create_token.rs`
- `src/protocol_serde/shape_create_token_with_iam.rs`
- `src/protocol_serde/shape_expired_token_exception.rs`
- `src/protocol_serde/shape_internal_server_exception.rs`
- `src/protocol_serde/shape_invalid_client_exception.rs`
- `src/protocol_serde/shape_invalid_client_metadata_exception.rs`
- `src/protocol_serde/shape_invalid_grant_exception.rs`
- `src/protocol_serde/shape_invalid_redirect_uri_exception.rs`
- `src/protocol_serde/shape_invalid_request_exception.rs`
- `src/protocol_serde/shape_invalid_request_region_exception.rs`
- `src/protocol_serde/shape_invalid_scope_exception.rs`
- `src/protocol_serde/shape_register_client.rs`
- `src/protocol_serde/shape_scopes.rs`
- `src/protocol_serde/shape_slow_down_exception.rs`
- `src/protocol_serde/shape_start_device_authorization.rs`
- `src/protocol_serde/shape_unauthorized_client_exception.rs`
- `src/protocol_serde/shape_unsupported_grant_type_exception.rs`
- `src/sdk_feature_tracker.rs`
- `src/serialization_settings.rs`
- `src/types.rs`
- `src/types/_access_denied_exception_reason.rs`
- `src/types/_invalid_request_exception_reason.rs`
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
+op_create_token = []
+op_create_token_with_iam = []
+op_register_client = []
+op_start_device_authorization = []
+
+[lib]
+name = "aws_sdk_ssooidc"
```

### `src/client.rs`

```diff
diff --git a/src/client.rs b/src/client.rs
--- a/src/client.rs
+++ b/src/client.rs
@@ -90,8 +90,10 @@
     }
 }

+#[cfg(feature = "op_create_token")]
 mod create_token;

+#[cfg(feature = "op_create_token_with_iam")]
 mod create_token_with_iam;

 /// Operation customization and supporting types.
@@ -121,6 +123,8 @@
 /// ```
 pub mod customize;

+#[cfg(feature = "op_register_client")]
 mod register_client;

+#[cfg(feature = "op_start_device_authorization")]
 mod start_device_authorization;
```

### `src/client/create_token_with_iam.rs`

```diff
diff --git a/src/client/create_token_with_iam.rs b/src/client/create_token_with_iam.rs
--- a/src/client/create_token_with_iam.rs
+++ b/src/client/create_token_with_iam.rs
@@ -24,7 +24,9 @@
     ///   - [`scope(Option<Vec::<String>>)`](crate::operation::create_token_with_iam::CreateTokenWithIamOutput::scope): <p>The list of scopes for which authorization is granted. The access token that is issued is limited to the scopes that are granted.</p>
     ///   - [`aws_additional_details(Option<AwsAdditionalDetails>)`](crate::operation::create_token_with_iam::CreateTokenWithIamOutput::aws_additional_details): <p>A structure containing information from IAM Identity Center managed user and group information.</p>
     /// - On failure, responds with [`SdkError<CreateTokenWithIAMError>`](crate::operation::create_token_with_iam::CreateTokenWithIAMError)
-    pub fn create_token_with_iam(&self) -> crate::operation::create_token_with_iam::builders::CreateTokenWithIAMFluentBuilder {
+    pub fn create_token_with_iam(
+        &self,
+    ) -> crate::operation::create_token_with_iam::builders::CreateTokenWithIAMFluentBuilder {
         crate::operation::create_token_with_iam::builders::CreateTokenWithIAMFluentBuilder::new(self.handle.clone())
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

### `src/client/start_device_authorization.rs`

```diff
diff --git a/src/client/start_device_authorization.rs b/src/client/start_device_authorization.rs
--- a/src/client/start_device_authorization.rs
+++ b/src/client/start_device_authorization.rs
@@ -14,7 +14,11 @@
     ///   - [`expires_in(i32)`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput::expires_in): <p>Indicates the number of seconds in which the verification code will become invalid.</p>
     ///   - [`interval(i32)`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput::interval): <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
     /// - On failure, responds with [`SdkError<StartDeviceAuthorizationError>`](crate::operation::start_device_authorization::StartDeviceAuthorizationError)
-    pub fn start_device_authorization(&self) -> crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder {
-        crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder::new(self.handle.clone())
+    pub fn start_device_authorization(
+        &self,
+    ) -> crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder {
+        crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder::new(
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
     /// let config = aws_sdk_ssooidc::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_ssooidc::Client::from_conf(config);
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
     /// let client = aws_sdk_ssooidc::Client::from_conf(config);
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
@@ -98,7 +101,8 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut =
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

         _fut
     }
@@ -133,7 +137,10 @@
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
@@ -143,7 +150,9 @@
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
-        let endpoint = endpoint.expect("Expected valid endpoint: https://oidc-fips.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://oidc-fips.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -744,7 +747,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -762,7 +767,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -779,7 +786,10 @@
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
@@ -804,7 +814,9 @@
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
@@ -815,7 +827,10 @@
 /// Endpoint resolver trait specific to AWS SSO OIDC
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -840,7 +855,9 @@
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
@@ -884,7 +901,8 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError>
+    {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -900,22 +918,36 @@
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
@@ -931,9 +963,13 @@
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
@@ -951,7 +987,10 @@
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
@@ -967,12 +1006,18 @@
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
@@ -988,12 +1033,18 @@
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
@@ -1009,19 +1060,23 @@
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
@@ -1070,7 +1125,11 @@
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
@@ -1078,18 +1137,24 @@
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
@@ -1222,7 +1287,9 @@
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
@@ -1353,7 +1420,9 @@
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
@@ -3,62 +3,156 @@
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
 pub enum Error {
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>You do not have sufficient access to perform this action.</p>
     AccessDeniedException(crate::types::error::AccessDeniedException),
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>Indicates that a request to authorize a client with an access user session token is pending.</p>
     AuthorizationPendingException(crate::types::error::AuthorizationPendingException),
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>Indicates that the token issued by the service is expired and is no longer valid.</p>
     ExpiredTokenException(crate::types::error::ExpiredTokenException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that an error from the service occurred while trying to process a request.</p>
     InternalServerException(crate::types::error::InternalServerException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that the <code>clientId</code> or <code>clientSecret</code> in the request is invalid. For example, this can occur when a client sends an incorrect <code>clientId</code> or an expired <code>clientSecret</code>.</p>
     InvalidClientException(crate::types::error::InvalidClientException),
+    #[cfg(feature = "op_register_client")]
     /// <p>Indicates that the client information sent in the request during registration is invalid.</p>
     InvalidClientMetadataException(crate::types::error::InvalidClientMetadataException),
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>Indicates that a request contains an invalid grant. This can occur if a client makes a <code>CreateToken</code> request with an invalid grant type.</p>
     InvalidGrantException(crate::types::error::InvalidGrantException),
+    #[cfg(feature = "op_register_client")]
     /// <p>Indicates that one or more redirect URI in the request is not supported for this operation.</p>
     InvalidRedirectUriException(crate::types::error::InvalidRedirectUriException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter might be missing or out of range.</p>
     InvalidRequestException(crate::types::error::InvalidRequestException),
+    #[cfg(feature = "op_create_token_with_iam")]
     /// <p>Indicates that a token provided as input to the request was issued by and is only usable by calling IAM Identity Center endpoints in another region.</p>
     InvalidRequestRegionException(crate::types::error::InvalidRequestRegionException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client"
+    ))]
     /// <p>Indicates that the scope provided in the request is invalid.</p>
     InvalidScopeException(crate::types::error::InvalidScopeException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that the client is making the request too frequently and is more than the service can handle.</p>
     SlowDownException(crate::types::error::SlowDownException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that the client is not currently authorized to make the request. This can happen when a <code>clientId</code> is not issued for a public client.</p>
     UnauthorizedClientException(crate::types::error::UnauthorizedClientException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client"
+    ))]
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
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
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AccessDeniedException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AuthorizationPendingException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::ExpiredTokenException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InternalServerException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidClientException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidClientMetadataException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::InvalidGrantException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidRedirectUriException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidRequestException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_create_token_with_iam")]
             Error::InvalidRequestRegionException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::InvalidScopeException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::SlowDownException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::UnauthorizedClientException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::UnsupportedGrantTypeException(inner) => inner.fmt(f),
             Error::Unhandled(_) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -78,29 +172,78 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
     fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
         match self {
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AccessDeniedException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AuthorizationPendingException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::ExpiredTokenException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InternalServerException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidClientException(inner) => inner.meta(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidClientMetadataException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::InvalidGrantException(inner) => inner.meta(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidRedirectUriException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidRequestException(inner) => inner.meta(),
+            #[cfg(feature = "op_create_token_with_iam")]
             Self::InvalidRequestRegionException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::InvalidScopeException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::SlowDownException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::UnauthorizedClientException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::UnsupportedGrantTypeException(inner) => inner.meta(),
             Self::Unhandled(inner) => &inner.meta,
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>> for Error
+#[cfg(feature = "op_create_token")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -110,29 +253,64 @@
         }
     }
 }
+#[cfg(feature = "op_create_token")]
 impl From<crate::operation::create_token::CreateTokenError> for Error {
     fn from(err: crate::operation::create_token::CreateTokenError) -> Self {
         match err {
-            crate::operation::create_token::CreateTokenError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
-            crate::operation::create_token::CreateTokenError::AuthorizationPendingException(inner) => Error::AuthorizationPendingException(inner),
-            crate::operation::create_token::CreateTokenError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::create_token::CreateTokenError::InternalServerException(inner) => Error::InternalServerException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidClientException(inner) => Error::InvalidClientException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidGrantException(inner) => Error::InvalidGrantException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidScopeException(inner) => Error::InvalidScopeException(inner),
-            crate::operation::create_token::CreateTokenError::SlowDownException(inner) => Error::SlowDownException(inner),
-            crate::operation::create_token::CreateTokenError::UnauthorizedClientException(inner) => Error::UnauthorizedClientException(inner),
-            crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException(inner) => Error::UnsupportedGrantTypeException(inner),
+            crate::operation::create_token::CreateTokenError::AccessDeniedException(inner) => {
+                Error::AccessDeniedException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::AuthorizationPendingException(inner) => {
+                Error::AuthorizationPendingException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InternalServerException(inner) => {
+                Error::InternalServerException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidClientException(inner) => {
+                Error::InvalidClientException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidGrantException(inner) => {
+                Error::InvalidGrantException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidScopeException(inner) => {
+                Error::InvalidScopeException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::SlowDownException(inner) => {
+                Error::SlowDownException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::UnauthorizedClientException(inner) => {
+                Error::UnauthorizedClientException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException(inner) => {
+                Error::UnsupportedGrantTypeException(inner)
+            }
             crate::operation::create_token::CreateTokenError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token_with_iam::CreateTokenWithIAMError, R>> for Error
+#[cfg(feature = "op_create_token_with_iam")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token_with_iam::CreateTokenWithIAMError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -142,23 +320,40 @@
         }
     }
 }
+#[cfg(feature = "op_create_token_with_iam")]
 impl From<crate::operation::create_token_with_iam::CreateTokenWithIAMError> for Error {
     fn from(err: crate::operation::create_token_with_iam::CreateTokenWithIAMError) -> Self {
         match err {
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException(inner) => {
+                Error::AccessDeniedException(inner)
+            }
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::AuthorizationPendingException(inner) => {
                 Error::AuthorizationPendingException(inner)
             }
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException(inner) => Error::InternalServerException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException(inner) => Error::InvalidClientException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException(inner) => Error::InvalidGrantException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException(inner) => {
+                Error::InternalServerException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException(inner) => {
+                Error::InvalidClientException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException(inner) => {
+                Error::InvalidGrantException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestRegionException(inner) => {
                 Error::InvalidRequestRegionException(inner)
             }
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException(inner) => Error::InvalidScopeException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::SlowDownException(inner) => Error::SlowDownException(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException(inner) => {
+                Error::InvalidScopeException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::SlowDownException(inner) => {
+                Error::SlowDownException(inner)
+            }
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnauthorizedClientException(inner) => {
                 Error::UnauthorizedClientException(inner)
             }
@@ -165,15 +360,25 @@
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnsupportedGrantTypeException(inner) => {
                 Error::UnsupportedGrantTypeException(inner)
             }
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_client::RegisterClientError, R>> for Error
+#[cfg(feature = "op_register_client")]
+impl<R>
+    From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_client::RegisterClientError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_client::RegisterClientError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::register_client::RegisterClientError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -183,17 +388,28 @@
         }
     }
 }
+#[cfg(feature = "op_register_client")]
 impl From<crate::operation::register_client::RegisterClientError> for Error {
     fn from(err: crate::operation::register_client::RegisterClientError) -> Self {
         match err {
-            crate::operation::register_client::RegisterClientError::InternalServerException(inner) => Error::InternalServerException(inner),
+            crate::operation::register_client::RegisterClientError::InternalServerException(inner) => {
+                Error::InternalServerException(inner)
+            }
             crate::operation::register_client::RegisterClientError::InvalidClientMetadataException(inner) => {
                 Error::InvalidClientMetadataException(inner)
             }
-            crate::operation::register_client::RegisterClientError::InvalidRedirectUriException(inner) => Error::InvalidRedirectUriException(inner),
-            crate::operation::register_client::RegisterClientError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::register_client::RegisterClientError::InvalidScopeException(inner) => Error::InvalidScopeException(inner),
-            crate::operation::register_client::RegisterClientError::SlowDownException(inner) => Error::SlowDownException(inner),
+            crate::operation::register_client::RegisterClientError::InvalidRedirectUriException(inner) => {
+                Error::InvalidRedirectUriException(inner)
+            }
+            crate::operation::register_client::RegisterClientError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::register_client::RegisterClientError::InvalidScopeException(inner) => {
+                Error::InvalidScopeException(inner)
+            }
+            crate::operation::register_client::RegisterClientError::SlowDownException(inner) => {
+                Error::SlowDownException(inner)
+            }
             crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException(inner) => {
                 Error::UnsupportedGrantTypeException(inner)
             }
@@ -201,13 +417,22 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_device_authorization::StartDeviceAuthorizationError, R>>
-    for Error
+#[cfg(feature = "op_start_device_authorization")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_device_authorization::StartDeviceAuthorizationError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -218,6 +443,7 @@
         }
     }
 }
+#[cfg(feature = "op_start_device_authorization")]
 impl From<crate::operation::start_device_authorization::StartDeviceAuthorizationError> for Error {
     fn from(err: crate::operation::start_device_authorization::StartDeviceAuthorizationError) -> Self {
         match err {
@@ -241,19 +467,64 @@
 impl ::std::error::Error for Error {
     fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AccessDeniedException(inner) => inner.source(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AuthorizationPendingException(inner) => inner.source(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::ExpiredTokenException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InternalServerException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidClientException(inner) => inner.source(),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidClientMetadataException(inner) => inner.source(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::InvalidGrantException(inner) => inner.source(),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidRedirectUriException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidRequestException(inner) => inner.source(),
+            #[cfg(feature = "op_create_token_with_iam")]
             Error::InvalidRequestRegionException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::InvalidScopeException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::SlowDownException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::UnauthorizedClientException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::UnsupportedGrantTypeException(inner) => inner.source(),
             Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
         }
@@ -262,19 +533,64 @@
 impl ::aws_types::request_id::RequestId for Error {
     fn request_id(&self) -> Option<&str> {
         match self {
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AccessDeniedException(e) => e.request_id(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AuthorizationPendingException(e) => e.request_id(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::ExpiredTokenException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InternalServerException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidClientException(e) => e.request_id(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidClientMetadataException(e) => e.request_id(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::InvalidGrantException(e) => e.request_id(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidRedirectUriException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidRequestException(e) => e.request_id(),
+            #[cfg(feature = "op_create_token_with_iam")]
             Self::InvalidRequestRegionException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::InvalidScopeException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::SlowDownException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::UnauthorizedClientException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::UnsupportedGrantTypeException(e) => e.request_id(),
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

+#[cfg(feature = "op_create_token")]
 /// Types for the `CreateToken` operation.
 pub mod create_token;

+#[cfg(feature = "op_create_token_with_iam")]
 /// Types for the `CreateTokenWithIAM` operation.
 pub mod create_token_with_iam;

+#[cfg(feature = "op_register_client")]
 /// Types for the `RegisterClient` operation.
 pub mod register_client;

+#[cfg(feature = "op_start_device_authorization")]
 /// Types for the `StartDeviceAuthorization` operation.
 pub mod start_device_authorization;
```

### `src/operation/create_token.rs`

```diff
diff --git a/src/operation/create_token.rs b/src/operation/create_token.rs
--- a/src/operation/create_token.rs
+++ b/src/operation/create_token.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "CreateToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.CreateToken",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "CreateToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "CreateToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.CreateToken",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "CreateToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             CreateTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(CreateTokenResponseDeserializer),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("CreateToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("CreateToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateToken", "SSO OIDC"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "CreateToken",
+            "SSO OIDC",
+        ));

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,25 +128,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateToken")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "CreateToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 CreateTokenTelemetryInputCaptureInterceptor,
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
                 CreateTokenEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::create_token::CreateTokenError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_token::CreateTokenError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_token::CreateTokenError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::create_token::CreateTokenError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -225,12 +252,20 @@
 #[derive(Debug)]
 struct CreateTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateTokenRequestSerializer {
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
             .downcast::<crate::operation::create_token::CreateTokenInput>()
             .expect("correct type");
@@ -252,19 +287,30 @@
             fn update_http_builder(
                 input: &crate::operation::create_token::CreateTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_token::ser_create_token_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_create_token::ser_create_token_input(&input)?,
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
@@ -297,13 +343,21 @@
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
@@ -338,18 +392,22 @@
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl CreateTokenError {
     /// Creates the `CreateTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -371,7 +429,9 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::AccessDeniedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::AuthorizationPendingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::AuthorizationPendingException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
@@ -380,7 +440,9 @@
             Self::InvalidScopeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::SlowDownException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::UnauthorizedClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnsupportedGrantTypeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::UnsupportedGrantTypeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -462,7 +524,9 @@
             Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
             Self::UnsupportedGrantTypeException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -482,17 +546,37 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::AccessDeniedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::AuthorizationPendingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidGrantException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidScopeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::AccessDeniedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::AuthorizationPendingException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidGrantException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidScopeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnsupportedGrantTypeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnauthorizedClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnsupportedGrantTypeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/create_token/_create_token_input.rs`

```diff
diff --git a/src/operation/create_token/_create_token_input.rs b/src/operation/create_token/_create_token_input.rs
--- a/src/operation/create_token/_create_token_input.rs
+++ b/src/operation/create_token/_create_token_input.rs
@@ -255,7 +255,12 @@
         &self.code_verifier
     }
     /// Consumes the builder and constructs a [`CreateTokenInput`](crate::operation::create_token::CreateTokenInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::create_token::CreateTokenInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::create_token::CreateTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::create_token::CreateTokenInput {
             client_id: self.client_id,
             client_secret: self.client_secret,
```

### `src/operation/create_token/builders.rs`

```diff
diff --git a/src/operation/create_token/builders.rs b/src/operation/create_token/builders.rs
--- a/src/operation/create_token/builders.rs
+++ b/src/operation/create_token/builders.rs
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

### `src/operation/create_token_with_iam.rs`

```diff
diff --git a/src/operation/create_token_with_iam.rs b/src/operation/create_token_with_iam.rs
--- a/src/operation/create_token_with_iam.rs
+++ b/src/operation/create_token_with_iam.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "CreateTokenWithIAM", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.CreateTokenWithIAM",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "CreateTokenWithIAM",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "CreateTokenWithIAM",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.CreateTokenWithIAM",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "CreateTokenWithIAM",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             CreateTokenWithIAMRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateTokenWithIAMResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                CreateTokenWithIAMResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("CreateTokenWithIAM")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("CreateTokenWithIAM")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -126,25 +140,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateTokenWithIAM")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "CreateTokenWithIAM",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 CreateTokenWithIAMTelemetryInputCaptureInterceptor,
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
                 CreateTokenWithIAMEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::create_token_with_iam::CreateTokenWithIAMError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_token_with_iam::CreateTokenWithIAMError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_token_with_iam::CreateTokenWithIAMError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -233,9 +259,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_error(status, headers, body)
+            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_response(status, headers, body)
+            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -243,12 +273,20 @@
 #[derive(Debug)]
 struct CreateTokenWithIAMRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateTokenWithIAMRequestSerializer {
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
             .downcast::<crate::operation::create_token_with_iam::CreateTokenWithIamInput>()
             .expect("correct type");
@@ -278,7 +316,8 @@
             fn update_http_builder(
                 input: &crate::operation::create_token_with_iam::CreateTokenWithIamInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -285,15 +324,23 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_token_with_iam::ser_create_token_with_iam_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_create_token_with_iam::ser_create_token_with_iam_input(&input)?,
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
@@ -326,13 +373,21 @@
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
@@ -369,18 +424,22 @@
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenWithIAMError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenWithIAMError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl CreateTokenWithIAMError {
     /// Creates the `CreateTokenWithIAMError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -402,17 +461,23 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::AccessDeniedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::AuthorizationPendingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::AuthorizationPendingException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidGrantException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidRequestRegionException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestRegionException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::InvalidScopeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::SlowDownException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::UnauthorizedClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnsupportedGrantTypeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::UnsupportedGrantTypeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -500,7 +565,9 @@
             Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
             Self::UnsupportedGrantTypeException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -520,18 +587,40 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateTokenWithIAMError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::AccessDeniedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::AuthorizationPendingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidGrantException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestRegionException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidScopeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::AccessDeniedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::AuthorizationPendingException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidGrantException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestRegionException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidScopeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnsupportedGrantTypeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnauthorizedClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnsupportedGrantTypeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/create_token_with_iam/_create_token_with_iam_input.rs`

```diff
diff --git a/src/operation/create_token_with_iam/_create_token_with_iam_input.rs b/src/operation/create_token_with_iam/_create_token_with_iam_input.rs
--- a/src/operation/create_token_with_iam/_create_token_with_iam_input.rs
+++ b/src/operation/create_token_with_iam/_create_token_with_iam_input.rs
@@ -320,8 +320,10 @@
     /// Consumes the builder and constructs a [`CreateTokenWithIamInput`](crate::operation::create_token_with_iam::CreateTokenWithIamInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::create_token_with_iam::CreateTokenWithIamInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::create_token_with_iam::CreateTokenWithIamInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::create_token_with_iam::CreateTokenWithIamInput {
             client_id: self.client_id,
             grant_type: self.grant_type,
```

### `src/operation/create_token_with_iam/_create_token_with_iam_output.rs`

```diff
diff --git a/src/operation/create_token_with_iam/_create_token_with_iam_output.rs b/src/operation/create_token_with_iam/_create_token_with_iam_output.rs
--- a/src/operation/create_token_with_iam/_create_token_with_iam_output.rs
+++ b/src/operation/create_token_with_iam/_create_token_with_iam_output.rs
@@ -224,7 +224,10 @@
         self
     }
     /// <p>A structure containing information from IAM Identity Center managed user and group information.</p>
-    pub fn set_aws_additional_details(mut self, input: ::std::option::Option<crate::types::AwsAdditionalDetails>) -> Self {
+    pub fn set_aws_additional_details(
+        mut self,
+        input: ::std::option::Option<crate::types::AwsAdditionalDetails>,
+    ) -> Self {
         self.aws_additional_details = input;
         self
     }
```

### `src/operation/create_token_with_iam/builders.rs`

```diff
diff --git a/src/operation/create_token_with_iam/builders.rs b/src/operation/create_token_with_iam/builders.rs
--- a/src/operation/create_token_with_iam/builders.rs
+++ b/src/operation/create_token_with_iam/builders.rs
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

### `src/operation/register_client.rs`

```diff
diff --git a/src/operation/register_client.rs b/src/operation/register_client.rs
--- a/src/operation/register_client.rs
+++ b/src/operation/register_client.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "RegisterClient", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.RegisterClient",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "RegisterClient",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "RegisterClient",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.RegisterClient",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "RegisterClient",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             RegisterClientRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            RegisterClientResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                RegisterClientResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("RegisterClient")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("RegisterClient")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -116,25 +130,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RegisterClient")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "RegisterClient",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 RegisterClientTelemetryInputCaptureInterceptor,
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
                 RegisterClientEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::register_client::RegisterClientError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::register_client::RegisterClientError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::register_client::RegisterClientError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::register_client::RegisterClientError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -223,12 +249,20 @@
 #[derive(Debug)]
 struct RegisterClientRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for RegisterClientRequestSerializer {
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
             .downcast::<crate::operation::register_client::RegisterClientInput>()
             .expect("correct type");
@@ -250,19 +284,30 @@
             fn update_http_builder(
                 input: &crate::operation::register_client::RegisterClientInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_register_client::ser_register_client_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_register_client::ser_register_client_input(&input)?,
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
@@ -295,13 +340,21 @@
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
@@ -328,18 +381,22 @@
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RegisterClientError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RegisterClientError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl RegisterClientError {
     /// Creates the `RegisterClientError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -361,12 +418,16 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidClientMetadataException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidClientMetadataException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::InvalidRedirectUriException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidScopeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::SlowDownException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnsupportedGrantTypeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::UnsupportedGrantTypeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -424,7 +485,9 @@
             Self::SlowDownException(_inner) => _inner.fmt(f),
             Self::UnsupportedGrantTypeException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -444,13 +507,25 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RegisterClientError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientMetadataException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRedirectUriException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidScopeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientMetadataException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRedirectUriException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidScopeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnsupportedGrantTypeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnsupportedGrantTypeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/register_client/_register_client_input.rs`

```diff
diff --git a/src/operation/register_client/_register_client_input.rs b/src/operation/register_client/_register_client_input.rs
--- a/src/operation/register_client/_register_client_input.rs
+++ b/src/operation/register_client/_register_client_input.rs
@@ -210,7 +210,10 @@
     /// Consumes the builder and constructs a [`RegisterClientInput`](crate::operation::register_client::RegisterClientInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::register_client::RegisterClientInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::register_client::RegisterClientInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::register_client::RegisterClientInput {
             client_name: self.client_name,
             client_type: self.client_type,
```

### `src/operation/register_client/builders.rs`

```diff
diff --git a/src/operation/register_client/builders.rs b/src/operation/register_client/builders.rs
--- a/src/operation/register_client/builders.rs
+++ b/src/operation/register_client/builders.rs
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

### `src/operation/start_device_authorization.rs`

```diff
diff --git a/src/operation/start_device_authorization.rs b/src/operation/start_device_authorization.rs
--- a/src/operation/start_device_authorization.rs
+++ b/src/operation/start_device_authorization.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "StartDeviceAuthorization", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.StartDeviceAuthorization",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "StartDeviceAuthorization",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "StartDeviceAuthorization",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.StartDeviceAuthorization",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "StartDeviceAuthorization",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             StartDeviceAuthorizationRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            StartDeviceAuthorizationResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                StartDeviceAuthorizationResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("StartDeviceAuthorization")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("StartDeviceAuthorization")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "StartDeviceAuthorization",
@@ -115,25 +129,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StartDeviceAuthorization")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "StartDeviceAuthorization",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 StartDeviceAuthorizationTelemetryInputCaptureInterceptor,
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
                 StartDeviceAuthorizationEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::start_device_authorization::StartDeviceAuthorizationError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::start_device_authorization::StartDeviceAuthorizationError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::start_device_authorization::StartDeviceAuthorizationError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -143,7 +169,9 @@
 struct StartDeviceAuthorizationTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for StartDeviceAuthorizationTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for StartDeviceAuthorizationTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "StartDeviceAuthorizationTelemetryInputCaptureInterceptor"
     }
@@ -202,9 +230,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_error(status, headers, body)
+            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_response(status, headers, body)
+            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -212,12 +244,20 @@
 #[derive(Debug)]
 struct StartDeviceAuthorizationRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for StartDeviceAuthorizationRequestSerializer {
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
             .downcast::<crate::operation::start_device_authorization::StartDeviceAuthorizationInput>()
             .expect("correct type");
@@ -239,13 +279,18 @@
             fn update_http_builder(
                 input: &crate::operation::start_device_authorization::StartDeviceAuthorizationInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -253,7 +298,11 @@
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
@@ -286,13 +335,21 @@
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
     /// <p>Indicates that the client is not currently authorized to make the request. This can happen when a <code>clientId</code> is not issued for a public client.</p>
     UnauthorizedClientException(crate::types::error::UnauthorizedClientException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-StartDeviceAuthorizationError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-StartDeviceAuthorizationError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl StartDeviceAuthorizationError {
     /// Creates the `StartDeviceAuthorizationError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -397,7 +458,9 @@
             Self::SlowDownException(_inner) => _inner.fmt(f),
             Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -417,11 +480,19 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartDeviceAuthorizationError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnauthorizedClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -437,7 +508,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::start_device_authorization::StartDeviceAuthorizationError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::start_device_authorization::StartDeviceAuthorizationError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/start_device_authorization/_start_device_authorization_input.rs`

```diff
diff --git a/src/operation/start_device_authorization/_start_device_authorization_input.rs b/src/operation/start_device_authorization/_start_device_authorization_input.rs
--- a/src/operation/start_device_authorization/_start_device_authorization_input.rs
+++ b/src/operation/start_device_authorization/_start_device_authorization_input.rs
@@ -101,11 +101,13 @@
         crate::operation::start_device_authorization::StartDeviceAuthorizationInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::start_device_authorization::StartDeviceAuthorizationInput {
-            client_id: self.client_id,
-            client_secret: self.client_secret,
-            start_url: self.start_url,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::start_device_authorization::StartDeviceAuthorizationInput {
+                client_id: self.client_id,
+                client_secret: self.client_secret,
+                start_url: self.start_url,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for StartDeviceAuthorizationInputBuilder {
```

### `src/operation/start_device_authorization/builders.rs`

```diff
diff --git a/src/operation/start_device_authorization/builders.rs b/src/operation/start_device_authorization/builders.rs
--- a/src/operation/start_device_authorization/builders.rs
+++ b/src/operation/start_device_authorization/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the StartDeviceAuthorization as a reference.
-    pub fn as_input(&self) -> &crate::operation::start_device_authorization::builders::StartDeviceAuthorizationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::start_device_authorization::builders::StartDeviceAuthorizationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::start_device_authorization::StartDeviceAuthorization::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::start_device_authorization::StartDeviceAuthorization::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::start_device_authorization::StartDeviceAuthorization::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::start_device_authorization::StartDeviceAuthorization::orchestrate(&runtime_plugins, input)
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

### `src/primitives/sealed_enum_unknown.rs`

```diff
diff --git a/src/primitives/sealed_enum_unknown.rs b/src/primitives/sealed_enum_unknown.rs
--- a/src/primitives/sealed_enum_unknown.rs
+++ b/src/primitives/sealed_enum_unknown.rs
@@ -6,7 +6,13 @@
 /// This is not intended to be used directly.
 #[non_exhaustive]
 #[derive(
-    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
+    ::std::clone::Clone,
+    ::std::cmp::Eq,
+    ::std::cmp::Ord,
+    ::std::cmp::PartialEq,
+    ::std::cmp::PartialOrd,
+    ::std::fmt::Debug,
+    ::std::hash::Hash,
 )]
 pub struct UnknownVariantValue(pub(crate) ::std::string::String);
 impl UnknownVariantValue {
```

### `src/protocol_serde.rs`

```diff
diff --git a/src/protocol_serde.rs b/src/protocol_serde.rs
--- a/src/protocol_serde.rs
+++ b/src/protocol_serde.rs
@@ -1,9 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
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
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
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

+#[cfg(feature = "op_create_token")]
 pub(crate) mod shape_create_token;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_create_token_with_iam;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_register_client;

+#[cfg(feature = "op_start_device_authorization")]
 pub(crate) mod shape_start_device_authorization;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -39,42 +66,93 @@
     }
 }

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_access_denied_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_authorization_pending_exception;

+#[cfg(feature = "op_create_token")]
 pub(crate) mod shape_create_token_input;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_create_token_with_iam_input;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_expired_token_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_internal_server_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_invalid_client_exception;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_invalid_client_metadata_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_invalid_grant_exception;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_invalid_redirect_uri_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_invalid_request_exception;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_invalid_request_region_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub(crate) mod shape_invalid_scope_exception;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_register_client_input;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_slow_down_exception;

+#[cfg(feature = "op_start_device_authorization")]
 pub(crate) mod shape_start_device_authorization_input;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_unauthorized_client_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub(crate) mod shape_unsupported_grant_type_exception;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_aws_additional_details;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_scopes;
```

### `src/protocol_serde/shape_access_denied_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_access_denied_exception.rs b/src/protocol_serde/shape_access_denied_exception.rs
--- a/src/protocol_serde/shape_access_denied_exception.rs
+++ b/src/protocol_serde/shape_access_denied_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_access_denied_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::AccessDeniedExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::AccessDeniedExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,7 +26,10 @@
                 "reason" => {
                     builder = builder.set_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| crate::types::AccessDeniedExceptionReason::from(u.as_ref())))
+                            .map(|s| {
+                                s.to_unescaped()
+                                    .map(|u| crate::types::AccessDeniedExceptionReason::from(u.as_ref()))
+                            })
                             .transpose()?,
                     );
                 }
@@ -43,9 +50,9 @@
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

### `src/protocol_serde/shape_authorization_pending_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_authorization_pending_exception.rs b/src/protocol_serde/shape_authorization_pending_exception.rs
--- a/src/protocol_serde/shape_authorization_pending_exception.rs
+++ b/src/protocol_serde/shape_authorization_pending_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_authorization_pending_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::AuthorizationPendingExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::AuthorizationPendingExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::AuthorizationPendingExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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

### `src/protocol_serde/shape_aws_additional_details.rs`

```diff
diff --git a/src/protocol_serde/shape_aws_additional_details.rs b/src/protocol_serde/shape_aws_additional_details.rs
--- a/src/protocol_serde/shape_aws_additional_details.rs
+++ b/src/protocol_serde/shape_aws_additional_details.rs
@@ -3,9 +3,17 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::AwsAdditionalDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::AwsAdditionalDetails>,
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
@@ -20,20 +28,22 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "identityContext" => {
-                            builder = builder.set_identity_context(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "identityContext" => {
+                                builder = builder.set_identity_context(
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

### `src/protocol_serde/shape_create_token.rs`

```diff
diff --git a/src/protocol_serde/shape_create_token.rs b/src/protocol_serde/shape_create_token.rs
--- a/src/protocol_serde/shape_create_token.rs
+++ b/src/protocol_serde/shape_create_token.rs
@@ -4,10 +4,14 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::create_token::CreateTokenOutput, crate::operation::create_token::CreateTokenError> {
+) -> std::result::Result<
+    crate::operation::create_token::CreateTokenOutput,
+    crate::operation::create_token::CreateTokenError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +26,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,29 +39,34 @@
             }
             tmp
         }),
-        "AuthorizationPendingException" => crate::operation::create_token::CreateTokenError::AuthorizationPendingException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "AuthorizationPendingException" => {
+            crate::operation::create_token::CreateTokenError::AuthorizationPendingException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_authorization_pending_exception::de_authorization_pending_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
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
         "ExpiredTokenException" => crate::operation::create_token::CreateTokenError::ExpiredTokenException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +80,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +98,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -98,8 +116,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidGrantExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +134,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +152,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -143,8 +170,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -153,40 +183,44 @@
             }
             tmp
         }),
-        "UnauthorizedClientException" => crate::operation::create_token::CreateTokenError::UnauthorizedClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnauthorizedClientException" => {
+            crate::operation::create_token::CreateTokenError::UnauthorizedClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnsupportedGrantTypeException" => crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException({
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
+        "UnsupportedGrantTypeException" => {
+            crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
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
         _ => crate::operation::create_token::CreateTokenError::generic(generic),
     })
 }
@@ -196,7 +230,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::create_token::CreateTokenOutput, crate::operation::create_token::CreateTokenError> {
+) -> std::result::Result<
+    crate::operation::create_token::CreateTokenOutput,
+    crate::operation::create_token::CreateTokenError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::create_token::builders::CreateTokenOutputBuilder::default();
@@ -209,7 +246,8 @@

 pub fn ser_create_token_input(
     input: &crate::operation::create_token::CreateTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     crate::protocol_serde::shape_create_token_input::ser_create_token_input_input(&mut object, input)?;
@@ -220,9 +258,12 @@
 pub(crate) fn de_create_token(
     _value: &[u8],
     mut builder: crate::operation::create_token::builders::CreateTokenOutputBuilder,
-) -> ::std::result::Result<crate::operation::create_token::builders::CreateTokenOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::operation::create_token::builders::CreateTokenOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -269,9 +310,9 @@
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

### `src/protocol_serde/shape_create_token_with_iam.rs`

```diff
diff --git a/src/protocol_serde/shape_create_token_with_iam.rs b/src/protocol_serde/shape_create_token_with_iam.rs
--- a/src/protocol_serde/shape_create_token_with_iam.rs
+++ b/src/protocol_serde/shape_create_token_with_iam.rs
@@ -9,8 +9,9 @@
     crate::operation::create_token_with_iam::CreateTokenWithIAMError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,152 +21,197 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AccessDeniedException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "AccessDeniedException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "AuthorizationPendingException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::AuthorizationPendingException({
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
+        "AuthorizationPendingException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AuthorizationPendingException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_authorization_pending_exception::de_authorization_pending_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ExpiredTokenException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException({
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
+        "ExpiredTokenException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InternalServerException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException({
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
+        "InternalServerException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidClientException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidClientException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidGrantException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidGrantException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidGrantExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidGrantExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRequestException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException({
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
+        "InvalidRequestException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRequestRegionException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestRegionException({
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
+                        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidRequestRegionException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestRegionException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestRegionExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_region_exception::de_invalid_request_region_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRequestRegionExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_region_exception::de_invalid_request_region_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidScopeException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException({
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
+        "InvalidScopeException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
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
         "SlowDownException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::SlowDownException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -174,40 +220,44 @@
             }
             tmp
         }),
-        "UnauthorizedClientException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnauthorizedClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnauthorizedClientException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnauthorizedClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnsupportedGrantTypeException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnsupportedGrantTypeException({
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
+        "UnsupportedGrantTypeException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnsupportedGrantTypeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
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
         _ => crate::operation::create_token_with_iam::CreateTokenWithIAMError::generic(generic),
     })
 }
@@ -233,10 +283,14 @@

 pub fn ser_create_token_with_iam_input(
     input: &crate::operation::create_token_with_iam::CreateTokenWithIamInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    crate::protocol_serde::shape_create_token_with_iam_input::ser_create_token_with_iam_input_input(&mut object, input)?;
+    crate::protocol_serde::shape_create_token_with_iam_input::ser_create_token_with_iam_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
@@ -248,7 +302,8 @@
     crate::operation::create_token_with_iam::builders::CreateTokenWithIamOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -265,11 +320,13 @@
                     );
                 }
                 "awsAdditionalDetails" => {
-                    builder = builder.set_aws_additional_details(crate::protocol_serde::shape_aws_additional_details::de_aws_additional_details(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                    builder = builder.set_aws_additional_details(
+                        crate::protocol_serde::shape_aws_additional_details::de_aws_additional_details(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
                 }
                 "expiresIn" => {
                     builder = builder.set_expires_in(
@@ -300,7 +357,11 @@
                     );
                 }
                 "scope" => {
-                    builder = builder.set_scope(crate::protocol_serde::shape_scopes::de_scopes(tokens, _value, depth + 1)?);
+                    builder = builder.set_scope(crate::protocol_serde::shape_scopes::de_scopes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "tokenType" => {
                     builder = builder.set_token_type(
@@ -312,9 +373,9 @@
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

### `src/protocol_serde/shape_expired_token_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_expired_token_exception.rs b/src/protocol_serde/shape_expired_token_exception.rs
--- a/src/protocol_serde/shape_expired_token_exception.rs
+++ b/src/protocol_serde/shape_expired_token_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_expired_token_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::ExpiredTokenExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::ExpiredTokenExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::ExpiredTokenExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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

### `src/protocol_serde/shape_internal_server_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_internal_server_exception.rs b/src/protocol_serde/shape_internal_server_exception.rs
--- a/src/protocol_serde/shape_internal_server_exception.rs
+++ b/src/protocol_serde/shape_internal_server_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_internal_server_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InternalServerExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InternalServerExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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

### `src/protocol_serde/shape_invalid_client_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_client_exception.rs b/src/protocol_serde/shape_invalid_client_exception.rs
--- a/src/protocol_serde/shape_invalid_client_exception.rs
+++ b/src/protocol_serde/shape_invalid_client_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_client_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidClientExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidClientExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidClientExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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

### `src/protocol_serde/shape_invalid_client_metadata_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_client_metadata_exception.rs b/src/protocol_serde/shape_invalid_client_metadata_exception.rs
--- a/src/protocol_serde/shape_invalid_client_metadata_exception.rs
+++ b/src/protocol_serde/shape_invalid_client_metadata_exception.rs
@@ -6,7 +6,8 @@
     crate::types::error::builders::InvalidClientMetadataExceptionBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -39,9 +40,9 @@
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

### `src/protocol_serde/shape_invalid_grant_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_grant_exception.rs b/src/protocol_serde/shape_invalid_grant_exception.rs
--- a/src/protocol_serde/shape_invalid_grant_exception.rs
+++ b/src/protocol_serde/shape_invalid_grant_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_grant_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidGrantExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidGrantExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidGrantExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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

### `src/protocol_serde/shape_invalid_redirect_uri_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_redirect_uri_exception.rs b/src/protocol_serde/shape_invalid_redirect_uri_exception.rs
--- a/src/protocol_serde/shape_invalid_redirect_uri_exception.rs
+++ b/src/protocol_serde/shape_invalid_redirect_uri_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_invalid_redirect_uri_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRedirectUriExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRedirectUriExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRedirectUriExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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
@@ -22,7 +26,10 @@
                 "reason" => {
                     builder = builder.set_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| crate::types::InvalidRequestExceptionReason::from(u.as_ref())))
+                            .map(|s| {
+                                s.to_unescaped()
+                                    .map(|u| crate::types::InvalidRequestExceptionReason::from(u.as_ref()))
+                            })
                             .transpose()?,
                     );
                 }
@@ -43,9 +50,9 @@
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

### `src/protocol_serde/shape_invalid_request_region_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_request_region_exception.rs b/src/protocol_serde/shape_invalid_request_region_exception.rs
--- a/src/protocol_serde/shape_invalid_request_region_exception.rs
+++ b/src/protocol_serde/shape_invalid_request_region_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_invalid_request_region_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRequestRegionExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRequestRegionExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRequestRegionExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -51,9 +54,9 @@
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

### `src/protocol_serde/shape_invalid_scope_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_scope_exception.rs b/src/protocol_serde/shape_invalid_scope_exception.rs
--- a/src/protocol_serde/shape_invalid_scope_exception.rs
+++ b/src/protocol_serde/shape_invalid_scope_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_scope_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidScopeExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidScopeExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidScopeExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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

### `src/protocol_serde/shape_register_client.rs`

```diff
diff --git a/src/protocol_serde/shape_register_client.rs b/src/protocol_serde/shape_register_client.rs
--- a/src/protocol_serde/shape_register_client.rs
+++ b/src/protocol_serde/shape_register_client.rs
@@ -4,15 +4,23 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::register_client::RegisterClientOutput, crate::operation::register_client::RegisterClientError> {
+) -> std::result::Result<
+    crate::operation::register_client::RegisterClientOutput,
+    crate::operation::register_client::RegisterClientError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::register_client::RegisterClientError::unhandled(generic)),
+        None => {
+            return Err(crate::operation::register_client::RegisterClientError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -22,8 +30,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,47 +43,54 @@
             }
             tmp
         }),
-        "InvalidClientMetadataException" => crate::operation::register_client::RegisterClientError::InvalidClientMetadataException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidClientMetadataException" => {
+            crate::operation::register_client::RegisterClientError::InvalidClientMetadataException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidClientMetadataExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_metadata_exception::de_invalid_client_metadata_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidClientMetadataExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_client_metadata_exception::de_invalid_client_metadata_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRedirectUriException" => crate::operation::register_client::RegisterClientError::InvalidRedirectUriException({
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
+        "InvalidRedirectUriException" => {
+            crate::operation::register_client::RegisterClientError::InvalidRedirectUriException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRedirectUriExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRedirectUriExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_invalid_redirect_uri_exception::de_invalid_redirect_uri_exception_json_err(_response_body, output)
                         .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
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
         "InvalidRequestException" => crate::operation::register_client::RegisterClientError::InvalidRequestException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,8 +104,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -101,8 +122,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -111,24 +135,26 @@
             }
             tmp
         }),
-        "UnsupportedGrantTypeException" => crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnsupportedGrantTypeException" => {
+            crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
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
         _ => crate::operation::register_client::RegisterClientError::generic(generic),
     })
 }
@@ -138,7 +164,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::register_client::RegisterClientOutput, crate::operation::register_client::RegisterClientError> {
+) -> std::result::Result<
+    crate::operation::register_client::RegisterClientOutput,
+    crate::operation::register_client::RegisterClientError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::register_client::builders::RegisterClientOutputBuilder::default();
@@ -151,7 +180,8 @@

 pub fn ser_register_client_input(
     input: &crate::operation::register_client::RegisterClientInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     crate::protocol_serde::shape_register_client_input::ser_register_client_input_input(&mut object, input)?;
@@ -166,7 +196,8 @@
     crate::operation::register_client::builders::RegisterClientOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -220,9 +251,9 @@
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

### `src/protocol_serde/shape_scopes.rs`

```diff
diff --git a/src/protocol_serde/shape_scopes.rs b/src/protocol_serde/shape_scopes.rs
--- a/src/protocol_serde/shape_scopes.rs
+++ b/src/protocol_serde/shape_scopes.rs
@@ -3,9 +3,17 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<::std::string::String>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<::std::string::String>>,
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

### `src/protocol_serde/shape_slow_down_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_slow_down_exception.rs b/src/protocol_serde/shape_slow_down_exception.rs
--- a/src/protocol_serde/shape_slow_down_exception.rs
+++ b/src/protocol_serde/shape_slow_down_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_slow_down_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::SlowDownExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::SlowDownExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::SlowDownExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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

### `src/protocol_serde/shape_start_device_authorization.rs`

```diff
diff --git a/src/protocol_serde/shape_start_device_authorization.rs b/src/protocol_serde/shape_start_device_authorization.rs
--- a/src/protocol_serde/shape_start_device_authorization.rs
+++ b/src/protocol_serde/shape_start_device_authorization.rs
@@ -9,97 +9,127 @@
     crate::operation::start_device_authorization::StartDeviceAuthorizationError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
         None => {
-            return Err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled(
-                generic,
-            ))
+            return Err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled(generic))
         }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalServerException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::InternalServerException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InternalServerException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::InternalServerException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidClientException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled,
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
+        "InvalidClientException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRequestException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled,
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
+        "InvalidRequestException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "SlowDownException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::SlowDownException({
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
+                        .map_err(
+                            crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled,
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
+        "SlowDownException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::SlowDownException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedClientException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::UnauthorizedClientException({
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
+        "UnauthorizedClientException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::UnauthorizedClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                         .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
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
         _ => crate::operation::start_device_authorization::StartDeviceAuthorizationError::generic(generic),
     })
 }
@@ -115,9 +145,13 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder::default();
-        output = crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization(_response_body, output)
-            .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
+        let mut output =
+            crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder::default();
+        output = crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -125,10 +159,14 @@

 pub fn ser_start_device_authorization_input(
     input: &crate::operation::start_device_authorization::StartDeviceAuthorizationInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    crate::protocol_serde::shape_start_device_authorization_input::ser_start_device_authorization_input_input(&mut object, input)?;
+    crate::protocol_serde::shape_start_device_authorization_input::ser_start_device_authorization_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
@@ -140,7 +178,8 @@
     crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -194,9 +233,9 @@
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

### `src/protocol_serde/shape_unauthorized_client_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_unauthorized_client_exception.rs b/src/protocol_serde/shape_unauthorized_client_exception.rs
--- a/src/protocol_serde/shape_unauthorized_client_exception.rs
+++ b/src/protocol_serde/shape_unauthorized_client_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_unauthorized_client_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnauthorizedClientExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnauthorizedClientExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnauthorizedClientExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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

### `src/protocol_serde/shape_unsupported_grant_type_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_unsupported_grant_type_exception.rs b/src/protocol_serde/shape_unsupported_grant_type_exception.rs
--- a/src/protocol_serde/shape_unsupported_grant_type_exception.rs
+++ b/src/protocol_serde/shape_unsupported_grant_type_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_unsupported_grant_type_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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
@@ -1,14 +1,30 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::_access_denied_exception_reason::AccessDeniedExceptionReason;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::_invalid_request_exception_reason::InvalidRequestExceptionReason;

+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::_aws_additional_details::AwsAdditionalDetails;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _access_denied_exception_reason;

+#[cfg(feature = "op_create_token_with_iam")]
 mod _aws_additional_details;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _invalid_request_exception_reason;

 /// Builders
```

### `src/types/_access_denied_exception_reason.rs`

```diff
diff --git a/src/types/_access_denied_exception_reason.rs b/src/types/_access_denied_exception_reason.rs
--- a/src/types/_access_denied_exception_reason.rs
+++ b/src/types/_access_denied_exception_reason.rs
@@ -38,13 +38,21 @@
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
-    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
+    ::std::clone::Clone,
+    ::std::cmp::Eq,
+    ::std::cmp::Ord,
+    ::std::cmp::PartialEq,
+    ::std::cmp::PartialOrd,
+    ::std::fmt::Debug,
+    ::std::hash::Hash,
 )]
 pub enum AccessDeniedExceptionReason {
     #[allow(missing_docs)] // documentation missing in model
     KmsAccessDenied,
     /// `Unknown` contains new variants that have been added since this code was generated.
-    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
+    #[deprecated(
+        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
+    )]
     Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
 }
 impl ::std::convert::From<&str> for AccessDeniedExceptionReason {
@@ -51,7 +59,9 @@
     fn from(s: &str) -> Self {
         match s {
             "KMS_AccessDeniedException" => AccessDeniedExceptionReason::KmsAccessDenied,
-            other => AccessDeniedExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => AccessDeniedExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(
+                other.to_owned(),
+            )),
         }
     }
 }
```

### `src/types/_invalid_request_exception_reason.rs`

```diff
diff --git a/src/types/_invalid_request_exception_reason.rs b/src/types/_invalid_request_exception_reason.rs
--- a/src/types/_invalid_request_exception_reason.rs
+++ b/src/types/_invalid_request_exception_reason.rs
@@ -41,7 +41,13 @@
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
-    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
+    ::std::clone::Clone,
+    ::std::cmp::Eq,
+    ::std::cmp::Ord,
+    ::std::cmp::PartialEq,
+    ::std::cmp::PartialOrd,
+    ::std::fmt::Debug,
+    ::std::hash::Hash,
 )]
 pub enum InvalidRequestExceptionReason {
     #[allow(missing_docs)] // documentation missing in model
@@ -53,7 +59,9 @@
     #[allow(missing_docs)] // documentation missing in model
     KmsKeyNotFound,
     /// `Unknown` contains new variants that have been added since this code was generated.
-    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
+    #[deprecated(
+        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
+    )]
     Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
 }
 impl ::std::convert::From<&str> for InvalidRequestExceptionReason {
@@ -63,7 +71,9 @@
             "KMS_InvalidKeyUsageException" => InvalidRequestExceptionReason::KmsInvalidKeyUsage,
             "KMS_InvalidStateException" => InvalidRequestExceptionReason::KmsInvalidState,
             "KMS_NotFoundException" => InvalidRequestExceptionReason::KmsKeyNotFound,
-            other => InvalidRequestExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => InvalidRequestExceptionReason::Unknown(
+                crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()),
+            ),
         }
     }
 }
```

### `src/types/builders.rs`

```diff
diff --git a/src/types/builders.rs b/src/types/builders.rs
--- a/src/types/builders.rs
+++ b/src/types/builders.rs
@@ -1,2 +1,3 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::_aws_additional_details::AwsAdditionalDetailsBuilder;
```

### `src/types/error.rs`

```diff
diff --git a/src/types/error.rs b/src/types/error.rs
--- a/src/types/error.rs
+++ b/src/types/error.rs
@@ -1,58 +1,148 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_access_denied_exception::AccessDeniedException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_authorization_pending_exception::AuthorizationPendingException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_internal_server_exception::InternalServerException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_client_exception::InvalidClientException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_invalid_grant_exception::InvalidGrantException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_invalid_scope_exception::InvalidScopeException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_slow_down_exception::SlowDownException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_unauthorized_client_exception::UnauthorizedClientException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_unsupported_grant_type_exception::UnsupportedGrantTypeException;

+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::error::_invalid_request_region_exception::InvalidRequestRegionException;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_client_metadata_exception::InvalidClientMetadataException;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_redirect_uri_exception::InvalidRedirectUriException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _access_denied_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _authorization_pending_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _expired_token_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _internal_server_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 mod _invalid_client_exception;

+#[cfg(feature = "op_register_client")]
 mod _invalid_client_metadata_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _invalid_grant_exception;

+#[cfg(feature = "op_register_client")]
 mod _invalid_redirect_uri_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _invalid_request_exception;

+#[cfg(feature = "op_create_token_with_iam")]
 mod _invalid_request_region_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 mod _invalid_scope_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _slow_down_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 mod _unauthorized_client_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 mod _unsupported_grant_type_exception;

 /// Builders
```

### `src/types/error/builders.rs`

```diff
diff --git a/src/types/error/builders.rs b/src/types/error/builders.rs
--- a/src/types/error/builders.rs
+++ b/src/types/error/builders.rs
@@ -1,28 +1,73 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_access_denied_exception::AccessDeniedExceptionBuilder;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_authorization_pending_exception::AuthorizationPendingExceptionBuilder;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_internal_server_exception::InternalServerExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_client_exception::InvalidClientExceptionBuilder;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_invalid_grant_exception::InvalidGrantExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_invalid_scope_exception::InvalidScopeExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_slow_down_exception::SlowDownExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_unauthorized_client_exception::UnauthorizedClientExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_unsupported_grant_type_exception::UnsupportedGrantTypeExceptionBuilder;

+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::error::_invalid_request_region_exception::InvalidRequestRegionExceptionBuilder;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_client_metadata_exception::InvalidClientMetadataExceptionBuilder;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_redirect_uri_exception::InvalidRedirectUriExceptionBuilder;
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
+op_create_token = []
+op_create_token_with_iam = []
+op_register_client = []
+op_start_device_authorization = []
+
+[lib]
+name = "aws_sdk_ssooidc"
diff --git a/src/client.rs b/src/client.rs
--- a/src/client.rs
+++ b/src/client.rs
@@ -90,8 +90,10 @@
     }
 }

+#[cfg(feature = "op_create_token")]
 mod create_token;

+#[cfg(feature = "op_create_token_with_iam")]
 mod create_token_with_iam;

 /// Operation customization and supporting types.
@@ -121,6 +123,8 @@
 /// ```
 pub mod customize;

+#[cfg(feature = "op_register_client")]
 mod register_client;

+#[cfg(feature = "op_start_device_authorization")]
 mod start_device_authorization;
diff --git a/src/client/create_token_with_iam.rs b/src/client/create_token_with_iam.rs
--- a/src/client/create_token_with_iam.rs
+++ b/src/client/create_token_with_iam.rs
@@ -24,7 +24,9 @@
     ///   - [`scope(Option<Vec::<String>>)`](crate::operation::create_token_with_iam::CreateTokenWithIamOutput::scope): <p>The list of scopes for which authorization is granted. The access token that is issued is limited to the scopes that are granted.</p>
     ///   - [`aws_additional_details(Option<AwsAdditionalDetails>)`](crate::operation::create_token_with_iam::CreateTokenWithIamOutput::aws_additional_details): <p>A structure containing information from IAM Identity Center managed user and group information.</p>
     /// - On failure, responds with [`SdkError<CreateTokenWithIAMError>`](crate::operation::create_token_with_iam::CreateTokenWithIAMError)
-    pub fn create_token_with_iam(&self) -> crate::operation::create_token_with_iam::builders::CreateTokenWithIAMFluentBuilder {
+    pub fn create_token_with_iam(
+        &self,
+    ) -> crate::operation::create_token_with_iam::builders::CreateTokenWithIAMFluentBuilder {
         crate::operation::create_token_with_iam::builders::CreateTokenWithIAMFluentBuilder::new(self.handle.clone())
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
diff --git a/src/client/start_device_authorization.rs b/src/client/start_device_authorization.rs
--- a/src/client/start_device_authorization.rs
+++ b/src/client/start_device_authorization.rs
@@ -14,7 +14,11 @@
     ///   - [`expires_in(i32)`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput::expires_in): <p>Indicates the number of seconds in which the verification code will become invalid.</p>
     ///   - [`interval(i32)`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput::interval): <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
     /// - On failure, responds with [`SdkError<StartDeviceAuthorizationError>`](crate::operation::start_device_authorization::StartDeviceAuthorizationError)
-    pub fn start_device_authorization(&self) -> crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder {
-        crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder::new(self.handle.clone())
+    pub fn start_device_authorization(
+        &self,
+    ) -> crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder {
+        crate::operation::start_device_authorization::builders::StartDeviceAuthorizationFluentBuilder::new(
+            self.handle.clone(),
+        )
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
     /// let config = aws_sdk_ssooidc::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_ssooidc::Client::from_conf(config);
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
     /// let client = aws_sdk_ssooidc::Client::from_conf(config);
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
@@ -98,7 +101,8 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut =
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

         _fut
     }
@@ -133,7 +137,10 @@
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
@@ -143,7 +150,9 @@
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
-        let endpoint = endpoint.expect("Expected valid endpoint: https://oidc-fips.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://oidc-fips.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -744,7 +747,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -762,7 +767,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -779,7 +786,10 @@
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
@@ -804,7 +814,9 @@
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
@@ -815,7 +827,10 @@
 /// Endpoint resolver trait specific to AWS SSO OIDC
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -840,7 +855,9 @@
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
@@ -884,7 +901,8 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError>
+    {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -900,22 +918,36 @@
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
@@ -931,9 +963,13 @@
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
@@ -951,7 +987,10 @@
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
@@ -967,12 +1006,18 @@
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
@@ -988,12 +1033,18 @@
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
@@ -1009,19 +1060,23 @@
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
@@ -1070,7 +1125,11 @@
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
@@ -1078,18 +1137,24 @@
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
@@ -1222,7 +1287,9 @@
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
@@ -1353,7 +1420,9 @@
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
@@ -3,62 +3,156 @@
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
 pub enum Error {
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>You do not have sufficient access to perform this action.</p>
     AccessDeniedException(crate::types::error::AccessDeniedException),
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>Indicates that a request to authorize a client with an access user session token is pending.</p>
     AuthorizationPendingException(crate::types::error::AuthorizationPendingException),
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>Indicates that the token issued by the service is expired and is no longer valid.</p>
     ExpiredTokenException(crate::types::error::ExpiredTokenException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that an error from the service occurred while trying to process a request.</p>
     InternalServerException(crate::types::error::InternalServerException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that the <code>clientId</code> or <code>clientSecret</code> in the request is invalid. For example, this can occur when a client sends an incorrect <code>clientId</code> or an expired <code>clientSecret</code>.</p>
     InvalidClientException(crate::types::error::InvalidClientException),
+    #[cfg(feature = "op_register_client")]
     /// <p>Indicates that the client information sent in the request during registration is invalid.</p>
     InvalidClientMetadataException(crate::types::error::InvalidClientMetadataException),
+    #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
     /// <p>Indicates that a request contains an invalid grant. This can occur if a client makes a <code>CreateToken</code> request with an invalid grant type.</p>
     InvalidGrantException(crate::types::error::InvalidGrantException),
+    #[cfg(feature = "op_register_client")]
     /// <p>Indicates that one or more redirect URI in the request is not supported for this operation.</p>
     InvalidRedirectUriException(crate::types::error::InvalidRedirectUriException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter might be missing or out of range.</p>
     InvalidRequestException(crate::types::error::InvalidRequestException),
+    #[cfg(feature = "op_create_token_with_iam")]
     /// <p>Indicates that a token provided as input to the request was issued by and is only usable by calling IAM Identity Center endpoints in another region.</p>
     InvalidRequestRegionException(crate::types::error::InvalidRequestRegionException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client"
+    ))]
     /// <p>Indicates that the scope provided in the request is invalid.</p>
     InvalidScopeException(crate::types::error::InvalidScopeException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that the client is making the request too frequently and is more than the service can handle.</p>
     SlowDownException(crate::types::error::SlowDownException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_start_device_authorization"
+    ))]
     /// <p>Indicates that the client is not currently authorized to make the request. This can happen when a <code>clientId</code> is not issued for a public client.</p>
     UnauthorizedClientException(crate::types::error::UnauthorizedClientException),
+    #[cfg(any(
+        feature = "op_create_token",
+        feature = "op_create_token_with_iam",
+        feature = "op_register_client"
+    ))]
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
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
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AccessDeniedException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AuthorizationPendingException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::ExpiredTokenException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InternalServerException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidClientException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidClientMetadataException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::InvalidGrantException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidRedirectUriException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidRequestException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_create_token_with_iam")]
             Error::InvalidRequestRegionException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::InvalidScopeException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::SlowDownException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::UnauthorizedClientException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::UnsupportedGrantTypeException(inner) => inner.fmt(f),
             Error::Unhandled(_) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -78,29 +172,78 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
     fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
         match self {
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AccessDeniedException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AuthorizationPendingException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::ExpiredTokenException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InternalServerException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidClientException(inner) => inner.meta(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidClientMetadataException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::InvalidGrantException(inner) => inner.meta(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidRedirectUriException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidRequestException(inner) => inner.meta(),
+            #[cfg(feature = "op_create_token_with_iam")]
             Self::InvalidRequestRegionException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::InvalidScopeException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::SlowDownException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::UnauthorizedClientException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::UnsupportedGrantTypeException(inner) => inner.meta(),
             Self::Unhandled(inner) => &inner.meta,
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>> for Error
+#[cfg(feature = "op_create_token")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token::CreateTokenError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -110,29 +253,64 @@
         }
     }
 }
+#[cfg(feature = "op_create_token")]
 impl From<crate::operation::create_token::CreateTokenError> for Error {
     fn from(err: crate::operation::create_token::CreateTokenError) -> Self {
         match err {
-            crate::operation::create_token::CreateTokenError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
-            crate::operation::create_token::CreateTokenError::AuthorizationPendingException(inner) => Error::AuthorizationPendingException(inner),
-            crate::operation::create_token::CreateTokenError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::create_token::CreateTokenError::InternalServerException(inner) => Error::InternalServerException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidClientException(inner) => Error::InvalidClientException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidGrantException(inner) => Error::InvalidGrantException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::create_token::CreateTokenError::InvalidScopeException(inner) => Error::InvalidScopeException(inner),
-            crate::operation::create_token::CreateTokenError::SlowDownException(inner) => Error::SlowDownException(inner),
-            crate::operation::create_token::CreateTokenError::UnauthorizedClientException(inner) => Error::UnauthorizedClientException(inner),
-            crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException(inner) => Error::UnsupportedGrantTypeException(inner),
+            crate::operation::create_token::CreateTokenError::AccessDeniedException(inner) => {
+                Error::AccessDeniedException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::AuthorizationPendingException(inner) => {
+                Error::AuthorizationPendingException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InternalServerException(inner) => {
+                Error::InternalServerException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidClientException(inner) => {
+                Error::InvalidClientException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidGrantException(inner) => {
+                Error::InvalidGrantException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::InvalidScopeException(inner) => {
+                Error::InvalidScopeException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::SlowDownException(inner) => {
+                Error::SlowDownException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::UnauthorizedClientException(inner) => {
+                Error::UnauthorizedClientException(inner)
+            }
+            crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException(inner) => {
+                Error::UnsupportedGrantTypeException(inner)
+            }
             crate::operation::create_token::CreateTokenError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token_with_iam::CreateTokenWithIAMError, R>> for Error
+#[cfg(feature = "op_create_token_with_iam")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_token_with_iam::CreateTokenWithIAMError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -142,23 +320,40 @@
         }
     }
 }
+#[cfg(feature = "op_create_token_with_iam")]
 impl From<crate::operation::create_token_with_iam::CreateTokenWithIAMError> for Error {
     fn from(err: crate::operation::create_token_with_iam::CreateTokenWithIAMError) -> Self {
         match err {
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException(inner) => {
+                Error::AccessDeniedException(inner)
+            }
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::AuthorizationPendingException(inner) => {
                 Error::AuthorizationPendingException(inner)
             }
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException(inner) => Error::InternalServerException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException(inner) => Error::InvalidClientException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException(inner) => Error::InvalidGrantException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException(inner) => {
+                Error::InternalServerException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException(inner) => {
+                Error::InvalidClientException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException(inner) => {
+                Error::InvalidGrantException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestRegionException(inner) => {
                 Error::InvalidRequestRegionException(inner)
             }
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException(inner) => Error::InvalidScopeException(inner),
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::SlowDownException(inner) => Error::SlowDownException(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException(inner) => {
+                Error::InvalidScopeException(inner)
+            }
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::SlowDownException(inner) => {
+                Error::SlowDownException(inner)
+            }
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnauthorizedClientException(inner) => {
                 Error::UnauthorizedClientException(inner)
             }
@@ -165,15 +360,25 @@
             crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnsupportedGrantTypeException(inner) => {
                 Error::UnsupportedGrantTypeException(inner)
             }
-            crate::operation::create_token_with_iam::CreateTokenWithIAMError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_client::RegisterClientError, R>> for Error
+#[cfg(feature = "op_register_client")]
+impl<R>
+    From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_client::RegisterClientError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_client::RegisterClientError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::register_client::RegisterClientError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -183,17 +388,28 @@
         }
     }
 }
+#[cfg(feature = "op_register_client")]
 impl From<crate::operation::register_client::RegisterClientError> for Error {
     fn from(err: crate::operation::register_client::RegisterClientError) -> Self {
         match err {
-            crate::operation::register_client::RegisterClientError::InternalServerException(inner) => Error::InternalServerException(inner),
+            crate::operation::register_client::RegisterClientError::InternalServerException(inner) => {
+                Error::InternalServerException(inner)
+            }
             crate::operation::register_client::RegisterClientError::InvalidClientMetadataException(inner) => {
                 Error::InvalidClientMetadataException(inner)
             }
-            crate::operation::register_client::RegisterClientError::InvalidRedirectUriException(inner) => Error::InvalidRedirectUriException(inner),
-            crate::operation::register_client::RegisterClientError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::register_client::RegisterClientError::InvalidScopeException(inner) => Error::InvalidScopeException(inner),
-            crate::operation::register_client::RegisterClientError::SlowDownException(inner) => Error::SlowDownException(inner),
+            crate::operation::register_client::RegisterClientError::InvalidRedirectUriException(inner) => {
+                Error::InvalidRedirectUriException(inner)
+            }
+            crate::operation::register_client::RegisterClientError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::register_client::RegisterClientError::InvalidScopeException(inner) => {
+                Error::InvalidScopeException(inner)
+            }
+            crate::operation::register_client::RegisterClientError::SlowDownException(inner) => {
+                Error::SlowDownException(inner)
+            }
             crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException(inner) => {
                 Error::UnsupportedGrantTypeException(inner)
             }
@@ -201,13 +417,22 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_device_authorization::StartDeviceAuthorizationError, R>>
-    for Error
+#[cfg(feature = "op_start_device_authorization")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_device_authorization::StartDeviceAuthorizationError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -218,6 +443,7 @@
         }
     }
 }
+#[cfg(feature = "op_start_device_authorization")]
 impl From<crate::operation::start_device_authorization::StartDeviceAuthorizationError> for Error {
     fn from(err: crate::operation::start_device_authorization::StartDeviceAuthorizationError) -> Self {
         match err {
@@ -241,19 +467,64 @@
 impl ::std::error::Error for Error {
     fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AccessDeniedException(inner) => inner.source(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::AuthorizationPendingException(inner) => inner.source(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::ExpiredTokenException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InternalServerException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidClientException(inner) => inner.source(),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidClientMetadataException(inner) => inner.source(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Error::InvalidGrantException(inner) => inner.source(),
+            #[cfg(feature = "op_register_client")]
             Error::InvalidRedirectUriException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::InvalidRequestException(inner) => inner.source(),
+            #[cfg(feature = "op_create_token_with_iam")]
             Error::InvalidRequestRegionException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::InvalidScopeException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Error::SlowDownException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Error::UnauthorizedClientException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Error::UnsupportedGrantTypeException(inner) => inner.source(),
             Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
         }
@@ -262,19 +533,64 @@
 impl ::aws_types::request_id::RequestId for Error {
     fn request_id(&self) -> Option<&str> {
         match self {
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AccessDeniedException(e) => e.request_id(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::AuthorizationPendingException(e) => e.request_id(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::ExpiredTokenException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InternalServerException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidClientException(e) => e.request_id(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidClientMetadataException(e) => e.request_id(),
+            #[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
             Self::InvalidGrantException(e) => e.request_id(),
+            #[cfg(feature = "op_register_client")]
             Self::InvalidRedirectUriException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::InvalidRequestException(e) => e.request_id(),
+            #[cfg(feature = "op_create_token_with_iam")]
             Self::InvalidRequestRegionException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::InvalidScopeException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client",
+                feature = "op_start_device_authorization"
+            ))]
             Self::SlowDownException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_start_device_authorization"
+            ))]
             Self::UnauthorizedClientException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_create_token",
+                feature = "op_create_token_with_iam",
+                feature = "op_register_client"
+            ))]
             Self::UnsupportedGrantTypeException(e) => e.request_id(),
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

+#[cfg(feature = "op_create_token")]
 /// Types for the `CreateToken` operation.
 pub mod create_token;

+#[cfg(feature = "op_create_token_with_iam")]
 /// Types for the `CreateTokenWithIAM` operation.
 pub mod create_token_with_iam;

+#[cfg(feature = "op_register_client")]
 /// Types for the `RegisterClient` operation.
 pub mod register_client;

+#[cfg(feature = "op_start_device_authorization")]
 /// Types for the `StartDeviceAuthorization` operation.
 pub mod start_device_authorization;
diff --git a/src/operation/create_token.rs b/src/operation/create_token.rs
--- a/src/operation/create_token.rs
+++ b/src/operation/create_token.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "CreateToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.CreateToken",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "CreateToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "CreateToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.CreateToken",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "CreateToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             CreateTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(CreateTokenResponseDeserializer),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("CreateToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("CreateToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateToken", "SSO OIDC"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "CreateToken",
+            "SSO OIDC",
+        ));

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,25 +128,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateToken")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "CreateToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 CreateTokenTelemetryInputCaptureInterceptor,
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
                 CreateTokenEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::create_token::CreateTokenError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_token::CreateTokenError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_token::CreateTokenError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::create_token::CreateTokenError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -225,12 +252,20 @@
 #[derive(Debug)]
 struct CreateTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateTokenRequestSerializer {
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
             .downcast::<crate::operation::create_token::CreateTokenInput>()
             .expect("correct type");
@@ -252,19 +287,30 @@
             fn update_http_builder(
                 input: &crate::operation::create_token::CreateTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_token::ser_create_token_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_create_token::ser_create_token_input(&input)?,
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
@@ -297,13 +343,21 @@
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
@@ -338,18 +392,22 @@
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl CreateTokenError {
     /// Creates the `CreateTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -371,7 +429,9 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::AccessDeniedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::AuthorizationPendingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::AuthorizationPendingException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
@@ -380,7 +440,9 @@
             Self::InvalidScopeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::SlowDownException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::UnauthorizedClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnsupportedGrantTypeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::UnsupportedGrantTypeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -462,7 +524,9 @@
             Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
             Self::UnsupportedGrantTypeException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -482,17 +546,37 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::AccessDeniedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::AuthorizationPendingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidGrantException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidScopeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::AccessDeniedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::AuthorizationPendingException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidGrantException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidScopeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnsupportedGrantTypeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnauthorizedClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnsupportedGrantTypeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/create_token/_create_token_input.rs b/src/operation/create_token/_create_token_input.rs
--- a/src/operation/create_token/_create_token_input.rs
+++ b/src/operation/create_token/_create_token_input.rs
@@ -255,7 +255,12 @@
         &self.code_verifier
     }
     /// Consumes the builder and constructs a [`CreateTokenInput`](crate::operation::create_token::CreateTokenInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::create_token::CreateTokenInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::create_token::CreateTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::create_token::CreateTokenInput {
             client_id: self.client_id,
             client_secret: self.client_secret,
diff --git a/src/operation/create_token/builders.rs b/src/operation/create_token/builders.rs
--- a/src/operation/create_token/builders.rs
+++ b/src/operation/create_token/builders.rs
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
diff --git a/src/operation/create_token_with_iam.rs b/src/operation/create_token_with_iam.rs
--- a/src/operation/create_token_with_iam.rs
+++ b/src/operation/create_token_with_iam.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "CreateTokenWithIAM", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.CreateTokenWithIAM",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "CreateTokenWithIAM",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "CreateTokenWithIAM",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.CreateTokenWithIAM",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "CreateTokenWithIAM",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             CreateTokenWithIAMRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateTokenWithIAMResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                CreateTokenWithIAMResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("CreateTokenWithIAM")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("CreateTokenWithIAM")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -126,25 +140,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateTokenWithIAM")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "CreateTokenWithIAM",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 CreateTokenWithIAMTelemetryInputCaptureInterceptor,
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
                 CreateTokenWithIAMEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::create_token_with_iam::CreateTokenWithIAMError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_token_with_iam::CreateTokenWithIAMError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_token_with_iam::CreateTokenWithIAMError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -233,9 +259,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_error(status, headers, body)
+            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_response(status, headers, body)
+            crate::protocol_serde::shape_create_token_with_iam::de_create_token_with_iam_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -243,12 +273,20 @@
 #[derive(Debug)]
 struct CreateTokenWithIAMRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateTokenWithIAMRequestSerializer {
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
             .downcast::<crate::operation::create_token_with_iam::CreateTokenWithIamInput>()
             .expect("correct type");
@@ -278,7 +316,8 @@
             fn update_http_builder(
                 input: &crate::operation::create_token_with_iam::CreateTokenWithIamInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -285,15 +324,23 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_token_with_iam::ser_create_token_with_iam_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_create_token_with_iam::ser_create_token_with_iam_input(&input)?,
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
@@ -326,13 +373,21 @@
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
@@ -369,18 +424,22 @@
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenWithIAMError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateTokenWithIAMError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl CreateTokenWithIAMError {
     /// Creates the `CreateTokenWithIAMError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -402,17 +461,23 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::AccessDeniedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::AuthorizationPendingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::AuthorizationPendingException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidGrantException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidRequestRegionException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestRegionException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::InvalidScopeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::SlowDownException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::UnauthorizedClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnsupportedGrantTypeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::UnsupportedGrantTypeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -500,7 +565,9 @@
             Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
             Self::UnsupportedGrantTypeException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -520,18 +587,40 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateTokenWithIAMError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::AccessDeniedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::AuthorizationPendingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidGrantException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestRegionException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidScopeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::AccessDeniedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::AuthorizationPendingException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidGrantException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestRegionException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidScopeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnsupportedGrantTypeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnauthorizedClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnsupportedGrantTypeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/create_token_with_iam/_create_token_with_iam_input.rs b/src/operation/create_token_with_iam/_create_token_with_iam_input.rs
--- a/src/operation/create_token_with_iam/_create_token_with_iam_input.rs
+++ b/src/operation/create_token_with_iam/_create_token_with_iam_input.rs
@@ -320,8 +320,10 @@
     /// Consumes the builder and constructs a [`CreateTokenWithIamInput`](crate::operation::create_token_with_iam::CreateTokenWithIamInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::create_token_with_iam::CreateTokenWithIamInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::create_token_with_iam::CreateTokenWithIamInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::create_token_with_iam::CreateTokenWithIamInput {
             client_id: self.client_id,
             grant_type: self.grant_type,
diff --git a/src/operation/create_token_with_iam/_create_token_with_iam_output.rs b/src/operation/create_token_with_iam/_create_token_with_iam_output.rs
--- a/src/operation/create_token_with_iam/_create_token_with_iam_output.rs
+++ b/src/operation/create_token_with_iam/_create_token_with_iam_output.rs
@@ -224,7 +224,10 @@
         self
     }
     /// <p>A structure containing information from IAM Identity Center managed user and group information.</p>
-    pub fn set_aws_additional_details(mut self, input: ::std::option::Option<crate::types::AwsAdditionalDetails>) -> Self {
+    pub fn set_aws_additional_details(
+        mut self,
+        input: ::std::option::Option<crate::types::AwsAdditionalDetails>,
+    ) -> Self {
         self.aws_additional_details = input;
         self
     }
diff --git a/src/operation/create_token_with_iam/builders.rs b/src/operation/create_token_with_iam/builders.rs
--- a/src/operation/create_token_with_iam/builders.rs
+++ b/src/operation/create_token_with_iam/builders.rs
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
diff --git a/src/operation/register_client.rs b/src/operation/register_client.rs
--- a/src/operation/register_client.rs
+++ b/src/operation/register_client.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "RegisterClient", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.RegisterClient",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "RegisterClient",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "RegisterClient",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.RegisterClient",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "RegisterClient",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             RegisterClientRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            RegisterClientResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                RegisterClientResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("RegisterClient")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("RegisterClient")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -116,25 +130,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RegisterClient")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "RegisterClient",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 RegisterClientTelemetryInputCaptureInterceptor,
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
                 RegisterClientEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::register_client::RegisterClientError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::register_client::RegisterClientError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::register_client::RegisterClientError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::register_client::RegisterClientError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -223,12 +249,20 @@
 #[derive(Debug)]
 struct RegisterClientRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for RegisterClientRequestSerializer {
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
             .downcast::<crate::operation::register_client::RegisterClientInput>()
             .expect("correct type");
@@ -250,19 +284,30 @@
             fn update_http_builder(
                 input: &crate::operation::register_client::RegisterClientInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_register_client::ser_register_client_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_register_client::ser_register_client_input(&input)?,
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
@@ -295,13 +340,21 @@
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
@@ -328,18 +381,22 @@
     /// <p>Indicates that the grant type in the request is not supported by the service.</p>
     UnsupportedGrantTypeException(crate::types::error::UnsupportedGrantTypeException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RegisterClientError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RegisterClientError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl RegisterClientError {
     /// Creates the `RegisterClientError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -361,12 +418,16 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidClientMetadataException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidClientMetadataException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::InvalidRedirectUriException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::InvalidScopeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::SlowDownException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnsupportedGrantTypeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::UnsupportedGrantTypeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -424,7 +485,9 @@
             Self::SlowDownException(_inner) => _inner.fmt(f),
             Self::UnsupportedGrantTypeException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -444,13 +507,25 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RegisterClientError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientMetadataException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRedirectUriException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidScopeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientMetadataException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRedirectUriException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidScopeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnsupportedGrantTypeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnsupportedGrantTypeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/register_client/_register_client_input.rs b/src/operation/register_client/_register_client_input.rs
--- a/src/operation/register_client/_register_client_input.rs
+++ b/src/operation/register_client/_register_client_input.rs
@@ -210,7 +210,10 @@
     /// Consumes the builder and constructs a [`RegisterClientInput`](crate::operation::register_client::RegisterClientInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::register_client::RegisterClientInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::register_client::RegisterClientInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::register_client::RegisterClientInput {
             client_name: self.client_name,
             client_type: self.client_type,
diff --git a/src/operation/register_client/builders.rs b/src/operation/register_client/builders.rs
--- a/src/operation/register_client/builders.rs
+++ b/src/operation/register_client/builders.rs
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
diff --git a/src/operation/start_device_authorization.rs b/src/operation/start_device_authorization.rs
--- a/src/operation/start_device_authorization.rs
+++ b/src/operation/start_device_authorization.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO OIDC", "StartDeviceAuthorization", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO OIDC.StartDeviceAuthorization",
-                "rpc.service" = "SSO OIDC",
-                "rpc.method" = "StartDeviceAuthorization",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO OIDC",
+            "StartDeviceAuthorization",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO OIDC.StartDeviceAuthorization",
+            "rpc.service" = "SSO OIDC",
+            "rpc.method" = "StartDeviceAuthorization",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             StartDeviceAuthorizationRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            StartDeviceAuthorizationResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                StartDeviceAuthorizationResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("StartDeviceAuthorization")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("StartDeviceAuthorization")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "StartDeviceAuthorization",
@@ -115,25 +129,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StartDeviceAuthorization")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "StartDeviceAuthorization",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 StartDeviceAuthorizationTelemetryInputCaptureInterceptor,
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
                 StartDeviceAuthorizationEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::start_device_authorization::StartDeviceAuthorizationError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::start_device_authorization::StartDeviceAuthorizationError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::start_device_authorization::StartDeviceAuthorizationError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -143,7 +169,9 @@
 struct StartDeviceAuthorizationTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for StartDeviceAuthorizationTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for StartDeviceAuthorizationTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "StartDeviceAuthorizationTelemetryInputCaptureInterceptor"
     }
@@ -202,9 +230,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_error(status, headers, body)
+            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_response(status, headers, body)
+            crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -212,12 +244,20 @@
 #[derive(Debug)]
 struct StartDeviceAuthorizationRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for StartDeviceAuthorizationRequestSerializer {
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
             .downcast::<crate::operation::start_device_authorization::StartDeviceAuthorizationInput>()
             .expect("correct type");
@@ -239,13 +279,18 @@
             fn update_http_builder(
                 input: &crate::operation::start_device_authorization::StartDeviceAuthorizationInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/json",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -253,7 +298,11 @@
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
@@ -286,13 +335,21 @@
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
     /// <p>Indicates that the client is not currently authorized to make the request. This can happen when a <code>clientId</code> is not issued for a public client.</p>
     UnauthorizedClientException(crate::types::error::UnauthorizedClientException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-StartDeviceAuthorizationError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-StartDeviceAuthorizationError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl StartDeviceAuthorizationError {
     /// Creates the `StartDeviceAuthorizationError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -397,7 +458,9 @@
             Self::SlowDownException(_inner) => _inner.fmt(f),
             Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -417,11 +480,19 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartDeviceAuthorizationError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::SlowDownException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::UnauthorizedClientException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -437,7 +508,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::start_device_authorization::StartDeviceAuthorizationError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::start_device_authorization::StartDeviceAuthorizationError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
diff --git a/src/operation/start_device_authorization/_start_device_authorization_input.rs b/src/operation/start_device_authorization/_start_device_authorization_input.rs
--- a/src/operation/start_device_authorization/_start_device_authorization_input.rs
+++ b/src/operation/start_device_authorization/_start_device_authorization_input.rs
@@ -101,11 +101,13 @@
         crate::operation::start_device_authorization::StartDeviceAuthorizationInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::start_device_authorization::StartDeviceAuthorizationInput {
-            client_id: self.client_id,
-            client_secret: self.client_secret,
-            start_url: self.start_url,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::start_device_authorization::StartDeviceAuthorizationInput {
+                client_id: self.client_id,
+                client_secret: self.client_secret,
+                start_url: self.start_url,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for StartDeviceAuthorizationInputBuilder {
diff --git a/src/operation/start_device_authorization/builders.rs b/src/operation/start_device_authorization/builders.rs
--- a/src/operation/start_device_authorization/builders.rs
+++ b/src/operation/start_device_authorization/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the StartDeviceAuthorization as a reference.
-    pub fn as_input(&self) -> &crate::operation::start_device_authorization::builders::StartDeviceAuthorizationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::start_device_authorization::builders::StartDeviceAuthorizationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::start_device_authorization::StartDeviceAuthorization::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::start_device_authorization::StartDeviceAuthorization::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::start_device_authorization::StartDeviceAuthorization::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::start_device_authorization::StartDeviceAuthorization::orchestrate(&runtime_plugins, input)
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
diff --git a/src/primitives/sealed_enum_unknown.rs b/src/primitives/sealed_enum_unknown.rs
--- a/src/primitives/sealed_enum_unknown.rs
+++ b/src/primitives/sealed_enum_unknown.rs
@@ -6,7 +6,13 @@
 /// This is not intended to be used directly.
 #[non_exhaustive]
 #[derive(
-    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
+    ::std::clone::Clone,
+    ::std::cmp::Eq,
+    ::std::cmp::Ord,
+    ::std::cmp::PartialEq,
+    ::std::cmp::PartialOrd,
+    ::std::fmt::Debug,
+    ::std::hash::Hash,
 )]
 pub struct UnknownVariantValue(pub(crate) ::std::string::String);
 impl UnknownVariantValue {
diff --git a/src/protocol_serde.rs b/src/protocol_serde.rs
--- a/src/protocol_serde.rs
+++ b/src/protocol_serde.rs
@@ -1,9 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
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
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
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

+#[cfg(feature = "op_create_token")]
 pub(crate) mod shape_create_token;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_create_token_with_iam;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_register_client;

+#[cfg(feature = "op_start_device_authorization")]
 pub(crate) mod shape_start_device_authorization;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -39,42 +66,93 @@
     }
 }

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_access_denied_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_authorization_pending_exception;

+#[cfg(feature = "op_create_token")]
 pub(crate) mod shape_create_token_input;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_create_token_with_iam_input;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_expired_token_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_internal_server_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_invalid_client_exception;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_invalid_client_metadata_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub(crate) mod shape_invalid_grant_exception;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_invalid_redirect_uri_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_invalid_request_exception;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_invalid_request_region_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub(crate) mod shape_invalid_scope_exception;

+#[cfg(feature = "op_register_client")]
 pub(crate) mod shape_register_client_input;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_slow_down_exception;

+#[cfg(feature = "op_start_device_authorization")]
 pub(crate) mod shape_start_device_authorization_input;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub(crate) mod shape_unauthorized_client_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub(crate) mod shape_unsupported_grant_type_exception;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_aws_additional_details;

+#[cfg(feature = "op_create_token_with_iam")]
 pub(crate) mod shape_scopes;
diff --git a/src/protocol_serde/shape_access_denied_exception.rs b/src/protocol_serde/shape_access_denied_exception.rs
--- a/src/protocol_serde/shape_access_denied_exception.rs
+++ b/src/protocol_serde/shape_access_denied_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_access_denied_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::AccessDeniedExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::AccessDeniedExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,7 +26,10 @@
                 "reason" => {
                     builder = builder.set_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| crate::types::AccessDeniedExceptionReason::from(u.as_ref())))
+                            .map(|s| {
+                                s.to_unescaped()
+                                    .map(|u| crate::types::AccessDeniedExceptionReason::from(u.as_ref()))
+                            })
                             .transpose()?,
                     );
                 }
@@ -43,9 +50,9 @@
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
diff --git a/src/protocol_serde/shape_authorization_pending_exception.rs b/src/protocol_serde/shape_authorization_pending_exception.rs
--- a/src/protocol_serde/shape_authorization_pending_exception.rs
+++ b/src/protocol_serde/shape_authorization_pending_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_authorization_pending_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::AuthorizationPendingExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::AuthorizationPendingExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::AuthorizationPendingExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_aws_additional_details.rs b/src/protocol_serde/shape_aws_additional_details.rs
--- a/src/protocol_serde/shape_aws_additional_details.rs
+++ b/src/protocol_serde/shape_aws_additional_details.rs
@@ -3,9 +3,17 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::AwsAdditionalDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::AwsAdditionalDetails>,
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
@@ -20,20 +28,22 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "identityContext" => {
-                            builder = builder.set_identity_context(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "identityContext" => {
+                                builder = builder.set_identity_context(
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
diff --git a/src/protocol_serde/shape_create_token.rs b/src/protocol_serde/shape_create_token.rs
--- a/src/protocol_serde/shape_create_token.rs
+++ b/src/protocol_serde/shape_create_token.rs
@@ -4,10 +4,14 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::create_token::CreateTokenOutput, crate::operation::create_token::CreateTokenError> {
+) -> std::result::Result<
+    crate::operation::create_token::CreateTokenOutput,
+    crate::operation::create_token::CreateTokenError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +26,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,29 +39,34 @@
             }
             tmp
         }),
-        "AuthorizationPendingException" => crate::operation::create_token::CreateTokenError::AuthorizationPendingException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "AuthorizationPendingException" => {
+            crate::operation::create_token::CreateTokenError::AuthorizationPendingException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_authorization_pending_exception::de_authorization_pending_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
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
         "ExpiredTokenException" => crate::operation::create_token::CreateTokenError::ExpiredTokenException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +80,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +98,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -98,8 +116,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidGrantExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +134,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +152,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -143,8 +170,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
+                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -153,40 +183,44 @@
             }
             tmp
         }),
-        "UnauthorizedClientException" => crate::operation::create_token::CreateTokenError::UnauthorizedClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnauthorizedClientException" => {
+            crate::operation::create_token::CreateTokenError::UnauthorizedClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnsupportedGrantTypeException" => crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException({
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
+        "UnsupportedGrantTypeException" => {
+            crate::operation::create_token::CreateTokenError::UnsupportedGrantTypeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::create_token::CreateTokenError::unhandled)?;
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
         _ => crate::operation::create_token::CreateTokenError::generic(generic),
     })
 }
@@ -196,7 +230,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::create_token::CreateTokenOutput, crate::operation::create_token::CreateTokenError> {
+) -> std::result::Result<
+    crate::operation::create_token::CreateTokenOutput,
+    crate::operation::create_token::CreateTokenError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::create_token::builders::CreateTokenOutputBuilder::default();
@@ -209,7 +246,8 @@

 pub fn ser_create_token_input(
     input: &crate::operation::create_token::CreateTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     crate::protocol_serde::shape_create_token_input::ser_create_token_input_input(&mut object, input)?;
@@ -220,9 +258,12 @@
 pub(crate) fn de_create_token(
     _value: &[u8],
     mut builder: crate::operation::create_token::builders::CreateTokenOutputBuilder,
-) -> ::std::result::Result<crate::operation::create_token::builders::CreateTokenOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::operation::create_token::builders::CreateTokenOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -269,9 +310,9 @@
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
diff --git a/src/protocol_serde/shape_create_token_with_iam.rs b/src/protocol_serde/shape_create_token_with_iam.rs
--- a/src/protocol_serde/shape_create_token_with_iam.rs
+++ b/src/protocol_serde/shape_create_token_with_iam.rs
@@ -9,8 +9,9 @@
     crate::operation::create_token_with_iam::CreateTokenWithIAMError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,152 +21,197 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AccessDeniedException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "AccessDeniedException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AccessDeniedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "AuthorizationPendingException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::AuthorizationPendingException({
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
+        "AuthorizationPendingException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::AuthorizationPendingException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::AuthorizationPendingExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_authorization_pending_exception::de_authorization_pending_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ExpiredTokenException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException({
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
+        "ExpiredTokenException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::ExpiredTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InternalServerException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException({
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
+        "InternalServerException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InternalServerException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidClientException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidClientException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidGrantException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidGrantException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidGrantException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidGrantExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidGrantExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_grant_exception::de_invalid_grant_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRequestException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException({
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
+        "InvalidRequestException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRequestRegionException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestRegionException({
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
+                        .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidRequestRegionException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidRequestRegionException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestRegionExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_region_exception::de_invalid_request_region_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRequestRegionExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_region_exception::de_invalid_request_region_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidScopeException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException({
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
+        "InvalidScopeException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::InvalidScopeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
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
         "SlowDownException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::SlowDownException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
+                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -174,40 +220,44 @@
             }
             tmp
         }),
-        "UnauthorizedClientException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnauthorizedClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnauthorizedClientException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnauthorizedClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                         .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnsupportedGrantTypeException" => crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnsupportedGrantTypeException({
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
+        "UnsupportedGrantTypeException" => {
+            crate::operation::create_token_with_iam::CreateTokenWithIAMError::UnsupportedGrantTypeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::create_token_with_iam::CreateTokenWithIAMError::unhandled)?;
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
         _ => crate::operation::create_token_with_iam::CreateTokenWithIAMError::generic(generic),
     })
 }
@@ -233,10 +283,14 @@

 pub fn ser_create_token_with_iam_input(
     input: &crate::operation::create_token_with_iam::CreateTokenWithIamInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    crate::protocol_serde::shape_create_token_with_iam_input::ser_create_token_with_iam_input_input(&mut object, input)?;
+    crate::protocol_serde::shape_create_token_with_iam_input::ser_create_token_with_iam_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
@@ -248,7 +302,8 @@
     crate::operation::create_token_with_iam::builders::CreateTokenWithIamOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -265,11 +320,13 @@
                     );
                 }
                 "awsAdditionalDetails" => {
-                    builder = builder.set_aws_additional_details(crate::protocol_serde::shape_aws_additional_details::de_aws_additional_details(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                    builder = builder.set_aws_additional_details(
+                        crate::protocol_serde::shape_aws_additional_details::de_aws_additional_details(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
                 }
                 "expiresIn" => {
                     builder = builder.set_expires_in(
@@ -300,7 +357,11 @@
                     );
                 }
                 "scope" => {
-                    builder = builder.set_scope(crate::protocol_serde::shape_scopes::de_scopes(tokens, _value, depth + 1)?);
+                    builder = builder.set_scope(crate::protocol_serde::shape_scopes::de_scopes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "tokenType" => {
                     builder = builder.set_token_type(
@@ -312,9 +373,9 @@
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
diff --git a/src/protocol_serde/shape_expired_token_exception.rs b/src/protocol_serde/shape_expired_token_exception.rs
--- a/src/protocol_serde/shape_expired_token_exception.rs
+++ b/src/protocol_serde/shape_expired_token_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_expired_token_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::ExpiredTokenExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::ExpiredTokenExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::ExpiredTokenExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_internal_server_exception.rs b/src/protocol_serde/shape_internal_server_exception.rs
--- a/src/protocol_serde/shape_internal_server_exception.rs
+++ b/src/protocol_serde/shape_internal_server_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_internal_server_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InternalServerExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InternalServerExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_invalid_client_exception.rs b/src/protocol_serde/shape_invalid_client_exception.rs
--- a/src/protocol_serde/shape_invalid_client_exception.rs
+++ b/src/protocol_serde/shape_invalid_client_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_client_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidClientExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidClientExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidClientExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_invalid_client_metadata_exception.rs b/src/protocol_serde/shape_invalid_client_metadata_exception.rs
--- a/src/protocol_serde/shape_invalid_client_metadata_exception.rs
+++ b/src/protocol_serde/shape_invalid_client_metadata_exception.rs
@@ -6,7 +6,8 @@
     crate::types::error::builders::InvalidClientMetadataExceptionBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -39,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_invalid_grant_exception.rs b/src/protocol_serde/shape_invalid_grant_exception.rs
--- a/src/protocol_serde/shape_invalid_grant_exception.rs
+++ b/src/protocol_serde/shape_invalid_grant_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_grant_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidGrantExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidGrantExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidGrantExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_invalid_redirect_uri_exception.rs b/src/protocol_serde/shape_invalid_redirect_uri_exception.rs
--- a/src/protocol_serde/shape_invalid_redirect_uri_exception.rs
+++ b/src/protocol_serde/shape_invalid_redirect_uri_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_invalid_redirect_uri_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRedirectUriExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRedirectUriExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRedirectUriExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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
@@ -22,7 +26,10 @@
                 "reason" => {
                     builder = builder.set_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| crate::types::InvalidRequestExceptionReason::from(u.as_ref())))
+                            .map(|s| {
+                                s.to_unescaped()
+                                    .map(|u| crate::types::InvalidRequestExceptionReason::from(u.as_ref()))
+                            })
                             .transpose()?,
                     );
                 }
@@ -43,9 +50,9 @@
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
diff --git a/src/protocol_serde/shape_invalid_request_region_exception.rs b/src/protocol_serde/shape_invalid_request_region_exception.rs
--- a/src/protocol_serde/shape_invalid_request_region_exception.rs
+++ b/src/protocol_serde/shape_invalid_request_region_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_invalid_request_region_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRequestRegionExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRequestRegionExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRequestRegionExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -51,9 +54,9 @@
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
diff --git a/src/protocol_serde/shape_invalid_scope_exception.rs b/src/protocol_serde/shape_invalid_scope_exception.rs
--- a/src/protocol_serde/shape_invalid_scope_exception.rs
+++ b/src/protocol_serde/shape_invalid_scope_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_scope_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidScopeExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidScopeExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidScopeExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_register_client.rs b/src/protocol_serde/shape_register_client.rs
--- a/src/protocol_serde/shape_register_client.rs
+++ b/src/protocol_serde/shape_register_client.rs
@@ -4,15 +4,23 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::register_client::RegisterClientOutput, crate::operation::register_client::RegisterClientError> {
+) -> std::result::Result<
+    crate::operation::register_client::RegisterClientOutput,
+    crate::operation::register_client::RegisterClientError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::register_client::RegisterClientError::unhandled(generic)),
+        None => {
+            return Err(crate::operation::register_client::RegisterClientError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -22,8 +30,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,47 +43,54 @@
             }
             tmp
         }),
-        "InvalidClientMetadataException" => crate::operation::register_client::RegisterClientError::InvalidClientMetadataException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidClientMetadataException" => {
+            crate::operation::register_client::RegisterClientError::InvalidClientMetadataException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidClientMetadataExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_metadata_exception::de_invalid_client_metadata_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidClientMetadataExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_client_metadata_exception::de_invalid_client_metadata_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRedirectUriException" => crate::operation::register_client::RegisterClientError::InvalidRedirectUriException({
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
+        "InvalidRedirectUriException" => {
+            crate::operation::register_client::RegisterClientError::InvalidRedirectUriException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRedirectUriExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRedirectUriExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_invalid_redirect_uri_exception::de_invalid_redirect_uri_exception_json_err(_response_body, output)
                         .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
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
         "InvalidRequestException" => crate::operation::register_client::RegisterClientError::InvalidRequestException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,8 +104,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -101,8 +122,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
+                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -111,24 +135,26 @@
             }
             tmp
         }),
-        "UnsupportedGrantTypeException" => crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnsupportedGrantTypeException" => {
+            crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
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
         _ => crate::operation::register_client::RegisterClientError::generic(generic),
     })
 }
@@ -138,7 +164,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::register_client::RegisterClientOutput, crate::operation::register_client::RegisterClientError> {
+) -> std::result::Result<
+    crate::operation::register_client::RegisterClientOutput,
+    crate::operation::register_client::RegisterClientError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::register_client::builders::RegisterClientOutputBuilder::default();
@@ -151,7 +180,8 @@

 pub fn ser_register_client_input(
     input: &crate::operation::register_client::RegisterClientInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     crate::protocol_serde::shape_register_client_input::ser_register_client_input_input(&mut object, input)?;
@@ -166,7 +196,8 @@
     crate::operation::register_client::builders::RegisterClientOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -220,9 +251,9 @@
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
diff --git a/src/protocol_serde/shape_scopes.rs b/src/protocol_serde/shape_scopes.rs
--- a/src/protocol_serde/shape_scopes.rs
+++ b/src/protocol_serde/shape_scopes.rs
@@ -3,9 +3,17 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<::std::string::String>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<::std::string::String>>,
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
diff --git a/src/protocol_serde/shape_slow_down_exception.rs b/src/protocol_serde/shape_slow_down_exception.rs
--- a/src/protocol_serde/shape_slow_down_exception.rs
+++ b/src/protocol_serde/shape_slow_down_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_slow_down_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::SlowDownExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::SlowDownExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::SlowDownExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -36,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_start_device_authorization.rs b/src/protocol_serde/shape_start_device_authorization.rs
--- a/src/protocol_serde/shape_start_device_authorization.rs
+++ b/src/protocol_serde/shape_start_device_authorization.rs
@@ -9,97 +9,127 @@
     crate::operation::start_device_authorization::StartDeviceAuthorizationError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
         None => {
-            return Err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled(
-                generic,
-            ))
+            return Err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled(generic))
         }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalServerException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::InternalServerException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InternalServerException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::InternalServerException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
-                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidClientException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidClientException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled,
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
+        "InvalidClientException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidRequestException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidClientExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_client_exception::de_invalid_client_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled,
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
+        "InvalidRequestException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "SlowDownException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::SlowDownException({
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
+                        .map_err(
+                            crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled,
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
+        "SlowDownException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::SlowDownException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
-                output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::SlowDownExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_slow_down_exception::de_slow_down_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedClientException" => crate::operation::start_device_authorization::StartDeviceAuthorizationError::UnauthorizedClientException({
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
+        "UnauthorizedClientException" => {
+            crate::operation::start_device_authorization::StartDeviceAuthorizationError::UnauthorizedClientException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
-                output =
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
+                    output =
                     crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                         .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
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
         _ => crate::operation::start_device_authorization::StartDeviceAuthorizationError::generic(generic),
     })
 }
@@ -115,9 +145,13 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder::default();
-        output = crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization(_response_body, output)
-            .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
+        let mut output =
+            crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder::default();
+        output = crate::protocol_serde::shape_start_device_authorization::de_start_device_authorization(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::start_device_authorization::StartDeviceAuthorizationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -125,10 +159,14 @@

 pub fn ser_start_device_authorization_input(
     input: &crate::operation::start_device_authorization::StartDeviceAuthorizationInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    crate::protocol_serde::shape_start_device_authorization_input::ser_start_device_authorization_input_input(&mut object, input)?;
+    crate::protocol_serde::shape_start_device_authorization_input::ser_start_device_authorization_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
@@ -140,7 +178,8 @@
     crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -194,9 +233,9 @@
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
diff --git a/src/protocol_serde/shape_unauthorized_client_exception.rs b/src/protocol_serde/shape_unauthorized_client_exception.rs
--- a/src/protocol_serde/shape_unauthorized_client_exception.rs
+++ b/src/protocol_serde/shape_unauthorized_client_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_unauthorized_client_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnauthorizedClientExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnauthorizedClientExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnauthorizedClientExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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
diff --git a/src/protocol_serde/shape_unsupported_grant_type_exception.rs b/src/protocol_serde/shape_unsupported_grant_type_exception.rs
--- a/src/protocol_serde/shape_unsupported_grant_type_exception.rs
+++ b/src/protocol_serde/shape_unsupported_grant_type_exception.rs
@@ -2,9 +2,12 @@
 pub(crate) fn de_unsupported_grant_type_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -37,9 +40,9 @@
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
@@ -1,14 +1,30 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::_access_denied_exception_reason::AccessDeniedExceptionReason;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::_invalid_request_exception_reason::InvalidRequestExceptionReason;

+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::_aws_additional_details::AwsAdditionalDetails;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _access_denied_exception_reason;

+#[cfg(feature = "op_create_token_with_iam")]
 mod _aws_additional_details;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _invalid_request_exception_reason;

 /// Builders
diff --git a/src/types/_access_denied_exception_reason.rs b/src/types/_access_denied_exception_reason.rs
--- a/src/types/_access_denied_exception_reason.rs
+++ b/src/types/_access_denied_exception_reason.rs
@@ -38,13 +38,21 @@
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
-    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
+    ::std::clone::Clone,
+    ::std::cmp::Eq,
+    ::std::cmp::Ord,
+    ::std::cmp::PartialEq,
+    ::std::cmp::PartialOrd,
+    ::std::fmt::Debug,
+    ::std::hash::Hash,
 )]
 pub enum AccessDeniedExceptionReason {
     #[allow(missing_docs)] // documentation missing in model
     KmsAccessDenied,
     /// `Unknown` contains new variants that have been added since this code was generated.
-    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
+    #[deprecated(
+        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
+    )]
     Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
 }
 impl ::std::convert::From<&str> for AccessDeniedExceptionReason {
@@ -51,7 +59,9 @@
     fn from(s: &str) -> Self {
         match s {
             "KMS_AccessDeniedException" => AccessDeniedExceptionReason::KmsAccessDenied,
-            other => AccessDeniedExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => AccessDeniedExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(
+                other.to_owned(),
+            )),
         }
     }
 }
diff --git a/src/types/_invalid_request_exception_reason.rs b/src/types/_invalid_request_exception_reason.rs
--- a/src/types/_invalid_request_exception_reason.rs
+++ b/src/types/_invalid_request_exception_reason.rs
@@ -41,7 +41,13 @@
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
-    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
+    ::std::clone::Clone,
+    ::std::cmp::Eq,
+    ::std::cmp::Ord,
+    ::std::cmp::PartialEq,
+    ::std::cmp::PartialOrd,
+    ::std::fmt::Debug,
+    ::std::hash::Hash,
 )]
 pub enum InvalidRequestExceptionReason {
     #[allow(missing_docs)] // documentation missing in model
@@ -53,7 +59,9 @@
     #[allow(missing_docs)] // documentation missing in model
     KmsKeyNotFound,
     /// `Unknown` contains new variants that have been added since this code was generated.
-    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
+    #[deprecated(
+        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
+    )]
     Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
 }
 impl ::std::convert::From<&str> for InvalidRequestExceptionReason {
@@ -63,7 +71,9 @@
             "KMS_InvalidKeyUsageException" => InvalidRequestExceptionReason::KmsInvalidKeyUsage,
             "KMS_InvalidStateException" => InvalidRequestExceptionReason::KmsInvalidState,
             "KMS_NotFoundException" => InvalidRequestExceptionReason::KmsKeyNotFound,
-            other => InvalidRequestExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => InvalidRequestExceptionReason::Unknown(
+                crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()),
+            ),
         }
     }
 }
diff --git a/src/types/builders.rs b/src/types/builders.rs
--- a/src/types/builders.rs
+++ b/src/types/builders.rs
@@ -1,2 +1,3 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::_aws_additional_details::AwsAdditionalDetailsBuilder;
diff --git a/src/types/error.rs b/src/types/error.rs
--- a/src/types/error.rs
+++ b/src/types/error.rs
@@ -1,58 +1,148 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_access_denied_exception::AccessDeniedException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_authorization_pending_exception::AuthorizationPendingException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_internal_server_exception::InternalServerException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_client_exception::InvalidClientException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_invalid_grant_exception::InvalidGrantException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_invalid_scope_exception::InvalidScopeException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_slow_down_exception::SlowDownException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_unauthorized_client_exception::UnauthorizedClientException;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_unsupported_grant_type_exception::UnsupportedGrantTypeException;

+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::error::_invalid_request_region_exception::InvalidRequestRegionException;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_client_metadata_exception::InvalidClientMetadataException;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_redirect_uri_exception::InvalidRedirectUriException;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _access_denied_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _authorization_pending_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _expired_token_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _internal_server_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 mod _invalid_client_exception;

+#[cfg(feature = "op_register_client")]
 mod _invalid_client_metadata_exception;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 mod _invalid_grant_exception;

+#[cfg(feature = "op_register_client")]
 mod _invalid_redirect_uri_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _invalid_request_exception;

+#[cfg(feature = "op_create_token_with_iam")]
 mod _invalid_request_region_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 mod _invalid_scope_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 mod _slow_down_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 mod _unauthorized_client_exception;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 mod _unsupported_grant_type_exception;

 /// Builders
diff --git a/src/types/error/builders.rs b/src/types/error/builders.rs
--- a/src/types/error/builders.rs
+++ b/src/types/error/builders.rs
@@ -1,28 +1,73 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_access_denied_exception::AccessDeniedExceptionBuilder;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_authorization_pending_exception::AuthorizationPendingExceptionBuilder;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_internal_server_exception::InternalServerExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_client_exception::InvalidClientExceptionBuilder;

+#[cfg(any(feature = "op_create_token", feature = "op_create_token_with_iam"))]
 pub use crate::types::error::_invalid_grant_exception::InvalidGrantExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_invalid_scope_exception::InvalidScopeExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_slow_down_exception::SlowDownExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_start_device_authorization"
+))]
 pub use crate::types::error::_unauthorized_client_exception::UnauthorizedClientExceptionBuilder;

+#[cfg(any(
+    feature = "op_create_token",
+    feature = "op_create_token_with_iam",
+    feature = "op_register_client"
+))]
 pub use crate::types::error::_unsupported_grant_type_exception::UnsupportedGrantTypeExceptionBuilder;

+#[cfg(feature = "op_create_token_with_iam")]
 pub use crate::types::error::_invalid_request_region_exception::InvalidRequestRegionExceptionBuilder;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_client_metadata_exception::InvalidClientMetadataExceptionBuilder;

+#[cfg(feature = "op_register_client")]
 pub use crate::types::error::_invalid_redirect_uri_exception::InvalidRedirectUriExceptionBuilder;
```


## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
