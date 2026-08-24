# AWS SDK Conformance Report: sts

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sts
**Progress:** `146/146` files compared · `77` matched · `27` mismatches · `42` missing · `0` extra · `52.74%` match (100.00% means fully matched)

### `src/client/assume_role_with_web_identity.rs`

```diff
--- reference/src/client/assume_role_with_web_identity.rs
+++ generated/src/client/assume_role_with_web_identity.rs
@@ -19,7 +19,9 @@
     ///   - [`audience(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::audience): <p>The intended audience (also known as client ID) of the web identity token. This is traditionally the client identifier issued to the application that requested the web identity token.</p>
     ///   - [`source_identity(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::source_identity): <p>The value of the source identity that is returned in the JSON web token (JWT) from the identity provider.</p> <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#id_roles_terms-and-concepts">chained role</a> sessions. You can configure your identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithWebIdentity</code>. You do this by adding a claim to the JSON web token. To learn more about OIDC tokens and claims, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html">Using Tokens with User Pools</a> in the <i>Amazon Cognito Developer Guide</i>. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
     /// - On failure, responds with [`SdkError<AssumeRoleWithWebIdentityError>`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError)
-    pub fn assume_role_with_web_identity(&self) -> super::super::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
+    pub fn assume_role_with_web_identity(
+        &self,
+    ) -> super::super::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
         super::super::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/decode_authorization_message.rs`

```diff
--- reference/src/client/decode_authorization_message.rs
+++ generated/src/client/decode_authorization_message.rs
@@ -7,7 +7,9 @@
     /// - On success, responds with [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput) with field(s):
     ///   - [`decoded_message(Option<String>)`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput::decoded_message): <p>The API returns a response with the decoded message.</p>
     /// - On failure, responds with [`SdkError<DecodeAuthorizationMessageError>`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageError)
-    pub fn decode_authorization_message(&self) -> super::super::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder {
+    pub fn decode_authorization_message(
+        &self,
+    ) -> super::super::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder {
         super::super::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder::new(self.handle.clone())
     }
 }
```

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

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -29,7 +29,10 @@
 /// Endpoint resolver trait specific to AWS Security Token Service
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -391,7 +394,10 @@
 }

 impl super::super::config::endpoint::ResolveEndpoint for DefaultResolver {
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         // Check single-entry cache (lock-free read via ArcSwap)
         let cached = self.endpoint_cache.load();
         if let Some((cached_params, cached_endpoint)) = cached.as_ref() {
```

### `src/operation/assume_role/_assume_role_input.rs`

```diff
--- reference/src/operation/assume_role/_assume_role_input.rs
+++ generated/src/operation/assume_role/_assume_role_input.rs
@@ -461,7 +461,9 @@
         &self.provided_contexts
     }
     /// Consumes the builder and constructs a [`AssumeRoleInput`](crate::operation::assume_role::AssumeRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::assume_role::AssumeRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::assume_role::AssumeRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::assume_role::AssumeRoleInput {
             role_arn: self.role_arn,
             role_session_name: self.role_session_name,
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
@@ -257,7 +251,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::assume_role::AssumeRoleInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::assume_role::AssumeRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -282,12 +278,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_assume_role_input::ser_assume_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_assume_role_input::ser_assume_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs`

```diff
--- reference/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
+++ generated/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
@@ -236,8 +236,10 @@
     /// Consumes the builder and constructs a [`AssumeRoleWithSamlInput`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::assume_role_with_saml::AssumeRoleWithSamlInput {
             role_arn: self.role_arn,
             principal_arn: self.principal_arn,
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
@@ -104,7 +104,17 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoleWithSAML", "STS"));
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
@@ -240,12 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_input_input_input(&input)?,
+            super::super::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -255,12 +272,12 @@
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

### `src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs`

```diff
--- reference/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
+++ generated/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
@@ -287,15 +287,17 @@
         super::super::super::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput {
-            role_arn: self.role_arn,
-            role_session_name: self.role_session_name,
-            web_identity_token: self.web_identity_token,
-            provider_id: self.provider_id,
-            policy_arns: self.policy_arns,
-            policy: self.policy,
-            duration_seconds: self.duration_seconds,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput {
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
@@ -264,12 +266,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_assume_role_with_web_identity_input::ser_assume_role_with_web_identity_input_input_input(&input)?,
+            super::super::protocol_serde::shape_assume_role_with_web_identity_input::ser_assume_role_with_web_identity_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -451,10 +452,7 @@
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

### `src/operation/assume_root/_assume_root_input.rs`

```diff
--- reference/src/operation/assume_root/_assume_root_input.rs
+++ generated/src/operation/assume_root/_assume_root_input.rs
@@ -150,7 +150,9 @@
         &self.duration_seconds
     }
     /// Consumes the builder and constructs a [`AssumeRootInput`](crate::operation::assume_root::AssumeRootInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::assume_root::AssumeRootInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::assume_root::AssumeRootInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::assume_root::AssumeRootInput {
             target_principal: self.target_principal,
             task_policy_arn: self.task_policy_arn,
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
@@ -227,7 +221,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::assume_root::AssumeRootInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::assume_root::AssumeRootInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -252,12 +248,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_assume_root_input::ser_assume_root_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_assume_root_input::ser_assume_root_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/decode_authorization_message/_decode_authorization_message_input.rs`

```diff
--- reference/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
+++ generated/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
@@ -48,8 +48,10 @@
         super::super::super::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::decode_authorization_message::DecodeAuthorizationMessageInput {
-            encoded_message: self.encoded_message,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::decode_authorization_message::DecodeAuthorizationMessageInput {
+                encoded_message: self.encoded_message,
+            },
+        )
     }
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
@@ -258,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_decode_authorization_message_input::ser_decode_authorization_message_input_input_input(&input)?,
+            super::super::protocol_serde::shape_decode_authorization_message_input::ser_decode_authorization_message_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_access_key_info/_get_access_key_info_input.rs`

```diff
--- reference/src/operation/get_access_key_info/_get_access_key_info_input.rs
+++ generated/src/operation/get_access_key_info/_get_access_key_info_input.rs
@@ -49,7 +49,10 @@
     /// Consumes the builder and constructs a [`GetAccessKeyInfoInput`](crate::operation::get_access_key_info::GetAccessKeyInfoInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_access_key_info::GetAccessKeyInfoInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_access_key_info::GetAccessKeyInfoInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_access_key_info::GetAccessKeyInfoInput {
             access_key_id: self.access_key_id,
         })
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
@@ -254,12 +247,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_access_key_info_input::ser_get_access_key_info_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_access_key_info_input::ser_get_access_key_info_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_caller_identity/_get_caller_identity_input.rs`

```diff
--- reference/src/operation/get_caller_identity/_get_caller_identity_input.rs
+++ generated/src/operation/get_caller_identity/_get_caller_identity_input.rs
@@ -18,7 +18,10 @@
     /// Consumes the builder and constructs a [`GetCallerIdentityInput`](crate::operation::get_caller_identity::GetCallerIdentityInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_caller_identity::GetCallerIdentityInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_caller_identity::GetCallerIdentityInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_caller_identity::GetCallerIdentityInput {})
     }
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
@@ -195,13 +201,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_caller_identity_input::ser_get_caller_identity_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
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
@@ -213,12 +205,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_delegated_access_token_input::ser_get_delegated_access_token_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_delegated_access_token_input::ser_get_delegated_access_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_federation_token/_get_federation_token_input.rs`

```diff
--- reference/src/operation/get_federation_token/_get_federation_token_input.rs
+++ generated/src/operation/get_federation_token/_get_federation_token_input.rs
@@ -239,8 +239,10 @@
     /// Consumes the builder and constructs a [`GetFederationTokenInput`](crate::operation::get_federation_token::GetFederationTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_federation_token::GetFederationTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_federation_token::GetFederationTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_federation_token::GetFederationTokenInput {
             name: self.name,
             policy: self.policy,
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
@@ -245,12 +253,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_session_token/_get_session_token_input.rs`

```diff
--- reference/src/operation/get_session_token/_get_session_token_input.rs
+++ generated/src/operation/get_session_token/_get_session_token_input.rs
@@ -95,7 +95,10 @@
     /// Consumes the builder and constructs a [`GetSessionTokenInput`](crate::operation::get_session_token::GetSessionTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_session_token::GetSessionTokenInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_session_token::GetSessionTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_session_token::GetSessionTokenInput {
             duration_seconds: self.duration_seconds,
             serial_number: self.serial_number,
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
@@ -259,12 +253,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_session_token_input::ser_get_session_token_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_session_token_input::ser_get_session_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_web_identity_token/_get_web_identity_token_input.rs`

```diff
--- reference/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
+++ generated/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
@@ -123,8 +123,10 @@
     /// Consumes the builder and constructs a [`GetWebIdentityTokenInput`](crate::operation::get_web_identity_token::GetWebIdentityTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_web_identity_token::GetWebIdentityTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_web_identity_token::GetWebIdentityTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_web_identity_token::GetWebIdentityTokenInput {
             audience: self.audience,
             duration_seconds: self.duration_seconds,
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
@@ -243,12 +251,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
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

### Missing reference files

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
- `src/protocol_serde/shape_policy_descriptor_type.rs`
- `src/protocol_serde/shape_provided_context.rs`
- `src/protocol_serde/shape_region_disabled_exception.rs`
- `src/protocol_serde/shape_session_duration_escalation_exception.rs`
- `src/protocol_serde/shape_tag.rs`
- `src/protocol_serde.rs`
- `src/rest_xml_wrapped_errors.rs`
