# AWS SDK Conformance Report: sesv2

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sesv2
**Progress:** `1159/1159` files compared · `1132` matched · `26` mismatches · `1` missing · `0` extra · `97.67%` match (100.00% means fully matched)

### `src/config/auth.rs`

```diff
--- reference/src/config/auth.rs
+++ generated/src/config/auth.rs
@@ -55,19 +55,10 @@
 impl Default for DefaultAuthSchemeResolver {
     fn default() -> Self {
         Self {
-            service_defaults: vec![
-                ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
-                    .scheme_id(::aws_runtime::auth::sigv4::SCHEME_ID)
-                    .build()
-                    .expect("required fields set"),
-                #[cfg(feature = "sigv4a")]
-                {
-                    ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
-                        .scheme_id(::aws_runtime::auth::sigv4a::SCHEME_ID)
-                        .build()
-                        .expect("required fields set")
-                },
-            ],
+            service_defaults: vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
+                .scheme_id(::aws_runtime::auth::sigv4::SCHEME_ID)
+                .build()
+                .expect("required fields set")],
             operation_overrides: ::std::collections::HashMap::new(),
         }
     }
@@ -89,10 +80,6 @@

         let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::new(async move {
-            super::super::endpoint_auth::resolve_endpoint_based_auth_scheme_options(modeled_auth_options, _cfg, _runtime_components).await
-        });
-
         _fut
     }
 }
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -147,10 +147,6 @@
     pub fn signing_name(&self) -> &'static str {
         "ses"
     }
-    /// Returns the SigV4a signing region set, if configured.
-    pub fn sigv4a_signing_region_set(&self) -> Option<&::aws_types::region::SigningRegionSet> {
-        self.config.load::<::aws_types::region::SigningRegionSet>()
-    }
     /// Returns the AWS region, if it was provided.
     pub fn region(&self) -> ::std::option::Option<&super::config::Region> {
         self.config.load::<super::config::Region>()
@@ -209,7 +205,6 @@
         builder.set_endpoint_url(config_bag.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()));
         builder.set_use_dual_stack(config_bag.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0));
         builder.set_use_fips(config_bag.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0));
-        builder.set_sigv4a_signing_region_set(config_bag.load::<::aws_types::region::SigningRegionSet>().cloned());
         builder.set_region(config_bag.load::<super::config::Region>().cloned());
         builder
     }
@@ -1219,17 +1214,6 @@
         self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
-    /// Sets the SigV4a signing region set.
-    pub fn sigv4a_signing_region_set(mut self, v: impl Into<::aws_types::region::SigningRegionSet>) -> Self {
-        self.set_sigv4a_signing_region_set(Some(v.into()));
-        self
-    }
-
-    /// Sets the SigV4a signing region set.
-    pub fn set_sigv4a_signing_region_set(&mut self, v: Option<::aws_types::region::SigningRegionSet>) -> &mut Self {
-        self.config.store_or_unset(v);
-        self
-    }
     /// Sets the AWS region to use when making requests.
     ///
     /// # Examples
@@ -1260,11 +1244,6 @@
     /// Sets the credentials provider for this service
     pub fn set_credentials_provider(&mut self, credentials_provider: ::std::option::Option<super::config::SharedCredentialsProvider>) -> &mut Self {
         if let Some(credentials_provider) = credentials_provider {
-            #[cfg(feature = "sigv4a")]
-            {
-                self.runtime_components
-                    .set_identity_resolver(::aws_runtime::auth::sigv4a::SCHEME_ID, credentials_provider.clone());
-            }
             self.runtime_components
                 .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
         }
@@ -1463,12 +1442,6 @@
         runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
             ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
         ));
-        #[cfg(feature = "sigv4a")]
-        {
-            runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
-                ::aws_runtime::auth::sigv4a::SigV4aAuthScheme::new(),
-            ));
-        }
         runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
             super::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
         ));
@@ -1575,7 +1548,6 @@
         let mut builder = Builder::default();
         builder.set_credentials_provider(input.credentials_provider());
         builder = builder.region(input.region().cloned());
-        builder.set_sigv4a_signing_region_set(input.sigv4a_signing_region_set().cloned());
         builder.set_use_fips(input.use_fips());
         builder.set_use_dual_stack(input.use_dual_stack());
         if input.get_origin("endpoint_url").is_client_config() {
```

### `src/endpoint_lib.rs`

```diff
--- reference/src/endpoint_lib.rs
+++ generated/src/endpoint_lib.rs
@@ -1,4 +1,3 @@
-// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 // Loading the partition JSON is expensive since it involves many regex compilations,
 // so cache the result so that it only need to be paid for the first constructed client.
 pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<super::endpoint_lib::partition::PartitionResolver> = std::sync::LazyLock::new(
@@ -19,6 +18,6 @@

 pub(crate) mod diagnostic;

-pub(crate) mod host;
-
 pub(crate) mod partition;
+
+pub(crate) mod host;
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -199,8 +199,6 @@

 mod serialization_settings;

-pub(crate) mod endpoint_auth;
-
 mod endpoint_lib;

 mod lens;
```

### `src/operation/get_reputation_entity.rs`

```diff
--- reference/src/operation/get_reputation_entity.rs
+++ generated/src/operation/get_reputation_entity.rs
@@ -241,8 +241,7 @@
                 let input_1 = input_1.as_ref().ok_or_else(|| {
                     ::aws_smithy_types::error::operation::BuildError::missing_field("reputation_entity_type", "cannot be empty or unset")
                 })?;
-                let reputation_entity_type =
-                    ::aws_smithy_http::label::fmt_string(input_1.as_str(), ::aws_smithy_http::label::EncodingStrategy::Default);
+                let reputation_entity_type = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if reputation_entity_type.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "reputation_entity_type",
```

### `src/operation/put_email_identity_dkim_signing_attributes/_put_email_identity_dkim_signing_attributes_output.rs`

```diff
--- reference/src/operation/put_email_identity_dkim_signing_attributes/_put_email_identity_dkim_signing_attributes_output.rs
+++ generated/src/operation/put_email_identity_dkim_signing_attributes/_put_email_identity_dkim_signing_attributes_output.rs
@@ -27,12 +27,9 @@
     pub dkim_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub signing_hosted_zone: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
 }
@@ -65,12 +62,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn signing_hosted_zone(&self) -> ::std::option::Option<&str> {
         self.signing_hosted_zone.as_deref()
     }
@@ -181,12 +175,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn signing_hosted_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.signing_hosted_zone = ::std::option::Option::Some(input.into());
         self
@@ -193,12 +184,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn set_signing_hosted_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.signing_hosted_zone = input;
         self
@@ -205,12 +193,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn get_signing_hosted_zone(&self) -> &::std::option::Option<::std::string::String> {
         &self.signing_hosted_zone
     }
```

### `src/operation/update_reputation_entity_customer_managed_status.rs`

```diff
--- reference/src/operation/update_reputation_entity_customer_managed_status.rs
+++ generated/src/operation/update_reputation_entity_customer_managed_status.rs
@@ -248,8 +248,7 @@
                 let input_1 = input_1.as_ref().ok_or_else(|| {
                     ::aws_smithy_types::error::operation::BuildError::missing_field("reputation_entity_type", "cannot be empty or unset")
                 })?;
-                let reputation_entity_type =
-                    ::aws_smithy_http::label::fmt_string(input_1.as_str(), ::aws_smithy_http::label::EncodingStrategy::Default);
+                let reputation_entity_type = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if reputation_entity_type.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "reputation_entity_type",
```

### `src/operation/update_reputation_entity_policy.rs`

```diff
--- reference/src/operation/update_reputation_entity_policy.rs
+++ generated/src/operation/update_reputation_entity_policy.rs
@@ -252,8 +252,7 @@
                 let input_1 = input_1.as_ref().ok_or_else(|| {
                     ::aws_smithy_types::error::operation::BuildError::missing_field("reputation_entity_type", "cannot be empty or unset")
                 })?;
-                let reputation_entity_type =
-                    ::aws_smithy_http::label::fmt_string(input_1.as_str(), ::aws_smithy_http::label::EncodingStrategy::Default);
+                let reputation_entity_type = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if reputation_entity_type.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "reputation_entity_type",
```

### `src/protocol_serde/shape_attachment.rs`

```diff
--- reference/src/protocol_serde/shape_attachment.rs
+++ generated/src/protocol_serde/shape_attachment.rs
@@ -6,7 +6,7 @@
     {
         object
             .key("RawContent")
-            .string_unchecked(&::aws_smithy_types::base64::encode(&input.raw_content));
+            .string_unchecked(&::aws_smithy_types::base64::encode(input.raw_content));
     }
     if let Some(var_1) = &input.content_disposition {
         object.key("ContentDisposition").string(var_1.as_str());
```

### `src/protocol_serde/shape_batch_get_metric_data_query.rs`

```diff
--- reference/src/protocol_serde/shape_batch_get_metric_data_query.rs
+++ generated/src/protocol_serde/shape_batch_get_metric_data_query.rs
@@ -25,12 +25,12 @@
     {
         object
             .key("StartDate")
-            .date_time(&input.start_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.start_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     {
         object
             .key("EndDate")
-            .date_time(&input.end_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.end_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_event_destination_definition.rs`

```diff
--- reference/src/protocol_serde/shape_event_destination_definition.rs
+++ generated/src/protocol_serde/shape_event_destination_definition.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EventDestinationDefinition,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.enabled {
+    {
         object.key("Enabled").boolean(input.enabled);
     }
     if let Some(var_1) = &input.matching_event_types {
```

### `src/protocol_serde/shape_export_dimensions.rs`

```diff
--- reference/src/protocol_serde/shape_export_dimensions.rs
+++ generated/src/protocol_serde/shape_export_dimensions.rs
@@ -23,7 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key.to_unescaped().map(|u| super::super::types::MetricDimensionName::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = super::super::protocol_serde::shape_export_dimension_value::de_export_dimension_value(tokens, _value, depth + 1)?;
                         match value {
                             Some(value) => {
```

### `src/protocol_serde/shape_get_deliverability_dashboard_options.rs`

```diff
--- reference/src/protocol_serde/shape_get_deliverability_dashboard_options.rs
+++ generated/src/protocol_serde/shape_get_deliverability_dashboard_options.rs
@@ -91,7 +91,9 @@
         output = super::super::protocol_serde::shape_get_deliverability_dashboard_options::de_get_deliverability_dashboard_options(_response_body, output)
             .map_err(super::super::operation::get_deliverability_dashboard_options::GetDeliverabilityDashboardOptionsError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::get_deliverability_dashboard_options_output_output_correct_errors(output).build()
+        super::super::serde_util::get_deliverability_dashboard_options_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::get_deliverability_dashboard_options::GetDeliverabilityDashboardOptionsError::unhandled)?
     })
 }

```

### `src/protocol_serde/shape_inbox_placement_tracking_option.rs`

```diff
--- reference/src/protocol_serde/shape_inbox_placement_tracking_option.rs
+++ generated/src/protocol_serde/shape_inbox_placement_tracking_option.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::InboxPlacementTrackingOption,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.global {
+    {
         object.key("Global").boolean(input.global);
     }
     if let Some(var_1) = &input.tracked_isps {
```

### `src/protocol_serde/shape_message_insights_data_source.rs`

```diff
--- reference/src/protocol_serde/shape_message_insights_data_source.rs
+++ generated/src/protocol_serde/shape_message_insights_data_source.rs
@@ -6,12 +6,12 @@
     {
         object
             .key("StartDate")
-            .date_time(&input.start_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.start_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     {
         object
             .key("EndDate")
-            .date_time(&input.end_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.end_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     if let Some(var_1) = &input.include {
         #[allow(unused_mut)]
```

### `src/protocol_serde/shape_metrics_data_source.rs`

```diff
--- reference/src/protocol_serde/shape_metrics_data_source.rs
+++ generated/src/protocol_serde/shape_metrics_data_source.rs
@@ -37,12 +37,12 @@
     {
         object
             .key("StartDate")
-            .date_time(&input.start_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.start_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     {
         object
             .key("EndDate")
-            .date_time(&input.end_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.end_date, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_email_identity_configuration_set_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_configuration_set_attributes.rs
+++ generated/src/protocol_serde/shape_put_email_identity_configuration_set_attributes.rs
@@ -28,53 +28,47 @@
     Err(match error_code {
         "BadRequestException" => super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::BadRequestException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::BadRequestExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::BadRequestExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
             if tmp.message.is_none() {
-                                                            tmp.message = _error_message;
-                                                        }
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "NotFoundException" => super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::NotFoundException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NotFoundExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output).map_err(super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NotFoundExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output).map_err(super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
             if tmp.message.is_none() {
-                                                            tmp.message = _error_message;
-                                                        }
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "TooManyRequestsException" => super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::TooManyRequestsException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
             if tmp.message.is_none() {
-                                                            tmp.message = _error_message;
-                                                        }
+                tmp.message = _error_message;
+            }
             tmp
         }),
-        _ => super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::generic(generic)
+        _ => super::super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_raw_message.rs`

```diff
--- reference/src/protocol_serde/shape_raw_message.rs
+++ generated/src/protocol_serde/shape_raw_message.rs
@@ -4,7 +4,7 @@
     input: &super::super::types::RawMessage,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     {
-        object.key("Data").string_unchecked(&::aws_smithy_types::base64::encode(&input.data));
+        object.key("Data").string_unchecked(&::aws_smithy_types::base64::encode(input.data));
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_reputation_options.rs`

```diff
--- reference/src/protocol_serde/shape_reputation_options.rs
+++ generated/src/protocol_serde/shape_reputation_options.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ReputationOptions,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.reputation_metrics_enabled {
+    {
         object.key("ReputationMetricsEnabled").boolean(input.reputation_metrics_enabled);
     }
     if let Some(var_1) = &input.last_fresh_start {
```

### `src/protocol_serde/shape_sending_options.rs`

```diff
--- reference/src/protocol_serde/shape_sending_options.rs
+++ generated/src/protocol_serde/shape_sending_options.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::SendingOptions,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.sending_enabled {
+    {
         object.key("SendingEnabled").boolean(input.sending_enabled);
     }
     Ok(())
```

### `src/protocol_serde/shape_tenant_suppression_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_tenant_suppression_attributes.rs
+++ generated/src/protocol_serde/shape_tenant_suppression_attributes.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_tenant_suppression_attributes(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::TenantSuppressionAttributes,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.suppressed_reasons {
+        let mut array_2 = object.key("SuppressedReasons").start_array();
+        for item_3 in var_1 {
+            {
+                array_2.value().string(item_3.as_str());
+            }
+        }
+        array_2.finish();
+    }
+    if let Some(var_4) = &input.suppression_scope {
+        object.key("SuppressionScope").string(var_4.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_tenant_suppression_attributes<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -49,22 +68,3 @@
         )),
     }
 }
-
-pub fn ser_tenant_suppression_attributes(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::TenantSuppressionAttributes,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.suppressed_reasons {
-        let mut array_2 = object.key("SuppressedReasons").start_array();
-        for item_3 in var_1 {
-            {
-                array_2.value().string(item_3.as_str());
-            }
-        }
-        array_2.finish();
-    }
-    if let Some(var_4) = &input.suppression_scope {
-        object.key("SuppressionScope").string(var_4.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_topic_filter.rs`

```diff
--- reference/src/protocol_serde/shape_topic_filter.rs
+++ generated/src/protocol_serde/shape_topic_filter.rs
@@ -6,7 +6,7 @@
     if let Some(var_1) = &input.topic_name {
         object.key("TopicName").string(var_1.as_str());
     }
-    if input.use_default_if_preference_unavailable {
+    {
         object
             .key("UseDefaultIfPreferenceUnavailable")
             .boolean(input.use_default_if_preference_unavailable);
```

### `src/protocol_serde/shape_update_reputation_entity_customer_managed_status.rs`

```diff
--- reference/src/protocol_serde/shape_update_reputation_entity_customer_managed_status.rs
+++ generated/src/protocol_serde/shape_update_reputation_entity_customer_managed_status.rs
@@ -28,53 +28,47 @@
     Err(match error_code {
         "BadRequestException" => super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::BadRequestException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::BadRequestExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::BadRequestExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
             if tmp.message.is_none() {
-                                                            tmp.message = _error_message;
-                                                        }
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::ConflictException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ConflictExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ConflictExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
             if tmp.message.is_none() {
-                                                            tmp.message = _error_message;
-                                                        }
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "TooManyRequestsException" => super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::TooManyRequestsException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
             if tmp.message.is_none() {
-                                                            tmp.message = _error_message;
-                                                        }
+                tmp.message = _error_message;
+            }
             tmp
         }),
-        _ => super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::generic(generic)
+        _ => super::super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::generic(generic),
     })
 }

```

### `src/types/_dkim_attributes.rs`

```diff
--- reference/src/types/_dkim_attributes.rs
+++ generated/src/types/_dkim_attributes.rs
@@ -27,12 +27,9 @@
     pub tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub signing_hosted_zone: ::std::option::Option<::std::string::String>,
     /// <p>A string that indicates how DKIM was configured for the identity. These are the possible values:</p>
     /// <ul>
@@ -138,12 +135,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn signing_hosted_zone(&self) -> ::std::option::Option<&str> {
         self.signing_hosted_zone.as_deref()
     }
@@ -341,12 +335,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn signing_hosted_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.signing_hosted_zone = ::std::option::Option::Some(input.into());
         self
@@ -353,12 +344,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn set_signing_hosted_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.signing_hosted_zone = input;
         self
@@ -365,12 +353,9 @@
     }
     /// <p>The hosted zone where Amazon SES publishes the DKIM public key TXT records for this email identity. This value indicates the DNS zone that customers must reference when configuring their CNAME records for DKIM authentication.</p>
     /// <p>When configuring DKIM for your domain, create CNAME records in your DNS that point to the selectors in this hosted zone. For example:</p>
-    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone>
-    /// </signinghostedzone></code></p>
-    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone>
-    /// </signinghostedzone></code></p>
+    /// <p><code> selector1._domainkey.yourdomain.com CNAME selector1.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector2._domainkey.yourdomain.com CNAME selector2.<signinghostedzone></signinghostedzone> </code></p>
+    /// <p><code> selector3._domainkey.yourdomain.com CNAME selector3.<signinghostedzone></signinghostedzone> </code></p>
     pub fn get_signing_hosted_zone(&self) -> &::std::option::Option<::std::string::String> {
         &self.signing_hosted_zone
     }
```

### `src/types/_import_data_source.rs`

```diff
--- reference/src/types/_import_data_source.rs
+++ generated/src/types/_import_data_source.rs
@@ -4,15 +4,13 @@
 #[non_exhaustive]
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ImportDataSource {
-    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object>.
-    /// <p></p></object></i></p>
+    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object></object></i>.</p>
     pub s3_url: ::std::string::String,
     /// <p>The data format of the import job's data source.</p>
     pub data_format: super::super::types::DataFormat,
 }
 impl ImportDataSource {
-    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object>.
-    /// <p></p></object></i></p>
+    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object></object></i>.</p>
     pub fn s3_url(&self) -> &str {
         use std::ops::Deref;
         self.s3_url.deref()
@@ -37,21 +35,18 @@
     pub(crate) data_format: ::std::option::Option<super::super::types::DataFormat>,
 }
 impl ImportDataSourceBuilder {
-    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object>.
-    /// <p></p></object></i></p>
+    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object></object></i>.</p>
     /// This field is required.
     pub fn s3_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.s3_url = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object>.
-    /// <p></p></object></i></p>
+    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object></object></i>.</p>
     pub fn set_s3_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.s3_url = input;
         self
     }
-    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object>.
-    /// <p></p></object></i></p>
+    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object></object></i>.</p>
     pub fn get_s3_url(&self) -> &::std::option::Option<::std::string::String> {
         &self.s3_url
     }
```

### `src/types/_message_insights_filters.rs`

```diff
--- reference/src/types/_message_insights_filters.rs
+++ generated/src/types/_message_insights_filters.rs
@@ -5,7 +5,7 @@
 /// <p>If you specify multiple values for a filter, the values are joined by OR. Filter values are case-sensitive.</p>
 /// <p><code>FromEmailAddress</code>, <code>Destination</code>, and <code>Subject</code> filters support partial match. A partial match is performed by using the <code>*</code> wildcard character placed at the beginning (suffix match), the end (prefix match) or both ends of the string (contains match). In order to match the literal characters <code>*</code> or <code>\</code>, they must be escaped using the <code>\</code> character. If no wildcard character is present, an exact match is performed.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct MessageInsightsFilters {
     /// <p>The from address used to send the message.</p>
     pub from_email_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -60,6 +60,18 @@
         self.last_engagement_event.as_deref().unwrap_or_default()
     }
 }
+impl ::std::fmt::Debug for MessageInsightsFilters {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("MessageInsightsFilters");
+        formatter.field("from_email_address", &"*** Sensitive Data Redacted ***");
+        formatter.field("destination", &"*** Sensitive Data Redacted ***");
+        formatter.field("subject", &"*** Sensitive Data Redacted ***");
+        formatter.field("isp", &self.isp);
+        formatter.field("last_delivery_event", &self.last_delivery_event);
+        formatter.field("last_engagement_event", &self.last_engagement_event);
+        formatter.finish()
+    }
+}
 impl MessageInsightsFilters {
     /// Creates a new builder-style object to manufacture [`MessageInsightsFilters`](crate::types::MessageInsightsFilters).
     pub fn builder() -> super::super::types::builders::MessageInsightsFiltersBuilder {
@@ -68,7 +80,7 @@
 }

 /// A builder for [`MessageInsightsFilters`](crate::types::MessageInsightsFilters).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct MessageInsightsFiltersBuilder {
     pub(crate) from_email_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -214,3 +226,15 @@
         }
     }
 }
+impl ::std::fmt::Debug for MessageInsightsFiltersBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("MessageInsightsFiltersBuilder");
+        formatter.field("from_email_address", &"*** Sensitive Data Redacted ***");
+        formatter.field("destination", &"*** Sensitive Data Redacted ***");
+        formatter.field("subject", &"*** Sensitive Data Redacted ***");
+        formatter.field("isp", &self.isp);
+        formatter.field("last_delivery_event", &self.last_delivery_event);
+        formatter.field("last_engagement_event", &self.last_engagement_event);
+        formatter.finish()
+    }
+}
```

### Missing reference files

- `src/endpoint_auth.rs`
