# AWS SDK Conformance Report: codeartifact

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## codeartifact
**Progress:** `490/490` files compared · `339` matched · `119` mismatches · `1` missing · `31` extra · `69.18%` match (100.00% means fully matched)

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -24,9 +24,9 @@
 //! CodeArtifact is a fully managed artifact repository compatible with language-native package managers and build tools such as npm, Apache Maven, pip, and dotnet. You can use CodeArtifact to share packages with development teams and pull packages. Packages can be pulled from both public and CodeArtifact repositories. You can also create an upstream relationship between a CodeArtifact repository and another repository, which effectively merges their contents from the point of view of a package manager client.
 //!
 //! __CodeArtifact concepts__
-//!   - __Repository__: A CodeArtifact repository contains a set of [package versions](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html#welcome-concepts-package-version), each of which maps to a set of assets, or files. Repositories are polyglot, so a single repository can contain packages of any supported type. Each repository exposes endpoints for fetching and publishing packages using tools such as the __ npm __ CLI or the Maven CLI (__ mvn __). For a list of supported package managers, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).
-//!   - __Domain__: Repositories are aggregated into a higher-level entity known as a _domain_. All package assets and metadata are stored in the domain, but are consumed through repositories. A given package asset, such as a Maven JAR file, is stored once per domain, no matter how many repositories it's present in. All of the assets and metadata in a domain are encrypted with the same customer master key (CMK) stored in Key Management Service (KMS). Each repository is a member of a single domain and can't be moved to a different domain. The domain allows organizational policy to be applied across multiple repositories, such as which accounts can access repositories in the domain, and which public repositories can be used as sources of packages. Although an organization can have multiple domains, we recommend a single production domain that contains all published artifacts so that teams can find and share packages across their organization.
-//!   - __Package__: A _package_ is a bundle of software and the metadata required to resolve dependencies and install the software. CodeArtifact supports npm, PyPI, Maven, NuGet, Swift, Ruby, Cargo, and generic package formats. For more information about the supported package formats and how to use CodeArtifact with them, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html). In CodeArtifact, a package consists of:
+//!   - __Repository__: A CodeArtifact repository contains a set of [package versions](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html#welcome-concepts-package-version), each of which maps to a set of assets, or files. Repositories are polyglot, so a single repository can contain packages of any supported type. Each repository exposes endpoints for fetching and publishing packages using tools such as the __npm__ CLI or the Maven CLI (__mvn__). For a list of supported package managers, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).
+//!   - __Domain__: Repositories are aggregated into a higher-level entity known as a _domain_. All package assets and metadata are stored in the domain, but are consumed through repositories. A given package asset, such as a Maven JAR file, is stored once per domain, no matter how many repositories it's present in. All of the assets and metadata in a domain are encrypted with the same customer master key (CMK) stored in Key Management Service (KMS).Each repository is a member of a single domain and can't be moved to a different domain.The domain allows organizational policy to be applied across multiple repositories, such as which accounts can access repositories in the domain, and which public repositories can be used as sources of packages.Although an organization can have multiple domains, we recommend a single production domain that contains all published artifacts so that teams can find and share packages across their organization.
+//!   - __Package__: A _package_ is a bundle of software and the metadata required to resolve dependencies and install the software. CodeArtifact supports npm, PyPI, Maven, NuGet, Swift, Ruby, Cargo, and generic package formats. For more information about the supported package formats and how to use CodeArtifact with them, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).In CodeArtifact, a package consists of:
 //!     - A _name_ (for example, webpack is the name of a popular npm package)
 //!     - An optional namespace (for example, @types in @types/node)
 //!     - A set of versions (for example, 1.0.0, 1.0.1, 1.0.2, etc.)
```

### `src/operation/associate_external_connection.rs`

```diff
--- reference/src/operation/associate_external_connection.rs
+++ generated/src/operation/associate_external_connection.rs
@@ -317,10 +317,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_associate_external_connection::ser_associate_external_connection_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_domain.rs`

```diff
--- reference/src/operation/delete_domain.rs
+++ generated/src/operation/delete_domain.rs
@@ -279,10 +279,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_domain::ser_delete_domain_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_domain_permissions_policy.rs`

```diff
--- reference/src/operation/delete_domain_permissions_policy.rs
+++ generated/src/operation/delete_domain_permissions_policy.rs
@@ -295,10 +295,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_domain_permissions_policy::ser_delete_domain_permissions_policy_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_package.rs`

```diff
--- reference/src/operation/delete_package.rs
+++ generated/src/operation/delete_package.rs
@@ -326,10 +326,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_package::ser_delete_package_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_package_group.rs`

```diff
--- reference/src/operation/delete_package_group.rs
+++ generated/src/operation/delete_package_group.rs
@@ -295,10 +295,15 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_package_group::ser_delete_package_group_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_repository.rs`

```diff
--- reference/src/operation/delete_repository.rs
+++ generated/src/operation/delete_repository.rs
@@ -295,10 +295,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_repository::ser_delete_repository_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_repository_permissions_policy.rs`

```diff
--- reference/src/operation/delete_repository_permissions_policy.rs
+++ generated/src/operation/delete_repository_permissions_policy.rs
@@ -315,10 +315,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_repository_permissions_policy::ser_delete_repository_permissions_policy_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/describe_domain.rs`

```diff
--- reference/src/operation/describe_domain.rs
+++ generated/src/operation/describe_domain.rs
@@ -279,10 +279,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_describe_domain::ser_describe_domain_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/describe_package.rs`

```diff
--- reference/src/operation/describe_package.rs
+++ generated/src/operation/describe_package.rs
@@ -326,10 +326,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_describe_package::ser_describe_package_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/describe_package_group.rs`

```diff
--- reference/src/operation/describe_package_group.rs
+++ generated/src/operation/describe_package_group.rs
@@ -295,10 +295,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_describe_package_group::ser_describe_package_group_input(
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

### `src/operation/describe_package_version.rs`

```diff
--- reference/src/operation/describe_package_version.rs
+++ generated/src/operation/describe_package_version.rs
@@ -348,10 +348,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_describe_package_version::ser_describe_package_version_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/describe_repository.rs`

```diff
--- reference/src/operation/describe_repository.rs
+++ generated/src/operation/describe_repository.rs
@@ -295,10 +295,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_describe_repository::ser_describe_repository_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/disassociate_external_connection.rs`

```diff
--- reference/src/operation/disassociate_external_connection.rs
+++ generated/src/operation/disassociate_external_connection.rs
@@ -317,10 +317,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_disassociate_external_connection::ser_disassociate_external_connection_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_associated_package_group.rs`

```diff
--- reference/src/operation/get_associated_package_group.rs
+++ generated/src/operation/get_associated_package_group.rs
@@ -316,10 +316,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_associated_package_group::ser_get_associated_package_group_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_authorization_token.rs`

```diff
--- reference/src/operation/get_authorization_token.rs
+++ generated/src/operation/get_authorization_token.rs
@@ -108,6 +108,7 @@
                 .expect("required fields set"),
         ));

+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "GetAuthorizationToken",
             "codeartifact",
@@ -290,10 +291,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_authorization_token::ser_get_authorization_token_input(
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

### `src/operation/get_domain_permissions_policy.rs`

```diff
--- reference/src/operation/get_domain_permissions_policy.rs
+++ generated/src/operation/get_domain_permissions_policy.rs
@@ -285,10 +285,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_domain_permissions_policy::ser_get_domain_permissions_policy_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_package_version_asset.rs`

```diff
--- reference/src/operation/get_package_version_asset.rs
+++ generated/src/operation/get_package_version_asset.rs
@@ -242,6 +242,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
@@ -388,10 +389,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_package_version_asset::ser_get_package_version_asset_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_package_version_readme.rs`

```diff
--- reference/src/operation/get_package_version_readme.rs
+++ generated/src/operation/get_package_version_readme.rs
@@ -348,10 +348,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_package_version_readme::ser_get_package_version_readme_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_repository_endpoint.rs`

```diff
--- reference/src/operation/get_repository_endpoint.rs
+++ generated/src/operation/get_repository_endpoint.rs
@@ -311,10 +311,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_repository_endpoint::ser_get_repository_endpoint_input(
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

### `src/operation/get_repository_permissions_policy.rs`

```diff
--- reference/src/operation/get_repository_permissions_policy.rs
+++ generated/src/operation/get_repository_permissions_policy.rs
@@ -301,10 +301,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_repository_permissions_policy::ser_get_repository_permissions_policy_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_allowed_repositories_for_group.rs`

```diff
--- reference/src/operation/list_allowed_repositories_for_group.rs
+++ generated/src/operation/list_allowed_repositories_for_group.rs
@@ -323,10 +323,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_allowed_repositories_for_group::ser_list_allowed_repositories_for_group_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_associated_packages.rs`

```diff
--- reference/src/operation/list_associated_packages.rs
+++ generated/src/operation/list_associated_packages.rs
@@ -321,10 +321,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_associated_packages::ser_list_associated_packages_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_package_groups.rs`

```diff
--- reference/src/operation/list_package_groups.rs
+++ generated/src/operation/list_package_groups.rs
@@ -304,10 +304,14 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_package_groups::ser_list_package_groups_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_package_version_assets.rs`

```diff
--- reference/src/operation/list_package_version_assets.rs
+++ generated/src/operation/list_package_version_assets.rs
@@ -363,10 +363,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_package_version_assets::ser_list_package_version_assets_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_package_version_dependencies.rs`

```diff
--- reference/src/operation/list_package_version_dependencies.rs
+++ generated/src/operation/list_package_version_dependencies.rs
@@ -358,10 +358,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_package_version_dependencies::ser_list_package_version_dependencies_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_package_versions.rs`

```diff
--- reference/src/operation/list_package_versions.rs
+++ generated/src/operation/list_package_versions.rs
@@ -356,10 +356,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_package_versions::ser_list_package_versions_input(
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

### `src/operation/list_packages.rs`

```diff
--- reference/src/operation/list_packages.rs
+++ generated/src/operation/list_packages.rs
@@ -345,10 +345,14 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_packages::ser_list_packages_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_repositories.rs`

```diff
--- reference/src/operation/list_repositories.rs
+++ generated/src/operation/list_repositories.rs
@@ -278,10 +278,14 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_repositories::ser_list_repositories_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_repositories_in_domain.rs`

```diff
--- reference/src/operation/list_repositories_in_domain.rs
+++ generated/src/operation/list_repositories_in_domain.rs
@@ -320,10 +320,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_repositories_in_domain::ser_list_repositories_in_domain_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_sub_package_groups.rs`

```diff
--- reference/src/operation/list_sub_package_groups.rs
+++ generated/src/operation/list_sub_package_groups.rs
@@ -310,10 +310,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_sub_package_groups::ser_list_sub_package_groups_input(
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

### `src/operation/list_tags_for_resource.rs`

```diff
--- reference/src/operation/list_tags_for_resource.rs
+++ generated/src/operation/list_tags_for_resource.rs
@@ -269,10 +269,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
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

### `src/operation/publish_package_version.rs`

```diff
--- reference/src/operation/publish_package_version.rs
+++ generated/src/operation/publish_package_version.rs
@@ -375,10 +375,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = super::protocol_serde::shape_publish_package_version_input::ser_asset_content_http_payload(input.asset_content)?.into_inner();
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_publish_package_version::ser_publish_package_version_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/primitives/event_stream.rs`

```diff
--- reference/src/primitives/event_stream.rs
+++ generated/src/primitives/event_stream.rs
@@ -1 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub use super::event_receiver::EventReceiver;
+pub use ::aws_smithy_http::event_stream::EventStreamSender;
+pub use ::aws_smithy_types::event_stream::Header;
+pub use ::aws_smithy_types::event_stream::HeaderValue;
+pub use ::aws_smithy_types::event_stream::Message;
+pub use ::aws_smithy_types::str_bytes::StrBytes;
```

### `src/protocol_serde/shape_access_denied_exception.rs`

```diff
--- reference/src/protocol_serde/shape_access_denied_exception.rs
+++ generated/src/protocol_serde/shape_access_denied_exception.rs
@@ -33,5 +33,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::serde_util::access_denied_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_asset_hashes.rs`

```diff
--- reference/src/protocol_serde/shape_asset_hashes.rs
+++ generated/src/protocol_serde/shape_asset_hashes.rs
@@ -23,7 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key.to_unescaped().map(|u| super::types::HashAlgorithm::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?;
```

### `src/protocol_serde/shape_associate_external_connection.rs`

```diff
--- reference/src/protocol_serde/shape_associate_external_connection.rs
+++ generated/src/protocol_serde/shape_associate_external_connection.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::associate_external_connection::AssociateExternalConnectionError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::associate_external_connection::AssociateExternalConnectionError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => {
@@ -72,10 +75,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -91,10 +95,11 @@
                     )
                     .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -105,18 +110,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::associate_external_connection::AssociateExternalConnectionError::ValidationException({
@@ -127,10 +126,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::associate_external_connection::AssociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::associate_external_connection::AssociateExternalConnectionError::generic(generic),
@@ -156,6 +156,16 @@
     })
 }

+pub fn ser_associate_external_connection_input(
+    input: &super::operation::associate_external_connection::AssociateExternalConnectionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_associate_external_connection_input::ser_associate_external_connection_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_associate_external_connection(
     _value: &[u8],
     mut builder: super::operation::associate_external_connection::builders::AssociateExternalConnectionOutputBuilder,
```

### `src/protocol_serde/shape_conflict_exception.rs`

```diff
--- reference/src/protocol_serde/shape_conflict_exception.rs
+++ generated/src/protocol_serde/shape_conflict_exception.rs
@@ -47,5 +47,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::serde_util::conflict_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_copy_package_versions.rs`

```diff
--- reference/src/protocol_serde/shape_copy_package_versions.rs
+++ generated/src/protocol_serde/shape_copy_package_versions.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::copy_package_versions::CopyPackageVersionsError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::copy_package_versions::CopyPackageVersionsError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::copy_package_versions::CopyPackageVersionsError::ResourceNotFoundException({
@@ -70,10 +73,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::copy_package_versions::CopyPackageVersionsError::ServiceQuotaExceededException({
@@ -87,10 +91,11 @@
                 )
                 .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::copy_package_versions::CopyPackageVersionsError::ThrottlingException({
@@ -100,18 +105,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::copy_package_versions::CopyPackageVersionsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::copy_package_versions::CopyPackageVersionsError::ValidationException({
@@ -122,10 +121,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::copy_package_versions::CopyPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::copy_package_versions::CopyPackageVersionsError::generic(generic),
@@ -177,13 +177,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "failedVersions" => {
-                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "successfulVersions" => {
                     builder = builder.set_successful_versions(
                         super::protocol_serde::shape_successful_package_version_info_map::de_successful_package_version_info_map(
@@ -193,6 +186,13 @@
                         )?,
                     );
                 }
+                "failedVersions" => {
+                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_copy_package_versions_input.rs`

```diff
--- reference/src/protocol_serde/shape_copy_package_versions_input.rs
+++ generated/src/protocol_serde/shape_copy_package_versions_input.rs
@@ -3,30 +3,51 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::copy_package_versions::CopyPackageVersionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.allow_overwrite {
-        object.key("allowOverwrite").boolean(*var_1);
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.source_repository {
+        object.key("sourceRepository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.destination_repository {
+        object.key("destinationRepository").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.format {
+        object.key("format").string(var_5.as_str());
     }
-    if let Some(var_2) = &input.include_from_upstream {
-        object.key("includeFromUpstream").boolean(*var_2);
+    if let Some(var_6) = &input.namespace {
+        object.key("namespace").string(var_6.as_str());
     }
-    if let Some(var_3) = &input.version_revisions {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("versionRevisions").start_object();
-        for (key_5, value_6) in var_3 {
+    if let Some(var_7) = &input.package {
+        object.key("package").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.versions {
+        let mut array_9 = object.key("versions").start_array();
+        for item_10 in var_8 {
             {
-                object_4.key(key_5.as_str()).string(value_6.as_str());
+                array_9.value().string(item_10.as_str());
             }
         }
-        object_4.finish();
+        array_9.finish();
     }
-    if let Some(var_7) = &input.versions {
-        let mut array_8 = object.key("versions").start_array();
-        for item_9 in var_7 {
+    if let Some(var_11) = &input.version_revisions {
+        #[allow(unused_mut)]
+        let mut object_12 = object.key("versionRevisions").start_object();
+        for (key_13, value_14) in var_11 {
             {
-                array_8.value().string(item_9.as_str());
+                object_12.key(key_13.as_str()).string(value_14.as_str());
             }
         }
-        array_8.finish();
+        object_12.finish();
+    }
+    if let Some(var_15) = &input.allow_overwrite {
+        object.key("allowOverwrite").boolean(*var_15);
+    }
+    if let Some(var_16) = &input.include_from_upstream {
+        object.key("includeFromUpstream").boolean(*var_16);
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_domain.rs`

```diff
--- reference/src/protocol_serde/shape_create_domain.rs
+++ generated/src/protocol_serde/shape_create_domain.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::create_domain::CreateDomainError::ConflictException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::create_domain::CreateDomainError::InternalServerException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::create_domain::CreateDomainError::ResourceNotFoundException({
@@ -67,10 +70,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::create_domain::CreateDomainError::ServiceQuotaExceededException({
@@ -84,10 +88,11 @@
                 )
                 .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::create_domain::CreateDomainError::ThrottlingException({
@@ -97,16 +102,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_domain::CreateDomainError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::create_domain::CreateDomainError::ValidationException({
@@ -117,10 +118,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_domain::CreateDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_domain::CreateDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::create_domain::CreateDomainError::generic(generic),
```

### `src/protocol_serde/shape_create_domain_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_domain_input.rs
+++ generated/src/protocol_serde/shape_create_domain_input.rs
@@ -3,20 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_domain::CreateDomainInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.encryption_key {
-        object.key("encryptionKey").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.tags {
-        let mut array_3 = object.key("tags").start_array();
-        for item_4 in var_2 {
+    if let Some(var_2) = &input.encryption_key {
+        object.key("encryptionKey").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.tags {
+        let mut array_4 = object.key("tags").start_array();
+        for item_5 in var_3 {
             {
                 #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
-                object_5.finish();
+                let mut object_6 = array_4.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
+                object_6.finish();
             }
         }
-        array_3.finish();
+        array_4.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_package_group.rs`

```diff
--- reference/src/protocol_serde/shape_create_package_group.rs
+++ generated/src/protocol_serde/shape_create_package_group.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::create_package_group::CreatePackageGroupError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::create_package_group::CreatePackageGroupError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::create_package_group::CreatePackageGroupError::ResourceNotFoundException({
@@ -70,10 +73,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::create_package_group::CreatePackageGroupError::ServiceQuotaExceededException({
@@ -87,10 +91,11 @@
                 )
                 .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::create_package_group::CreatePackageGroupError::ThrottlingException({
@@ -100,18 +105,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_package_group::CreatePackageGroupError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::create_package_group::CreatePackageGroupError::ValidationException({
@@ -122,10 +121,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_package_group::CreatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::create_package_group::CreatePackageGroupError::generic(generic),
```

### `src/protocol_serde/shape_create_package_group_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_package_group_input.rs
+++ generated/src/protocol_serde/shape_create_package_group_input.rs
@@ -3,26 +3,32 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_package_group::CreatePackageGroupInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.contact_info {
-        object.key("contactInfo").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.description {
-        object.key("description").string(var_2.as_str());
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
     }
     if let Some(var_3) = &input.package_group {
         object.key("packageGroup").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.tags {
-        let mut array_5 = object.key("tags").start_array();
-        for item_6 in var_4 {
+    if let Some(var_4) = &input.contact_info {
+        object.key("contactInfo").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.description {
+        object.key("description").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.tags {
+        let mut array_7 = object.key("tags").start_array();
+        for item_8 in var_6 {
             {
                 #[allow(unused_mut)]
-                let mut object_7 = array_5.value().start_object();
-                super::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
-                object_7.finish();
+                let mut object_9 = array_7.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
+                object_9.finish();
             }
         }
-        array_5.finish();
+        array_7.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_repository.rs`

```diff
--- reference/src/protocol_serde/shape_create_repository.rs
+++ generated/src/protocol_serde/shape_create_repository.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::create_repository::CreateRepositoryError::ConflictException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::create_repository::CreateRepositoryError::InternalServerException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::create_repository::CreateRepositoryError::ResourceNotFoundException({
@@ -67,10 +70,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::create_repository::CreateRepositoryError::ServiceQuotaExceededException({
@@ -84,10 +88,11 @@
                 )
                 .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::create_repository::CreateRepositoryError::ThrottlingException({
@@ -97,18 +102,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_repository::CreateRepositoryError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::create_repository::CreateRepositoryError::ValidationException({
@@ -119,10 +118,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::create_repository::CreateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::create_repository::CreateRepositoryError::generic(generic),
```

### `src/protocol_serde/shape_create_repository_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_repository_input.rs
+++ generated/src/protocol_serde/shape_create_repository_input.rs
@@ -3,32 +3,41 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_repository::CreateRepositoryInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.description {
-        object.key("description").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.tags {
-        let mut array_3 = object.key("tags").start_array();
-        for item_4 in var_2 {
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.description {
+        object.key("description").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.upstreams {
+        let mut array_6 = object.key("upstreams").start_array();
+        for item_7 in var_5 {
             {
                 #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
-                object_5.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::protocol_serde::shape_upstream_repository::ser_upstream_repository(&mut object_8, item_7)?;
+                object_8.finish();
             }
         }
-        array_3.finish();
+        array_6.finish();
     }
-    if let Some(var_6) = &input.upstreams {
-        let mut array_7 = object.key("upstreams").start_array();
-        for item_8 in var_6 {
+    if let Some(var_9) = &input.tags {
+        let mut array_10 = object.key("tags").start_array();
+        for item_11 in var_9 {
             {
                 #[allow(unused_mut)]
-                let mut object_9 = array_7.value().start_object();
-                super::protocol_serde::shape_upstream_repository::ser_upstream_repository(&mut object_9, item_8)?;
-                object_9.finish();
+                let mut object_12 = array_10.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_12, item_11)?;
+                object_12.finish();
             }
         }
-        array_7.finish();
+        array_10.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_delete_domain.rs`

```diff
--- reference/src/protocol_serde/shape_delete_domain.rs
+++ generated/src/protocol_serde/shape_delete_domain.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::delete_domain::DeleteDomainError::ConflictException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::delete_domain::DeleteDomainError::InternalServerException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::delete_domain::DeleteDomainError::ThrottlingException({
@@ -66,16 +69,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_domain::DeleteDomainError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::delete_domain::DeleteDomainError::ValidationException({
@@ -86,10 +85,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain::DeleteDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::delete_domain::DeleteDomainError::generic(generic),
@@ -112,6 +112,16 @@
     })
 }

+pub fn ser_delete_domain_input(
+    input: &super::operation::delete_domain::DeleteDomainInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_domain_input::ser_delete_domain_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_domain(
     _value: &[u8],
     mut builder: super::operation::delete_domain::builders::DeleteDomainOutputBuilder,
```

### `src/protocol_serde/shape_delete_domain_permissions_policy.rs`

```diff
--- reference/src/protocol_serde/shape_delete_domain_permissions_policy.rs
+++ generated/src/protocol_serde/shape_delete_domain_permissions_policy.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -57,10 +59,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -74,10 +77,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -88,18 +92,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::ValidationException({
@@ -110,10 +108,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyError::generic(generic),
@@ -139,6 +138,16 @@
     })
 }

+pub fn ser_delete_domain_permissions_policy_input(
+    input: &super::operation::delete_domain_permissions_policy::DeleteDomainPermissionsPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_domain_permissions_policy_input::ser_delete_domain_permissions_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_domain_permissions_policy(
     _value: &[u8],
     mut builder: super::operation::delete_domain_permissions_policy::builders::DeleteDomainPermissionsPolicyOutputBuilder,
```

### `src/protocol_serde/shape_delete_package.rs`

```diff
--- reference/src/protocol_serde/shape_delete_package.rs
+++ generated/src/protocol_serde/shape_delete_package.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package::DeletePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package::DeletePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::delete_package::DeletePackageError::ConflictException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package::DeletePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package::DeletePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::delete_package::DeletePackageError::InternalServerException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package::DeletePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package::DeletePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::delete_package::DeletePackageError::ResourceNotFoundException({
@@ -67,10 +70,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package::DeletePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package::DeletePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::delete_package::DeletePackageError::ThrottlingException({
@@ -80,16 +84,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package::DeletePackageError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_package::DeletePackageError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package::DeletePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::delete_package::DeletePackageError::ValidationException({
@@ -100,10 +100,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package::DeletePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package::DeletePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::delete_package::DeletePackageError::generic(generic),
@@ -126,6 +127,16 @@
     })
 }

+pub fn ser_delete_package_input(
+    input: &super::operation::delete_package::DeletePackageInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_package_input::ser_delete_package_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_package(
     _value: &[u8],
     mut builder: super::operation::delete_package::builders::DeletePackageOutputBuilder,
```

### `src/protocol_serde/shape_delete_package_group.rs`

```diff
--- reference/src/protocol_serde/shape_delete_package_group.rs
+++ generated/src/protocol_serde/shape_delete_package_group.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::delete_package_group::DeletePackageGroupError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::delete_package_group::DeletePackageGroupError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::delete_package_group::DeletePackageGroupError::ResourceNotFoundException({
@@ -70,10 +73,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::delete_package_group::DeletePackageGroupError::ServiceQuotaExceededException({
@@ -87,10 +91,11 @@
                 )
                 .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::delete_package_group::DeletePackageGroupError::ThrottlingException({
@@ -100,18 +105,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_package_group::DeletePackageGroupError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::delete_package_group::DeletePackageGroupError::ValidationException({
@@ -122,10 +121,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_group::DeletePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::delete_package_group::DeletePackageGroupError::generic(generic),
@@ -151,6 +151,16 @@
     })
 }

+pub fn ser_delete_package_group_input(
+    input: &super::operation::delete_package_group::DeletePackageGroupInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_package_group_input::ser_delete_package_group_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_package_group(
     _value: &[u8],
     mut builder: super::operation::delete_package_group::builders::DeletePackageGroupOutputBuilder,
```

### `src/protocol_serde/shape_delete_package_versions.rs`

```diff
--- reference/src/protocol_serde/shape_delete_package_versions.rs
+++ generated/src/protocol_serde/shape_delete_package_versions.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::delete_package_versions::DeletePackageVersionsError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::delete_package_versions::DeletePackageVersionsError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::delete_package_versions::DeletePackageVersionsError::ResourceNotFoundException({
@@ -70,10 +73,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::delete_package_versions::DeletePackageVersionsError::ThrottlingException({
@@ -83,18 +87,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_package_versions::DeletePackageVersionsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::delete_package_versions::DeletePackageVersionsError::ValidationException({
@@ -105,10 +103,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_package_versions::DeletePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::delete_package_versions::DeletePackageVersionsError::generic(generic),
@@ -160,13 +159,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "failedVersions" => {
-                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "successfulVersions" => {
                     builder = builder.set_successful_versions(
                         super::protocol_serde::shape_successful_package_version_info_map::de_successful_package_version_info_map(
@@ -176,6 +168,13 @@
                         )?,
                     );
                 }
+                "failedVersions" => {
+                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_delete_package_versions_input.rs`

```diff
--- reference/src/protocol_serde/shape_delete_package_versions_input.rs
+++ generated/src/protocol_serde/shape_delete_package_versions_input.rs
@@ -3,17 +3,35 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::delete_package_versions::DeletePackageVersionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.expected_status {
-        object.key("expectedStatus").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.format {
+        object.key("format").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.namespace {
+        object.key("namespace").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.package {
+        object.key("package").string(var_6.as_str());
     }
-    if let Some(var_2) = &input.versions {
-        let mut array_3 = object.key("versions").start_array();
-        for item_4 in var_2 {
+    if let Some(var_7) = &input.versions {
+        let mut array_8 = object.key("versions").start_array();
+        for item_9 in var_7 {
             {
-                array_3.value().string(item_4.as_str());
+                array_8.value().string(item_9.as_str());
             }
         }
-        array_3.finish();
+        array_8.finish();
+    }
+    if let Some(var_10) = &input.expected_status {
+        object.key("expectedStatus").string(var_10.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_delete_repository.rs`

```diff
--- reference/src/protocol_serde/shape_delete_repository.rs
+++ generated/src/protocol_serde/shape_delete_repository.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::delete_repository::DeleteRepositoryError::ConflictException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::delete_repository::DeleteRepositoryError::InternalServerException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::delete_repository::DeleteRepositoryError::ResourceNotFoundException({
@@ -67,10 +70,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::delete_repository::DeleteRepositoryError::ThrottlingException({
@@ -80,18 +84,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_repository::DeleteRepositoryError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::delete_repository::DeleteRepositoryError::ValidationException({
@@ -102,10 +100,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository::DeleteRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::delete_repository::DeleteRepositoryError::generic(generic),
@@ -128,6 +127,16 @@
     })
 }

+pub fn ser_delete_repository_input(
+    input: &super::operation::delete_repository::DeleteRepositoryInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_repository_input::ser_delete_repository_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_repository(
     _value: &[u8],
     mut builder: super::operation::delete_repository::builders::DeleteRepositoryOutputBuilder,
```

### `src/protocol_serde/shape_delete_repository_permissions_policy.rs`

```diff
--- reference/src/protocol_serde/shape_delete_repository_permissions_policy.rs
+++ generated/src/protocol_serde/shape_delete_repository_permissions_policy.rs
@@ -29,10 +29,11 @@
                     output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                         .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::access_denied_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -44,10 +45,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -59,10 +61,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -76,10 +79,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -91,18 +95,12 @@
                     let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                     output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                         .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
-                    super::serde_util::throttling_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -115,10 +113,11 @@
                     output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                         .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::validation_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -145,6 +144,19 @@
     })
 }

+pub fn ser_delete_repository_permissions_policy_input(
+    input: &super::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_repository_permissions_policy_input::ser_delete_repository_permissions_policy_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_repository_permissions_policy(
     _value: &[u8],
     mut builder: super::operation::delete_repository_permissions_policy::builders::DeleteRepositoryPermissionsPolicyOutputBuilder,
```

### `src/protocol_serde/shape_describe_domain.rs`

```diff
--- reference/src/protocol_serde/shape_describe_domain.rs
+++ generated/src/protocol_serde/shape_describe_domain.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::describe_domain::DescribeDomainError::InternalServerException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::describe_domain::DescribeDomainError::ResourceNotFoundException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::describe_domain::DescribeDomainError::ThrottlingException({
@@ -66,18 +69,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::describe_domain::DescribeDomainError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::describe_domain::DescribeDomainError::ValidationException({
@@ -88,10 +85,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_domain::DescribeDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::describe_domain::DescribeDomainError::generic(generic),
@@ -114,6 +112,16 @@
     })
 }

+pub fn ser_describe_domain_input(
+    input: &super::operation::describe_domain::DescribeDomainInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_describe_domain_input::ser_describe_domain_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_describe_domain(
     _value: &[u8],
     mut builder: super::operation::describe_domain::builders::DescribeDomainOutputBuilder,
```

### `src/protocol_serde/shape_describe_package.rs`

```diff
--- reference/src/protocol_serde/shape_describe_package.rs
+++ generated/src/protocol_serde/shape_describe_package.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package::DescribePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package::DescribePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::describe_package::DescribePackageError::InternalServerException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package::DescribePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package::DescribePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::describe_package::DescribePackageError::ResourceNotFoundException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package::DescribePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package::DescribePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::describe_package::DescribePackageError::ThrottlingException({
@@ -66,18 +69,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package::DescribePackageError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::describe_package::DescribePackageError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package::DescribePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::describe_package::DescribePackageError::ValidationException({
@@ -88,10 +85,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package::DescribePackageError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package::DescribePackageError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::describe_package::DescribePackageError::generic(generic),
@@ -114,6 +112,16 @@
     })
 }

+pub fn ser_describe_package_input(
+    input: &super::operation::describe_package::DescribePackageInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_describe_package_input::ser_describe_package_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_describe_package(
     _value: &[u8],
     mut builder: super::operation::describe_package::builders::DescribePackageOutputBuilder,
```

### `src/protocol_serde/shape_describe_package_group.rs`

```diff
--- reference/src/protocol_serde/shape_describe_package_group.rs
+++ generated/src/protocol_serde/shape_describe_package_group.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::describe_package_group::DescribePackageGroupError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::describe_package_group::DescribePackageGroupError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::describe_package_group::DescribePackageGroupError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::describe_package_group::DescribePackageGroupError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::describe_package_group::DescribePackageGroupError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_group::DescribePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::describe_package_group::DescribePackageGroupError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_describe_package_group_input(
+    input: &super::operation::describe_package_group::DescribePackageGroupInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_describe_package_group_input::ser_describe_package_group_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_describe_package_group(
     _value: &[u8],
     mut builder: super::operation::describe_package_group::builders::DescribePackageGroupOutputBuilder,
```

### `src/protocol_serde/shape_describe_package_version.rs`

```diff
--- reference/src/protocol_serde/shape_describe_package_version.rs
+++ generated/src/protocol_serde/shape_describe_package_version.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::describe_package_version::DescribePackageVersionError::ConflictException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::describe_package_version::DescribePackageVersionError::InternalServerException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::describe_package_version::DescribePackageVersionError::ResourceNotFoundException({
@@ -74,10 +77,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::describe_package_version::DescribePackageVersionError::ThrottlingException({
@@ -87,18 +91,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::describe_package_version::DescribePackageVersionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::describe_package_version::DescribePackageVersionError::ValidationException({
@@ -109,10 +107,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_package_version::DescribePackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::describe_package_version::DescribePackageVersionError::generic(generic),
@@ -138,6 +137,16 @@
     })
 }

+pub fn ser_describe_package_version_input(
+    input: &super::operation::describe_package_version::DescribePackageVersionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_describe_package_version_input::ser_describe_package_version_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_describe_package_version(
     _value: &[u8],
     mut builder: super::operation::describe_package_version::builders::DescribePackageVersionOutputBuilder,
```

### `src/protocol_serde/shape_describe_repository.rs`

```diff
--- reference/src/protocol_serde/shape_describe_repository.rs
+++ generated/src/protocol_serde/shape_describe_repository.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::describe_repository::DescribeRepositoryError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::describe_repository::DescribeRepositoryError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::describe_repository::DescribeRepositoryError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::describe_repository::DescribeRepositoryError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::describe_repository::DescribeRepositoryError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::describe_repository::DescribeRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::describe_repository::DescribeRepositoryError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_describe_repository_input(
+    input: &super::operation::describe_repository::DescribeRepositoryInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_describe_repository_input::ser_describe_repository_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_describe_repository(
     _value: &[u8],
     mut builder: super::operation::describe_repository::builders::DescribeRepositoryOutputBuilder,
```

### `src/protocol_serde/shape_disassociate_external_connection.rs`

```diff
--- reference/src/protocol_serde/shape_disassociate_external_connection.rs
+++ generated/src/protocol_serde/shape_disassociate_external_connection.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::disassociate_external_connection::DisassociateExternalConnectionError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -57,10 +59,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -74,10 +77,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -93,10 +97,11 @@
                     )
                     .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -107,18 +112,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::disassociate_external_connection::DisassociateExternalConnectionError::ValidationException({
@@ -129,10 +128,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::disassociate_external_connection::DisassociateExternalConnectionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::disassociate_external_connection::DisassociateExternalConnectionError::generic(generic),
@@ -158,6 +158,16 @@
     })
 }

+pub fn ser_disassociate_external_connection_input(
+    input: &super::operation::disassociate_external_connection::DisassociateExternalConnectionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_disassociate_external_connection_input::ser_disassociate_external_connection_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_disassociate_external_connection(
     _value: &[u8],
     mut builder: super::operation::disassociate_external_connection::builders::DisassociateExternalConnectionOutputBuilder,
```

### `src/protocol_serde/shape_dispose_package_versions.rs`

```diff
--- reference/src/protocol_serde/shape_dispose_package_versions.rs
+++ generated/src/protocol_serde/shape_dispose_package_versions.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::dispose_package_versions::DisposePackageVersionsError::ConflictException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::dispose_package_versions::DisposePackageVersionsError::InternalServerException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::dispose_package_versions::DisposePackageVersionsError::ResourceNotFoundException({
@@ -74,10 +77,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::dispose_package_versions::DisposePackageVersionsError::ThrottlingException({
@@ -87,18 +91,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::dispose_package_versions::DisposePackageVersionsError::ValidationException({
@@ -109,10 +107,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::dispose_package_versions::DisposePackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::dispose_package_versions::DisposePackageVersionsError::generic(generic),
@@ -164,13 +163,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "failedVersions" => {
-                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "successfulVersions" => {
                     builder = builder.set_successful_versions(
                         super::protocol_serde::shape_successful_package_version_info_map::de_successful_package_version_info_map(
@@ -180,6 +172,13 @@
                         )?,
                     );
                 }
+                "failedVersions" => {
+                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_dispose_package_versions_input.rs`

```diff
--- reference/src/protocol_serde/shape_dispose_package_versions_input.rs
+++ generated/src/protocol_serde/shape_dispose_package_versions_input.rs
@@ -3,27 +3,45 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::dispose_package_versions::DisposePackageVersionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.expected_status {
-        object.key("expectedStatus").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.format {
+        object.key("format").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.namespace {
+        object.key("namespace").string(var_5.as_str());
     }
-    if let Some(var_2) = &input.version_revisions {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("versionRevisions").start_object();
-        for (key_4, value_5) in var_2 {
+    if let Some(var_6) = &input.package {
+        object.key("package").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.versions {
+        let mut array_8 = object.key("versions").start_array();
+        for item_9 in var_7 {
             {
-                object_3.key(key_4.as_str()).string(value_5.as_str());
+                array_8.value().string(item_9.as_str());
             }
         }
-        object_3.finish();
+        array_8.finish();
     }
-    if let Some(var_6) = &input.versions {
-        let mut array_7 = object.key("versions").start_array();
-        for item_8 in var_6 {
+    if let Some(var_10) = &input.version_revisions {
+        #[allow(unused_mut)]
+        let mut object_11 = object.key("versionRevisions").start_object();
+        for (key_12, value_13) in var_10 {
             {
-                array_7.value().string(item_8.as_str());
+                object_11.key(key_12.as_str()).string(value_13.as_str());
             }
         }
-        array_7.finish();
+        object_11.finish();
+    }
+    if let Some(var_14) = &input.expected_status {
+        object.key("expectedStatus").string(var_14.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_get_associated_package_group.rs`

```diff
--- reference/src/protocol_serde/shape_get_associated_package_group.rs
+++ generated/src/protocol_serde/shape_get_associated_package_group.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::get_associated_package_group::GetAssociatedPackageGroupError::InternalServerException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::get_associated_package_group::GetAssociatedPackageGroupError::ResourceNotFoundException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_associated_package_group::GetAssociatedPackageGroupError::ValidationException({
@@ -74,10 +77,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_associated_package_group::GetAssociatedPackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_associated_package_group::GetAssociatedPackageGroupError::generic(generic),
@@ -103,6 +107,16 @@
     })
 }

+pub fn ser_get_associated_package_group_input(
+    input: &super::operation::get_associated_package_group::GetAssociatedPackageGroupInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_associated_package_group_input::ser_get_associated_package_group_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_associated_package_group(
     _value: &[u8],
     mut builder: super::operation::get_associated_package_group::builders::GetAssociatedPackageGroupOutputBuilder,
@@ -119,6 +133,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "packageGroup" => {
+                    builder = builder.set_package_group(super::protocol_serde::shape_package_group_description::de_package_group_description(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "associationType" => {
                     builder = builder.set_association_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -126,13 +147,6 @@
                             .transpose()?,
                     );
                 }
-                "packageGroup" => {
-                    builder = builder.set_package_group(super::protocol_serde::shape_package_group_description::de_package_group_description(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_authorization_token.rs`

```diff
--- reference/src/protocol_serde/shape_get_authorization_token.rs
+++ generated/src/protocol_serde/shape_get_authorization_token.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::get_authorization_token::GetAuthorizationTokenError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::get_authorization_token::GetAuthorizationTokenError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::get_authorization_token::GetAuthorizationTokenError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_authorization_token::GetAuthorizationTokenError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_authorization_token::GetAuthorizationTokenError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_authorization_token::GetAuthorizationTokenError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_get_authorization_token_input(
+    input: &super::operation::get_authorization_token::GetAuthorizationTokenInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_authorization_token_input::ser_get_authorization_token_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_authorization_token(
     _value: &[u8],
     mut builder: super::operation::get_authorization_token::builders::GetAuthorizationTokenOutputBuilder,
```

### `src/protocol_serde/shape_get_domain_permissions_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_domain_permissions_policy.rs
+++ generated/src/protocol_serde/shape_get_domain_permissions_policy.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_get_domain_permissions_policy_input(
+    input: &super::operation::get_domain_permissions_policy::GetDomainPermissionsPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_domain_permissions_policy_input::ser_get_domain_permissions_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_domain_permissions_policy(
     _value: &[u8],
     mut builder: super::operation::get_domain_permissions_policy::builders::GetDomainPermissionsPolicyOutputBuilder,
```

### `src/protocol_serde/shape_get_package_version_asset.rs`

```diff
--- reference/src/protocol_serde/shape_get_package_version_asset.rs
+++ generated/src/protocol_serde/shape_get_package_version_asset.rs
@@ -1,50 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(clippy::unnecessary_wraps)]
-pub fn de_get_package_version_asset_http_response(
-    response: &mut ::aws_smithy_runtime_api::http::Response,
-) -> std::result::Result<
-    super::operation::get_package_version_asset::GetPackageVersionAssetOutput,
-    super::operation::get_package_version_asset::GetPackageVersionAssetError,
-> {
-    let mut _response_body = ::aws_smithy_types::body::SdkBody::taken();
-    std::mem::swap(&mut _response_body, response.body_mut());
-    let _response_body = &mut _response_body;
-
-    let _response_status = response.status().as_u16();
-    let _response_headers = response.headers();
-    Ok({
-        #[allow(unused_mut)]
-        let mut output = super::operation::get_package_version_asset::builders::GetPackageVersionAssetOutputBuilder::default();
-        output = output.set_asset(Some(super::protocol_serde::shape_get_package_version_asset_output::de_asset_payload(
-            _response_body,
-        )?));
-        output = output.set_asset_name(
-            super::protocol_serde::shape_get_package_version_asset_output::de_asset_name_header(_response_headers).map_err(|_| {
-                super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled(
-                    "Failed to parse assetName from header `X-AssetName",
-                )
-            })?,
-        );
-        output = output.set_package_version(
-            super::protocol_serde::shape_get_package_version_asset_output::de_package_version_header(_response_headers).map_err(|_| {
-                super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled(
-                    "Failed to parse packageVersion from header `X-PackageVersion",
-                )
-            })?,
-        );
-        output = output.set_package_version_revision(
-            super::protocol_serde::shape_get_package_version_asset_output::de_package_version_revision_header(_response_headers).map_err(|_| {
-                super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled(
-                    "Failed to parse packageVersionRevision from header `X-PackageVersionRevision",
-                )
-            })?,
-        );
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        output.build()
-    })
-}
-
-#[allow(clippy::unnecessary_wraps)]
 pub fn de_get_package_version_asset_http_error(
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
@@ -77,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::get_package_version_asset::GetPackageVersionAssetError::ConflictException({
@@ -91,10 +47,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::get_package_version_asset::GetPackageVersionAssetError::InternalServerException({
@@ -105,10 +62,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::get_package_version_asset::GetPackageVersionAssetError::ResourceNotFoundException({
@@ -119,10 +77,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::get_package_version_asset::GetPackageVersionAssetError::ThrottlingException({
@@ -132,18 +91,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_package_version_asset::GetPackageVersionAssetError::ValidationException({
@@ -154,12 +107,99 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_package_version_asset::GetPackageVersionAssetError::generic(generic),
     })
 }
+
+#[allow(clippy::unnecessary_wraps)]
+pub fn de_get_package_version_asset_http_response(
+    _response_status: u16,
+    _response_headers: &::aws_smithy_runtime_api::http::Headers,
+    _response_body: &[u8],
+) -> std::result::Result<
+    super::operation::get_package_version_asset::GetPackageVersionAssetOutput,
+    super::operation::get_package_version_asset::GetPackageVersionAssetError,
+> {
+    Ok({
+        #[allow(unused_mut)]
+        let mut output = super::operation::get_package_version_asset::builders::GetPackageVersionAssetOutputBuilder::default();
+        output = super::protocol_serde::shape_get_package_version_asset::de_get_package_version_asset(_response_body, output)
+            .map_err(super::operation::get_package_version_asset::GetPackageVersionAssetError::unhandled)?;
+        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        output.build()
+    })
+}
+
+pub fn ser_get_package_version_asset_input(
+    input: &super::operation::get_package_version_asset::GetPackageVersionAssetInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_package_version_asset_input::ser_get_package_version_asset_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_get_package_version_asset(
+    _value: &[u8],
+    mut builder: super::operation::get_package_version_asset::builders::GetPackageVersionAssetOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::get_package_version_asset::builders::GetPackageVersionAssetOutputBuilder,
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
+                "asset" => {
+                    builder = builder.set_asset(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
+                }
+                "assetName" => {
+                    builder = builder.set_asset_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "packageVersion" => {
+                    builder = builder.set_package_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "packageVersionRevision" => {
+                    builder = builder.set_package_version_revision(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
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

### `src/protocol_serde/shape_get_package_version_readme.rs`

```diff
--- reference/src/protocol_serde/shape_get_package_version_readme.rs
+++ generated/src/protocol_serde/shape_get_package_version_readme.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::get_package_version_readme::GetPackageVersionReadmeError::InternalServerException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::get_package_version_readme::GetPackageVersionReadmeError::ResourceNotFoundException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::get_package_version_readme::GetPackageVersionReadmeError::ThrottlingException({
@@ -73,18 +76,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_package_version_readme::GetPackageVersionReadmeError::ValidationException({
@@ -95,10 +92,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_package_version_readme::GetPackageVersionReadmeError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_package_version_readme::GetPackageVersionReadmeError::generic(generic),
@@ -124,6 +122,16 @@
     })
 }

+pub fn ser_get_package_version_readme_input(
+    input: &super::operation::get_package_version_readme::GetPackageVersionReadmeInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_package_version_readme_input::ser_get_package_version_readme_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_package_version_readme(
     _value: &[u8],
     mut builder: super::operation::get_package_version_readme::builders::GetPackageVersionReadmeOutputBuilder,
@@ -161,22 +169,22 @@
                             .transpose()?,
                     );
                 }
-                "readme" => {
-                    builder = builder.set_readme(
+                "version" => {
+                    builder = builder.set_version(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "version" => {
-                    builder = builder.set_version(
+                "versionRevision" => {
+                    builder = builder.set_version_revision(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "versionRevision" => {
-                    builder = builder.set_version_revision(
+                "readme" => {
+                    builder = builder.set_readme(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_get_repository_endpoint.rs`

```diff
--- reference/src/protocol_serde/shape_get_repository_endpoint.rs
+++ generated/src/protocol_serde/shape_get_repository_endpoint.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::get_repository_endpoint::GetRepositoryEndpointError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::get_repository_endpoint::GetRepositoryEndpointError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::get_repository_endpoint::GetRepositoryEndpointError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_repository_endpoint::GetRepositoryEndpointError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_endpoint::GetRepositoryEndpointError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_repository_endpoint::GetRepositoryEndpointError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_get_repository_endpoint_input(
+    input: &super::operation::get_repository_endpoint::GetRepositoryEndpointInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_repository_endpoint_input::ser_get_repository_endpoint_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_repository_endpoint(
     _value: &[u8],
     mut builder: super::operation::get_repository_endpoint::builders::GetRepositoryEndpointOutputBuilder,
```

### `src/protocol_serde/shape_get_repository_permissions_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_repository_permissions_policy.rs
+++ generated/src/protocol_serde/shape_get_repository_permissions_policy.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -43,10 +44,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -60,10 +62,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -74,18 +77,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::ValidationException({
@@ -96,10 +93,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyError::generic(generic),
@@ -125,6 +123,16 @@
     })
 }

+pub fn ser_get_repository_permissions_policy_input(
+    input: &super::operation::get_repository_permissions_policy::GetRepositoryPermissionsPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_repository_permissions_policy_input::ser_get_repository_permissions_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_repository_permissions_policy(
     _value: &[u8],
     mut builder: super::operation::get_repository_permissions_policy::builders::GetRepositoryPermissionsPolicyOutputBuilder,
```

### `src/protocol_serde/shape_internal_server_exception.rs`

```diff
--- reference/src/protocol_serde/shape_internal_server_exception.rs
+++ generated/src/protocol_serde/shape_internal_server_exception.rs
@@ -33,5 +33,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::serde_util::internal_server_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_list_allowed_repositories_for_group.rs`

```diff
--- reference/src/protocol_serde/shape_list_allowed_repositories_for_group.rs
+++ generated/src/protocol_serde/shape_list_allowed_repositories_for_group.rs
@@ -29,10 +29,11 @@
                     output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                         .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::access_denied_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -45,10 +46,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -62,10 +64,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -81,10 +84,11 @@
                     )
                     .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -95,18 +99,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::ValidationException({
@@ -117,10 +115,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupError::generic(generic),
@@ -146,6 +145,16 @@
     })
 }

+pub fn ser_list_allowed_repositories_for_group_input(
+    input: &super::operation::list_allowed_repositories_for_group::ListAllowedRepositoriesForGroupInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_allowed_repositories_for_group_input::ser_list_allowed_repositories_for_group_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_allowed_repositories_for_group(
     _value: &[u8],
     mut builder: super::operation::list_allowed_repositories_for_group::builders::ListAllowedRepositoriesForGroupOutputBuilder,
```

### `src/protocol_serde/shape_list_associated_packages.rs`

```diff
--- reference/src/protocol_serde/shape_list_associated_packages.rs
+++ generated/src/protocol_serde/shape_list_associated_packages.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_associated_packages::ListAssociatedPackagesError::InternalServerException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_associated_packages::ListAssociatedPackagesError::ResourceNotFoundException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_associated_packages::ListAssociatedPackagesError::ValidationException({
@@ -74,10 +77,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_associated_packages::ListAssociatedPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_associated_packages::ListAssociatedPackagesError::generic(generic),
@@ -103,6 +107,16 @@
     })
 }

+pub fn ser_list_associated_packages_input(
+    input: &super::operation::list_associated_packages::ListAssociatedPackagesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_associated_packages_input::ser_list_associated_packages_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_associated_packages(
     _value: &[u8],
     mut builder: super::operation::list_associated_packages::builders::ListAssociatedPackagesOutputBuilder,
@@ -119,6 +133,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "packages" => {
+                    builder = builder.set_packages(super::protocol_serde::shape_associated_package_list::de_associated_package_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -126,13 +147,6 @@
                             .transpose()?,
                     );
                 }
-                "packages" => {
-                    builder = builder.set_packages(super::protocol_serde::shape_associated_package_list::de_associated_package_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_domains.rs`

```diff
--- reference/src/protocol_serde/shape_list_domains.rs
+++ generated/src/protocol_serde/shape_list_domains.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_domains::ListDomainsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_domains::ListDomainsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_domains::ListDomainsError::InternalServerException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_domains::ListDomainsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_domains::ListDomainsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_domains::ListDomainsError::ThrottlingException({
@@ -52,16 +54,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_domains::ListDomainsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_domains::ListDomainsError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_domains::ListDomainsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_domains::ListDomainsError::ValidationException({
@@ -72,10 +70,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_domains::ListDomainsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_domains::ListDomainsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_domains::ListDomainsError::generic(generic),
```

### `src/protocol_serde/shape_list_package_groups.rs`

```diff
--- reference/src/protocol_serde/shape_list_package_groups.rs
+++ generated/src/protocol_serde/shape_list_package_groups.rs
@@ -26,10 +26,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_package_groups::ListPackageGroupsError::InternalServerException({
@@ -40,10 +41,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_package_groups::ListPackageGroupsError::ResourceNotFoundException({
@@ -54,10 +56,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_package_groups::ListPackageGroupsError::ThrottlingException({
@@ -67,18 +70,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_package_groups::ListPackageGroupsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_package_groups::ListPackageGroupsError::ValidationException({
@@ -89,10 +86,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_groups::ListPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_package_groups::ListPackageGroupsError::generic(generic),
@@ -116,6 +114,16 @@
     })
 }

+pub fn ser_list_package_groups_input(
+    input: &super::operation::list_package_groups::ListPackageGroupsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_package_groups_input::ser_list_package_groups_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_package_groups(
     _value: &[u8],
     mut builder: super::operation::list_package_groups::builders::ListPackageGroupsOutputBuilder,
@@ -132,6 +140,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "packageGroups" => {
+                    builder = builder.set_package_groups(super::protocol_serde::shape_package_group_summary_list::de_package_group_summary_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -139,13 +154,6 @@
                             .transpose()?,
                     );
                 }
-                "packageGroups" => {
-                    builder = builder.set_package_groups(super::protocol_serde::shape_package_group_summary_list::de_package_group_summary_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_package_version_assets.rs`

```diff
--- reference/src/protocol_serde/shape_list_package_version_assets.rs
+++ generated/src/protocol_serde/shape_list_package_version_assets.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_package_version_assets::ListPackageVersionAssetsError::InternalServerException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_package_version_assets::ListPackageVersionAssetsError::ResourceNotFoundException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_package_version_assets::ListPackageVersionAssetsError::ThrottlingException({
@@ -73,18 +76,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_package_version_assets::ListPackageVersionAssetsError::ValidationException({
@@ -95,10 +92,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_assets::ListPackageVersionAssetsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_package_version_assets::ListPackageVersionAssetsError::generic(generic),
@@ -124,6 +122,16 @@
     })
 }

+pub fn ser_list_package_version_assets_input(
+    input: &super::operation::list_package_version_assets::ListPackageVersionAssetsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_package_version_assets_input::ser_list_package_version_assets_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_package_version_assets(
     _value: &[u8],
     mut builder: super::operation::list_package_version_assets::builders::ListPackageVersionAssetsOutputBuilder,
@@ -140,13 +148,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "assets" => {
-                    builder = builder.set_assets(super::protocol_serde::shape_asset_summary_list::de_asset_summary_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "format" => {
                     builder = builder.set_format(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -161,13 +162,6 @@
                             .transpose()?,
                     );
                 }
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "package" => {
                     builder = builder.set_package(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -189,6 +183,20 @@
                             .transpose()?,
                     );
                 }
+                "nextToken" => {
+                    builder = builder.set_next_token(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "assets" => {
+                    builder = builder.set_assets(super::protocol_serde::shape_asset_summary_list::de_asset_summary_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_package_version_dependencies.rs`

```diff
--- reference/src/protocol_serde/shape_list_package_version_dependencies.rs
+++ generated/src/protocol_serde/shape_list_package_version_dependencies.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -43,10 +44,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -60,10 +62,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -74,18 +77,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::ValidationException({
@@ -96,10 +93,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_package_version_dependencies::ListPackageVersionDependenciesError::generic(generic),
@@ -125,6 +123,16 @@
     })
 }

+pub fn ser_list_package_version_dependencies_input(
+    input: &super::operation::list_package_version_dependencies::ListPackageVersionDependenciesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_package_version_dependencies_input::ser_list_package_version_dependencies_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_package_version_dependencies(
     _value: &[u8],
     mut builder: super::operation::list_package_version_dependencies::builders::ListPackageVersionDependenciesOutputBuilder,
@@ -141,13 +149,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "dependencies" => {
-                    builder = builder.set_dependencies(super::protocol_serde::shape_package_dependency_list::de_package_dependency_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "format" => {
                     builder = builder.set_format(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -162,13 +163,6 @@
                             .transpose()?,
                     );
                 }
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "package" => {
                     builder = builder.set_package(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -190,6 +184,20 @@
                             .transpose()?,
                     );
                 }
+                "nextToken" => {
+                    builder = builder.set_next_token(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "dependencies" => {
+                    builder = builder.set_dependencies(super::protocol_serde::shape_package_dependency_list::de_package_dependency_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_package_versions.rs`

```diff
--- reference/src/protocol_serde/shape_list_package_versions.rs
+++ generated/src/protocol_serde/shape_list_package_versions.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_package_versions::ListPackageVersionsError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_package_versions::ListPackageVersionsError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_package_versions::ListPackageVersionsError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_package_versions::ListPackageVersionsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_package_versions::ListPackageVersionsError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_package_versions::ListPackageVersionsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_package_versions::ListPackageVersionsError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_list_package_versions_input(
+    input: &super::operation::list_package_versions::ListPackageVersionsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_package_versions_input::ser_list_package_versions_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_package_versions(
     _value: &[u8],
     mut builder: super::operation::list_package_versions::builders::ListPackageVersionsOutputBuilder,
@@ -157,13 +165,6 @@
                             .transpose()?,
                     );
                 }
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "package" => {
                     builder = builder.set_package(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -176,6 +177,13 @@
                         super::protocol_serde::shape_package_version_summary_list::de_package_version_summary_list(tokens, _value, depth + 1)?,
                     );
                 }
+                "nextToken" => {
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

### `src/protocol_serde/shape_list_packages.rs`

```diff
--- reference/src/protocol_serde/shape_list_packages.rs
+++ generated/src/protocol_serde/shape_list_packages.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_packages::ListPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_packages::ListPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_packages::ListPackagesError::InternalServerException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_packages::ListPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_packages::ListPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_packages::ListPackagesError::ResourceNotFoundException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_packages::ListPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_packages::ListPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_packages::ListPackagesError::ThrottlingException({
@@ -66,16 +69,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_packages::ListPackagesError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_packages::ListPackagesError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_packages::ListPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_packages::ListPackagesError::ValidationException({
@@ -86,10 +85,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_packages::ListPackagesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_packages::ListPackagesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_packages::ListPackagesError::generic(generic),
@@ -112,6 +112,16 @@
     })
 }

+pub fn ser_list_packages_input(
+    input: &super::operation::list_packages::ListPackagesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_packages_input::ser_list_packages_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_packages(
     _value: &[u8],
     mut builder: super::operation::list_packages::builders::ListPackagesOutputBuilder,
@@ -128,6 +138,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "packages" => {
+                    builder = builder.set_packages(super::protocol_serde::shape_package_summary_list::de_package_summary_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -135,13 +152,6 @@
                             .transpose()?,
                     );
                 }
-                "packages" => {
-                    builder = builder.set_packages(super::protocol_serde::shape_package_summary_list::de_package_summary_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_repositories.rs`

```diff
--- reference/src/protocol_serde/shape_list_repositories.rs
+++ generated/src/protocol_serde/shape_list_repositories.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_repositories::ListRepositoriesError::InternalServerException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_repositories::ListRepositoriesError::ThrottlingException({
@@ -52,18 +54,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_repositories::ListRepositoriesError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_repositories::ListRepositoriesError::ValidationException({
@@ -74,10 +70,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories::ListRepositoriesError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_repositories::ListRepositoriesError::generic(generic),
@@ -100,6 +97,16 @@
     })
 }

+pub fn ser_list_repositories_input(
+    input: &super::operation::list_repositories::ListRepositoriesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_repositories_input::ser_list_repositories_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_repositories(
     _value: &[u8],
     mut builder: super::operation::list_repositories::builders::ListRepositoriesOutputBuilder,
@@ -116,6 +123,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "repositories" => {
+                    builder = builder.set_repositories(super::protocol_serde::shape_repository_summary_list::de_repository_summary_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -123,13 +137,6 @@
                             .transpose()?,
                     );
                 }
-                "repositories" => {
-                    builder = builder.set_repositories(super::protocol_serde::shape_repository_summary_list::de_repository_summary_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_repositories_in_domain.rs`

```diff
--- reference/src/protocol_serde/shape_list_repositories_in_domain.rs
+++ generated/src/protocol_serde/shape_list_repositories_in_domain.rs
@@ -32,10 +32,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::InternalServerException({
@@ -46,10 +47,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::ResourceNotFoundException({
@@ -60,10 +62,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::ThrottlingException({
@@ -73,18 +76,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::ValidationException({
@@ -95,10 +92,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_repositories_in_domain::ListRepositoriesInDomainError::generic(generic),
@@ -124,6 +122,16 @@
     })
 }

+pub fn ser_list_repositories_in_domain_input(
+    input: &super::operation::list_repositories_in_domain::ListRepositoriesInDomainInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_repositories_in_domain_input::ser_list_repositories_in_domain_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_repositories_in_domain(
     _value: &[u8],
     mut builder: super::operation::list_repositories_in_domain::builders::ListRepositoriesInDomainOutputBuilder,
@@ -140,6 +148,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "repositories" => {
+                    builder = builder.set_repositories(super::protocol_serde::shape_repository_summary_list::de_repository_summary_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -147,13 +162,6 @@
                             .transpose()?,
                     );
                 }
-                "repositories" => {
-                    builder = builder.set_repositories(super::protocol_serde::shape_repository_summary_list::de_repository_summary_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_sub_package_groups.rs`

```diff
--- reference/src/protocol_serde/shape_list_sub_package_groups.rs
+++ generated/src/protocol_serde/shape_list_sub_package_groups.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::list_sub_package_groups::ListSubPackageGroupsError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_sub_package_groups::ListSubPackageGroupsError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_sub_package_groups::ListSubPackageGroupsError::ThrottlingException({
@@ -69,18 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_sub_package_groups::ListSubPackageGroupsError::ValidationException({
@@ -91,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_sub_package_groups::ListSubPackageGroupsError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_sub_package_groups::ListSubPackageGroupsError::generic(generic),
@@ -120,6 +118,16 @@
     })
 }

+pub fn ser_list_sub_package_groups_input(
+    input: &super::operation::list_sub_package_groups::ListSubPackageGroupsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_sub_package_groups_input::ser_list_sub_package_groups_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_sub_package_groups(
     _value: &[u8],
     mut builder: super::operation::list_sub_package_groups::builders::ListSubPackageGroupsOutputBuilder,
@@ -136,6 +144,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "packageGroups" => {
+                    builder = builder.set_package_groups(super::protocol_serde::shape_package_group_summary_list::de_package_group_summary_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -143,13 +158,6 @@
                             .transpose()?,
                     );
                 }
-                "packageGroups" => {
-                    builder = builder.set_package_groups(super::protocol_serde::shape_package_group_summary_list::de_package_group_summary_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_tags_for_resource.rs`

```diff
--- reference/src/protocol_serde/shape_list_tags_for_resource.rs
+++ generated/src/protocol_serde/shape_list_tags_for_resource.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::list_tags_for_resource::ListTagsForResourceError::ThrottlingException({
@@ -55,18 +57,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException({
@@ -77,10 +73,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::list_tags_for_resource::ListTagsForResourceError::generic(generic),
@@ -106,6 +103,16 @@
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

### `src/protocol_serde/shape_package_group_allowed_repository_update.rs`

```diff
--- reference/src/protocol_serde/shape_package_group_allowed_repository_update.rs
+++ generated/src/protocol_serde/shape_package_group_allowed_repository_update.rs
@@ -23,9 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key
-                            .to_unescaped()
-                            .map(|u| super::types::PackageGroupAllowedRepositoryUpdateType::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = super::protocol_serde::shape_repository_name_list::de_repository_name_list(tokens, _value, depth + 1)?;
                         match value {
                             Some(value) => {
```

### `src/protocol_serde/shape_package_group_allowed_repository_updates.rs`

```diff
--- reference/src/protocol_serde/shape_package_group_allowed_repository_updates.rs
+++ generated/src/protocol_serde/shape_package_group_allowed_repository_updates.rs
@@ -28,9 +28,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key
-                            .to_unescaped()
-                            .map(|u| super::types::PackageGroupOriginRestrictionType::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = super::protocol_serde::shape_package_group_allowed_repository_update::de_package_group_allowed_repository_update(
                             tokens,
                             _value,
```

### `src/protocol_serde/shape_package_group_origin_restrictions.rs`

```diff
--- reference/src/protocol_serde/shape_package_group_origin_restrictions.rs
+++ generated/src/protocol_serde/shape_package_group_origin_restrictions.rs
@@ -23,9 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key
-                            .to_unescaped()
-                            .map(|u| super::types::PackageGroupOriginRestrictionType::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = super::protocol_serde::shape_package_group_origin_restriction::de_package_group_origin_restriction(
                             tokens,
                             _value,
```

### `src/protocol_serde/shape_publish_package_version.rs`

```diff
--- reference/src/protocol_serde/shape_publish_package_version.rs
+++ generated/src/protocol_serde/shape_publish_package_version.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::publish_package_version::PublishPackageVersionError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::publish_package_version::PublishPackageVersionError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::publish_package_version::PublishPackageVersionError::ResourceNotFoundException({
@@ -70,10 +73,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::publish_package_version::PublishPackageVersionError::ServiceQuotaExceededException({
@@ -87,10 +91,11 @@
                 )
                 .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::publish_package_version::PublishPackageVersionError::ThrottlingException({
@@ -100,18 +105,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::publish_package_version::PublishPackageVersionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::publish_package_version::PublishPackageVersionError::ValidationException({
@@ -122,10 +121,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::publish_package_version::PublishPackageVersionError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::publish_package_version::PublishPackageVersionError::generic(generic),
@@ -151,22 +151,14 @@
     })
 }

-pub fn ser_publish_package_version_headers(
+pub fn ser_publish_package_version_input(
     input: &super::operation::publish_package_version::PublishPackageVersionInput,
-    mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
-    if let ::std::option::Option::Some(inner_1) = &input.asset_sha256 {
-        let formatted_2 = inner_1.as_str();
-        let header_value = formatted_2;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "asset_sha256",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("x-amz-content-sha256", header_value);
-    }
-    Ok(builder)
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_publish_package_version_input::ser_publish_package_version_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
 }

 pub(crate) fn de_publish_package_version(
@@ -185,9 +177,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "asset" => {
-                    builder = builder.set_asset(super::protocol_serde::shape_asset_summary::de_asset_summary(tokens, _value, depth + 1)?);
-                }
                 "format" => {
                     builder = builder.set_format(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -209,13 +198,6 @@
                             .transpose()?,
                     );
                 }
-                "status" => {
-                    builder = builder.set_status(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::PackageVersionStatus::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 "version" => {
                     builder = builder.set_version(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -230,6 +212,16 @@
                             .transpose()?,
                     );
                 }
+                "status" => {
+                    builder = builder.set_status(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::PackageVersionStatus::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
+                "asset" => {
+                    builder = builder.set_asset(super::protocol_serde::shape_asset_summary::de_asset_summary(tokens, _value, depth + 1)?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_publish_package_version_input.rs`

```diff
--- reference/src/protocol_serde/shape_publish_package_version_input.rs
+++ generated/src/protocol_serde/shape_publish_package_version_input.rs
@@ -1,6 +1,40 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_asset_content_http_payload(
-    payload: ::aws_smithy_types::byte_stream::ByteStream,
-) -> ::std::result::Result<::aws_smithy_types::byte_stream::ByteStream, ::aws_smithy_types::error::operation::BuildError> {
-    Ok(payload)
+pub fn ser_publish_package_version_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::operation::publish_package_version::PublishPackageVersionInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.format {
+        object.key("format").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.namespace {
+        object.key("namespace").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.package {
+        object.key("package").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.package_version {
+        object.key("packageVersion").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.asset_content {
+        object.key("assetContent").string_unchecked(&::aws_smithy_types::base64::encode(var_8));
+    }
+    if let Some(var_9) = &input.asset_name {
+        object.key("assetName").string(var_9.as_str());
+    }
+    if let Some(var_10) = &input.asset_sha256 {
+        object.key("assetSHA256").string(var_10.as_str());
+    }
+    if let Some(var_11) = &input.unfinished {
+        object.key("unfinished").boolean(*var_11);
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_put_domain_permissions_policy.rs`

```diff
--- reference/src/protocol_serde/shape_put_domain_permissions_policy.rs
+++ generated/src/protocol_serde/shape_put_domain_permissions_policy.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::ResourceNotFoundException({
@@ -70,10 +73,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => {
@@ -88,10 +92,11 @@
                     )
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -102,18 +107,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::ValidationException({
@@ -124,10 +123,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::put_domain_permissions_policy::PutDomainPermissionsPolicyError::generic(generic),
```

### `src/protocol_serde/shape_put_domain_permissions_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_domain_permissions_policy_input.rs
+++ generated/src/protocol_serde/shape_put_domain_permissions_policy_input.rs
@@ -9,11 +9,11 @@
     if let Some(var_2) = &input.domain_owner {
         object.key("domainOwner").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.policy_document {
-        object.key("policyDocument").string(var_3.as_str());
+    if let Some(var_3) = &input.policy_revision {
+        object.key("policyRevision").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.policy_revision {
-        object.key("policyRevision").string(var_4.as_str());
+    if let Some(var_4) = &input.policy_document {
+        object.key("policyDocument").string(var_4.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_package_origin_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_put_package_origin_configuration.rs
+++ generated/src/protocol_serde/shape_put_package_origin_configuration.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -43,10 +44,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -60,10 +62,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -74,18 +77,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::ValidationException({
@@ -96,10 +93,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::put_package_origin_configuration::PutPackageOriginConfigurationError::generic(generic),
```

### `src/protocol_serde/shape_put_package_origin_configuration_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_package_origin_configuration_input.rs
+++ generated/src/protocol_serde/shape_put_package_origin_configuration_input.rs
@@ -3,11 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_package_origin_configuration::PutPackageOriginConfigurationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.restrictions {
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.format {
+        object.key("format").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.namespace {
+        object.key("namespace").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.package {
+        object.key("package").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.restrictions {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("restrictions").start_object();
-        super::protocol_serde::shape_package_origin_restrictions::ser_package_origin_restrictions(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_8 = object.key("restrictions").start_object();
+        super::protocol_serde::shape_package_origin_restrictions::ser_package_origin_restrictions(&mut object_8, var_7)?;
+        object_8.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_repository_permissions_policy.rs`

```diff
--- reference/src/protocol_serde/shape_put_repository_permissions_policy.rs
+++ generated/src/protocol_serde/shape_put_repository_permissions_policy.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => {
@@ -57,10 +59,11 @@
                     output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                         .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -74,10 +77,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -93,10 +97,11 @@
                     )
                     .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -107,18 +112,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::ValidationException({
@@ -129,10 +128,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyError::generic(generic),
```

### `src/protocol_serde/shape_put_repository_permissions_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_repository_permissions_policy_input.rs
+++ generated/src/protocol_serde/shape_put_repository_permissions_policy_input.rs
@@ -3,11 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_repository_permissions_policy::PutRepositoryPermissionsPolicyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.policy_document {
-        object.key("policyDocument").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.policy_revision {
-        object.key("policyRevision").string(var_2.as_str());
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.policy_revision {
+        object.key("policyRevision").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.policy_document {
+        object.key("policyDocument").string(var_5.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_repository_description.rs`

```diff
--- reference/src/protocol_serde/shape_repository_description.rs
+++ generated/src/protocol_serde/shape_repository_description.rs
@@ -74,9 +74,7 @@
                                 );
                             }
                             "externalConnections" => {
-                                builder = builder.set_external_connections(
-                                    super::protocol_serde::shape_repository_external_connection_info_list::de_repository_external_connection_info_list(tokens, _value, depth + 1)?
-                                );
+                                builder = builder.set_external_connections(super::protocol_serde::shape_repository_external_connection_info_list::de_repository_external_connection_info_list(tokens, _value, depth + 1)?);
                             }
                             "createdTime" => {
                                 builder = builder.set_created_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
```

### `src/protocol_serde/shape_resource_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_resource_not_found_exception.rs
+++ generated/src/protocol_serde/shape_resource_not_found_exception.rs
@@ -47,5 +47,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::serde_util::resource_not_found_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_service_quota_exceeded_exception.rs`

```diff
--- reference/src/protocol_serde/shape_service_quota_exceeded_exception.rs
+++ generated/src/protocol_serde/shape_service_quota_exceeded_exception.rs
@@ -48,5 +48,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::serde_util::service_quota_exceeded_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_tag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource.rs
+++ generated/src/protocol_serde/shape_tag_resource.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::tag_resource::TagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::tag_resource::TagResourceError::ResourceNotFoundException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::tag_resource::TagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::tag_resource::TagResourceError::ServiceQuotaExceededException({
@@ -56,10 +58,11 @@
                 )
                 .map_err(super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::tag_resource::TagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::tag_resource::TagResourceError::ThrottlingException({
@@ -69,16 +72,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::tag_resource::TagResourceError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::tag_resource::TagResourceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::tag_resource::TagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::tag_resource::TagResourceError::ValidationException({
@@ -89,10 +88,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::tag_resource::TagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::tag_resource::TagResourceError::generic(generic),
@@ -122,3 +122,34 @@
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

### `src/protocol_serde/shape_tag_resource_input.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource_input.rs
+++ generated/src/protocol_serde/shape_tag_resource_input.rs
@@ -3,17 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::tag_resource::TagResourceInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.tags {
-        let mut array_2 = object.key("tags").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.resource_arn {
+        object.key("resourceArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.tags {
+        let mut array_3 = object.key("tags").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::protocol_serde::shape_tag::ser_tag(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_2.finish();
+        array_3.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_throttling_exception.rs`

```diff
--- reference/src/protocol_serde/shape_throttling_exception.rs
+++ generated/src/protocol_serde/shape_throttling_exception.rs
@@ -40,21 +40,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
-}
-
-pub(crate) fn de_retry_after_seconds_header(
-    header_map: &::aws_smithy_runtime_api::http::Headers,
-) -> ::std::result::Result<::std::option::Option<i32>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("Retry-After");
-    let var_1 = ::aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
-    if var_1.len() > 1 {
-        Err(::aws_smithy_http::header::ParseError::new(format!(
-            "expected one item but found {}",
-            var_1.len()
-        )))
-    } else {
-        let mut var_1 = var_1;
-        Ok(var_1.pop())
-    }
+    Ok(super::serde_util::throttling_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_untag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource.rs
+++ generated/src/protocol_serde/shape_untag_resource.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::untag_resource::UntagResourceError::ResourceNotFoundException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::untag_resource::UntagResourceError::ThrottlingException({
@@ -52,16 +54,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::untag_resource::UntagResourceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::untag_resource::UntagResourceError::ValidationException({
@@ -72,10 +70,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::untag_resource::UntagResourceError::generic(generic),
@@ -105,3 +104,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
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

### `src/protocol_serde/shape_untag_resource_input.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource_input.rs
+++ generated/src/protocol_serde/shape_untag_resource_input.rs
@@ -3,14 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::untag_resource::UntagResourceInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.tag_keys {
-        let mut array_2 = object.key("tagKeys").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.resource_arn {
+        object.key("resourceArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.tag_keys {
+        let mut array_3 = object.key("tagKeys").start_array();
+        for item_4 in var_2 {
             {
-                array_2.value().string(item_3.as_str());
+                array_3.value().string(item_4.as_str());
             }
         }
-        array_2.finish();
+        array_3.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_package_group.rs`

```diff
--- reference/src/protocol_serde/shape_update_package_group.rs
+++ generated/src/protocol_serde/shape_update_package_group.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::update_package_group::UpdatePackageGroupError::InternalServerException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::update_package_group::UpdatePackageGroupError::ResourceNotFoundException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::update_package_group::UpdatePackageGroupError::ServiceQuotaExceededException({
@@ -73,10 +76,11 @@
                 )
                 .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::update_package_group::UpdatePackageGroupError::ThrottlingException({
@@ -86,18 +90,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_package_group::UpdatePackageGroupError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::update_package_group::UpdatePackageGroupError::ValidationException({
@@ -108,10 +106,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_group::UpdatePackageGroupError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::update_package_group::UpdatePackageGroupError::generic(generic),
```

### `src/protocol_serde/shape_update_package_group_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_package_group_input.rs
+++ generated/src/protocol_serde/shape_update_package_group_input.rs
@@ -3,14 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_package_group::UpdatePackageGroupInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.contact_info {
-        object.key("contactInfo").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.description {
-        object.key("description").string(var_2.as_str());
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
     }
     if let Some(var_3) = &input.package_group {
         object.key("packageGroup").string(var_3.as_str());
     }
+    if let Some(var_4) = &input.contact_info {
+        object.key("contactInfo").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.description {
+        object.key("description").string(var_5.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_package_group_origin_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_update_package_group_origin_configuration.rs
+++ generated/src/protocol_serde/shape_update_package_group_origin_configuration.rs
@@ -33,10 +33,11 @@
                             super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled,
                         )?;
                     let output = output.meta(generic);
-                    super::serde_util::access_denied_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -51,10 +52,11 @@
                             super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled,
                         )?;
                     let output = output.meta(generic);
-                    super::serde_util::internal_server_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -70,10 +72,11 @@
                             super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled,
                         )?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -89,10 +92,11 @@
                     )
                     .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -105,18 +109,12 @@
                     output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(
                         super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled,
                     )?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
-                    super::serde_util::throttling_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -130,10 +128,11 @@
                         super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled,
                     )?;
                     let output = output.meta(generic);
-                    super::serde_util::validation_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -193,6 +192,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "packageGroup" => {
+                    builder = builder.set_package_group(super::protocol_serde::shape_package_group_description::de_package_group_description(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "allowedRepositoryUpdates" => {
                     builder = builder.set_allowed_repository_updates(
                         super::protocol_serde::shape_package_group_allowed_repository_updates::de_package_group_allowed_repository_updates(
@@ -202,13 +208,6 @@
                         )?,
                     );
                 }
-                "packageGroup" => {
-                    builder = builder.set_package_group(super::protocol_serde::shape_package_group_description::de_package_group_description(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_update_package_group_origin_configuration_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_package_group_origin_configuration_input.rs
+++ generated/src/protocol_serde/shape_update_package_group_origin_configuration_input.rs
@@ -3,39 +3,48 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_package_group_origin_configuration::UpdatePackageGroupOriginConfigurationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.add_allowed_repositories {
-        let mut array_2 = object.key("addAllowedRepositories").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.package_group {
+        object.key("packageGroup").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.restrictions {
+        #[allow(unused_mut)]
+        let mut object_5 = object.key("restrictions").start_object();
+        for (key_6, value_7) in var_4 {
             {
-                #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::protocol_serde::shape_package_group_allowed_repository::ser_package_group_allowed_repository(&mut object_4, item_3)?;
-                object_4.finish();
+                object_5.key(key_6.as_str()).string(value_7.as_str());
             }
         }
-        array_2.finish();
+        object_5.finish();
     }
-    if let Some(var_5) = &input.remove_allowed_repositories {
-        let mut array_6 = object.key("removeAllowedRepositories").start_array();
-        for item_7 in var_5 {
+    if let Some(var_8) = &input.add_allowed_repositories {
+        let mut array_9 = object.key("addAllowedRepositories").start_array();
+        for item_10 in var_8 {
             {
                 #[allow(unused_mut)]
-                let mut object_8 = array_6.value().start_object();
-                super::protocol_serde::shape_package_group_allowed_repository::ser_package_group_allowed_repository(&mut object_8, item_7)?;
-                object_8.finish();
+                let mut object_11 = array_9.value().start_object();
+                super::protocol_serde::shape_package_group_allowed_repository::ser_package_group_allowed_repository(&mut object_11, item_10)?;
+                object_11.finish();
             }
         }
-        array_6.finish();
+        array_9.finish();
     }
-    if let Some(var_9) = &input.restrictions {
-        #[allow(unused_mut)]
-        let mut object_10 = object.key("restrictions").start_object();
-        for (key_11, value_12) in var_9 {
+    if let Some(var_12) = &input.remove_allowed_repositories {
+        let mut array_13 = object.key("removeAllowedRepositories").start_array();
+        for item_14 in var_12 {
             {
-                object_10.key(key_11.as_str()).string(value_12.as_str());
+                #[allow(unused_mut)]
+                let mut object_15 = array_13.value().start_object();
+                super::protocol_serde::shape_package_group_allowed_repository::ser_package_group_allowed_repository(&mut object_15, item_14)?;
+                object_15.finish();
             }
         }
-        object_10.finish();
+        array_13.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_package_versions_status.rs`

```diff
--- reference/src/protocol_serde/shape_update_package_versions_status.rs
+++ generated/src/protocol_serde/shape_update_package_versions_status.rs
@@ -28,10 +28,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::ConflictException({
@@ -42,10 +43,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::InternalServerException({
@@ -56,10 +58,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => {
@@ -72,10 +75,11 @@
                         super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                             .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?;
                     let output = output.meta(generic);
-                    super::serde_util::resource_not_found_exception_correct_errors(output)
-                        .build()
-                        .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?
+                    output.build()
                 };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
                 tmp
             })
         }
@@ -86,18 +90,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::ValidationException({
@@ -108,10 +106,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::update_package_versions_status::UpdatePackageVersionsStatusError::generic(generic),
@@ -163,13 +162,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "failedVersions" => {
-                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "successfulVersions" => {
                     builder = builder.set_successful_versions(
                         super::protocol_serde::shape_successful_package_version_info_map::de_successful_package_version_info_map(
@@ -179,6 +171,13 @@
                         )?,
                     );
                 }
+                "failedVersions" => {
+                    builder = builder.set_failed_versions(super::protocol_serde::shape_package_version_error_map::de_package_version_error_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_update_package_versions_status_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_package_versions_status_input.rs
+++ generated/src/protocol_serde/shape_update_package_versions_status_input.rs
@@ -3,21 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_package_versions_status::UpdatePackageVersionsStatusInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.expected_status {
-        object.key("expectedStatus").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.format {
+        object.key("format").string(var_4.as_str());
     }
-    if let Some(var_2) = &input.target_status {
-        object.key("targetStatus").string(var_2.as_str());
+    if let Some(var_5) = &input.namespace {
+        object.key("namespace").string(var_5.as_str());
     }
-    if let Some(var_3) = &input.version_revisions {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("versionRevisions").start_object();
-        for (key_5, value_6) in var_3 {
-            {
-                object_4.key(key_5.as_str()).string(value_6.as_str());
-            }
-        }
-        object_4.finish();
+    if let Some(var_6) = &input.package {
+        object.key("package").string(var_6.as_str());
     }
     if let Some(var_7) = &input.versions {
         let mut array_8 = object.key("versions").start_array();
@@ -28,5 +30,21 @@
         }
         array_8.finish();
     }
+    if let Some(var_10) = &input.version_revisions {
+        #[allow(unused_mut)]
+        let mut object_11 = object.key("versionRevisions").start_object();
+        for (key_12, value_13) in var_10 {
+            {
+                object_11.key(key_12.as_str()).string(value_13.as_str());
+            }
+        }
+        object_11.finish();
+    }
+    if let Some(var_14) = &input.expected_status {
+        object.key("expectedStatus").string(var_14.as_str());
+    }
+    if let Some(var_15) = &input.target_status {
+        object.key("targetStatus").string(var_15.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_repository.rs`

```diff
--- reference/src/protocol_serde/shape_update_repository.rs
+++ generated/src/protocol_serde/shape_update_repository.rs
@@ -25,10 +25,11 @@
                 output = super::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::access_denied_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ConflictException" => super::operation::update_repository::UpdateRepositoryError::ConflictException({
@@ -39,10 +40,11 @@
                 output = super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::conflict_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "InternalServerException" => super::operation::update_repository::UpdateRepositoryError::InternalServerException({
@@ -53,10 +55,11 @@
                 output = super::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::internal_server_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ResourceNotFoundException" => super::operation::update_repository::UpdateRepositoryError::ResourceNotFoundException({
@@ -67,10 +70,11 @@
                 output = super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::resource_not_found_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ServiceQuotaExceededException" => super::operation::update_repository::UpdateRepositoryError::ServiceQuotaExceededException({
@@ -84,10 +88,11 @@
                 )
                 .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::service_quota_exceeded_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ThrottlingException" => super::operation::update_repository::UpdateRepositoryError::ThrottlingException({
@@ -97,18 +102,12 @@
                 let mut output = super::types::error::builders::ThrottlingExceptionBuilder::default();
                 output = super::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_repository::UpdateRepositoryError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
-                super::serde_util::throttling_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         "ValidationException" => super::operation::update_repository::UpdateRepositoryError::ValidationException({
@@ -119,10 +118,11 @@
                 output = super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?;
                 let output = output.meta(generic);
-                super::serde_util::validation_exception_correct_errors(output)
-                    .build()
-                    .map_err(super::operation::update_repository::UpdateRepositoryError::unhandled)?
+                output.build()
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::operation::update_repository::UpdateRepositoryError::generic(generic),
```

### `src/protocol_serde/shape_update_repository_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_repository_input.rs
+++ generated/src/protocol_serde/shape_update_repository_input.rs
@@ -3,20 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_repository::UpdateRepositoryInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.description {
-        object.key("description").string(var_1.as_str());
+    if let Some(var_1) = &input.domain {
+        object.key("domain").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.domain_owner {
+        object.key("domainOwner").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.repository {
+        object.key("repository").string(var_3.as_str());
     }
-    if let Some(var_2) = &input.upstreams {
-        let mut array_3 = object.key("upstreams").start_array();
-        for item_4 in var_2 {
+    if let Some(var_4) = &input.description {
+        object.key("description").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.upstreams {
+        let mut array_6 = object.key("upstreams").start_array();
+        for item_7 in var_5 {
             {
                 #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::protocol_serde::shape_upstream_repository::ser_upstream_repository(&mut object_5, item_4)?;
-                object_5.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::protocol_serde::shape_upstream_repository::ser_upstream_repository(&mut object_8, item_7)?;
+                object_8.finish();
             }
         }
-        array_3.finish();
+        array_6.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_validation_exception.rs`

```diff
--- reference/src/protocol_serde/shape_validation_exception.rs
+++ generated/src/protocol_serde/shape_validation_exception.rs
@@ -40,5 +40,7 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::serde_util::validation_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -101,8 +101,6 @@

 pub(crate) mod shape_publish_package_version;

-pub(crate) mod shape_publish_package_version_input;
-
 pub(crate) mod shape_put_domain_permissions_policy;

 pub(crate) mod shape_put_package_origin_configuration;
@@ -131,6 +129,8 @@

 pub(crate) mod shape_access_denied_exception;

+pub(crate) mod shape_associate_external_connection_input;
+
 pub(crate) mod shape_conflict_exception;

 pub(crate) mod shape_copy_package_versions_input;
@@ -141,16 +141,76 @@

 pub(crate) mod shape_create_repository_input;

+pub(crate) mod shape_delete_domain_input;
+
+pub(crate) mod shape_delete_domain_permissions_policy_input;
+
+pub(crate) mod shape_delete_package_group_input;
+
+pub(crate) mod shape_delete_package_input;
+
 pub(crate) mod shape_delete_package_versions_input;

+pub(crate) mod shape_delete_repository_input;
+
+pub(crate) mod shape_delete_repository_permissions_policy_input;
+
+pub(crate) mod shape_describe_domain_input;
+
+pub(crate) mod shape_describe_package_group_input;
+
+pub(crate) mod shape_describe_package_input;
+
+pub(crate) mod shape_describe_package_version_input;
+
+pub(crate) mod shape_describe_repository_input;
+
+pub(crate) mod shape_disassociate_external_connection_input;
+
 pub(crate) mod shape_dispose_package_versions_input;

-pub(crate) mod shape_get_package_version_asset_output;
+pub(crate) mod shape_get_associated_package_group_input;
+
+pub(crate) mod shape_get_authorization_token_input;
+
+pub(crate) mod shape_get_domain_permissions_policy_input;
+
+pub(crate) mod shape_get_package_version_asset_input;
+
+pub(crate) mod shape_get_package_version_readme_input;
+
+pub(crate) mod shape_get_repository_endpoint_input;
+
+pub(crate) mod shape_get_repository_permissions_policy_input;

 pub(crate) mod shape_internal_server_exception;

+pub(crate) mod shape_list_allowed_repositories_for_group_input;
+
+pub(crate) mod shape_list_associated_packages_input;
+
 pub(crate) mod shape_list_domains_input;

+pub(crate) mod shape_list_package_groups_input;
+
+pub(crate) mod shape_list_package_version_assets_input;
+
+pub(crate) mod shape_list_package_version_dependencies_input;
+
+pub(crate) mod shape_list_package_versions_input;
+
+pub(crate) mod shape_list_packages_input;
+
+pub(crate) mod shape_list_repositories_in_domain_input;
+
+pub(crate) mod shape_list_repositories_input;
+
+pub(crate) mod shape_list_sub_package_groups_input;
+
+pub(crate) mod shape_list_tags_for_resource_input;
+
+pub(crate) mod shape_publish_package_version_input;
+
 pub(crate) mod shape_put_domain_permissions_policy_input;

 pub(crate) mod shape_put_package_origin_configuration_input;
```

### `src/types/_package_version_origin_type.rs`

```diff
--- reference/src/types/_package_version_origin_type.rs
+++ generated/src/types/_package_version_origin_type.rs
@@ -14,7 +14,7 @@
 /// match packageversionorigintype {
 ///     PackageVersionOriginType::External => { /* ... */ },
 ///     PackageVersionOriginType::Internal => { /* ... */ },
-///     PackageVersionOriginType::UnknownValue => { /* ... */ },
+///     PackageVersionOriginType::Unknown => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -37,8 +37,7 @@
 /// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
 /// - It might inadvertently shadow other intended match arms.
 ///
-///
-/// _Note: `PackageVersionOriginType::Unknown` has been renamed to `::UnknownValue`._
+#[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
@@ -48,9 +47,8 @@
     External,
     #[allow(missing_docs)] // documentation missing in model
     Internal,
-    ///
-    /// _Note: `::Unknown` has been renamed to `::UnknownValue`._
-    UnknownValue,
+    #[allow(missing_docs)] // documentation missing in model
+    Unknown,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -60,7 +58,7 @@
         match s {
             "EXTERNAL" => PackageVersionOriginType::External,
             "INTERNAL" => PackageVersionOriginType::Internal,
-            "UNKNOWN" => PackageVersionOriginType::UnknownValue,
+            "UNKNOWN" => PackageVersionOriginType::Unknown,
             other => PackageVersionOriginType::Unknown(super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -78,7 +76,7 @@
         match self {
             PackageVersionOriginType::External => "EXTERNAL",
             PackageVersionOriginType::Internal => "INTERNAL",
-            PackageVersionOriginType::UnknownValue => "UNKNOWN",
+            PackageVersionOriginType::Unknown => "UNKNOWN",
             PackageVersionOriginType::Unknown(value) => value.as_str(),
         }
     }
@@ -109,7 +107,7 @@
         match self {
             PackageVersionOriginType::External => write!(f, "EXTERNAL"),
             PackageVersionOriginType::Internal => write!(f, "INTERNAL"),
-            PackageVersionOriginType::UnknownValue => write!(f, "UNKNOWN"),
+            PackageVersionOriginType::Unknown => write!(f, "UNKNOWN"),
             PackageVersionOriginType::Unknown(value) => write!(f, "{value}"),
         }
     }
```

### `src/types/error/_access_denied_exception.rs`

```diff
--- reference/src/types/error/_access_denied_exception.rs
+++ generated/src/types/error/_access_denied_exception.rs
@@ -10,15 +10,17 @@
 }
 impl AccessDeniedException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for AccessDeniedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "AccessDeniedException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_conflict_exception.rs`

```diff
--- reference/src/types/error/_conflict_exception.rs
+++ generated/src/types/error/_conflict_exception.rs
@@ -24,15 +24,17 @@
 }
 impl ConflictException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for ConflictException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "ConflictException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_internal_server_exception.rs`

```diff
--- reference/src/types/error/_internal_server_exception.rs
+++ generated/src/types/error/_internal_server_exception.rs
@@ -10,15 +10,17 @@
 }
 impl InternalServerException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for InternalServerException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "InternalServerException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_resource_not_found_exception.rs`

```diff
--- reference/src/types/error/_resource_not_found_exception.rs
+++ generated/src/types/error/_resource_not_found_exception.rs
@@ -24,15 +24,17 @@
 }
 impl ResourceNotFoundException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for ResourceNotFoundException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "ResourceNotFoundException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_service_quota_exceeded_exception.rs`

```diff
--- reference/src/types/error/_service_quota_exceeded_exception.rs
+++ generated/src/types/error/_service_quota_exceeded_exception.rs
@@ -24,15 +24,17 @@
 }
 impl ServiceQuotaExceededException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for ServiceQuotaExceededException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "ServiceQuotaExceededException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_throttling_exception.rs`

```diff
--- reference/src/types/error/_throttling_exception.rs
+++ generated/src/types/error/_throttling_exception.rs
@@ -18,15 +18,17 @@
 }
 impl ThrottlingException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for ThrottlingException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "ThrottlingException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_validation_exception.rs`

```diff
--- reference/src/types/error/_validation_exception.rs
+++ generated/src/types/error/_validation_exception.rs
@@ -18,15 +18,17 @@
 }
 impl ValidationException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for ValidationException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "ValidationException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### Missing reference files

- `src/protocol_serde/shape_get_package_version_asset_output.rs`

### Unexpected generated files

- `src/protocol_serde/shape_associate_external_connection_input.rs`
- `src/protocol_serde/shape_delete_domain_input.rs`
- `src/protocol_serde/shape_delete_domain_permissions_policy_input.rs`
- `src/protocol_serde/shape_delete_package_group_input.rs`
- `src/protocol_serde/shape_delete_package_input.rs`
- `src/protocol_serde/shape_delete_repository_input.rs`
- `src/protocol_serde/shape_delete_repository_permissions_policy_input.rs`
- `src/protocol_serde/shape_describe_domain_input.rs`
- `src/protocol_serde/shape_describe_package_group_input.rs`
- `src/protocol_serde/shape_describe_package_input.rs`
- `src/protocol_serde/shape_describe_package_version_input.rs`
- `src/protocol_serde/shape_describe_repository_input.rs`
- `src/protocol_serde/shape_disassociate_external_connection_input.rs`
- `src/protocol_serde/shape_get_associated_package_group_input.rs`
- `src/protocol_serde/shape_get_authorization_token_input.rs`
- `src/protocol_serde/shape_get_domain_permissions_policy_input.rs`
- `src/protocol_serde/shape_get_package_version_asset_input.rs`
- `src/protocol_serde/shape_get_package_version_readme_input.rs`
- `src/protocol_serde/shape_get_repository_endpoint_input.rs`
- `src/protocol_serde/shape_get_repository_permissions_policy_input.rs`
- `src/protocol_serde/shape_list_allowed_repositories_for_group_input.rs`
- `src/protocol_serde/shape_list_associated_packages_input.rs`
- `src/protocol_serde/shape_list_package_groups_input.rs`
- `src/protocol_serde/shape_list_package_version_assets_input.rs`
- `src/protocol_serde/shape_list_package_version_dependencies_input.rs`
- `src/protocol_serde/shape_list_package_versions_input.rs`
- `src/protocol_serde/shape_list_packages_input.rs`
- `src/protocol_serde/shape_list_repositories_in_domain_input.rs`
- `src/protocol_serde/shape_list_repositories_input.rs`
- `src/protocol_serde/shape_list_sub_package_groups_input.rs`
- `src/protocol_serde/shape_list_tags_for_resource_input.rs`
