# AWS SDK Conformance Report: codeartifact

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## codeartifact
**Progress:** `459/459` files compared · `445` matched · `14` mismatches · `0` missing · `0` extra · `96.95%` match (100.00% means fully matched)

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
```

### `src/operation/publish_package_version.rs`

```diff
--- reference/src/operation/publish_package_version.rs
+++ generated/src/operation/publish_package_version.rs
@@ -378,7 +378,9 @@
             builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
             builder
         };
-        let body = super::super::protocol_serde::shape_publish_package_version_input::ser_asset_content_http_payload(input.asset_content)?.into_inner();
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_publish_package_version_input::ser_asset_content_http_payload(input.asset_content)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
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
+    Ok(super::super::serde_util::access_denied_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
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
+    Ok(super::super::serde_util::conflict_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_get_package_version_asset_output.rs`

```diff
--- reference/src/protocol_serde/shape_get_package_version_asset_output.rs
+++ generated/src/protocol_serde/shape_get_package_version_asset_output.rs
@@ -1,12 +1,9 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn de_asset_payload(
-    body: &mut ::aws_smithy_types::body::SdkBody,
-) -> std::result::Result<::aws_smithy_types::byte_stream::ByteStream, super::super::operation::get_package_version_asset::GetPackageVersionAssetError> {
-    // replace the body with an empty body
-    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());
-    Ok(::aws_smithy_types::byte_stream::ByteStream::new(body))
+pub(crate) fn de_asset_payload(
+    body: &[u8],
+) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, super::super::operation::get_package_version_asset::GetPackageVersionAssetError> {
+    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
 }
-
 pub(crate) fn de_asset_name_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
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
+    Ok(super::super::serde_util::internal_server_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_publish_package_version.rs`

```diff
--- reference/src/protocol_serde/shape_publish_package_version.rs
+++ generated/src/protocol_serde/shape_publish_package_version.rs
@@ -151,24 +151,6 @@
     })
 }

-pub fn ser_publish_package_version_headers(
-    input: &super::super::operation::publish_package_version::PublishPackageVersionInput,
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
-}
-
 pub(crate) fn de_publish_package_version(
     _value: &[u8],
     mut builder: super::super::operation::publish_package_version::builders::PublishPackageVersionOutputBuilder,
```

### `src/protocol_serde/shape_publish_package_version_input.rs`

```diff
--- reference/src/protocol_serde/shape_publish_package_version_input.rs
+++ generated/src/protocol_serde/shape_publish_package_version_input.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_asset_content_http_payload(
-    payload: ::aws_smithy_types::byte_stream::ByteStream,
-) -> ::std::result::Result<::aws_smithy_types::byte_stream::ByteStream, ::aws_smithy_types::error::operation::BuildError> {
-    Ok(payload)
+    payload: ::std::option::Option<::aws_smithy_types::Blob>,
+) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {
+    let payload = match payload {
+        Some(t) => t,
+        None => return Ok(::bytes::Bytes::new()),
+    };
+    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())
 }
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
+    Ok(super::super::serde_util::resource_not_found_exception_correct_errors(builder)
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
+    Ok(super::super::serde_util::service_quota_exceeded_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }
```

### `src/protocol_serde/shape_throttling_exception.rs`

```diff
--- reference/src/protocol_serde/shape_throttling_exception.rs
+++ generated/src/protocol_serde/shape_throttling_exception.rs
@@ -40,7 +40,9 @@
             "found more JSON tokens after completing parsing",
         ));
     }
-    Ok(builder)
+    Ok(super::super::serde_util::throttling_exception_correct_errors(builder)
+        .build()
+        .map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom("missing field"))?)
 }

 pub(crate) fn de_retry_after_seconds_header(
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
+    Ok(super::super::serde_util::validation_exception_correct_errors(builder)
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
@@ -151,6 +149,8 @@

 pub(crate) mod shape_list_domains_input;

+pub(crate) mod shape_publish_package_version_input;
+
 pub(crate) mod shape_put_domain_permissions_policy_input;

 pub(crate) mod shape_put_package_origin_configuration_input;
```
