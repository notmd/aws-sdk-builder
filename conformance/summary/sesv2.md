# AWS SDK Conformance Report: sesv2

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sesv2
**Progress:** `1159/1159` files compared · `1060` matched · `98` mismatches · `1` missing · `0` extra · `91.46%` match (100.00% means fully matched)

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

### `src/operation/create_contact/_create_contact_input.rs`

```diff
--- reference/src/operation/create_contact/_create_contact_input.rs
+++ generated/src/operation/create_contact/_create_contact_input.rs
@@ -142,7 +142,7 @@
             contact_list_name: self.contact_list_name,
             email_address: self.email_address,
             topic_preferences: self.topic_preferences,
-            unsubscribe_all: self.unsubscribe_all,
+            unsubscribe_all: self.unsubscribe_all.unwrap_or_default(),
             attributes_data: self.attributes_data,
         })
     }
```

### `src/operation/get_blacklist_reports.rs`

```diff
--- reference/src/operation/get_blacklist_reports.rs
+++ generated/src/operation/get_blacklist_reports.rs
@@ -203,9 +203,7 @@
                 let inner_1 = inner_1.as_ref().ok_or_else(|| {
                     ::aws_smithy_types::error::operation::BuildError::missing_field("blacklist_item_names", "cannot be empty or unset")
                 })?;
-                for inner_2 in inner_1 {
-                    query.push_kv("BlacklistItemNames", &::aws_smithy_http::query::fmt_string(inner_2));
-                }
+                query.push_kv("BlacklistItemNames", ::aws_smithy_types::primitive::Encoder::from(*inner_1).encode());
                 ::std::result::Result::Ok(())
             }
             #[allow(clippy::unnecessary_wraps)]
```

### `src/operation/get_domain_statistics_report.rs`

```diff
--- reference/src/operation/get_domain_statistics_report.rs
+++ generated/src/operation/get_domain_statistics_report.rs
@@ -263,7 +263,7 @@
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("start_date", "cannot be empty or unset"))?;
                 query.push_kv(
                     "StartDate",
-                    &::aws_smithy_http::query::fmt_timestamp(inner_2, ::aws_smithy_types::date_time::Format::DateTime)?,
+                    &::aws_smithy_http::query::fmt_timestamp(inner_2, ::aws_smithy_types::date_time::Format::HttpDate)?,
                 );
                 let inner_3 = &_input.end_date;
                 let inner_3 = inner_3
@@ -271,7 +271,7 @@
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("end_date", "cannot be empty or unset"))?;
                 query.push_kv(
                     "EndDate",
-                    &::aws_smithy_http::query::fmt_timestamp(inner_3, ::aws_smithy_types::date_time::Format::DateTime)?,
+                    &::aws_smithy_http::query::fmt_timestamp(inner_3, ::aws_smithy_types::date_time::Format::HttpDate)?,
                 );
                 ::std::result::Result::Ok(())
             }
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

### `src/operation/list_domain_deliverability_campaigns.rs`

```diff
--- reference/src/operation/list_domain_deliverability_campaigns.rs
+++ generated/src/operation/list_domain_deliverability_campaigns.rs
@@ -282,7 +282,7 @@
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("start_date", "cannot be empty or unset"))?;
                 query.push_kv(
                     "StartDate",
-                    &::aws_smithy_http::query::fmt_timestamp(inner_2, ::aws_smithy_types::date_time::Format::DateTime)?,
+                    &::aws_smithy_http::query::fmt_timestamp(inner_2, ::aws_smithy_types::date_time::Format::HttpDate)?,
                 );
                 let inner_3 = &_input.end_date;
                 let inner_3 = inner_3
@@ -290,7 +290,7 @@
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("end_date", "cannot be empty or unset"))?;
                 query.push_kv(
                     "EndDate",
-                    &::aws_smithy_http::query::fmt_timestamp(inner_3, ::aws_smithy_types::date_time::Format::DateTime)?,
+                    &::aws_smithy_http::query::fmt_timestamp(inner_3, ::aws_smithy_types::date_time::Format::HttpDate)?,
                 );
                 if let ::std::option::Option::Some(inner_4) = &_input.next_token {
                     {
```

### `src/operation/list_suppressed_destinations.rs`

```diff
--- reference/src/operation/list_suppressed_destinations.rs
+++ generated/src/operation/list_suppressed_destinations.rs
@@ -257,35 +257,33 @@
                 }
                 if let ::std::option::Option::Some(inner_2) = &_input.reasons {
                     {
-                        for inner_3 in inner_2 {
-                            query.push_kv("Reason", &::aws_smithy_http::query::fmt_string(inner_3.as_str()));
-                        }
+                        query.push_kv("Reason", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
                     }
                 }
-                if let ::std::option::Option::Some(inner_4) = &_input.start_date {
+                if let ::std::option::Option::Some(inner_3) = &_input.start_date {
                     {
                         query.push_kv(
                             "StartDate",
-                            &::aws_smithy_http::query::fmt_timestamp(inner_4, ::aws_smithy_types::date_time::Format::DateTime)?,
+                            &::aws_smithy_http::query::fmt_timestamp(inner_3, ::aws_smithy_types::date_time::Format::HttpDate)?,
                         );
                     }
                 }
-                if let ::std::option::Option::Some(inner_5) = &_input.end_date {
+                if let ::std::option::Option::Some(inner_4) = &_input.end_date {
                     {
                         query.push_kv(
                             "EndDate",
-                            &::aws_smithy_http::query::fmt_timestamp(inner_5, ::aws_smithy_types::date_time::Format::DateTime)?,
+                            &::aws_smithy_http::query::fmt_timestamp(inner_4, ::aws_smithy_types::date_time::Format::HttpDate)?,
                         );
                     }
                 }
-                if let ::std::option::Option::Some(inner_6) = &_input.next_token {
+                if let ::std::option::Option::Some(inner_5) = &_input.next_token {
                     {
-                        query.push_kv("NextToken", &::aws_smithy_http::query::fmt_string(inner_6));
+                        query.push_kv("NextToken", &::aws_smithy_http::query::fmt_string(inner_5));
                     }
                 }
-                if let ::std::option::Option::Some(inner_7) = &_input.page_size {
+                if let ::std::option::Option::Some(inner_6) = &_input.page_size {
                     {
-                        query.push_kv("PageSize", ::aws_smithy_types::primitive::Encoder::from(*inner_7).encode());
+                        query.push_kv("PageSize", ::aws_smithy_types::primitive::Encoder::from(*inner_6).encode());
                     }
                 }
                 ::std::result::Result::Ok(())
```

### `src/operation/put_account_dedicated_ip_warmup_attributes/_put_account_dedicated_ip_warmup_attributes_input.rs`

```diff
--- reference/src/operation/put_account_dedicated_ip_warmup_attributes/_put_account_dedicated_ip_warmup_attributes_input.rs
+++ generated/src/operation/put_account_dedicated_ip_warmup_attributes/_put_account_dedicated_ip_warmup_attributes_input.rs
@@ -50,7 +50,7 @@
     > {
         ::std::result::Result::Ok(
             super::super::super::operation::put_account_dedicated_ip_warmup_attributes::PutAccountDedicatedIpWarmupAttributesInput {
-                auto_warmup_enabled: self.auto_warmup_enabled,
+                auto_warmup_enabled: self.auto_warmup_enabled.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/put_account_sending_attributes/_put_account_sending_attributes_input.rs`

```diff
--- reference/src/operation/put_account_sending_attributes/_put_account_sending_attributes_input.rs
+++ generated/src/operation/put_account_sending_attributes/_put_account_sending_attributes_input.rs
@@ -59,7 +59,7 @@
         ::aws_smithy_types::error::operation::BuildError,
     > {
         ::std::result::Result::Ok(super::super::super::operation::put_account_sending_attributes::PutAccountSendingAttributesInput {
-            sending_enabled: self.sending_enabled,
+            sending_enabled: self.sending_enabled.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/put_configuration_set_reputation_options/_put_configuration_set_reputation_options_input.rs`

```diff
--- reference/src/operation/put_configuration_set_reputation_options/_put_configuration_set_reputation_options_input.rs
+++ generated/src/operation/put_configuration_set_reputation_options/_put_configuration_set_reputation_options_input.rs
@@ -73,7 +73,7 @@
         ::std::result::Result::Ok(
             super::super::super::operation::put_configuration_set_reputation_options::PutConfigurationSetReputationOptionsInput {
                 configuration_set_name: self.configuration_set_name,
-                reputation_metrics_enabled: self.reputation_metrics_enabled,
+                reputation_metrics_enabled: self.reputation_metrics_enabled.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/put_configuration_set_sending_options/_put_configuration_set_sending_options_input.rs`

```diff
--- reference/src/operation/put_configuration_set_sending_options/_put_configuration_set_sending_options_input.rs
+++ generated/src/operation/put_configuration_set_sending_options/_put_configuration_set_sending_options_input.rs
@@ -73,7 +73,7 @@
         ::std::result::Result::Ok(
             super::super::super::operation::put_configuration_set_sending_options::PutConfigurationSetSendingOptionsInput {
                 configuration_set_name: self.configuration_set_name,
-                sending_enabled: self.sending_enabled,
+                sending_enabled: self.sending_enabled.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/put_deliverability_dashboard_option/_put_deliverability_dashboard_option_input.rs`

```diff
--- reference/src/operation/put_deliverability_dashboard_option/_put_deliverability_dashboard_option_input.rs
+++ generated/src/operation/put_deliverability_dashboard_option/_put_deliverability_dashboard_option_input.rs
@@ -81,7 +81,7 @@
     > {
         ::std::result::Result::Ok(
             super::super::super::operation::put_deliverability_dashboard_option::PutDeliverabilityDashboardOptionInput {
-                dashboard_enabled: self.dashboard_enabled,
+                dashboard_enabled: self.dashboard_enabled.unwrap_or_default(),
                 subscribed_domains: self.subscribed_domains,
             },
         )
```

### `src/operation/put_email_identity_dkim_attributes/_put_email_identity_dkim_attributes_input.rs`

```diff
--- reference/src/operation/put_email_identity_dkim_attributes/_put_email_identity_dkim_attributes_input.rs
+++ generated/src/operation/put_email_identity_dkim_attributes/_put_email_identity_dkim_attributes_input.rs
@@ -78,7 +78,7 @@
         ::std::result::Result::Ok(
             super::super::super::operation::put_email_identity_dkim_attributes::PutEmailIdentityDkimAttributesInput {
                 email_identity: self.email_identity,
-                signing_enabled: self.signing_enabled,
+                signing_enabled: self.signing_enabled.unwrap_or_default(),
             },
         )
     }
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

### `src/operation/put_email_identity_feedback_attributes/_put_email_identity_feedback_attributes_input.rs`

```diff
--- reference/src/operation/put_email_identity_feedback_attributes/_put_email_identity_feedback_attributes_input.rs
+++ generated/src/operation/put_email_identity_feedback_attributes/_put_email_identity_feedback_attributes_input.rs
@@ -83,7 +83,7 @@
         ::std::result::Result::Ok(
             super::super::super::operation::put_email_identity_feedback_attributes::PutEmailIdentityFeedbackAttributesInput {
                 email_identity: self.email_identity,
-                email_forwarding_enabled: self.email_forwarding_enabled,
+                email_forwarding_enabled: self.email_forwarding_enabled.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -257,9 +257,7 @@
                 let inner_2 = inner_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("tag_keys", "cannot be empty or unset"))?;
-                for inner_3 in inner_2 {
-                    query.push_kv("TagKeys", &::aws_smithy_http::query::fmt_string(inner_3));
-                }
+                query.push_kv("TagKeys", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
                 ::std::result::Result::Ok(())
             }
             #[allow(clippy::unnecessary_wraps)]
```

### `src/operation/update_contact/_update_contact_input.rs`

```diff
--- reference/src/operation/update_contact/_update_contact_input.rs
+++ generated/src/operation/update_contact/_update_contact_input.rs
@@ -142,7 +142,7 @@
             contact_list_name: self.contact_list_name,
             email_address: self.email_address,
             topic_preferences: self.topic_preferences,
-            unsubscribe_all: self.unsubscribe_all,
+            unsubscribe_all: self.unsubscribe_all.unwrap_or_default(),
             attributes_data: self.attributes_data,
         })
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

### `src/protocol_serde/shape_batch_get_metric_data.rs`

```diff
--- reference/src/protocol_serde/shape_batch_get_metric_data.rs
+++ generated/src/protocol_serde/shape_batch_get_metric_data.rs
@@ -132,15 +132,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Errors" => {
-                    builder = builder.set_errors(super::super::protocol_serde::shape_metric_data_error_list::de_metric_data_error_list(
+                "Results" => {
+                    builder = builder.set_results(super::super::protocol_serde::shape_metric_data_result_list::de_metric_data_result_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "Results" => {
-                    builder = builder.set_results(super::super::protocol_serde::shape_metric_data_result_list::de_metric_data_result_list(
+                "Errors" => {
+                    builder = builder.set_errors(super::super::protocol_serde::shape_metric_data_error_list::de_metric_data_error_list(
                         tokens,
                         _value,
                         depth + 1,
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

### `src/protocol_serde/shape_create_configuration_set_event_destination_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_configuration_set_event_destination_input.rs
+++ generated/src/protocol_serde/shape_create_configuration_set_event_destination_input.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_configuration_set_event_destination::CreateConfigurationSetEventDestinationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.event_destination {
+    if let Some(var_1) = &input.event_destination_name {
+        object.key("EventDestinationName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.event_destination {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("EventDestination").start_object();
-        super::super::protocol_serde::shape_event_destination_definition::ser_event_destination_definition(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.event_destination_name {
-        object.key("EventDestinationName").string(var_3.as_str());
+        let mut object_3 = object.key("EventDestination").start_object();
+        super::super::protocol_serde::shape_event_destination_definition::ser_event_destination_definition(&mut object_3, var_2)?;
+        object_3.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_configuration_set_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_configuration_set_input.rs
+++ generated/src/protocol_serde/shape_create_configuration_set_input.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_configuration_set::CreateConfigurationSetInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.archiving_options {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("ArchivingOptions").start_object();
-        super::super::protocol_serde::shape_archiving_options::ser_archiving_options(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_3.as_str());
+    if let Some(var_2) = &input.tracking_options {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("TrackingOptions").start_object();
+        super::super::protocol_serde::shape_tracking_options::ser_tracking_options(&mut object_3, var_2)?;
+        object_3.finish();
     }
     if let Some(var_4) = &input.delivery_options {
         #[allow(unused_mut)]
@@ -30,34 +30,34 @@
         super::super::protocol_serde::shape_sending_options::ser_sending_options(&mut object_9, var_8)?;
         object_9.finish();
     }
-    if let Some(var_10) = &input.suppression_options {
-        #[allow(unused_mut)]
-        let mut object_11 = object.key("SuppressionOptions").start_object();
-        super::super::protocol_serde::shape_suppression_options::ser_suppression_options(&mut object_11, var_10)?;
-        object_11.finish();
-    }
-    if let Some(var_12) = &input.tags {
-        let mut array_13 = object.key("Tags").start_array();
-        for item_14 in var_12 {
+    if let Some(var_10) = &input.tags {
+        let mut array_11 = object.key("Tags").start_array();
+        for item_12 in var_10 {
             {
                 #[allow(unused_mut)]
-                let mut object_15 = array_13.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_15, item_14)?;
-                object_15.finish();
+                let mut object_13 = array_11.value().start_object();
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
+                object_13.finish();
             }
         }
-        array_13.finish();
+        array_11.finish();
+    }
+    if let Some(var_14) = &input.suppression_options {
+        #[allow(unused_mut)]
+        let mut object_15 = object.key("SuppressionOptions").start_object();
+        super::super::protocol_serde::shape_suppression_options::ser_suppression_options(&mut object_15, var_14)?;
+        object_15.finish();
     }
-    if let Some(var_16) = &input.tracking_options {
+    if let Some(var_16) = &input.vdm_options {
         #[allow(unused_mut)]
-        let mut object_17 = object.key("TrackingOptions").start_object();
-        super::super::protocol_serde::shape_tracking_options::ser_tracking_options(&mut object_17, var_16)?;
+        let mut object_17 = object.key("VdmOptions").start_object();
+        super::super::protocol_serde::shape_vdm_options::ser_vdm_options(&mut object_17, var_16)?;
         object_17.finish();
     }
-    if let Some(var_18) = &input.vdm_options {
+    if let Some(var_18) = &input.archiving_options {
         #[allow(unused_mut)]
-        let mut object_19 = object.key("VdmOptions").start_object();
-        super::super::protocol_serde::shape_vdm_options::ser_vdm_options(&mut object_19, var_18)?;
+        let mut object_19 = object.key("ArchivingOptions").start_object();
+        super::super::protocol_serde::shape_archiving_options::ser_archiving_options(&mut object_19, var_18)?;
         object_19.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_create_contact_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_contact_input.rs
+++ generated/src/protocol_serde/shape_create_contact_input.rs
@@ -3,26 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_contact::CreateContactInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.attributes_data {
-        object.key("AttributesData").string(var_1.as_str());
+    if let Some(var_1) = &input.email_address {
+        object.key("EmailAddress").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.email_address {
-        object.key("EmailAddress").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.topic_preferences {
-        let mut array_4 = object.key("TopicPreferences").start_array();
-        for item_5 in var_3 {
+    if let Some(var_2) = &input.topic_preferences {
+        let mut array_3 = object.key("TopicPreferences").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_6 = array_4.value().start_object();
-                super::super::protocol_serde::shape_topic_preference::ser_topic_preference(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_topic_preference::ser_topic_preference(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_4.finish();
+        array_3.finish();
+    }
+    if let Some(var_6) = &input.unsubscribe_all {
+        object.key("UnsubscribeAll").boolean(*var_6);
     }
-    if let Some(var_7) = &input.unsubscribe_all {
-        object.key("UnsubscribeAll").boolean(*var_7);
+    if let Some(var_7) = &input.attributes_data {
+        object.key("AttributesData").string(var_7.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_contact_list_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_contact_list_input.rs
+++ generated/src/protocol_serde/shape_create_contact_list_input.rs
@@ -6,28 +6,28 @@
     if let Some(var_1) = &input.contact_list_name {
         object.key("ContactListName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.description {
-        object.key("Description").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.tags {
-        let mut array_4 = object.key("Tags").start_array();
-        for item_5 in var_3 {
+    if let Some(var_2) = &input.topics {
+        let mut array_3 = object.key("Topics").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_6 = array_4.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_topic::ser_topic(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_4.finish();
+        array_3.finish();
     }
-    if let Some(var_7) = &input.topics {
-        let mut array_8 = object.key("Topics").start_array();
+    if let Some(var_6) = &input.description {
+        object.key("Description").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.tags {
+        let mut array_8 = object.key("Tags").start_array();
         for item_9 in var_7 {
             {
                 #[allow(unused_mut)]
                 let mut object_10 = array_8.value().start_object();
-                super::super::protocol_serde::shape_topic::ser_topic(&mut object_10, item_9)?;
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                 object_10.finish();
             }
         }
```

### `src/protocol_serde/shape_create_custom_verification_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_custom_verification_email_template_input.rs
+++ generated/src/protocol_serde/shape_create_custom_verification_email_template_input.rs
@@ -3,35 +3,35 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_custom_verification_email_template::CreateCustomVerificationEmailTemplateInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.failure_redirection_url {
-        object.key("FailureRedirectionURL").string(var_1.as_str());
+    if let Some(var_1) = &input.template_name {
+        object.key("TemplateName").string(var_1.as_str());
     }
     if let Some(var_2) = &input.from_email_address {
         object.key("FromEmailAddress").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.success_redirection_url {
-        object.key("SuccessRedirectionURL").string(var_3.as_str());
+    if let Some(var_3) = &input.template_subject {
+        object.key("TemplateSubject").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.template_content {
+        object.key("TemplateContent").string(var_4.as_str());
     }
-    if let Some(var_4) = &input.tags {
-        let mut array_5 = object.key("Tags").start_array();
-        for item_6 in var_4 {
+    if let Some(var_5) = &input.tags {
+        let mut array_6 = object.key("Tags").start_array();
+        for item_7 in var_5 {
             {
                 #[allow(unused_mut)]
-                let mut object_7 = array_5.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
-                object_7.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
+                object_8.finish();
             }
         }
-        array_5.finish();
+        array_6.finish();
     }
-    if let Some(var_8) = &input.template_content {
-        object.key("TemplateContent").string(var_8.as_str());
-    }
-    if let Some(var_9) = &input.template_name {
-        object.key("TemplateName").string(var_9.as_str());
+    if let Some(var_9) = &input.success_redirection_url {
+        object.key("SuccessRedirectionURL").string(var_9.as_str());
     }
-    if let Some(var_10) = &input.template_subject {
-        object.key("TemplateSubject").string(var_10.as_str());
+    if let Some(var_10) = &input.failure_redirection_url {
+        object.key("FailureRedirectionURL").string(var_10.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_dedicated_ip_pool_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_dedicated_ip_pool_input.rs
+++ generated/src/protocol_serde/shape_create_dedicated_ip_pool_input.rs
@@ -6,20 +6,20 @@
     if let Some(var_1) = &input.pool_name {
         object.key("PoolName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.scaling_mode {
-        object.key("ScalingMode").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.tags {
-        let mut array_4 = object.key("Tags").start_array();
-        for item_5 in var_3 {
+    if let Some(var_2) = &input.tags {
+        let mut array_3 = object.key("Tags").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_6 = array_4.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_4.finish();
+        array_3.finish();
+    }
+    if let Some(var_6) = &input.scaling_mode {
+        object.key("ScalingMode").string(var_6.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_deliverability_test_report.rs`

```diff
--- reference/src/protocol_serde/shape_create_deliverability_test_report.rs
+++ generated/src/protocol_serde/shape_create_deliverability_test_report.rs
@@ -227,17 +227,17 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "DeliverabilityTestStatus" => {
-                    builder = builder.set_deliverability_test_status(
+                "ReportId" => {
+                    builder = builder.set_report_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::super::types::DeliverabilityTestStatus::from(u.as_ref())))
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "ReportId" => {
-                    builder = builder.set_report_id(
+                "DeliverabilityTestStatus" => {
+                    builder = builder.set_deliverability_test_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::super::types::DeliverabilityTestStatus::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
```

### `src/protocol_serde/shape_create_deliverability_test_report_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_deliverability_test_report_input.rs
+++ generated/src/protocol_serde/shape_create_deliverability_test_report_input.rs
@@ -3,17 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_deliverability_test_report::CreateDeliverabilityTestReportInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.content {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("Content").start_object();
-        super::super::protocol_serde::shape_email_content::ser_email_content(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.report_name {
+        object.key("ReportName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.from_email_address {
-        object.key("FromEmailAddress").string(var_3.as_str());
+    if let Some(var_2) = &input.from_email_address {
+        object.key("FromEmailAddress").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.report_name {
-        object.key("ReportName").string(var_4.as_str());
+    if let Some(var_3) = &input.content {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("Content").start_object();
+        super::super::protocol_serde::shape_email_content::ser_email_content(&mut object_4, var_3)?;
+        object_4.finish();
     }
     if let Some(var_5) = &input.tags {
         let mut array_6 = object.key("Tags").start_array();
```

### `src/protocol_serde/shape_create_email_identity.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_identity.rs
+++ generated/src/protocol_serde/shape_create_email_identity.rs
@@ -162,13 +162,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "DkimAttributes" => {
-                    builder = builder.set_dkim_attributes(super::super::protocol_serde::shape_dkim_attributes::de_dkim_attributes(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "IdentityType" => {
                     builder = builder.set_identity_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -179,6 +172,13 @@
                 "VerifiedForSendingStatus" => {
                     builder = builder.set_verified_for_sending_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                 }
+                "DkimAttributes" => {
+                    builder = builder.set_dkim_attributes(super::super::protocol_serde::shape_dkim_attributes::de_dkim_attributes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_create_email_identity_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_identity_input.rs
+++ generated/src/protocol_serde/shape_create_email_identity_input.rs
@@ -3,29 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_email_identity::CreateEmailIdentityInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_1.as_str());
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.dkim_signing_attributes {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("DkimSigningAttributes").start_object();
-        super::super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_3, var_2)?;
-        object_3.finish();
-    }
-    if let Some(var_4) = &input.email_identity {
-        object.key("EmailIdentity").string(var_4.as_str());
-    }
-    if let Some(var_5) = &input.tags {
-        let mut array_6 = object.key("Tags").start_array();
-        for item_7 in var_5 {
+    if let Some(var_2) = &input.tags {
+        let mut array_3 = object.key("Tags").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_8 = array_6.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
-                object_8.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_6.finish();
+        array_3.finish();
+    }
+    if let Some(var_6) = &input.dkim_signing_attributes {
+        #[allow(unused_mut)]
+        let mut object_7 = object.key("DkimSigningAttributes").start_object();
+        super::super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_7, var_6)?;
+        object_7.finish();
+    }
+    if let Some(var_8) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_8.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_template_input.rs
+++ generated/src/protocol_serde/shape_create_email_template_input.rs
@@ -3,26 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_email_template::CreateEmailTemplateInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.tags {
-        let mut array_2 = object.key("Tags").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.template_name {
+        object.key("TemplateName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.template_content {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("TemplateContent").start_object();
+        super::super::protocol_serde::shape_email_template_content::ser_email_template_content(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    if let Some(var_4) = &input.tags {
+        let mut array_5 = object.key("Tags").start_array();
+        for item_6 in var_4 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_7 = array_5.value().start_object();
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
+                object_7.finish();
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_5) = &input.template_content {
-        #[allow(unused_mut)]
-        let mut object_6 = object.key("TemplateContent").start_object();
-        super::super::protocol_serde::shape_email_template_content::ser_email_template_content(&mut object_6, var_5)?;
-        object_6.finish();
-    }
-    if let Some(var_7) = &input.template_name {
-        object.key("TemplateName").string(var_7.as_str());
+        array_5.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_import_job_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_import_job_input.rs
+++ generated/src/protocol_serde/shape_create_import_job_input.rs
@@ -3,16 +3,16 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_import_job::CreateImportJobInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.import_data_source {
+    if let Some(var_1) = &input.import_destination {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("ImportDataSource").start_object();
-        super::super::protocol_serde::shape_import_data_source::ser_import_data_source(&mut object_2, var_1)?;
+        let mut object_2 = object.key("ImportDestination").start_object();
+        super::super::protocol_serde::shape_import_destination::ser_import_destination(&mut object_2, var_1)?;
         object_2.finish();
     }
-    if let Some(var_3) = &input.import_destination {
+    if let Some(var_3) = &input.import_data_source {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("ImportDestination").start_object();
-        super::super::protocol_serde::shape_import_destination::ser_import_destination(&mut object_4, var_3)?;
+        let mut object_4 = object.key("ImportDataSource").start_object();
+        super::super::protocol_serde::shape_import_data_source::ser_import_data_source(&mut object_4, var_3)?;
         object_4.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_create_multi_region_endpoint.rs`

```diff
--- reference/src/protocol_serde/shape_create_multi_region_endpoint.rs
+++ generated/src/protocol_serde/shape_create_multi_region_endpoint.rs
@@ -133,17 +133,17 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "EndpointId" => {
-                    builder = builder.set_endpoint_id(
+                "Status" => {
+                    builder = builder.set_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::super::types::Status::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "Status" => {
-                    builder = builder.set_status(
+                "EndpointId" => {
+                    builder = builder.set_endpoint_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::super::types::Status::from(u.as_ref())))
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
```

### `src/protocol_serde/shape_create_multi_region_endpoint_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_multi_region_endpoint_input.rs
+++ generated/src/protocol_serde/shape_create_multi_region_endpoint_input.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_multi_region_endpoint::CreateMultiRegionEndpointInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.details {
+    if let Some(var_1) = &input.endpoint_name {
+        object.key("EndpointName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.details {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("Details").start_object();
-        super::super::protocol_serde::shape_details::ser_details(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.endpoint_name {
-        object.key("EndpointName").string(var_3.as_str());
+        let mut object_3 = object.key("Details").start_object();
+        super::super::protocol_serde::shape_details::ser_details(&mut object_3, var_2)?;
+        object_3.finish();
     }
     if let Some(var_4) = &input.tags {
         let mut array_5 = object.key("Tags").start_array();
```

### `src/protocol_serde/shape_create_tenant.rs`

```diff
--- reference/src/protocol_serde/shape_create_tenant.rs
+++ generated/src/protocol_serde/shape_create_tenant.rs
@@ -123,27 +123,20 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "SendingStatus" => {
-                    builder = builder.set_sending_status(
+                "TenantName" => {
+                    builder = builder.set_tenant_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::super::types::SendingStatus::from(u.as_ref())))
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "SuppressionAttributes" => {
-                    builder = builder.set_suppression_attributes(
-                        super::super::protocol_serde::shape_tenant_suppression_attributes::de_tenant_suppression_attributes(tokens, _value, depth + 1)?,
+                "TenantId" => {
+                    builder = builder.set_tenant_id(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "Tags" => {
-                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
-                }
                 "TenantArn" => {
                     builder = builder.set_tenant_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -151,18 +144,25 @@
                             .transpose()?,
                     );
                 }
-                "TenantId" => {
-                    builder = builder.set_tenant_id(
+                "CreatedTimestamp" => {
+                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
+                "Tags" => {
+                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                }
+                "SendingStatus" => {
+                    builder = builder.set_sending_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::super::types::SendingStatus::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "TenantName" => {
-                    builder = builder.set_tenant_name(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "SuppressionAttributes" => {
+                    builder = builder.set_suppression_attributes(
+                        super::super::protocol_serde::shape_tenant_suppression_attributes::de_tenant_suppression_attributes(tokens, _value, depth + 1)?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_create_tenant_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_tenant_input.rs
+++ generated/src/protocol_serde/shape_create_tenant_input.rs
@@ -3,26 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_tenant::CreateTenantInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.suppression_attributes {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("SuppressionAttributes").start_object();
-        super::super::protocol_serde::shape_tenant_suppression_attributes::ser_tenant_suppression_attributes(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.tenant_name {
+        object.key("TenantName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.tags {
-        let mut array_4 = object.key("Tags").start_array();
-        for item_5 in var_3 {
+    if let Some(var_2) = &input.tags {
+        let mut array_3 = object.key("Tags").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_6 = array_4.value().start_object();
-                super::super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_4.finish();
+        array_3.finish();
     }
-    if let Some(var_7) = &input.tenant_name {
-        object.key("TenantName").string(var_7.as_str());
+    if let Some(var_6) = &input.suppression_attributes {
+        #[allow(unused_mut)]
+        let mut object_7 = object.key("SuppressionAttributes").start_object();
+        super::super::protocol_serde::shape_tenant_suppression_attributes::ser_tenant_suppression_attributes(&mut object_7, var_6)?;
+        object_7.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_tenant_resource_association_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_tenant_resource_association_input.rs
+++ generated/src/protocol_serde/shape_create_tenant_resource_association_input.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_tenant_resource_association::CreateTenantResourceAssociationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.resource_arn {
-        object.key("ResourceArn").string(var_1.as_str());
+    if let Some(var_1) = &input.tenant_name {
+        object.key("TenantName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.tenant_name {
-        object.key("TenantName").string(var_2.as_str());
+    if let Some(var_2) = &input.resource_arn {
+        object.key("ResourceArn").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_dashboard_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_dashboard_attributes.rs
+++ generated/src/protocol_serde/shape_dashboard_attributes.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_dashboard_attributes(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::DashboardAttributes,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.engagement_metrics {
+        object.key("EngagementMetrics").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_dashboard_attributes<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_dashboard_attributes(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::DashboardAttributes,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.engagement_metrics {
-        object.key("EngagementMetrics").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_delete_tenant_resource_association_input.rs`

```diff
--- reference/src/protocol_serde/shape_delete_tenant_resource_association_input.rs
+++ generated/src/protocol_serde/shape_delete_tenant_resource_association_input.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.resource_arn {
-        object.key("ResourceArn").string(var_1.as_str());
+    if let Some(var_1) = &input.tenant_name {
+        object.key("TenantName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.tenant_name {
-        object.key("TenantName").string(var_2.as_str());
+    if let Some(var_2) = &input.resource_arn {
+        object.key("ResourceArn").string(var_2.as_str());
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

### `src/protocol_serde/shape_get_account.rs`

```diff
--- reference/src/protocol_serde/shape_get_account.rs
+++ generated/src/protocol_serde/shape_get_account.rs
@@ -85,13 +85,6 @@
                     builder =
                         builder.set_dedicated_ip_auto_warmup_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                 }
-                "Details" => {
-                    builder = builder.set_details(super::super::protocol_serde::shape_account_details::de_account_details(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "EnforcementStatus" => {
                     builder = builder.set_enforcement_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -99,13 +92,6 @@
                             .transpose()?,
                     );
                 }
-                "PricingAttributes" => {
-                    builder = builder.set_pricing_attributes(super::super::protocol_serde::shape_pricing_attributes::de_pricing_attributes(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "ProductionAccessEnabled" => {
                     builder = builder.set_production_access_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                 }
@@ -122,9 +108,23 @@
                         depth + 1,
                     )?);
                 }
+                "Details" => {
+                    builder = builder.set_details(super::super::protocol_serde::shape_account_details::de_account_details(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "VdmAttributes" => {
                     builder = builder.set_vdm_attributes(super::super::protocol_serde::shape_vdm_attributes::de_vdm_attributes(tokens, _value, depth + 1)?);
                 }
+                "PricingAttributes" => {
+                    builder = builder.set_pricing_attributes(super::super::protocol_serde::shape_pricing_attributes::de_pricing_attributes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_configuration_set.rs`

```diff
--- reference/src/protocol_serde/shape_get_configuration_set.rs
+++ generated/src/protocol_serde/shape_get_configuration_set.rs
@@ -104,13 +104,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "ArchivingOptions" => {
-                    builder = builder.set_archiving_options(super::super::protocol_serde::shape_archiving_options::de_archiving_options(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "ConfigurationSetName" => {
                     builder = builder.set_configuration_set_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -118,6 +111,13 @@
                             .transpose()?,
                     );
                 }
+                "TrackingOptions" => {
+                    builder = builder.set_tracking_options(super::super::protocol_serde::shape_tracking_options::de_tracking_options(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "DeliveryOptions" => {
                     builder = builder.set_delivery_options(super::super::protocol_serde::shape_delivery_options::de_delivery_options(
                         tokens,
@@ -139,6 +139,9 @@
                         depth + 1,
                     )?);
                 }
+                "Tags" => {
+                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                }
                 "SuppressionOptions" => {
                     builder = builder.set_suppression_options(super::super::protocol_serde::shape_suppression_options::de_suppression_options(
                         tokens,
@@ -146,19 +149,16 @@
                         depth + 1,
                     )?);
                 }
-                "Tags" => {
-                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                "VdmOptions" => {
+                    builder = builder.set_vdm_options(super::super::protocol_serde::shape_vdm_options::de_vdm_options(tokens, _value, depth + 1)?);
                 }
-                "TrackingOptions" => {
-                    builder = builder.set_tracking_options(super::super::protocol_serde::shape_tracking_options::de_tracking_options(
+                "ArchivingOptions" => {
+                    builder = builder.set_archiving_options(super::super::protocol_serde::shape_archiving_options::de_archiving_options(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "VdmOptions" => {
-                    builder = builder.set_vdm_options(super::super::protocol_serde::shape_vdm_options::de_vdm_options(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_contact.rs`

```diff
--- reference/src/protocol_serde/shape_get_contact.rs
+++ generated/src/protocol_serde/shape_get_contact.rs
@@ -97,13 +97,6 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
-                    "AttributesData" => {
-                        builder = builder.set_attributes_data(
-                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                .transpose()?,
-                        );
-                    }
                     "ContactListName" => {
                         builder = builder.set_contact_list_name(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -111,12 +104,6 @@
                                 .transpose()?,
                         );
                     }
-                    "CreatedTimestamp" => {
-                        builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                            tokens.next(),
-                            ::aws_smithy_types::date_time::Format::EpochSeconds,
-                        )?);
-                    }
                     "EmailAddress" => {
                         builder = builder.set_email_address(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -124,10 +111,11 @@
                                 .transpose()?,
                         );
                     }
-                    "LastUpdatedTimestamp" => {
-                        builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                            tokens.next(),
-                            ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    "TopicPreferences" => {
+                        builder = builder.set_topic_preferences(super::super::protocol_serde::shape_topic_preference_list::de_topic_preference_list(
+                            tokens,
+                            _value,
+                            depth + 1,
                         )?);
                     }
                     "TopicDefaultPreferences" => {
@@ -135,15 +123,27 @@
                             super::super::protocol_serde::shape_topic_preference_list::de_topic_preference_list(tokens, _value, depth + 1)?,
                         );
                     }
-                    "TopicPreferences" => {
-                        builder = builder.set_topic_preferences(super::super::protocol_serde::shape_topic_preference_list::de_topic_preference_list(
-                            tokens,
-                            _value,
-                            depth + 1,
+                    "UnsubscribeAll" => {
+                        builder = builder.set_unsubscribe_all(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                    }
+                    "AttributesData" => {
+                        builder = builder.set_attributes_data(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "CreatedTimestamp" => {
+                        builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                            tokens.next(),
+                            ::aws_smithy_types::date_time::Format::EpochSeconds,
                         )?);
                     }
-                    "UnsubscribeAll" => {
-                        builder = builder.set_unsubscribe_all(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                    "LastUpdatedTimestamp" => {
+                        builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                            tokens.next(),
+                            ::aws_smithy_types::date_time::Format::EpochSeconds,
+                        )?);
                     }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
```

### `src/protocol_serde/shape_get_contact_list.rs`

```diff
--- reference/src/protocol_serde/shape_get_contact_list.rs
+++ generated/src/protocol_serde/shape_get_contact_list.rs
@@ -105,11 +105,8 @@
                             .transpose()?,
                     );
                 }
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
+                "Topics" => {
+                    builder = builder.set_topics(super::super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
                 }
                 "Description" => {
                     builder = builder.set_description(
@@ -118,6 +115,12 @@
                             .transpose()?,
                     );
                 }
+                "CreatedTimestamp" => {
+                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "LastUpdatedTimestamp" => {
                     builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                         tokens.next(),
@@ -127,9 +130,6 @@
                 "Tags" => {
                     builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
-                "Topics" => {
-                    builder = builder.set_topics(super::super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_custom_verification_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_get_custom_verification_email_template.rs
+++ generated/src/protocol_serde/shape_get_custom_verification_email_template.rs
@@ -113,8 +113,8 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "FailureRedirectionURL" => {
-                    builder = builder.set_failure_redirection_url(
+                "TemplateName" => {
+                    builder = builder.set_template_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -127,16 +127,13 @@
                             .transpose()?,
                     );
                 }
-                "SuccessRedirectionURL" => {
-                    builder = builder.set_success_redirection_url(
+                "TemplateSubject" => {
+                    builder = builder.set_template_subject(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Tags" => {
-                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
-                }
                 "TemplateContent" => {
                     builder = builder.set_template_content(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -144,15 +141,18 @@
                             .transpose()?,
                     );
                 }
-                "TemplateName" => {
-                    builder = builder.set_template_name(
+                "Tags" => {
+                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                }
+                "SuccessRedirectionURL" => {
+                    builder = builder.set_success_redirection_url(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "TemplateSubject" => {
-                    builder = builder.set_template_subject(
+                "FailureRedirectionURL" => {
+                    builder = builder.set_failure_redirection_url(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
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

@@ -111,6 +113,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "DashboardEnabled" => {
+                    builder = builder.set_dashboard_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
+                "SubscriptionExpiryDate" => {
+                    builder = builder.set_subscription_expiry_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "AccountStatus" => {
                     builder = builder.set_account_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -130,9 +141,6 @@
                         )?,
                     );
                 }
-                "DashboardEnabled" => {
-                    builder = builder.set_dashboard_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
                 "PendingExpirationSubscribedDomains" => {
                     builder = builder.set_pending_expiration_subscribed_domains(
                         super::super::protocol_serde::shape_domain_deliverability_tracking_options::de_domain_deliverability_tracking_options(
@@ -142,12 +150,6 @@
                         )?,
                     );
                 }
-                "SubscriptionExpiryDate" => {
-                    builder = builder.set_subscription_expiry_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_deliverability_test_report.rs`

```diff
--- reference/src/protocol_serde/shape_get_deliverability_test_report.rs
+++ generated/src/protocol_serde/shape_get_deliverability_test_report.rs
@@ -111,6 +111,13 @@
                         super::super::protocol_serde::shape_deliverability_test_report::de_deliverability_test_report(tokens, _value, depth + 1)?,
                     );
                 }
+                "OverallPlacement" => {
+                    builder = builder.set_overall_placement(super::super::protocol_serde::shape_placement_statistics::de_placement_statistics(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "IspPlacements" => {
                     builder = builder.set_isp_placements(super::super::protocol_serde::shape_isp_placements::de_isp_placements(tokens, _value, depth + 1)?);
                 }
@@ -121,13 +128,6 @@
                             .transpose()?,
                     );
                 }
-                "OverallPlacement" => {
-                    builder = builder.set_overall_placement(super::super::protocol_serde::shape_placement_statistics::de_placement_statistics(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "Tags" => {
                     builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
```

### `src/protocol_serde/shape_get_domain_statistics_report.rs`

```diff
--- reference/src/protocol_serde/shape_get_domain_statistics_report.rs
+++ generated/src/protocol_serde/shape_get_domain_statistics_report.rs
@@ -110,12 +110,12 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "OverallVolume" => {
+                    builder = builder.set_overall_volume(super::super::protocol_serde::shape_overall_volume::de_overall_volume(tokens, _value, depth + 1)?);
+                }
                 "DailyVolumes" => {
                     builder = builder.set_daily_volumes(super::super::protocol_serde::shape_daily_volumes::de_daily_volumes(tokens, _value, depth + 1)?);
                 }
-                "OverallVolume" => {
-                    builder = builder.set_overall_volume(super::super::protocol_serde::shape_overall_volume::de_overall_volume(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_email_identity.rs`

```diff
--- reference/src/protocol_serde/shape_get_email_identity.rs
+++ generated/src/protocol_serde/shape_get_email_identity.rs
@@ -98,13 +98,19 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "ConfigurationSetName" => {
-                    builder = builder.set_configuration_set_name(
+                "IdentityType" => {
+                    builder = builder.set_identity_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::super::types::IdentityType::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
+                "FeedbackForwardingStatus" => {
+                    builder = builder.set_feedback_forwarding_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
+                "VerifiedForSendingStatus" => {
+                    builder = builder.set_verified_for_sending_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
                 "DkimAttributes" => {
                     builder = builder.set_dkim_attributes(super::super::protocol_serde::shape_dkim_attributes::de_dkim_attributes(
                         tokens,
@@ -112,16 +118,6 @@
                         depth + 1,
                     )?);
                 }
-                "FeedbackForwardingStatus" => {
-                    builder = builder.set_feedback_forwarding_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
-                "IdentityType" => {
-                    builder = builder.set_identity_type(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::super::types::IdentityType::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 "MailFromAttributes" => {
                     builder = builder.set_mail_from_attributes(super::super::protocol_serde::shape_mail_from_attributes::de_mail_from_attributes(
                         tokens,
@@ -135,12 +131,12 @@
                 "Tags" => {
                     builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
-                "VerificationInfo" => {
-                    builder = builder.set_verification_info(super::super::protocol_serde::shape_verification_info::de_verification_info(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "ConfigurationSetName" => {
+                    builder = builder.set_configuration_set_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
                 "VerificationStatus" => {
                     builder = builder.set_verification_status(
@@ -149,8 +145,12 @@
                             .transpose()?,
                     );
                 }
-                "VerifiedForSendingStatus" => {
-                    builder = builder.set_verified_for_sending_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                "VerificationInfo" => {
+                    builder = builder.set_verification_info(super::super::protocol_serde::shape_verification_info::de_verification_info(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_get_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_get_email_template.rs
+++ generated/src/protocol_serde/shape_get_email_template.rs
@@ -100,8 +100,12 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Tags" => {
-                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                "TemplateName" => {
+                    builder = builder.set_template_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
                 "TemplateContent" => {
                     builder = builder.set_template_content(super::super::protocol_serde::shape_email_template_content::de_email_template_content(
@@ -110,12 +114,8 @@
                         depth + 1,
                     )?);
                 }
-                "TemplateName" => {
-                    builder = builder.set_template_name(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "Tags" => {
+                    builder = builder.set_tags(super::super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_get_export_job.rs`

```diff
--- reference/src/protocol_serde/shape_get_export_job.rs
+++ generated/src/protocol_serde/shape_get_export_job.rs
@@ -98,16 +98,32 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CompletedTimestamp" => {
-                    builder = builder.set_completed_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
+                "JobId" => {
+                    builder = builder.set_job_id(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "ExportSourceType" => {
+                    builder = builder.set_export_source_type(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::super::types::ExportSourceType::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
+                "JobStatus" => {
+                    builder = builder.set_job_status(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::super::types::JobStatus::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                "ExportDestination" => {
+                    builder = builder.set_export_destination(super::super::protocol_serde::shape_export_destination::de_export_destination(
+                        tokens,
+                        _value,
+                        depth + 1,
                     )?);
                 }
                 "ExportDataSource" => {
@@ -117,37 +133,21 @@
                         depth + 1,
                     )?);
                 }
-                "ExportDestination" => {
-                    builder = builder.set_export_destination(super::super::protocol_serde::shape_export_destination::de_export_destination(
-                        tokens,
-                        _value,
-                        depth + 1,
+                "CreatedTimestamp" => {
+                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                     )?);
                 }
-                "ExportSourceType" => {
-                    builder = builder.set_export_source_type(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::super::types::ExportSourceType::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "CompletedTimestamp" => {
+                    builder = builder.set_completed_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
                 }
                 "FailureInfo" => {
                     builder = builder.set_failure_info(super::super::protocol_serde::shape_failure_info::de_failure_info(tokens, _value, depth + 1)?);
                 }
-                "JobId" => {
-                    builder = builder.set_job_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "JobStatus" => {
-                    builder = builder.set_job_status(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::super::types::JobStatus::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 "Statistics" => {
                     builder = builder.set_statistics(super::super::protocol_serde::shape_export_statistics::de_export_statistics(
                         tokens,
```

### `src/protocol_serde/shape_get_import_job.rs`

```diff
--- reference/src/protocol_serde/shape_get_import_job.rs
+++ generated/src/protocol_serde/shape_get_import_job.rs
@@ -98,48 +98,29 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CompletedTimestamp" => {
-                    builder = builder.set_completed_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "FailedRecordsCount" => {
-                    builder = builder.set_failed_records_count(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "JobId" => {
+                    builder = builder.set_job_id(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FailureInfo" => {
-                    builder = builder.set_failure_info(super::super::protocol_serde::shape_failure_info::de_failure_info(tokens, _value, depth + 1)?);
-                }
-                "ImportDataSource" => {
-                    builder = builder.set_import_data_source(super::super::protocol_serde::shape_import_data_source::de_import_data_source(
+                "ImportDestination" => {
+                    builder = builder.set_import_destination(super::super::protocol_serde::shape_import_destination::de_import_destination(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "ImportDestination" => {
-                    builder = builder.set_import_destination(super::super::protocol_serde::shape_import_destination::de_import_destination(
+                "ImportDataSource" => {
+                    builder = builder.set_import_data_source(super::super::protocol_serde::shape_import_data_source::de_import_data_source(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "JobId" => {
-                    builder = builder.set_job_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "FailureInfo" => {
+                    builder = builder.set_failure_info(super::super::protocol_serde::shape_failure_info::de_failure_info(tokens, _value, depth + 1)?);
                 }
                 "JobStatus" => {
                     builder = builder.set_job_status(
@@ -148,6 +129,18 @@
                             .transpose()?,
                     );
                 }
+                "CreatedTimestamp" => {
+                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
+                "CompletedTimestamp" => {
+                    builder = builder.set_completed_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "ProcessedRecordsCount" => {
                     builder = builder.set_processed_records_count(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -155,6 +148,13 @@
                             .transpose()?,
                     );
                 }
+                "FailedRecordsCount" => {
+                    builder = builder.set_failed_records_count(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_message_insights.rs`

```diff
--- reference/src/protocol_serde/shape_get_message_insights.rs
+++ generated/src/protocol_serde/shape_get_message_insights.rs
@@ -104,29 +104,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "EmailTags" => {
-                    builder = builder.set_email_tags(super::super::protocol_serde::shape_message_tag_list::de_message_tag_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "FromEmailAddress" => {
-                    builder = builder.set_from_email_address(
+                "MessageId" => {
+                    builder = builder.set_message_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Insights" => {
-                    builder = builder.set_insights(super::super::protocol_serde::shape_email_insights_list::de_email_insights_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "MessageId" => {
-                    builder = builder.set_message_id(
+                "FromEmailAddress" => {
+                    builder = builder.set_from_email_address(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -139,6 +125,20 @@
                             .transpose()?,
                     );
                 }
+                "EmailTags" => {
+                    builder = builder.set_email_tags(super::super::protocol_serde::shape_message_tag_list::de_message_tag_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "Insights" => {
+                    builder = builder.set_insights(super::super::protocol_serde::shape_email_insights_list::de_email_insights_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_multi_region_endpoint.rs`

```diff
--- reference/src/protocol_serde/shape_get_multi_region_endpoint.rs
+++ generated/src/protocol_serde/shape_get_multi_region_endpoint.rs
@@ -108,32 +108,20 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "EndpointId" => {
-                    builder = builder.set_endpoint_id(
+                "EndpointName" => {
+                    builder = builder.set_endpoint_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "EndpointName" => {
-                    builder = builder.set_endpoint_name(
+                "EndpointId" => {
+                    builder = builder.set_endpoint_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LastUpdatedTimestamp" => {
-                    builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
                 "Routes" => {
                     builder = builder.set_routes(super::super::protocol_serde::shape_routes::de_routes(tokens, _value, depth + 1)?);
                 }
@@ -144,6 +132,18 @@
                             .transpose()?,
                     );
                 }
+                "CreatedTimestamp" => {
+                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
+                "LastUpdatedTimestamp" => {
+                    builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_guardian_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_guardian_attributes.rs
+++ generated/src/protocol_serde/shape_guardian_attributes.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_guardian_attributes(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::GuardianAttributes,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.optimized_shared_delivery {
+        object.key("OptimizedSharedDelivery").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_guardian_attributes<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_guardian_attributes(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::GuardianAttributes,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.optimized_shared_delivery {
-        object.key("OptimizedSharedDelivery").string(var_1.as_str());
-    }
-    Ok(())
-}
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

### `src/protocol_serde/shape_list_contacts_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_contacts_input.rs
+++ generated/src/protocol_serde/shape_list_contacts_input.rs
@@ -9,14 +9,14 @@
         super::super::protocol_serde::shape_list_contacts_filter::ser_list_contacts_filter(&mut object_2, var_1)?;
         object_2.finish();
     }
-    if let Some(var_3) = &input.next_token {
-        object.key("NextToken").string(var_3.as_str());
-    }
-    if let Some(var_4) = &input.page_size {
+    if let Some(var_3) = &input.page_size {
         object.key("PageSize").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
+    if let Some(var_4) = &input.next_token {
+        object.key("NextToken").string(var_4.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_list_email_templates.rs`

```diff
--- reference/src/protocol_serde/shape_list_email_templates.rs
+++ generated/src/protocol_serde/shape_list_email_templates.rs
@@ -89,6 +89,11 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "TemplatesMetadata" => {
+                    builder = builder.set_templates_metadata(
+                        super::super::protocol_serde::shape_email_template_metadata_list::de_email_template_metadata_list(tokens, _value, depth + 1)?,
+                    );
+                }
                 "NextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -96,11 +101,6 @@
                             .transpose()?,
                     );
                 }
-                "TemplatesMetadata" => {
-                    builder = builder.set_templates_metadata(
-                        super::super::protocol_serde::shape_email_template_metadata_list::de_email_template_metadata_list(tokens, _value, depth + 1)?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_export_jobs_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_export_jobs_input.rs
+++ generated/src/protocol_serde/shape_list_export_jobs_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::list_export_jobs::ListExportJobsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.export_source_type {
-        object.key("ExportSourceType").string(var_1.as_str());
+    if let Some(var_1) = &input.next_token {
+        object.key("NextToken").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.job_status {
-        object.key("JobStatus").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.next_token {
-        object.key("NextToken").string(var_3.as_str());
-    }
-    if let Some(var_4) = &input.page_size {
+    if let Some(var_2) = &input.page_size {
         object.key("PageSize").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
+    if let Some(var_3) = &input.export_source_type {
+        object.key("ExportSourceType").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.job_status {
+        object.key("JobStatus").string(var_4.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_list_recommendations.rs`

```diff
--- reference/src/protocol_serde/shape_list_recommendations.rs
+++ generated/src/protocol_serde/shape_list_recommendations.rs
@@ -114,6 +114,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Recommendations" => {
+                    builder = builder.set_recommendations(super::super::protocol_serde::shape_recommendations_list::de_recommendations_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "NextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -121,13 +128,6 @@
                             .transpose()?,
                     );
                 }
-                "Recommendations" => {
-                    builder = builder.set_recommendations(super::super::protocol_serde::shape_recommendations_list::de_recommendations_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_reputation_entities.rs`

```diff
--- reference/src/protocol_serde/shape_list_reputation_entities.rs
+++ generated/src/protocol_serde/shape_list_reputation_entities.rs
@@ -104,6 +104,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
+                    "ReputationEntities" => {
+                        builder = builder.set_reputation_entities(
+                            super::super::protocol_serde::shape_reputation_entities_list::de_reputation_entities_list(tokens, _value, depth + 1)?,
+                        );
+                    }
                     "NextToken" => {
                         builder = builder.set_next_token(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -111,11 +116,6 @@
                                 .transpose()?,
                         );
                     }
-                    "ReputationEntities" => {
-                        builder = builder.set_reputation_entities(
-                            super::super::protocol_serde::shape_reputation_entities_list::de_reputation_entities_list(tokens, _value, depth + 1)?,
-                        );
-                    }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
             }
```

### `src/protocol_serde/shape_list_resource_tenants.rs`

```diff
--- reference/src/protocol_serde/shape_list_resource_tenants.rs
+++ generated/src/protocol_serde/shape_list_resource_tenants.rs
@@ -114,6 +114,11 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "ResourceTenants" => {
+                    builder = builder.set_resource_tenants(
+                        super::super::protocol_serde::shape_resource_tenant_metadata_list::de_resource_tenant_metadata_list(tokens, _value, depth + 1)?,
+                    );
+                }
                 "NextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -121,11 +126,6 @@
                             .transpose()?,
                     );
                 }
-                "ResourceTenants" => {
-                    builder = builder.set_resource_tenants(
-                        super::super::protocol_serde::shape_resource_tenant_metadata_list::de_resource_tenant_metadata_list(tokens, _value, depth + 1)?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_resource_tenants_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_resource_tenants_input.rs
+++ generated/src/protocol_serde/shape_list_resource_tenants_input.rs
@@ -3,8 +3,8 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::list_resource_tenants::ListResourceTenantsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.next_token {
-        object.key("NextToken").string(var_1.as_str());
+    if let Some(var_1) = &input.resource_arn {
+        object.key("ResourceArn").string(var_1.as_str());
     }
     if let Some(var_2) = &input.page_size {
         object.key("PageSize").number(
@@ -12,8 +12,8 @@
             ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
-    if let Some(var_3) = &input.resource_arn {
-        object.key("ResourceArn").string(var_3.as_str());
+    if let Some(var_3) = &input.next_token {
+        object.key("NextToken").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_list_suppressed_destinations.rs`

```diff
--- reference/src/protocol_serde/shape_list_suppressed_destinations.rs
+++ generated/src/protocol_serde/shape_list_suppressed_destinations.rs
@@ -119,13 +119,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "NextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "SuppressedDestinationSummaries" => {
                     builder = builder.set_suppressed_destination_summaries(
                         super::super::protocol_serde::shape_suppressed_destination_summaries::de_suppressed_destination_summaries(
@@ -135,6 +128,13 @@
                         )?,
                     );
                 }
+                "NextToken" => {
+                    builder = builder.set_next_token(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_tenant_resources.rs`

```diff
--- reference/src/protocol_serde/shape_list_tenant_resources.rs
+++ generated/src/protocol_serde/shape_list_tenant_resources.rs
@@ -114,6 +114,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "TenantResources" => {
+                    builder = builder.set_tenant_resources(super::super::protocol_serde::shape_tenant_resource_list::de_tenant_resource_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "NextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -121,13 +128,6 @@
                             .transpose()?,
                     );
                 }
-                "TenantResources" => {
-                    builder = builder.set_tenant_resources(super::super::protocol_serde::shape_tenant_resource_list::de_tenant_resource_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_tenant_resources_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_tenant_resources_input.rs
+++ generated/src/protocol_serde/shape_list_tenant_resources_input.rs
@@ -3,18 +3,18 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::list_tenant_resources::ListTenantResourcesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.filter {
+    if let Some(var_1) = &input.tenant_name {
+        object.key("TenantName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.filter {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("Filter").start_object();
-        for (key_3, value_4) in var_1 {
+        let mut object_3 = object.key("Filter").start_object();
+        for (key_4, value_5) in var_2 {
             {
-                object_2.key(key_3.as_str()).string(value_4.as_str());
+                object_3.key(key_4.as_str()).string(value_5.as_str());
             }
         }
-        object_2.finish();
-    }
-    if let Some(var_5) = &input.next_token {
-        object.key("NextToken").string(var_5.as_str());
+        object_3.finish();
     }
     if let Some(var_6) = &input.page_size {
         object.key("PageSize").number(
@@ -22,8 +22,8 @@
             ::aws_smithy_types::Number::NegInt((*var_6).into()),
         );
     }
-    if let Some(var_7) = &input.tenant_name {
-        object.key("TenantName").string(var_7.as_str());
+    if let Some(var_7) = &input.next_token {
+        object.key("NextToken").string(var_7.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_list_tenants.rs`

```diff
--- reference/src/protocol_serde/shape_list_tenants.rs
+++ generated/src/protocol_serde/shape_list_tenants.rs
@@ -91,6 +91,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Tenants" => {
+                    builder = builder.set_tenants(super::super::protocol_serde::shape_tenant_info_list::de_tenant_info_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "NextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -98,13 +105,6 @@
                             .transpose()?,
                     );
                 }
-                "Tenants" => {
-                    builder = builder.set_tenants(super::super::protocol_serde::shape_tenant_info_list::de_tenant_info_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
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

### `src/protocol_serde/shape_put_account_details_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_details_input.rs
+++ generated/src/protocol_serde/shape_put_account_details_input.rs
@@ -3,29 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::put_account_details::PutAccountDetailsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.additional_contact_email_addresses {
-        let mut array_2 = object.key("AdditionalContactEmailAddresses").start_array();
-        for item_3 in var_1 {
-            {
-                array_2.value().string(item_3.as_str());
-            }
-        }
-        array_2.finish();
+    if let Some(var_1) = &input.mail_type {
+        object.key("MailType").string(var_1.as_str());
     }
-    if let Some(var_4) = &input.contact_language {
-        object.key("ContactLanguage").string(var_4.as_str());
+    if let Some(var_2) = &input.website_url {
+        object.key("WebsiteURL").string(var_2.as_str());
     }
-    if let Some(var_5) = &input.mail_type {
-        object.key("MailType").string(var_5.as_str());
+    if let Some(var_3) = &input.contact_language {
+        object.key("ContactLanguage").string(var_3.as_str());
     }
-    if let Some(var_6) = &input.production_access_enabled {
-        object.key("ProductionAccessEnabled").boolean(*var_6);
+    if let Some(var_4) = &input.use_case_description {
+        object.key("UseCaseDescription").string(var_4.as_str());
     }
-    if let Some(var_7) = &input.use_case_description {
-        object.key("UseCaseDescription").string(var_7.as_str());
+    if let Some(var_5) = &input.additional_contact_email_addresses {
+        let mut array_6 = object.key("AdditionalContactEmailAddresses").start_array();
+        for item_7 in var_5 {
+            {
+                array_6.value().string(item_7.as_str());
+            }
+        }
+        array_6.finish();
     }
-    if let Some(var_8) = &input.website_url {
-        object.key("WebsiteURL").string(var_8.as_str());
+    if let Some(var_8) = &input.production_access_enabled {
+        object.key("ProductionAccessEnabled").boolean(*var_8);
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_delivery_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_delivery_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_delivery_options_input.rs
@@ -3,17 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::put_configuration_set_delivery_options::PutConfigurationSetDeliveryOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.max_delivery_seconds {
-        object.key("MaxDeliverySeconds").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
+    if let Some(var_1) = &input.tls_policy {
+        object.key("TlsPolicy").string(var_1.as_str());
     }
     if let Some(var_2) = &input.sending_pool_name {
         object.key("SendingPoolName").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.tls_policy {
-        object.key("TlsPolicy").string(var_3.as_str());
+    if let Some(var_3) = &input.max_delivery_seconds {
+        object.key("MaxDeliverySeconds").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+        );
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_suppression_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_suppression_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_suppression_options_input.rs
@@ -3,17 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.suppressed_reasons {
-        let mut array_2 = object.key("SuppressedReasons").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.suppression_scope {
+        object.key("SuppressionScope").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.suppressed_reasons {
+        let mut array_3 = object.key("SuppressedReasons").start_array();
+        for item_4 in var_2 {
             {
-                array_2.value().string(item_3.as_str());
+                array_3.value().string(item_4.as_str());
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_4) = &input.suppression_scope {
-        object.key("SuppressionScope").string(var_4.as_str());
+        array_3.finish();
     }
     if let Some(var_5) = &input.validation_options {
         #[allow(unused_mut)]
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

### `src/protocol_serde/shape_put_email_identity_dkim_signing_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_dkim_signing_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_dkim_signing_attributes_input.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::put_email_identity_dkim_signing_attributes::PutEmailIdentityDkimSigningAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.signing_attributes {
+    if let Some(var_1) = &input.signing_attributes_origin {
+        object.key("SigningAttributesOrigin").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.signing_attributes {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("SigningAttributes").start_object();
-        super::super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.signing_attributes_origin {
-        object.key("SigningAttributesOrigin").string(var_3.as_str());
+        let mut object_3 = object.key("SigningAttributes").start_object();
+        super::super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_3, var_2)?;
+        object_3.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_email_identity_mail_from_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_mail_from_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_mail_from_attributes_input.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::put_email_identity_mail_from_attributes::PutEmailIdentityMailFromAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.behavior_on_mx_failure {
-        object.key("BehaviorOnMxFailure").string(var_1.as_str());
+    if let Some(var_1) = &input.mail_from_domain {
+        object.key("MailFromDomain").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.mail_from_domain {
-        object.key("MailFromDomain").string(var_2.as_str());
+    if let Some(var_2) = &input.behavior_on_mx_failure {
+        object.key("BehaviorOnMxFailure").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_tenant_suppression_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_tenant_suppression_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_tenant_suppression_attributes_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::put_tenant_suppression_attributes::PutTenantSuppressionAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.suppressed_reasons {
-        let mut array_2 = object.key("SuppressedReasons").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.tenant_name {
+        object.key("TenantName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.suppressed_reasons {
+        let mut array_3 = object.key("SuppressedReasons").start_array();
+        for item_4 in var_2 {
             {
-                array_2.value().string(item_3.as_str());
+                array_3.value().string(item_4.as_str());
             }
         }
-        array_2.finish();
+        array_3.finish();
     }
-    if let Some(var_4) = &input.suppression_scope {
-        object.key("SuppressionScope").string(var_4.as_str());
-    }
-    if let Some(var_5) = &input.tenant_name {
-        object.key("TenantName").string(var_5.as_str());
+    if let Some(var_5) = &input.suppression_scope {
+        object.key("SuppressionScope").string(var_5.as_str());
     }
     Ok(())
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

### `src/protocol_serde/shape_send_bulk_email_input.rs`

```diff
--- reference/src/protocol_serde/shape_send_bulk_email_input.rs
+++ generated/src/protocol_serde/shape_send_bulk_email_input.rs
@@ -3,71 +3,71 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::send_bulk_email::SendBulkEmailInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.bulk_email_entries {
-        let mut array_2 = object.key("BulkEmailEntries").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.from_email_address {
+        object.key("FromEmailAddress").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.from_email_address_identity_arn {
+        object.key("FromEmailAddressIdentityArn").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.reply_to_addresses {
+        let mut array_4 = object.key("ReplyToAddresses").start_array();
+        for item_5 in var_3 {
             {
-                #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_bulk_email_entry::ser_bulk_email_entry(&mut object_4, item_3)?;
-                object_4.finish();
+                array_4.value().string(item_5.as_str());
             }
         }
-        array_2.finish();
+        array_4.finish();
     }
-    if let Some(var_5) = &input.configuration_overrides {
-        #[allow(unused_mut)]
-        let mut object_6 = object.key("ConfigurationOverrides").start_object();
-        super::super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_6, var_5)?;
-        object_6.finish();
+    if let Some(var_6) = &input.feedback_forwarding_email_address {
+        object.key("FeedbackForwardingEmailAddress").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.feedback_forwarding_email_address_identity_arn {
+        object.key("FeedbackForwardingEmailAddressIdentityArn").string(var_7.as_str());
     }
-    if let Some(var_7) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_7.as_str());
+    if let Some(var_8) = &input.default_email_tags {
+        let mut array_9 = object.key("DefaultEmailTags").start_array();
+        for item_10 in var_8 {
+            {
+                #[allow(unused_mut)]
+                let mut object_11 = array_9.value().start_object();
+                super::super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_11, item_10)?;
+                object_11.finish();
+            }
+        }
+        array_9.finish();
     }
-    if let Some(var_8) = &input.default_content {
+    if let Some(var_12) = &input.default_content {
         #[allow(unused_mut)]
-        let mut object_9 = object.key("DefaultContent").start_object();
-        super::super::protocol_serde::shape_bulk_email_content::ser_bulk_email_content(&mut object_9, var_8)?;
-        object_9.finish();
+        let mut object_13 = object.key("DefaultContent").start_object();
+        super::super::protocol_serde::shape_bulk_email_content::ser_bulk_email_content(&mut object_13, var_12)?;
+        object_13.finish();
     }
-    if let Some(var_10) = &input.default_email_tags {
-        let mut array_11 = object.key("DefaultEmailTags").start_array();
-        for item_12 in var_10 {
+    if let Some(var_14) = &input.bulk_email_entries {
+        let mut array_15 = object.key("BulkEmailEntries").start_array();
+        for item_16 in var_14 {
             {
                 #[allow(unused_mut)]
-                let mut object_13 = array_11.value().start_object();
-                super::super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_13, item_12)?;
-                object_13.finish();
+                let mut object_17 = array_15.value().start_object();
+                super::super::protocol_serde::shape_bulk_email_entry::ser_bulk_email_entry(&mut object_17, item_16)?;
+                object_17.finish();
             }
         }
-        array_11.finish();
-    }
-    if let Some(var_14) = &input.endpoint_id {
-        object.key("EndpointId").string(var_14.as_str());
-    }
-    if let Some(var_15) = &input.feedback_forwarding_email_address {
-        object.key("FeedbackForwardingEmailAddress").string(var_15.as_str());
-    }
-    if let Some(var_16) = &input.feedback_forwarding_email_address_identity_arn {
-        object.key("FeedbackForwardingEmailAddressIdentityArn").string(var_16.as_str());
+        array_15.finish();
     }
-    if let Some(var_17) = &input.from_email_address {
-        object.key("FromEmailAddress").string(var_17.as_str());
+    if let Some(var_18) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_18.as_str());
     }
-    if let Some(var_18) = &input.from_email_address_identity_arn {
-        object.key("FromEmailAddressIdentityArn").string(var_18.as_str());
+    if let Some(var_19) = &input.endpoint_id {
+        object.key("EndpointId").string(var_19.as_str());
     }
-    if let Some(var_19) = &input.reply_to_addresses {
-        let mut array_20 = object.key("ReplyToAddresses").start_array();
-        for item_21 in var_19 {
-            {
-                array_20.value().string(item_21.as_str());
-            }
-        }
-        array_20.finish();
+    if let Some(var_20) = &input.tenant_name {
+        object.key("TenantName").string(var_20.as_str());
     }
-    if let Some(var_22) = &input.tenant_name {
-        object.key("TenantName").string(var_22.as_str());
+    if let Some(var_21) = &input.configuration_overrides {
+        #[allow(unused_mut)]
+        let mut object_22 = object.key("ConfigurationOverrides").start_object();
+        super::super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_22, var_21)?;
+        object_22.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_send_custom_verification_email_input.rs`

```diff
--- reference/src/protocol_serde/shape_send_custom_verification_email_input.rs
+++ generated/src/protocol_serde/shape_send_custom_verification_email_input.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::send_custom_verification_email::SendCustomVerificationEmailInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_1.as_str());
+    if let Some(var_1) = &input.email_address {
+        object.key("EmailAddress").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.email_address {
-        object.key("EmailAddress").string(var_2.as_str());
+    if let Some(var_2) = &input.template_name {
+        object.key("TemplateName").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.template_name {
-        object.key("TemplateName").string(var_3.as_str());
+    if let Some(var_3) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_send_email_input.rs`

```diff
--- reference/src/protocol_serde/shape_send_email_input.rs
+++ generated/src/protocol_serde/shape_send_email_input.rs
@@ -3,71 +3,71 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::send_email::SendEmailInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.configuration_overrides {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("ConfigurationOverrides").start_object();
-        super::super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.from_email_address {
+        object.key("FromEmailAddress").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_3.as_str());
+    if let Some(var_2) = &input.from_email_address_identity_arn {
+        object.key("FromEmailAddressIdentityArn").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.content {
+    if let Some(var_3) = &input.destination {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("Content").start_object();
-        super::super::protocol_serde::shape_email_content::ser_email_content(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_4 = object.key("Destination").start_object();
+        super::super::protocol_serde::shape_destination::ser_destination(&mut object_4, var_3)?;
+        object_4.finish();
     }
-    if let Some(var_6) = &input.destination {
+    if let Some(var_5) = &input.reply_to_addresses {
+        let mut array_6 = object.key("ReplyToAddresses").start_array();
+        for item_7 in var_5 {
+            {
+                array_6.value().string(item_7.as_str());
+            }
+        }
+        array_6.finish();
+    }
+    if let Some(var_8) = &input.feedback_forwarding_email_address {
+        object.key("FeedbackForwardingEmailAddress").string(var_8.as_str());
+    }
+    if let Some(var_9) = &input.feedback_forwarding_email_address_identity_arn {
+        object.key("FeedbackForwardingEmailAddressIdentityArn").string(var_9.as_str());
+    }
+    if let Some(var_10) = &input.content {
         #[allow(unused_mut)]
-        let mut object_7 = object.key("Destination").start_object();
-        super::super::protocol_serde::shape_destination::ser_destination(&mut object_7, var_6)?;
-        object_7.finish();
+        let mut object_11 = object.key("Content").start_object();
+        super::super::protocol_serde::shape_email_content::ser_email_content(&mut object_11, var_10)?;
+        object_11.finish();
     }
-    if let Some(var_8) = &input.email_tags {
-        let mut array_9 = object.key("EmailTags").start_array();
-        for item_10 in var_8 {
+    if let Some(var_12) = &input.email_tags {
+        let mut array_13 = object.key("EmailTags").start_array();
+        for item_14 in var_12 {
             {
                 #[allow(unused_mut)]
-                let mut object_11 = array_9.value().start_object();
-                super::super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_11, item_10)?;
-                object_11.finish();
+                let mut object_15 = array_13.value().start_object();
+                super::super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_15, item_14)?;
+                object_15.finish();
             }
         }
-        array_9.finish();
-    }
-    if let Some(var_12) = &input.endpoint_id {
-        object.key("EndpointId").string(var_12.as_str());
-    }
-    if let Some(var_13) = &input.feedback_forwarding_email_address {
-        object.key("FeedbackForwardingEmailAddress").string(var_13.as_str());
+        array_13.finish();
     }
-    if let Some(var_14) = &input.feedback_forwarding_email_address_identity_arn {
-        object.key("FeedbackForwardingEmailAddressIdentityArn").string(var_14.as_str());
+    if let Some(var_16) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_16.as_str());
     }
-    if let Some(var_15) = &input.from_email_address {
-        object.key("FromEmailAddress").string(var_15.as_str());
+    if let Some(var_17) = &input.endpoint_id {
+        object.key("EndpointId").string(var_17.as_str());
     }
-    if let Some(var_16) = &input.from_email_address_identity_arn {
-        object.key("FromEmailAddressIdentityArn").string(var_16.as_str());
+    if let Some(var_18) = &input.tenant_name {
+        object.key("TenantName").string(var_18.as_str());
     }
-    if let Some(var_17) = &input.list_management_options {
+    if let Some(var_19) = &input.list_management_options {
         #[allow(unused_mut)]
-        let mut object_18 = object.key("ListManagementOptions").start_object();
-        super::super::protocol_serde::shape_list_management_options::ser_list_management_options(&mut object_18, var_17)?;
-        object_18.finish();
+        let mut object_20 = object.key("ListManagementOptions").start_object();
+        super::super::protocol_serde::shape_list_management_options::ser_list_management_options(&mut object_20, var_19)?;
+        object_20.finish();
     }
-    if let Some(var_19) = &input.reply_to_addresses {
-        let mut array_20 = object.key("ReplyToAddresses").start_array();
-        for item_21 in var_19 {
-            {
-                array_20.value().string(item_21.as_str());
-            }
-        }
-        array_20.finish();
-    }
-    if let Some(var_22) = &input.tenant_name {
-        object.key("TenantName").string(var_22.as_str());
+    if let Some(var_21) = &input.configuration_overrides {
+        #[allow(unused_mut)]
+        let mut object_22 = object.key("ConfigurationOverrides").start_object();
+        super::super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_22, var_21)?;
+        object_22.finish();
     }
     Ok(())
 }
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

### `src/protocol_serde/shape_update_contact_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_contact_input.rs
+++ generated/src/protocol_serde/shape_update_contact_input.rs
@@ -3,23 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_contact::UpdateContactInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.attributes_data {
-        object.key("AttributesData").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.topic_preferences {
-        let mut array_3 = object.key("TopicPreferences").start_array();
-        for item_4 in var_2 {
+    if let Some(var_1) = &input.topic_preferences {
+        let mut array_2 = object.key("TopicPreferences").start_array();
+        for item_3 in var_1 {
             {
                 #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::super::protocol_serde::shape_topic_preference::ser_topic_preference(&mut object_5, item_4)?;
-                object_5.finish();
+                let mut object_4 = array_2.value().start_object();
+                super::super::protocol_serde::shape_topic_preference::ser_topic_preference(&mut object_4, item_3)?;
+                object_4.finish();
             }
         }
-        array_3.finish();
+        array_2.finish();
+    }
+    if let Some(var_5) = &input.unsubscribe_all {
+        object.key("UnsubscribeAll").boolean(*var_5);
     }
-    if let Some(var_6) = &input.unsubscribe_all {
-        object.key("UnsubscribeAll").boolean(*var_6);
+    if let Some(var_6) = &input.attributes_data {
+        object.key("AttributesData").string(var_6.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_contact_list_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_contact_list_input.rs
+++ generated/src/protocol_serde/shape_update_contact_list_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_contact_list::UpdateContactListInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.description {
-        object.key("Description").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.topics {
-        let mut array_3 = object.key("Topics").start_array();
-        for item_4 in var_2 {
+    if let Some(var_1) = &input.topics {
+        let mut array_2 = object.key("Topics").start_array();
+        for item_3 in var_1 {
             {
                 #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::super::protocol_serde::shape_topic::ser_topic(&mut object_5, item_4)?;
-                object_5.finish();
+                let mut object_4 = array_2.value().start_object();
+                super::super::protocol_serde::shape_topic::ser_topic(&mut object_4, item_3)?;
+                object_4.finish();
             }
         }
-        array_3.finish();
+        array_2.finish();
+    }
+    if let Some(var_5) = &input.description {
+        object.key("Description").string(var_5.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_custom_verification_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_custom_verification_email_template_input.rs
+++ generated/src/protocol_serde/shape_update_custom_verification_email_template_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_custom_verification_email_template::UpdateCustomVerificationEmailTemplateInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.failure_redirection_url {
-        object.key("FailureRedirectionURL").string(var_1.as_str());
+    if let Some(var_1) = &input.from_email_address {
+        object.key("FromEmailAddress").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.from_email_address {
-        object.key("FromEmailAddress").string(var_2.as_str());
+    if let Some(var_2) = &input.template_subject {
+        object.key("TemplateSubject").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.success_redirection_url {
-        object.key("SuccessRedirectionURL").string(var_3.as_str());
+    if let Some(var_3) = &input.template_content {
+        object.key("TemplateContent").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.template_content {
-        object.key("TemplateContent").string(var_4.as_str());
+    if let Some(var_4) = &input.success_redirection_url {
+        object.key("SuccessRedirectionURL").string(var_4.as_str());
     }
-    if let Some(var_5) = &input.template_subject {
-        object.key("TemplateSubject").string(var_5.as_str());
+    if let Some(var_5) = &input.failure_redirection_url {
+        object.key("FailureRedirectionURL").string(var_5.as_str());
     }
     Ok(())
 }
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

### `src/protocol_serde/shape_vdm_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_vdm_attributes.rs
+++ generated/src/protocol_serde/shape_vdm_attributes.rs
@@ -1,4 +1,26 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_vdm_attributes(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::VdmAttributes,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("VdmEnabled").string(input.vdm_enabled.as_str());
+    }
+    if let Some(var_1) = &input.dashboard_attributes {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("DashboardAttributes").start_object();
+        super::super::protocol_serde::shape_dashboard_attributes::ser_dashboard_attributes(&mut object_2, var_1)?;
+        object_2.finish();
+    }
+    if let Some(var_3) = &input.guardian_attributes {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("GuardianAttributes").start_object();
+        super::super::protocol_serde::shape_guardian_attributes::ser_guardian_attributes(&mut object_4, var_3)?;
+        object_4.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_vdm_attributes<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -60,25 +82,3 @@
         )),
     }
 }
-
-pub fn ser_vdm_attributes(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::VdmAttributes,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("VdmEnabled").string(input.vdm_enabled.as_str());
-    }
-    if let Some(var_1) = &input.dashboard_attributes {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("DashboardAttributes").start_object();
-        super::super::protocol_serde::shape_dashboard_attributes::ser_dashboard_attributes(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.guardian_attributes {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("GuardianAttributes").start_object();
-        super::super::protocol_serde::shape_guardian_attributes::ser_guardian_attributes(&mut object_4, var_3)?;
-        object_4.finish();
-    }
-    Ok(())
-}
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
