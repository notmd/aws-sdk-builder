# AWS SDK Conformance Report: sts

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sts
**Progress:** `146/146` files compared · `142` matched · `4` mismatches · `0` missing · `0` extra · `97.26%` match (100.00% means fully matched)

### `src/client.rs`

```diff
--- reference/src/client.rs
+++ generated/src/client.rs
@@ -10,6 +10,52 @@
 ///
 /// Client for invoking operations on AWS Security Token Service. Each operation on AWS Security Token Service is a method on this
 /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
+/// ## Constructing a `Client`
+///
+/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
+/// crate should be used to automatically resolve this config using
+/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
+/// across multiple different AWS SDK clients. This config resolution process can be customized
+/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
+/// the [builder pattern] to customize the default config.
+///
+/// In the simplest case, creating a client looks as follows:
+/// ```rust,no_run
+/// # async fn wrapper() {
+/// let config = aws_config::load_from_env().await;
+/// let client = aws_sdk_sts::Client::new(&config);
+/// # }
+/// ```
+///
+/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
+/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
+/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
+/// done as follows:
+///
+/// ```rust,no_run
+/// # async fn wrapper() {
+/// let sdk_config = ::aws_config::load_from_env().await;
+/// let config = aws_sdk_sts::config::Builder::from(&sdk_config)
+/// # /*
+///     .some_service_specific_setting("value")
+/// # */
+///     .build();
+/// # }
+/// ```
+///
+/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
+///
+/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
+/// be done once at application start-up.
+///
+/// [`Config`]: crate::Config
+/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
+/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
+/// [`aws-config` docs]: https://docs.rs/aws-config/*
+/// [`aws-config`]: https://crates.io/crates/aws-config
+/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
+/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
+/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
 /// # Using the `Client`
 ///
 /// A client has a function for every operation that can be performed by the service.
```

### `src/operation/get_caller_identity.rs`

```diff
--- reference/src/operation/get_caller_identity.rs
+++ generated/src/operation/get_caller_identity.rs
@@ -123,7 +123,7 @@
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
                     let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCallerIdentity")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
 .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCallerIdentityEndpointParamsInterceptor))
                             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<super::super::operation::get_caller_identity::GetCallerIdentityError>::new())
 .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<super::super::operation::get_caller_identity::GetCallerIdentityError>::new())
@@ -202,7 +202,10 @@
         let body = ::aws_smithy_types::body::SdkBody::from(
             super::super::protocol_serde::shape_get_caller_identity_input::ser_get_caller_identity_input_input_input(&input)?,
         );
-
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -87,18 +87,18 @@

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

 pub(crate) mod shape_federated_user;
+
+pub(crate) mod shape_policy_descriptor_type;
+
+pub(crate) mod shape_provided_context;
+
+pub(crate) mod shape_tag;
```

### `src/types/error/_idp_communication_error_exception.rs`

```diff
--- reference/src/types/error/_idp_communication_error_exception.rs
+++ generated/src/types/error/_idp_communication_error_exception.rs
@@ -13,6 +13,8 @@
     pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
         ::aws_smithy_types::retry::ErrorKind::ServerError
     }
+}
+impl IdpCommunicationErrorException {
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
         self.message.as_deref()
```
