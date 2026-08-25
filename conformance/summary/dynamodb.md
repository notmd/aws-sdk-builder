# AWS SDK Conformance Report: dynamodb

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## dynamodb
**Progress:** `882/882` files compared · `881` matched · `1` mismatches · `0` missing · `0` extra · `99.89%` match (100.00% means fully matched)

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -153,422 +153,391 @@
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
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned()).build())
                         },
-5 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: FIPS and local endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-6 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Dualstack and local endpoint are not supported"
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
-                        },
-8 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: AccountIdEndpointMode is required and FIPS is enabled, but FIPS account endpoints are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
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
-                        },
-16 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled but this partition does not support FIPS"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
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
-                        },
-28 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled but this partition does not support DualStack"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
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
-                        },
-35 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Missing Region"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
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
@@ -626,10 +595,8 @@
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
@@ -658,10 +625,8 @@
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
@@ -704,7 +669,7 @@
                         16 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_2 {
                                     inner.region()
                                 } else {
@@ -711,7 +676,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         17 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
@@ -754,10 +719,11 @@
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
@@ -777,7 +743,7 @@
                         23 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_1 {
                                     inner.region()
                                 } else {
@@ -784,7 +750,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         24 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
```
