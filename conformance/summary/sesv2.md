# AWS SDK Conformance Report: sesv2

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sesv2
**Progress:** `1204/1204` files compared · `927` matched · `231` mismatches · `1` missing · `45` extra · `76.99%` match (100.00% means fully matched)

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
-            super::endpoint_auth::resolve_endpoint_based_auth_scheme_options(modeled_auth_options, _cfg, _runtime_components).await
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
@@ -145,11 +145,7 @@
     /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
     /// [`SigningName`](aws_types::SigningName) during operation construction
     pub fn signing_name(&self) -> &'static str {
-        "ses"
-    }
-    /// Returns the SigV4a signing region set, if configured.
-    pub fn sigv4a_signing_region_set(&self) -> Option<&::aws_types::region::SigningRegionSet> {
-        self.config.load::<::aws_types::region::SigningRegionSet>()
+        "sesv2"
     }
     /// Returns the AWS region, if it was provided.
     pub fn region(&self) -> ::std::option::Option<&super::config::Region> {
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
@@ -1406,7 +1385,7 @@
                 .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
         }
         layer.store_put(super::meta::API_METADATA.clone());
-        layer.store_put(::aws_types::SigningName::from_static("ses"));
+        layer.store_put(::aws_types::SigningName::from_static("sesv2"));
         layer
             .load::<::aws_types::region::Region>()
             .cloned()
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
@@ -19,6 +19,6 @@

 pub(crate) mod diagnostic;

+pub(crate) mod partition;
+
 pub(crate) mod host;
-
-pub(crate) mod partition;
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

### `src/operation/cancel_export_job.rs`

```diff
--- reference/src/operation/cancel_export_job.rs
+++ generated/src/operation/cancel_export_job.rs
@@ -258,10 +258,14 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_cancel_export_job::ser_cancel_export_job_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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

### `src/operation/delete_configuration_set.rs`

```diff
--- reference/src/operation/delete_configuration_set.rs
+++ generated/src/operation/delete_configuration_set.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_configuration_set::ser_delete_configuration_set_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_configuration_set_event_destination.rs`

```diff
--- reference/src/operation/delete_configuration_set_event_destination.rs
+++ generated/src/operation/delete_configuration_set_event_destination.rs
@@ -293,10 +293,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_configuration_set_event_destination::ser_delete_configuration_set_event_destination_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_contact.rs`

```diff
--- reference/src/operation/delete_contact.rs
+++ generated/src/operation/delete_contact.rs
@@ -280,10 +280,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_contact::ser_delete_contact_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_contact_list.rs`

```diff
--- reference/src/operation/delete_contact_list.rs
+++ generated/src/operation/delete_contact_list.rs
@@ -262,10 +262,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_contact_list::ser_delete_contact_list_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_custom_verification_email_template.rs`

```diff
--- reference/src/operation/delete_custom_verification_email_template.rs
+++ generated/src/operation/delete_custom_verification_email_template.rs
@@ -276,10 +276,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_custom_verification_email_template::ser_delete_custom_verification_email_template_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_dedicated_ip_pool.rs`

```diff
--- reference/src/operation/delete_dedicated_ip_pool.rs
+++ generated/src/operation/delete_dedicated_ip_pool.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_dedicated_ip_pool::ser_delete_dedicated_ip_pool_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_email_identity.rs`

```diff
--- reference/src/operation/delete_email_identity.rs
+++ generated/src/operation/delete_email_identity.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_email_identity::ser_delete_email_identity_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_email_identity_policy.rs`

```diff
--- reference/src/operation/delete_email_identity_policy.rs
+++ generated/src/operation/delete_email_identity_policy.rs
@@ -283,10 +283,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_email_identity_policy::ser_delete_email_identity_policy_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_email_template.rs`

```diff
--- reference/src/operation/delete_email_template.rs
+++ generated/src/operation/delete_email_template.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_email_template::ser_delete_email_template_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_multi_region_endpoint.rs`

```diff
--- reference/src/operation/delete_multi_region_endpoint.rs
+++ generated/src/operation/delete_multi_region_endpoint.rs
@@ -262,10 +262,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_multi_region_endpoint::ser_delete_multi_region_endpoint_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_suppressed_destination.rs`

```diff
--- reference/src/operation/delete_suppressed_destination.rs
+++ generated/src/operation/delete_suppressed_destination.rs
@@ -280,10 +280,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_suppressed_destination::ser_delete_suppressed_destination_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_account.rs`

```diff
--- reference/src/operation/get_account.rs
+++ generated/src/operation/get_account.rs
@@ -200,10 +200,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_account::ser_get_account_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
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
@@ -219,10 +217,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_blacklist_reports::ser_get_blacklist_reports_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_configuration_set.rs`

```diff
--- reference/src/operation/get_configuration_set.rs
+++ generated/src/operation/get_configuration_set.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_configuration_set::ser_get_configuration_set_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_configuration_set_event_destinations.rs`

```diff
--- reference/src/operation/get_configuration_set_event_destinations.rs
+++ generated/src/operation/get_configuration_set_event_destinations.rs
@@ -276,10 +276,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_configuration_set_event_destinations::ser_get_configuration_set_event_destinations_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_contact.rs`

```diff
--- reference/src/operation/get_contact.rs
+++ generated/src/operation/get_contact.rs
@@ -278,10 +278,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_contact::ser_get_contact_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_contact_list.rs`

```diff
--- reference/src/operation/get_contact_list.rs
+++ generated/src/operation/get_contact_list.rs
@@ -259,10 +259,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_contact_list::ser_get_contact_list_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_custom_verification_email_template.rs`

```diff
--- reference/src/operation/get_custom_verification_email_template.rs
+++ generated/src/operation/get_custom_verification_email_template.rs
@@ -276,10 +276,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_custom_verification_email_template::ser_get_custom_verification_email_template_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_dedicated_ip.rs`

```diff
--- reference/src/operation/get_dedicated_ip.rs
+++ generated/src/operation/get_dedicated_ip.rs
@@ -258,10 +258,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_dedicated_ip::ser_get_dedicated_ip_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_dedicated_ip_pool.rs`

```diff
--- reference/src/operation/get_dedicated_ip_pool.rs
+++ generated/src/operation/get_dedicated_ip_pool.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_dedicated_ip_pool::ser_get_dedicated_ip_pool_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_dedicated_ips.rs`

```diff
--- reference/src/operation/get_dedicated_ips.rs
+++ generated/src/operation/get_dedicated_ips.rs
@@ -275,10 +275,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_dedicated_ips::ser_get_dedicated_ips_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_deliverability_dashboard_options.rs`

```diff
--- reference/src/operation/get_deliverability_dashboard_options.rs
+++ generated/src/operation/get_deliverability_dashboard_options.rs
@@ -214,10 +214,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_deliverability_dashboard_options::ser_get_deliverability_dashboard_options_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_deliverability_test_report.rs`

```diff
--- reference/src/operation/get_deliverability_test_report.rs
+++ generated/src/operation/get_deliverability_test_report.rs
@@ -262,10 +262,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_deliverability_test_report::ser_get_deliverability_test_report_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_domain_deliverability_campaign.rs`

```diff
--- reference/src/operation/get_domain_deliverability_campaign.rs
+++ generated/src/operation/get_domain_deliverability_campaign.rs
@@ -274,10 +274,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_domain_deliverability_campaign::ser_get_domain_deliverability_campaign_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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
@@ -286,10 +286,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_domain_statistics_report::ser_get_domain_statistics_report_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_email_identity.rs`

```diff
--- reference/src/operation/get_email_identity.rs
+++ generated/src/operation/get_email_identity.rs
@@ -258,10 +258,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_email_identity::ser_get_email_identity_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_email_identity_policies.rs`

```diff
--- reference/src/operation/get_email_identity_policies.rs
+++ generated/src/operation/get_email_identity_policies.rs
@@ -262,10 +262,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_email_identity_policies::ser_get_email_identity_policies_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_email_template.rs`

```diff
--- reference/src/operation/get_email_template.rs
+++ generated/src/operation/get_email_template.rs
@@ -258,10 +258,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_email_template::ser_get_email_template_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_export_job.rs`

```diff
--- reference/src/operation/get_export_job.rs
+++ generated/src/operation/get_export_job.rs
@@ -259,10 +259,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_export_job::ser_get_export_job_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_import_job.rs`

```diff
--- reference/src/operation/get_import_job.rs
+++ generated/src/operation/get_import_job.rs
@@ -258,10 +258,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_import_job::ser_get_import_job_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_message_insights.rs`

```diff
--- reference/src/operation/get_message_insights.rs
+++ generated/src/operation/get_message_insights.rs
@@ -262,10 +262,15 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_message_insights::ser_get_message_insights_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_multi_region_endpoint.rs`

```diff
--- reference/src/operation/get_multi_region_endpoint.rs
+++ generated/src/operation/get_multi_region_endpoint.rs
@@ -262,10 +262,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_multi_region_endpoint::ser_get_multi_region_endpoint_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
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
@@ -279,10 +278,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_reputation_entity::ser_get_reputation_entity_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_suppressed_destination.rs`

```diff
--- reference/src/operation/get_suppressed_destination.rs
+++ generated/src/operation/get_suppressed_destination.rs
@@ -280,10 +280,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_suppressed_destination::ser_get_suppressed_destination_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_configuration_sets.rs`

```diff
--- reference/src/operation/list_configuration_sets.rs
+++ generated/src/operation/list_configuration_sets.rs
@@ -268,10 +268,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_configuration_sets::ser_list_configuration_sets_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_contact_lists.rs`

```diff
--- reference/src/operation/list_contact_lists.rs
+++ generated/src/operation/list_contact_lists.rs
@@ -265,10 +265,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_contact_lists::ser_list_contact_lists_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_custom_verification_email_templates.rs`

```diff
--- reference/src/operation/list_custom_verification_email_templates.rs
+++ generated/src/operation/list_custom_verification_email_templates.rs
@@ -278,10 +278,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_custom_verification_email_templates::ser_list_custom_verification_email_templates_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_dedicated_ip_pools.rs`

```diff
--- reference/src/operation/list_dedicated_ip_pools.rs
+++ generated/src/operation/list_dedicated_ip_pools.rs
@@ -268,10 +268,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_dedicated_ip_pools::ser_list_dedicated_ip_pools_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_deliverability_test_reports.rs`

```diff
--- reference/src/operation/list_deliverability_test_reports.rs
+++ generated/src/operation/list_deliverability_test_reports.rs
@@ -274,10 +274,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_deliverability_test_reports::ser_list_deliverability_test_reports_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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
@@ -315,10 +315,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_domain_deliverability_campaigns::ser_list_domain_deliverability_campaigns_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_email_identities.rs`

```diff
--- reference/src/operation/list_email_identities.rs
+++ generated/src/operation/list_email_identities.rs
@@ -268,10 +268,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_email_identities::ser_list_email_identities_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_email_templates.rs`

```diff
--- reference/src/operation/list_email_templates.rs
+++ generated/src/operation/list_email_templates.rs
@@ -268,10 +268,15 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_email_templates::ser_list_email_templates_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_multi_region_endpoints.rs`

```diff
--- reference/src/operation/list_multi_region_endpoints.rs
+++ generated/src/operation/list_multi_region_endpoints.rs
@@ -268,10 +268,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_multi_region_endpoints::ser_list_multi_region_endpoints_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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
@@ -301,10 +299,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_suppressed_destinations::ser_list_suppressed_destinations_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_tags_for_resource.rs`

```diff
--- reference/src/operation/list_tags_for_resource.rs
+++ generated/src/operation/list_tags_for_resource.rs
@@ -269,10 +269,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_tags_for_resource::ser_list_tags_for_resource_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/put_account_dedicated_ip_warmup_attributes/_put_account_dedicated_ip_warmup_attributes_input.rs`

```diff
--- reference/src/operation/put_account_dedicated_ip_warmup_attributes/_put_account_dedicated_ip_warmup_attributes_input.rs
+++ generated/src/operation/put_account_dedicated_ip_warmup_attributes/_put_account_dedicated_ip_warmup_attributes_input.rs
@@ -50,7 +50,7 @@
     > {
         ::std::result::Result::Ok(
             super::operation::put_account_dedicated_ip_warmup_attributes::PutAccountDedicatedIpWarmupAttributesInput {
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
         ::std::result::Result::Ok(super::operation::put_account_sending_attributes::PutAccountSendingAttributesInput {
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
             super::operation::put_configuration_set_reputation_options::PutConfigurationSetReputationOptionsInput {
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
             super::operation::put_configuration_set_sending_options::PutConfigurationSetSendingOptionsInput {
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
             super::operation::put_deliverability_dashboard_option::PutDeliverabilityDashboardOptionInput {
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
             super::operation::put_email_identity_dkim_attributes::PutEmailIdentityDkimAttributesInput {
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
             super::operation::put_email_identity_feedback_attributes::PutEmailIdentityFeedbackAttributesInput {
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
@@ -273,10 +271,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_untag_resource::ser_untag_resource_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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
-                    builder = builder.set_errors(super::protocol_serde::shape_metric_data_error_list::de_metric_data_error_list(
+                "Results" => {
+                    builder = builder.set_results(super::protocol_serde::shape_metric_data_result_list::de_metric_data_result_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "Results" => {
-                    builder = builder.set_results(super::protocol_serde::shape_metric_data_result_list::de_metric_data_result_list(
+                "Errors" => {
+                    builder = builder.set_errors(super::protocol_serde::shape_metric_data_error_list::de_metric_data_error_list(
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

### `src/protocol_serde/shape_cancel_export_job.rs`

```diff
--- reference/src/protocol_serde/shape_cancel_export_job.rs
+++ generated/src/protocol_serde/shape_cancel_export_job.rs
@@ -79,3 +79,46 @@
         output.build()
     })
 }
+
+pub fn ser_cancel_export_job_input(
+    input: &super::operation::cancel_export_job::CancelExportJobInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_cancel_export_job_input::ser_cancel_export_job_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_cancel_export_job(
+    _value: &[u8],
+    mut builder: super::operation::cancel_export_job::builders::CancelExportJobOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::cancel_export_job::builders::CancelExportJobOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_configuration_set.rs`

```diff
--- reference/src/protocol_serde/shape_create_configuration_set.rs
+++ generated/src/protocol_serde/shape_create_configuration_set.rs
@@ -149,3 +149,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_configuration_set(
+    _value: &[u8],
+    mut builder: super::operation::create_configuration_set::builders::CreateConfigurationSetOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_configuration_set::builders::CreateConfigurationSetOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_configuration_set_event_destination.rs`

```diff
--- reference/src/protocol_serde/shape_create_configuration_set_event_destination.rs
+++ generated/src/protocol_serde/shape_create_configuration_set_event_destination.rs
@@ -150,3 +150,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_configuration_set_event_destination(
+    _value: &[u8],
+    mut builder: super::operation::create_configuration_set_event_destination::builders::CreateConfigurationSetEventDestinationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_configuration_set_event_destination::builders::CreateConfigurationSetEventDestinationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_configuration_set_event_destination_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_configuration_set_event_destination_input.rs
+++ generated/src/protocol_serde/shape_create_configuration_set_event_destination_input.rs
@@ -3,14 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_configuration_set_event_destination::CreateConfigurationSetEventDestinationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.event_destination {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("EventDestination").start_object();
-        super::protocol_serde::shape_event_destination_definition::ser_event_destination_definition(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.event_destination_name {
-        object.key("EventDestinationName").string(var_3.as_str());
+    if let Some(var_2) = &input.event_destination_name {
+        object.key("EventDestinationName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.event_destination {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("EventDestination").start_object();
+        super::protocol_serde::shape_event_destination_definition::ser_event_destination_definition(&mut object_4, var_3)?;
+        object_4.finish();
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
     input: &super::operation::create_configuration_set::CreateConfigurationSetInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.archiving_options {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("ArchivingOptions").start_object();
-        super::protocol_serde::shape_archiving_options::ser_archiving_options(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_3.as_str());
+    if let Some(var_2) = &input.tracking_options {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("TrackingOptions").start_object();
+        super::protocol_serde::shape_tracking_options::ser_tracking_options(&mut object_3, var_2)?;
+        object_3.finish();
     }
     if let Some(var_4) = &input.delivery_options {
         #[allow(unused_mut)]
@@ -30,34 +30,34 @@
         super::protocol_serde::shape_sending_options::ser_sending_options(&mut object_9, var_8)?;
         object_9.finish();
     }
-    if let Some(var_10) = &input.suppression_options {
-        #[allow(unused_mut)]
-        let mut object_11 = object.key("SuppressionOptions").start_object();
-        super::protocol_serde::shape_suppression_options::ser_suppression_options(&mut object_11, var_10)?;
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
-                super::protocol_serde::shape_tag::ser_tag(&mut object_15, item_14)?;
-                object_15.finish();
+                let mut object_13 = array_11.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
+                object_13.finish();
             }
         }
-        array_13.finish();
+        array_11.finish();
+    }
+    if let Some(var_14) = &input.suppression_options {
+        #[allow(unused_mut)]
+        let mut object_15 = object.key("SuppressionOptions").start_object();
+        super::protocol_serde::shape_suppression_options::ser_suppression_options(&mut object_15, var_14)?;
+        object_15.finish();
     }
-    if let Some(var_16) = &input.tracking_options {
+    if let Some(var_16) = &input.vdm_options {
         #[allow(unused_mut)]
-        let mut object_17 = object.key("TrackingOptions").start_object();
-        super::protocol_serde::shape_tracking_options::ser_tracking_options(&mut object_17, var_16)?;
+        let mut object_17 = object.key("VdmOptions").start_object();
+        super::protocol_serde::shape_vdm_options::ser_vdm_options(&mut object_17, var_16)?;
         object_17.finish();
     }
-    if let Some(var_18) = &input.vdm_options {
+    if let Some(var_18) = &input.archiving_options {
         #[allow(unused_mut)]
-        let mut object_19 = object.key("VdmOptions").start_object();
-        super::protocol_serde::shape_vdm_options::ser_vdm_options(&mut object_19, var_18)?;
+        let mut object_19 = object.key("ArchivingOptions").start_object();
+        super::protocol_serde::shape_archiving_options::ser_archiving_options(&mut object_19, var_18)?;
         object_19.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_create_contact.rs`

```diff
--- reference/src/protocol_serde/shape_create_contact.rs
+++ generated/src/protocol_serde/shape_create_contact.rs
@@ -104,3 +104,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_contact(
+    _value: &[u8],
+    mut builder: super::operation::create_contact::builders::CreateContactOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_contact::builders::CreateContactOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_contact_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_contact_input.rs
+++ generated/src/protocol_serde/shape_create_contact_input.rs
@@ -3,8 +3,8 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_contact::CreateContactInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.attributes_data {
-        object.key("AttributesData").string(var_1.as_str());
+    if let Some(var_1) = &input.contact_list_name {
+        object.key("ContactListName").string(var_1.as_str());
     }
     if let Some(var_2) = &input.email_address {
         object.key("EmailAddress").string(var_2.as_str());
@@ -24,5 +24,8 @@
     if let Some(var_7) = &input.unsubscribe_all {
         object.key("UnsubscribeAll").boolean(*var_7);
     }
+    if let Some(var_8) = &input.attributes_data {
+        object.key("AttributesData").string(var_8.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_contact_list.rs`

```diff
--- reference/src/protocol_serde/shape_create_contact_list.rs
+++ generated/src/protocol_serde/shape_create_contact_list.rs
@@ -106,3 +106,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_contact_list(
+    _value: &[u8],
+    mut builder: super::operation::create_contact_list::builders::CreateContactListOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_contact_list::builders::CreateContactListOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
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
-                super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::protocol_serde::shape_topic::ser_topic(&mut object_5, item_4)?;
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
-                super::protocol_serde::shape_topic::ser_topic(&mut object_10, item_9)?;
+                super::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                 object_10.finish();
             }
         }
```

### `src/protocol_serde/shape_create_custom_verification_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_create_custom_verification_email_template.rs
+++ generated/src/protocol_serde/shape_create_custom_verification_email_template.rs
@@ -150,3 +150,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_custom_verification_email_template(
+    _value: &[u8],
+    mut builder: super::operation::create_custom_verification_email_template::builders::CreateCustomVerificationEmailTemplateOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_custom_verification_email_template::builders::CreateCustomVerificationEmailTemplateOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_custom_verification_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_custom_verification_email_template_input.rs
+++ generated/src/protocol_serde/shape_create_custom_verification_email_template_input.rs
@@ -3,35 +3,35 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_custom_verification_email_template::CreateCustomVerificationEmailTemplateInput,
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
-                super::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
-                object_7.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
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

### `src/protocol_serde/shape_create_dedicated_ip_pool.rs`

```diff
--- reference/src/protocol_serde/shape_create_dedicated_ip_pool.rs
+++ generated/src/protocol_serde/shape_create_dedicated_ip_pool.rs
@@ -130,3 +130,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_dedicated_ip_pool(
+    _value: &[u8],
+    mut builder: super::operation::create_dedicated_ip_pool::builders::CreateDedicatedIpPoolOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_dedicated_ip_pool::builders::CreateDedicatedIpPoolOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
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
-                super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
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
-                            .map(|s| s.to_unescaped().map(|u| super::types::DeliverabilityTestStatus::from(u.as_ref())))
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
+                            .map(|s| s.to_unescaped().map(|u| super::types::DeliverabilityTestStatus::from(u.as_ref())))
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
     input: &super::operation::create_deliverability_test_report::CreateDeliverabilityTestReportInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.content {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("Content").start_object();
-        super::protocol_serde::shape_email_content::ser_email_content(&mut object_2, var_1)?;
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
+        super::protocol_serde::shape_email_content::ser_email_content(&mut object_4, var_3)?;
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
-                    builder = builder.set_dkim_attributes(super::protocol_serde::shape_dkim_attributes::de_dkim_attributes(
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
+                    builder = builder.set_dkim_attributes(super::protocol_serde::shape_dkim_attributes::de_dkim_attributes(
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
     input: &super::operation::create_email_identity::CreateEmailIdentityInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_1.as_str());
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.dkim_signing_attributes {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("DkimSigningAttributes").start_object();
-        super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_3, var_2)?;
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
-                super::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
-                object_8.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_6.finish();
+        array_3.finish();
+    }
+    if let Some(var_6) = &input.dkim_signing_attributes {
+        #[allow(unused_mut)]
+        let mut object_7 = object.key("DkimSigningAttributes").start_object();
+        super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_7, var_6)?;
+        object_7.finish();
+    }
+    if let Some(var_8) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_8.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_email_identity_policy.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_identity_policy.rs
+++ generated/src/protocol_serde/shape_create_email_identity_policy.rs
@@ -129,3 +129,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_email_identity_policy(
+    _value: &[u8],
+    mut builder: super::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_email_identity_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_identity_policy_input.rs
+++ generated/src/protocol_serde/shape_create_email_identity_policy_input.rs
@@ -3,8 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_email_identity_policy::CreateEmailIdentityPolicyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.policy {
-        object.key("Policy").string(var_1.as_str());
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.policy_name {
+        object.key("PolicyName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.policy {
+        object.key("Policy").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_template.rs
+++ generated/src/protocol_serde/shape_create_email_template.rs
@@ -110,3 +110,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_email_template(
+    _value: &[u8],
+    mut builder: super::operation::create_email_template::builders::CreateEmailTemplateOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_email_template::builders::CreateEmailTemplateOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_email_template_input.rs
+++ generated/src/protocol_serde/shape_create_email_template_input.rs
@@ -3,26 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_email_template::CreateEmailTemplateInput,
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
+        super::protocol_serde::shape_email_template_content::ser_email_template_content(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    if let Some(var_4) = &input.tags {
+        let mut array_5 = object.key("Tags").start_array();
+        for item_6 in var_4 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::protocol_serde::shape_tag::ser_tag(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_7 = array_5.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
+                object_7.finish();
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_5) = &input.template_content {
-        #[allow(unused_mut)]
-        let mut object_6 = object.key("TemplateContent").start_object();
-        super::protocol_serde::shape_email_template_content::ser_email_template_content(&mut object_6, var_5)?;
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
     input: &super::operation::create_import_job::CreateImportJobInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.import_data_source {
+    if let Some(var_1) = &input.import_destination {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("ImportDataSource").start_object();
-        super::protocol_serde::shape_import_data_source::ser_import_data_source(&mut object_2, var_1)?;
+        let mut object_2 = object.key("ImportDestination").start_object();
+        super::protocol_serde::shape_import_destination::ser_import_destination(&mut object_2, var_1)?;
         object_2.finish();
     }
-    if let Some(var_3) = &input.import_destination {
+    if let Some(var_3) = &input.import_data_source {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("ImportDestination").start_object();
-        super::protocol_serde::shape_import_destination::ser_import_destination(&mut object_4, var_3)?;
+        let mut object_4 = object.key("ImportDataSource").start_object();
+        super::protocol_serde::shape_import_data_source::ser_import_data_source(&mut object_4, var_3)?;
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
+                            .map(|s| s.to_unescaped().map(|u| super::types::Status::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "Status" => {
-                    builder = builder.set_status(
+                "EndpointId" => {
+                    builder = builder.set_endpoint_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::Status::from(u.as_ref())))
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
     input: &super::operation::create_multi_region_endpoint::CreateMultiRegionEndpointInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.details {
+    if let Some(var_1) = &input.endpoint_name {
+        object.key("EndpointName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.details {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("Details").start_object();
-        super::protocol_serde::shape_details::ser_details(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.endpoint_name {
-        object.key("EndpointName").string(var_3.as_str());
+        let mut object_3 = object.key("Details").start_object();
+        super::protocol_serde::shape_details::ser_details(&mut object_3, var_2)?;
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
-                            .map(|s| s.to_unescaped().map(|u| super::types::SendingStatus::from(u.as_ref())))
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "SuppressionAttributes" => {
-                    builder = builder.set_suppression_attributes(
-                        super::protocol_serde::shape_tenant_suppression_attributes::de_tenant_suppression_attributes(tokens, _value, depth + 1)?,
+                "TenantId" => {
+                    builder = builder.set_tenant_id(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "Tags" => {
-                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
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
+                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                }
+                "SendingStatus" => {
+                    builder = builder.set_sending_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::SendingStatus::from(u.as_ref())))
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
+                        super::protocol_serde::shape_tenant_suppression_attributes::de_tenant_suppression_attributes(tokens, _value, depth + 1)?,
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
     input: &super::operation::create_tenant::CreateTenantInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.suppression_attributes {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("SuppressionAttributes").start_object();
-        super::protocol_serde::shape_tenant_suppression_attributes::ser_tenant_suppression_attributes(&mut object_2, var_1)?;
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
-                super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
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
+        super::protocol_serde::shape_tenant_suppression_attributes::ser_tenant_suppression_attributes(&mut object_7, var_6)?;
+        object_7.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_tenant_resource_association.rs`

```diff
--- reference/src/protocol_serde/shape_create_tenant_resource_association.rs
+++ generated/src/protocol_serde/shape_create_tenant_resource_association.rs
@@ -115,3 +115,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_tenant_resource_association(
+    _value: &[u8],
+    mut builder: super::operation::create_tenant_resource_association::builders::CreateTenantResourceAssociationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::create_tenant_resource_association::builders::CreateTenantResourceAssociationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_create_tenant_resource_association_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_tenant_resource_association_input.rs
+++ generated/src/protocol_serde/shape_create_tenant_resource_association_input.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_tenant_resource_association::CreateTenantResourceAssociationInput,
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
+    input: &super::types::DashboardAttributes,
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
-    input: &super::types::DashboardAttributes,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.engagement_metrics {
-        object.key("EngagementMetrics").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_delete_configuration_set.rs`

```diff
--- reference/src/protocol_serde/shape_delete_configuration_set.rs
+++ generated/src/protocol_serde/shape_delete_configuration_set.rs
@@ -109,3 +109,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_configuration_set_input(
+    input: &super::operation::delete_configuration_set::DeleteConfigurationSetInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_configuration_set_input::ser_delete_configuration_set_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_configuration_set(
+    _value: &[u8],
+    mut builder: super::operation::delete_configuration_set::builders::DeleteConfigurationSetOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_configuration_set::builders::DeleteConfigurationSetOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_configuration_set_event_destination.rs`

```diff
--- reference/src/protocol_serde/shape_delete_configuration_set_event_destination.rs
+++ generated/src/protocol_serde/shape_delete_configuration_set_event_destination.rs
@@ -99,3 +99,49 @@
         output.build()
     })
 }
+
+pub fn ser_delete_configuration_set_event_destination_input(
+    input: &super::operation::delete_configuration_set_event_destination::DeleteConfigurationSetEventDestinationInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_configuration_set_event_destination_input::ser_delete_configuration_set_event_destination_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_configuration_set_event_destination(
+    _value: &[u8],
+    mut builder: super::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_contact.rs`

```diff
--- reference/src/protocol_serde/shape_delete_contact.rs
+++ generated/src/protocol_serde/shape_delete_contact.rs
@@ -79,3 +79,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_contact_input(
+    input: &super::operation::delete_contact::DeleteContactInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_contact_input::ser_delete_contact_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_contact(
+    _value: &[u8],
+    mut builder: super::operation::delete_contact::builders::DeleteContactOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_contact::builders::DeleteContactOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_contact_list.rs`

```diff
--- reference/src/protocol_serde/shape_delete_contact_list.rs
+++ generated/src/protocol_serde/shape_delete_contact_list.rs
@@ -99,3 +99,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_contact_list_input(
+    input: &super::operation::delete_contact_list::DeleteContactListInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_contact_list_input::ser_delete_contact_list_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_contact_list(
+    _value: &[u8],
+    mut builder: super::operation::delete_contact_list::builders::DeleteContactListOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_contact_list::builders::DeleteContactListOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_custom_verification_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_delete_custom_verification_email_template.rs
+++ generated/src/protocol_serde/shape_delete_custom_verification_email_template.rs
@@ -99,3 +99,49 @@
         output.build()
     })
 }
+
+pub fn ser_delete_custom_verification_email_template_input(
+    input: &super::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_custom_verification_email_template_input::ser_delete_custom_verification_email_template_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_custom_verification_email_template(
+    _value: &[u8],
+    mut builder: super::operation::delete_custom_verification_email_template::builders::DeleteCustomVerificationEmailTemplateOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_custom_verification_email_template::builders::DeleteCustomVerificationEmailTemplateOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_dedicated_ip_pool.rs`

```diff
--- reference/src/protocol_serde/shape_delete_dedicated_ip_pool.rs
+++ generated/src/protocol_serde/shape_delete_dedicated_ip_pool.rs
@@ -105,3 +105,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_dedicated_ip_pool_input(
+    input: &super::operation::delete_dedicated_ip_pool::DeleteDedicatedIpPoolInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_dedicated_ip_pool_input::ser_delete_dedicated_ip_pool_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_dedicated_ip_pool(
+    _value: &[u8],
+    mut builder: super::operation::delete_dedicated_ip_pool::builders::DeleteDedicatedIpPoolOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_dedicated_ip_pool::builders::DeleteDedicatedIpPoolOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_email_identity.rs`

```diff
--- reference/src/protocol_serde/shape_delete_email_identity.rs
+++ generated/src/protocol_serde/shape_delete_email_identity.rs
@@ -103,3 +103,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_email_identity_input(
+    input: &super::operation::delete_email_identity::DeleteEmailIdentityInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_email_identity_input::ser_delete_email_identity_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_email_identity(
+    _value: &[u8],
+    mut builder: super::operation::delete_email_identity::builders::DeleteEmailIdentityOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_email_identity::builders::DeleteEmailIdentityOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_email_identity_policy.rs`

```diff
--- reference/src/protocol_serde/shape_delete_email_identity_policy.rs
+++ generated/src/protocol_serde/shape_delete_email_identity_policy.rs
@@ -89,3 +89,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_email_identity_policy_input(
+    input: &super::operation::delete_email_identity_policy::DeleteEmailIdentityPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_email_identity_policy_input::ser_delete_email_identity_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_email_identity_policy(
+    _value: &[u8],
+    mut builder: super::operation::delete_email_identity_policy::builders::DeleteEmailIdentityPolicyOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_email_identity_policy::builders::DeleteEmailIdentityPolicyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_delete_email_template.rs
+++ generated/src/protocol_serde/shape_delete_email_template.rs
@@ -85,3 +85,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_email_template_input(
+    input: &super::operation::delete_email_template::DeleteEmailTemplateInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_email_template_input::ser_delete_email_template_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_email_template(
+    _value: &[u8],
+    mut builder: super::operation::delete_email_template::builders::DeleteEmailTemplateOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_email_template::builders::DeleteEmailTemplateOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_multi_region_endpoint.rs`

```diff
--- reference/src/protocol_serde/shape_delete_multi_region_endpoint.rs
+++ generated/src/protocol_serde/shape_delete_multi_region_endpoint.rs
@@ -112,6 +112,16 @@
     })
 }

+pub fn ser_delete_multi_region_endpoint_input(
+    input: &super::operation::delete_multi_region_endpoint::DeleteMultiRegionEndpointInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_multi_region_endpoint_input::ser_delete_multi_region_endpoint_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_multi_region_endpoint(
     _value: &[u8],
     mut builder: super::operation::delete_multi_region_endpoint::builders::DeleteMultiRegionEndpointOutputBuilder,
```

### `src/protocol_serde/shape_delete_suppressed_destination.rs`

```diff
--- reference/src/protocol_serde/shape_delete_suppressed_destination.rs
+++ generated/src/protocol_serde/shape_delete_suppressed_destination.rs
@@ -85,3 +85,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_suppressed_destination_input(
+    input: &super::operation::delete_suppressed_destination::DeleteSuppressedDestinationInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_suppressed_destination_input::ser_delete_suppressed_destination_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_suppressed_destination(
+    _value: &[u8],
+    mut builder: super::operation::delete_suppressed_destination::builders::DeleteSuppressedDestinationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_suppressed_destination::builders::DeleteSuppressedDestinationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_tenant.rs`

```diff
--- reference/src/protocol_serde/shape_delete_tenant.rs
+++ generated/src/protocol_serde/shape_delete_tenant.rs
@@ -89,3 +89,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_tenant(
+    _value: &[u8],
+    mut builder: super::operation::delete_tenant::builders::DeleteTenantOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_tenant::builders::DeleteTenantOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_tenant_resource_association.rs`

```diff
--- reference/src/protocol_serde/shape_delete_tenant_resource_association.rs
+++ generated/src/protocol_serde/shape_delete_tenant_resource_association.rs
@@ -98,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_tenant_resource_association(
+    _value: &[u8],
+    mut builder: super::operation::delete_tenant_resource_association::builders::DeleteTenantResourceAssociationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_tenant_resource_association::builders::DeleteTenantResourceAssociationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_tenant_resource_association_input.rs`

```diff
--- reference/src/protocol_serde/shape_delete_tenant_resource_association_input.rs
+++ generated/src/protocol_serde/shape_delete_tenant_resource_association_input.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput,
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
     input: &super::types::EventDestinationDefinition,
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
-                        let key = key.to_unescaped().map(|u| super::types::MetricDimensionName::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = super::protocol_serde::shape_export_dimension_value::de_export_dimension_value(tokens, _value, depth + 1)?;
                         match value {
                             Some(value) => {
```

### `src/protocol_serde/shape_get_account.rs`

```diff
--- reference/src/protocol_serde/shape_get_account.rs
+++ generated/src/protocol_serde/shape_get_account.rs
@@ -67,6 +67,12 @@
     })
 }

+pub fn ser_get_account_input(
+    _input: &super::operation::get_account::GetAccountInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
+}
+
 pub(crate) fn de_get_account(
     _value: &[u8],
     mut builder: super::operation::get_account::builders::GetAccountOutputBuilder,
@@ -85,13 +91,6 @@
                     builder =
                         builder.set_dedicated_ip_auto_warmup_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                 }
-                "Details" => {
-                    builder = builder.set_details(super::protocol_serde::shape_account_details::de_account_details(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "EnforcementStatus" => {
                     builder = builder.set_enforcement_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -99,13 +98,6 @@
                             .transpose()?,
                     );
                 }
-                "PricingAttributes" => {
-                    builder = builder.set_pricing_attributes(super::protocol_serde::shape_pricing_attributes::de_pricing_attributes(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "ProductionAccessEnabled" => {
                     builder = builder.set_production_access_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                 }
@@ -122,9 +114,23 @@
                         depth + 1,
                     )?);
                 }
+                "Details" => {
+                    builder = builder.set_details(super::protocol_serde::shape_account_details::de_account_details(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "VdmAttributes" => {
                     builder = builder.set_vdm_attributes(super::protocol_serde::shape_vdm_attributes::de_vdm_attributes(tokens, _value, depth + 1)?);
                 }
+                "PricingAttributes" => {
+                    builder = builder.set_pricing_attributes(super::protocol_serde::shape_pricing_attributes::de_pricing_attributes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_blacklist_reports.rs`

```diff
--- reference/src/protocol_serde/shape_get_blacklist_reports.rs
+++ generated/src/protocol_serde/shape_get_blacklist_reports.rs
@@ -90,6 +90,16 @@
     })
 }

+pub fn ser_get_blacklist_reports_input(
+    input: &super::operation::get_blacklist_reports::GetBlacklistReportsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_blacklist_reports_input::ser_get_blacklist_reports_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_blacklist_reports(
     _value: &[u8],
     mut builder: super::operation::get_blacklist_reports::builders::GetBlacklistReportsOutputBuilder,
```

### `src/protocol_serde/shape_get_configuration_set.rs`

```diff
--- reference/src/protocol_serde/shape_get_configuration_set.rs
+++ generated/src/protocol_serde/shape_get_configuration_set.rs
@@ -88,6 +88,16 @@
     })
 }

+pub fn ser_get_configuration_set_input(
+    input: &super::operation::get_configuration_set::GetConfigurationSetInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_configuration_set_input::ser_get_configuration_set_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_configuration_set(
     _value: &[u8],
     mut builder: super::operation::get_configuration_set::builders::GetConfigurationSetOutputBuilder,
@@ -104,13 +114,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "ArchivingOptions" => {
-                    builder = builder.set_archiving_options(super::protocol_serde::shape_archiving_options::de_archiving_options(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "ConfigurationSetName" => {
                     builder = builder.set_configuration_set_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -118,6 +121,13 @@
                             .transpose()?,
                     );
                 }
+                "TrackingOptions" => {
+                    builder = builder.set_tracking_options(super::protocol_serde::shape_tracking_options::de_tracking_options(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "DeliveryOptions" => {
                     builder = builder.set_delivery_options(super::protocol_serde::shape_delivery_options::de_delivery_options(
                         tokens,
@@ -139,6 +149,9 @@
                         depth + 1,
                     )?);
                 }
+                "Tags" => {
+                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                }
                 "SuppressionOptions" => {
                     builder = builder.set_suppression_options(super::protocol_serde::shape_suppression_options::de_suppression_options(
                         tokens,
@@ -146,19 +159,16 @@
                         depth + 1,
                     )?);
                 }
-                "Tags" => {
-                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                "VdmOptions" => {
+                    builder = builder.set_vdm_options(super::protocol_serde::shape_vdm_options::de_vdm_options(tokens, _value, depth + 1)?);
                 }
-                "TrackingOptions" => {
-                    builder = builder.set_tracking_options(super::protocol_serde::shape_tracking_options::de_tracking_options(
+                "ArchivingOptions" => {
+                    builder = builder.set_archiving_options(super::protocol_serde::shape_archiving_options::de_archiving_options(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "VdmOptions" => {
-                    builder = builder.set_vdm_options(super::protocol_serde::shape_vdm_options::de_vdm_options(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_configuration_set_event_destinations.rs`

```diff
--- reference/src/protocol_serde/shape_get_configuration_set_event_destinations.rs
+++ generated/src/protocol_serde/shape_get_configuration_set_event_destinations.rs
@@ -103,6 +103,19 @@
     })
 }

+pub fn ser_get_configuration_set_event_destinations_input(
+    input: &super::operation::get_configuration_set_event_destinations::GetConfigurationSetEventDestinationsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_configuration_set_event_destinations_input::ser_get_configuration_set_event_destinations_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_configuration_set_event_destinations(
     _value: &[u8],
     mut builder: super::operation::get_configuration_set_event_destinations::builders::GetConfigurationSetEventDestinationsOutputBuilder,
```

### `src/protocol_serde/shape_get_contact.rs`

```diff
--- reference/src/protocol_serde/shape_get_contact.rs
+++ generated/src/protocol_serde/shape_get_contact.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_contact_input(
+    input: &super::operation::get_contact::GetContactInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_contact_input::ser_get_contact_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_contact(
     _value: &[u8],
     mut builder: super::operation::get_contact::builders::GetContactOutputBuilder,
@@ -97,13 +107,6 @@
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
@@ -111,12 +114,6 @@
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
@@ -124,10 +121,11 @@
                                 .transpose()?,
                         );
                     }
-                    "LastUpdatedTimestamp" => {
-                        builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                            tokens.next(),
-                            ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    "TopicPreferences" => {
+                        builder = builder.set_topic_preferences(super::protocol_serde::shape_topic_preference_list::de_topic_preference_list(
+                            tokens,
+                            _value,
+                            depth + 1,
                         )?);
                     }
                     "TopicDefaultPreferences" => {
@@ -135,16 +133,28 @@
                             super::protocol_serde::shape_topic_preference_list::de_topic_preference_list(tokens, _value, depth + 1)?,
                         );
                     }
-                    "TopicPreferences" => {
-                        builder = builder.set_topic_preferences(super::protocol_serde::shape_topic_preference_list::de_topic_preference_list(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
                     "UnsubscribeAll" => {
                         builder = builder.set_unsubscribe_all(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                     }
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
+                        )?);
+                    }
+                    "LastUpdatedTimestamp" => {
+                        builder = builder.set_last_updated_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                            tokens.next(),
+                            ::aws_smithy_types::date_time::Format::EpochSeconds,
+                        )?);
+                    }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
             }
```

### `src/protocol_serde/shape_get_contact_list.rs`

```diff
--- reference/src/protocol_serde/shape_get_contact_list.rs
+++ generated/src/protocol_serde/shape_get_contact_list.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_contact_list_input(
+    input: &super::operation::get_contact_list::GetContactListInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_contact_list_input::ser_get_contact_list_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_contact_list(
     _value: &[u8],
     mut builder: super::operation::get_contact_list::builders::GetContactListOutputBuilder,
@@ -105,11 +115,8 @@
                             .transpose()?,
                     );
                 }
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
+                "Topics" => {
+                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
                 }
                 "Description" => {
                     builder = builder.set_description(
@@ -118,6 +125,12 @@
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
@@ -127,9 +140,6 @@
                 "Tags" => {
                     builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
-                "Topics" => {
-                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_custom_verification_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_get_custom_verification_email_template.rs
+++ generated/src/protocol_serde/shape_get_custom_verification_email_template.rs
@@ -97,6 +97,19 @@
     })
 }

+pub fn ser_get_custom_verification_email_template_input(
+    input: &super::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_custom_verification_email_template_input::ser_get_custom_verification_email_template_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_custom_verification_email_template(
     _value: &[u8],
     mut builder: super::operation::get_custom_verification_email_template::builders::GetCustomVerificationEmailTemplateOutputBuilder,
@@ -113,8 +126,8 @@
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
@@ -127,16 +140,13 @@
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
-                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
-                }
                 "TemplateContent" => {
                     builder = builder.set_template_content(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -144,15 +154,18 @@
                             .transpose()?,
                     );
                 }
-                "TemplateName" => {
-                    builder = builder.set_template_name(
+                "Tags" => {
+                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
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

### `src/protocol_serde/shape_get_dedicated_ip.rs`

```diff
--- reference/src/protocol_serde/shape_get_dedicated_ip.rs
+++ generated/src/protocol_serde/shape_get_dedicated_ip.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_dedicated_ip_input(
+    input: &super::operation::get_dedicated_ip::GetDedicatedIpInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_dedicated_ip_input::ser_get_dedicated_ip_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_dedicated_ip(
     _value: &[u8],
     mut builder: super::operation::get_dedicated_ip::builders::GetDedicatedIpOutputBuilder,
```

### `src/protocol_serde/shape_get_dedicated_ip_pool.rs`

```diff
--- reference/src/protocol_serde/shape_get_dedicated_ip_pool.rs
+++ generated/src/protocol_serde/shape_get_dedicated_ip_pool.rs
@@ -88,6 +88,16 @@
     })
 }

+pub fn ser_get_dedicated_ip_pool_input(
+    input: &super::operation::get_dedicated_ip_pool::GetDedicatedIpPoolInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_dedicated_ip_pool_input::ser_get_dedicated_ip_pool_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_dedicated_ip_pool(
     _value: &[u8],
     mut builder: super::operation::get_dedicated_ip_pool::builders::GetDedicatedIpPoolOutputBuilder,
```

### `src/protocol_serde/shape_get_dedicated_ips.rs`

```diff
--- reference/src/protocol_serde/shape_get_dedicated_ips.rs
+++ generated/src/protocol_serde/shape_get_dedicated_ips.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_dedicated_ips_input(
+    input: &super::operation::get_dedicated_ips::GetDedicatedIpsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_dedicated_ips_input::ser_get_dedicated_ips_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_dedicated_ips(
     _value: &[u8],
     mut builder: super::operation::get_dedicated_ips::builders::GetDedicatedIpsOutputBuilder,
```

### `src/protocol_serde/shape_get_deliverability_dashboard_options.rs`

```diff
--- reference/src/protocol_serde/shape_get_deliverability_dashboard_options.rs
+++ generated/src/protocol_serde/shape_get_deliverability_dashboard_options.rs
@@ -91,10 +91,18 @@
         output = super::protocol_serde::shape_get_deliverability_dashboard_options::de_get_deliverability_dashboard_options(_response_body, output)
             .map_err(super::operation::get_deliverability_dashboard_options::GetDeliverabilityDashboardOptionsError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::serde_util::get_deliverability_dashboard_options_output_output_correct_errors(output).build()
+        super::serde_util::get_deliverability_dashboard_options_output_output_correct_errors(output)
+            .build()
+            .map_err(super::operation::get_deliverability_dashboard_options::GetDeliverabilityDashboardOptionsError::unhandled)?
     })
 }

+pub fn ser_get_deliverability_dashboard_options_input(
+    _input: &super::operation::get_deliverability_dashboard_options::GetDeliverabilityDashboardOptionsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
+}
+
 pub(crate) fn de_get_deliverability_dashboard_options(
     _value: &[u8],
     mut builder: super::operation::get_deliverability_dashboard_options::builders::GetDeliverabilityDashboardOptionsOutputBuilder,
@@ -111,6 +119,15 @@
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
@@ -130,9 +147,6 @@
                         )?,
                     );
                 }
-                "DashboardEnabled" => {
-                    builder = builder.set_dashboard_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
                 "PendingExpirationSubscribedDomains" => {
                     builder = builder.set_pending_expiration_subscribed_domains(
                         super::protocol_serde::shape_domain_deliverability_tracking_options::de_domain_deliverability_tracking_options(
@@ -142,12 +156,6 @@
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
@@ -90,6 +90,16 @@
     })
 }

+pub fn ser_get_deliverability_test_report_input(
+    input: &super::operation::get_deliverability_test_report::GetDeliverabilityTestReportInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_deliverability_test_report_input::ser_get_deliverability_test_report_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_deliverability_test_report(
     _value: &[u8],
     mut builder: super::operation::get_deliverability_test_report::builders::GetDeliverabilityTestReportOutputBuilder,
@@ -111,6 +121,13 @@
                         super::protocol_serde::shape_deliverability_test_report::de_deliverability_test_report(tokens, _value, depth + 1)?,
                     );
                 }
+                "OverallPlacement" => {
+                    builder = builder.set_overall_placement(super::protocol_serde::shape_placement_statistics::de_placement_statistics(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "IspPlacements" => {
                     builder = builder.set_isp_placements(super::protocol_serde::shape_isp_placements::de_isp_placements(tokens, _value, depth + 1)?);
                 }
@@ -121,13 +138,6 @@
                             .transpose()?,
                     );
                 }
-                "OverallPlacement" => {
-                    builder = builder.set_overall_placement(super::protocol_serde::shape_placement_statistics::de_placement_statistics(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "Tags" => {
                     builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
```

### `src/protocol_serde/shape_get_domain_deliverability_campaign.rs`

```diff
--- reference/src/protocol_serde/shape_get_domain_deliverability_campaign.rs
+++ generated/src/protocol_serde/shape_get_domain_deliverability_campaign.rs
@@ -91,6 +91,16 @@
     })
 }

+pub fn ser_get_domain_deliverability_campaign_input(
+    input: &super::operation::get_domain_deliverability_campaign::GetDomainDeliverabilityCampaignInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_domain_deliverability_campaign_input::ser_get_domain_deliverability_campaign_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_domain_deliverability_campaign(
     _value: &[u8],
     mut builder: super::operation::get_domain_deliverability_campaign::builders::GetDomainDeliverabilityCampaignOutputBuilder,
```

### `src/protocol_serde/shape_get_domain_statistics_report.rs`

```diff
--- reference/src/protocol_serde/shape_get_domain_statistics_report.rs
+++ generated/src/protocol_serde/shape_get_domain_statistics_report.rs
@@ -94,6 +94,16 @@
     })
 }

+pub fn ser_get_domain_statistics_report_input(
+    input: &super::operation::get_domain_statistics_report::GetDomainStatisticsReportInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_domain_statistics_report_input::ser_get_domain_statistics_report_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_domain_statistics_report(
     _value: &[u8],
     mut builder: super::operation::get_domain_statistics_report::builders::GetDomainStatisticsReportOutputBuilder,
@@ -110,12 +120,12 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "OverallVolume" => {
+                    builder = builder.set_overall_volume(super::protocol_serde::shape_overall_volume::de_overall_volume(tokens, _value, depth + 1)?);
+                }
                 "DailyVolumes" => {
                     builder = builder.set_daily_volumes(super::protocol_serde::shape_daily_volumes::de_daily_volumes(tokens, _value, depth + 1)?);
                 }
-                "OverallVolume" => {
-                    builder = builder.set_overall_volume(super::protocol_serde::shape_overall_volume::de_overall_volume(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_email_identity.rs`

```diff
--- reference/src/protocol_serde/shape_get_email_identity.rs
+++ generated/src/protocol_serde/shape_get_email_identity.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_email_identity_input(
+    input: &super::operation::get_email_identity::GetEmailIdentityInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_email_identity_input::ser_get_email_identity_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_email_identity(
     _value: &[u8],
     mut builder: super::operation::get_email_identity::builders::GetEmailIdentityOutputBuilder,
@@ -98,13 +108,19 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "ConfigurationSetName" => {
-                    builder = builder.set_configuration_set_name(
+                "IdentityType" => {
+                    builder = builder.set_identity_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::IdentityType::from(u.as_ref())))
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
                     builder = builder.set_dkim_attributes(super::protocol_serde::shape_dkim_attributes::de_dkim_attributes(
                         tokens,
@@ -112,16 +128,6 @@
                         depth + 1,
                     )?);
                 }
-                "FeedbackForwardingStatus" => {
-                    builder = builder.set_feedback_forwarding_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
-                "IdentityType" => {
-                    builder = builder.set_identity_type(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::IdentityType::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 "MailFromAttributes" => {
                     builder = builder.set_mail_from_attributes(super::protocol_serde::shape_mail_from_attributes::de_mail_from_attributes(
                         tokens,
@@ -135,12 +141,12 @@
                 "Tags" => {
                     builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
-                "VerificationInfo" => {
-                    builder = builder.set_verification_info(super::protocol_serde::shape_verification_info::de_verification_info(
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
@@ -149,8 +155,12 @@
                             .transpose()?,
                     );
                 }
-                "VerifiedForSendingStatus" => {
-                    builder = builder.set_verified_for_sending_status(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                "VerificationInfo" => {
+                    builder = builder.set_verification_info(super::protocol_serde::shape_verification_info::de_verification_info(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_get_email_identity_policies.rs`

```diff
--- reference/src/protocol_serde/shape_get_email_identity_policies.rs
+++ generated/src/protocol_serde/shape_get_email_identity_policies.rs
@@ -92,6 +92,16 @@
     })
 }

+pub fn ser_get_email_identity_policies_input(
+    input: &super::operation::get_email_identity_policies::GetEmailIdentityPoliciesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_email_identity_policies_input::ser_get_email_identity_policies_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_email_identity_policies(
     _value: &[u8],
     mut builder: super::operation::get_email_identity_policies::builders::GetEmailIdentityPoliciesOutputBuilder,
```

### `src/protocol_serde/shape_get_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_get_email_template.rs
+++ generated/src/protocol_serde/shape_get_email_template.rs
@@ -84,6 +84,16 @@
     })
 }

+pub fn ser_get_email_template_input(
+    input: &super::operation::get_email_template::GetEmailTemplateInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_email_template_input::ser_get_email_template_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_email_template(
     _value: &[u8],
     mut builder: super::operation::get_email_template::builders::GetEmailTemplateOutputBuilder,
@@ -100,8 +110,12 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Tags" => {
-                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
+                "TemplateName" => {
+                    builder = builder.set_template_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
                 "TemplateContent" => {
                     builder = builder.set_template_content(super::protocol_serde::shape_email_template_content::de_email_template_content(
@@ -110,12 +124,8 @@
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
+                    builder = builder.set_tags(super::protocol_serde::shape_tag_list::de_tag_list(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_get_export_job.rs`

```diff
--- reference/src/protocol_serde/shape_get_export_job.rs
+++ generated/src/protocol_serde/shape_get_export_job.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_export_job_input(
+    input: &super::operation::get_export_job::GetExportJobInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_export_job_input::ser_get_export_job_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_export_job(
     _value: &[u8],
     mut builder: super::operation::get_export_job::builders::GetExportJobOutputBuilder,
@@ -98,16 +108,32 @@
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
+                            .map(|s| s.to_unescaped().map(|u| super::types::ExportSourceType::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CreatedTimestamp" => {
-                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                "JobStatus" => {
+                    builder = builder.set_job_status(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::JobStatus::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
+                "ExportDestination" => {
+                    builder = builder.set_export_destination(super::protocol_serde::shape_export_destination::de_export_destination(
+                        tokens,
+                        _value,
+                        depth + 1,
                     )?);
                 }
                 "ExportDataSource" => {
@@ -117,37 +143,21 @@
                         depth + 1,
                     )?);
                 }
-                "ExportDestination" => {
-                    builder = builder.set_export_destination(super::protocol_serde::shape_export_destination::de_export_destination(
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
-                            .map(|s| s.to_unescaped().map(|u| super::types::ExportSourceType::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "CompletedTimestamp" => {
+                    builder = builder.set_completed_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
                 }
                 "FailureInfo" => {
                     builder = builder.set_failure_info(super::protocol_serde::shape_failure_info::de_failure_info(tokens, _value, depth + 1)?);
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
-                            .map(|s| s.to_unescaped().map(|u| super::types::JobStatus::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 "Statistics" => {
                     builder = builder.set_statistics(super::protocol_serde::shape_export_statistics::de_export_statistics(
                         tokens,
```

### `src/protocol_serde/shape_get_import_job.rs`

```diff
--- reference/src/protocol_serde/shape_get_import_job.rs
+++ generated/src/protocol_serde/shape_get_import_job.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_get_import_job_input(
+    input: &super::operation::get_import_job::GetImportJobInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_import_job_input::ser_get_import_job_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_import_job(
     _value: &[u8],
     mut builder: super::operation::get_import_job::builders::GetImportJobOutputBuilder,
@@ -98,48 +108,29 @@
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
-                    builder = builder.set_failure_info(super::protocol_serde::shape_failure_info::de_failure_info(tokens, _value, depth + 1)?);
-                }
-                "ImportDataSource" => {
-                    builder = builder.set_import_data_source(super::protocol_serde::shape_import_data_source::de_import_data_source(
+                "ImportDestination" => {
+                    builder = builder.set_import_destination(super::protocol_serde::shape_import_destination::de_import_destination(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "ImportDestination" => {
-                    builder = builder.set_import_destination(super::protocol_serde::shape_import_destination::de_import_destination(
+                "ImportDataSource" => {
+                    builder = builder.set_import_data_source(super::protocol_serde::shape_import_data_source::de_import_data_source(
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
+                    builder = builder.set_failure_info(super::protocol_serde::shape_failure_info::de_failure_info(tokens, _value, depth + 1)?);
                 }
                 "JobStatus" => {
                     builder = builder.set_job_status(
@@ -148,6 +139,18 @@
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
@@ -155,6 +158,13 @@
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
@@ -88,6 +88,16 @@
     })
 }

+pub fn ser_get_message_insights_input(
+    input: &super::operation::get_message_insights::GetMessageInsightsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_message_insights_input::ser_get_message_insights_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_message_insights(
     _value: &[u8],
     mut builder: super::operation::get_message_insights::builders::GetMessageInsightsOutputBuilder,
@@ -104,29 +114,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "EmailTags" => {
-                    builder = builder.set_email_tags(super::protocol_serde::shape_message_tag_list::de_message_tag_list(
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
-                    builder = builder.set_insights(super::protocol_serde::shape_email_insights_list::de_email_insights_list(
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
@@ -139,6 +135,20 @@
                             .transpose()?,
                     );
                 }
+                "EmailTags" => {
+                    builder = builder.set_email_tags(super::protocol_serde::shape_message_tag_list::de_message_tag_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "Insights" => {
+                    builder = builder.set_insights(super::protocol_serde::shape_email_insights_list::de_email_insights_list(
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
@@ -92,6 +92,16 @@
     })
 }

+pub fn ser_get_multi_region_endpoint_input(
+    input: &super::operation::get_multi_region_endpoint::GetMultiRegionEndpointInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_multi_region_endpoint_input::ser_get_multi_region_endpoint_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_multi_region_endpoint(
     _value: &[u8],
     mut builder: super::operation::get_multi_region_endpoint::builders::GetMultiRegionEndpointOutputBuilder,
@@ -108,32 +118,20 @@
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
                     builder = builder.set_routes(super::protocol_serde::shape_routes::de_routes(tokens, _value, depth + 1)?);
                 }
@@ -144,6 +142,18 @@
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

### `src/protocol_serde/shape_get_reputation_entity.rs`

```diff
--- reference/src/protocol_serde/shape_get_reputation_entity.rs
+++ generated/src/protocol_serde/shape_get_reputation_entity.rs
@@ -88,6 +88,16 @@
     })
 }

+pub fn ser_get_reputation_entity_input(
+    input: &super::operation::get_reputation_entity::GetReputationEntityInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_reputation_entity_input::ser_get_reputation_entity_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_reputation_entity(
     _value: &[u8],
     mut builder: super::operation::get_reputation_entity::builders::GetReputationEntityOutputBuilder,
```

### `src/protocol_serde/shape_get_suppressed_destination.rs`

```diff
--- reference/src/protocol_serde/shape_get_suppressed_destination.rs
+++ generated/src/protocol_serde/shape_get_suppressed_destination.rs
@@ -92,6 +92,16 @@
     })
 }

+pub fn ser_get_suppressed_destination_input(
+    input: &super::operation::get_suppressed_destination::GetSuppressedDestinationInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_suppressed_destination_input::ser_get_suppressed_destination_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_suppressed_destination(
     _value: &[u8],
     mut builder: super::operation::get_suppressed_destination::builders::GetSuppressedDestinationOutputBuilder,
```

### `src/protocol_serde/shape_guardian_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_guardian_attributes.rs
+++ generated/src/protocol_serde/shape_guardian_attributes.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_guardian_attributes(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::GuardianAttributes,
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
-    input: &super::types::GuardianAttributes,
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
     input: &super::types::InboxPlacementTrackingOption,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.global {
+    {
         object.key("Global").boolean(input.global);
     }
     if let Some(var_1) = &input.tracked_isps {
```

### `src/protocol_serde/shape_list_configuration_sets.rs`

```diff
--- reference/src/protocol_serde/shape_list_configuration_sets.rs
+++ generated/src/protocol_serde/shape_list_configuration_sets.rs
@@ -73,6 +73,16 @@
     })
 }

+pub fn ser_list_configuration_sets_input(
+    input: &super::operation::list_configuration_sets::ListConfigurationSetsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_configuration_sets_input::ser_list_configuration_sets_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_configuration_sets(
     _value: &[u8],
     mut builder: super::operation::list_configuration_sets::builders::ListConfigurationSetsOutputBuilder,
```

### `src/protocol_serde/shape_list_contact_lists.rs`

```diff
--- reference/src/protocol_serde/shape_list_contact_lists.rs
+++ generated/src/protocol_serde/shape_list_contact_lists.rs
@@ -67,6 +67,16 @@
     })
 }

+pub fn ser_list_contact_lists_input(
+    input: &super::operation::list_contact_lists::ListContactListsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_contact_lists_input::ser_list_contact_lists_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_contact_lists(
     _value: &[u8],
     mut builder: super::operation::list_contact_lists::builders::ListContactListsOutputBuilder,
```

### `src/protocol_serde/shape_list_contacts_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_contacts_input.rs
+++ generated/src/protocol_serde/shape_list_contacts_input.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::list_contacts::ListContactsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.filter {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("Filter").start_object();
-        super::protocol_serde::shape_list_contacts_filter::ser_list_contacts_filter(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.contact_list_name {
+        object.key("ContactListName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.next_token {
-        object.key("NextToken").string(var_3.as_str());
+    if let Some(var_2) = &input.filter {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("Filter").start_object();
+        super::protocol_serde::shape_list_contacts_filter::ser_list_contacts_filter(&mut object_3, var_2)?;
+        object_3.finish();
     }
     if let Some(var_4) = &input.page_size {
         object.key("PageSize").number(
@@ -18,5 +18,8 @@
             ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
+    if let Some(var_5) = &input.next_token {
+        object.key("NextToken").string(var_5.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_list_custom_verification_email_templates.rs`

```diff
--- reference/src/protocol_serde/shape_list_custom_verification_email_templates.rs
+++ generated/src/protocol_serde/shape_list_custom_verification_email_templates.rs
@@ -86,6 +86,19 @@
     })
 }

+pub fn ser_list_custom_verification_email_templates_input(
+    input: &super::operation::list_custom_verification_email_templates::ListCustomVerificationEmailTemplatesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_custom_verification_email_templates_input::ser_list_custom_verification_email_templates_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_custom_verification_email_templates(
     _value: &[u8],
     mut builder: super::operation::list_custom_verification_email_templates::builders::ListCustomVerificationEmailTemplatesOutputBuilder,
```

### `src/protocol_serde/shape_list_dedicated_ip_pools.rs`

```diff
--- reference/src/protocol_serde/shape_list_dedicated_ip_pools.rs
+++ generated/src/protocol_serde/shape_list_dedicated_ip_pools.rs
@@ -73,6 +73,16 @@
     })
 }

+pub fn ser_list_dedicated_ip_pools_input(
+    input: &super::operation::list_dedicated_ip_pools::ListDedicatedIpPoolsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_dedicated_ip_pools_input::ser_list_dedicated_ip_pools_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_dedicated_ip_pools(
     _value: &[u8],
     mut builder: super::operation::list_dedicated_ip_pools::builders::ListDedicatedIpPoolsOutputBuilder,
```

### `src/protocol_serde/shape_list_deliverability_test_reports.rs`

```diff
--- reference/src/protocol_serde/shape_list_deliverability_test_reports.rs
+++ generated/src/protocol_serde/shape_list_deliverability_test_reports.rs
@@ -93,6 +93,16 @@
     })
 }

+pub fn ser_list_deliverability_test_reports_input(
+    input: &super::operation::list_deliverability_test_reports::ListDeliverabilityTestReportsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_deliverability_test_reports_input::ser_list_deliverability_test_reports_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_deliverability_test_reports(
     _value: &[u8],
     mut builder: super::operation::list_deliverability_test_reports::builders::ListDeliverabilityTestReportsOutputBuilder,
```

### `src/protocol_serde/shape_list_domain_deliverability_campaigns.rs`

```diff
--- reference/src/protocol_serde/shape_list_domain_deliverability_campaigns.rs
+++ generated/src/protocol_serde/shape_list_domain_deliverability_campaigns.rs
@@ -95,6 +95,19 @@
     })
 }

+pub fn ser_list_domain_deliverability_campaigns_input(
+    input: &super::operation::list_domain_deliverability_campaigns::ListDomainDeliverabilityCampaignsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_domain_deliverability_campaigns_input::ser_list_domain_deliverability_campaigns_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_domain_deliverability_campaigns(
     _value: &[u8],
     mut builder: super::operation::list_domain_deliverability_campaigns::builders::ListDomainDeliverabilityCampaignsOutputBuilder,
```

### `src/protocol_serde/shape_list_email_identities.rs`

```diff
--- reference/src/protocol_serde/shape_list_email_identities.rs
+++ generated/src/protocol_serde/shape_list_email_identities.rs
@@ -73,6 +73,16 @@
     })
 }

+pub fn ser_list_email_identities_input(
+    input: &super::operation::list_email_identities::ListEmailIdentitiesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_email_identities_input::ser_list_email_identities_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_email_identities(
     _value: &[u8],
     mut builder: super::operation::list_email_identities::builders::ListEmailIdentitiesOutputBuilder,
```

### `src/protocol_serde/shape_list_email_templates.rs`

```diff
--- reference/src/protocol_serde/shape_list_email_templates.rs
+++ generated/src/protocol_serde/shape_list_email_templates.rs
@@ -73,6 +73,16 @@
     })
 }

+pub fn ser_list_email_templates_input(
+    input: &super::operation::list_email_templates::ListEmailTemplatesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_email_templates_input::ser_list_email_templates_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_email_templates(
     _value: &[u8],
     mut builder: super::operation::list_email_templates::builders::ListEmailTemplatesOutputBuilder,
@@ -89,6 +99,11 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "TemplatesMetadata" => {
+                    builder = builder.set_templates_metadata(
+                        super::protocol_serde::shape_email_template_metadata_list::de_email_template_metadata_list(tokens, _value, depth + 1)?,
+                    );
+                }
                 "NextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -96,11 +111,6 @@
                             .transpose()?,
                     );
                 }
-                "TemplatesMetadata" => {
-                    builder = builder.set_templates_metadata(
-                        super::protocol_serde::shape_email_template_metadata_list::de_email_template_metadata_list(tokens, _value, depth + 1)?,
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
     input: &super::operation::list_export_jobs::ListExportJobsInput,
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

### `src/protocol_serde/shape_list_multi_region_endpoints.rs`

```diff
--- reference/src/protocol_serde/shape_list_multi_region_endpoints.rs
+++ generated/src/protocol_serde/shape_list_multi_region_endpoints.rs
@@ -77,6 +77,16 @@
     })
 }

+pub fn ser_list_multi_region_endpoints_input(
+    input: &super::operation::list_multi_region_endpoints::ListMultiRegionEndpointsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_multi_region_endpoints_input::ser_list_multi_region_endpoints_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_multi_region_endpoints(
     _value: &[u8],
     mut builder: super::operation::list_multi_region_endpoints::builders::ListMultiRegionEndpointsOutputBuilder,
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
+                    builder = builder.set_recommendations(super::protocol_serde::shape_recommendations_list::de_recommendations_list(
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
-                    builder = builder.set_recommendations(super::protocol_serde::shape_recommendations_list::de_recommendations_list(
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
+                            super::protocol_serde::shape_reputation_entities_list::de_reputation_entities_list(tokens, _value, depth + 1)?,
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
-                            super::protocol_serde::shape_reputation_entities_list::de_reputation_entities_list(tokens, _value, depth + 1)?,
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
+                        super::protocol_serde::shape_resource_tenant_metadata_list::de_resource_tenant_metadata_list(tokens, _value, depth + 1)?,
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
-                        super::protocol_serde::shape_resource_tenant_metadata_list::de_resource_tenant_metadata_list(tokens, _value, depth + 1)?,
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
     input: &super::operation::list_resource_tenants::ListResourceTenantsInput,
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
@@ -103,6 +103,16 @@
     })
 }

+pub fn ser_list_suppressed_destinations_input(
+    input: &super::operation::list_suppressed_destinations::ListSuppressedDestinationsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_suppressed_destinations_input::ser_list_suppressed_destinations_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_suppressed_destinations(
     _value: &[u8],
     mut builder: super::operation::list_suppressed_destinations::builders::ListSuppressedDestinationsOutputBuilder,
@@ -119,13 +129,6 @@
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
                         super::protocol_serde::shape_suppressed_destination_summaries::de_suppressed_destination_summaries(
@@ -135,6 +138,13 @@
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

### `src/protocol_serde/shape_list_tags_for_resource.rs`

```diff
--- reference/src/protocol_serde/shape_list_tags_for_resource.rs
+++ generated/src/protocol_serde/shape_list_tags_for_resource.rs
@@ -90,6 +90,16 @@
     })
 }

+pub fn ser_list_tags_for_resource_input(
+    input: &super::operation::list_tags_for_resource::ListTagsForResourceInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_tags_for_resource_input::ser_list_tags_for_resource_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_tags_for_resource(
     _value: &[u8],
     mut builder: super::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder,
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
+                    builder = builder.set_tenant_resources(super::protocol_serde::shape_tenant_resource_list::de_tenant_resource_list(
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
-                    builder = builder.set_tenant_resources(super::protocol_serde::shape_tenant_resource_list::de_tenant_resource_list(
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
     input: &super::operation::list_tenant_resources::ListTenantResourcesInput,
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
+                    builder = builder.set_tenants(super::protocol_serde::shape_tenant_info_list::de_tenant_info_list(
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
-                    builder = builder.set_tenants(super::protocol_serde::shape_tenant_info_list::de_tenant_info_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_mailbox_validation.rs`

```diff
--- reference/src/protocol_serde/shape_mailbox_validation.rs
+++ generated/src/protocol_serde/shape_mailbox_validation.rs
@@ -31,9 +31,7 @@
                             );
                         }
                         "Evaluations" => {
-                            builder = builder.set_evaluations(
-                                    super::protocol_serde::shape_email_address_insights_mailbox_evaluations::de_email_address_insights_mailbox_evaluations(tokens, _value, depth + 1)?
-                                );
+                            builder = builder.set_evaluations(super::protocol_serde::shape_email_address_insights_mailbox_evaluations::de_email_address_insights_mailbox_evaluations(tokens, _value, depth + 1)?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
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

### `src/protocol_serde/shape_put_account_dedicated_ip_warmup_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_dedicated_ip_warmup_attributes.rs
+++ generated/src/protocol_serde/shape_put_account_dedicated_ip_warmup_attributes.rs
@@ -94,3 +94,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_account_dedicated_ip_warmup_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_account_dedicated_ip_warmup_attributes::builders::PutAccountDedicatedIpWarmupAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_account_dedicated_ip_warmup_attributes::builders::PutAccountDedicatedIpWarmupAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_account_details.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_details.rs
+++ generated/src/protocol_serde/shape_put_account_details.rs
@@ -91,3 +91,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_account_details(
+    _value: &[u8],
+    mut builder: super::operation::put_account_details::builders::PutAccountDetailsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_account_details::builders::PutAccountDetailsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_account_details_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_details_input.rs
+++ generated/src/protocol_serde/shape_put_account_details_input.rs
@@ -3,29 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_account_details::PutAccountDetailsInput,
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

### `src/protocol_serde/shape_put_account_pricing_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_pricing_attributes.rs
+++ generated/src/protocol_serde/shape_put_account_pricing_attributes.rs
@@ -95,3 +95,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_account_pricing_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_account_pricing_attributes::builders::PutAccountPricingAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_account_pricing_attributes::builders::PutAccountPricingAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_account_sending_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_sending_attributes.rs
+++ generated/src/protocol_serde/shape_put_account_sending_attributes.rs
@@ -80,3 +80,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_account_sending_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_account_sending_attributes::builders::PutAccountSendingAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_account_sending_attributes::builders::PutAccountSendingAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_account_suppression_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_suppression_attributes.rs
+++ generated/src/protocol_serde/shape_put_account_suppression_attributes.rs
@@ -83,3 +83,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_account_suppression_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_account_suppression_attributes::builders::PutAccountSuppressionAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_account_suppression_attributes::builders::PutAccountSuppressionAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_account_vdm_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_account_vdm_attributes.rs
+++ generated/src/protocol_serde/shape_put_account_vdm_attributes.rs
@@ -84,3 +84,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_account_vdm_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_account_vdm_attributes::builders::PutAccountVdmAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_account_vdm_attributes::builders::PutAccountVdmAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_archiving_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_archiving_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_archiving_options.rs
@@ -108,3 +108,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_archiving_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_archiving_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_archiving_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_archiving_options_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_archiving_options::PutConfigurationSetArchivingOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.archive_arn {
-        object.key("ArchiveArn").string(var_1.as_str());
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.archive_arn {
+        object.key("ArchiveArn").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_delivery_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_delivery_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_delivery_options.rs
@@ -106,3 +106,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_delivery_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_delivery_options::builders::PutConfigurationSetDeliveryOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_delivery_options::builders::PutConfigurationSetDeliveryOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_delivery_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_delivery_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_delivery_options_input.rs
@@ -3,17 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_delivery_options::PutConfigurationSetDeliveryOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.max_delivery_seconds {
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.tls_policy {
+        object.key("TlsPolicy").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.sending_pool_name {
+        object.key("SendingPoolName").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.max_delivery_seconds {
         object.key("MaxDeliverySeconds").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
-    if let Some(var_2) = &input.sending_pool_name {
-        object.key("SendingPoolName").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.tls_policy {
-        object.key("TlsPolicy").string(var_3.as_str());
-    }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_reputation_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_reputation_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_reputation_options.rs
@@ -110,3 +110,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_reputation_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_reputation_options::builders::PutConfigurationSetReputationOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_reputation_options::builders::PutConfigurationSetReputationOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_reputation_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_reputation_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_reputation_options_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_reputation_options::PutConfigurationSetReputationOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.reputation_metrics_enabled {
-        object.key("ReputationMetricsEnabled").boolean(*var_1);
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.reputation_metrics_enabled {
+        object.key("ReputationMetricsEnabled").boolean(*var_2);
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_sending_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_sending_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_sending_options.rs
@@ -103,3 +103,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_sending_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_sending_options::builders::PutConfigurationSetSendingOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_sending_options::builders::PutConfigurationSetSendingOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_sending_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_sending_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_sending_options_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_sending_options::PutConfigurationSetSendingOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.sending_enabled {
-        object.key("SendingEnabled").boolean(*var_1);
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.sending_enabled {
+        object.key("SendingEnabled").boolean(*var_2);
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_suppression_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_suppression_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_suppression_options.rs
@@ -112,3 +112,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_suppression_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_suppression_options::builders::PutConfigurationSetSuppressionOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_suppression_options::builders::PutConfigurationSetSuppressionOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_suppression_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_suppression_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_suppression_options_input.rs
@@ -3,23 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.suppressed_reasons {
-        let mut array_2 = object.key("SuppressedReasons").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.suppression_scope {
+        object.key("SuppressionScope").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.suppressed_reasons {
+        let mut array_4 = object.key("SuppressedReasons").start_array();
+        for item_5 in var_3 {
             {
-                array_2.value().string(item_3.as_str());
+                array_4.value().string(item_5.as_str());
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_4) = &input.suppression_scope {
-        object.key("SuppressionScope").string(var_4.as_str());
+        array_4.finish();
     }
-    if let Some(var_5) = &input.validation_options {
+    if let Some(var_6) = &input.validation_options {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("ValidationOptions").start_object();
-        super::protocol_serde::shape_suppression_validation_options::ser_suppression_validation_options(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_7 = object.key("ValidationOptions").start_object();
+        super::protocol_serde::shape_suppression_validation_options::ser_suppression_validation_options(&mut object_7, var_6)?;
+        object_7.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_tracking_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_tracking_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_tracking_options.rs
@@ -106,3 +106,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_tracking_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_tracking_options::builders::PutConfigurationSetTrackingOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_tracking_options::builders::PutConfigurationSetTrackingOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_tracking_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_tracking_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_tracking_options_input.rs
@@ -3,11 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_tracking_options::PutConfigurationSetTrackingOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.custom_redirect_domain {
-        object.key("CustomRedirectDomain").string(var_1.as_str());
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.https_policy {
-        object.key("HttpsPolicy").string(var_2.as_str());
+    if let Some(var_2) = &input.custom_redirect_domain {
+        object.key("CustomRedirectDomain").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.https_policy {
+        object.key("HttpsPolicy").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_configuration_set_vdm_options.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_vdm_options.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_vdm_options.rs
@@ -98,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_configuration_set_vdm_options(
+    _value: &[u8],
+    mut builder: super::operation::put_configuration_set_vdm_options::builders::PutConfigurationSetVdmOptionsOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_configuration_set_vdm_options::builders::PutConfigurationSetVdmOptionsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_configuration_set_vdm_options_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_configuration_set_vdm_options_input.rs
+++ generated/src/protocol_serde/shape_put_configuration_set_vdm_options_input.rs
@@ -3,11 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_configuration_set_vdm_options::PutConfigurationSetVdmOptionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.vdm_options {
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.vdm_options {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("VdmOptions").start_object();
-        super::protocol_serde::shape_vdm_options::ser_vdm_options(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_3 = object.key("VdmOptions").start_object();
+        super::protocol_serde::shape_vdm_options::ser_vdm_options(&mut object_3, var_2)?;
+        object_3.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_dedicated_ip_in_pool.rs`

```diff
--- reference/src/protocol_serde/shape_put_dedicated_ip_in_pool.rs
+++ generated/src/protocol_serde/shape_put_dedicated_ip_in_pool.rs
@@ -95,3 +95,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_dedicated_ip_in_pool(
+    _value: &[u8],
+    mut builder: super::operation::put_dedicated_ip_in_pool::builders::PutDedicatedIpInPoolOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_dedicated_ip_in_pool::builders::PutDedicatedIpInPoolOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_dedicated_ip_in_pool_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_dedicated_ip_in_pool_input.rs
+++ generated/src/protocol_serde/shape_put_dedicated_ip_in_pool_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_dedicated_ip_in_pool::PutDedicatedIpInPoolInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.destination_pool_name {
-        object.key("DestinationPoolName").string(var_1.as_str());
+    if let Some(var_1) = &input.ip {
+        object.key("Ip").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.destination_pool_name {
+        object.key("DestinationPoolName").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_dedicated_ip_pool_scaling_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_dedicated_ip_pool_scaling_attributes.rs
+++ generated/src/protocol_serde/shape_put_dedicated_ip_pool_scaling_attributes.rs
@@ -128,3 +128,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_dedicated_ip_pool_scaling_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_dedicated_ip_pool_scaling_attributes::builders::PutDedicatedIpPoolScalingAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_dedicated_ip_pool_scaling_attributes::builders::PutDedicatedIpPoolScalingAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_dedicated_ip_pool_scaling_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_dedicated_ip_pool_scaling_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_dedicated_ip_pool_scaling_attributes_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_dedicated_ip_pool_scaling_attributes::PutDedicatedIpPoolScalingAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.scaling_mode {
-        object.key("ScalingMode").string(var_1.as_str());
+    if let Some(var_1) = &input.pool_name {
+        object.key("PoolName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.scaling_mode {
+        object.key("ScalingMode").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_dedicated_ip_warmup_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_dedicated_ip_warmup_attributes.rs
+++ generated/src/protocol_serde/shape_put_dedicated_ip_warmup_attributes.rs
@@ -98,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_dedicated_ip_warmup_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_dedicated_ip_warmup_attributes::builders::PutDedicatedIpWarmupAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_dedicated_ip_warmup_attributes::builders::PutDedicatedIpWarmupAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_dedicated_ip_warmup_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_dedicated_ip_warmup_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_dedicated_ip_warmup_attributes_input.rs
@@ -3,10 +3,13 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_dedicated_ip_warmup_attributes::PutDedicatedIpWarmupAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.warmup_percentage {
+    if let Some(var_1) = &input.ip {
+        object.key("Ip").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.warmup_percentage {
         object.key("WarmupPercentage").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
     Ok(())
```

### `src/protocol_serde/shape_put_deliverability_dashboard_option.rs`

```diff
--- reference/src/protocol_serde/shape_put_deliverability_dashboard_option.rs
+++ generated/src/protocol_serde/shape_put_deliverability_dashboard_option.rs
@@ -132,3 +132,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_deliverability_dashboard_option(
+    _value: &[u8],
+    mut builder: super::operation::put_deliverability_dashboard_option::builders::PutDeliverabilityDashboardOptionOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_deliverability_dashboard_option::builders::PutDeliverabilityDashboardOptionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_email_identity_configuration_set_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_configuration_set_attributes.rs
+++ generated/src/protocol_serde/shape_put_email_identity_configuration_set_attributes.rs
@@ -28,53 +28,47 @@
     Err(match error_code {
         "BadRequestException" => super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::BadRequestException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::types::error::builders::BadRequestExceptionBuilder::default();
-                    output = super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::types::error::builders::BadRequestExceptionBuilder::default();
+                output = super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
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
         "NotFoundException" => super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::NotFoundException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::types::error::builders::NotFoundExceptionBuilder::default();
-                    output = super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output).map_err(super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::types::error::builders::NotFoundExceptionBuilder::default();
+                output = super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output).map_err(super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
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
         "TooManyRequestsException" => super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::TooManyRequestsException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                    output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::unhandled)?;
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
-        _ => super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::generic(generic)
+        _ => super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesError::generic(generic),
     })
 }

@@ -104,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_email_identity_configuration_set_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_email_identity_configuration_set_attributes::builders::PutEmailIdentityConfigurationSetAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_email_identity_configuration_set_attributes::builders::PutEmailIdentityConfigurationSetAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_email_identity_configuration_set_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_configuration_set_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_configuration_set_attributes_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_email_identity_configuration_set_attributes::PutEmailIdentityConfigurationSetAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.configuration_set_name {
-        object.key("ConfigurationSetName").string(var_1.as_str());
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_email_identity_dkim_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_dkim_attributes.rs
+++ generated/src/protocol_serde/shape_put_email_identity_dkim_attributes.rs
@@ -98,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_email_identity_dkim_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_email_identity_dkim_attributes::builders::PutEmailIdentityDkimAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_email_identity_dkim_attributes::builders::PutEmailIdentityDkimAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_email_identity_dkim_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_dkim_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_dkim_attributes_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_email_identity_dkim_attributes::PutEmailIdentityDkimAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.signing_enabled {
-        object.key("SigningEnabled").boolean(*var_1);
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.signing_enabled {
+        object.key("SigningEnabled").boolean(*var_2);
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_email_identity_dkim_signing_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_dkim_signing_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_dkim_signing_attributes_input.rs
@@ -3,14 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_email_identity_dkim_signing_attributes::PutEmailIdentityDkimSigningAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.signing_attributes {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("SigningAttributes").start_object();
-        super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.signing_attributes_origin {
-        object.key("SigningAttributesOrigin").string(var_3.as_str());
+    if let Some(var_2) = &input.signing_attributes_origin {
+        object.key("SigningAttributesOrigin").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.signing_attributes {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("SigningAttributes").start_object();
+        super::protocol_serde::shape_dkim_signing_attributes::ser_dkim_signing_attributes(&mut object_4, var_3)?;
+        object_4.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_email_identity_feedback_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_feedback_attributes.rs
+++ generated/src/protocol_serde/shape_put_email_identity_feedback_attributes.rs
@@ -106,3 +106,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_email_identity_feedback_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_email_identity_feedback_attributes::builders::PutEmailIdentityFeedbackAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_email_identity_feedback_attributes::builders::PutEmailIdentityFeedbackAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_email_identity_feedback_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_feedback_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_feedback_attributes_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_email_identity_feedback_attributes::PutEmailIdentityFeedbackAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.email_forwarding_enabled {
-        object.key("EmailForwardingEnabled").boolean(*var_1);
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.email_forwarding_enabled {
+        object.key("EmailForwardingEnabled").boolean(*var_2);
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_email_identity_mail_from_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_mail_from_attributes.rs
+++ generated/src/protocol_serde/shape_put_email_identity_mail_from_attributes.rs
@@ -106,3 +106,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_email_identity_mail_from_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_email_identity_mail_from_attributes::builders::PutEmailIdentityMailFromAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_email_identity_mail_from_attributes::builders::PutEmailIdentityMailFromAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_email_identity_mail_from_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_email_identity_mail_from_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_email_identity_mail_from_attributes_input.rs
@@ -3,11 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_email_identity_mail_from_attributes::PutEmailIdentityMailFromAttributesInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.behavior_on_mx_failure {
-        object.key("BehaviorOnMxFailure").string(var_1.as_str());
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
     }
     if let Some(var_2) = &input.mail_from_domain {
         object.key("MailFromDomain").string(var_2.as_str());
     }
+    if let Some(var_3) = &input.behavior_on_mx_failure {
+        object.key("BehaviorOnMxFailure").string(var_3.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_suppressed_destination.rs`

```diff
--- reference/src/protocol_serde/shape_put_suppressed_destination.rs
+++ generated/src/protocol_serde/shape_put_suppressed_destination.rs
@@ -99,3 +99,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_suppressed_destination(
+    _value: &[u8],
+    mut builder: super::operation::put_suppressed_destination::builders::PutSuppressedDestinationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_suppressed_destination::builders::PutSuppressedDestinationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_tenant_suppression_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_put_tenant_suppression_attributes.rs
+++ generated/src/protocol_serde/shape_put_tenant_suppression_attributes.rs
@@ -98,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_tenant_suppression_attributes(
+    _value: &[u8],
+    mut builder: super::operation::put_tenant_suppression_attributes::builders::PutTenantSuppressionAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::put_tenant_suppression_attributes::builders::PutTenantSuppressionAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_put_tenant_suppression_attributes_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_tenant_suppression_attributes_input.rs
+++ generated/src/protocol_serde/shape_put_tenant_suppression_attributes_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_tenant_suppression_attributes::PutTenantSuppressionAttributesInput,
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
     input: &super::types::RawMessage,
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
     input: &super::types::ReputationOptions,
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
     input: &super::operation::send_bulk_email::SendBulkEmailInput,
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
-                super::protocol_serde::shape_bulk_email_entry::ser_bulk_email_entry(&mut object_4, item_3)?;
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
-        super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_6, var_5)?;
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
+                super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_11, item_10)?;
+                object_11.finish();
+            }
+        }
+        array_9.finish();
     }
-    if let Some(var_8) = &input.default_content {
+    if let Some(var_12) = &input.default_content {
         #[allow(unused_mut)]
-        let mut object_9 = object.key("DefaultContent").start_object();
-        super::protocol_serde::shape_bulk_email_content::ser_bulk_email_content(&mut object_9, var_8)?;
-        object_9.finish();
+        let mut object_13 = object.key("DefaultContent").start_object();
+        super::protocol_serde::shape_bulk_email_content::ser_bulk_email_content(&mut object_13, var_12)?;
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
-                super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_13, item_12)?;
-                object_13.finish();
+                let mut object_17 = array_15.value().start_object();
+                super::protocol_serde::shape_bulk_email_entry::ser_bulk_email_entry(&mut object_17, item_16)?;
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
+        super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_22, var_21)?;
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
     input: &super::operation::send_custom_verification_email::SendCustomVerificationEmailInput,
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
     input: &super::operation::send_email::SendEmailInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.configuration_overrides {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("ConfigurationOverrides").start_object();
-        super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_2, var_1)?;
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
-        super::protocol_serde::shape_email_content::ser_email_content(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_4 = object.key("Destination").start_object();
+        super::protocol_serde::shape_destination::ser_destination(&mut object_4, var_3)?;
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
-        super::protocol_serde::shape_destination::ser_destination(&mut object_7, var_6)?;
-        object_7.finish();
+        let mut object_11 = object.key("Content").start_object();
+        super::protocol_serde::shape_email_content::ser_email_content(&mut object_11, var_10)?;
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
-                super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_11, item_10)?;
-                object_11.finish();
+                let mut object_15 = array_13.value().start_object();
+                super::protocol_serde::shape_message_tag::ser_message_tag(&mut object_15, item_14)?;
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
-        super::protocol_serde::shape_list_management_options::ser_list_management_options(&mut object_18, var_17)?;
-        object_18.finish();
+        let mut object_20 = object.key("ListManagementOptions").start_object();
+        super::protocol_serde::shape_list_management_options::ser_list_management_options(&mut object_20, var_19)?;
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
+        super::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(&mut object_22, var_21)?;
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
     input: &super::types::SendingOptions,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.sending_enabled {
+    {
         object.key("SendingEnabled").boolean(input.sending_enabled);
     }
     Ok(())
```

### `src/protocol_serde/shape_tag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource.rs
+++ generated/src/protocol_serde/shape_tag_resource.rs
@@ -107,3 +107,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_tag_resource(
+    _value: &[u8],
+    mut builder: super::operation::tag_resource::builders::TagResourceOutputBuilder,
+) -> ::std::result::Result<super::operation::tag_resource::builders::TagResourceOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_tenant_suppression_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_tenant_suppression_attributes.rs
+++ generated/src/protocol_serde/shape_tenant_suppression_attributes.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_tenant_suppression_attributes(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::TenantSuppressionAttributes,
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
-    input: &super::types::TenantSuppressionAttributes,
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

### `src/protocol_serde/shape_test_render_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_test_render_email_template_input.rs
+++ generated/src/protocol_serde/shape_test_render_email_template_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::test_render_email_template::TestRenderEmailTemplateInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.template_data {
-        object.key("TemplateData").string(var_1.as_str());
+    if let Some(var_1) = &input.template_name {
+        object.key("TemplateName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.template_data {
+        object.key("TemplateData").string(var_2.as_str());
     }
     Ok(())
 }
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

### `src/protocol_serde/shape_untag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource.rs
+++ generated/src/protocol_serde/shape_untag_resource.rs
@@ -97,3 +97,46 @@
         output.build()
     })
 }
+
+pub fn ser_untag_resource_input(
+    input: &super::operation::untag_resource::UntagResourceInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_untag_resource_input::ser_untag_resource_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_untag_resource(
+    _value: &[u8],
+    mut builder: super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_configuration_set_event_destination.rs`

```diff
--- reference/src/protocol_serde/shape_update_configuration_set_event_destination.rs
+++ generated/src/protocol_serde/shape_update_configuration_set_event_destination.rs
@@ -112,3 +112,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_configuration_set_event_destination(
+    _value: &[u8],
+    mut builder: super::operation::update_configuration_set_event_destination::builders::UpdateConfigurationSetEventDestinationOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_configuration_set_event_destination::builders::UpdateConfigurationSetEventDestinationOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_configuration_set_event_destination_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_configuration_set_event_destination_input.rs
+++ generated/src/protocol_serde/shape_update_configuration_set_event_destination_input.rs
@@ -3,11 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.event_destination {
+    if let Some(var_1) = &input.configuration_set_name {
+        object.key("ConfigurationSetName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.event_destination_name {
+        object.key("EventDestinationName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.event_destination {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("EventDestination").start_object();
-        super::protocol_serde::shape_event_destination_definition::ser_event_destination_definition(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_4 = object.key("EventDestination").start_object();
+        super::protocol_serde::shape_event_destination_definition::ser_event_destination_definition(&mut object_4, var_3)?;
+        object_4.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_contact.rs`

```diff
--- reference/src/protocol_serde/shape_update_contact.rs
+++ generated/src/protocol_serde/shape_update_contact.rs
@@ -107,3 +107,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_contact(
+    _value: &[u8],
+    mut builder: super::operation::update_contact::builders::UpdateContactOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_contact::builders::UpdateContactOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_contact_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_contact_input.rs
+++ generated/src/protocol_serde/shape_update_contact_input.rs
@@ -3,23 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_contact::UpdateContactInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.attributes_data {
-        object.key("AttributesData").string(var_1.as_str());
+    if let Some(var_1) = &input.contact_list_name {
+        object.key("ContactListName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.email_address {
+        object.key("EmailAddress").string(var_2.as_str());
     }
-    if let Some(var_2) = &input.topic_preferences {
-        let mut array_3 = object.key("TopicPreferences").start_array();
-        for item_4 in var_2 {
+    if let Some(var_3) = &input.topic_preferences {
+        let mut array_4 = object.key("TopicPreferences").start_array();
+        for item_5 in var_3 {
             {
                 #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::protocol_serde::shape_topic_preference::ser_topic_preference(&mut object_5, item_4)?;
-                object_5.finish();
+                let mut object_6 = array_4.value().start_object();
+                super::protocol_serde::shape_topic_preference::ser_topic_preference(&mut object_6, item_5)?;
+                object_6.finish();
             }
         }
-        array_3.finish();
+        array_4.finish();
+    }
+    if let Some(var_7) = &input.unsubscribe_all {
+        object.key("UnsubscribeAll").boolean(*var_7);
     }
-    if let Some(var_6) = &input.unsubscribe_all {
-        object.key("UnsubscribeAll").boolean(*var_6);
+    if let Some(var_8) = &input.attributes_data {
+        object.key("AttributesData").string(var_8.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_contact_list.rs`

```diff
--- reference/src/protocol_serde/shape_update_contact_list.rs
+++ generated/src/protocol_serde/shape_update_contact_list.rs
@@ -109,3 +109,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_contact_list(
+    _value: &[u8],
+    mut builder: super::operation::update_contact_list::builders::UpdateContactListOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_contact_list::builders::UpdateContactListOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_contact_list_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_contact_list_input.rs
+++ generated/src/protocol_serde/shape_update_contact_list_input.rs
@@ -3,8 +3,8 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_contact_list::UpdateContactListInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.description {
-        object.key("Description").string(var_1.as_str());
+    if let Some(var_1) = &input.contact_list_name {
+        object.key("ContactListName").string(var_1.as_str());
     }
     if let Some(var_2) = &input.topics {
         let mut array_3 = object.key("Topics").start_array();
@@ -18,5 +18,8 @@
         }
         array_3.finish();
     }
+    if let Some(var_6) = &input.description {
+        object.key("Description").string(var_6.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_custom_verification_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_update_custom_verification_email_template.rs
+++ generated/src/protocol_serde/shape_update_custom_verification_email_template.rs
@@ -112,3 +112,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_custom_verification_email_template(
+    _value: &[u8],
+    mut builder: super::operation::update_custom_verification_email_template::builders::UpdateCustomVerificationEmailTemplateOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_custom_verification_email_template::builders::UpdateCustomVerificationEmailTemplateOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_custom_verification_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_custom_verification_email_template_input.rs
+++ generated/src/protocol_serde/shape_update_custom_verification_email_template_input.rs
@@ -3,20 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_custom_verification_email_template::UpdateCustomVerificationEmailTemplateInput,
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
     }
     if let Some(var_4) = &input.template_content {
         object.key("TemplateContent").string(var_4.as_str());
     }
-    if let Some(var_5) = &input.template_subject {
-        object.key("TemplateSubject").string(var_5.as_str());
+    if let Some(var_5) = &input.success_redirection_url {
+        object.key("SuccessRedirectionURL").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.failure_redirection_url {
+        object.key("FailureRedirectionURL").string(var_6.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_email_identity_policy.rs`

```diff
--- reference/src/protocol_serde/shape_update_email_identity_policy.rs
+++ generated/src/protocol_serde/shape_update_email_identity_policy.rs
@@ -99,3 +99,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_email_identity_policy(
+    _value: &[u8],
+    mut builder: super::operation::update_email_identity_policy::builders::UpdateEmailIdentityPolicyOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_email_identity_policy::builders::UpdateEmailIdentityPolicyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_email_identity_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_email_identity_policy_input.rs
+++ generated/src/protocol_serde/shape_update_email_identity_policy_input.rs
@@ -3,8 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_email_identity_policy::UpdateEmailIdentityPolicyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.policy {
-        object.key("Policy").string(var_1.as_str());
+    if let Some(var_1) = &input.email_identity {
+        object.key("EmailIdentity").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.policy_name {
+        object.key("PolicyName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.policy {
+        object.key("Policy").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_email_template.rs`

```diff
--- reference/src/protocol_serde/shape_update_email_template.rs
+++ generated/src/protocol_serde/shape_update_email_template.rs
@@ -95,3 +95,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_email_template(
+    _value: &[u8],
+    mut builder: super::operation::update_email_template::builders::UpdateEmailTemplateOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_email_template::builders::UpdateEmailTemplateOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_email_template_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_email_template_input.rs
+++ generated/src/protocol_serde/shape_update_email_template_input.rs
@@ -3,11 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_email_template::UpdateEmailTemplateInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.template_content {
+    if let Some(var_1) = &input.template_name {
+        object.key("TemplateName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.template_content {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("TemplateContent").start_object();
-        super::protocol_serde::shape_email_template_content::ser_email_template_content(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_3 = object.key("TemplateContent").start_object();
+        super::protocol_serde::shape_email_template_content::ser_email_template_content(&mut object_3, var_2)?;
+        object_3.finish();
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
         "BadRequestException" => super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::BadRequestException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::types::error::builders::BadRequestExceptionBuilder::default();
-                    output = super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::types::error::builders::BadRequestExceptionBuilder::default();
+                output = super::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
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
         "ConflictException" => super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::ConflictException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::types::error::builders::ConflictExceptionBuilder::default();
-                    output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::types::error::builders::ConflictExceptionBuilder::default();
+                output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
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
         "TooManyRequestsException" => super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::TooManyRequestsException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                    output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
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
-        _ => super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::generic(generic)
+        _ => super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::generic(generic),
     })
 }

@@ -104,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_reputation_entity_customer_managed_status(
+    _value: &[u8],
+    mut builder: super::operation::update_reputation_entity_customer_managed_status::builders::UpdateReputationEntityCustomerManagedStatusOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_reputation_entity_customer_managed_status::builders::UpdateReputationEntityCustomerManagedStatusOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_reputation_entity_customer_managed_status_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_reputation_entity_customer_managed_status_input.rs
+++ generated/src/protocol_serde/shape_update_reputation_entity_customer_managed_status_input.rs
@@ -3,8 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.sending_status {
-        object.key("SendingStatus").string(var_1.as_str());
+    if let Some(var_1) = &input.reputation_entity_type {
+        object.key("ReputationEntityType").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.reputation_entity_reference {
+        object.key("ReputationEntityReference").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.sending_status {
+        object.key("SendingStatus").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_reputation_entity_policy.rs`

```diff
--- reference/src/protocol_serde/shape_update_reputation_entity_policy.rs
+++ generated/src/protocol_serde/shape_update_reputation_entity_policy.rs
@@ -98,3 +98,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_reputation_entity_policy(
+    _value: &[u8],
+    mut builder: super::operation::update_reputation_entity_policy::builders::UpdateReputationEntityPolicyOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::update_reputation_entity_policy::builders::UpdateReputationEntityPolicyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_reputation_entity_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_reputation_entity_policy_input.rs
+++ generated/src/protocol_serde/shape_update_reputation_entity_policy_input.rs
@@ -3,8 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_reputation_entity_policy::UpdateReputationEntityPolicyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.reputation_entity_policy {
-        object.key("ReputationEntityPolicy").string(var_1.as_str());
+    if let Some(var_1) = &input.reputation_entity_type {
+        object.key("ReputationEntityType").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.reputation_entity_reference {
+        object.key("ReputationEntityReference").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.reputation_entity_policy {
+        object.key("ReputationEntityPolicy").string(var_3.as_str());
     }
     Ok(())
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
+    input: &super::types::VdmAttributes,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("VdmEnabled").string(input.vdm_enabled.as_str());
+    }
+    if let Some(var_1) = &input.dashboard_attributes {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("DashboardAttributes").start_object();
+        super::protocol_serde::shape_dashboard_attributes::ser_dashboard_attributes(&mut object_2, var_1)?;
+        object_2.finish();
+    }
+    if let Some(var_3) = &input.guardian_attributes {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("GuardianAttributes").start_object();
+        super::protocol_serde::shape_guardian_attributes::ser_guardian_attributes(&mut object_4, var_3)?;
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
-    input: &super::types::VdmAttributes,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("VdmEnabled").string(input.vdm_enabled.as_str());
-    }
-    if let Some(var_1) = &input.dashboard_attributes {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("DashboardAttributes").start_object();
-        super::protocol_serde::shape_dashboard_attributes::ser_dashboard_attributes(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.guardian_attributes {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("GuardianAttributes").start_object();
-        super::protocol_serde::shape_guardian_attributes::ser_guardian_attributes(&mut object_4, var_3)?;
-        object_4.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -263,6 +263,8 @@

 pub(crate) mod shape_batch_get_metric_data_input;

+pub(crate) mod shape_cancel_export_job_input;
+
 pub(crate) mod shape_concurrent_modification_exception;

 pub(crate) mod shape_conflict_exception;
@@ -297,12 +299,76 @@

 pub(crate) mod shape_create_tenant_resource_association_input;

+pub(crate) mod shape_delete_configuration_set_event_destination_input;
+
+pub(crate) mod shape_delete_configuration_set_input;
+
+pub(crate) mod shape_delete_contact_input;
+
+pub(crate) mod shape_delete_contact_list_input;
+
+pub(crate) mod shape_delete_custom_verification_email_template_input;
+
+pub(crate) mod shape_delete_dedicated_ip_pool_input;
+
+pub(crate) mod shape_delete_email_identity_input;
+
+pub(crate) mod shape_delete_email_identity_policy_input;
+
+pub(crate) mod shape_delete_email_template_input;
+
+pub(crate) mod shape_delete_multi_region_endpoint_input;
+
+pub(crate) mod shape_delete_suppressed_destination_input;
+
 pub(crate) mod shape_delete_tenant_input;

 pub(crate) mod shape_delete_tenant_resource_association_input;

+pub(crate) mod shape_get_blacklist_reports_input;
+
+pub(crate) mod shape_get_configuration_set_event_destinations_input;
+
+pub(crate) mod shape_get_configuration_set_input;
+
+pub(crate) mod shape_get_contact_input;
+
+pub(crate) mod shape_get_contact_list_input;
+
+pub(crate) mod shape_get_custom_verification_email_template_input;
+
+pub(crate) mod shape_get_dedicated_ip_input;
+
+pub(crate) mod shape_get_dedicated_ip_pool_input;
+
+pub(crate) mod shape_get_dedicated_ips_input;
+
+pub(crate) mod shape_get_deliverability_test_report_input;
+
+pub(crate) mod shape_get_domain_deliverability_campaign_input;
+
+pub(crate) mod shape_get_domain_statistics_report_input;
+
 pub(crate) mod shape_get_email_address_insights_input;

+pub(crate) mod shape_get_email_identity_input;
+
+pub(crate) mod shape_get_email_identity_policies_input;
+
+pub(crate) mod shape_get_email_template_input;
+
+pub(crate) mod shape_get_export_job_input;
+
+pub(crate) mod shape_get_import_job_input;
+
+pub(crate) mod shape_get_message_insights_input;
+
+pub(crate) mod shape_get_multi_region_endpoint_input;
+
+pub(crate) mod shape_get_reputation_entity_input;
+
+pub(crate) mod shape_get_suppressed_destination_input;
+
 pub(crate) mod shape_get_tenant_input;

 pub(crate) mod shape_internal_service_error_exception;
@@ -311,12 +377,30 @@

 pub(crate) mod shape_limit_exceeded_exception;

+pub(crate) mod shape_list_configuration_sets_input;
+
+pub(crate) mod shape_list_contact_lists_input;
+
 pub(crate) mod shape_list_contacts_input;

+pub(crate) mod shape_list_custom_verification_email_templates_input;
+
+pub(crate) mod shape_list_dedicated_ip_pools_input;
+
+pub(crate) mod shape_list_deliverability_test_reports_input;
+
+pub(crate) mod shape_list_domain_deliverability_campaigns_input;
+
+pub(crate) mod shape_list_email_identities_input;
+
+pub(crate) mod shape_list_email_templates_input;
+
 pub(crate) mod shape_list_export_jobs_input;

 pub(crate) mod shape_list_import_jobs_input;

+pub(crate) mod shape_list_multi_region_endpoints_input;
+
 pub(crate) mod shape_list_recommendations_input;

 pub(crate) mod shape_list_reputation_entities_input;
@@ -323,6 +407,10 @@

 pub(crate) mod shape_list_resource_tenants_input;

+pub(crate) mod shape_list_suppressed_destinations_input;
+
+pub(crate) mod shape_list_tags_for_resource_input;
+
 pub(crate) mod shape_list_tenant_resources_input;

 pub(crate) mod shape_list_tenants_input;
@@ -395,6 +483,8 @@

 pub(crate) mod shape_too_many_requests_exception;

+pub(crate) mod shape_untag_resource_input;
+
 pub(crate) mod shape_update_configuration_set_event_destination_input;

 pub(crate) mod shape_update_contact_input;
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
     pub data_format: super::types::DataFormat,
 }
 impl ImportDataSource {
-    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object>.
-    /// <p></p></object></i></p>
+    /// <p>An Amazon S3 URL in the format s3://<i><bucket_name></bucket_name></i>/<i><object></object></i>.</p>
     pub fn s3_url(&self) -> &str {
         use std::ops::Deref;
         self.s3_url.deref()
@@ -37,21 +35,18 @@
     pub(crate) data_format: ::std::option::Option<super::types::DataFormat>,
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
     pub fn builder() -> super::types::builders::MessageInsightsFiltersBuilder {
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

### Unexpected generated files

- `src/protocol_serde/shape_cancel_export_job_input.rs`
- `src/protocol_serde/shape_delete_configuration_set_event_destination_input.rs`
- `src/protocol_serde/shape_delete_configuration_set_input.rs`
- `src/protocol_serde/shape_delete_contact_input.rs`
- `src/protocol_serde/shape_delete_contact_list_input.rs`
- `src/protocol_serde/shape_delete_custom_verification_email_template_input.rs`
- `src/protocol_serde/shape_delete_dedicated_ip_pool_input.rs`
- `src/protocol_serde/shape_delete_email_identity_input.rs`
- `src/protocol_serde/shape_delete_email_identity_policy_input.rs`
- `src/protocol_serde/shape_delete_email_template_input.rs`
- `src/protocol_serde/shape_delete_multi_region_endpoint_input.rs`
- `src/protocol_serde/shape_delete_suppressed_destination_input.rs`
- `src/protocol_serde/shape_get_blacklist_reports_input.rs`
- `src/protocol_serde/shape_get_configuration_set_event_destinations_input.rs`
- `src/protocol_serde/shape_get_configuration_set_input.rs`
- `src/protocol_serde/shape_get_contact_input.rs`
- `src/protocol_serde/shape_get_contact_list_input.rs`
- `src/protocol_serde/shape_get_custom_verification_email_template_input.rs`
- `src/protocol_serde/shape_get_dedicated_ip_input.rs`
- `src/protocol_serde/shape_get_dedicated_ip_pool_input.rs`
- `src/protocol_serde/shape_get_dedicated_ips_input.rs`
- `src/protocol_serde/shape_get_deliverability_test_report_input.rs`
- `src/protocol_serde/shape_get_domain_deliverability_campaign_input.rs`
- `src/protocol_serde/shape_get_domain_statistics_report_input.rs`
- `src/protocol_serde/shape_get_email_identity_input.rs`
- `src/protocol_serde/shape_get_email_identity_policies_input.rs`
- `src/protocol_serde/shape_get_email_template_input.rs`
- `src/protocol_serde/shape_get_export_job_input.rs`
- `src/protocol_serde/shape_get_import_job_input.rs`
- `src/protocol_serde/shape_get_message_insights_input.rs`
- `src/protocol_serde/shape_get_multi_region_endpoint_input.rs`
- `src/protocol_serde/shape_get_reputation_entity_input.rs`
- `src/protocol_serde/shape_get_suppressed_destination_input.rs`
- `src/protocol_serde/shape_list_configuration_sets_input.rs`
- `src/protocol_serde/shape_list_contact_lists_input.rs`
- `src/protocol_serde/shape_list_custom_verification_email_templates_input.rs`
- `src/protocol_serde/shape_list_dedicated_ip_pools_input.rs`
- `src/protocol_serde/shape_list_deliverability_test_reports_input.rs`
- `src/protocol_serde/shape_list_domain_deliverability_campaigns_input.rs`
- `src/protocol_serde/shape_list_email_identities_input.rs`
- `src/protocol_serde/shape_list_email_templates_input.rs`
- `src/protocol_serde/shape_list_multi_region_endpoints_input.rs`
- `src/protocol_serde/shape_list_suppressed_destinations_input.rs`
- `src/protocol_serde/shape_list_tags_for_resource_input.rs`
- `src/protocol_serde/shape_untag_resource_input.rs`
