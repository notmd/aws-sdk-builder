# AWS SDK Conformance Report: dynamodb

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## dynamodb
**Progress:** `882/882` files compared · `873` matched · `9` mismatches · `0` missing · `0` extra · `98.98%` match (100.00% means fully matched)

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -153,422 +153,396 @@
             match current_ref {
                 ref_val if ref_val >= 100_000_000 => {
                     return match (ref_val - 100_000_000) as usize {
-                                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched")) as ::aws_smithy_runtime_api::box_error::BoxError),
-1 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: FIPS and custom endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-2 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Dualstack and custom endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-3 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Endpoint override is not supported for dual-stack endpoints. Please enable dual-stack functionality by enabling the configuration. For more details, see: https://docs.aws.amazon.com/sdkref/latest/guide/feature-endpoints.html"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-4 => {
+                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "No endpoint rule matched",
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        1 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        2 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        3 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Endpoint override is not supported for dual-stack endpoints. Please enable dual-stack functionality by enabling the configuration. For more details, see: https://docs.aws.amazon.com/sdkref/latest/guide/feature-endpoints.html".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        4 => {
                             let endpoint = params.endpoint.as_deref().unwrap_or_default();
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned())
-.build())
-                        },
-5 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: FIPS and local endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-6 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Dualstack and local endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-7 => {
-
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url("http://localhost:8000"
-.to_string())
-.auth_scheme(::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4"
-.to_string(), 2)
-.put("signingName", "dynamodb")
-.put("signingRegion", "us-east-1")
-)
-.build())
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&endpoint.as_ref());
+                                        out
+                                    }).build())
                         },
-8 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: AccountIdEndpointMode is required and FIPS is enabled, but FIPS account endpoints are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
+                        5 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: FIPS and local endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        6 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: Dualstack and local endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        7 => {
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url("http://localhost:8000".to_string()).auth_scheme(
+                                ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4".to_string(), 2)
+                                    .put("signingName", "dynamodb")
+                                    .put("signingRegion", "us-east-1")
+                            ).build())
                         },
-9 => {
+                        8 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: AccountIdEndpointMode is required and FIPS is enabled, but FIPS account endpoints are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        9 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-10 => {
+                        10 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-11 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-12 => {
+                        11 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "FIPS and DualStack are enabled, but this partition does not support one or both".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        12 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-13 => {
+                        13 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-14 => {
+                        14 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-15 => {
+                        15 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-16 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled but this partition does not support FIPS"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-17 => {
+                        16 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "FIPS is enabled but this partition does not support FIPS".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        17 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-18 => {
+                        18 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-19 => {
+                        19 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-20 => {
+                        20 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-21 => {
+                        21 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-22 => {
+                        22 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-23 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Credentials-sourced account ID parameter is invalid"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-24 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("AccountIdEndpointMode is required but no AccountID was provided or able to be loaded"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-25 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: AccountIdEndpointMode is required but account endpoints are not supported in this partition"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-26 => {
+                        23 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Credentials-sourced account ID parameter is invalid".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        24 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "AccountIdEndpointMode is required but no AccountID was provided or able to be loaded".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        25 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: AccountIdEndpointMode is required but account endpoints are not supported in this partition".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        26 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-27 => {
+                        27 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-28 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled but this partition does not support DualStack"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-29 => {
+                        28 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "DualStack is enabled but this partition does not support DualStack".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        29 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-30 => {
+                        30 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-31 => {
+                        31 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-32 => {
+                        32 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-33 => {
+                        33 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-34 => {
+                        34 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-35 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Missing Region"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-                                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched")) as ::aws_smithy_runtime_api::box_error::BoxError),
-                                    };
+                        35 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: Missing Region".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "No endpoint rule matched",
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                    };
                 }
                 1 | -1 => {
                     return ::std::result::Result::Err(
@@ -626,10 +600,8 @@
                             (&{
                                 let mut out = String::new();
                                 out.push_str("dynamodb.");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&region.as_deref().unwrap_or_default());
                                 out.push_str(".");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&if let Some(inner) = partition_result {
                                     inner.dual_stack_dns_suffix()
                                 } else {
@@ -658,10 +630,8 @@
                             (&{
                                 let mut out = String::new();
                                 out.push_str("search-dynamodb.");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&region.as_deref().unwrap_or_default());
                                 out.push_str(".");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&if let Some(inner) = partition_result {
                                     inner.dual_stack_dns_suffix()
                                 } else {
@@ -704,7 +674,7 @@
                         16 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_2 {
                                     inner.region()
                                 } else {
@@ -711,7 +681,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         17 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
@@ -754,10 +724,11 @@
                             let partition_resolver = &self.partition_resolver;
                             {
                                 *first_arn = if let Some(inner) = resource_arn_list {
-                                    inner.first().map(|s| s.as_str())
+                                    inner.first().cloned()
                                 } else {
                                     return false;
-                                };
+                                }
+                                .map(|inner| inner.into());
                                 first_arn.is_some()
                             }
                         })(&mut _diagnostic_collector),
@@ -777,7 +748,7 @@
                         23 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_1 {
                                     inner.region()
                                 } else {
@@ -784,7 +755,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         24 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
```

### `src/operation/describe_endpoints.rs`

```diff
--- reference/src/operation/describe_endpoints.rs
+++ generated/src/operation/describe_endpoints.rs
@@ -204,7 +204,6 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
             builder = _header_serialization_settings.set_default_header(
                 builder,
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
@@ -212,7 +211,7 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_endpoints::ser_describe_endpoints_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/describe_limits.rs`

```diff
--- reference/src/operation/describe_limits.rs
+++ generated/src/operation/describe_limits.rs
@@ -204,7 +204,6 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
             builder = _header_serialization_settings.set_default_header(
                 builder,
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
@@ -212,7 +211,7 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_limits::ser_describe_limits_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/protocol_serde/shape_describe_endpoints.rs`

```diff
--- reference/src/protocol_serde/shape_describe_endpoints.rs
+++ generated/src/protocol_serde/shape_describe_endpoints.rs
@@ -33,12 +33,6 @@
     })
 }

-pub fn ser_describe_endpoints_input(
-    _input: &super::super::operation::describe_endpoints::DescribeEndpointsInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
-    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
-}
-
 pub(crate) fn de_describe_endpoints(
     _value: &[u8],
     mut builder: super::super::operation::describe_endpoints::builders::DescribeEndpointsOutputBuilder,
```

### `src/protocol_serde/shape_describe_limits.rs`

```diff
--- reference/src/protocol_serde/shape_describe_limits.rs
+++ generated/src/protocol_serde/shape_describe_limits.rs
@@ -67,12 +67,6 @@
     })
 }

-pub fn ser_describe_limits_input(
-    _input: &super::super::operation::describe_limits::DescribeLimitsInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
-    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
-}
-
 pub(crate) fn de_describe_limits(
     _value: &[u8],
     mut builder: super::super::operation::describe_limits::builders::DescribeLimitsOutputBuilder,
```

### `src/protocol_serde/shape_put_resource_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_resource_policy_input.rs
+++ generated/src/protocol_serde/shape_put_resource_policy_input.rs
@@ -12,8 +12,5 @@
     if let Some(var_3) = &input.expected_revision_id {
         object.key("ExpectedRevisionId").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.confirm_remove_self_resource_access {
-        object.key("ConfirmRemoveSelfResourceAccess").boolean(*var_4);
-    }
     Ok(())
 }
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -14,7 +14,7 @@
     if builder.import_table_description.is_none() {
         builder.import_table_description = {
             let builder = super::types::builders::ImportTableDescriptionBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
@@ -26,7 +26,7 @@
     if builder.import_table_description.is_none() {
         builder.import_table_description = {
             let builder = super::types::builders::ImportTableDescriptionBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
```

### `src/types/_attribute_value.rs`

```diff
--- reference/src/types/_attribute_value.rs
+++ generated/src/types/_attribute_value.rs
@@ -75,7 +75,7 @@
     pub fn is_bool(&self) -> bool {
         self.as_bool().is_ok()
     }
-    /// Tries to convert the enum instance into [`Bs`](crate::types::AttributeValue::Bs), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Bs`](crate::types::AttributeValue::Bs), extracting the inner [`Vec::<Blob>`](::std::vec::Vec<::aws_smithy_types::Blob>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_bs(&self) -> ::std::result::Result<&::std::vec::Vec<::aws_smithy_types::Blob>, &Self> {
         if let AttributeValue::Bs(val) = &self {
@@ -88,7 +88,7 @@
     pub fn is_bs(&self) -> bool {
         self.as_bs().is_ok()
     }
-    /// Tries to convert the enum instance into [`L`](crate::types::AttributeValue::L), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`L`](crate::types::AttributeValue::L), extracting the inner [`Vec::<AttributeValue>`](::std::vec::Vec<crate::types::AttributeValue>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_l(&self) -> ::std::result::Result<&::std::vec::Vec<super::super::types::AttributeValue>, &Self> {
         if let AttributeValue::L(val) = &self {
@@ -101,7 +101,7 @@
     pub fn is_l(&self) -> bool {
         self.as_l().is_ok()
     }
-    /// Tries to convert the enum instance into [`M`](crate::types::AttributeValue::M), extracting the inner [`HashMap`](::std::collections::HashMap).
+    /// Tries to convert the enum instance into [`M`](crate::types::AttributeValue::M), extracting the inner [`HashMap::<String, AttributeValue>`](::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_m(&self) -> ::std::result::Result<&::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>, &Self> {
         if let AttributeValue::M(val) = &self {
@@ -127,7 +127,7 @@
     pub fn is_n(&self) -> bool {
         self.as_n().is_ok()
     }
-    /// Tries to convert the enum instance into [`Ns`](crate::types::AttributeValue::Ns), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Ns`](crate::types::AttributeValue::Ns), extracting the inner [`Vec::<String>`](::std::vec::Vec<::std::string::String>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_ns(&self) -> ::std::result::Result<&::std::vec::Vec<::std::string::String>, &Self> {
         if let AttributeValue::Ns(val) = &self {
@@ -166,7 +166,7 @@
     pub fn is_s(&self) -> bool {
         self.as_s().is_ok()
     }
-    /// Tries to convert the enum instance into [`Ss`](crate::types::AttributeValue::Ss), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Ss`](crate::types::AttributeValue::Ss), extracting the inner [`Vec::<String>`](::std::vec::Vec<::std::string::String>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_ss(&self) -> ::std::result::Result<&::std::vec::Vec<::std::string::String>, &Self> {
         if let AttributeValue::Ss(val) = &self {
```

### `src/types/error/_replicated_write_conflict_exception.rs`

```diff
--- reference/src/types/error/_replicated_write_conflict_exception.rs
+++ generated/src/types/error/_replicated_write_conflict_exception.rs
@@ -11,7 +11,7 @@
 impl ReplicatedWriteConflictException {
     /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
     pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
-        ::aws_smithy_types::retry::ErrorKind::ClientError
+        ::aws_smithy_types::retry::ErrorKind::ServerError
     }
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
```
