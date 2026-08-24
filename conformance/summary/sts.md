# AWS SDK Conformance Report: sts

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sts
**Progress:** `146/146` files compared · `130` matched · `16` mismatches · `0` missing · `0` extra · `89.04%` match (100.00% means fully matched)

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

### `src/operation/assume_role.rs`

```diff
--- reference/src/operation/assume_role.rs
+++ generated/src/operation/assume_role.rs
@@ -139,15 +139,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::assume_role::AssumeRoleError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::assume_role::AssumeRoleError>::builder()
-                    .transient_errors({
-                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
-                        ::std::borrow::Cow::Owned(transient_errors)
-                    })
-                    .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::assume_role::AssumeRoleError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/assume_role_with_saml/builders.rs`

```diff
--- reference/src/operation/assume_role_with_saml/builders.rs
+++ generated/src/operation/assume_role_with_saml/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `AssumeRoleWithSAML`.
+/// Fluent builder constructing a request to `AssumeRoleWithSaml`.
 ///
 /// <p>Returns a set of temporary security credentials for users who have been authenticated via a SAML authentication response. This operation provides a mechanism for tying an enterprise identity store or directory to role-based Amazon Web Services access without user-specific credentials or configuration. For a comparison of <code>AssumeRoleWithSAML</code> with the other API operations that produce temporary credentials, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_request.html">Requesting Temporary Security Credentials</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_sts-comparison.html">Compare STS credentials</a> in the <i>IAM User Guide</i>.</p>
 /// <p>The temporary security credentials returned by this operation consist of an access key ID, a secret access key, and a security token. Applications can use these temporary security credentials to sign calls to Amazon Web Services services.</p><note>
@@ -58,7 +58,7 @@
 /// <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-idp_saml.html">Creating a Role for SAML 2.0 Federation</a> in the <i>IAM User Guide</i>.</p></li>
 /// </ul>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct AssumeRoleWithSAMLFluentBuilder {
+pub struct AssumeRoleWithSamlFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::assume_role_with_saml::builders::AssumeRoleWithSamlInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -66,8 +66,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
-    > for AssumeRoleWithSAMLFluentBuilder
+        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
+    > for AssumeRoleWithSamlFluentBuilder
 {
     fn send(
         self,
@@ -75,14 +75,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-            super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl AssumeRoleWithSAMLFluentBuilder {
-    /// Creates a new `AssumeRoleWithSAMLFluentBuilder`.
+impl AssumeRoleWithSamlFluentBuilder {
+    /// Creates a new `AssumeRoleWithSamlFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -90,7 +90,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the AssumeRoleWithSAML as a reference.
+    /// Access the AssumeRoleWithSaml as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::assume_role_with_saml::builders::AssumeRoleWithSamlInputBuilder {
         &self.inner
     }
@@ -107,7 +107,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -115,12 +115,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAML::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::assume_role_with_saml::AssumeRoleWithSaml::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAML::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSaml::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -128,7 +128,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/assume_role_with_saml.rs`

```diff
--- reference/src/operation/assume_role_with_saml.rs
+++ generated/src/operation/assume_role_with_saml.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `AssumeRoleWithSAML`.
+/// Orchestration and serialization glue logic for `AssumeRoleWithSaml`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct AssumeRoleWithSAML;
-impl AssumeRoleWithSAML {
-    /// Creates a new `AssumeRoleWithSAML`
+pub struct AssumeRoleWithSaml;
+impl AssumeRoleWithSaml {
+    /// Creates a new `AssumeRoleWithSaml`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AssumeRoleWithSAML {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AssumeRoleWithSaml {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AssumeRoleWithSAML");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            AssumeRoleWithSAMLRequestSerializer,
+            AssumeRoleWithSamlRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleWithSAMLResponseDeserializer,
+            AssumeRoleWithSamlResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -104,6 +104,16 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoleWithSAML", "STS"));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,17 +123,25 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithSAML")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithSAML")
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                AssumeRoleWithSamlTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                AssumeRoleWithSamlEndpointParamsInterceptor,
+            ))
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new())
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new())
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -130,12 +148,12 @@
 }

 #[derive(Debug)]
-struct AssumeRoleWithSAMLTelemetryInputCaptureInterceptor;
+struct AssumeRoleWithSamlTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSAMLTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSamlTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "AssumeRoleWithSAMLTelemetryInputCaptureInterceptor"
+        "AssumeRoleWithSamlTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -255,12 +273,12 @@
     }
 }
 #[derive(Debug)]
-struct AssumeRoleWithSAMLEndpointParamsInterceptor;
+struct AssumeRoleWithSamlEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSAMLEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSamlEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "AssumeRoleWithSAMLEndpointParamsInterceptor"
+        "AssumeRoleWithSamlEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/assume_role_with_web_identity.rs`

```diff
--- reference/src/operation/assume_role_with_web_identity.rs
+++ generated/src/operation/assume_role_with_web_identity.rs
@@ -107,7 +107,17 @@
             "AssumeRoleWithWebIdentity",
             "STS",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;

+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });
+
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -132,17 +142,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    super::super::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -451,10 +453,7 @@
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
     fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
-        match self {
-            Self::IdpCommunicationErrorException(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
-            _ => ::std::option::Option::None,
-        }
+        ::std::option::Option::None
     }
 }
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithWebIdentityError {
```

### `src/operation/assume_root.rs`

```diff
--- reference/src/operation/assume_root.rs
+++ generated/src/operation/assume_root.rs
@@ -139,15 +139,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::assume_root::AssumeRootError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::assume_root::AssumeRootError>::builder()
-                    .transient_errors({
-                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
-                        ::std::borrow::Cow::Owned(transient_errors)
-                    })
-                    .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::assume_root::AssumeRootError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/decode_authorization_message.rs`

```diff
--- reference/src/operation/decode_authorization_message.rs
+++ generated/src/operation/decode_authorization_message.rs
@@ -141,17 +141,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::decode_authorization_message::DecodeAuthorizationMessageError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    super::super::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/get_access_key_info.rs`

```diff
--- reference/src/operation/get_access_key_info.rs
+++ generated/src/operation/get_access_key_info.rs
@@ -138,16 +138,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::get_access_key_info::GetAccessKeyInfoError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::get_access_key_info::GetAccessKeyInfoError>::builder(
-                )
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::get_access_key_info::GetAccessKeyInfoError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/get_caller_identity.rs`

```diff
--- reference/src/operation/get_caller_identity.rs
+++ generated/src/operation/get_caller_identity.rs
@@ -122,16 +122,22 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCallerIdentity")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCallerIdentityEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<super::super::operation::get_caller_identity::GetCallerIdentityError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<super::super::operation::get_caller_identity::GetCallerIdentityError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::get_caller_identity::GetCallerIdentityError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCallerIdentity")
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetCallerIdentityEndpointParamsInterceptor,
+            ))
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                super::super::operation::get_caller_identity::GetCallerIdentityError,
+            >::new())
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                super::super::operation::get_caller_identity::GetCallerIdentityError,
+            >::new())
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::get_caller_identity::GetCallerIdentityError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -202,7 +208,10 @@
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

### `src/operation/get_delegated_access_token.rs`

```diff
--- reference/src/operation/get_delegated_access_token.rs
+++ generated/src/operation/get_delegated_access_token.rs
@@ -139,17 +139,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    super::super::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/get_federation_token.rs`

```diff
--- reference/src/operation/get_federation_token.rs
+++ generated/src/operation/get_federation_token.rs
@@ -123,17 +123,25 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFederationToken")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFederationTokenTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFederationTokenEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<super::super::operation::get_federation_token::GetFederationTokenError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<super::super::operation::get_federation_token::GetFederationTokenError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::get_federation_token::GetFederationTokenError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFederationToken")
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetFederationTokenTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetFederationTokenEndpointParamsInterceptor,
+            ))
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                super::super::operation::get_federation_token::GetFederationTokenError,
+            >::new())
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                super::super::operation::get_federation_token::GetFederationTokenError,
+            >::new())
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::get_federation_token::GetFederationTokenError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/get_session_token.rs`

```diff
--- reference/src/operation/get_session_token.rs
+++ generated/src/operation/get_session_token.rs
@@ -139,15 +139,9 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 super::super::operation::get_session_token::GetSessionTokenError,
             >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::get_session_token::GetSessionTokenError>::builder()
-                    .transient_errors({
-                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
-                        ::std::borrow::Cow::Owned(transient_errors)
-                    })
-                    .build(),
-            );
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::get_session_token::GetSessionTokenError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/operation/get_web_identity_token.rs`

```diff
--- reference/src/operation/get_web_identity_token.rs
+++ generated/src/operation/get_web_identity_token.rs
@@ -126,17 +126,25 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetWebIdentityToken")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetWebIdentityTokenTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetWebIdentityTokenEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<super::super::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<super::super::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<super::super::operation::get_web_identity_token::GetWebIdentityTokenError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetWebIdentityToken")
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetWebIdentityTokenTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetWebIdentityTokenEndpointParamsInterceptor,
+            ))
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                super::super::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new())
+            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                super::super::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new())
+            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                super::super::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
```

### `src/protocol_serde/shape_assume_role_with_saml.rs`

```diff
--- reference/src/protocol_serde/shape_assume_role_with_saml.rs
+++ generated/src/protocol_serde/shape_assume_role_with_saml.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-    super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+    super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+        .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled(generic)),
+        None => return Err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ExpiredTokenException" => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException({
+        "ExpiredTokenException" => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::ExpiredTokenException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ExpiredTokenExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                    .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "IDPRejectedClaim" => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException({
+        "IDPRejectedClaim" => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::IdpRejectedClaimException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::IdpRejectedClaimExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_idp_rejected_claim_exception::de_idp_rejected_claim_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                    .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,7 +50,7 @@
             }
             tmp
         }),
-        "InvalidIdentityToken" => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException({
+        "InvalidIdentityToken" => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::InvalidIdentityTokenException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -59,7 +59,7 @@
                     _response_body,
                     output,
                 )
-                .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,7 +68,7 @@
             }
             tmp
         }),
-        "MalformedPolicyDocument" => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException({
+        "MalformedPolicyDocument" => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::MalformedPolicyDocumentException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +77,7 @@
                     _response_body,
                     output,
                 )
-                .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,7 +86,7 @@
             }
             tmp
         }),
-        "PackedPolicyTooLarge" => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException({
+        "PackedPolicyTooLarge" => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::PackedPolicyTooLargeException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -95,7 +95,7 @@
                     _response_body,
                     output,
                 )
-                .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -104,13 +104,13 @@
             }
             tmp
         }),
-        "RegionDisabledException" => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException({
+        "RegionDisabledException" => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::RegionDisabledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::RegionDisabledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                    .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,7 +119,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::generic(generic),
+        _ => super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::generic(generic),
     })
 }

@@ -130,13 +130,13 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-    super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+    super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError,
 > {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder::default();
         output = super::super::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml(_response_body, output)
-            .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+            .map_err(super::super::operation::assume_role_with_saml::AssumeRoleWithSamlError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -170,7 +170,7 @@
         }
         while let Some(mut tag) = result_tag.next_tag() {
             match tag.start_el() {
-            s if s.matches("Credentials") /* Credentials com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$Credentials */ =>  {
+            s if s.matches("Credentials") /* Credentials com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$Credentials */ =>  {
                 let var_1 =
                     Some(
                         super::super::protocol_serde::shape_credentials::de_credentials(&mut tag, depth + 1)
@@ -180,7 +180,7 @@
                 builder = builder.set_credentials(var_1);
             }
             ,
-            s if s.matches("AssumedRoleUser") /* AssumedRoleUser com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$AssumedRoleUser */ =>  {
+            s if s.matches("AssumedRoleUser") /* AssumedRoleUser com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$AssumedRoleUser */ =>  {
                 let var_2 =
                     Some(
                         super::super::protocol_serde::shape_assumed_role_user::de_assumed_role_user(&mut tag, depth + 1)
@@ -190,7 +190,7 @@
                 builder = builder.set_assumed_role_user(var_2);
             }
             ,
-            s if s.matches("PackedPolicySize") /* PackedPolicySize com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$PackedPolicySize */ =>  {
+            s if s.matches("PackedPolicySize") /* PackedPolicySize com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$PackedPolicySize */ =>  {
                 let var_3 =
                     Some(
                          {
@@ -205,7 +205,7 @@
                 builder = builder.set_packed_policy_size(var_3);
             }
             ,
-            s if s.matches("Subject") /* Subject com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$Subject */ =>  {
+            s if s.matches("Subject") /* Subject com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$Subject */ =>  {
                 let var_4 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
@@ -218,7 +218,7 @@
                 builder = builder.set_subject(var_4);
             }
             ,
-            s if s.matches("SubjectType") /* SubjectType com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$SubjectType */ =>  {
+            s if s.matches("SubjectType") /* SubjectType com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$SubjectType */ =>  {
                 let var_5 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
@@ -231,7 +231,7 @@
                 builder = builder.set_subject_type(var_5);
             }
             ,
-            s if s.matches("Issuer") /* Issuer com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$Issuer */ =>  {
+            s if s.matches("Issuer") /* Issuer com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$Issuer */ =>  {
                 let var_6 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
@@ -244,7 +244,7 @@
                 builder = builder.set_issuer(var_6);
             }
             ,
-            s if s.matches("Audience") /* Audience com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$Audience */ =>  {
+            s if s.matches("Audience") /* Audience com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$Audience */ =>  {
                 let var_7 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
@@ -257,7 +257,7 @@
                 builder = builder.set_audience(var_7);
             }
             ,
-            s if s.matches("NameQualifier") /* NameQualifier com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$NameQualifier */ =>  {
+            s if s.matches("NameQualifier") /* NameQualifier com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$NameQualifier */ =>  {
                 let var_8 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
@@ -270,7 +270,7 @@
                 builder = builder.set_name_qualifier(var_8);
             }
             ,
-            s if s.matches("SourceIdentity") /* SourceIdentity com.amazonaws.sts.synthetic#AssumeRoleWithSAMLOutput$SourceIdentity */ =>  {
+            s if s.matches("SourceIdentity") /* SourceIdentity com.amazonaws.sts.synthetic#AssumeRoleWithSamlOutput$SourceIdentity */ =>  {
                 let var_9 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
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
@@ -9,10 +9,6 @@
     pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
 }
 impl IdpCommunicationErrorException {
-    /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
-    pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
-        ::aws_smithy_types::retry::ErrorKind::ServerError
-    }
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
         self.message.as_deref()
```
