# AWS SDK Conformance Report: config

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## config
**Progress:** `1262/1262` files compared · `1233` matched · `29` mismatches · `0` missing · `0` extra · `97.70%` match (100.00% means fully matched)

### `src/client/associate_resource_types.rs`

```diff
--- reference/src/client/associate_resource_types.rs
+++ generated/src/client/associate_resource_types.rs
@@ -6,7 +6,7 @@
     ///   - [`configuration_recorder_arn(impl Into<String>)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::configuration_recorder_arn) / [`set_configuration_recorder_arn(Option<String>)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::set_configuration_recorder_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p><br>
     ///   - [`resource_types(ResourceType)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::resource_types) / [`set_resource_types(Option<Vec::<ResourceType>>)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::set_resource_types):<br>required: **true**<br><p>The list of resource types you want to add to the recording group of the specified configuration recorder.</p><br>
     /// - On success, responds with [`AssociateResourceTypesOutput`](crate::operation::associate_resource_types::AssociateResourceTypesOutput) with field(s):
-    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::associate_resource_types::AssociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b> </a> in the <i>Config Developer Guide</i>.</p>
+    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::associate_resource_types::AssociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b></a> in the <i>Config Developer Guide</i>.</p>
     /// - On failure, responds with [`SdkError<AssociateResourceTypesError>`](crate::operation::associate_resource_types::AssociateResourceTypesError)
     pub fn associate_resource_types(&self) -> super::super::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder {
         super::super::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::new(self.handle.clone())
```

### `src/client/describe_config_rules.rs`

```diff
--- reference/src/client/describe_config_rules.rs
+++ generated/src/client/describe_config_rules.rs
@@ -5,7 +5,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`config_rule_names(impl Into<String>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::config_rule_names) / [`set_config_rule_names(Option<Vec::<String>>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_config_rule_names):<br>required: **false**<br><p>The names of the Config rules for which you want details. If you do not specify any names, Config returns details for all your rules.</p><br>
-    ///   - [`filters(DescribeConfigRulesFilters)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::filters) / [`set_filters(Option<DescribeConfigRulesFilters>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_filters):<br>required: **false**<br><p>Returns a list of Detective or Proactive Config rules. By default, this API returns an unfiltered list. For more information on Detective or Proactive Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config-rules.html"> <b>Evaluation Mode</b> </a> in the <i>Config Developer Guide</i>.</p><br>
+    ///   - [`filters(DescribeConfigRulesFilters)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::filters) / [`set_filters(Option<DescribeConfigRulesFilters>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_filters):<br>required: **false**<br><p>Returns a list of Detective or Proactive Config rules. By default, this API returns an unfiltered list. For more information on Detective or Proactive Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config-rules.html"> <b>Evaluation Mode</b></a> in the <i>Config Developer Guide</i>.</p><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p><br>
     /// - On success, responds with [`DescribeConfigRulesOutput`](crate::operation::describe_config_rules::DescribeConfigRulesOutput) with field(s):
     ///   - [`config_rules(Option<Vec::<ConfigRule>>)`](crate::operation::describe_config_rules::DescribeConfigRulesOutput::config_rules): <p>The details about your Config rules.</p>
```

### `src/client/disassociate_resource_types.rs`

```diff
--- reference/src/client/disassociate_resource_types.rs
+++ generated/src/client/disassociate_resource_types.rs
@@ -6,7 +6,7 @@
     ///   - [`configuration_recorder_arn(impl Into<String>)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::configuration_recorder_arn) / [`set_configuration_recorder_arn(Option<String>)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::set_configuration_recorder_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p><br>
     ///   - [`resource_types(ResourceType)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::resource_types) / [`set_resource_types(Option<Vec::<ResourceType>>)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::set_resource_types):<br>required: **true**<br><p>The list of resource types you want to remove from the recording group of the specified configuration recorder.</p><br>
     /// - On success, responds with [`DisassociateResourceTypesOutput`](crate::operation::disassociate_resource_types::DisassociateResourceTypesOutput) with field(s):
-    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::disassociate_resource_types::DisassociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b> </a> in the <i>Config Developer Guide</i>.</p>
+    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::disassociate_resource_types::DisassociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b></a> in the <i>Config Developer Guide</i>.</p>
     /// - On failure, responds with [`SdkError<DisassociateResourceTypesError>`](crate::operation::disassociate_resource_types::DisassociateResourceTypesError)
     pub fn disassociate_resource_types(&self) -> super::super::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder {
         super::super::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::new(self.handle.clone())
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -766,8 +766,8 @@
     ///
     /// # Default Behavior
     ///
-    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `configservice`
-    /// (or `configservice-<region>` if a region is configured).
+    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `config`
+    /// (or `config-<region>` if a region is configured).
     /// All Config Service clients without an explicit retry partition will share this default partition.
     ///
     /// # Notes
@@ -1620,7 +1620,7 @@
         }
     }

-    let default_retry_partition = "configservice";
+    let default_retry_partition = "config";
     let default_retry_partition = match config.region() {
         Some(region) => ::std::borrow::Cow::from(format!("{default_retry_partition}-{region}")),
         None => ::std::borrow::Cow::from(default_retry_partition),
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -203,9 +203,9 @@

 mod lens;

+mod json_errors;
+
 mod serde_util;

-mod json_errors;
-
 #[doc(inline)]
 pub use client::Client;
```

### `src/meta.rs`

```diff
--- reference/src/meta.rs
+++ generated/src/meta.rs
@@ -1,6 +1,6 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =
-    ::aws_runtime::user_agent::ApiMetadata::new("configservice", super::meta::PKG_VERSION);
+    ::aws_runtime::user_agent::ApiMetadata::new("config", super::meta::PKG_VERSION);

 /// Crate version number.
 pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
```

### `src/operation/get_compliance_summary_by_config_rule.rs`

```diff
--- reference/src/operation/get_compliance_summary_by_config_rule.rs
+++ generated/src/operation/get_compliance_summary_by_config_rule.rs
@@ -214,7 +214,6 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
             builder = _header_serialization_settings.set_default_header(
                 builder,
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
@@ -222,9 +221,7 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_compliance_summary_by_config_rule::ser_get_compliance_summary_by_config_rule_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/protocol_serde/shape_delete_service_linked_configuration_recorder.rs`

```diff
--- reference/src/protocol_serde/shape_delete_service_linked_configuration_recorder.rs
+++ generated/src/protocol_serde/shape_delete_service_linked_configuration_recorder.rs
@@ -26,53 +26,47 @@
     Err(match error_code {
         "ConflictException" => super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::ConflictException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ConflictExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ConflictExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::unhandled)?;
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
         "NoSuchConfigurationRecorderException" => super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::NoSuchConfigurationRecorderException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationRecorderExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_recorder_exception::de_no_such_configuration_recorder_exception_json_err(_response_body, output).map_err(super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationRecorderExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_recorder_exception::de_no_such_configuration_recorder_exception_json_err(_response_body, output).map_err(super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::unhandled)?;
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
         "ValidationException" => super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::unhandled)?;
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
-        _ => super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::generic(generic)
+        _ => super::super::operation::delete_service_linked_configuration_recorder::DeleteServiceLinkedConfigurationRecorderError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_describe_aggregate_compliance_by_config_rules.rs`

```diff
--- reference/src/protocol_serde/shape_describe_aggregate_compliance_by_config_rules.rs
+++ generated/src/protocol_serde/shape_describe_aggregate_compliance_by_config_rules.rs
@@ -26,69 +26,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
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
         "ValidationException" => super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::unhandled)?;
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
-        _ => super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::generic(generic)
+        _ => super::super::operation::describe_aggregate_compliance_by_config_rules::DescribeAggregateComplianceByConfigRulesError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_describe_aggregate_compliance_by_conformance_packs.rs`

```diff
--- reference/src/protocol_serde/shape_describe_aggregate_compliance_by_conformance_packs.rs
+++ generated/src/protocol_serde/shape_describe_aggregate_compliance_by_conformance_packs.rs
@@ -28,69 +28,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
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
         "ValidationException" => super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::unhandled)?;
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
-        _ => super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::generic(generic)
+        _ => super::super::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksError::generic(generic),
     })
 }

@@ -147,9 +139,7 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
                     "AggregateComplianceByConformancePacks" => {
-                        builder = builder.set_aggregate_compliance_by_conformance_packs(
-                            super::super::protocol_serde::shape_aggregate_compliance_by_conformance_pack_list::de_aggregate_compliance_by_conformance_pack_list(tokens, _value, depth + 1)?
-                        );
+                        builder = builder.set_aggregate_compliance_by_conformance_packs(super::super::protocol_serde::shape_aggregate_compliance_by_conformance_pack_list::de_aggregate_compliance_by_conformance_pack_list(tokens, _value, depth + 1)?);
                     }
                     "NextToken" => {
                         builder = builder.set_next_token(
```

### `src/protocol_serde/shape_describe_configuration_aggregator_sources_status.rs`

```diff
--- reference/src/protocol_serde/shape_describe_configuration_aggregator_sources_status.rs
+++ generated/src/protocol_serde/shape_describe_configuration_aggregator_sources_status.rs
@@ -28,69 +28,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
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
         "InvalidParameterValueException" => super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValueException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidParameterValueExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidParameterValueExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::unhandled)?;
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
-        _ => super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::generic(generic)
+        _ => super::super::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_describe_organization_config_rule_statuses.rs`

```diff
--- reference/src/protocol_serde/shape_describe_organization_config_rule_statuses.rs
+++ generated/src/protocol_serde/shape_describe_organization_config_rule_statuses.rs
@@ -24,69 +24,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
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
         "NoSuchOrganizationConfigRuleException" => super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::NoSuchOrganizationConfigRuleException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchOrganizationConfigRuleExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_organization_config_rule_exception::de_no_such_organization_config_rule_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchOrganizationConfigRuleExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_organization_config_rule_exception::de_no_such_organization_config_rule_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
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
         "OrganizationAccessDeniedException" => super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::OrganizationAccessDeniedException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::unhandled)?;
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
-        _ => super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::generic(generic)
+        _ => super::super::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_describe_organization_conformance_pack_statuses.rs`

```diff
--- reference/src/protocol_serde/shape_describe_organization_conformance_pack_statuses.rs
+++ generated/src/protocol_serde/shape_describe_organization_conformance_pack_statuses.rs
@@ -28,69 +28,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
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
         "NoSuchOrganizationConformancePackException" => super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::NoSuchOrganizationConformancePackException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchOrganizationConformancePackExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_organization_conformance_pack_exception::de_no_such_organization_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchOrganizationConformancePackExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_organization_conformance_pack_exception::de_no_such_organization_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
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
         "OrganizationAccessDeniedException" => super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::OrganizationAccessDeniedException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::unhandled)?;
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
-        _ => super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::generic(generic)
+        _ => super::super::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_describe_organization_conformance_packs.rs`

```diff
--- reference/src/protocol_serde/shape_describe_organization_conformance_packs.rs
+++ generated/src/protocol_serde/shape_describe_organization_conformance_packs.rs
@@ -22,69 +22,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
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
         "NoSuchOrganizationConformancePackException" => super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::NoSuchOrganizationConformancePackException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchOrganizationConformancePackExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_organization_conformance_pack_exception::de_no_such_organization_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchOrganizationConformancePackExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_organization_conformance_pack_exception::de_no_such_organization_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
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
         "OrganizationAccessDeniedException" => super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::OrganizationAccessDeniedException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::unhandled)?;
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
-        _ => super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::generic(generic)
+        _ => super::super::operation::describe_organization_conformance_packs::DescribeOrganizationConformancePacksError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_evaluation.rs`

```diff
--- reference/src/protocol_serde/shape_evaluation.rs
+++ generated/src/protocol_serde/shape_evaluation.rs
@@ -18,7 +18,7 @@
     {
         object
             .key("OrderingTimestamp")
-            .date_time(&input.ordering_timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.ordering_timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_external_evaluation.rs`

```diff
--- reference/src/protocol_serde/shape_external_evaluation.rs
+++ generated/src/protocol_serde/shape_external_evaluation.rs
@@ -18,7 +18,7 @@
     {
         object
             .key("OrderingTimestamp")
-            .date_time(&input.ordering_timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+            .date_time(input.ordering_timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_get_aggregate_compliance_details_by_config_rule.rs`

```diff
--- reference/src/protocol_serde/shape_get_aggregate_compliance_details_by_config_rule.rs
+++ generated/src/protocol_serde/shape_get_aggregate_compliance_details_by_config_rule.rs
@@ -26,69 +26,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
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
         "ValidationException" => super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::unhandled)?;
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
-        _ => super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::generic(generic)
+        _ => super::super::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_get_aggregate_config_rule_compliance_summary.rs`

```diff
--- reference/src/protocol_serde/shape_get_aggregate_config_rule_compliance_summary.rs
+++ generated/src/protocol_serde/shape_get_aggregate_config_rule_compliance_summary.rs
@@ -26,69 +26,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
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
         "ValidationException" => super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::unhandled)?;
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
-        _ => super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::generic(generic)
+        _ => super::super::operation::get_aggregate_config_rule_compliance_summary::GetAggregateConfigRuleComplianceSummaryError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_get_aggregate_conformance_pack_compliance_summary.rs`

```diff
--- reference/src/protocol_serde/shape_get_aggregate_conformance_pack_compliance_summary.rs
+++ generated/src/protocol_serde/shape_get_aggregate_conformance_pack_compliance_summary.rs
@@ -28,69 +28,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
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
         "ValidationException" => super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::unhandled)?;
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
-        _ => super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::generic(generic)
+        _ => super::super::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError::generic(generic),
     })
 }

@@ -146,9 +138,7 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "AggregateConformancePackComplianceSummaries" => {
-                    builder = builder.set_aggregate_conformance_pack_compliance_summaries(
-                            super::super::protocol_serde::shape_aggregate_conformance_pack_compliance_summary_list::de_aggregate_conformance_pack_compliance_summary_list(tokens, _value, depth + 1)?
-                        );
+                    builder = builder.set_aggregate_conformance_pack_compliance_summaries(super::super::protocol_serde::shape_aggregate_conformance_pack_compliance_summary_list::de_aggregate_conformance_pack_compliance_summary_list(tokens, _value, depth + 1)?);
                 }
                 "GroupByKey" => {
                     builder = builder.set_group_by_key(
```

### `src/protocol_serde/shape_get_aggregate_discovered_resource_counts.rs`

```diff
--- reference/src/protocol_serde/shape_get_aggregate_discovered_resource_counts.rs
+++ generated/src/protocol_serde/shape_get_aggregate_discovered_resource_counts.rs
@@ -24,69 +24,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
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
         "NoSuchConfigurationAggregatorException" => super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::NoSuchConfigurationAggregatorException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigurationAggregatorExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_configuration_aggregator_exception::de_no_such_configuration_aggregator_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
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
         "ValidationException" => super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
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
-        _ => super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::generic(generic)
+        _ => super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::generic(generic),
     })
 }

@@ -109,7 +101,9 @@
         )
         .map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::get_aggregate_discovered_resource_counts_output_output_correct_errors(output).build()
+        super::super::serde_util::get_aggregate_discovered_resource_counts_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::get_aggregate_discovered_resource_counts::GetAggregateDiscoveredResourceCountsError::unhandled)?
     })
 }

```

### `src/protocol_serde/shape_get_compliance_summary_by_config_rule.rs`

```diff
--- reference/src/protocol_serde/shape_get_compliance_summary_by_config_rule.rs
+++ generated/src/protocol_serde/shape_get_compliance_summary_by_config_rule.rs
@@ -35,12 +35,6 @@
     })
 }

-pub fn ser_get_compliance_summary_by_config_rule_input(
-    _input: &super::super::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
-    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
-}
-
 pub(crate) fn de_get_compliance_summary_by_config_rule(
     _value: &[u8],
     mut builder: super::super::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleOutputBuilder,
```

### `src/protocol_serde/shape_get_conformance_pack_compliance_details.rs`

```diff
--- reference/src/protocol_serde/shape_get_conformance_pack_compliance_details.rs
+++ generated/src/protocol_serde/shape_get_conformance_pack_compliance_details.rs
@@ -22,85 +22,75 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
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
         "InvalidParameterValueException" => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::InvalidParameterValueException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidParameterValueExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidParameterValueExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
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
         "NoSuchConfigRuleInConformancePackException" => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::NoSuchConfigRuleInConformancePackException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConfigRuleInConformancePackExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_config_rule_in_conformance_pack_exception::de_no_such_config_rule_in_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConfigRuleInConformancePackExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_config_rule_in_conformance_pack_exception::de_no_such_config_rule_in_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
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
         "NoSuchConformancePackException" => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::NoSuchConformancePackException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchConformancePackExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_conformance_pack_exception::de_no_such_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchConformancePackExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_conformance_pack_exception::de_no_such_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::unhandled)?;
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
-        _ => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::generic(generic)
+        _ => super::super::operation::get_conformance_pack_compliance_details::GetConformancePackComplianceDetailsError::generic(generic),
     })
 }

@@ -165,9 +155,7 @@
                         );
                     }
                     "ConformancePackRuleEvaluationResults" => {
-                        builder = builder.set_conformance_pack_rule_evaluation_results(
-                            super::super::protocol_serde::shape_conformance_pack_rule_evaluation_results_list::de_conformance_pack_rule_evaluation_results_list(tokens, _value, depth + 1)?
-                        );
+                        builder = builder.set_conformance_pack_rule_evaluation_results(super::super::protocol_serde::shape_conformance_pack_rule_evaluation_results_list::de_conformance_pack_rule_evaluation_results_list(tokens, _value, depth + 1)?);
                     }
                     "NextToken" => {
                         builder = builder.set_next_token(
```

### `src/protocol_serde/shape_get_organization_config_rule_detailed_status.rs`

```diff
--- reference/src/protocol_serde/shape_get_organization_config_rule_detailed_status.rs
+++ generated/src/protocol_serde/shape_get_organization_config_rule_detailed_status.rs
@@ -26,69 +26,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
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
         "NoSuchOrganizationConfigRuleException" => super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::NoSuchOrganizationConfigRuleException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchOrganizationConfigRuleExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_organization_config_rule_exception::de_no_such_organization_config_rule_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchOrganizationConfigRuleExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_organization_config_rule_exception::de_no_such_organization_config_rule_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
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
         "OrganizationAccessDeniedException" => super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::OrganizationAccessDeniedException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::unhandled)?;
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
-        _ => super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::generic(generic)
+        _ => super::super::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_get_organization_conformance_pack_detailed_status.rs`

```diff
--- reference/src/protocol_serde/shape_get_organization_conformance_pack_detailed_status.rs
+++ generated/src/protocol_serde/shape_get_organization_conformance_pack_detailed_status.rs
@@ -28,69 +28,61 @@
     Err(match error_code {
         "InvalidLimitException" => super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::InvalidLimitException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidLimitExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
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
         "InvalidNextTokenException" => super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::InvalidNextTokenException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InvalidNextTokenExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
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
         "NoSuchOrganizationConformancePackException" => super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::NoSuchOrganizationConformancePackException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoSuchOrganizationConformancePackExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_such_organization_conformance_pack_exception::de_no_such_organization_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoSuchOrganizationConformancePackExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_such_organization_conformance_pack_exception::de_no_such_organization_conformance_pack_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
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
         "OrganizationAccessDeniedException" => super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::OrganizationAccessDeniedException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::unhandled)?;
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
-        _ => super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::generic(generic)
+        _ => super::super::operation::get_organization_conformance_pack_detailed_status::GetOrganizationConformancePackDetailedStatusError::generic(generic),
     })
 }

@@ -146,9 +138,7 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "OrganizationConformancePackDetailedStatuses" => {
-                    builder = builder.set_organization_conformance_pack_detailed_statuses(
-                            super::super::protocol_serde::shape_organization_conformance_pack_detailed_statuses::de_organization_conformance_pack_detailed_statuses(tokens, _value, depth + 1)?
-                        );
+                    builder = builder.set_organization_conformance_pack_detailed_statuses(super::super::protocol_serde::shape_organization_conformance_pack_detailed_statuses::de_organization_conformance_pack_detailed_statuses(tokens, _value, depth + 1)?);
                 }
                 "NextToken" => {
                     builder = builder.set_next_token(
```

### `src/protocol_serde/shape_put_organization_conformance_pack.rs`

```diff
--- reference/src/protocol_serde/shape_put_organization_conformance_pack.rs
+++ generated/src/protocol_serde/shape_put_organization_conformance_pack.rs
@@ -22,133 +22,117 @@
     Err(match error_code {
         "InsufficientPermissionsException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::InsufficientPermissionsException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "MaxNumberOfOrganizationConformancePacksExceededException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::MaxNumberOfOrganizationConformancePacksExceededException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::MaxNumberOfOrganizationConformancePacksExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_max_number_of_organization_conformance_packs_exceeded_exception::de_max_number_of_organization_conformance_packs_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::MaxNumberOfOrganizationConformancePacksExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_max_number_of_organization_conformance_packs_exceeded_exception::de_max_number_of_organization_conformance_packs_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "NoAvailableOrganizationException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::NoAvailableOrganizationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::NoAvailableOrganizationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_no_available_organization_exception::de_no_available_organization_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::NoAvailableOrganizationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_no_available_organization_exception::de_no_available_organization_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "OrganizationAccessDeniedException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::OrganizationAccessDeniedException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "OrganizationAllFeaturesNotEnabledException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::OrganizationAllFeaturesNotEnabledException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationAllFeaturesNotEnabledExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_all_features_not_enabled_exception::de_organization_all_features_not_enabled_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationAllFeaturesNotEnabledExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_all_features_not_enabled_exception::de_organization_all_features_not_enabled_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "OrganizationConformancePackTemplateValidationException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::OrganizationConformancePackTemplateValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::OrganizationConformancePackTemplateValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_organization_conformance_pack_template_validation_exception::de_organization_conformance_pack_template_validation_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::OrganizationConformancePackTemplateValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_organization_conformance_pack_template_validation_exception::de_organization_conformance_pack_template_validation_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "ResourceInUseException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::ResourceInUseException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ResourceInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ResourceInUseExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
         "ValidationException" => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::unhandled)?;
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
-        _ => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::generic(generic)
+        _ => super::super::operation::put_organization_conformance_pack::PutOrganizationConformancePackError::generic(generic),
     })
 }

```

### `src/protocol_serde/shape_put_third_party_service_linked_configuration_recorder.rs`

```diff
--- reference/src/protocol_serde/shape_put_third_party_service_linked_configuration_recorder.rs
+++ generated/src/protocol_serde/shape_put_third_party_service_linked_configuration_recorder.rs
@@ -27,53 +27,47 @@
     Err(match error_code {
         "ConflictException" => super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::ConflictException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ConflictExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ConflictExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
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
         "InsufficientPermissionsException" => super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::InsufficientPermissionsException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
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
         "ValidationException" => super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::ValidationException({
             #[allow(unused_mut)]
-            let mut tmp =
-                 {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                }
-            ;
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ValidationExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
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
-        _ => super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::generic(generic)
+        _ => super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::generic(generic),
     })
 }

@@ -89,9 +83,12 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::put_third_party_service_linked_configuration_recorder::builders::PutThirdPartyServiceLinkedConfigurationRecorderOutputBuilder::default();
-        output = super::super::protocol_serde::shape_put_third_party_service_linked_configuration_recorder::de_put_third_party_service_linked_configuration_recorder(_response_body, output).map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
+        output = super::super::protocol_serde::shape_put_third_party_service_linked_configuration_recorder::de_put_third_party_service_linked_configuration_recorder(_response_body, output)
+            .map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::put_third_party_service_linked_configuration_recorder_output_output_correct_errors(output).build().map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?
+        super::super::serde_util::put_third_party_service_linked_configuration_recorder_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::put_third_party_service_linked_configuration_recorder::PutThirdPartyServiceLinkedConfigurationRecorderError::unhandled)?
     })
 }

```

### `src/types/_remediation_execution_state.rs`

```diff
--- reference/src/types/_remediation_execution_state.rs
+++ generated/src/types/_remediation_execution_state.rs
@@ -16,7 +16,7 @@
 ///     RemediationExecutionState::InProgress => { /* ... */ },
 ///     RemediationExecutionState::Queued => { /* ... */ },
 ///     RemediationExecutionState::Succeeded => { /* ... */ },
-///     RemediationExecutionState::UnknownValue => { /* ... */ },
+///     RemediationExecutionState::Unknown => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -39,8 +39,7 @@
 /// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
 /// - It might inadvertently shadow other intended match arms.
 ///
-///
-/// _Note: `RemediationExecutionState::Unknown` has been renamed to `::UnknownValue`._
+#[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
@@ -54,9 +53,8 @@
     Queued,
     #[allow(missing_docs)] // documentation missing in model
     Succeeded,
-    ///
-    /// _Note: `::Unknown` has been renamed to `::UnknownValue`._
-    UnknownValue,
+    #[allow(missing_docs)] // documentation missing in model
+    Unknown,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -68,7 +66,7 @@
             "IN_PROGRESS" => RemediationExecutionState::InProgress,
             "QUEUED" => RemediationExecutionState::Queued,
             "SUCCEEDED" => RemediationExecutionState::Succeeded,
-            "UNKNOWN" => RemediationExecutionState::UnknownValue,
+            "UNKNOWN" => RemediationExecutionState::Unknown,
             other => RemediationExecutionState::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -88,7 +86,7 @@
             RemediationExecutionState::InProgress => "IN_PROGRESS",
             RemediationExecutionState::Queued => "QUEUED",
             RemediationExecutionState::Succeeded => "SUCCEEDED",
-            RemediationExecutionState::UnknownValue => "UNKNOWN",
+            RemediationExecutionState::Unknown => "UNKNOWN",
             RemediationExecutionState::Unknown(value) => value.as_str(),
         }
     }
@@ -121,7 +119,7 @@
             RemediationExecutionState::InProgress => write!(f, "IN_PROGRESS"),
             RemediationExecutionState::Queued => write!(f, "QUEUED"),
             RemediationExecutionState::Succeeded => write!(f, "SUCCEEDED"),
-            RemediationExecutionState::UnknownValue => write!(f, "UNKNOWN"),
+            RemediationExecutionState::Unknown => write!(f, "UNKNOWN"),
             RemediationExecutionState::Unknown(value) => write!(f, "{value}"),
         }
     }
```

### `src/types/_remediation_execution_step_state.rs`

```diff
--- reference/src/types/_remediation_execution_step_state.rs
+++ generated/src/types/_remediation_execution_step_state.rs
@@ -17,7 +17,7 @@
 ///     RemediationExecutionStepState::InProgress => { /* ... */ },
 ///     RemediationExecutionStepState::Pending => { /* ... */ },
 ///     RemediationExecutionStepState::Succeeded => { /* ... */ },
-///     RemediationExecutionStepState::UnknownValue => { /* ... */ },
+///     RemediationExecutionStepState::Unknown => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -40,8 +40,7 @@
 /// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
 /// - It might inadvertently shadow other intended match arms.
 ///
-///
-/// _Note: `RemediationExecutionStepState::Unknown` has been renamed to `::UnknownValue`._
+#[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
@@ -57,9 +56,8 @@
     Pending,
     #[allow(missing_docs)] // documentation missing in model
     Succeeded,
-    ///
-    /// _Note: `::Unknown` has been renamed to `::UnknownValue`._
-    UnknownValue,
+    #[allow(missing_docs)] // documentation missing in model
+    Unknown,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -72,7 +70,7 @@
             "IN_PROGRESS" => RemediationExecutionStepState::InProgress,
             "PENDING" => RemediationExecutionStepState::Pending,
             "SUCCEEDED" => RemediationExecutionStepState::Succeeded,
-            "UNKNOWN" => RemediationExecutionStepState::UnknownValue,
+            "UNKNOWN" => RemediationExecutionStepState::Unknown,
             other => RemediationExecutionStepState::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -93,7 +91,7 @@
             RemediationExecutionStepState::InProgress => "IN_PROGRESS",
             RemediationExecutionStepState::Pending => "PENDING",
             RemediationExecutionStepState::Succeeded => "SUCCEEDED",
-            RemediationExecutionStepState::UnknownValue => "UNKNOWN",
+            RemediationExecutionStepState::Unknown => "UNKNOWN",
             RemediationExecutionStepState::Unknown(value) => value.as_str(),
         }
     }
@@ -127,7 +125,7 @@
             RemediationExecutionStepState::InProgress => write!(f, "IN_PROGRESS"),
             RemediationExecutionStepState::Pending => write!(f, "PENDING"),
             RemediationExecutionStepState::Succeeded => write!(f, "SUCCEEDED"),
-            RemediationExecutionStepState::UnknownValue => write!(f, "UNKNOWN"),
+            RemediationExecutionStepState::Unknown => write!(f, "UNKNOWN"),
             RemediationExecutionStepState::Unknown(value) => write!(f, "{value}"),
         }
     }
```

### `src/types/_resource_type.rs`

```diff
--- reference/src/types/_resource_type.rs
+++ generated/src/types/_resource_type.rs
@@ -12,21 +12,19 @@
 /// ```text
 /// # let resourcetype = unimplemented!();
 /// match resourcetype {
-///     ResourceType::Certificate => { /* ... */ },
 ///     ResourceType::AcmpcaCertificateAuthority => { /* ... */ },
 ///     ResourceType::AcmpcaCertificateAuthorityActivation => { /* ... */ },
 ///     ResourceType::ApsRuleGroupsNamespace => { /* ... */ },
 ///     ResourceType::AccessAnalyzerAnalyzer => { /* ... */ },
+///     ResourceType::AccountPublicAccessBlock => { /* ... */ },
+///     ResourceType::Alarm => { /* ... */ },
 ///     ResourceType::AmazonMqBroker => { /* ... */ },
 ///     ResourceType::AmplifyApp => { /* ... */ },
 ///     ResourceType::AmplifyBranch => { /* ... */ },
+///     ResourceType::Api => { /* ... */ },
 ///     ResourceType::ApiGatewayMethod => { /* ... */ },
-///     ResourceType::RestApi => { /* ... */ },
-///     ResourceType::Stage => { /* ... */ },
 ///     ResourceType::ApiGatewayUsagePlan => { /* ... */ },
-///     ResourceType::Api => { /* ... */ },
 ///     ResourceType::ApiGatewayV2Integration => { /* ... */ },
-///     ResourceType::StageV2 => { /* ... */ },
 ///     ResourceType::AppConfigApplication => { /* ... */ },
 ///     ResourceType::AppConfigConfigurationProfile => { /* ... */ },
 ///     ResourceType::AppConfigDeploymentStrategy => { /* ... */ },
@@ -53,53 +51,56 @@
 ///     ResourceType::AppStreamStack => { /* ... */ },
 ///     ResourceType::AppSyncApiCache => { /* ... */ },
 ///     ResourceType::AppSyncGraphQlApi => { /* ... */ },
+///     ResourceType::Application => { /* ... */ },
+///     ResourceType::ApplicationVersion => { /* ... */ },
+///     ResourceType::AssociationCompliance => { /* ... */ },
 ///     ResourceType::AthenaDataCatalog => { /* ... */ },
 ///     ResourceType::AthenaPreparedStatement => { /* ... */ },
 ///     ResourceType::AthenaWorkGroup => { /* ... */ },
 ///     ResourceType::AuditManagerAssessment => { /* ... */ },
 ///     ResourceType::AutoScalingGroup => { /* ... */ },
-///     ResourceType::LaunchConfiguration => { /* ... */ },
-///     ResourceType::ScalingPolicy => { /* ... */ },
-///     ResourceType::ScheduledAction => { /* ... */ },
 ///     ResourceType::AutoScalingWarmPool => { /* ... */ },
 ///     ResourceType::B2BiCapability => { /* ... */ },
 ///     ResourceType::BcmDataExportsExport => { /* ... */ },
+///     ResourceType::BackupGatewayHypervisor => { /* ... */ },
 ///     ResourceType::BackupPlan => { /* ... */ },
-///     ResourceType::BackupSelection => { /* ... */ },
-///     ResourceType::BackupVault => { /* ... */ },
 ///     ResourceType::BackupRecoveryPoint => { /* ... */ },
 ///     ResourceType::BackupReportPlan => { /* ... */ },
 ///     ResourceType::BackupRestoreTestingPlan => { /* ... */ },
-///     ResourceType::BackupGatewayHypervisor => { /* ... */ },
+///     ResourceType::BackupSelection => { /* ... */ },
+///     ResourceType::BackupVault => { /* ... */ },
 ///     ResourceType::BatchComputeEnvironment => { /* ... */ },
 ///     ResourceType::BatchJobQueue => { /* ... */ },
 ///     ResourceType::BatchSchedulingPolicy => { /* ... */ },
+///     ResourceType::BedrockAgentCoreBrowserCustom => { /* ... */ },
+///     ResourceType::BedrockAgentCoreRuntime => { /* ... */ },
 ///     ResourceType::BedrockApplicationInferenceProfile => { /* ... */ },
 ///     ResourceType::BedrockGuardrail => { /* ... */ },
 ///     ResourceType::BedrockKnowledgeBase => { /* ... */ },
 ///     ResourceType::BedrockPrompt => { /* ... */ },
-///     ResourceType::BedrockAgentCoreBrowserCustom => { /* ... */ },
-///     ResourceType::BedrockAgentCoreRuntime => { /* ... */ },
+///     ResourceType::Bucket => { /* ... */ },
 ///     ResourceType::BudgetsBudgetsAction => { /* ... */ },
 ///     ResourceType::CassandraKeyspace => { /* ... */ },
+///     ResourceType::Certificate => { /* ... */ },
 ///     ResourceType::CleanRoomsMlTrainingDataset => { /* ... */ },
 ///     ResourceType::Cloud9EnvironmentEc2 => { /* ... */ },
 ///     ResourceType::CloudFormationGuardHook => { /* ... */ },
 ///     ResourceType::CloudFormationLambdaHook => { /* ... */ },
-///     ResourceType::Stack => { /* ... */ },
+///     ResourceType::CloudFormationProduct => { /* ... */ },
+///     ResourceType::CloudFormationProvisionedProduct => { /* ... */ },
 ///     ResourceType::CloudFormationStackSet => { /* ... */ },
-///     ResourceType::Distribution => { /* ... */ },
 ///     ResourceType::CloudFrontKeyValueStore => { /* ... */ },
 ///     ResourceType::CloudFrontPublicKey => { /* ... */ },
 ///     ResourceType::CloudFrontRealtimeLogConfig => { /* ... */ },
-///     ResourceType::StreamingDistribution => { /* ... */ },
 ///     ResourceType::CloudTrailEventDataStore => { /* ... */ },
-///     ResourceType::Trail => { /* ... */ },
-///     ResourceType::Alarm => { /* ... */ },
 ///     ResourceType::CloudWatchMetricStream => { /* ... */ },
+///     ResourceType::Cluster => { /* ... */ },
+///     ResourceType::ClusterParameterGroup => { /* ... */ },
+///     ResourceType::ClusterSecurityGroup => { /* ... */ },
+///     ResourceType::ClusterSnapshot => { /* ... */ },
+///     ResourceType::ClusterSubnetGroup => { /* ... */ },
 ///     ResourceType::CodeArtifactDomain => { /* ... */ },
 ///     ResourceType::CodeArtifactRepository => { /* ... */ },
-///     ResourceType::Project => { /* ... */ },
 ///     ResourceType::CodeBuildReportGroup => { /* ... */ },
 ///     ResourceType::CodeDeployApplication => { /* ... */ },
 ///     ResourceType::CodeDeployDeploymentConfig => { /* ... */ },
@@ -106,7 +107,6 @@
 ///     ResourceType::CodeDeployDeploymentGroup => { /* ... */ },
 ///     ResourceType::CodeGuruProfilerProfilingGroup => { /* ... */ },
 ///     ResourceType::CodeGuruReviewerRepositoryAssociation => { /* ... */ },
-///     ResourceType::Pipeline => { /* ... */ },
 ///     ResourceType::CognitoIdentityPool => { /* ... */ },
 ///     ResourceType::CognitoUserPool => { /* ... */ },
 ///     ResourceType::CognitoUserPoolClient => { /* ... */ },
@@ -114,9 +114,8 @@
 ///     ResourceType::ComprehendFlywheel => { /* ... */ },
 ///     ResourceType::ConfigAggregationAuthorization => { /* ... */ },
 ///     ResourceType::ConfigConformancePack => { /* ... */ },
+///     ResourceType::ConfigStoredQuery => { /* ... */ },
 ///     ResourceType::ConformancePackCompliance => { /* ... */ },
-///     ResourceType::ResourceCompliance => { /* ... */ },
-///     ResourceType::ConfigStoredQuery => { /* ... */ },
 ///     ResourceType::ConnectInstance => { /* ... */ },
 ///     ResourceType::ConnectPhoneNumber => { /* ... */ },
 ///     ResourceType::ConnectQuickConnect => { /* ... */ },
@@ -123,8 +122,15 @@
 ///     ResourceType::ConnectRule => { /* ... */ },
 ///     ResourceType::ConnectSecurityProfile => { /* ... */ },
 ///     ResourceType::ConnectUser => { /* ... */ },
+///     ResourceType::CustomerGateway => { /* ... */ },
 ///     ResourceType::CustomerProfilesDomain => { /* ... */ },
 ///     ResourceType::CustomerProfilesObjectType => { /* ... */ },
+///     ResourceType::DbCluster => { /* ... */ },
+///     ResourceType::DbClusterSnapshot => { /* ... */ },
+///     ResourceType::DbInstance => { /* ... */ },
+///     ResourceType::DbSecurityGroup => { /* ... */ },
+///     ResourceType::DbSnapshot => { /* ... */ },
+///     ResourceType::DbSubnetGroup => { /* ... */ },
 ///     ResourceType::DmsCertificate => { /* ... */ },
 ///     ResourceType::DmsEndpoint => { /* ... */ },
 ///     ResourceType::DmsEventSubscription => { /* ... */ },
@@ -146,19 +152,15 @@
 ///     ResourceType::DeviceFarmInstanceProfile => { /* ... */ },
 ///     ResourceType::DeviceFarmProject => { /* ... */ },
 ///     ResourceType::DeviceFarmTestGridProject => { /* ... */ },
-///     ResourceType::Table => { /* ... */ },
+///     ResourceType::Distribution => { /* ... */ },
+///     ResourceType::Domain => { /* ... */ },
 ///     ResourceType::Ec2CapacityReservation => { /* ... */ },
 ///     ResourceType::Ec2CarrierGateway => { /* ... */ },
 ///     ResourceType::Ec2ClientVpnEndpoint => { /* ... */ },
 ///     ResourceType::Ec2ClientVpnTargetNetworkAssociation => { /* ... */ },
-///     ResourceType::CustomerGateway => { /* ... */ },
 ///     ResourceType::Ec2DhcpOptions => { /* ... */ },
 ///     ResourceType::Ec2Ec2Fleet => { /* ... */ },
-///     ResourceType::Eip => { /* ... */ },
 ///     ResourceType::Ec2EipAssociation => { /* ... */ },
-///     ResourceType::EgressOnlyInternetGateway => { /* ... */ },
-///     ResourceType::FlowLog => { /* ... */ },
-///     ResourceType::Host => { /* ... */ },
 ///     ResourceType::Ec2Ipam => { /* ... */ },
 ///     ResourceType::Ec2IpamPool => { /* ... */ },
 ///     ResourceType::Ec2IpamPoolCidr => { /* ... */ },
@@ -165,25 +167,14 @@
 ///     ResourceType::Ec2IpamResourceDiscovery => { /* ... */ },
 ///     ResourceType::Ec2IpamResourceDiscoveryAssociation => { /* ... */ },
 ///     ResourceType::Ec2IpamScope => { /* ... */ },
-///     ResourceType::Instance => { /* ... */ },
 ///     ResourceType::Ec2InstanceConnectEndpoint => { /* ... */ },
-///     ResourceType::InternetGateway => { /* ... */ },
-///     ResourceType::LaunchTemplate => { /* ... */ },
-///     ResourceType::NatGateway => { /* ... */ },
-///     ResourceType::NetworkAcl => { /* ... */ },
 ///     ResourceType::Ec2NetworkInsightsAccessScope => { /* ... */ },
-///     ResourceType::NetworkInsightsAccessScopeAnalysis => { /* ... */ },
 ///     ResourceType::Ec2NetworkInsightsAnalysis => { /* ... */ },
 ///     ResourceType::Ec2NetworkInsightsPath => { /* ... */ },
-///     ResourceType::NetworkInterface => { /* ... */ },
 ///     ResourceType::Ec2PrefixList => { /* ... */ },
-///     ResourceType::RegisteredHaInstance => { /* ... */ },
-///     ResourceType::RouteTable => { /* ... */ },
-///     ResourceType::SecurityGroup => { /* ... */ },
 ///     ResourceType::Ec2SecurityGroupVpcAssociation => { /* ... */ },
 ///     ResourceType::Ec2SnapshotBlockPublicAccess => { /* ... */ },
 ///     ResourceType::Ec2SpotFleet => { /* ... */ },
-///     ResourceType::Subnet => { /* ... */ },
 ///     ResourceType::Ec2SubnetCidrBlock => { /* ... */ },
 ///     ResourceType::Ec2SubnetNetworkAclAssociation => { /* ... */ },
 ///     ResourceType::Ec2SubnetRouteTableAssociation => { /* ... */ },
@@ -190,24 +181,14 @@
 ///     ResourceType::Ec2TrafficMirrorFilter => { /* ... */ },
 ///     ResourceType::Ec2TrafficMirrorSession => { /* ... */ },
 ///     ResourceType::Ec2TrafficMirrorTarget => { /* ... */ },
-///     ResourceType::TransitGateway => { /* ... */ },
-///     ResourceType::TransitGatewayAttachment => { /* ... */ },
 ///     ResourceType::Ec2TransitGatewayConnect => { /* ... */ },
 ///     ResourceType::Ec2TransitGatewayMulticastDomain => { /* ... */ },
-///     ResourceType::TransitGatewayRouteTable => { /* ... */ },
-///     ResourceType::Vpc => { /* ... */ },
 ///     ResourceType::Ec2VpcBlockPublicAccessExclusion => { /* ... */ },
 ///     ResourceType::Ec2VpcBlockPublicAccessOptions => { /* ... */ },
-///     ResourceType::VpcEndpoint => { /* ... */ },
 ///     ResourceType::Ec2VpcEndpointConnectionNotification => { /* ... */ },
-///     ResourceType::VpcEndpointService => { /* ... */ },
 ///     ResourceType::Ec2VpcGatewayAttachment => { /* ... */ },
-///     ResourceType::VpcPeeringConnection => { /* ... */ },
-///     ResourceType::VpnConnection => { /* ... */ },
 ///     ResourceType::Ec2VpnConnectionRoute => { /* ... */ },
-///     ResourceType::VpnGateway => { /* ... */ },
 ///     ResourceType::Ec2VerifiedAccessInstance => { /* ... */ },
-///     ResourceType::Volume => { /* ... */ },
 ///     ResourceType::EcrPublicRepository => { /* ... */ },
 ///     ResourceType::EcrPullThroughCacheRule => { /* ... */ },
 ///     ResourceType::EcrRegistryPolicy => { /* ... */ },
@@ -221,29 +202,27 @@
 ///     ResourceType::EcsTaskSet => { /* ... */ },
 ///     ResourceType::EfsAccessPoint => { /* ... */ },
 ///     ResourceType::EfsFileSystem => { /* ... */ },
+///     ResourceType::Eip => { /* ... */ },
 ///     ResourceType::EksAddon => { /* ... */ },
 ///     ResourceType::EksCluster => { /* ... */ },
 ///     ResourceType::EksFargateProfile => { /* ... */ },
 ///     ResourceType::EksIdentityProviderConfig => { /* ... */ },
+///     ResourceType::EmrContainersVirtualCluster => { /* ... */ },
 ///     ResourceType::EmrSecurityConfiguration => { /* ... */ },
+///     ResourceType::EmrServerlessApplication => { /* ... */ },
 ///     ResourceType::EmrStudio => { /* ... */ },
-///     ResourceType::EmrContainersVirtualCluster => { /* ... */ },
-///     ResourceType::EmrServerlessApplication => { /* ... */ },
-///     ResourceType::Application => { /* ... */ },
-///     ResourceType::ApplicationVersion => { /* ... */ },
-///     ResourceType::Environment => { /* ... */ },
-///     ResourceType::LoadBalancer => { /* ... */ },
-///     ResourceType::ListenerV2 => { /* ... */ },
-///     ResourceType::LoadBalancerV2 => { /* ... */ },
+///     ResourceType::EgressOnlyInternetGateway => { /* ... */ },
 ///     ResourceType::ElasticLoadBalancingV2TargetGroup => { /* ... */ },
-///     ResourceType::Domain => { /* ... */ },
+///     ResourceType::EncryptionConfig => { /* ... */ },
 ///     ResourceType::EntityResolutionIdMappingWorkflow => { /* ... */ },
 ///     ResourceType::EntityResolutionMatchingWorkflow => { /* ... */ },
 ///     ResourceType::EntityResolutionSchemaMapping => { /* ... */ },
+///     ResourceType::Environment => { /* ... */ },
 ///     ResourceType::EventSchemasDiscoverer => { /* ... */ },
 ///     ResourceType::EventSchemasRegistry => { /* ... */ },
 ///     ResourceType::EventSchemasRegistryPolicy => { /* ... */ },
 ///     ResourceType::EventSchemasSchema => { /* ... */ },
+///     ResourceType::EventSubscription => { /* ... */ },
 ///     ResourceType::EventsApiDestination => { /* ... */ },
 ///     ResourceType::EventsArchive => { /* ... */ },
 ///     ResourceType::EventsConnection => { /* ... */ },
@@ -254,6 +233,8 @@
 ///     ResourceType::EvidentlyProject => { /* ... */ },
 ///     ResourceType::EvidentlySegment => { /* ... */ },
 ///     ResourceType::FisExperimentTemplate => { /* ... */ },
+///     ResourceType::FileData => { /* ... */ },
+///     ResourceType::FlowLog => { /* ... */ },
 ///     ResourceType::ForecastDataset => { /* ... */ },
 ///     ResourceType::ForecastDatasetGroup => { /* ... */ },
 ///     ResourceType::FraudDetectorEntityType => { /* ... */ },
@@ -260,6 +241,7 @@
 ///     ResourceType::FraudDetectorLabel => { /* ... */ },
 ///     ResourceType::FraudDetectorOutcome => { /* ... */ },
 ///     ResourceType::FraudDetectorVariable => { /* ... */ },
+///     ResourceType::Function => { /* ... */ },
 ///     ResourceType::GameLiftBuild => { /* ... */ },
 ///     ResourceType::GlobalAcceleratorAccelerator => { /* ... */ },
 ///     ResourceType::GlobalAcceleratorEndpointGroup => { /* ... */ },
@@ -273,6 +255,7 @@
 ///     ResourceType::GroundStationConfig => { /* ... */ },
 ///     ResourceType::GroundStationDataflowEndpointGroup => { /* ... */ },
 ///     ResourceType::GroundStationMissionProfile => { /* ... */ },
+///     ResourceType::Group => { /* ... */ },
 ///     ResourceType::GuardDutyDetector => { /* ... */ },
 ///     ResourceType::GuardDutyFilter => { /* ... */ },
 ///     ResourceType::GuardDutyIpSet => { /* ... */ },
@@ -279,14 +262,12 @@
 ///     ResourceType::GuardDutyMalwareProtectionPlan => { /* ... */ },
 ///     ResourceType::GuardDutyThreatIntelSet => { /* ... */ },
 ///     ResourceType::HealthLakeFhirDatastore => { /* ... */ },
-///     ResourceType::Group => { /* ... */ },
+///     ResourceType::Host => { /* ... */ },
 ///     ResourceType::IamInstanceProfile => { /* ... */ },
 ///     ResourceType::IamoidcProvider => { /* ... */ },
-///     ResourceType::Policy => { /* ... */ },
-///     ResourceType::Role => { /* ... */ },
 ///     ResourceType::IamsamlProvider => { /* ... */ },
 ///     ResourceType::IamServerCertificate => { /* ... */ },
-///     ResourceType::User => { /* ... */ },
+///     ResourceType::IpSetV2 => { /* ... */ },
 ///     ResourceType::IvsChannel => { /* ... */ },
 ///     ResourceType::IvsPlaybackKeyPair => { /* ... */ },
 ///     ResourceType::IvsRecordingConfiguration => { /* ... */ },
@@ -298,12 +279,22 @@
 ///     ResourceType::ImageBuilderLifecyclePolicy => { /* ... */ },
 ///     ResourceType::InspectorV2Activation => { /* ... */ },
 ///     ResourceType::InspectorV2Filter => { /* ... */ },
+///     ResourceType::Instance => { /* ... */ },
+///     ResourceType::InternetGateway => { /* ... */ },
 ///     ResourceType::IoTAccountAuditConfiguration => { /* ... */ },
+///     ResourceType::IoTAnalyticsChannel => { /* ... */ },
+///     ResourceType::IoTAnalyticsDataset => { /* ... */ },
+///     ResourceType::IoTAnalyticsDatastore => { /* ... */ },
+///     ResourceType::IoTAnalyticsPipeline => { /* ... */ },
 ///     ResourceType::IoTAuthorizer => { /* ... */ },
 ///     ResourceType::IoTcaCertificate => { /* ... */ },
+///     ResourceType::IoTCoreDeviceAdvisorSuiteDefinition => { /* ... */ },
 ///     ResourceType::IoTCustomMetric => { /* ... */ },
 ///     ResourceType::IoTDimension => { /* ... */ },
 ///     ResourceType::IoTDomainConfiguration => { /* ... */ },
+///     ResourceType::IoTEventsAlarmModel => { /* ... */ },
+///     ResourceType::IoTEventsDetectorModel => { /* ... */ },
+///     ResourceType::IoTEventsInput => { /* ... */ },
 ///     ResourceType::IoTFleetMetric => { /* ... */ },
 ///     ResourceType::IoTJobTemplate => { /* ... */ },
 ///     ResourceType::IoTMitigationAction => { /* ... */ },
@@ -312,15 +303,6 @@
 ///     ResourceType::IoTRoleAlias => { /* ... */ },
 ///     ResourceType::IoTScheduledAudit => { /* ... */ },
 ///     ResourceType::IoTSecurityProfile => { /* ... */ },
-///     ResourceType::IoTThingGroup => { /* ... */ },
-///     ResourceType::IoTAnalyticsChannel => { /* ... */ },
-///     ResourceType::IoTAnalyticsDataset => { /* ... */ },
-///     ResourceType::IoTAnalyticsDatastore => { /* ... */ },
-///     ResourceType::IoTAnalyticsPipeline => { /* ... */ },
-///     ResourceType::IoTCoreDeviceAdvisorSuiteDefinition => { /* ... */ },
-///     ResourceType::IoTEventsAlarmModel => { /* ... */ },
-///     ResourceType::IoTEventsDetectorModel => { /* ... */ },
-///     ResourceType::IoTEventsInput => { /* ... */ },
 ///     ResourceType::IoTSiteWiseAsset => { /* ... */ },
 ///     ResourceType::IoTSiteWiseAssetModel => { /* ... */ },
 ///     ResourceType::IoTSiteWiseDashboard => { /* ... */ },
@@ -327,6 +309,7 @@
 ///     ResourceType::IoTSiteWiseGateway => { /* ... */ },
 ///     ResourceType::IoTSiteWisePortal => { /* ... */ },
 ///     ResourceType::IoTSiteWiseProject => { /* ... */ },
+///     ResourceType::IoTThingGroup => { /* ... */ },
 ///     ResourceType::IoTTwinMakerComponentType => { /* ... */ },
 ///     ResourceType::IoTTwinMakerEntity => { /* ... */ },
 ///     ResourceType::IoTTwinMakerScene => { /* ... */ },
@@ -336,18 +319,19 @@
 ///     ResourceType::IoTWirelessMulticastGroup => { /* ... */ },
 ///     ResourceType::IoTWirelessServiceProfile => { /* ... */ },
 ///     ResourceType::KmsAlias => { /* ... */ },
-///     ResourceType::Key => { /* ... */ },
 ///     ResourceType::KafkaConnectConnector => { /* ... */ },
 ///     ResourceType::KafkaConnectCustomPlugin => { /* ... */ },
 ///     ResourceType::KendraIndex => { /* ... */ },
+///     ResourceType::Key => { /* ... */ },
+///     ResourceType::KinesisAnalyticsV2Application => { /* ... */ },
+///     ResourceType::KinesisFirehoseDeliveryStream => { /* ... */ },
 ///     ResourceType::KinesisStream => { /* ... */ },
 ///     ResourceType::KinesisStreamConsumer => { /* ... */ },
-///     ResourceType::KinesisAnalyticsV2Application => { /* ... */ },
-///     ResourceType::KinesisFirehoseDeliveryStream => { /* ... */ },
 ///     ResourceType::KinesisVideoSignalingChannel => { /* ... */ },
 ///     ResourceType::KinesisVideoStream => { /* ... */ },
 ///     ResourceType::LambdaCodeSigningConfig => { /* ... */ },
-///     ResourceType::Function => { /* ... */ },
+///     ResourceType::LaunchConfiguration => { /* ... */ },
+///     ResourceType::LaunchTemplate => { /* ... */ },
 ///     ResourceType::LexBot => { /* ... */ },
 ///     ResourceType::LexBotAlias => { /* ... */ },
 ///     ResourceType::LightsailBucket => { /* ... */ },
@@ -354,6 +338,9 @@
 ///     ResourceType::LightsailCertificate => { /* ... */ },
 ///     ResourceType::LightsailDisk => { /* ... */ },
 ///     ResourceType::LightsailStaticIp => { /* ... */ },
+///     ResourceType::ListenerV2 => { /* ... */ },
+///     ResourceType::LoadBalancer => { /* ... */ },
+///     ResourceType::LoadBalancerV2 => { /* ... */ },
 ///     ResourceType::LocationApiKey => { /* ... */ },
 ///     ResourceType::LogsDestination => { /* ... */ },
 ///     ResourceType::LookoutMetricsAlert => { /* ... */ },
@@ -365,6 +352,8 @@
 ///     ResourceType::MskConfiguration => { /* ... */ },
 ///     ResourceType::MskServerlessCluster => { /* ... */ },
 ///     ResourceType::MskVpcConnection => { /* ... */ },
+///     ResourceType::ManagedInstanceInventory => { /* ... */ },
+///     ResourceType::ManagedRuleSetV2 => { /* ... */ },
 ///     ResourceType::MediaConnectFlowEntitlement => { /* ... */ },
 ///     ResourceType::MediaConnectFlowSource => { /* ... */ },
 ///     ResourceType::MediaConnectFlowVpcInterface => { /* ... */ },
@@ -376,9 +365,13 @@
 ///     ResourceType::MediaTailorLiveSource => { /* ... */ },
 ///     ResourceType::MediaTailorPlaybackConfiguration => { /* ... */ },
 ///     ResourceType::MemoryDbSubnetGroup => { /* ... */ },
+///     ResourceType::NatGateway => { /* ... */ },
+///     ResourceType::NetworkAcl => { /* ... */ },
 ///     ResourceType::NetworkFirewallFirewall => { /* ... */ },
 ///     ResourceType::NetworkFirewallFirewallPolicy => { /* ... */ },
 ///     ResourceType::NetworkFirewallRuleGroup => { /* ... */ },
+///     ResourceType::NetworkInsightsAccessScopeAnalysis => { /* ... */ },
+///     ResourceType::NetworkInterface => { /* ... */ },
 ///     ResourceType::NetworkManagerConnectPeer => { /* ... */ },
 ///     ResourceType::NetworkManagerCustomerGatewayAssociation => { /* ... */ },
 ///     ResourceType::NetworkManagerDevice => { /* ... */ },
@@ -396,6 +389,7 @@
 ///     ResourceType::PcaConnectorAdConnector => { /* ... */ },
 ///     ResourceType::PcaConnectorAdDirectoryRegistration => { /* ... */ },
 ///     ResourceType::PanoramaPackage => { /* ... */ },
+///     ResourceType::PatchCompliance => { /* ... */ },
 ///     ResourceType::PersonalizeDataset => { /* ... */ },
 ///     ResourceType::PersonalizeDatasetGroup => { /* ... */ },
 ///     ResourceType::PersonalizeSchema => { /* ... */ },
@@ -408,37 +402,42 @@
 ///     ResourceType::PinpointEventStream => { /* ... */ },
 ///     ResourceType::PinpointInAppTemplate => { /* ... */ },
 ///     ResourceType::PinpointSegment => { /* ... */ },
+///     ResourceType::Pipeline => { /* ... */ },
+///     ResourceType::Policy => { /* ... */ },
+///     ResourceType::Portfolio => { /* ... */ },
+///     ResourceType::Project => { /* ... */ },
+///     ResourceType::Protection => { /* ... */ },
 ///     ResourceType::QldbLedger => { /* ... */ },
+///     ResourceType::Queue => { /* ... */ },
 ///     ResourceType::QuickSightDataSource => { /* ... */ },
 ///     ResourceType::QuickSightTemplate => { /* ... */ },
 ///     ResourceType::QuickSightTheme => { /* ... */ },
-///     ResourceType::DbCluster => { /* ... */ },
-///     ResourceType::DbClusterSnapshot => { /* ... */ },
-///     ResourceType::DbInstance => { /* ... */ },
-///     ResourceType::DbSecurityGroup => { /* ... */ },
-///     ResourceType::DbSnapshot => { /* ... */ },
-///     ResourceType::DbSubnetGroup => { /* ... */ },
-///     ResourceType::EventSubscription => { /* ... */ },
 ///     ResourceType::RdsGlobalCluster => { /* ... */ },
 ///     ResourceType::RdsIntegration => { /* ... */ },
 ///     ResourceType::RdsOptionGroup => { /* ... */ },
 ///     ResourceType::RumAppMonitor => { /* ... */ },
-///     ResourceType::Cluster => { /* ... */ },
-///     ResourceType::ClusterParameterGroup => { /* ... */ },
-///     ResourceType::ClusterSecurityGroup => { /* ... */ },
-///     ResourceType::ClusterSnapshot => { /* ... */ },
-///     ResourceType::ClusterSubnetGroup => { /* ... */ },
+///     ResourceType::RateBasedRule => { /* ... */ },
 ///     ResourceType::RedshiftEndpointAccess => { /* ... */ },
 ///     ResourceType::RedshiftEndpointAuthorization => { /* ... */ },
 ///     ResourceType::RedshiftEventSubscription => { /* ... */ },
 ///     ResourceType::RedshiftIntegration => { /* ... */ },
 ///     ResourceType::RedshiftScheduledAction => { /* ... */ },
+///     ResourceType::RegexPatternSetV2 => { /* ... */ },
+///     ResourceType::RegionalProtection => { /* ... */ },
+///     ResourceType::RegionalRateBasedRule => { /* ... */ },
+///     ResourceType::RegionalRule => { /* ... */ },
+///     ResourceType::RegionalRuleGroup => { /* ... */ },
+///     ResourceType::RegionalWebAcl => { /* ... */ },
+///     ResourceType::RegisteredHaInstance => { /* ... */ },
 ///     ResourceType::ResilienceHubApp => { /* ... */ },
 ///     ResourceType::ResilienceHubResiliencyPolicy => { /* ... */ },
+///     ResourceType::ResourceCompliance => { /* ... */ },
 ///     ResourceType::ResourceExplorer2Index => { /* ... */ },
+///     ResourceType::RestApi => { /* ... */ },
 ///     ResourceType::RoboMakerRobotApplication => { /* ... */ },
 ///     ResourceType::RoboMakerRobotApplicationVersion => { /* ... */ },
 ///     ResourceType::RoboMakerSimulationApplication => { /* ... */ },
+///     ResourceType::Role => { /* ... */ },
 ///     ResourceType::RolesAnywhereProfile => { /* ... */ },
 ///     ResourceType::RolesAnywhereTrustAnchor => { /* ... */ },
 ///     ResourceType::Route53Dnssec => { /* ... */ },
@@ -461,17 +460,19 @@
 ///     ResourceType::Route53ResolverResolverQueryLoggingConfigAssociation => { /* ... */ },
 ///     ResourceType::Route53ResolverResolverRule => { /* ... */ },
 ///     ResourceType::Route53ResolverResolverRuleAssociation => { /* ... */ },
+///     ResourceType::RouteTable => { /* ... */ },
+///     ResourceType::Rule => { /* ... */ },
+///     ResourceType::RuleGroup => { /* ... */ },
+///     ResourceType::RuleGroupV2 => { /* ... */ },
 ///     ResourceType::S3AccessGrant => { /* ... */ },
 ///     ResourceType::S3AccessGrantsInstance => { /* ... */ },
 ///     ResourceType::S3AccessGrantsLocation => { /* ... */ },
 ///     ResourceType::S3AccessPoint => { /* ... */ },
-///     ResourceType::AccountPublicAccessBlock => { /* ... */ },
-///     ResourceType::Bucket => { /* ... */ },
+///     ResourceType::S3ExpressBucketPolicy => { /* ... */ },
+///     ResourceType::S3ExpressDirectoryBucket => { /* ... */ },
 ///     ResourceType::S3MultiRegionAccessPoint => { /* ... */ },
 ///     ResourceType::S3StorageLens => { /* ... */ },
 ///     ResourceType::S3StorageLensGroup => { /* ... */ },
-///     ResourceType::S3ExpressBucketPolicy => { /* ... */ },
-///     ResourceType::S3ExpressDirectoryBucket => { /* ... */ },
 ///     ResourceType::S3TablesTableBucket => { /* ... */ },
 ///     ResourceType::S3TablesTableBucketPolicy => { /* ... */ },
 ///     ResourceType::SesConfigurationSet => { /* ... */ },
@@ -481,16 +482,10 @@
 ///     ResourceType::SesReceiptFilter => { /* ... */ },
 ///     ResourceType::SesReceiptRuleSet => { /* ... */ },
 ///     ResourceType::SesTemplate => { /* ... */ },
-///     ResourceType::Topic => { /* ... */ },
-///     ResourceType::Queue => { /* ... */ },
-///     ResourceType::AssociationCompliance => { /* ... */ },
+///     ResourceType::SsmContactsContact => { /* ... */ },
 ///     ResourceType::SsmDocument => { /* ... */ },
-///     ResourceType::FileData => { /* ... */ },
-///     ResourceType::ManagedInstanceInventory => { /* ... */ },
-///     ResourceType::PatchCompliance => { /* ... */ },
+///     ResourceType::SsmIncidentsResponsePlan => { /* ... */ },
 ///     ResourceType::SsmResourceDataSync => { /* ... */ },
-///     ResourceType::SsmContactsContact => { /* ... */ },
-///     ResourceType::SsmIncidentsResponsePlan => { /* ... */ },
 ///     ResourceType::SageMakerAppImageConfig => { /* ... */ },
 ///     ResourceType::SageMakerCodeRepository => { /* ... */ },
 ///     ResourceType::SageMakerDataQualityJobDefinition => { /* ... */ },
@@ -507,22 +502,28 @@
 ///     ResourceType::SageMakerStudioLifecycleConfig => { /* ... */ },
 ///     ResourceType::SageMakerUserProfile => { /* ... */ },
 ///     ResourceType::SageMakerWorkteam => { /* ... */ },
+///     ResourceType::ScalingPolicy => { /* ... */ },
+///     ResourceType::ScheduledAction => { /* ... */ },
+///     ResourceType::Secret => { /* ... */ },
 ///     ResourceType::SecretsManagerResourcePolicy => { /* ... */ },
 ///     ResourceType::SecretsManagerRotationSchedule => { /* ... */ },
-///     ResourceType::Secret => { /* ... */ },
+///     ResourceType::SecurityGroup => { /* ... */ },
 ///     ResourceType::SecurityHubStandard => { /* ... */ },
-///     ResourceType::CloudFormationProduct => { /* ... */ },
-///     ResourceType::CloudFormationProvisionedProduct => { /* ... */ },
-///     ResourceType::Portfolio => { /* ... */ },
 ///     ResourceType::ServiceDiscoveryHttpNamespace => { /* ... */ },
 ///     ResourceType::ServiceDiscoveryInstance => { /* ... */ },
 ///     ResourceType::ServiceDiscoveryPublicDnsNamespace => { /* ... */ },
 ///     ResourceType::ServiceDiscoveryService => { /* ... */ },
-///     ResourceType::Protection => { /* ... */ },
-///     ResourceType::RegionalProtection => { /* ... */ },
 ///     ResourceType::SignerSigningProfile => { /* ... */ },
+///     ResourceType::Stack => { /* ... */ },
+///     ResourceType::Stage => { /* ... */ },
+///     ResourceType::StageV2 => { /* ... */ },
 ///     ResourceType::StepFunctionsActivity => { /* ... */ },
 ///     ResourceType::StepFunctionsStateMachine => { /* ... */ },
+///     ResourceType::StreamingDistribution => { /* ... */ },
+///     ResourceType::Subnet => { /* ... */ },
+///     ResourceType::Table => { /* ... */ },
+///     ResourceType::Topic => { /* ... */ },
+///     ResourceType::Trail => { /* ... */ },
 ///     ResourceType::TransferAgreement => { /* ... */ },
 ///     ResourceType::TransferCertificate => { /* ... */ },
 ///     ResourceType::TransferConnector => { /* ... */ },
@@ -529,22 +530,21 @@
 ///     ResourceType::TransferProfile => { /* ... */ },
 ///     ResourceType::TransferServer => { /* ... */ },
 ///     ResourceType::TransferWorkflow => { /* ... */ },
-///     ResourceType::RateBasedRule => { /* ... */ },
-///     ResourceType::Rule => { /* ... */ },
-///     ResourceType::RuleGroup => { /* ... */ },
+///     ResourceType::TransitGateway => { /* ... */ },
+///     ResourceType::TransitGatewayAttachment => { /* ... */ },
+///     ResourceType::TransitGatewayRouteTable => { /* ... */ },
+///     ResourceType::User => { /* ... */ },
+///     ResourceType::Vpc => { /* ... */ },
+///     ResourceType::VpcEndpoint => { /* ... */ },
+///     ResourceType::VpcEndpointService => { /* ... */ },
+///     ResourceType::VpcPeeringConnection => { /* ... */ },
+///     ResourceType::VpnConnection => { /* ... */ },
+///     ResourceType::VpnGateway => { /* ... */ },
+///     ResourceType::Volume => { /* ... */ },
 ///     ResourceType::WebAcl => { /* ... */ },
-///     ResourceType::RegionalRateBasedRule => { /* ... */ },
-///     ResourceType::RegionalRule => { /* ... */ },
-///     ResourceType::RegionalRuleGroup => { /* ... */ },
-///     ResourceType::RegionalWebAcl => { /* ... */ },
-///     ResourceType::IpSetV2 => { /* ... */ },
-///     ResourceType::ManagedRuleSetV2 => { /* ... */ },
-///     ResourceType::RegexPatternSetV2 => { /* ... */ },
-///     ResourceType::RuleGroupV2 => { /* ... */ },
 ///     ResourceType::WebAclv2 => { /* ... */ },
 ///     ResourceType::WorkSpacesConnectionAlias => { /* ... */ },
 ///     ResourceType::WorkSpacesWorkspace => { /* ... */ },
-///     ResourceType::EncryptionConfig => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -574,8 +574,6 @@
 )]
 pub enum ResourceType {
     #[allow(missing_docs)] // documentation missing in model
-    Certificate,
-    #[allow(missing_docs)] // documentation missing in model
     AcmpcaCertificateAuthority,
     #[allow(missing_docs)] // documentation missing in model
     AcmpcaCertificateAuthorityActivation,
@@ -584,6 +582,10 @@
     #[allow(missing_docs)] // documentation missing in model
     AccessAnalyzerAnalyzer,
     #[allow(missing_docs)] // documentation missing in model
+    AccountPublicAccessBlock,
+    #[allow(missing_docs)] // documentation missing in model
+    Alarm,
+    #[allow(missing_docs)] // documentation missing in model
     AmazonMqBroker,
     #[allow(missing_docs)] // documentation missing in model
     AmplifyApp,
@@ -590,20 +592,14 @@
     #[allow(missing_docs)] // documentation missing in model
     AmplifyBranch,
     #[allow(missing_docs)] // documentation missing in model
+    Api,
+    #[allow(missing_docs)] // documentation missing in model
     ApiGatewayMethod,
     #[allow(missing_docs)] // documentation missing in model
-    RestApi,
-    #[allow(missing_docs)] // documentation missing in model
-    Stage,
-    #[allow(missing_docs)] // documentation missing in model
     ApiGatewayUsagePlan,
     #[allow(missing_docs)] // documentation missing in model
-    Api,
-    #[allow(missing_docs)] // documentation missing in model
     ApiGatewayV2Integration,
     #[allow(missing_docs)] // documentation missing in model
-    StageV2,
-    #[allow(missing_docs)] // documentation missing in model
     AppConfigApplication,
     #[allow(missing_docs)] // documentation missing in model
     AppConfigConfigurationProfile,
@@ -656,6 +652,12 @@
     #[allow(missing_docs)] // documentation missing in model
     AppSyncGraphQlApi,
     #[allow(missing_docs)] // documentation missing in model
+    Application,
+    #[allow(missing_docs)] // documentation missing in model
+    ApplicationVersion,
+    #[allow(missing_docs)] // documentation missing in model
+    AssociationCompliance,
+    #[allow(missing_docs)] // documentation missing in model
     AthenaDataCatalog,
     #[allow(missing_docs)] // documentation missing in model
     AthenaPreparedStatement,
@@ -666,12 +668,6 @@
     #[allow(missing_docs)] // documentation missing in model
     AutoScalingGroup,
     #[allow(missing_docs)] // documentation missing in model
-    LaunchConfiguration,
-    #[allow(missing_docs)] // documentation missing in model
-    ScalingPolicy,
-    #[allow(missing_docs)] // documentation missing in model
-    ScheduledAction,
-    #[allow(missing_docs)] // documentation missing in model
     AutoScalingWarmPool,
     #[allow(missing_docs)] // documentation missing in model
     B2BiCapability,
@@ -678,12 +674,10 @@
     #[allow(missing_docs)] // documentation missing in model
     BcmDataExportsExport,
     #[allow(missing_docs)] // documentation missing in model
+    BackupGatewayHypervisor,
+    #[allow(missing_docs)] // documentation missing in model
     BackupPlan,
     #[allow(missing_docs)] // documentation missing in model
-    BackupSelection,
-    #[allow(missing_docs)] // documentation missing in model
-    BackupVault,
-    #[allow(missing_docs)] // documentation missing in model
     BackupRecoveryPoint,
     #[allow(missing_docs)] // documentation missing in model
     BackupReportPlan,
@@ -690,7 +684,9 @@
     #[allow(missing_docs)] // documentation missing in model
     BackupRestoreTestingPlan,
     #[allow(missing_docs)] // documentation missing in model
-    BackupGatewayHypervisor,
+    BackupSelection,
+    #[allow(missing_docs)] // documentation missing in model
+    BackupVault,
     #[allow(missing_docs)] // documentation missing in model
     BatchComputeEnvironment,
     #[allow(missing_docs)] // documentation missing in model
@@ -698,6 +694,10 @@
     #[allow(missing_docs)] // documentation missing in model
     BatchSchedulingPolicy,
     #[allow(missing_docs)] // documentation missing in model
+    BedrockAgentCoreBrowserCustom,
+    #[allow(missing_docs)] // documentation missing in model
+    BedrockAgentCoreRuntime,
+    #[allow(missing_docs)] // documentation missing in model
     BedrockApplicationInferenceProfile,
     #[allow(missing_docs)] // documentation missing in model
     BedrockGuardrail,
@@ -706,14 +706,14 @@
     #[allow(missing_docs)] // documentation missing in model
     BedrockPrompt,
     #[allow(missing_docs)] // documentation missing in model
-    BedrockAgentCoreBrowserCustom,
-    #[allow(missing_docs)] // documentation missing in model
-    BedrockAgentCoreRuntime,
+    Bucket,
     #[allow(missing_docs)] // documentation missing in model
     BudgetsBudgetsAction,
     #[allow(missing_docs)] // documentation missing in model
     CassandraKeyspace,
     #[allow(missing_docs)] // documentation missing in model
+    Certificate,
+    #[allow(missing_docs)] // documentation missing in model
     CleanRoomsMlTrainingDataset,
     #[allow(missing_docs)] // documentation missing in model
     Cloud9EnvironmentEc2,
@@ -722,12 +722,12 @@
     #[allow(missing_docs)] // documentation missing in model
     CloudFormationLambdaHook,
     #[allow(missing_docs)] // documentation missing in model
-    Stack,
+    CloudFormationProduct,
+    #[allow(missing_docs)] // documentation missing in model
+    CloudFormationProvisionedProduct,
     #[allow(missing_docs)] // documentation missing in model
     CloudFormationStackSet,
     #[allow(missing_docs)] // documentation missing in model
-    Distribution,
-    #[allow(missing_docs)] // documentation missing in model
     CloudFrontKeyValueStore,
     #[allow(missing_docs)] // documentation missing in model
     CloudFrontPublicKey,
@@ -734,22 +734,24 @@
     #[allow(missing_docs)] // documentation missing in model
     CloudFrontRealtimeLogConfig,
     #[allow(missing_docs)] // documentation missing in model
-    StreamingDistribution,
-    #[allow(missing_docs)] // documentation missing in model
     CloudTrailEventDataStore,
     #[allow(missing_docs)] // documentation missing in model
-    Trail,
+    CloudWatchMetricStream,
     #[allow(missing_docs)] // documentation missing in model
-    Alarm,
+    Cluster,
     #[allow(missing_docs)] // documentation missing in model
-    CloudWatchMetricStream,
+    ClusterParameterGroup,
+    #[allow(missing_docs)] // documentation missing in model
+    ClusterSecurityGroup,
+    #[allow(missing_docs)] // documentation missing in model
+    ClusterSnapshot,
+    #[allow(missing_docs)] // documentation missing in model
+    ClusterSubnetGroup,
     #[allow(missing_docs)] // documentation missing in model
     CodeArtifactDomain,
     #[allow(missing_docs)] // documentation missing in model
     CodeArtifactRepository,
     #[allow(missing_docs)] // documentation missing in model
-    Project,
-    #[allow(missing_docs)] // documentation missing in model
     CodeBuildReportGroup,
     #[allow(missing_docs)] // documentation missing in model
     CodeDeployApplication,
@@ -762,8 +764,6 @@
     #[allow(missing_docs)] // documentation missing in model
     CodeGuruReviewerRepositoryAssociation,
     #[allow(missing_docs)] // documentation missing in model
-    Pipeline,
-    #[allow(missing_docs)] // documentation missing in model
     CognitoIdentityPool,
     #[allow(missing_docs)] // documentation missing in model
     CognitoUserPool,
@@ -778,11 +778,9 @@
     #[allow(missing_docs)] // documentation missing in model
     ConfigConformancePack,
     #[allow(missing_docs)] // documentation missing in model
-    ConformancePackCompliance,
-    #[allow(missing_docs)] // documentation missing in model
-    ResourceCompliance,
+    ConfigStoredQuery,
     #[allow(missing_docs)] // documentation missing in model
-    ConfigStoredQuery,
+    ConformancePackCompliance,
     #[allow(missing_docs)] // documentation missing in model
     ConnectInstance,
     #[allow(missing_docs)] // documentation missing in model
@@ -796,10 +794,24 @@
     #[allow(missing_docs)] // documentation missing in model
     ConnectUser,
     #[allow(missing_docs)] // documentation missing in model
+    CustomerGateway,
+    #[allow(missing_docs)] // documentation missing in model
     CustomerProfilesDomain,
     #[allow(missing_docs)] // documentation missing in model
     CustomerProfilesObjectType,
     #[allow(missing_docs)] // documentation missing in model
+    DbCluster,
+    #[allow(missing_docs)] // documentation missing in model
+    DbClusterSnapshot,
+    #[allow(missing_docs)] // documentation missing in model
+    DbInstance,
+    #[allow(missing_docs)] // documentation missing in model
+    DbSecurityGroup,
+    #[allow(missing_docs)] // documentation missing in model
+    DbSnapshot,
+    #[allow(missing_docs)] // documentation missing in model
+    DbSubnetGroup,
+    #[allow(missing_docs)] // documentation missing in model
     DmsCertificate,
     #[allow(missing_docs)] // documentation missing in model
     DmsEndpoint,
@@ -842,7 +854,9 @@
     #[allow(missing_docs)] // documentation missing in model
     DeviceFarmTestGridProject,
     #[allow(missing_docs)] // documentation missing in model
-    Table,
+    Distribution,
+    #[allow(missing_docs)] // documentation missing in model
+    Domain,
     #[allow(missing_docs)] // documentation missing in model
     Ec2CapacityReservation,
     #[allow(missing_docs)] // documentation missing in model
@@ -852,22 +866,12 @@
     #[allow(missing_docs)] // documentation missing in model
     Ec2ClientVpnTargetNetworkAssociation,
     #[allow(missing_docs)] // documentation missing in model
-    CustomerGateway,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2DhcpOptions,
     #[allow(missing_docs)] // documentation missing in model
     Ec2Ec2Fleet,
     #[allow(missing_docs)] // documentation missing in model
-    Eip,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2EipAssociation,
     #[allow(missing_docs)] // documentation missing in model
-    EgressOnlyInternetGateway,
-    #[allow(missing_docs)] // documentation missing in model
-    FlowLog,
-    #[allow(missing_docs)] // documentation missing in model
-    Host,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2Ipam,
     #[allow(missing_docs)] // documentation missing in model
     Ec2IpamPool,
@@ -880,36 +884,16 @@
     #[allow(missing_docs)] // documentation missing in model
     Ec2IpamScope,
     #[allow(missing_docs)] // documentation missing in model
-    Instance,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2InstanceConnectEndpoint,
     #[allow(missing_docs)] // documentation missing in model
-    InternetGateway,
-    #[allow(missing_docs)] // documentation missing in model
-    LaunchTemplate,
-    #[allow(missing_docs)] // documentation missing in model
-    NatGateway,
-    #[allow(missing_docs)] // documentation missing in model
-    NetworkAcl,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2NetworkInsightsAccessScope,
     #[allow(missing_docs)] // documentation missing in model
-    NetworkInsightsAccessScopeAnalysis,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2NetworkInsightsAnalysis,
     #[allow(missing_docs)] // documentation missing in model
     Ec2NetworkInsightsPath,
     #[allow(missing_docs)] // documentation missing in model
-    NetworkInterface,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2PrefixList,
     #[allow(missing_docs)] // documentation missing in model
-    RegisteredHaInstance,
-    #[allow(missing_docs)] // documentation missing in model
-    RouteTable,
-    #[allow(missing_docs)] // documentation missing in model
-    SecurityGroup,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2SecurityGroupVpcAssociation,
     #[allow(missing_docs)] // documentation missing in model
     Ec2SnapshotBlockPublicAccess,
@@ -916,8 +900,6 @@
     #[allow(missing_docs)] // documentation missing in model
     Ec2SpotFleet,
     #[allow(missing_docs)] // documentation missing in model
-    Subnet,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2SubnetCidrBlock,
     #[allow(missing_docs)] // documentation missing in model
     Ec2SubnetNetworkAclAssociation,
@@ -930,42 +912,22 @@
     #[allow(missing_docs)] // documentation missing in model
     Ec2TrafficMirrorTarget,
     #[allow(missing_docs)] // documentation missing in model
-    TransitGateway,
-    #[allow(missing_docs)] // documentation missing in model
-    TransitGatewayAttachment,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2TransitGatewayConnect,
     #[allow(missing_docs)] // documentation missing in model
     Ec2TransitGatewayMulticastDomain,
     #[allow(missing_docs)] // documentation missing in model
-    TransitGatewayRouteTable,
-    #[allow(missing_docs)] // documentation missing in model
-    Vpc,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2VpcBlockPublicAccessExclusion,
     #[allow(missing_docs)] // documentation missing in model
     Ec2VpcBlockPublicAccessOptions,
     #[allow(missing_docs)] // documentation missing in model
-    VpcEndpoint,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2VpcEndpointConnectionNotification,
     #[allow(missing_docs)] // documentation missing in model
-    VpcEndpointService,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2VpcGatewayAttachment,
     #[allow(missing_docs)] // documentation missing in model
-    VpcPeeringConnection,
-    #[allow(missing_docs)] // documentation missing in model
-    VpnConnection,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2VpnConnectionRoute,
     #[allow(missing_docs)] // documentation missing in model
-    VpnGateway,
-    #[allow(missing_docs)] // documentation missing in model
     Ec2VerifiedAccessInstance,
     #[allow(missing_docs)] // documentation missing in model
-    Volume,
-    #[allow(missing_docs)] // documentation missing in model
     EcrPublicRepository,
     #[allow(missing_docs)] // documentation missing in model
     EcrPullThroughCacheRule,
@@ -992,6 +954,8 @@
     #[allow(missing_docs)] // documentation missing in model
     EfsFileSystem,
     #[allow(missing_docs)] // documentation missing in model
+    Eip,
+    #[allow(missing_docs)] // documentation missing in model
     EksAddon,
     #[allow(missing_docs)] // documentation missing in model
     EksCluster,
@@ -1000,29 +964,19 @@
     #[allow(missing_docs)] // documentation missing in model
     EksIdentityProviderConfig,
     #[allow(missing_docs)] // documentation missing in model
-    EmrSecurityConfiguration,
-    #[allow(missing_docs)] // documentation missing in model
-    EmrStudio,
-    #[allow(missing_docs)] // documentation missing in model
     EmrContainersVirtualCluster,
     #[allow(missing_docs)] // documentation missing in model
-    EmrServerlessApplication,
-    #[allow(missing_docs)] // documentation missing in model
-    Application,
-    #[allow(missing_docs)] // documentation missing in model
-    ApplicationVersion,
-    #[allow(missing_docs)] // documentation missing in model
-    Environment,
+    EmrSecurityConfiguration,
     #[allow(missing_docs)] // documentation missing in model
-    LoadBalancer,
+    EmrServerlessApplication,
     #[allow(missing_docs)] // documentation missing in model
-    ListenerV2,
+    EmrStudio,
     #[allow(missing_docs)] // documentation missing in model
-    LoadBalancerV2,
+    EgressOnlyInternetGateway,
     #[allow(missing_docs)] // documentation missing in model
     ElasticLoadBalancingV2TargetGroup,
     #[allow(missing_docs)] // documentation missing in model
-    Domain,
+    EncryptionConfig,
     #[allow(missing_docs)] // documentation missing in model
     EntityResolutionIdMappingWorkflow,
     #[allow(missing_docs)] // documentation missing in model
@@ -1030,6 +984,8 @@
     #[allow(missing_docs)] // documentation missing in model
     EntityResolutionSchemaMapping,
     #[allow(missing_docs)] // documentation missing in model
+    Environment,
+    #[allow(missing_docs)] // documentation missing in model
     EventSchemasDiscoverer,
     #[allow(missing_docs)] // documentation missing in model
     EventSchemasRegistry,
@@ -1038,6 +994,8 @@
     #[allow(missing_docs)] // documentation missing in model
     EventSchemasSchema,
     #[allow(missing_docs)] // documentation missing in model
+    EventSubscription,
+    #[allow(missing_docs)] // documentation missing in model
     EventsApiDestination,
     #[allow(missing_docs)] // documentation missing in model
     EventsArchive,
@@ -1058,6 +1016,10 @@
     #[allow(missing_docs)] // documentation missing in model
     FisExperimentTemplate,
     #[allow(missing_docs)] // documentation missing in model
+    FileData,
+    #[allow(missing_docs)] // documentation missing in model
+    FlowLog,
+    #[allow(missing_docs)] // documentation missing in model
     ForecastDataset,
     #[allow(missing_docs)] // documentation missing in model
     ForecastDatasetGroup,
@@ -1070,6 +1032,8 @@
     #[allow(missing_docs)] // documentation missing in model
     FraudDetectorVariable,
     #[allow(missing_docs)] // documentation missing in model
+    Function,
+    #[allow(missing_docs)] // documentation missing in model
     GameLiftBuild,
     #[allow(missing_docs)] // documentation missing in model
     GlobalAcceleratorAccelerator,
@@ -1096,6 +1060,8 @@
     #[allow(missing_docs)] // documentation missing in model
     GroundStationMissionProfile,
     #[allow(missing_docs)] // documentation missing in model
+    Group,
+    #[allow(missing_docs)] // documentation missing in model
     GuardDutyDetector,
     #[allow(missing_docs)] // documentation missing in model
     GuardDutyFilter,
@@ -1108,21 +1074,17 @@
     #[allow(missing_docs)] // documentation missing in model
     HealthLakeFhirDatastore,
     #[allow(missing_docs)] // documentation missing in model
-    Group,
+    Host,
     #[allow(missing_docs)] // documentation missing in model
     IamInstanceProfile,
     #[allow(missing_docs)] // documentation missing in model
     IamoidcProvider,
     #[allow(missing_docs)] // documentation missing in model
-    Policy,
-    #[allow(missing_docs)] // documentation missing in model
-    Role,
-    #[allow(missing_docs)] // documentation missing in model
     IamsamlProvider,
     #[allow(missing_docs)] // documentation missing in model
     IamServerCertificate,
     #[allow(missing_docs)] // documentation missing in model
-    User,
+    IpSetV2,
     #[allow(missing_docs)] // documentation missing in model
     IvsChannel,
     #[allow(missing_docs)] // documentation missing in model
@@ -1146,12 +1108,26 @@
     #[allow(missing_docs)] // documentation missing in model
     InspectorV2Filter,
     #[allow(missing_docs)] // documentation missing in model
+    Instance,
+    #[allow(missing_docs)] // documentation missing in model
+    InternetGateway,
+    #[allow(missing_docs)] // documentation missing in model
     IoTAccountAuditConfiguration,
     #[allow(missing_docs)] // documentation missing in model
+    IoTAnalyticsChannel,
+    #[allow(missing_docs)] // documentation missing in model
+    IoTAnalyticsDataset,
+    #[allow(missing_docs)] // documentation missing in model
+    IoTAnalyticsDatastore,
+    #[allow(missing_docs)] // documentation missing in model
+    IoTAnalyticsPipeline,
+    #[allow(missing_docs)] // documentation missing in model
     IoTAuthorizer,
     #[allow(missing_docs)] // documentation missing in model
     IoTcaCertificate,
     #[allow(missing_docs)] // documentation missing in model
+    IoTCoreDeviceAdvisorSuiteDefinition,
+    #[allow(missing_docs)] // documentation missing in model
     IoTCustomMetric,
     #[allow(missing_docs)] // documentation missing in model
     IoTDimension,
@@ -1158,6 +1134,12 @@
     #[allow(missing_docs)] // documentation missing in model
     IoTDomainConfiguration,
     #[allow(missing_docs)] // documentation missing in model
+    IoTEventsAlarmModel,
+    #[allow(missing_docs)] // documentation missing in model
+    IoTEventsDetectorModel,
+    #[allow(missing_docs)] // documentation missing in model
+    IoTEventsInput,
+    #[allow(missing_docs)] // documentation missing in model
     IoTFleetMetric,
     #[allow(missing_docs)] // documentation missing in model
     IoTJobTemplate,
@@ -1174,24 +1156,6 @@
     #[allow(missing_docs)] // documentation missing in model
     IoTSecurityProfile,
     #[allow(missing_docs)] // documentation missing in model
-    IoTThingGroup,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTAnalyticsChannel,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTAnalyticsDataset,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTAnalyticsDatastore,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTAnalyticsPipeline,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTCoreDeviceAdvisorSuiteDefinition,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTEventsAlarmModel,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTEventsDetectorModel,
-    #[allow(missing_docs)] // documentation missing in model
-    IoTEventsInput,
-    #[allow(missing_docs)] // documentation missing in model
     IoTSiteWiseAsset,
     #[allow(missing_docs)] // documentation missing in model
     IoTSiteWiseAssetModel,
@@ -1204,6 +1168,8 @@
     #[allow(missing_docs)] // documentation missing in model
     IoTSiteWiseProject,
     #[allow(missing_docs)] // documentation missing in model
+    IoTThingGroup,
+    #[allow(missing_docs)] // documentation missing in model
     IoTTwinMakerComponentType,
     #[allow(missing_docs)] // documentation missing in model
     IoTTwinMakerEntity,
@@ -1222,8 +1188,6 @@
     #[allow(missing_docs)] // documentation missing in model
     KmsAlias,
     #[allow(missing_docs)] // documentation missing in model
-    Key,
-    #[allow(missing_docs)] // documentation missing in model
     KafkaConnectConnector,
     #[allow(missing_docs)] // documentation missing in model
     KafkaConnectCustomPlugin,
@@ -1230,14 +1194,16 @@
     #[allow(missing_docs)] // documentation missing in model
     KendraIndex,
     #[allow(missing_docs)] // documentation missing in model
-    KinesisStream,
-    #[allow(missing_docs)] // documentation missing in model
-    KinesisStreamConsumer,
+    Key,
     #[allow(missing_docs)] // documentation missing in model
     KinesisAnalyticsV2Application,
     #[allow(missing_docs)] // documentation missing in model
     KinesisFirehoseDeliveryStream,
     #[allow(missing_docs)] // documentation missing in model
+    KinesisStream,
+    #[allow(missing_docs)] // documentation missing in model
+    KinesisStreamConsumer,
+    #[allow(missing_docs)] // documentation missing in model
     KinesisVideoSignalingChannel,
     #[allow(missing_docs)] // documentation missing in model
     KinesisVideoStream,
@@ -1244,7 +1210,9 @@
     #[allow(missing_docs)] // documentation missing in model
     LambdaCodeSigningConfig,
     #[allow(missing_docs)] // documentation missing in model
-    Function,
+    LaunchConfiguration,
+    #[allow(missing_docs)] // documentation missing in model
+    LaunchTemplate,
     #[allow(missing_docs)] // documentation missing in model
     LexBot,
     #[allow(missing_docs)] // documentation missing in model
@@ -1258,6 +1226,12 @@
     #[allow(missing_docs)] // documentation missing in model
     LightsailStaticIp,
     #[allow(missing_docs)] // documentation missing in model
+    ListenerV2,
+    #[allow(missing_docs)] // documentation missing in model
+    LoadBalancer,
+    #[allow(missing_docs)] // documentation missing in model
+    LoadBalancerV2,
+    #[allow(missing_docs)] // documentation missing in model
     LocationApiKey,
     #[allow(missing_docs)] // documentation missing in model
     LogsDestination,
@@ -1280,6 +1254,10 @@
     #[allow(missing_docs)] // documentation missing in model
     MskVpcConnection,
     #[allow(missing_docs)] // documentation missing in model
+    ManagedInstanceInventory,
+    #[allow(missing_docs)] // documentation missing in model
+    ManagedRuleSetV2,
+    #[allow(missing_docs)] // documentation missing in model
     MediaConnectFlowEntitlement,
     #[allow(missing_docs)] // documentation missing in model
     MediaConnectFlowSource,
@@ -1302,6 +1280,10 @@
     #[allow(missing_docs)] // documentation missing in model
     MemoryDbSubnetGroup,
     #[allow(missing_docs)] // documentation missing in model
+    NatGateway,
+    #[allow(missing_docs)] // documentation missing in model
+    NetworkAcl,
+    #[allow(missing_docs)] // documentation missing in model
     NetworkFirewallFirewall,
     #[allow(missing_docs)] // documentation missing in model
     NetworkFirewallFirewallPolicy,
@@ -1308,6 +1290,10 @@
     #[allow(missing_docs)] // documentation missing in model
     NetworkFirewallRuleGroup,
     #[allow(missing_docs)] // documentation missing in model
+    NetworkInsightsAccessScopeAnalysis,
+    #[allow(missing_docs)] // documentation missing in model
+    NetworkInterface,
+    #[allow(missing_docs)] // documentation missing in model
     NetworkManagerConnectPeer,
     #[allow(missing_docs)] // documentation missing in model
     NetworkManagerCustomerGatewayAssociation,
@@ -1342,6 +1328,8 @@
     #[allow(missing_docs)] // documentation missing in model
     PanoramaPackage,
     #[allow(missing_docs)] // documentation missing in model
+    PatchCompliance,
+    #[allow(missing_docs)] // documentation missing in model
     PersonalizeDataset,
     #[allow(missing_docs)] // documentation missing in model
     PersonalizeDatasetGroup,
@@ -1366,27 +1354,25 @@
     #[allow(missing_docs)] // documentation missing in model
     PinpointSegment,
     #[allow(missing_docs)] // documentation missing in model
-    QldbLedger,
-    #[allow(missing_docs)] // documentation missing in model
-    QuickSightDataSource,
+    Pipeline,
     #[allow(missing_docs)] // documentation missing in model
-    QuickSightTemplate,
+    Policy,
     #[allow(missing_docs)] // documentation missing in model
-    QuickSightTheme,
+    Portfolio,
     #[allow(missing_docs)] // documentation missing in model
-    DbCluster,
+    Project,
     #[allow(missing_docs)] // documentation missing in model
-    DbClusterSnapshot,
+    Protection,
     #[allow(missing_docs)] // documentation missing in model
-    DbInstance,
+    QldbLedger,
     #[allow(missing_docs)] // documentation missing in model
-    DbSecurityGroup,
+    Queue,
     #[allow(missing_docs)] // documentation missing in model
-    DbSnapshot,
+    QuickSightDataSource,
     #[allow(missing_docs)] // documentation missing in model
-    DbSubnetGroup,
+    QuickSightTemplate,
     #[allow(missing_docs)] // documentation missing in model
-    EventSubscription,
+    QuickSightTheme,
     #[allow(missing_docs)] // documentation missing in model
     RdsGlobalCluster,
     #[allow(missing_docs)] // documentation missing in model
@@ -1396,15 +1382,7 @@
     #[allow(missing_docs)] // documentation missing in model
     RumAppMonitor,
     #[allow(missing_docs)] // documentation missing in model
-    Cluster,
-    #[allow(missing_docs)] // documentation missing in model
-    ClusterParameterGroup,
-    #[allow(missing_docs)] // documentation missing in model
-    ClusterSecurityGroup,
-    #[allow(missing_docs)] // documentation missing in model
-    ClusterSnapshot,
-    #[allow(missing_docs)] // documentation missing in model
-    ClusterSubnetGroup,
+    RateBasedRule,
     #[allow(missing_docs)] // documentation missing in model
     RedshiftEndpointAccess,
     #[allow(missing_docs)] // documentation missing in model
@@ -1416,12 +1394,30 @@
     #[allow(missing_docs)] // documentation missing in model
     RedshiftScheduledAction,
     #[allow(missing_docs)] // documentation missing in model
+    RegexPatternSetV2,
+    #[allow(missing_docs)] // documentation missing in model
+    RegionalProtection,
+    #[allow(missing_docs)] // documentation missing in model
+    RegionalRateBasedRule,
+    #[allow(missing_docs)] // documentation missing in model
+    RegionalRule,
+    #[allow(missing_docs)] // documentation missing in model
+    RegionalRuleGroup,
+    #[allow(missing_docs)] // documentation missing in model
+    RegionalWebAcl,
+    #[allow(missing_docs)] // documentation missing in model
+    RegisteredHaInstance,
+    #[allow(missing_docs)] // documentation missing in model
     ResilienceHubApp,
     #[allow(missing_docs)] // documentation missing in model
     ResilienceHubResiliencyPolicy,
     #[allow(missing_docs)] // documentation missing in model
+    ResourceCompliance,
+    #[allow(missing_docs)] // documentation missing in model
     ResourceExplorer2Index,
     #[allow(missing_docs)] // documentation missing in model
+    RestApi,
+    #[allow(missing_docs)] // documentation missing in model
     RoboMakerRobotApplication,
     #[allow(missing_docs)] // documentation missing in model
     RoboMakerRobotApplicationVersion,
@@ -1428,6 +1424,8 @@
     #[allow(missing_docs)] // documentation missing in model
     RoboMakerSimulationApplication,
     #[allow(missing_docs)] // documentation missing in model
+    Role,
+    #[allow(missing_docs)] // documentation missing in model
     RolesAnywhereProfile,
     #[allow(missing_docs)] // documentation missing in model
     RolesAnywhereTrustAnchor,
@@ -1472,6 +1470,14 @@
     #[allow(missing_docs)] // documentation missing in model
     Route53ResolverResolverRuleAssociation,
     #[allow(missing_docs)] // documentation missing in model
+    RouteTable,
+    #[allow(missing_docs)] // documentation missing in model
+    Rule,
+    #[allow(missing_docs)] // documentation missing in model
+    RuleGroup,
+    #[allow(missing_docs)] // documentation missing in model
+    RuleGroupV2,
+    #[allow(missing_docs)] // documentation missing in model
     S3AccessGrant,
     #[allow(missing_docs)] // documentation missing in model
     S3AccessGrantsInstance,
@@ -1480,9 +1486,9 @@
     #[allow(missing_docs)] // documentation missing in model
     S3AccessPoint,
     #[allow(missing_docs)] // documentation missing in model
-    AccountPublicAccessBlock,
+    S3ExpressBucketPolicy,
     #[allow(missing_docs)] // documentation missing in model
-    Bucket,
+    S3ExpressDirectoryBucket,
     #[allow(missing_docs)] // documentation missing in model
     S3MultiRegionAccessPoint,
     #[allow(missing_docs)] // documentation missing in model
@@ -1490,10 +1496,6 @@
     #[allow(missing_docs)] // documentation missing in model
     S3StorageLensGroup,
     #[allow(missing_docs)] // documentation missing in model
-    S3ExpressBucketPolicy,
-    #[allow(missing_docs)] // documentation missing in model
-    S3ExpressDirectoryBucket,
-    #[allow(missing_docs)] // documentation missing in model
     S3TablesTableBucket,
     #[allow(missing_docs)] // documentation missing in model
     S3TablesTableBucketPolicy,
@@ -1512,26 +1514,14 @@
     #[allow(missing_docs)] // documentation missing in model
     SesTemplate,
     #[allow(missing_docs)] // documentation missing in model
-    Topic,
-    #[allow(missing_docs)] // documentation missing in model
-    Queue,
-    #[allow(missing_docs)] // documentation missing in model
-    AssociationCompliance,
+    SsmContactsContact,
     #[allow(missing_docs)] // documentation missing in model
     SsmDocument,
     #[allow(missing_docs)] // documentation missing in model
-    FileData,
-    #[allow(missing_docs)] // documentation missing in model
-    ManagedInstanceInventory,
-    #[allow(missing_docs)] // documentation missing in model
-    PatchCompliance,
+    SsmIncidentsResponsePlan,
     #[allow(missing_docs)] // documentation missing in model
     SsmResourceDataSync,
     #[allow(missing_docs)] // documentation missing in model
-    SsmContactsContact,
-    #[allow(missing_docs)] // documentation missing in model
-    SsmIncidentsResponsePlan,
-    #[allow(missing_docs)] // documentation missing in model
     SageMakerAppImageConfig,
     #[allow(missing_docs)] // documentation missing in model
     SageMakerCodeRepository,
@@ -1564,19 +1554,19 @@
     #[allow(missing_docs)] // documentation missing in model
     SageMakerWorkteam,
     #[allow(missing_docs)] // documentation missing in model
-    SecretsManagerResourcePolicy,
+    ScalingPolicy,
     #[allow(missing_docs)] // documentation missing in model
-    SecretsManagerRotationSchedule,
+    ScheduledAction,
     #[allow(missing_docs)] // documentation missing in model
     Secret,
     #[allow(missing_docs)] // documentation missing in model
-    SecurityHubStandard,
+    SecretsManagerResourcePolicy,
     #[allow(missing_docs)] // documentation missing in model
-    CloudFormationProduct,
+    SecretsManagerRotationSchedule,
     #[allow(missing_docs)] // documentation missing in model
-    CloudFormationProvisionedProduct,
+    SecurityGroup,
     #[allow(missing_docs)] // documentation missing in model
-    Portfolio,
+    SecurityHubStandard,
     #[allow(missing_docs)] // documentation missing in model
     ServiceDiscoveryHttpNamespace,
     #[allow(missing_docs)] // documentation missing in model
@@ -1586,16 +1576,28 @@
     #[allow(missing_docs)] // documentation missing in model
     ServiceDiscoveryService,
     #[allow(missing_docs)] // documentation missing in model
-    Protection,
+    SignerSigningProfile,
     #[allow(missing_docs)] // documentation missing in model
-    RegionalProtection,
+    Stack,
     #[allow(missing_docs)] // documentation missing in model
-    SignerSigningProfile,
+    Stage,
+    #[allow(missing_docs)] // documentation missing in model
+    StageV2,
     #[allow(missing_docs)] // documentation missing in model
     StepFunctionsActivity,
     #[allow(missing_docs)] // documentation missing in model
     StepFunctionsStateMachine,
     #[allow(missing_docs)] // documentation missing in model
+    StreamingDistribution,
+    #[allow(missing_docs)] // documentation missing in model
+    Subnet,
+    #[allow(missing_docs)] // documentation missing in model
+    Table,
+    #[allow(missing_docs)] // documentation missing in model
+    Topic,
+    #[allow(missing_docs)] // documentation missing in model
+    Trail,
+    #[allow(missing_docs)] // documentation missing in model
     TransferAgreement,
     #[allow(missing_docs)] // documentation missing in model
     TransferCertificate,
@@ -1608,29 +1610,29 @@
     #[allow(missing_docs)] // documentation missing in model
     TransferWorkflow,
     #[allow(missing_docs)] // documentation missing in model
-    RateBasedRule,
+    TransitGateway,
     #[allow(missing_docs)] // documentation missing in model
-    Rule,
+    TransitGatewayAttachment,
     #[allow(missing_docs)] // documentation missing in model
-    RuleGroup,
+    TransitGatewayRouteTable,
     #[allow(missing_docs)] // documentation missing in model
-    WebAcl,
+    User,
     #[allow(missing_docs)] // documentation missing in model
-    RegionalRateBasedRule,
+    Vpc,
     #[allow(missing_docs)] // documentation missing in model
-    RegionalRule,
+    VpcEndpoint,
     #[allow(missing_docs)] // documentation missing in model
-    RegionalRuleGroup,
+    VpcEndpointService,
     #[allow(missing_docs)] // documentation missing in model
-    RegionalWebAcl,
+    VpcPeeringConnection,
     #[allow(missing_docs)] // documentation missing in model
-    IpSetV2,
+    VpnConnection,
     #[allow(missing_docs)] // documentation missing in model
-    ManagedRuleSetV2,
+    VpnGateway,
     #[allow(missing_docs)] // documentation missing in model
-    RegexPatternSetV2,
+    Volume,
     #[allow(missing_docs)] // documentation missing in model
-    RuleGroupV2,
+    WebAcl,
     #[allow(missing_docs)] // documentation missing in model
     WebAclv2,
     #[allow(missing_docs)] // documentation missing in model
@@ -1637,8 +1639,6 @@
     WorkSpacesConnectionAlias,
     #[allow(missing_docs)] // documentation missing in model
     WorkSpacesWorkspace,
-    #[allow(missing_docs)] // documentation missing in model
-    EncryptionConfig,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -1646,21 +1646,19 @@
 impl ::std::convert::From<&str> for ResourceType {
     fn from(s: &str) -> Self {
         match s {
-            "AWS::ACM::Certificate" => ResourceType::Certificate,
             "AWS::ACMPCA::CertificateAuthority" => ResourceType::AcmpcaCertificateAuthority,
             "AWS::ACMPCA::CertificateAuthorityActivation" => ResourceType::AcmpcaCertificateAuthorityActivation,
             "AWS::APS::RuleGroupsNamespace" => ResourceType::ApsRuleGroupsNamespace,
             "AWS::AccessAnalyzer::Analyzer" => ResourceType::AccessAnalyzerAnalyzer,
+            "AWS::S3::AccountPublicAccessBlock" => ResourceType::AccountPublicAccessBlock,
+            "AWS::CloudWatch::Alarm" => ResourceType::Alarm,
             "AWS::AmazonMQ::Broker" => ResourceType::AmazonMqBroker,
             "AWS::Amplify::App" => ResourceType::AmplifyApp,
             "AWS::Amplify::Branch" => ResourceType::AmplifyBranch,
+            "AWS::ApiGatewayV2::Api" => ResourceType::Api,
             "AWS::ApiGateway::Method" => ResourceType::ApiGatewayMethod,
-            "AWS::ApiGateway::RestApi" => ResourceType::RestApi,
-            "AWS::ApiGateway::Stage" => ResourceType::Stage,
             "AWS::ApiGateway::UsagePlan" => ResourceType::ApiGatewayUsagePlan,
-            "AWS::ApiGatewayV2::Api" => ResourceType::Api,
             "AWS::ApiGatewayV2::Integration" => ResourceType::ApiGatewayV2Integration,
-            "AWS::ApiGatewayV2::Stage" => ResourceType::StageV2,
             "AWS::AppConfig::Application" => ResourceType::AppConfigApplication,
             "AWS::AppConfig::ConfigurationProfile" => ResourceType::AppConfigConfigurationProfile,
             "AWS::AppConfig::DeploymentStrategy" => ResourceType::AppConfigDeploymentStrategy,
@@ -1687,53 +1685,56 @@
             "AWS::AppStream::Stack" => ResourceType::AppStreamStack,
             "AWS::AppSync::ApiCache" => ResourceType::AppSyncApiCache,
             "AWS::AppSync::GraphQLApi" => ResourceType::AppSyncGraphQlApi,
+            "AWS::ElasticBeanstalk::Application" => ResourceType::Application,
+            "AWS::ElasticBeanstalk::ApplicationVersion" => ResourceType::ApplicationVersion,
+            "AWS::SSM::AssociationCompliance" => ResourceType::AssociationCompliance,
             "AWS::Athena::DataCatalog" => ResourceType::AthenaDataCatalog,
             "AWS::Athena::PreparedStatement" => ResourceType::AthenaPreparedStatement,
             "AWS::Athena::WorkGroup" => ResourceType::AthenaWorkGroup,
             "AWS::AuditManager::Assessment" => ResourceType::AuditManagerAssessment,
             "AWS::AutoScaling::AutoScalingGroup" => ResourceType::AutoScalingGroup,
-            "AWS::AutoScaling::LaunchConfiguration" => ResourceType::LaunchConfiguration,
-            "AWS::AutoScaling::ScalingPolicy" => ResourceType::ScalingPolicy,
-            "AWS::AutoScaling::ScheduledAction" => ResourceType::ScheduledAction,
             "AWS::AutoScaling::WarmPool" => ResourceType::AutoScalingWarmPool,
             "AWS::B2BI::Capability" => ResourceType::B2BiCapability,
             "AWS::BCMDataExports::Export" => ResourceType::BcmDataExportsExport,
+            "AWS::BackupGateway::Hypervisor" => ResourceType::BackupGatewayHypervisor,
             "AWS::Backup::BackupPlan" => ResourceType::BackupPlan,
-            "AWS::Backup::BackupSelection" => ResourceType::BackupSelection,
-            "AWS::Backup::BackupVault" => ResourceType::BackupVault,
             "AWS::Backup::RecoveryPoint" => ResourceType::BackupRecoveryPoint,
             "AWS::Backup::ReportPlan" => ResourceType::BackupReportPlan,
             "AWS::Backup::RestoreTestingPlan" => ResourceType::BackupRestoreTestingPlan,
-            "AWS::BackupGateway::Hypervisor" => ResourceType::BackupGatewayHypervisor,
+            "AWS::Backup::BackupSelection" => ResourceType::BackupSelection,
+            "AWS::Backup::BackupVault" => ResourceType::BackupVault,
             "AWS::Batch::ComputeEnvironment" => ResourceType::BatchComputeEnvironment,
             "AWS::Batch::JobQueue" => ResourceType::BatchJobQueue,
             "AWS::Batch::SchedulingPolicy" => ResourceType::BatchSchedulingPolicy,
+            "AWS::BedrockAgentCore::BrowserCustom" => ResourceType::BedrockAgentCoreBrowserCustom,
+            "AWS::BedrockAgentCore::Runtime" => ResourceType::BedrockAgentCoreRuntime,
             "AWS::Bedrock::ApplicationInferenceProfile" => ResourceType::BedrockApplicationInferenceProfile,
             "AWS::Bedrock::Guardrail" => ResourceType::BedrockGuardrail,
             "AWS::Bedrock::KnowledgeBase" => ResourceType::BedrockKnowledgeBase,
             "AWS::Bedrock::Prompt" => ResourceType::BedrockPrompt,
-            "AWS::BedrockAgentCore::BrowserCustom" => ResourceType::BedrockAgentCoreBrowserCustom,
-            "AWS::BedrockAgentCore::Runtime" => ResourceType::BedrockAgentCoreRuntime,
+            "AWS::S3::Bucket" => ResourceType::Bucket,
             "AWS::Budgets::BudgetsAction" => ResourceType::BudgetsBudgetsAction,
             "AWS::Cassandra::Keyspace" => ResourceType::CassandraKeyspace,
+            "AWS::ACM::Certificate" => ResourceType::Certificate,
             "AWS::CleanRoomsML::TrainingDataset" => ResourceType::CleanRoomsMlTrainingDataset,
             "AWS::Cloud9::EnvironmentEC2" => ResourceType::Cloud9EnvironmentEc2,
             "AWS::CloudFormation::GuardHook" => ResourceType::CloudFormationGuardHook,
             "AWS::CloudFormation::LambdaHook" => ResourceType::CloudFormationLambdaHook,
-            "AWS::CloudFormation::Stack" => ResourceType::Stack,
+            "AWS::ServiceCatalog::CloudFormationProduct" => ResourceType::CloudFormationProduct,
+            "AWS::ServiceCatalog::CloudFormationProvisionedProduct" => ResourceType::CloudFormationProvisionedProduct,
             "AWS::CloudFormation::StackSet" => ResourceType::CloudFormationStackSet,
-            "AWS::CloudFront::Distribution" => ResourceType::Distribution,
             "AWS::CloudFront::KeyValueStore" => ResourceType::CloudFrontKeyValueStore,
             "AWS::CloudFront::PublicKey" => ResourceType::CloudFrontPublicKey,
             "AWS::CloudFront::RealtimeLogConfig" => ResourceType::CloudFrontRealtimeLogConfig,
-            "AWS::CloudFront::StreamingDistribution" => ResourceType::StreamingDistribution,
             "AWS::CloudTrail::EventDataStore" => ResourceType::CloudTrailEventDataStore,
-            "AWS::CloudTrail::Trail" => ResourceType::Trail,
-            "AWS::CloudWatch::Alarm" => ResourceType::Alarm,
             "AWS::CloudWatch::MetricStream" => ResourceType::CloudWatchMetricStream,
+            "AWS::Redshift::Cluster" => ResourceType::Cluster,
+            "AWS::Redshift::ClusterParameterGroup" => ResourceType::ClusterParameterGroup,
+            "AWS::Redshift::ClusterSecurityGroup" => ResourceType::ClusterSecurityGroup,
+            "AWS::Redshift::ClusterSnapshot" => ResourceType::ClusterSnapshot,
+            "AWS::Redshift::ClusterSubnetGroup" => ResourceType::ClusterSubnetGroup,
             "AWS::CodeArtifact::Domain" => ResourceType::CodeArtifactDomain,
             "AWS::CodeArtifact::Repository" => ResourceType::CodeArtifactRepository,
-            "AWS::CodeBuild::Project" => ResourceType::Project,
             "AWS::CodeBuild::ReportGroup" => ResourceType::CodeBuildReportGroup,
             "AWS::CodeDeploy::Application" => ResourceType::CodeDeployApplication,
             "AWS::CodeDeploy::DeploymentConfig" => ResourceType::CodeDeployDeploymentConfig,
@@ -1740,7 +1741,6 @@
             "AWS::CodeDeploy::DeploymentGroup" => ResourceType::CodeDeployDeploymentGroup,
             "AWS::CodeGuruProfiler::ProfilingGroup" => ResourceType::CodeGuruProfilerProfilingGroup,
             "AWS::CodeGuruReviewer::RepositoryAssociation" => ResourceType::CodeGuruReviewerRepositoryAssociation,
-            "AWS::CodePipeline::Pipeline" => ResourceType::Pipeline,
             "AWS::Cognito::IdentityPool" => ResourceType::CognitoIdentityPool,
             "AWS::Cognito::UserPool" => ResourceType::CognitoUserPool,
             "AWS::Cognito::UserPoolClient" => ResourceType::CognitoUserPoolClient,
@@ -1748,9 +1748,8 @@
             "AWS::Comprehend::Flywheel" => ResourceType::ComprehendFlywheel,
             "AWS::Config::AggregationAuthorization" => ResourceType::ConfigAggregationAuthorization,
             "AWS::Config::ConformancePack" => ResourceType::ConfigConformancePack,
+            "AWS::Config::StoredQuery" => ResourceType::ConfigStoredQuery,
             "AWS::Config::ConformancePackCompliance" => ResourceType::ConformancePackCompliance,
-            "AWS::Config::ResourceCompliance" => ResourceType::ResourceCompliance,
-            "AWS::Config::StoredQuery" => ResourceType::ConfigStoredQuery,
             "AWS::Connect::Instance" => ResourceType::ConnectInstance,
             "AWS::Connect::PhoneNumber" => ResourceType::ConnectPhoneNumber,
             "AWS::Connect::QuickConnect" => ResourceType::ConnectQuickConnect,
@@ -1757,8 +1756,15 @@
             "AWS::Connect::Rule" => ResourceType::ConnectRule,
             "AWS::Connect::SecurityProfile" => ResourceType::ConnectSecurityProfile,
             "AWS::Connect::User" => ResourceType::ConnectUser,
+            "AWS::EC2::CustomerGateway" => ResourceType::CustomerGateway,
             "AWS::CustomerProfiles::Domain" => ResourceType::CustomerProfilesDomain,
             "AWS::CustomerProfiles::ObjectType" => ResourceType::CustomerProfilesObjectType,
+            "AWS::RDS::DBCluster" => ResourceType::DbCluster,
+            "AWS::RDS::DBClusterSnapshot" => ResourceType::DbClusterSnapshot,
+            "AWS::RDS::DBInstance" => ResourceType::DbInstance,
+            "AWS::RDS::DBSecurityGroup" => ResourceType::DbSecurityGroup,
+            "AWS::RDS::DBSnapshot" => ResourceType::DbSnapshot,
+            "AWS::RDS::DBSubnetGroup" => ResourceType::DbSubnetGroup,
             "AWS::DMS::Certificate" => ResourceType::DmsCertificate,
             "AWS::DMS::Endpoint" => ResourceType::DmsEndpoint,
             "AWS::DMS::EventSubscription" => ResourceType::DmsEventSubscription,
@@ -1780,19 +1786,15 @@
             "AWS::DeviceFarm::InstanceProfile" => ResourceType::DeviceFarmInstanceProfile,
             "AWS::DeviceFarm::Project" => ResourceType::DeviceFarmProject,
             "AWS::DeviceFarm::TestGridProject" => ResourceType::DeviceFarmTestGridProject,
-            "AWS::DynamoDB::Table" => ResourceType::Table,
+            "AWS::CloudFront::Distribution" => ResourceType::Distribution,
+            "AWS::Elasticsearch::Domain" => ResourceType::Domain,
             "AWS::EC2::CapacityReservation" => ResourceType::Ec2CapacityReservation,
             "AWS::EC2::CarrierGateway" => ResourceType::Ec2CarrierGateway,
             "AWS::EC2::ClientVpnEndpoint" => ResourceType::Ec2ClientVpnEndpoint,
             "AWS::EC2::ClientVpnTargetNetworkAssociation" => ResourceType::Ec2ClientVpnTargetNetworkAssociation,
-            "AWS::EC2::CustomerGateway" => ResourceType::CustomerGateway,
             "AWS::EC2::DHCPOptions" => ResourceType::Ec2DhcpOptions,
             "AWS::EC2::EC2Fleet" => ResourceType::Ec2Ec2Fleet,
-            "AWS::EC2::EIP" => ResourceType::Eip,
             "AWS::EC2::EIPAssociation" => ResourceType::Ec2EipAssociation,
-            "AWS::EC2::EgressOnlyInternetGateway" => ResourceType::EgressOnlyInternetGateway,
-            "AWS::EC2::FlowLog" => ResourceType::FlowLog,
-            "AWS::EC2::Host" => ResourceType::Host,
             "AWS::EC2::IPAM" => ResourceType::Ec2Ipam,
             "AWS::EC2::IPAMPool" => ResourceType::Ec2IpamPool,
             "AWS::EC2::IPAMPoolCidr" => ResourceType::Ec2IpamPoolCidr,
@@ -1799,25 +1801,14 @@
             "AWS::EC2::IPAMResourceDiscovery" => ResourceType::Ec2IpamResourceDiscovery,
             "AWS::EC2::IPAMResourceDiscoveryAssociation" => ResourceType::Ec2IpamResourceDiscoveryAssociation,
             "AWS::EC2::IPAMScope" => ResourceType::Ec2IpamScope,
-            "AWS::EC2::Instance" => ResourceType::Instance,
             "AWS::EC2::InstanceConnectEndpoint" => ResourceType::Ec2InstanceConnectEndpoint,
-            "AWS::EC2::InternetGateway" => ResourceType::InternetGateway,
-            "AWS::EC2::LaunchTemplate" => ResourceType::LaunchTemplate,
-            "AWS::EC2::NatGateway" => ResourceType::NatGateway,
-            "AWS::EC2::NetworkAcl" => ResourceType::NetworkAcl,
             "AWS::EC2::NetworkInsightsAccessScope" => ResourceType::Ec2NetworkInsightsAccessScope,
-            "AWS::EC2::NetworkInsightsAccessScopeAnalysis" => ResourceType::NetworkInsightsAccessScopeAnalysis,
             "AWS::EC2::NetworkInsightsAnalysis" => ResourceType::Ec2NetworkInsightsAnalysis,
             "AWS::EC2::NetworkInsightsPath" => ResourceType::Ec2NetworkInsightsPath,
-            "AWS::EC2::NetworkInterface" => ResourceType::NetworkInterface,
             "AWS::EC2::PrefixList" => ResourceType::Ec2PrefixList,
-            "AWS::EC2::RegisteredHAInstance" => ResourceType::RegisteredHaInstance,
-            "AWS::EC2::RouteTable" => ResourceType::RouteTable,
-            "AWS::EC2::SecurityGroup" => ResourceType::SecurityGroup,
             "AWS::EC2::SecurityGroupVpcAssociation" => ResourceType::Ec2SecurityGroupVpcAssociation,
             "AWS::EC2::SnapshotBlockPublicAccess" => ResourceType::Ec2SnapshotBlockPublicAccess,
             "AWS::EC2::SpotFleet" => ResourceType::Ec2SpotFleet,
-            "AWS::EC2::Subnet" => ResourceType::Subnet,
             "AWS::EC2::SubnetCidrBlock" => ResourceType::Ec2SubnetCidrBlock,
             "AWS::EC2::SubnetNetworkAclAssociation" => ResourceType::Ec2SubnetNetworkAclAssociation,
             "AWS::EC2::SubnetRouteTableAssociation" => ResourceType::Ec2SubnetRouteTableAssociation,
@@ -1824,24 +1815,14 @@
             "AWS::EC2::TrafficMirrorFilter" => ResourceType::Ec2TrafficMirrorFilter,
             "AWS::EC2::TrafficMirrorSession" => ResourceType::Ec2TrafficMirrorSession,
             "AWS::EC2::TrafficMirrorTarget" => ResourceType::Ec2TrafficMirrorTarget,
-            "AWS::EC2::TransitGateway" => ResourceType::TransitGateway,
-            "AWS::EC2::TransitGatewayAttachment" => ResourceType::TransitGatewayAttachment,
             "AWS::EC2::TransitGatewayConnect" => ResourceType::Ec2TransitGatewayConnect,
             "AWS::EC2::TransitGatewayMulticastDomain" => ResourceType::Ec2TransitGatewayMulticastDomain,
-            "AWS::EC2::TransitGatewayRouteTable" => ResourceType::TransitGatewayRouteTable,
-            "AWS::EC2::VPC" => ResourceType::Vpc,
             "AWS::EC2::VPCBlockPublicAccessExclusion" => ResourceType::Ec2VpcBlockPublicAccessExclusion,
             "AWS::EC2::VPCBlockPublicAccessOptions" => ResourceType::Ec2VpcBlockPublicAccessOptions,
-            "AWS::EC2::VPCEndpoint" => ResourceType::VpcEndpoint,
             "AWS::EC2::VPCEndpointConnectionNotification" => ResourceType::Ec2VpcEndpointConnectionNotification,
-            "AWS::EC2::VPCEndpointService" => ResourceType::VpcEndpointService,
             "AWS::EC2::VPCGatewayAttachment" => ResourceType::Ec2VpcGatewayAttachment,
-            "AWS::EC2::VPCPeeringConnection" => ResourceType::VpcPeeringConnection,
-            "AWS::EC2::VPNConnection" => ResourceType::VpnConnection,
             "AWS::EC2::VPNConnectionRoute" => ResourceType::Ec2VpnConnectionRoute,
-            "AWS::EC2::VPNGateway" => ResourceType::VpnGateway,
             "AWS::EC2::VerifiedAccessInstance" => ResourceType::Ec2VerifiedAccessInstance,
-            "AWS::EC2::Volume" => ResourceType::Volume,
             "AWS::ECR::PublicRepository" => ResourceType::EcrPublicRepository,
             "AWS::ECR::PullThroughCacheRule" => ResourceType::EcrPullThroughCacheRule,
             "AWS::ECR::RegistryPolicy" => ResourceType::EcrRegistryPolicy,
@@ -1855,29 +1836,27 @@
             "AWS::ECS::TaskSet" => ResourceType::EcsTaskSet,
             "AWS::EFS::AccessPoint" => ResourceType::EfsAccessPoint,
             "AWS::EFS::FileSystem" => ResourceType::EfsFileSystem,
+            "AWS::EC2::EIP" => ResourceType::Eip,
             "AWS::EKS::Addon" => ResourceType::EksAddon,
             "AWS::EKS::Cluster" => ResourceType::EksCluster,
             "AWS::EKS::FargateProfile" => ResourceType::EksFargateProfile,
             "AWS::EKS::IdentityProviderConfig" => ResourceType::EksIdentityProviderConfig,
+            "AWS::EMRContainers::VirtualCluster" => ResourceType::EmrContainersVirtualCluster,
             "AWS::EMR::SecurityConfiguration" => ResourceType::EmrSecurityConfiguration,
+            "AWS::EMRServerless::Application" => ResourceType::EmrServerlessApplication,
             "AWS::EMR::Studio" => ResourceType::EmrStudio,
-            "AWS::EMRContainers::VirtualCluster" => ResourceType::EmrContainersVirtualCluster,
-            "AWS::EMRServerless::Application" => ResourceType::EmrServerlessApplication,
-            "AWS::ElasticBeanstalk::Application" => ResourceType::Application,
-            "AWS::ElasticBeanstalk::ApplicationVersion" => ResourceType::ApplicationVersion,
-            "AWS::ElasticBeanstalk::Environment" => ResourceType::Environment,
-            "AWS::ElasticLoadBalancing::LoadBalancer" => ResourceType::LoadBalancer,
-            "AWS::ElasticLoadBalancingV2::Listener" => ResourceType::ListenerV2,
-            "AWS::ElasticLoadBalancingV2::LoadBalancer" => ResourceType::LoadBalancerV2,
+            "AWS::EC2::EgressOnlyInternetGateway" => ResourceType::EgressOnlyInternetGateway,
             "AWS::ElasticLoadBalancingV2::TargetGroup" => ResourceType::ElasticLoadBalancingV2TargetGroup,
-            "AWS::Elasticsearch::Domain" => ResourceType::Domain,
+            "AWS::XRay::EncryptionConfig" => ResourceType::EncryptionConfig,
             "AWS::EntityResolution::IdMappingWorkflow" => ResourceType::EntityResolutionIdMappingWorkflow,
             "AWS::EntityResolution::MatchingWorkflow" => ResourceType::EntityResolutionMatchingWorkflow,
             "AWS::EntityResolution::SchemaMapping" => ResourceType::EntityResolutionSchemaMapping,
+            "AWS::ElasticBeanstalk::Environment" => ResourceType::Environment,
             "AWS::EventSchemas::Discoverer" => ResourceType::EventSchemasDiscoverer,
             "AWS::EventSchemas::Registry" => ResourceType::EventSchemasRegistry,
             "AWS::EventSchemas::RegistryPolicy" => ResourceType::EventSchemasRegistryPolicy,
             "AWS::EventSchemas::Schema" => ResourceType::EventSchemasSchema,
+            "AWS::RDS::EventSubscription" => ResourceType::EventSubscription,
             "AWS::Events::ApiDestination" => ResourceType::EventsApiDestination,
             "AWS::Events::Archive" => ResourceType::EventsArchive,
             "AWS::Events::Connection" => ResourceType::EventsConnection,
@@ -1888,6 +1867,8 @@
             "AWS::Evidently::Project" => ResourceType::EvidentlyProject,
             "AWS::Evidently::Segment" => ResourceType::EvidentlySegment,
             "AWS::FIS::ExperimentTemplate" => ResourceType::FisExperimentTemplate,
+            "AWS::SSM::FileData" => ResourceType::FileData,
+            "AWS::EC2::FlowLog" => ResourceType::FlowLog,
             "AWS::Forecast::Dataset" => ResourceType::ForecastDataset,
             "AWS::Forecast::DatasetGroup" => ResourceType::ForecastDatasetGroup,
             "AWS::FraudDetector::EntityType" => ResourceType::FraudDetectorEntityType,
@@ -1894,6 +1875,7 @@
             "AWS::FraudDetector::Label" => ResourceType::FraudDetectorLabel,
             "AWS::FraudDetector::Outcome" => ResourceType::FraudDetectorOutcome,
             "AWS::FraudDetector::Variable" => ResourceType::FraudDetectorVariable,
+            "AWS::Lambda::Function" => ResourceType::Function,
             "AWS::GameLift::Build" => ResourceType::GameLiftBuild,
             "AWS::GlobalAccelerator::Accelerator" => ResourceType::GlobalAcceleratorAccelerator,
             "AWS::GlobalAccelerator::EndpointGroup" => ResourceType::GlobalAcceleratorEndpointGroup,
@@ -1907,6 +1889,7 @@
             "AWS::GroundStation::Config" => ResourceType::GroundStationConfig,
             "AWS::GroundStation::DataflowEndpointGroup" => ResourceType::GroundStationDataflowEndpointGroup,
             "AWS::GroundStation::MissionProfile" => ResourceType::GroundStationMissionProfile,
+            "AWS::IAM::Group" => ResourceType::Group,
             "AWS::GuardDuty::Detector" => ResourceType::GuardDutyDetector,
             "AWS::GuardDuty::Filter" => ResourceType::GuardDutyFilter,
             "AWS::GuardDuty::IPSet" => ResourceType::GuardDutyIpSet,
@@ -1913,14 +1896,12 @@
             "AWS::GuardDuty::MalwareProtectionPlan" => ResourceType::GuardDutyMalwareProtectionPlan,
             "AWS::GuardDuty::ThreatIntelSet" => ResourceType::GuardDutyThreatIntelSet,
             "AWS::HealthLake::FHIRDatastore" => ResourceType::HealthLakeFhirDatastore,
-            "AWS::IAM::Group" => ResourceType::Group,
+            "AWS::EC2::Host" => ResourceType::Host,
             "AWS::IAM::InstanceProfile" => ResourceType::IamInstanceProfile,
             "AWS::IAM::OIDCProvider" => ResourceType::IamoidcProvider,
-            "AWS::IAM::Policy" => ResourceType::Policy,
-            "AWS::IAM::Role" => ResourceType::Role,
             "AWS::IAM::SAMLProvider" => ResourceType::IamsamlProvider,
             "AWS::IAM::ServerCertificate" => ResourceType::IamServerCertificate,
-            "AWS::IAM::User" => ResourceType::User,
+            "AWS::WAFv2::IPSet" => ResourceType::IpSetV2,
             "AWS::IVS::Channel" => ResourceType::IvsChannel,
             "AWS::IVS::PlaybackKeyPair" => ResourceType::IvsPlaybackKeyPair,
             "AWS::IVS::RecordingConfiguration" => ResourceType::IvsRecordingConfiguration,
@@ -1932,12 +1913,22 @@
             "AWS::ImageBuilder::LifecyclePolicy" => ResourceType::ImageBuilderLifecyclePolicy,
             "AWS::InspectorV2::Activation" => ResourceType::InspectorV2Activation,
             "AWS::InspectorV2::Filter" => ResourceType::InspectorV2Filter,
+            "AWS::EC2::Instance" => ResourceType::Instance,
+            "AWS::EC2::InternetGateway" => ResourceType::InternetGateway,
             "AWS::IoT::AccountAuditConfiguration" => ResourceType::IoTAccountAuditConfiguration,
+            "AWS::IoTAnalytics::Channel" => ResourceType::IoTAnalyticsChannel,
+            "AWS::IoTAnalytics::Dataset" => ResourceType::IoTAnalyticsDataset,
+            "AWS::IoTAnalytics::Datastore" => ResourceType::IoTAnalyticsDatastore,
+            "AWS::IoTAnalytics::Pipeline" => ResourceType::IoTAnalyticsPipeline,
             "AWS::IoT::Authorizer" => ResourceType::IoTAuthorizer,
             "AWS::IoT::CACertificate" => ResourceType::IoTcaCertificate,
+            "AWS::IoTCoreDeviceAdvisor::SuiteDefinition" => ResourceType::IoTCoreDeviceAdvisorSuiteDefinition,
             "AWS::IoT::CustomMetric" => ResourceType::IoTCustomMetric,
             "AWS::IoT::Dimension" => ResourceType::IoTDimension,
             "AWS::IoT::DomainConfiguration" => ResourceType::IoTDomainConfiguration,
+            "AWS::IoTEvents::AlarmModel" => ResourceType::IoTEventsAlarmModel,
+            "AWS::IoTEvents::DetectorModel" => ResourceType::IoTEventsDetectorModel,
+            "AWS::IoTEvents::Input" => ResourceType::IoTEventsInput,
             "AWS::IoT::FleetMetric" => ResourceType::IoTFleetMetric,
             "AWS::IoT::JobTemplate" => ResourceType::IoTJobTemplate,
             "AWS::IoT::MitigationAction" => ResourceType::IoTMitigationAction,
@@ -1946,15 +1937,6 @@
             "AWS::IoT::RoleAlias" => ResourceType::IoTRoleAlias,
             "AWS::IoT::ScheduledAudit" => ResourceType::IoTScheduledAudit,
             "AWS::IoT::SecurityProfile" => ResourceType::IoTSecurityProfile,
-            "AWS::IoT::ThingGroup" => ResourceType::IoTThingGroup,
-            "AWS::IoTAnalytics::Channel" => ResourceType::IoTAnalyticsChannel,
-            "AWS::IoTAnalytics::Dataset" => ResourceType::IoTAnalyticsDataset,
-            "AWS::IoTAnalytics::Datastore" => ResourceType::IoTAnalyticsDatastore,
-            "AWS::IoTAnalytics::Pipeline" => ResourceType::IoTAnalyticsPipeline,
-            "AWS::IoTCoreDeviceAdvisor::SuiteDefinition" => ResourceType::IoTCoreDeviceAdvisorSuiteDefinition,
-            "AWS::IoTEvents::AlarmModel" => ResourceType::IoTEventsAlarmModel,
-            "AWS::IoTEvents::DetectorModel" => ResourceType::IoTEventsDetectorModel,
-            "AWS::IoTEvents::Input" => ResourceType::IoTEventsInput,
             "AWS::IoTSiteWise::Asset" => ResourceType::IoTSiteWiseAsset,
             "AWS::IoTSiteWise::AssetModel" => ResourceType::IoTSiteWiseAssetModel,
             "AWS::IoTSiteWise::Dashboard" => ResourceType::IoTSiteWiseDashboard,
@@ -1961,6 +1943,7 @@
             "AWS::IoTSiteWise::Gateway" => ResourceType::IoTSiteWiseGateway,
             "AWS::IoTSiteWise::Portal" => ResourceType::IoTSiteWisePortal,
             "AWS::IoTSiteWise::Project" => ResourceType::IoTSiteWiseProject,
+            "AWS::IoT::ThingGroup" => ResourceType::IoTThingGroup,
             "AWS::IoTTwinMaker::ComponentType" => ResourceType::IoTTwinMakerComponentType,
             "AWS::IoTTwinMaker::Entity" => ResourceType::IoTTwinMakerEntity,
             "AWS::IoTTwinMaker::Scene" => ResourceType::IoTTwinMakerScene,
@@ -1970,18 +1953,19 @@
             "AWS::IoTWireless::MulticastGroup" => ResourceType::IoTWirelessMulticastGroup,
             "AWS::IoTWireless::ServiceProfile" => ResourceType::IoTWirelessServiceProfile,
             "AWS::KMS::Alias" => ResourceType::KmsAlias,
-            "AWS::KMS::Key" => ResourceType::Key,
             "AWS::KafkaConnect::Connector" => ResourceType::KafkaConnectConnector,
             "AWS::KafkaConnect::CustomPlugin" => ResourceType::KafkaConnectCustomPlugin,
             "AWS::Kendra::Index" => ResourceType::KendraIndex,
+            "AWS::KMS::Key" => ResourceType::Key,
+            "AWS::KinesisAnalyticsV2::Application" => ResourceType::KinesisAnalyticsV2Application,
+            "AWS::KinesisFirehose::DeliveryStream" => ResourceType::KinesisFirehoseDeliveryStream,
             "AWS::Kinesis::Stream" => ResourceType::KinesisStream,
             "AWS::Kinesis::StreamConsumer" => ResourceType::KinesisStreamConsumer,
-            "AWS::KinesisAnalyticsV2::Application" => ResourceType::KinesisAnalyticsV2Application,
-            "AWS::KinesisFirehose::DeliveryStream" => ResourceType::KinesisFirehoseDeliveryStream,
             "AWS::KinesisVideo::SignalingChannel" => ResourceType::KinesisVideoSignalingChannel,
             "AWS::KinesisVideo::Stream" => ResourceType::KinesisVideoStream,
             "AWS::Lambda::CodeSigningConfig" => ResourceType::LambdaCodeSigningConfig,
-            "AWS::Lambda::Function" => ResourceType::Function,
+            "AWS::AutoScaling::LaunchConfiguration" => ResourceType::LaunchConfiguration,
+            "AWS::EC2::LaunchTemplate" => ResourceType::LaunchTemplate,
             "AWS::Lex::Bot" => ResourceType::LexBot,
             "AWS::Lex::BotAlias" => ResourceType::LexBotAlias,
             "AWS::Lightsail::Bucket" => ResourceType::LightsailBucket,
@@ -1988,6 +1972,9 @@
             "AWS::Lightsail::Certificate" => ResourceType::LightsailCertificate,
             "AWS::Lightsail::Disk" => ResourceType::LightsailDisk,
             "AWS::Lightsail::StaticIp" => ResourceType::LightsailStaticIp,
+            "AWS::ElasticLoadBalancingV2::Listener" => ResourceType::ListenerV2,
+            "AWS::ElasticLoadBalancing::LoadBalancer" => ResourceType::LoadBalancer,
+            "AWS::ElasticLoadBalancingV2::LoadBalancer" => ResourceType::LoadBalancerV2,
             "AWS::Location::APIKey" => ResourceType::LocationApiKey,
             "AWS::Logs::Destination" => ResourceType::LogsDestination,
             "AWS::LookoutMetrics::Alert" => ResourceType::LookoutMetricsAlert,
@@ -1999,6 +1986,8 @@
             "AWS::MSK::Configuration" => ResourceType::MskConfiguration,
             "AWS::MSK::ServerlessCluster" => ResourceType::MskServerlessCluster,
             "AWS::MSK::VpcConnection" => ResourceType::MskVpcConnection,
+            "AWS::SSM::ManagedInstanceInventory" => ResourceType::ManagedInstanceInventory,
+            "AWS::WAFv2::ManagedRuleSet" => ResourceType::ManagedRuleSetV2,
             "AWS::MediaConnect::FlowEntitlement" => ResourceType::MediaConnectFlowEntitlement,
             "AWS::MediaConnect::FlowSource" => ResourceType::MediaConnectFlowSource,
             "AWS::MediaConnect::FlowVpcInterface" => ResourceType::MediaConnectFlowVpcInterface,
@@ -2010,9 +1999,13 @@
             "AWS::MediaTailor::LiveSource" => ResourceType::MediaTailorLiveSource,
             "AWS::MediaTailor::PlaybackConfiguration" => ResourceType::MediaTailorPlaybackConfiguration,
             "AWS::MemoryDB::SubnetGroup" => ResourceType::MemoryDbSubnetGroup,
+            "AWS::EC2::NatGateway" => ResourceType::NatGateway,
+            "AWS::EC2::NetworkAcl" => ResourceType::NetworkAcl,
             "AWS::NetworkFirewall::Firewall" => ResourceType::NetworkFirewallFirewall,
             "AWS::NetworkFirewall::FirewallPolicy" => ResourceType::NetworkFirewallFirewallPolicy,
             "AWS::NetworkFirewall::RuleGroup" => ResourceType::NetworkFirewallRuleGroup,
+            "AWS::EC2::NetworkInsightsAccessScopeAnalysis" => ResourceType::NetworkInsightsAccessScopeAnalysis,
+            "AWS::EC2::NetworkInterface" => ResourceType::NetworkInterface,
             "AWS::NetworkManager::ConnectPeer" => ResourceType::NetworkManagerConnectPeer,
             "AWS::NetworkManager::CustomerGatewayAssociation" => ResourceType::NetworkManagerCustomerGatewayAssociation,
             "AWS::NetworkManager::Device" => ResourceType::NetworkManagerDevice,
@@ -2030,6 +2023,7 @@
             "AWS::PCAConnectorAD::Connector" => ResourceType::PcaConnectorAdConnector,
             "AWS::PCAConnectorAD::DirectoryRegistration" => ResourceType::PcaConnectorAdDirectoryRegistration,
             "AWS::Panorama::Package" => ResourceType::PanoramaPackage,
+            "AWS::SSM::PatchCompliance" => ResourceType::PatchCompliance,
             "AWS::Personalize::Dataset" => ResourceType::PersonalizeDataset,
             "AWS::Personalize::DatasetGroup" => ResourceType::PersonalizeDatasetGroup,
             "AWS::Personalize::Schema" => ResourceType::PersonalizeSchema,
@@ -2042,37 +2036,42 @@
             "AWS::Pinpoint::EventStream" => ResourceType::PinpointEventStream,
             "AWS::Pinpoint::InAppTemplate" => ResourceType::PinpointInAppTemplate,
             "AWS::Pinpoint::Segment" => ResourceType::PinpointSegment,
+            "AWS::CodePipeline::Pipeline" => ResourceType::Pipeline,
+            "AWS::IAM::Policy" => ResourceType::Policy,
+            "AWS::ServiceCatalog::Portfolio" => ResourceType::Portfolio,
+            "AWS::CodeBuild::Project" => ResourceType::Project,
+            "AWS::Shield::Protection" => ResourceType::Protection,
             "AWS::QLDB::Ledger" => ResourceType::QldbLedger,
+            "AWS::SQS::Queue" => ResourceType::Queue,
             "AWS::QuickSight::DataSource" => ResourceType::QuickSightDataSource,
             "AWS::QuickSight::Template" => ResourceType::QuickSightTemplate,
             "AWS::QuickSight::Theme" => ResourceType::QuickSightTheme,
-            "AWS::RDS::DBCluster" => ResourceType::DbCluster,
-            "AWS::RDS::DBClusterSnapshot" => ResourceType::DbClusterSnapshot,
-            "AWS::RDS::DBInstance" => ResourceType::DbInstance,
-            "AWS::RDS::DBSecurityGroup" => ResourceType::DbSecurityGroup,
-            "AWS::RDS::DBSnapshot" => ResourceType::DbSnapshot,
-            "AWS::RDS::DBSubnetGroup" => ResourceType::DbSubnetGroup,
-            "AWS::RDS::EventSubscription" => ResourceType::EventSubscription,
             "AWS::RDS::GlobalCluster" => ResourceType::RdsGlobalCluster,
             "AWS::RDS::Integration" => ResourceType::RdsIntegration,
             "AWS::RDS::OptionGroup" => ResourceType::RdsOptionGroup,
             "AWS::RUM::AppMonitor" => ResourceType::RumAppMonitor,
-            "AWS::Redshift::Cluster" => ResourceType::Cluster,
-            "AWS::Redshift::ClusterParameterGroup" => ResourceType::ClusterParameterGroup,
-            "AWS::Redshift::ClusterSecurityGroup" => ResourceType::ClusterSecurityGroup,
-            "AWS::Redshift::ClusterSnapshot" => ResourceType::ClusterSnapshot,
-            "AWS::Redshift::ClusterSubnetGroup" => ResourceType::ClusterSubnetGroup,
+            "AWS::WAF::RateBasedRule" => ResourceType::RateBasedRule,
             "AWS::Redshift::EndpointAccess" => ResourceType::RedshiftEndpointAccess,
             "AWS::Redshift::EndpointAuthorization" => ResourceType::RedshiftEndpointAuthorization,
             "AWS::Redshift::EventSubscription" => ResourceType::RedshiftEventSubscription,
             "AWS::Redshift::Integration" => ResourceType::RedshiftIntegration,
             "AWS::Redshift::ScheduledAction" => ResourceType::RedshiftScheduledAction,
+            "AWS::WAFv2::RegexPatternSet" => ResourceType::RegexPatternSetV2,
+            "AWS::ShieldRegional::Protection" => ResourceType::RegionalProtection,
+            "AWS::WAFRegional::RateBasedRule" => ResourceType::RegionalRateBasedRule,
+            "AWS::WAFRegional::Rule" => ResourceType::RegionalRule,
+            "AWS::WAFRegional::RuleGroup" => ResourceType::RegionalRuleGroup,
+            "AWS::WAFRegional::WebACL" => ResourceType::RegionalWebAcl,
+            "AWS::EC2::RegisteredHAInstance" => ResourceType::RegisteredHaInstance,
             "AWS::ResilienceHub::App" => ResourceType::ResilienceHubApp,
             "AWS::ResilienceHub::ResiliencyPolicy" => ResourceType::ResilienceHubResiliencyPolicy,
+            "AWS::Config::ResourceCompliance" => ResourceType::ResourceCompliance,
             "AWS::ResourceExplorer2::Index" => ResourceType::ResourceExplorer2Index,
+            "AWS::ApiGateway::RestApi" => ResourceType::RestApi,
             "AWS::RoboMaker::RobotApplication" => ResourceType::RoboMakerRobotApplication,
             "AWS::RoboMaker::RobotApplicationVersion" => ResourceType::RoboMakerRobotApplicationVersion,
             "AWS::RoboMaker::SimulationApplication" => ResourceType::RoboMakerSimulationApplication,
+            "AWS::IAM::Role" => ResourceType::Role,
             "AWS::RolesAnywhere::Profile" => ResourceType::RolesAnywhereProfile,
             "AWS::RolesAnywhere::TrustAnchor" => ResourceType::RolesAnywhereTrustAnchor,
             "AWS::Route53::DNSSEC" => ResourceType::Route53Dnssec,
@@ -2095,17 +2094,19 @@
             "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation" => ResourceType::Route53ResolverResolverQueryLoggingConfigAssociation,
             "AWS::Route53Resolver::ResolverRule" => ResourceType::Route53ResolverResolverRule,
             "AWS::Route53Resolver::ResolverRuleAssociation" => ResourceType::Route53ResolverResolverRuleAssociation,
+            "AWS::EC2::RouteTable" => ResourceType::RouteTable,
+            "AWS::WAF::Rule" => ResourceType::Rule,
+            "AWS::WAF::RuleGroup" => ResourceType::RuleGroup,
+            "AWS::WAFv2::RuleGroup" => ResourceType::RuleGroupV2,
             "AWS::S3::AccessGrant" => ResourceType::S3AccessGrant,
             "AWS::S3::AccessGrantsInstance" => ResourceType::S3AccessGrantsInstance,
             "AWS::S3::AccessGrantsLocation" => ResourceType::S3AccessGrantsLocation,
             "AWS::S3::AccessPoint" => ResourceType::S3AccessPoint,
-            "AWS::S3::AccountPublicAccessBlock" => ResourceType::AccountPublicAccessBlock,
-            "AWS::S3::Bucket" => ResourceType::Bucket,
+            "AWS::S3Express::BucketPolicy" => ResourceType::S3ExpressBucketPolicy,
+            "AWS::S3Express::DirectoryBucket" => ResourceType::S3ExpressDirectoryBucket,
             "AWS::S3::MultiRegionAccessPoint" => ResourceType::S3MultiRegionAccessPoint,
             "AWS::S3::StorageLens" => ResourceType::S3StorageLens,
             "AWS::S3::StorageLensGroup" => ResourceType::S3StorageLensGroup,
-            "AWS::S3Express::BucketPolicy" => ResourceType::S3ExpressBucketPolicy,
-            "AWS::S3Express::DirectoryBucket" => ResourceType::S3ExpressDirectoryBucket,
             "AWS::S3Tables::TableBucket" => ResourceType::S3TablesTableBucket,
             "AWS::S3Tables::TableBucketPolicy" => ResourceType::S3TablesTableBucketPolicy,
             "AWS::SES::ConfigurationSet" => ResourceType::SesConfigurationSet,
@@ -2115,16 +2116,10 @@
             "AWS::SES::ReceiptFilter" => ResourceType::SesReceiptFilter,
             "AWS::SES::ReceiptRuleSet" => ResourceType::SesReceiptRuleSet,
             "AWS::SES::Template" => ResourceType::SesTemplate,
-            "AWS::SNS::Topic" => ResourceType::Topic,
-            "AWS::SQS::Queue" => ResourceType::Queue,
-            "AWS::SSM::AssociationCompliance" => ResourceType::AssociationCompliance,
-            "AWS::SSM::Document" => ResourceType::SsmDocument,
-            "AWS::SSM::FileData" => ResourceType::FileData,
-            "AWS::SSM::ManagedInstanceInventory" => ResourceType::ManagedInstanceInventory,
-            "AWS::SSM::PatchCompliance" => ResourceType::PatchCompliance,
-            "AWS::SSM::ResourceDataSync" => ResourceType::SsmResourceDataSync,
             "AWS::SSMContacts::Contact" => ResourceType::SsmContactsContact,
+            "AWS::SSM::Document" => ResourceType::SsmDocument,
             "AWS::SSMIncidents::ResponsePlan" => ResourceType::SsmIncidentsResponsePlan,
+            "AWS::SSM::ResourceDataSync" => ResourceType::SsmResourceDataSync,
             "AWS::SageMaker::AppImageConfig" => ResourceType::SageMakerAppImageConfig,
             "AWS::SageMaker::CodeRepository" => ResourceType::SageMakerCodeRepository,
             "AWS::SageMaker::DataQualityJobDefinition" => ResourceType::SageMakerDataQualityJobDefinition,
@@ -2141,22 +2136,28 @@
             "AWS::SageMaker::StudioLifecycleConfig" => ResourceType::SageMakerStudioLifecycleConfig,
             "AWS::SageMaker::UserProfile" => ResourceType::SageMakerUserProfile,
             "AWS::SageMaker::Workteam" => ResourceType::SageMakerWorkteam,
+            "AWS::AutoScaling::ScalingPolicy" => ResourceType::ScalingPolicy,
+            "AWS::AutoScaling::ScheduledAction" => ResourceType::ScheduledAction,
+            "AWS::SecretsManager::Secret" => ResourceType::Secret,
             "AWS::SecretsManager::ResourcePolicy" => ResourceType::SecretsManagerResourcePolicy,
             "AWS::SecretsManager::RotationSchedule" => ResourceType::SecretsManagerRotationSchedule,
-            "AWS::SecretsManager::Secret" => ResourceType::Secret,
+            "AWS::EC2::SecurityGroup" => ResourceType::SecurityGroup,
             "AWS::SecurityHub::Standard" => ResourceType::SecurityHubStandard,
-            "AWS::ServiceCatalog::CloudFormationProduct" => ResourceType::CloudFormationProduct,
-            "AWS::ServiceCatalog::CloudFormationProvisionedProduct" => ResourceType::CloudFormationProvisionedProduct,
-            "AWS::ServiceCatalog::Portfolio" => ResourceType::Portfolio,
             "AWS::ServiceDiscovery::HttpNamespace" => ResourceType::ServiceDiscoveryHttpNamespace,
             "AWS::ServiceDiscovery::Instance" => ResourceType::ServiceDiscoveryInstance,
             "AWS::ServiceDiscovery::PublicDnsNamespace" => ResourceType::ServiceDiscoveryPublicDnsNamespace,
             "AWS::ServiceDiscovery::Service" => ResourceType::ServiceDiscoveryService,
-            "AWS::Shield::Protection" => ResourceType::Protection,
-            "AWS::ShieldRegional::Protection" => ResourceType::RegionalProtection,
             "AWS::Signer::SigningProfile" => ResourceType::SignerSigningProfile,
+            "AWS::CloudFormation::Stack" => ResourceType::Stack,
+            "AWS::ApiGateway::Stage" => ResourceType::Stage,
+            "AWS::ApiGatewayV2::Stage" => ResourceType::StageV2,
             "AWS::StepFunctions::Activity" => ResourceType::StepFunctionsActivity,
             "AWS::StepFunctions::StateMachine" => ResourceType::StepFunctionsStateMachine,
+            "AWS::CloudFront::StreamingDistribution" => ResourceType::StreamingDistribution,
+            "AWS::EC2::Subnet" => ResourceType::Subnet,
+            "AWS::DynamoDB::Table" => ResourceType::Table,
+            "AWS::SNS::Topic" => ResourceType::Topic,
+            "AWS::CloudTrail::Trail" => ResourceType::Trail,
             "AWS::Transfer::Agreement" => ResourceType::TransferAgreement,
             "AWS::Transfer::Certificate" => ResourceType::TransferCertificate,
             "AWS::Transfer::Connector" => ResourceType::TransferConnector,
@@ -2163,22 +2164,21 @@
             "AWS::Transfer::Profile" => ResourceType::TransferProfile,
             "AWS::Transfer::Server" => ResourceType::TransferServer,
             "AWS::Transfer::Workflow" => ResourceType::TransferWorkflow,
-            "AWS::WAF::RateBasedRule" => ResourceType::RateBasedRule,
-            "AWS::WAF::Rule" => ResourceType::Rule,
-            "AWS::WAF::RuleGroup" => ResourceType::RuleGroup,
+            "AWS::EC2::TransitGateway" => ResourceType::TransitGateway,
+            "AWS::EC2::TransitGatewayAttachment" => ResourceType::TransitGatewayAttachment,
+            "AWS::EC2::TransitGatewayRouteTable" => ResourceType::TransitGatewayRouteTable,
+            "AWS::IAM::User" => ResourceType::User,
+            "AWS::EC2::VPC" => ResourceType::Vpc,
+            "AWS::EC2::VPCEndpoint" => ResourceType::VpcEndpoint,
+            "AWS::EC2::VPCEndpointService" => ResourceType::VpcEndpointService,
+            "AWS::EC2::VPCPeeringConnection" => ResourceType::VpcPeeringConnection,
+            "AWS::EC2::VPNConnection" => ResourceType::VpnConnection,
+            "AWS::EC2::VPNGateway" => ResourceType::VpnGateway,
+            "AWS::EC2::Volume" => ResourceType::Volume,
             "AWS::WAF::WebACL" => ResourceType::WebAcl,
-            "AWS::WAFRegional::RateBasedRule" => ResourceType::RegionalRateBasedRule,
-            "AWS::WAFRegional::Rule" => ResourceType::RegionalRule,
-            "AWS::WAFRegional::RuleGroup" => ResourceType::RegionalRuleGroup,
-            "AWS::WAFRegional::WebACL" => ResourceType::RegionalWebAcl,
-            "AWS::WAFv2::IPSet" => ResourceType::IpSetV2,
-            "AWS::WAFv2::ManagedRuleSet" => ResourceType::ManagedRuleSetV2,
-            "AWS::WAFv2::RegexPatternSet" => ResourceType::RegexPatternSetV2,
-            "AWS::WAFv2::RuleGroup" => ResourceType::RuleGroupV2,
             "AWS::WAFv2::WebACL" => ResourceType::WebAclv2,
             "AWS::WorkSpaces::ConnectionAlias" => ResourceType::WorkSpacesConnectionAlias,
             "AWS::WorkSpaces::Workspace" => ResourceType::WorkSpacesWorkspace,
-            "AWS::XRay::EncryptionConfig" => ResourceType::EncryptionConfig,
             other => ResourceType::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -2194,21 +2194,19 @@
     /// Returns the `&str` value of the enum member.
     pub fn as_str(&self) -> &str {
         match self {
-            ResourceType::Certificate => "AWS::ACM::Certificate",
             ResourceType::AcmpcaCertificateAuthority => "AWS::ACMPCA::CertificateAuthority",
             ResourceType::AcmpcaCertificateAuthorityActivation => "AWS::ACMPCA::CertificateAuthorityActivation",
             ResourceType::ApsRuleGroupsNamespace => "AWS::APS::RuleGroupsNamespace",
             ResourceType::AccessAnalyzerAnalyzer => "AWS::AccessAnalyzer::Analyzer",
+            ResourceType::AccountPublicAccessBlock => "AWS::S3::AccountPublicAccessBlock",
+            ResourceType::Alarm => "AWS::CloudWatch::Alarm",
             ResourceType::AmazonMqBroker => "AWS::AmazonMQ::Broker",
             ResourceType::AmplifyApp => "AWS::Amplify::App",
             ResourceType::AmplifyBranch => "AWS::Amplify::Branch",
+            ResourceType::Api => "AWS::ApiGatewayV2::Api",
             ResourceType::ApiGatewayMethod => "AWS::ApiGateway::Method",
-            ResourceType::RestApi => "AWS::ApiGateway::RestApi",
-            ResourceType::Stage => "AWS::ApiGateway::Stage",
             ResourceType::ApiGatewayUsagePlan => "AWS::ApiGateway::UsagePlan",
-            ResourceType::Api => "AWS::ApiGatewayV2::Api",
             ResourceType::ApiGatewayV2Integration => "AWS::ApiGatewayV2::Integration",
-            ResourceType::StageV2 => "AWS::ApiGatewayV2::Stage",
             ResourceType::AppConfigApplication => "AWS::AppConfig::Application",
             ResourceType::AppConfigConfigurationProfile => "AWS::AppConfig::ConfigurationProfile",
             ResourceType::AppConfigDeploymentStrategy => "AWS::AppConfig::DeploymentStrategy",
@@ -2235,53 +2233,56 @@
             ResourceType::AppStreamStack => "AWS::AppStream::Stack",
             ResourceType::AppSyncApiCache => "AWS::AppSync::ApiCache",
             ResourceType::AppSyncGraphQlApi => "AWS::AppSync::GraphQLApi",
+            ResourceType::Application => "AWS::ElasticBeanstalk::Application",
+            ResourceType::ApplicationVersion => "AWS::ElasticBeanstalk::ApplicationVersion",
+            ResourceType::AssociationCompliance => "AWS::SSM::AssociationCompliance",
             ResourceType::AthenaDataCatalog => "AWS::Athena::DataCatalog",
             ResourceType::AthenaPreparedStatement => "AWS::Athena::PreparedStatement",
             ResourceType::AthenaWorkGroup => "AWS::Athena::WorkGroup",
             ResourceType::AuditManagerAssessment => "AWS::AuditManager::Assessment",
             ResourceType::AutoScalingGroup => "AWS::AutoScaling::AutoScalingGroup",
-            ResourceType::LaunchConfiguration => "AWS::AutoScaling::LaunchConfiguration",
-            ResourceType::ScalingPolicy => "AWS::AutoScaling::ScalingPolicy",
-            ResourceType::ScheduledAction => "AWS::AutoScaling::ScheduledAction",
             ResourceType::AutoScalingWarmPool => "AWS::AutoScaling::WarmPool",
             ResourceType::B2BiCapability => "AWS::B2BI::Capability",
             ResourceType::BcmDataExportsExport => "AWS::BCMDataExports::Export",
+            ResourceType::BackupGatewayHypervisor => "AWS::BackupGateway::Hypervisor",
             ResourceType::BackupPlan => "AWS::Backup::BackupPlan",
-            ResourceType::BackupSelection => "AWS::Backup::BackupSelection",
-            ResourceType::BackupVault => "AWS::Backup::BackupVault",
             ResourceType::BackupRecoveryPoint => "AWS::Backup::RecoveryPoint",
             ResourceType::BackupReportPlan => "AWS::Backup::ReportPlan",
             ResourceType::BackupRestoreTestingPlan => "AWS::Backup::RestoreTestingPlan",
-            ResourceType::BackupGatewayHypervisor => "AWS::BackupGateway::Hypervisor",
+            ResourceType::BackupSelection => "AWS::Backup::BackupSelection",
+            ResourceType::BackupVault => "AWS::Backup::BackupVault",
             ResourceType::BatchComputeEnvironment => "AWS::Batch::ComputeEnvironment",
             ResourceType::BatchJobQueue => "AWS::Batch::JobQueue",
             ResourceType::BatchSchedulingPolicy => "AWS::Batch::SchedulingPolicy",
+            ResourceType::BedrockAgentCoreBrowserCustom => "AWS::BedrockAgentCore::BrowserCustom",
+            ResourceType::BedrockAgentCoreRuntime => "AWS::BedrockAgentCore::Runtime",
             ResourceType::BedrockApplicationInferenceProfile => "AWS::Bedrock::ApplicationInferenceProfile",
             ResourceType::BedrockGuardrail => "AWS::Bedrock::Guardrail",
             ResourceType::BedrockKnowledgeBase => "AWS::Bedrock::KnowledgeBase",
             ResourceType::BedrockPrompt => "AWS::Bedrock::Prompt",
-            ResourceType::BedrockAgentCoreBrowserCustom => "AWS::BedrockAgentCore::BrowserCustom",
-            ResourceType::BedrockAgentCoreRuntime => "AWS::BedrockAgentCore::Runtime",
+            ResourceType::Bucket => "AWS::S3::Bucket",
             ResourceType::BudgetsBudgetsAction => "AWS::Budgets::BudgetsAction",
             ResourceType::CassandraKeyspace => "AWS::Cassandra::Keyspace",
+            ResourceType::Certificate => "AWS::ACM::Certificate",
             ResourceType::CleanRoomsMlTrainingDataset => "AWS::CleanRoomsML::TrainingDataset",
             ResourceType::Cloud9EnvironmentEc2 => "AWS::Cloud9::EnvironmentEC2",
             ResourceType::CloudFormationGuardHook => "AWS::CloudFormation::GuardHook",
             ResourceType::CloudFormationLambdaHook => "AWS::CloudFormation::LambdaHook",
-            ResourceType::Stack => "AWS::CloudFormation::Stack",
+            ResourceType::CloudFormationProduct => "AWS::ServiceCatalog::CloudFormationProduct",
+            ResourceType::CloudFormationProvisionedProduct => "AWS::ServiceCatalog::CloudFormationProvisionedProduct",
             ResourceType::CloudFormationStackSet => "AWS::CloudFormation::StackSet",
-            ResourceType::Distribution => "AWS::CloudFront::Distribution",
             ResourceType::CloudFrontKeyValueStore => "AWS::CloudFront::KeyValueStore",
             ResourceType::CloudFrontPublicKey => "AWS::CloudFront::PublicKey",
             ResourceType::CloudFrontRealtimeLogConfig => "AWS::CloudFront::RealtimeLogConfig",
-            ResourceType::StreamingDistribution => "AWS::CloudFront::StreamingDistribution",
             ResourceType::CloudTrailEventDataStore => "AWS::CloudTrail::EventDataStore",
-            ResourceType::Trail => "AWS::CloudTrail::Trail",
-            ResourceType::Alarm => "AWS::CloudWatch::Alarm",
             ResourceType::CloudWatchMetricStream => "AWS::CloudWatch::MetricStream",
+            ResourceType::Cluster => "AWS::Redshift::Cluster",
+            ResourceType::ClusterParameterGroup => "AWS::Redshift::ClusterParameterGroup",
+            ResourceType::ClusterSecurityGroup => "AWS::Redshift::ClusterSecurityGroup",
+            ResourceType::ClusterSnapshot => "AWS::Redshift::ClusterSnapshot",
+            ResourceType::ClusterSubnetGroup => "AWS::Redshift::ClusterSubnetGroup",
             ResourceType::CodeArtifactDomain => "AWS::CodeArtifact::Domain",
             ResourceType::CodeArtifactRepository => "AWS::CodeArtifact::Repository",
-            ResourceType::Project => "AWS::CodeBuild::Project",
             ResourceType::CodeBuildReportGroup => "AWS::CodeBuild::ReportGroup",
             ResourceType::CodeDeployApplication => "AWS::CodeDeploy::Application",
             ResourceType::CodeDeployDeploymentConfig => "AWS::CodeDeploy::DeploymentConfig",
@@ -2288,7 +2289,6 @@
             ResourceType::CodeDeployDeploymentGroup => "AWS::CodeDeploy::DeploymentGroup",
             ResourceType::CodeGuruProfilerProfilingGroup => "AWS::CodeGuruProfiler::ProfilingGroup",
             ResourceType::CodeGuruReviewerRepositoryAssociation => "AWS::CodeGuruReviewer::RepositoryAssociation",
-            ResourceType::Pipeline => "AWS::CodePipeline::Pipeline",
             ResourceType::CognitoIdentityPool => "AWS::Cognito::IdentityPool",
             ResourceType::CognitoUserPool => "AWS::Cognito::UserPool",
             ResourceType::CognitoUserPoolClient => "AWS::Cognito::UserPoolClient",
@@ -2296,9 +2296,8 @@
             ResourceType::ComprehendFlywheel => "AWS::Comprehend::Flywheel",
             ResourceType::ConfigAggregationAuthorization => "AWS::Config::AggregationAuthorization",
             ResourceType::ConfigConformancePack => "AWS::Config::ConformancePack",
+            ResourceType::ConfigStoredQuery => "AWS::Config::StoredQuery",
             ResourceType::ConformancePackCompliance => "AWS::Config::ConformancePackCompliance",
-            ResourceType::ResourceCompliance => "AWS::Config::ResourceCompliance",
-            ResourceType::ConfigStoredQuery => "AWS::Config::StoredQuery",
             ResourceType::ConnectInstance => "AWS::Connect::Instance",
             ResourceType::ConnectPhoneNumber => "AWS::Connect::PhoneNumber",
             ResourceType::ConnectQuickConnect => "AWS::Connect::QuickConnect",
@@ -2305,8 +2304,15 @@
             ResourceType::ConnectRule => "AWS::Connect::Rule",
             ResourceType::ConnectSecurityProfile => "AWS::Connect::SecurityProfile",
             ResourceType::ConnectUser => "AWS::Connect::User",
+            ResourceType::CustomerGateway => "AWS::EC2::CustomerGateway",
             ResourceType::CustomerProfilesDomain => "AWS::CustomerProfiles::Domain",
             ResourceType::CustomerProfilesObjectType => "AWS::CustomerProfiles::ObjectType",
+            ResourceType::DbCluster => "AWS::RDS::DBCluster",
+            ResourceType::DbClusterSnapshot => "AWS::RDS::DBClusterSnapshot",
+            ResourceType::DbInstance => "AWS::RDS::DBInstance",
+            ResourceType::DbSecurityGroup => "AWS::RDS::DBSecurityGroup",
+            ResourceType::DbSnapshot => "AWS::RDS::DBSnapshot",
+            ResourceType::DbSubnetGroup => "AWS::RDS::DBSubnetGroup",
             ResourceType::DmsCertificate => "AWS::DMS::Certificate",
             ResourceType::DmsEndpoint => "AWS::DMS::Endpoint",
             ResourceType::DmsEventSubscription => "AWS::DMS::EventSubscription",
@@ -2328,19 +2334,15 @@
             ResourceType::DeviceFarmInstanceProfile => "AWS::DeviceFarm::InstanceProfile",
             ResourceType::DeviceFarmProject => "AWS::DeviceFarm::Project",
             ResourceType::DeviceFarmTestGridProject => "AWS::DeviceFarm::TestGridProject",
-            ResourceType::Table => "AWS::DynamoDB::Table",
+            ResourceType::Distribution => "AWS::CloudFront::Distribution",
+            ResourceType::Domain => "AWS::Elasticsearch::Domain",
             ResourceType::Ec2CapacityReservation => "AWS::EC2::CapacityReservation",
             ResourceType::Ec2CarrierGateway => "AWS::EC2::CarrierGateway",
             ResourceType::Ec2ClientVpnEndpoint => "AWS::EC2::ClientVpnEndpoint",
             ResourceType::Ec2ClientVpnTargetNetworkAssociation => "AWS::EC2::ClientVpnTargetNetworkAssociation",
-            ResourceType::CustomerGateway => "AWS::EC2::CustomerGateway",
             ResourceType::Ec2DhcpOptions => "AWS::EC2::DHCPOptions",
             ResourceType::Ec2Ec2Fleet => "AWS::EC2::EC2Fleet",
-            ResourceType::Eip => "AWS::EC2::EIP",
             ResourceType::Ec2EipAssociation => "AWS::EC2::EIPAssociation",
-            ResourceType::EgressOnlyInternetGateway => "AWS::EC2::EgressOnlyInternetGateway",
-            ResourceType::FlowLog => "AWS::EC2::FlowLog",
-            ResourceType::Host => "AWS::EC2::Host",
             ResourceType::Ec2Ipam => "AWS::EC2::IPAM",
             ResourceType::Ec2IpamPool => "AWS::EC2::IPAMPool",
             ResourceType::Ec2IpamPoolCidr => "AWS::EC2::IPAMPoolCidr",
@@ -2347,25 +2349,14 @@
             ResourceType::Ec2IpamResourceDiscovery => "AWS::EC2::IPAMResourceDiscovery",
             ResourceType::Ec2IpamResourceDiscoveryAssociation => "AWS::EC2::IPAMResourceDiscoveryAssociation",
             ResourceType::Ec2IpamScope => "AWS::EC2::IPAMScope",
-            ResourceType::Instance => "AWS::EC2::Instance",
             ResourceType::Ec2InstanceConnectEndpoint => "AWS::EC2::InstanceConnectEndpoint",
-            ResourceType::InternetGateway => "AWS::EC2::InternetGateway",
-            ResourceType::LaunchTemplate => "AWS::EC2::LaunchTemplate",
-            ResourceType::NatGateway => "AWS::EC2::NatGateway",
-            ResourceType::NetworkAcl => "AWS::EC2::NetworkAcl",
             ResourceType::Ec2NetworkInsightsAccessScope => "AWS::EC2::NetworkInsightsAccessScope",
-            ResourceType::NetworkInsightsAccessScopeAnalysis => "AWS::EC2::NetworkInsightsAccessScopeAnalysis",
             ResourceType::Ec2NetworkInsightsAnalysis => "AWS::EC2::NetworkInsightsAnalysis",
             ResourceType::Ec2NetworkInsightsPath => "AWS::EC2::NetworkInsightsPath",
-            ResourceType::NetworkInterface => "AWS::EC2::NetworkInterface",
             ResourceType::Ec2PrefixList => "AWS::EC2::PrefixList",
-            ResourceType::RegisteredHaInstance => "AWS::EC2::RegisteredHAInstance",
-            ResourceType::RouteTable => "AWS::EC2::RouteTable",
-            ResourceType::SecurityGroup => "AWS::EC2::SecurityGroup",
             ResourceType::Ec2SecurityGroupVpcAssociation => "AWS::EC2::SecurityGroupVpcAssociation",
             ResourceType::Ec2SnapshotBlockPublicAccess => "AWS::EC2::SnapshotBlockPublicAccess",
             ResourceType::Ec2SpotFleet => "AWS::EC2::SpotFleet",
-            ResourceType::Subnet => "AWS::EC2::Subnet",
             ResourceType::Ec2SubnetCidrBlock => "AWS::EC2::SubnetCidrBlock",
             ResourceType::Ec2SubnetNetworkAclAssociation => "AWS::EC2::SubnetNetworkAclAssociation",
             ResourceType::Ec2SubnetRouteTableAssociation => "AWS::EC2::SubnetRouteTableAssociation",
@@ -2372,24 +2363,14 @@
             ResourceType::Ec2TrafficMirrorFilter => "AWS::EC2::TrafficMirrorFilter",
             ResourceType::Ec2TrafficMirrorSession => "AWS::EC2::TrafficMirrorSession",
             ResourceType::Ec2TrafficMirrorTarget => "AWS::EC2::TrafficMirrorTarget",
-            ResourceType::TransitGateway => "AWS::EC2::TransitGateway",
-            ResourceType::TransitGatewayAttachment => "AWS::EC2::TransitGatewayAttachment",
             ResourceType::Ec2TransitGatewayConnect => "AWS::EC2::TransitGatewayConnect",
             ResourceType::Ec2TransitGatewayMulticastDomain => "AWS::EC2::TransitGatewayMulticastDomain",
-            ResourceType::TransitGatewayRouteTable => "AWS::EC2::TransitGatewayRouteTable",
-            ResourceType::Vpc => "AWS::EC2::VPC",
             ResourceType::Ec2VpcBlockPublicAccessExclusion => "AWS::EC2::VPCBlockPublicAccessExclusion",
             ResourceType::Ec2VpcBlockPublicAccessOptions => "AWS::EC2::VPCBlockPublicAccessOptions",
-            ResourceType::VpcEndpoint => "AWS::EC2::VPCEndpoint",
             ResourceType::Ec2VpcEndpointConnectionNotification => "AWS::EC2::VPCEndpointConnectionNotification",
-            ResourceType::VpcEndpointService => "AWS::EC2::VPCEndpointService",
             ResourceType::Ec2VpcGatewayAttachment => "AWS::EC2::VPCGatewayAttachment",
-            ResourceType::VpcPeeringConnection => "AWS::EC2::VPCPeeringConnection",
-            ResourceType::VpnConnection => "AWS::EC2::VPNConnection",
             ResourceType::Ec2VpnConnectionRoute => "AWS::EC2::VPNConnectionRoute",
-            ResourceType::VpnGateway => "AWS::EC2::VPNGateway",
             ResourceType::Ec2VerifiedAccessInstance => "AWS::EC2::VerifiedAccessInstance",
-            ResourceType::Volume => "AWS::EC2::Volume",
             ResourceType::EcrPublicRepository => "AWS::ECR::PublicRepository",
             ResourceType::EcrPullThroughCacheRule => "AWS::ECR::PullThroughCacheRule",
             ResourceType::EcrRegistryPolicy => "AWS::ECR::RegistryPolicy",
@@ -2403,29 +2384,27 @@
             ResourceType::EcsTaskSet => "AWS::ECS::TaskSet",
             ResourceType::EfsAccessPoint => "AWS::EFS::AccessPoint",
             ResourceType::EfsFileSystem => "AWS::EFS::FileSystem",
+            ResourceType::Eip => "AWS::EC2::EIP",
             ResourceType::EksAddon => "AWS::EKS::Addon",
             ResourceType::EksCluster => "AWS::EKS::Cluster",
             ResourceType::EksFargateProfile => "AWS::EKS::FargateProfile",
             ResourceType::EksIdentityProviderConfig => "AWS::EKS::IdentityProviderConfig",
+            ResourceType::EmrContainersVirtualCluster => "AWS::EMRContainers::VirtualCluster",
             ResourceType::EmrSecurityConfiguration => "AWS::EMR::SecurityConfiguration",
-            ResourceType::EmrStudio => "AWS::EMR::Studio",
-            ResourceType::EmrContainersVirtualCluster => "AWS::EMRContainers::VirtualCluster",
             ResourceType::EmrServerlessApplication => "AWS::EMRServerless::Application",
-            ResourceType::Application => "AWS::ElasticBeanstalk::Application",
-            ResourceType::ApplicationVersion => "AWS::ElasticBeanstalk::ApplicationVersion",
-            ResourceType::Environment => "AWS::ElasticBeanstalk::Environment",
-            ResourceType::LoadBalancer => "AWS::ElasticLoadBalancing::LoadBalancer",
-            ResourceType::ListenerV2 => "AWS::ElasticLoadBalancingV2::Listener",
-            ResourceType::LoadBalancerV2 => "AWS::ElasticLoadBalancingV2::LoadBalancer",
+            ResourceType::EmrStudio => "AWS::EMR::Studio",
+            ResourceType::EgressOnlyInternetGateway => "AWS::EC2::EgressOnlyInternetGateway",
             ResourceType::ElasticLoadBalancingV2TargetGroup => "AWS::ElasticLoadBalancingV2::TargetGroup",
-            ResourceType::Domain => "AWS::Elasticsearch::Domain",
+            ResourceType::EncryptionConfig => "AWS::XRay::EncryptionConfig",
             ResourceType::EntityResolutionIdMappingWorkflow => "AWS::EntityResolution::IdMappingWorkflow",
             ResourceType::EntityResolutionMatchingWorkflow => "AWS::EntityResolution::MatchingWorkflow",
             ResourceType::EntityResolutionSchemaMapping => "AWS::EntityResolution::SchemaMapping",
+            ResourceType::Environment => "AWS::ElasticBeanstalk::Environment",
             ResourceType::EventSchemasDiscoverer => "AWS::EventSchemas::Discoverer",
             ResourceType::EventSchemasRegistry => "AWS::EventSchemas::Registry",
             ResourceType::EventSchemasRegistryPolicy => "AWS::EventSchemas::RegistryPolicy",
             ResourceType::EventSchemasSchema => "AWS::EventSchemas::Schema",
+            ResourceType::EventSubscription => "AWS::RDS::EventSubscription",
             ResourceType::EventsApiDestination => "AWS::Events::ApiDestination",
             ResourceType::EventsArchive => "AWS::Events::Archive",
             ResourceType::EventsConnection => "AWS::Events::Connection",
@@ -2436,6 +2415,8 @@
             ResourceType::EvidentlyProject => "AWS::Evidently::Project",
             ResourceType::EvidentlySegment => "AWS::Evidently::Segment",
             ResourceType::FisExperimentTemplate => "AWS::FIS::ExperimentTemplate",
+            ResourceType::FileData => "AWS::SSM::FileData",
+            ResourceType::FlowLog => "AWS::EC2::FlowLog",
             ResourceType::ForecastDataset => "AWS::Forecast::Dataset",
             ResourceType::ForecastDatasetGroup => "AWS::Forecast::DatasetGroup",
             ResourceType::FraudDetectorEntityType => "AWS::FraudDetector::EntityType",
@@ -2442,6 +2423,7 @@
             ResourceType::FraudDetectorLabel => "AWS::FraudDetector::Label",
             ResourceType::FraudDetectorOutcome => "AWS::FraudDetector::Outcome",
             ResourceType::FraudDetectorVariable => "AWS::FraudDetector::Variable",
+            ResourceType::Function => "AWS::Lambda::Function",
             ResourceType::GameLiftBuild => "AWS::GameLift::Build",
             ResourceType::GlobalAcceleratorAccelerator => "AWS::GlobalAccelerator::Accelerator",
             ResourceType::GlobalAcceleratorEndpointGroup => "AWS::GlobalAccelerator::EndpointGroup",
@@ -2455,6 +2437,7 @@
             ResourceType::GroundStationConfig => "AWS::GroundStation::Config",
             ResourceType::GroundStationDataflowEndpointGroup => "AWS::GroundStation::DataflowEndpointGroup",
             ResourceType::GroundStationMissionProfile => "AWS::GroundStation::MissionProfile",
+            ResourceType::Group => "AWS::IAM::Group",
             ResourceType::GuardDutyDetector => "AWS::GuardDuty::Detector",
             ResourceType::GuardDutyFilter => "AWS::GuardDuty::Filter",
             ResourceType::GuardDutyIpSet => "AWS::GuardDuty::IPSet",
@@ -2461,14 +2444,12 @@
             ResourceType::GuardDutyMalwareProtectionPlan => "AWS::GuardDuty::MalwareProtectionPlan",
             ResourceType::GuardDutyThreatIntelSet => "AWS::GuardDuty::ThreatIntelSet",
             ResourceType::HealthLakeFhirDatastore => "AWS::HealthLake::FHIRDatastore",
-            ResourceType::Group => "AWS::IAM::Group",
+            ResourceType::Host => "AWS::EC2::Host",
             ResourceType::IamInstanceProfile => "AWS::IAM::InstanceProfile",
             ResourceType::IamoidcProvider => "AWS::IAM::OIDCProvider",
-            ResourceType::Policy => "AWS::IAM::Policy",
-            ResourceType::Role => "AWS::IAM::Role",
             ResourceType::IamsamlProvider => "AWS::IAM::SAMLProvider",
             ResourceType::IamServerCertificate => "AWS::IAM::ServerCertificate",
-            ResourceType::User => "AWS::IAM::User",
+            ResourceType::IpSetV2 => "AWS::WAFv2::IPSet",
             ResourceType::IvsChannel => "AWS::IVS::Channel",
             ResourceType::IvsPlaybackKeyPair => "AWS::IVS::PlaybackKeyPair",
             ResourceType::IvsRecordingConfiguration => "AWS::IVS::RecordingConfiguration",
@@ -2480,12 +2461,22 @@
             ResourceType::ImageBuilderLifecyclePolicy => "AWS::ImageBuilder::LifecyclePolicy",
             ResourceType::InspectorV2Activation => "AWS::InspectorV2::Activation",
             ResourceType::InspectorV2Filter => "AWS::InspectorV2::Filter",
+            ResourceType::Instance => "AWS::EC2::Instance",
+            ResourceType::InternetGateway => "AWS::EC2::InternetGateway",
             ResourceType::IoTAccountAuditConfiguration => "AWS::IoT::AccountAuditConfiguration",
+            ResourceType::IoTAnalyticsChannel => "AWS::IoTAnalytics::Channel",
+            ResourceType::IoTAnalyticsDataset => "AWS::IoTAnalytics::Dataset",
+            ResourceType::IoTAnalyticsDatastore => "AWS::IoTAnalytics::Datastore",
+            ResourceType::IoTAnalyticsPipeline => "AWS::IoTAnalytics::Pipeline",
             ResourceType::IoTAuthorizer => "AWS::IoT::Authorizer",
             ResourceType::IoTcaCertificate => "AWS::IoT::CACertificate",
+            ResourceType::IoTCoreDeviceAdvisorSuiteDefinition => "AWS::IoTCoreDeviceAdvisor::SuiteDefinition",
             ResourceType::IoTCustomMetric => "AWS::IoT::CustomMetric",
             ResourceType::IoTDimension => "AWS::IoT::Dimension",
             ResourceType::IoTDomainConfiguration => "AWS::IoT::DomainConfiguration",
+            ResourceType::IoTEventsAlarmModel => "AWS::IoTEvents::AlarmModel",
+            ResourceType::IoTEventsDetectorModel => "AWS::IoTEvents::DetectorModel",
+            ResourceType::IoTEventsInput => "AWS::IoTEvents::Input",
             ResourceType::IoTFleetMetric => "AWS::IoT::FleetMetric",
             ResourceType::IoTJobTemplate => "AWS::IoT::JobTemplate",
             ResourceType::IoTMitigationAction => "AWS::IoT::MitigationAction",
@@ -2494,15 +2485,6 @@
             ResourceType::IoTRoleAlias => "AWS::IoT::RoleAlias",
             ResourceType::IoTScheduledAudit => "AWS::IoT::ScheduledAudit",
             ResourceType::IoTSecurityProfile => "AWS::IoT::SecurityProfile",
-            ResourceType::IoTThingGroup => "AWS::IoT::ThingGroup",
-            ResourceType::IoTAnalyticsChannel => "AWS::IoTAnalytics::Channel",
-            ResourceType::IoTAnalyticsDataset => "AWS::IoTAnalytics::Dataset",
-            ResourceType::IoTAnalyticsDatastore => "AWS::IoTAnalytics::Datastore",
-            ResourceType::IoTAnalyticsPipeline => "AWS::IoTAnalytics::Pipeline",
-            ResourceType::IoTCoreDeviceAdvisorSuiteDefinition => "AWS::IoTCoreDeviceAdvisor::SuiteDefinition",
-            ResourceType::IoTEventsAlarmModel => "AWS::IoTEvents::AlarmModel",
-            ResourceType::IoTEventsDetectorModel => "AWS::IoTEvents::DetectorModel",
-            ResourceType::IoTEventsInput => "AWS::IoTEvents::Input",
             ResourceType::IoTSiteWiseAsset => "AWS::IoTSiteWise::Asset",
             ResourceType::IoTSiteWiseAssetModel => "AWS::IoTSiteWise::AssetModel",
             ResourceType::IoTSiteWiseDashboard => "AWS::IoTSiteWise::Dashboard",
@@ -2509,6 +2491,7 @@
             ResourceType::IoTSiteWiseGateway => "AWS::IoTSiteWise::Gateway",
             ResourceType::IoTSiteWisePortal => "AWS::IoTSiteWise::Portal",
             ResourceType::IoTSiteWiseProject => "AWS::IoTSiteWise::Project",
+            ResourceType::IoTThingGroup => "AWS::IoT::ThingGroup",
             ResourceType::IoTTwinMakerComponentType => "AWS::IoTTwinMaker::ComponentType",
             ResourceType::IoTTwinMakerEntity => "AWS::IoTTwinMaker::Entity",
             ResourceType::IoTTwinMakerScene => "AWS::IoTTwinMaker::Scene",
@@ -2518,18 +2501,19 @@
             ResourceType::IoTWirelessMulticastGroup => "AWS::IoTWireless::MulticastGroup",
             ResourceType::IoTWirelessServiceProfile => "AWS::IoTWireless::ServiceProfile",
             ResourceType::KmsAlias => "AWS::KMS::Alias",
-            ResourceType::Key => "AWS::KMS::Key",
             ResourceType::KafkaConnectConnector => "AWS::KafkaConnect::Connector",
             ResourceType::KafkaConnectCustomPlugin => "AWS::KafkaConnect::CustomPlugin",
             ResourceType::KendraIndex => "AWS::Kendra::Index",
+            ResourceType::Key => "AWS::KMS::Key",
+            ResourceType::KinesisAnalyticsV2Application => "AWS::KinesisAnalyticsV2::Application",
+            ResourceType::KinesisFirehoseDeliveryStream => "AWS::KinesisFirehose::DeliveryStream",
             ResourceType::KinesisStream => "AWS::Kinesis::Stream",
             ResourceType::KinesisStreamConsumer => "AWS::Kinesis::StreamConsumer",
-            ResourceType::KinesisAnalyticsV2Application => "AWS::KinesisAnalyticsV2::Application",
-            ResourceType::KinesisFirehoseDeliveryStream => "AWS::KinesisFirehose::DeliveryStream",
             ResourceType::KinesisVideoSignalingChannel => "AWS::KinesisVideo::SignalingChannel",
             ResourceType::KinesisVideoStream => "AWS::KinesisVideo::Stream",
             ResourceType::LambdaCodeSigningConfig => "AWS::Lambda::CodeSigningConfig",
-            ResourceType::Function => "AWS::Lambda::Function",
+            ResourceType::LaunchConfiguration => "AWS::AutoScaling::LaunchConfiguration",
+            ResourceType::LaunchTemplate => "AWS::EC2::LaunchTemplate",
             ResourceType::LexBot => "AWS::Lex::Bot",
             ResourceType::LexBotAlias => "AWS::Lex::BotAlias",
             ResourceType::LightsailBucket => "AWS::Lightsail::Bucket",
@@ -2536,6 +2520,9 @@
             ResourceType::LightsailCertificate => "AWS::Lightsail::Certificate",
             ResourceType::LightsailDisk => "AWS::Lightsail::Disk",
             ResourceType::LightsailStaticIp => "AWS::Lightsail::StaticIp",
+            ResourceType::ListenerV2 => "AWS::ElasticLoadBalancingV2::Listener",
+            ResourceType::LoadBalancer => "AWS::ElasticLoadBalancing::LoadBalancer",
+            ResourceType::LoadBalancerV2 => "AWS::ElasticLoadBalancingV2::LoadBalancer",
             ResourceType::LocationApiKey => "AWS::Location::APIKey",
             ResourceType::LogsDestination => "AWS::Logs::Destination",
             ResourceType::LookoutMetricsAlert => "AWS::LookoutMetrics::Alert",
@@ -2547,6 +2534,8 @@
             ResourceType::MskConfiguration => "AWS::MSK::Configuration",
             ResourceType::MskServerlessCluster => "AWS::MSK::ServerlessCluster",
             ResourceType::MskVpcConnection => "AWS::MSK::VpcConnection",
+            ResourceType::ManagedInstanceInventory => "AWS::SSM::ManagedInstanceInventory",
+            ResourceType::ManagedRuleSetV2 => "AWS::WAFv2::ManagedRuleSet",
             ResourceType::MediaConnectFlowEntitlement => "AWS::MediaConnect::FlowEntitlement",
             ResourceType::MediaConnectFlowSource => "AWS::MediaConnect::FlowSource",
             ResourceType::MediaConnectFlowVpcInterface => "AWS::MediaConnect::FlowVpcInterface",
@@ -2558,9 +2547,13 @@
             ResourceType::MediaTailorLiveSource => "AWS::MediaTailor::LiveSource",
             ResourceType::MediaTailorPlaybackConfiguration => "AWS::MediaTailor::PlaybackConfiguration",
             ResourceType::MemoryDbSubnetGroup => "AWS::MemoryDB::SubnetGroup",
+            ResourceType::NatGateway => "AWS::EC2::NatGateway",
+            ResourceType::NetworkAcl => "AWS::EC2::NetworkAcl",
             ResourceType::NetworkFirewallFirewall => "AWS::NetworkFirewall::Firewall",
             ResourceType::NetworkFirewallFirewallPolicy => "AWS::NetworkFirewall::FirewallPolicy",
             ResourceType::NetworkFirewallRuleGroup => "AWS::NetworkFirewall::RuleGroup",
+            ResourceType::NetworkInsightsAccessScopeAnalysis => "AWS::EC2::NetworkInsightsAccessScopeAnalysis",
+            ResourceType::NetworkInterface => "AWS::EC2::NetworkInterface",
             ResourceType::NetworkManagerConnectPeer => "AWS::NetworkManager::ConnectPeer",
             ResourceType::NetworkManagerCustomerGatewayAssociation => "AWS::NetworkManager::CustomerGatewayAssociation",
             ResourceType::NetworkManagerDevice => "AWS::NetworkManager::Device",
@@ -2578,6 +2571,7 @@
             ResourceType::PcaConnectorAdConnector => "AWS::PCAConnectorAD::Connector",
             ResourceType::PcaConnectorAdDirectoryRegistration => "AWS::PCAConnectorAD::DirectoryRegistration",
             ResourceType::PanoramaPackage => "AWS::Panorama::Package",
+            ResourceType::PatchCompliance => "AWS::SSM::PatchCompliance",
             ResourceType::PersonalizeDataset => "AWS::Personalize::Dataset",
             ResourceType::PersonalizeDatasetGroup => "AWS::Personalize::DatasetGroup",
             ResourceType::PersonalizeSchema => "AWS::Personalize::Schema",
@@ -2590,37 +2584,42 @@
             ResourceType::PinpointEventStream => "AWS::Pinpoint::EventStream",
             ResourceType::PinpointInAppTemplate => "AWS::Pinpoint::InAppTemplate",
             ResourceType::PinpointSegment => "AWS::Pinpoint::Segment",
+            ResourceType::Pipeline => "AWS::CodePipeline::Pipeline",
+            ResourceType::Policy => "AWS::IAM::Policy",
+            ResourceType::Portfolio => "AWS::ServiceCatalog::Portfolio",
+            ResourceType::Project => "AWS::CodeBuild::Project",
+            ResourceType::Protection => "AWS::Shield::Protection",
             ResourceType::QldbLedger => "AWS::QLDB::Ledger",
+            ResourceType::Queue => "AWS::SQS::Queue",
             ResourceType::QuickSightDataSource => "AWS::QuickSight::DataSource",
             ResourceType::QuickSightTemplate => "AWS::QuickSight::Template",
             ResourceType::QuickSightTheme => "AWS::QuickSight::Theme",
-            ResourceType::DbCluster => "AWS::RDS::DBCluster",
-            ResourceType::DbClusterSnapshot => "AWS::RDS::DBClusterSnapshot",
-            ResourceType::DbInstance => "AWS::RDS::DBInstance",
-            ResourceType::DbSecurityGroup => "AWS::RDS::DBSecurityGroup",
-            ResourceType::DbSnapshot => "AWS::RDS::DBSnapshot",
-            ResourceType::DbSubnetGroup => "AWS::RDS::DBSubnetGroup",
-            ResourceType::EventSubscription => "AWS::RDS::EventSubscription",
             ResourceType::RdsGlobalCluster => "AWS::RDS::GlobalCluster",
             ResourceType::RdsIntegration => "AWS::RDS::Integration",
             ResourceType::RdsOptionGroup => "AWS::RDS::OptionGroup",
             ResourceType::RumAppMonitor => "AWS::RUM::AppMonitor",
-            ResourceType::Cluster => "AWS::Redshift::Cluster",
-            ResourceType::ClusterParameterGroup => "AWS::Redshift::ClusterParameterGroup",
-            ResourceType::ClusterSecurityGroup => "AWS::Redshift::ClusterSecurityGroup",
-            ResourceType::ClusterSnapshot => "AWS::Redshift::ClusterSnapshot",
-            ResourceType::ClusterSubnetGroup => "AWS::Redshift::ClusterSubnetGroup",
+            ResourceType::RateBasedRule => "AWS::WAF::RateBasedRule",
             ResourceType::RedshiftEndpointAccess => "AWS::Redshift::EndpointAccess",
             ResourceType::RedshiftEndpointAuthorization => "AWS::Redshift::EndpointAuthorization",
             ResourceType::RedshiftEventSubscription => "AWS::Redshift::EventSubscription",
             ResourceType::RedshiftIntegration => "AWS::Redshift::Integration",
             ResourceType::RedshiftScheduledAction => "AWS::Redshift::ScheduledAction",
+            ResourceType::RegexPatternSetV2 => "AWS::WAFv2::RegexPatternSet",
+            ResourceType::RegionalProtection => "AWS::ShieldRegional::Protection",
+            ResourceType::RegionalRateBasedRule => "AWS::WAFRegional::RateBasedRule",
+            ResourceType::RegionalRule => "AWS::WAFRegional::Rule",
+            ResourceType::RegionalRuleGroup => "AWS::WAFRegional::RuleGroup",
+            ResourceType::RegionalWebAcl => "AWS::WAFRegional::WebACL",
+            ResourceType::RegisteredHaInstance => "AWS::EC2::RegisteredHAInstance",
             ResourceType::ResilienceHubApp => "AWS::ResilienceHub::App",
             ResourceType::ResilienceHubResiliencyPolicy => "AWS::ResilienceHub::ResiliencyPolicy",
+            ResourceType::ResourceCompliance => "AWS::Config::ResourceCompliance",
             ResourceType::ResourceExplorer2Index => "AWS::ResourceExplorer2::Index",
+            ResourceType::RestApi => "AWS::ApiGateway::RestApi",
             ResourceType::RoboMakerRobotApplication => "AWS::RoboMaker::RobotApplication",
             ResourceType::RoboMakerRobotApplicationVersion => "AWS::RoboMaker::RobotApplicationVersion",
             ResourceType::RoboMakerSimulationApplication => "AWS::RoboMaker::SimulationApplication",
+            ResourceType::Role => "AWS::IAM::Role",
             ResourceType::RolesAnywhereProfile => "AWS::RolesAnywhere::Profile",
             ResourceType::RolesAnywhereTrustAnchor => "AWS::RolesAnywhere::TrustAnchor",
             ResourceType::Route53Dnssec => "AWS::Route53::DNSSEC",
@@ -2643,17 +2642,19 @@
             ResourceType::Route53ResolverResolverQueryLoggingConfigAssociation => "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation",
             ResourceType::Route53ResolverResolverRule => "AWS::Route53Resolver::ResolverRule",
             ResourceType::Route53ResolverResolverRuleAssociation => "AWS::Route53Resolver::ResolverRuleAssociation",
+            ResourceType::RouteTable => "AWS::EC2::RouteTable",
+            ResourceType::Rule => "AWS::WAF::Rule",
+            ResourceType::RuleGroup => "AWS::WAF::RuleGroup",
+            ResourceType::RuleGroupV2 => "AWS::WAFv2::RuleGroup",
             ResourceType::S3AccessGrant => "AWS::S3::AccessGrant",
             ResourceType::S3AccessGrantsInstance => "AWS::S3::AccessGrantsInstance",
             ResourceType::S3AccessGrantsLocation => "AWS::S3::AccessGrantsLocation",
             ResourceType::S3AccessPoint => "AWS::S3::AccessPoint",
-            ResourceType::AccountPublicAccessBlock => "AWS::S3::AccountPublicAccessBlock",
-            ResourceType::Bucket => "AWS::S3::Bucket",
+            ResourceType::S3ExpressBucketPolicy => "AWS::S3Express::BucketPolicy",
+            ResourceType::S3ExpressDirectoryBucket => "AWS::S3Express::DirectoryBucket",
             ResourceType::S3MultiRegionAccessPoint => "AWS::S3::MultiRegionAccessPoint",
             ResourceType::S3StorageLens => "AWS::S3::StorageLens",
             ResourceType::S3StorageLensGroup => "AWS::S3::StorageLensGroup",
-            ResourceType::S3ExpressBucketPolicy => "AWS::S3Express::BucketPolicy",
-            ResourceType::S3ExpressDirectoryBucket => "AWS::S3Express::DirectoryBucket",
             ResourceType::S3TablesTableBucket => "AWS::S3Tables::TableBucket",
             ResourceType::S3TablesTableBucketPolicy => "AWS::S3Tables::TableBucketPolicy",
             ResourceType::SesConfigurationSet => "AWS::SES::ConfigurationSet",
@@ -2663,16 +2664,10 @@
             ResourceType::SesReceiptFilter => "AWS::SES::ReceiptFilter",
             ResourceType::SesReceiptRuleSet => "AWS::SES::ReceiptRuleSet",
             ResourceType::SesTemplate => "AWS::SES::Template",
-            ResourceType::Topic => "AWS::SNS::Topic",
-            ResourceType::Queue => "AWS::SQS::Queue",
-            ResourceType::AssociationCompliance => "AWS::SSM::AssociationCompliance",
-            ResourceType::SsmDocument => "AWS::SSM::Document",
-            ResourceType::FileData => "AWS::SSM::FileData",
-            ResourceType::ManagedInstanceInventory => "AWS::SSM::ManagedInstanceInventory",
-            ResourceType::PatchCompliance => "AWS::SSM::PatchCompliance",
-            ResourceType::SsmResourceDataSync => "AWS::SSM::ResourceDataSync",
             ResourceType::SsmContactsContact => "AWS::SSMContacts::Contact",
+            ResourceType::SsmDocument => "AWS::SSM::Document",
             ResourceType::SsmIncidentsResponsePlan => "AWS::SSMIncidents::ResponsePlan",
+            ResourceType::SsmResourceDataSync => "AWS::SSM::ResourceDataSync",
             ResourceType::SageMakerAppImageConfig => "AWS::SageMaker::AppImageConfig",
             ResourceType::SageMakerCodeRepository => "AWS::SageMaker::CodeRepository",
             ResourceType::SageMakerDataQualityJobDefinition => "AWS::SageMaker::DataQualityJobDefinition",
@@ -2689,22 +2684,28 @@
             ResourceType::SageMakerStudioLifecycleConfig => "AWS::SageMaker::StudioLifecycleConfig",
             ResourceType::SageMakerUserProfile => "AWS::SageMaker::UserProfile",
             ResourceType::SageMakerWorkteam => "AWS::SageMaker::Workteam",
+            ResourceType::ScalingPolicy => "AWS::AutoScaling::ScalingPolicy",
+            ResourceType::ScheduledAction => "AWS::AutoScaling::ScheduledAction",
+            ResourceType::Secret => "AWS::SecretsManager::Secret",
             ResourceType::SecretsManagerResourcePolicy => "AWS::SecretsManager::ResourcePolicy",
             ResourceType::SecretsManagerRotationSchedule => "AWS::SecretsManager::RotationSchedule",
-            ResourceType::Secret => "AWS::SecretsManager::Secret",
+            ResourceType::SecurityGroup => "AWS::EC2::SecurityGroup",
             ResourceType::SecurityHubStandard => "AWS::SecurityHub::Standard",
-            ResourceType::CloudFormationProduct => "AWS::ServiceCatalog::CloudFormationProduct",
-            ResourceType::CloudFormationProvisionedProduct => "AWS::ServiceCatalog::CloudFormationProvisionedProduct",
-            ResourceType::Portfolio => "AWS::ServiceCatalog::Portfolio",
             ResourceType::ServiceDiscoveryHttpNamespace => "AWS::ServiceDiscovery::HttpNamespace",
             ResourceType::ServiceDiscoveryInstance => "AWS::ServiceDiscovery::Instance",
             ResourceType::ServiceDiscoveryPublicDnsNamespace => "AWS::ServiceDiscovery::PublicDnsNamespace",
             ResourceType::ServiceDiscoveryService => "AWS::ServiceDiscovery::Service",
-            ResourceType::Protection => "AWS::Shield::Protection",
-            ResourceType::RegionalProtection => "AWS::ShieldRegional::Protection",
             ResourceType::SignerSigningProfile => "AWS::Signer::SigningProfile",
+            ResourceType::Stack => "AWS::CloudFormation::Stack",
+            ResourceType::Stage => "AWS::ApiGateway::Stage",
+            ResourceType::StageV2 => "AWS::ApiGatewayV2::Stage",
             ResourceType::StepFunctionsActivity => "AWS::StepFunctions::Activity",
             ResourceType::StepFunctionsStateMachine => "AWS::StepFunctions::StateMachine",
+            ResourceType::StreamingDistribution => "AWS::CloudFront::StreamingDistribution",
+            ResourceType::Subnet => "AWS::EC2::Subnet",
+            ResourceType::Table => "AWS::DynamoDB::Table",
+            ResourceType::Topic => "AWS::SNS::Topic",
+            ResourceType::Trail => "AWS::CloudTrail::Trail",
             ResourceType::TransferAgreement => "AWS::Transfer::Agreement",
             ResourceType::TransferCertificate => "AWS::Transfer::Certificate",
             ResourceType::TransferConnector => "AWS::Transfer::Connector",
@@ -2711,22 +2712,21 @@
             ResourceType::TransferProfile => "AWS::Transfer::Profile",
             ResourceType::TransferServer => "AWS::Transfer::Server",
             ResourceType::TransferWorkflow => "AWS::Transfer::Workflow",
-            ResourceType::RateBasedRule => "AWS::WAF::RateBasedRule",
-            ResourceType::Rule => "AWS::WAF::Rule",
-            ResourceType::RuleGroup => "AWS::WAF::RuleGroup",
+            ResourceType::TransitGateway => "AWS::EC2::TransitGateway",
+            ResourceType::TransitGatewayAttachment => "AWS::EC2::TransitGatewayAttachment",
+            ResourceType::TransitGatewayRouteTable => "AWS::EC2::TransitGatewayRouteTable",
+            ResourceType::User => "AWS::IAM::User",
+            ResourceType::Vpc => "AWS::EC2::VPC",
+            ResourceType::VpcEndpoint => "AWS::EC2::VPCEndpoint",
+            ResourceType::VpcEndpointService => "AWS::EC2::VPCEndpointService",
+            ResourceType::VpcPeeringConnection => "AWS::EC2::VPCPeeringConnection",
+            ResourceType::VpnConnection => "AWS::EC2::VPNConnection",
+            ResourceType::VpnGateway => "AWS::EC2::VPNGateway",
+            ResourceType::Volume => "AWS::EC2::Volume",
             ResourceType::WebAcl => "AWS::WAF::WebACL",
-            ResourceType::RegionalRateBasedRule => "AWS::WAFRegional::RateBasedRule",
-            ResourceType::RegionalRule => "AWS::WAFRegional::Rule",
-            ResourceType::RegionalRuleGroup => "AWS::WAFRegional::RuleGroup",
-            ResourceType::RegionalWebAcl => "AWS::WAFRegional::WebACL",
-            ResourceType::IpSetV2 => "AWS::WAFv2::IPSet",
-            ResourceType::ManagedRuleSetV2 => "AWS::WAFv2::ManagedRuleSet",
-            ResourceType::RegexPatternSetV2 => "AWS::WAFv2::RegexPatternSet",
-            ResourceType::RuleGroupV2 => "AWS::WAFv2::RuleGroup",
             ResourceType::WebAclv2 => "AWS::WAFv2::WebACL",
             ResourceType::WorkSpacesConnectionAlias => "AWS::WorkSpaces::ConnectionAlias",
             ResourceType::WorkSpacesWorkspace => "AWS::WorkSpaces::Workspace",
-            ResourceType::EncryptionConfig => "AWS::XRay::EncryptionConfig",
             ResourceType::Unknown(value) => value.as_str(),
         }
     }
@@ -2733,21 +2733,19 @@
     /// Returns all the `&str` representations of the enum members.
     pub const fn values() -> &'static [&'static str] {
         &[
-            "AWS::ACM::Certificate",
             "AWS::ACMPCA::CertificateAuthority",
             "AWS::ACMPCA::CertificateAuthorityActivation",
             "AWS::APS::RuleGroupsNamespace",
             "AWS::AccessAnalyzer::Analyzer",
+            "AWS::S3::AccountPublicAccessBlock",
+            "AWS::CloudWatch::Alarm",
             "AWS::AmazonMQ::Broker",
             "AWS::Amplify::App",
             "AWS::Amplify::Branch",
+            "AWS::ApiGatewayV2::Api",
             "AWS::ApiGateway::Method",
-            "AWS::ApiGateway::RestApi",
-            "AWS::ApiGateway::Stage",
             "AWS::ApiGateway::UsagePlan",
-            "AWS::ApiGatewayV2::Api",
             "AWS::ApiGatewayV2::Integration",
-            "AWS::ApiGatewayV2::Stage",
             "AWS::AppConfig::Application",
             "AWS::AppConfig::ConfigurationProfile",
             "AWS::AppConfig::DeploymentStrategy",
@@ -2774,53 +2772,56 @@
             "AWS::AppStream::Stack",
             "AWS::AppSync::ApiCache",
             "AWS::AppSync::GraphQLApi",
+            "AWS::ElasticBeanstalk::Application",
+            "AWS::ElasticBeanstalk::ApplicationVersion",
+            "AWS::SSM::AssociationCompliance",
             "AWS::Athena::DataCatalog",
             "AWS::Athena::PreparedStatement",
             "AWS::Athena::WorkGroup",
             "AWS::AuditManager::Assessment",
             "AWS::AutoScaling::AutoScalingGroup",
-            "AWS::AutoScaling::LaunchConfiguration",
-            "AWS::AutoScaling::ScalingPolicy",
-            "AWS::AutoScaling::ScheduledAction",
             "AWS::AutoScaling::WarmPool",
             "AWS::B2BI::Capability",
             "AWS::BCMDataExports::Export",
+            "AWS::BackupGateway::Hypervisor",
             "AWS::Backup::BackupPlan",
-            "AWS::Backup::BackupSelection",
-            "AWS::Backup::BackupVault",
             "AWS::Backup::RecoveryPoint",
             "AWS::Backup::ReportPlan",
             "AWS::Backup::RestoreTestingPlan",
-            "AWS::BackupGateway::Hypervisor",
+            "AWS::Backup::BackupSelection",
+            "AWS::Backup::BackupVault",
             "AWS::Batch::ComputeEnvironment",
             "AWS::Batch::JobQueue",
             "AWS::Batch::SchedulingPolicy",
+            "AWS::BedrockAgentCore::BrowserCustom",
+            "AWS::BedrockAgentCore::Runtime",
             "AWS::Bedrock::ApplicationInferenceProfile",
             "AWS::Bedrock::Guardrail",
             "AWS::Bedrock::KnowledgeBase",
             "AWS::Bedrock::Prompt",
-            "AWS::BedrockAgentCore::BrowserCustom",
-            "AWS::BedrockAgentCore::Runtime",
+            "AWS::S3::Bucket",
             "AWS::Budgets::BudgetsAction",
             "AWS::Cassandra::Keyspace",
+            "AWS::ACM::Certificate",
             "AWS::CleanRoomsML::TrainingDataset",
             "AWS::Cloud9::EnvironmentEC2",
             "AWS::CloudFormation::GuardHook",
             "AWS::CloudFormation::LambdaHook",
-            "AWS::CloudFormation::Stack",
+            "AWS::ServiceCatalog::CloudFormationProduct",
+            "AWS::ServiceCatalog::CloudFormationProvisionedProduct",
             "AWS::CloudFormation::StackSet",
-            "AWS::CloudFront::Distribution",
             "AWS::CloudFront::KeyValueStore",
             "AWS::CloudFront::PublicKey",
             "AWS::CloudFront::RealtimeLogConfig",
-            "AWS::CloudFront::StreamingDistribution",
             "AWS::CloudTrail::EventDataStore",
-            "AWS::CloudTrail::Trail",
-            "AWS::CloudWatch::Alarm",
             "AWS::CloudWatch::MetricStream",
+            "AWS::Redshift::Cluster",
+            "AWS::Redshift::ClusterParameterGroup",
+            "AWS::Redshift::ClusterSecurityGroup",
+            "AWS::Redshift::ClusterSnapshot",
+            "AWS::Redshift::ClusterSubnetGroup",
             "AWS::CodeArtifact::Domain",
             "AWS::CodeArtifact::Repository",
-            "AWS::CodeBuild::Project",
             "AWS::CodeBuild::ReportGroup",
             "AWS::CodeDeploy::Application",
             "AWS::CodeDeploy::DeploymentConfig",
@@ -2827,7 +2828,6 @@
             "AWS::CodeDeploy::DeploymentGroup",
             "AWS::CodeGuruProfiler::ProfilingGroup",
             "AWS::CodeGuruReviewer::RepositoryAssociation",
-            "AWS::CodePipeline::Pipeline",
             "AWS::Cognito::IdentityPool",
             "AWS::Cognito::UserPool",
             "AWS::Cognito::UserPoolClient",
@@ -2835,9 +2835,8 @@
             "AWS::Comprehend::Flywheel",
             "AWS::Config::AggregationAuthorization",
             "AWS::Config::ConformancePack",
+            "AWS::Config::StoredQuery",
             "AWS::Config::ConformancePackCompliance",
-            "AWS::Config::ResourceCompliance",
-            "AWS::Config::StoredQuery",
             "AWS::Connect::Instance",
             "AWS::Connect::PhoneNumber",
             "AWS::Connect::QuickConnect",
@@ -2844,8 +2843,15 @@
             "AWS::Connect::Rule",
             "AWS::Connect::SecurityProfile",
             "AWS::Connect::User",
+            "AWS::EC2::CustomerGateway",
             "AWS::CustomerProfiles::Domain",
             "AWS::CustomerProfiles::ObjectType",
+            "AWS::RDS::DBCluster",
+            "AWS::RDS::DBClusterSnapshot",
+            "AWS::RDS::DBInstance",
+            "AWS::RDS::DBSecurityGroup",
+            "AWS::RDS::DBSnapshot",
+            "AWS::RDS::DBSubnetGroup",
             "AWS::DMS::Certificate",
             "AWS::DMS::Endpoint",
             "AWS::DMS::EventSubscription",
@@ -2867,19 +2873,15 @@
             "AWS::DeviceFarm::InstanceProfile",
             "AWS::DeviceFarm::Project",
             "AWS::DeviceFarm::TestGridProject",
-            "AWS::DynamoDB::Table",
+            "AWS::CloudFront::Distribution",
+            "AWS::Elasticsearch::Domain",
             "AWS::EC2::CapacityReservation",
             "AWS::EC2::CarrierGateway",
             "AWS::EC2::ClientVpnEndpoint",
             "AWS::EC2::ClientVpnTargetNetworkAssociation",
-            "AWS::EC2::CustomerGateway",
             "AWS::EC2::DHCPOptions",
             "AWS::EC2::EC2Fleet",
-            "AWS::EC2::EIP",
             "AWS::EC2::EIPAssociation",
-            "AWS::EC2::EgressOnlyInternetGateway",
-            "AWS::EC2::FlowLog",
-            "AWS::EC2::Host",
             "AWS::EC2::IPAM",
             "AWS::EC2::IPAMPool",
             "AWS::EC2::IPAMPoolCidr",
@@ -2886,25 +2888,14 @@
             "AWS::EC2::IPAMResourceDiscovery",
             "AWS::EC2::IPAMResourceDiscoveryAssociation",
             "AWS::EC2::IPAMScope",
-            "AWS::EC2::Instance",
             "AWS::EC2::InstanceConnectEndpoint",
-            "AWS::EC2::InternetGateway",
-            "AWS::EC2::LaunchTemplate",
-            "AWS::EC2::NatGateway",
-            "AWS::EC2::NetworkAcl",
             "AWS::EC2::NetworkInsightsAccessScope",
-            "AWS::EC2::NetworkInsightsAccessScopeAnalysis",
             "AWS::EC2::NetworkInsightsAnalysis",
             "AWS::EC2::NetworkInsightsPath",
-            "AWS::EC2::NetworkInterface",
             "AWS::EC2::PrefixList",
-            "AWS::EC2::RegisteredHAInstance",
-            "AWS::EC2::RouteTable",
-            "AWS::EC2::SecurityGroup",
             "AWS::EC2::SecurityGroupVpcAssociation",
             "AWS::EC2::SnapshotBlockPublicAccess",
             "AWS::EC2::SpotFleet",
-            "AWS::EC2::Subnet",
             "AWS::EC2::SubnetCidrBlock",
             "AWS::EC2::SubnetNetworkAclAssociation",
             "AWS::EC2::SubnetRouteTableAssociation",
@@ -2911,24 +2902,14 @@
             "AWS::EC2::TrafficMirrorFilter",
             "AWS::EC2::TrafficMirrorSession",
             "AWS::EC2::TrafficMirrorTarget",
-            "AWS::EC2::TransitGateway",
-            "AWS::EC2::TransitGatewayAttachment",
             "AWS::EC2::TransitGatewayConnect",
             "AWS::EC2::TransitGatewayMulticastDomain",
-            "AWS::EC2::TransitGatewayRouteTable",
-            "AWS::EC2::VPC",
             "AWS::EC2::VPCBlockPublicAccessExclusion",
             "AWS::EC2::VPCBlockPublicAccessOptions",
-            "AWS::EC2::VPCEndpoint",
             "AWS::EC2::VPCEndpointConnectionNotification",
-            "AWS::EC2::VPCEndpointService",
             "AWS::EC2::VPCGatewayAttachment",
-            "AWS::EC2::VPCPeeringConnection",
-            "AWS::EC2::VPNConnection",
             "AWS::EC2::VPNConnectionRoute",
-            "AWS::EC2::VPNGateway",
             "AWS::EC2::VerifiedAccessInstance",
-            "AWS::EC2::Volume",
             "AWS::ECR::PublicRepository",
             "AWS::ECR::PullThroughCacheRule",
             "AWS::ECR::RegistryPolicy",
@@ -2942,29 +2923,27 @@
             "AWS::ECS::TaskSet",
             "AWS::EFS::AccessPoint",
             "AWS::EFS::FileSystem",
+            "AWS::EC2::EIP",
             "AWS::EKS::Addon",
             "AWS::EKS::Cluster",
             "AWS::EKS::FargateProfile",
             "AWS::EKS::IdentityProviderConfig",
-            "AWS::EMR::SecurityConfiguration",
-            "AWS::EMR::Studio",
             "AWS::EMRContainers::VirtualCluster",
+            "AWS::EMR::SecurityConfiguration",
             "AWS::EMRServerless::Application",
-            "AWS::ElasticBeanstalk::Application",
-            "AWS::ElasticBeanstalk::ApplicationVersion",
-            "AWS::ElasticBeanstalk::Environment",
-            "AWS::ElasticLoadBalancing::LoadBalancer",
-            "AWS::ElasticLoadBalancingV2::Listener",
-            "AWS::ElasticLoadBalancingV2::LoadBalancer",
+            "AWS::EMR::Studio",
+            "AWS::EC2::EgressOnlyInternetGateway",
             "AWS::ElasticLoadBalancingV2::TargetGroup",
-            "AWS::Elasticsearch::Domain",
+            "AWS::XRay::EncryptionConfig",
             "AWS::EntityResolution::IdMappingWorkflow",
             "AWS::EntityResolution::MatchingWorkflow",
             "AWS::EntityResolution::SchemaMapping",
+            "AWS::ElasticBeanstalk::Environment",
             "AWS::EventSchemas::Discoverer",
             "AWS::EventSchemas::Registry",
             "AWS::EventSchemas::RegistryPolicy",
             "AWS::EventSchemas::Schema",
+            "AWS::RDS::EventSubscription",
             "AWS::Events::ApiDestination",
             "AWS::Events::Archive",
             "AWS::Events::Connection",
@@ -2975,6 +2954,8 @@
             "AWS::Evidently::Project",
             "AWS::Evidently::Segment",
             "AWS::FIS::ExperimentTemplate",
+            "AWS::SSM::FileData",
+            "AWS::EC2::FlowLog",
             "AWS::Forecast::Dataset",
             "AWS::Forecast::DatasetGroup",
             "AWS::FraudDetector::EntityType",
@@ -2981,6 +2962,7 @@
             "AWS::FraudDetector::Label",
             "AWS::FraudDetector::Outcome",
             "AWS::FraudDetector::Variable",
+            "AWS::Lambda::Function",
             "AWS::GameLift::Build",
             "AWS::GlobalAccelerator::Accelerator",
             "AWS::GlobalAccelerator::EndpointGroup",
@@ -2994,6 +2976,7 @@
             "AWS::GroundStation::Config",
             "AWS::GroundStation::DataflowEndpointGroup",
             "AWS::GroundStation::MissionProfile",
+            "AWS::IAM::Group",
             "AWS::GuardDuty::Detector",
             "AWS::GuardDuty::Filter",
             "AWS::GuardDuty::IPSet",
@@ -3000,14 +2983,12 @@
             "AWS::GuardDuty::MalwareProtectionPlan",
             "AWS::GuardDuty::ThreatIntelSet",
             "AWS::HealthLake::FHIRDatastore",
-            "AWS::IAM::Group",
+            "AWS::EC2::Host",
             "AWS::IAM::InstanceProfile",
             "AWS::IAM::OIDCProvider",
-            "AWS::IAM::Policy",
-            "AWS::IAM::Role",
             "AWS::IAM::SAMLProvider",
             "AWS::IAM::ServerCertificate",
-            "AWS::IAM::User",
+            "AWS::WAFv2::IPSet",
             "AWS::IVS::Channel",
             "AWS::IVS::PlaybackKeyPair",
             "AWS::IVS::RecordingConfiguration",
@@ -3019,12 +3000,22 @@
             "AWS::ImageBuilder::LifecyclePolicy",
             "AWS::InspectorV2::Activation",
             "AWS::InspectorV2::Filter",
+            "AWS::EC2::Instance",
+            "AWS::EC2::InternetGateway",
             "AWS::IoT::AccountAuditConfiguration",
+            "AWS::IoTAnalytics::Channel",
+            "AWS::IoTAnalytics::Dataset",
+            "AWS::IoTAnalytics::Datastore",
+            "AWS::IoTAnalytics::Pipeline",
             "AWS::IoT::Authorizer",
             "AWS::IoT::CACertificate",
+            "AWS::IoTCoreDeviceAdvisor::SuiteDefinition",
             "AWS::IoT::CustomMetric",
             "AWS::IoT::Dimension",
             "AWS::IoT::DomainConfiguration",
+            "AWS::IoTEvents::AlarmModel",
+            "AWS::IoTEvents::DetectorModel",
+            "AWS::IoTEvents::Input",
             "AWS::IoT::FleetMetric",
             "AWS::IoT::JobTemplate",
             "AWS::IoT::MitigationAction",
@@ -3033,15 +3024,6 @@
             "AWS::IoT::RoleAlias",
             "AWS::IoT::ScheduledAudit",
             "AWS::IoT::SecurityProfile",
-            "AWS::IoT::ThingGroup",
-            "AWS::IoTAnalytics::Channel",
-            "AWS::IoTAnalytics::Dataset",
-            "AWS::IoTAnalytics::Datastore",
-            "AWS::IoTAnalytics::Pipeline",
-            "AWS::IoTCoreDeviceAdvisor::SuiteDefinition",
-            "AWS::IoTEvents::AlarmModel",
-            "AWS::IoTEvents::DetectorModel",
-            "AWS::IoTEvents::Input",
             "AWS::IoTSiteWise::Asset",
             "AWS::IoTSiteWise::AssetModel",
             "AWS::IoTSiteWise::Dashboard",
@@ -3048,6 +3030,7 @@
             "AWS::IoTSiteWise::Gateway",
             "AWS::IoTSiteWise::Portal",
             "AWS::IoTSiteWise::Project",
+            "AWS::IoT::ThingGroup",
             "AWS::IoTTwinMaker::ComponentType",
             "AWS::IoTTwinMaker::Entity",
             "AWS::IoTTwinMaker::Scene",
@@ -3057,18 +3040,19 @@
             "AWS::IoTWireless::MulticastGroup",
             "AWS::IoTWireless::ServiceProfile",
             "AWS::KMS::Alias",
-            "AWS::KMS::Key",
             "AWS::KafkaConnect::Connector",
             "AWS::KafkaConnect::CustomPlugin",
             "AWS::Kendra::Index",
-            "AWS::Kinesis::Stream",
-            "AWS::Kinesis::StreamConsumer",
+            "AWS::KMS::Key",
             "AWS::KinesisAnalyticsV2::Application",
             "AWS::KinesisFirehose::DeliveryStream",
+            "AWS::Kinesis::Stream",
+            "AWS::Kinesis::StreamConsumer",
             "AWS::KinesisVideo::SignalingChannel",
             "AWS::KinesisVideo::Stream",
             "AWS::Lambda::CodeSigningConfig",
-            "AWS::Lambda::Function",
+            "AWS::AutoScaling::LaunchConfiguration",
+            "AWS::EC2::LaunchTemplate",
             "AWS::Lex::Bot",
             "AWS::Lex::BotAlias",
             "AWS::Lightsail::Bucket",
@@ -3075,6 +3059,9 @@
             "AWS::Lightsail::Certificate",
             "AWS::Lightsail::Disk",
             "AWS::Lightsail::StaticIp",
+            "AWS::ElasticLoadBalancingV2::Listener",
+            "AWS::ElasticLoadBalancing::LoadBalancer",
+            "AWS::ElasticLoadBalancingV2::LoadBalancer",
             "AWS::Location::APIKey",
             "AWS::Logs::Destination",
             "AWS::LookoutMetrics::Alert",
@@ -3086,6 +3073,8 @@
             "AWS::MSK::Configuration",
             "AWS::MSK::ServerlessCluster",
             "AWS::MSK::VpcConnection",
+            "AWS::SSM::ManagedInstanceInventory",
+            "AWS::WAFv2::ManagedRuleSet",
             "AWS::MediaConnect::FlowEntitlement",
             "AWS::MediaConnect::FlowSource",
             "AWS::MediaConnect::FlowVpcInterface",
@@ -3097,9 +3086,13 @@
             "AWS::MediaTailor::LiveSource",
             "AWS::MediaTailor::PlaybackConfiguration",
             "AWS::MemoryDB::SubnetGroup",
+            "AWS::EC2::NatGateway",
+            "AWS::EC2::NetworkAcl",
             "AWS::NetworkFirewall::Firewall",
             "AWS::NetworkFirewall::FirewallPolicy",
             "AWS::NetworkFirewall::RuleGroup",
+            "AWS::EC2::NetworkInsightsAccessScopeAnalysis",
+            "AWS::EC2::NetworkInterface",
             "AWS::NetworkManager::ConnectPeer",
             "AWS::NetworkManager::CustomerGatewayAssociation",
             "AWS::NetworkManager::Device",
@@ -3117,6 +3110,7 @@
             "AWS::PCAConnectorAD::Connector",
             "AWS::PCAConnectorAD::DirectoryRegistration",
             "AWS::Panorama::Package",
+            "AWS::SSM::PatchCompliance",
             "AWS::Personalize::Dataset",
             "AWS::Personalize::DatasetGroup",
             "AWS::Personalize::Schema",
@@ -3129,37 +3123,42 @@
             "AWS::Pinpoint::EventStream",
             "AWS::Pinpoint::InAppTemplate",
             "AWS::Pinpoint::Segment",
+            "AWS::CodePipeline::Pipeline",
+            "AWS::IAM::Policy",
+            "AWS::ServiceCatalog::Portfolio",
+            "AWS::CodeBuild::Project",
+            "AWS::Shield::Protection",
             "AWS::QLDB::Ledger",
+            "AWS::SQS::Queue",
             "AWS::QuickSight::DataSource",
             "AWS::QuickSight::Template",
             "AWS::QuickSight::Theme",
-            "AWS::RDS::DBCluster",
-            "AWS::RDS::DBClusterSnapshot",
-            "AWS::RDS::DBInstance",
-            "AWS::RDS::DBSecurityGroup",
-            "AWS::RDS::DBSnapshot",
-            "AWS::RDS::DBSubnetGroup",
-            "AWS::RDS::EventSubscription",
             "AWS::RDS::GlobalCluster",
             "AWS::RDS::Integration",
             "AWS::RDS::OptionGroup",
             "AWS::RUM::AppMonitor",
-            "AWS::Redshift::Cluster",
-            "AWS::Redshift::ClusterParameterGroup",
-            "AWS::Redshift::ClusterSecurityGroup",
-            "AWS::Redshift::ClusterSnapshot",
-            "AWS::Redshift::ClusterSubnetGroup",
+            "AWS::WAF::RateBasedRule",
             "AWS::Redshift::EndpointAccess",
             "AWS::Redshift::EndpointAuthorization",
             "AWS::Redshift::EventSubscription",
             "AWS::Redshift::Integration",
             "AWS::Redshift::ScheduledAction",
+            "AWS::WAFv2::RegexPatternSet",
+            "AWS::ShieldRegional::Protection",
+            "AWS::WAFRegional::RateBasedRule",
+            "AWS::WAFRegional::Rule",
+            "AWS::WAFRegional::RuleGroup",
+            "AWS::WAFRegional::WebACL",
+            "AWS::EC2::RegisteredHAInstance",
             "AWS::ResilienceHub::App",
             "AWS::ResilienceHub::ResiliencyPolicy",
+            "AWS::Config::ResourceCompliance",
             "AWS::ResourceExplorer2::Index",
+            "AWS::ApiGateway::RestApi",
             "AWS::RoboMaker::RobotApplication",
             "AWS::RoboMaker::RobotApplicationVersion",
             "AWS::RoboMaker::SimulationApplication",
+            "AWS::IAM::Role",
             "AWS::RolesAnywhere::Profile",
             "AWS::RolesAnywhere::TrustAnchor",
             "AWS::Route53::DNSSEC",
@@ -3182,17 +3181,19 @@
             "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation",
             "AWS::Route53Resolver::ResolverRule",
             "AWS::Route53Resolver::ResolverRuleAssociation",
+            "AWS::EC2::RouteTable",
+            "AWS::WAF::Rule",
+            "AWS::WAF::RuleGroup",
+            "AWS::WAFv2::RuleGroup",
             "AWS::S3::AccessGrant",
             "AWS::S3::AccessGrantsInstance",
             "AWS::S3::AccessGrantsLocation",
             "AWS::S3::AccessPoint",
-            "AWS::S3::AccountPublicAccessBlock",
-            "AWS::S3::Bucket",
+            "AWS::S3Express::BucketPolicy",
+            "AWS::S3Express::DirectoryBucket",
             "AWS::S3::MultiRegionAccessPoint",
             "AWS::S3::StorageLens",
             "AWS::S3::StorageLensGroup",
-            "AWS::S3Express::BucketPolicy",
-            "AWS::S3Express::DirectoryBucket",
             "AWS::S3Tables::TableBucket",
             "AWS::S3Tables::TableBucketPolicy",
             "AWS::SES::ConfigurationSet",
@@ -3202,16 +3203,10 @@
             "AWS::SES::ReceiptFilter",
             "AWS::SES::ReceiptRuleSet",
             "AWS::SES::Template",
-            "AWS::SNS::Topic",
-            "AWS::SQS::Queue",
-            "AWS::SSM::AssociationCompliance",
-            "AWS::SSM::Document",
-            "AWS::SSM::FileData",
-            "AWS::SSM::ManagedInstanceInventory",
-            "AWS::SSM::PatchCompliance",
-            "AWS::SSM::ResourceDataSync",
             "AWS::SSMContacts::Contact",
+            "AWS::SSM::Document",
             "AWS::SSMIncidents::ResponsePlan",
+            "AWS::SSM::ResourceDataSync",
             "AWS::SageMaker::AppImageConfig",
             "AWS::SageMaker::CodeRepository",
             "AWS::SageMaker::DataQualityJobDefinition",
@@ -3228,22 +3223,28 @@
             "AWS::SageMaker::StudioLifecycleConfig",
             "AWS::SageMaker::UserProfile",
             "AWS::SageMaker::Workteam",
+            "AWS::AutoScaling::ScalingPolicy",
+            "AWS::AutoScaling::ScheduledAction",
+            "AWS::SecretsManager::Secret",
             "AWS::SecretsManager::ResourcePolicy",
             "AWS::SecretsManager::RotationSchedule",
-            "AWS::SecretsManager::Secret",
+            "AWS::EC2::SecurityGroup",
             "AWS::SecurityHub::Standard",
-            "AWS::ServiceCatalog::CloudFormationProduct",
-            "AWS::ServiceCatalog::CloudFormationProvisionedProduct",
-            "AWS::ServiceCatalog::Portfolio",
             "AWS::ServiceDiscovery::HttpNamespace",
             "AWS::ServiceDiscovery::Instance",
             "AWS::ServiceDiscovery::PublicDnsNamespace",
             "AWS::ServiceDiscovery::Service",
-            "AWS::Shield::Protection",
-            "AWS::ShieldRegional::Protection",
             "AWS::Signer::SigningProfile",
+            "AWS::CloudFormation::Stack",
+            "AWS::ApiGateway::Stage",
+            "AWS::ApiGatewayV2::Stage",
             "AWS::StepFunctions::Activity",
             "AWS::StepFunctions::StateMachine",
+            "AWS::CloudFront::StreamingDistribution",
+            "AWS::EC2::Subnet",
+            "AWS::DynamoDB::Table",
+            "AWS::SNS::Topic",
+            "AWS::CloudTrail::Trail",
             "AWS::Transfer::Agreement",
             "AWS::Transfer::Certificate",
             "AWS::Transfer::Connector",
@@ -3250,22 +3251,21 @@
             "AWS::Transfer::Profile",
             "AWS::Transfer::Server",
             "AWS::Transfer::Workflow",
-            "AWS::WAF::RateBasedRule",
-            "AWS::WAF::Rule",
-            "AWS::WAF::RuleGroup",
+            "AWS::EC2::TransitGateway",
+            "AWS::EC2::TransitGatewayAttachment",
+            "AWS::EC2::TransitGatewayRouteTable",
+            "AWS::IAM::User",
+            "AWS::EC2::VPC",
+            "AWS::EC2::VPCEndpoint",
+            "AWS::EC2::VPCEndpointService",
+            "AWS::EC2::VPCPeeringConnection",
+            "AWS::EC2::VPNConnection",
+            "AWS::EC2::VPNGateway",
+            "AWS::EC2::Volume",
             "AWS::WAF::WebACL",
-            "AWS::WAFRegional::RateBasedRule",
-            "AWS::WAFRegional::Rule",
-            "AWS::WAFRegional::RuleGroup",
-            "AWS::WAFRegional::WebACL",
-            "AWS::WAFv2::IPSet",
-            "AWS::WAFv2::ManagedRuleSet",
-            "AWS::WAFv2::RegexPatternSet",
-            "AWS::WAFv2::RuleGroup",
             "AWS::WAFv2::WebACL",
             "AWS::WorkSpaces::ConnectionAlias",
             "AWS::WorkSpaces::Workspace",
-            "AWS::XRay::EncryptionConfig",
         ]
     }
 }
@@ -3289,21 +3289,19 @@
 impl ::std::fmt::Display for ResourceType {
     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
         match self {
-            ResourceType::Certificate => write!(f, "AWS::ACM::Certificate"),
             ResourceType::AcmpcaCertificateAuthority => write!(f, "AWS::ACMPCA::CertificateAuthority"),
             ResourceType::AcmpcaCertificateAuthorityActivation => write!(f, "AWS::ACMPCA::CertificateAuthorityActivation"),
             ResourceType::ApsRuleGroupsNamespace => write!(f, "AWS::APS::RuleGroupsNamespace"),
             ResourceType::AccessAnalyzerAnalyzer => write!(f, "AWS::AccessAnalyzer::Analyzer"),
+            ResourceType::AccountPublicAccessBlock => write!(f, "AWS::S3::AccountPublicAccessBlock"),
+            ResourceType::Alarm => write!(f, "AWS::CloudWatch::Alarm"),
             ResourceType::AmazonMqBroker => write!(f, "AWS::AmazonMQ::Broker"),
             ResourceType::AmplifyApp => write!(f, "AWS::Amplify::App"),
             ResourceType::AmplifyBranch => write!(f, "AWS::Amplify::Branch"),
+            ResourceType::Api => write!(f, "AWS::ApiGatewayV2::Api"),
             ResourceType::ApiGatewayMethod => write!(f, "AWS::ApiGateway::Method"),
-            ResourceType::RestApi => write!(f, "AWS::ApiGateway::RestApi"),
-            ResourceType::Stage => write!(f, "AWS::ApiGateway::Stage"),
             ResourceType::ApiGatewayUsagePlan => write!(f, "AWS::ApiGateway::UsagePlan"),
-            ResourceType::Api => write!(f, "AWS::ApiGatewayV2::Api"),
             ResourceType::ApiGatewayV2Integration => write!(f, "AWS::ApiGatewayV2::Integration"),
-            ResourceType::StageV2 => write!(f, "AWS::ApiGatewayV2::Stage"),
             ResourceType::AppConfigApplication => write!(f, "AWS::AppConfig::Application"),
             ResourceType::AppConfigConfigurationProfile => write!(f, "AWS::AppConfig::ConfigurationProfile"),
             ResourceType::AppConfigDeploymentStrategy => write!(f, "AWS::AppConfig::DeploymentStrategy"),
@@ -3330,53 +3328,56 @@
             ResourceType::AppStreamStack => write!(f, "AWS::AppStream::Stack"),
             ResourceType::AppSyncApiCache => write!(f, "AWS::AppSync::ApiCache"),
             ResourceType::AppSyncGraphQlApi => write!(f, "AWS::AppSync::GraphQLApi"),
+            ResourceType::Application => write!(f, "AWS::ElasticBeanstalk::Application"),
+            ResourceType::ApplicationVersion => write!(f, "AWS::ElasticBeanstalk::ApplicationVersion"),
+            ResourceType::AssociationCompliance => write!(f, "AWS::SSM::AssociationCompliance"),
             ResourceType::AthenaDataCatalog => write!(f, "AWS::Athena::DataCatalog"),
             ResourceType::AthenaPreparedStatement => write!(f, "AWS::Athena::PreparedStatement"),
             ResourceType::AthenaWorkGroup => write!(f, "AWS::Athena::WorkGroup"),
             ResourceType::AuditManagerAssessment => write!(f, "AWS::AuditManager::Assessment"),
             ResourceType::AutoScalingGroup => write!(f, "AWS::AutoScaling::AutoScalingGroup"),
-            ResourceType::LaunchConfiguration => write!(f, "AWS::AutoScaling::LaunchConfiguration"),
-            ResourceType::ScalingPolicy => write!(f, "AWS::AutoScaling::ScalingPolicy"),
-            ResourceType::ScheduledAction => write!(f, "AWS::AutoScaling::ScheduledAction"),
             ResourceType::AutoScalingWarmPool => write!(f, "AWS::AutoScaling::WarmPool"),
             ResourceType::B2BiCapability => write!(f, "AWS::B2BI::Capability"),
             ResourceType::BcmDataExportsExport => write!(f, "AWS::BCMDataExports::Export"),
+            ResourceType::BackupGatewayHypervisor => write!(f, "AWS::BackupGateway::Hypervisor"),
             ResourceType::BackupPlan => write!(f, "AWS::Backup::BackupPlan"),
-            ResourceType::BackupSelection => write!(f, "AWS::Backup::BackupSelection"),
-            ResourceType::BackupVault => write!(f, "AWS::Backup::BackupVault"),
             ResourceType::BackupRecoveryPoint => write!(f, "AWS::Backup::RecoveryPoint"),
             ResourceType::BackupReportPlan => write!(f, "AWS::Backup::ReportPlan"),
             ResourceType::BackupRestoreTestingPlan => write!(f, "AWS::Backup::RestoreTestingPlan"),
-            ResourceType::BackupGatewayHypervisor => write!(f, "AWS::BackupGateway::Hypervisor"),
+            ResourceType::BackupSelection => write!(f, "AWS::Backup::BackupSelection"),
+            ResourceType::BackupVault => write!(f, "AWS::Backup::BackupVault"),
             ResourceType::BatchComputeEnvironment => write!(f, "AWS::Batch::ComputeEnvironment"),
             ResourceType::BatchJobQueue => write!(f, "AWS::Batch::JobQueue"),
             ResourceType::BatchSchedulingPolicy => write!(f, "AWS::Batch::SchedulingPolicy"),
+            ResourceType::BedrockAgentCoreBrowserCustom => write!(f, "AWS::BedrockAgentCore::BrowserCustom"),
+            ResourceType::BedrockAgentCoreRuntime => write!(f, "AWS::BedrockAgentCore::Runtime"),
             ResourceType::BedrockApplicationInferenceProfile => write!(f, "AWS::Bedrock::ApplicationInferenceProfile"),
             ResourceType::BedrockGuardrail => write!(f, "AWS::Bedrock::Guardrail"),
             ResourceType::BedrockKnowledgeBase => write!(f, "AWS::Bedrock::KnowledgeBase"),
             ResourceType::BedrockPrompt => write!(f, "AWS::Bedrock::Prompt"),
-            ResourceType::BedrockAgentCoreBrowserCustom => write!(f, "AWS::BedrockAgentCore::BrowserCustom"),
-            ResourceType::BedrockAgentCoreRuntime => write!(f, "AWS::BedrockAgentCore::Runtime"),
+            ResourceType::Bucket => write!(f, "AWS::S3::Bucket"),
             ResourceType::BudgetsBudgetsAction => write!(f, "AWS::Budgets::BudgetsAction"),
             ResourceType::CassandraKeyspace => write!(f, "AWS::Cassandra::Keyspace"),
+            ResourceType::Certificate => write!(f, "AWS::ACM::Certificate"),
             ResourceType::CleanRoomsMlTrainingDataset => write!(f, "AWS::CleanRoomsML::TrainingDataset"),
             ResourceType::Cloud9EnvironmentEc2 => write!(f, "AWS::Cloud9::EnvironmentEC2"),
             ResourceType::CloudFormationGuardHook => write!(f, "AWS::CloudFormation::GuardHook"),
             ResourceType::CloudFormationLambdaHook => write!(f, "AWS::CloudFormation::LambdaHook"),
-            ResourceType::Stack => write!(f, "AWS::CloudFormation::Stack"),
+            ResourceType::CloudFormationProduct => write!(f, "AWS::ServiceCatalog::CloudFormationProduct"),
+            ResourceType::CloudFormationProvisionedProduct => write!(f, "AWS::ServiceCatalog::CloudFormationProvisionedProduct"),
             ResourceType::CloudFormationStackSet => write!(f, "AWS::CloudFormation::StackSet"),
-            ResourceType::Distribution => write!(f, "AWS::CloudFront::Distribution"),
             ResourceType::CloudFrontKeyValueStore => write!(f, "AWS::CloudFront::KeyValueStore"),
             ResourceType::CloudFrontPublicKey => write!(f, "AWS::CloudFront::PublicKey"),
             ResourceType::CloudFrontRealtimeLogConfig => write!(f, "AWS::CloudFront::RealtimeLogConfig"),
-            ResourceType::StreamingDistribution => write!(f, "AWS::CloudFront::StreamingDistribution"),
             ResourceType::CloudTrailEventDataStore => write!(f, "AWS::CloudTrail::EventDataStore"),
-            ResourceType::Trail => write!(f, "AWS::CloudTrail::Trail"),
-            ResourceType::Alarm => write!(f, "AWS::CloudWatch::Alarm"),
             ResourceType::CloudWatchMetricStream => write!(f, "AWS::CloudWatch::MetricStream"),
+            ResourceType::Cluster => write!(f, "AWS::Redshift::Cluster"),
+            ResourceType::ClusterParameterGroup => write!(f, "AWS::Redshift::ClusterParameterGroup"),
+            ResourceType::ClusterSecurityGroup => write!(f, "AWS::Redshift::ClusterSecurityGroup"),
+            ResourceType::ClusterSnapshot => write!(f, "AWS::Redshift::ClusterSnapshot"),
+            ResourceType::ClusterSubnetGroup => write!(f, "AWS::Redshift::ClusterSubnetGroup"),
             ResourceType::CodeArtifactDomain => write!(f, "AWS::CodeArtifact::Domain"),
             ResourceType::CodeArtifactRepository => write!(f, "AWS::CodeArtifact::Repository"),
-            ResourceType::Project => write!(f, "AWS::CodeBuild::Project"),
             ResourceType::CodeBuildReportGroup => write!(f, "AWS::CodeBuild::ReportGroup"),
             ResourceType::CodeDeployApplication => write!(f, "AWS::CodeDeploy::Application"),
             ResourceType::CodeDeployDeploymentConfig => write!(f, "AWS::CodeDeploy::DeploymentConfig"),
@@ -3383,7 +3384,6 @@
             ResourceType::CodeDeployDeploymentGroup => write!(f, "AWS::CodeDeploy::DeploymentGroup"),
             ResourceType::CodeGuruProfilerProfilingGroup => write!(f, "AWS::CodeGuruProfiler::ProfilingGroup"),
             ResourceType::CodeGuruReviewerRepositoryAssociation => write!(f, "AWS::CodeGuruReviewer::RepositoryAssociation"),
-            ResourceType::Pipeline => write!(f, "AWS::CodePipeline::Pipeline"),
             ResourceType::CognitoIdentityPool => write!(f, "AWS::Cognito::IdentityPool"),
             ResourceType::CognitoUserPool => write!(f, "AWS::Cognito::UserPool"),
             ResourceType::CognitoUserPoolClient => write!(f, "AWS::Cognito::UserPoolClient"),
@@ -3391,9 +3391,8 @@
             ResourceType::ComprehendFlywheel => write!(f, "AWS::Comprehend::Flywheel"),
             ResourceType::ConfigAggregationAuthorization => write!(f, "AWS::Config::AggregationAuthorization"),
             ResourceType::ConfigConformancePack => write!(f, "AWS::Config::ConformancePack"),
-            ResourceType::ConformancePackCompliance => write!(f, "AWS::Config::ConformancePackCompliance"),
-            ResourceType::ResourceCompliance => write!(f, "AWS::Config::ResourceCompliance"),
             ResourceType::ConfigStoredQuery => write!(f, "AWS::Config::StoredQuery"),
+            ResourceType::ConformancePackCompliance => write!(f, "AWS::Config::ConformancePackCompliance"),
             ResourceType::ConnectInstance => write!(f, "AWS::Connect::Instance"),
             ResourceType::ConnectPhoneNumber => write!(f, "AWS::Connect::PhoneNumber"),
             ResourceType::ConnectQuickConnect => write!(f, "AWS::Connect::QuickConnect"),
@@ -3400,8 +3399,15 @@
             ResourceType::ConnectRule => write!(f, "AWS::Connect::Rule"),
             ResourceType::ConnectSecurityProfile => write!(f, "AWS::Connect::SecurityProfile"),
             ResourceType::ConnectUser => write!(f, "AWS::Connect::User"),
+            ResourceType::CustomerGateway => write!(f, "AWS::EC2::CustomerGateway"),
             ResourceType::CustomerProfilesDomain => write!(f, "AWS::CustomerProfiles::Domain"),
             ResourceType::CustomerProfilesObjectType => write!(f, "AWS::CustomerProfiles::ObjectType"),
+            ResourceType::DbCluster => write!(f, "AWS::RDS::DBCluster"),
+            ResourceType::DbClusterSnapshot => write!(f, "AWS::RDS::DBClusterSnapshot"),
+            ResourceType::DbInstance => write!(f, "AWS::RDS::DBInstance"),
+            ResourceType::DbSecurityGroup => write!(f, "AWS::RDS::DBSecurityGroup"),
+            ResourceType::DbSnapshot => write!(f, "AWS::RDS::DBSnapshot"),
+            ResourceType::DbSubnetGroup => write!(f, "AWS::RDS::DBSubnetGroup"),
             ResourceType::DmsCertificate => write!(f, "AWS::DMS::Certificate"),
             ResourceType::DmsEndpoint => write!(f, "AWS::DMS::Endpoint"),
             ResourceType::DmsEventSubscription => write!(f, "AWS::DMS::EventSubscription"),
@@ -3423,19 +3429,15 @@
             ResourceType::DeviceFarmInstanceProfile => write!(f, "AWS::DeviceFarm::InstanceProfile"),
             ResourceType::DeviceFarmProject => write!(f, "AWS::DeviceFarm::Project"),
             ResourceType::DeviceFarmTestGridProject => write!(f, "AWS::DeviceFarm::TestGridProject"),
-            ResourceType::Table => write!(f, "AWS::DynamoDB::Table"),
+            ResourceType::Distribution => write!(f, "AWS::CloudFront::Distribution"),
+            ResourceType::Domain => write!(f, "AWS::Elasticsearch::Domain"),
             ResourceType::Ec2CapacityReservation => write!(f, "AWS::EC2::CapacityReservation"),
             ResourceType::Ec2CarrierGateway => write!(f, "AWS::EC2::CarrierGateway"),
             ResourceType::Ec2ClientVpnEndpoint => write!(f, "AWS::EC2::ClientVpnEndpoint"),
             ResourceType::Ec2ClientVpnTargetNetworkAssociation => write!(f, "AWS::EC2::ClientVpnTargetNetworkAssociation"),
-            ResourceType::CustomerGateway => write!(f, "AWS::EC2::CustomerGateway"),
             ResourceType::Ec2DhcpOptions => write!(f, "AWS::EC2::DHCPOptions"),
             ResourceType::Ec2Ec2Fleet => write!(f, "AWS::EC2::EC2Fleet"),
-            ResourceType::Eip => write!(f, "AWS::EC2::EIP"),
             ResourceType::Ec2EipAssociation => write!(f, "AWS::EC2::EIPAssociation"),
-            ResourceType::EgressOnlyInternetGateway => write!(f, "AWS::EC2::EgressOnlyInternetGateway"),
-            ResourceType::FlowLog => write!(f, "AWS::EC2::FlowLog"),
-            ResourceType::Host => write!(f, "AWS::EC2::Host"),
             ResourceType::Ec2Ipam => write!(f, "AWS::EC2::IPAM"),
             ResourceType::Ec2IpamPool => write!(f, "AWS::EC2::IPAMPool"),
             ResourceType::Ec2IpamPoolCidr => write!(f, "AWS::EC2::IPAMPoolCidr"),
@@ -3442,25 +3444,14 @@
             ResourceType::Ec2IpamResourceDiscovery => write!(f, "AWS::EC2::IPAMResourceDiscovery"),
             ResourceType::Ec2IpamResourceDiscoveryAssociation => write!(f, "AWS::EC2::IPAMResourceDiscoveryAssociation"),
             ResourceType::Ec2IpamScope => write!(f, "AWS::EC2::IPAMScope"),
-            ResourceType::Instance => write!(f, "AWS::EC2::Instance"),
             ResourceType::Ec2InstanceConnectEndpoint => write!(f, "AWS::EC2::InstanceConnectEndpoint"),
-            ResourceType::InternetGateway => write!(f, "AWS::EC2::InternetGateway"),
-            ResourceType::LaunchTemplate => write!(f, "AWS::EC2::LaunchTemplate"),
-            ResourceType::NatGateway => write!(f, "AWS::EC2::NatGateway"),
-            ResourceType::NetworkAcl => write!(f, "AWS::EC2::NetworkAcl"),
             ResourceType::Ec2NetworkInsightsAccessScope => write!(f, "AWS::EC2::NetworkInsightsAccessScope"),
-            ResourceType::NetworkInsightsAccessScopeAnalysis => write!(f, "AWS::EC2::NetworkInsightsAccessScopeAnalysis"),
             ResourceType::Ec2NetworkInsightsAnalysis => write!(f, "AWS::EC2::NetworkInsightsAnalysis"),
             ResourceType::Ec2NetworkInsightsPath => write!(f, "AWS::EC2::NetworkInsightsPath"),
-            ResourceType::NetworkInterface => write!(f, "AWS::EC2::NetworkInterface"),
             ResourceType::Ec2PrefixList => write!(f, "AWS::EC2::PrefixList"),
-            ResourceType::RegisteredHaInstance => write!(f, "AWS::EC2::RegisteredHAInstance"),
-            ResourceType::RouteTable => write!(f, "AWS::EC2::RouteTable"),
-            ResourceType::SecurityGroup => write!(f, "AWS::EC2::SecurityGroup"),
             ResourceType::Ec2SecurityGroupVpcAssociation => write!(f, "AWS::EC2::SecurityGroupVpcAssociation"),
             ResourceType::Ec2SnapshotBlockPublicAccess => write!(f, "AWS::EC2::SnapshotBlockPublicAccess"),
             ResourceType::Ec2SpotFleet => write!(f, "AWS::EC2::SpotFleet"),
-            ResourceType::Subnet => write!(f, "AWS::EC2::Subnet"),
             ResourceType::Ec2SubnetCidrBlock => write!(f, "AWS::EC2::SubnetCidrBlock"),
             ResourceType::Ec2SubnetNetworkAclAssociation => write!(f, "AWS::EC2::SubnetNetworkAclAssociation"),
             ResourceType::Ec2SubnetRouteTableAssociation => write!(f, "AWS::EC2::SubnetRouteTableAssociation"),
@@ -3467,24 +3458,14 @@
             ResourceType::Ec2TrafficMirrorFilter => write!(f, "AWS::EC2::TrafficMirrorFilter"),
             ResourceType::Ec2TrafficMirrorSession => write!(f, "AWS::EC2::TrafficMirrorSession"),
             ResourceType::Ec2TrafficMirrorTarget => write!(f, "AWS::EC2::TrafficMirrorTarget"),
-            ResourceType::TransitGateway => write!(f, "AWS::EC2::TransitGateway"),
-            ResourceType::TransitGatewayAttachment => write!(f, "AWS::EC2::TransitGatewayAttachment"),
             ResourceType::Ec2TransitGatewayConnect => write!(f, "AWS::EC2::TransitGatewayConnect"),
             ResourceType::Ec2TransitGatewayMulticastDomain => write!(f, "AWS::EC2::TransitGatewayMulticastDomain"),
-            ResourceType::TransitGatewayRouteTable => write!(f, "AWS::EC2::TransitGatewayRouteTable"),
-            ResourceType::Vpc => write!(f, "AWS::EC2::VPC"),
             ResourceType::Ec2VpcBlockPublicAccessExclusion => write!(f, "AWS::EC2::VPCBlockPublicAccessExclusion"),
             ResourceType::Ec2VpcBlockPublicAccessOptions => write!(f, "AWS::EC2::VPCBlockPublicAccessOptions"),
-            ResourceType::VpcEndpoint => write!(f, "AWS::EC2::VPCEndpoint"),
             ResourceType::Ec2VpcEndpointConnectionNotification => write!(f, "AWS::EC2::VPCEndpointConnectionNotification"),
-            ResourceType::VpcEndpointService => write!(f, "AWS::EC2::VPCEndpointService"),
             ResourceType::Ec2VpcGatewayAttachment => write!(f, "AWS::EC2::VPCGatewayAttachment"),
-            ResourceType::VpcPeeringConnection => write!(f, "AWS::EC2::VPCPeeringConnection"),
-            ResourceType::VpnConnection => write!(f, "AWS::EC2::VPNConnection"),
             ResourceType::Ec2VpnConnectionRoute => write!(f, "AWS::EC2::VPNConnectionRoute"),
-            ResourceType::VpnGateway => write!(f, "AWS::EC2::VPNGateway"),
             ResourceType::Ec2VerifiedAccessInstance => write!(f, "AWS::EC2::VerifiedAccessInstance"),
-            ResourceType::Volume => write!(f, "AWS::EC2::Volume"),
             ResourceType::EcrPublicRepository => write!(f, "AWS::ECR::PublicRepository"),
             ResourceType::EcrPullThroughCacheRule => write!(f, "AWS::ECR::PullThroughCacheRule"),
             ResourceType::EcrRegistryPolicy => write!(f, "AWS::ECR::RegistryPolicy"),
@@ -3498,29 +3479,27 @@
             ResourceType::EcsTaskSet => write!(f, "AWS::ECS::TaskSet"),
             ResourceType::EfsAccessPoint => write!(f, "AWS::EFS::AccessPoint"),
             ResourceType::EfsFileSystem => write!(f, "AWS::EFS::FileSystem"),
+            ResourceType::Eip => write!(f, "AWS::EC2::EIP"),
             ResourceType::EksAddon => write!(f, "AWS::EKS::Addon"),
             ResourceType::EksCluster => write!(f, "AWS::EKS::Cluster"),
             ResourceType::EksFargateProfile => write!(f, "AWS::EKS::FargateProfile"),
             ResourceType::EksIdentityProviderConfig => write!(f, "AWS::EKS::IdentityProviderConfig"),
+            ResourceType::EmrContainersVirtualCluster => write!(f, "AWS::EMRContainers::VirtualCluster"),
             ResourceType::EmrSecurityConfiguration => write!(f, "AWS::EMR::SecurityConfiguration"),
+            ResourceType::EmrServerlessApplication => write!(f, "AWS::EMRServerless::Application"),
             ResourceType::EmrStudio => write!(f, "AWS::EMR::Studio"),
-            ResourceType::EmrContainersVirtualCluster => write!(f, "AWS::EMRContainers::VirtualCluster"),
-            ResourceType::EmrServerlessApplication => write!(f, "AWS::EMRServerless::Application"),
-            ResourceType::Application => write!(f, "AWS::ElasticBeanstalk::Application"),
-            ResourceType::ApplicationVersion => write!(f, "AWS::ElasticBeanstalk::ApplicationVersion"),
-            ResourceType::Environment => write!(f, "AWS::ElasticBeanstalk::Environment"),
-            ResourceType::LoadBalancer => write!(f, "AWS::ElasticLoadBalancing::LoadBalancer"),
-            ResourceType::ListenerV2 => write!(f, "AWS::ElasticLoadBalancingV2::Listener"),
-            ResourceType::LoadBalancerV2 => write!(f, "AWS::ElasticLoadBalancingV2::LoadBalancer"),
+            ResourceType::EgressOnlyInternetGateway => write!(f, "AWS::EC2::EgressOnlyInternetGateway"),
             ResourceType::ElasticLoadBalancingV2TargetGroup => write!(f, "AWS::ElasticLoadBalancingV2::TargetGroup"),
-            ResourceType::Domain => write!(f, "AWS::Elasticsearch::Domain"),
+            ResourceType::EncryptionConfig => write!(f, "AWS::XRay::EncryptionConfig"),
             ResourceType::EntityResolutionIdMappingWorkflow => write!(f, "AWS::EntityResolution::IdMappingWorkflow"),
             ResourceType::EntityResolutionMatchingWorkflow => write!(f, "AWS::EntityResolution::MatchingWorkflow"),
             ResourceType::EntityResolutionSchemaMapping => write!(f, "AWS::EntityResolution::SchemaMapping"),
+            ResourceType::Environment => write!(f, "AWS::ElasticBeanstalk::Environment"),
             ResourceType::EventSchemasDiscoverer => write!(f, "AWS::EventSchemas::Discoverer"),
             ResourceType::EventSchemasRegistry => write!(f, "AWS::EventSchemas::Registry"),
             ResourceType::EventSchemasRegistryPolicy => write!(f, "AWS::EventSchemas::RegistryPolicy"),
             ResourceType::EventSchemasSchema => write!(f, "AWS::EventSchemas::Schema"),
+            ResourceType::EventSubscription => write!(f, "AWS::RDS::EventSubscription"),
             ResourceType::EventsApiDestination => write!(f, "AWS::Events::ApiDestination"),
             ResourceType::EventsArchive => write!(f, "AWS::Events::Archive"),
             ResourceType::EventsConnection => write!(f, "AWS::Events::Connection"),
@@ -3531,6 +3510,8 @@
             ResourceType::EvidentlyProject => write!(f, "AWS::Evidently::Project"),
             ResourceType::EvidentlySegment => write!(f, "AWS::Evidently::Segment"),
             ResourceType::FisExperimentTemplate => write!(f, "AWS::FIS::ExperimentTemplate"),
+            ResourceType::FileData => write!(f, "AWS::SSM::FileData"),
+            ResourceType::FlowLog => write!(f, "AWS::EC2::FlowLog"),
             ResourceType::ForecastDataset => write!(f, "AWS::Forecast::Dataset"),
             ResourceType::ForecastDatasetGroup => write!(f, "AWS::Forecast::DatasetGroup"),
             ResourceType::FraudDetectorEntityType => write!(f, "AWS::FraudDetector::EntityType"),
@@ -3537,6 +3518,7 @@
             ResourceType::FraudDetectorLabel => write!(f, "AWS::FraudDetector::Label"),
             ResourceType::FraudDetectorOutcome => write!(f, "AWS::FraudDetector::Outcome"),
             ResourceType::FraudDetectorVariable => write!(f, "AWS::FraudDetector::Variable"),
+            ResourceType::Function => write!(f, "AWS::Lambda::Function"),
             ResourceType::GameLiftBuild => write!(f, "AWS::GameLift::Build"),
             ResourceType::GlobalAcceleratorAccelerator => write!(f, "AWS::GlobalAccelerator::Accelerator"),
             ResourceType::GlobalAcceleratorEndpointGroup => write!(f, "AWS::GlobalAccelerator::EndpointGroup"),
@@ -3550,6 +3532,7 @@
             ResourceType::GroundStationConfig => write!(f, "AWS::GroundStation::Config"),
             ResourceType::GroundStationDataflowEndpointGroup => write!(f, "AWS::GroundStation::DataflowEndpointGroup"),
             ResourceType::GroundStationMissionProfile => write!(f, "AWS::GroundStation::MissionProfile"),
+            ResourceType::Group => write!(f, "AWS::IAM::Group"),
             ResourceType::GuardDutyDetector => write!(f, "AWS::GuardDuty::Detector"),
             ResourceType::GuardDutyFilter => write!(f, "AWS::GuardDuty::Filter"),
             ResourceType::GuardDutyIpSet => write!(f, "AWS::GuardDuty::IPSet"),
@@ -3556,14 +3539,12 @@
             ResourceType::GuardDutyMalwareProtectionPlan => write!(f, "AWS::GuardDuty::MalwareProtectionPlan"),
             ResourceType::GuardDutyThreatIntelSet => write!(f, "AWS::GuardDuty::ThreatIntelSet"),
             ResourceType::HealthLakeFhirDatastore => write!(f, "AWS::HealthLake::FHIRDatastore"),
-            ResourceType::Group => write!(f, "AWS::IAM::Group"),
+            ResourceType::Host => write!(f, "AWS::EC2::Host"),
             ResourceType::IamInstanceProfile => write!(f, "AWS::IAM::InstanceProfile"),
             ResourceType::IamoidcProvider => write!(f, "AWS::IAM::OIDCProvider"),
-            ResourceType::Policy => write!(f, "AWS::IAM::Policy"),
-            ResourceType::Role => write!(f, "AWS::IAM::Role"),
             ResourceType::IamsamlProvider => write!(f, "AWS::IAM::SAMLProvider"),
             ResourceType::IamServerCertificate => write!(f, "AWS::IAM::ServerCertificate"),
-            ResourceType::User => write!(f, "AWS::IAM::User"),
+            ResourceType::IpSetV2 => write!(f, "AWS::WAFv2::IPSet"),
             ResourceType::IvsChannel => write!(f, "AWS::IVS::Channel"),
             ResourceType::IvsPlaybackKeyPair => write!(f, "AWS::IVS::PlaybackKeyPair"),
             ResourceType::IvsRecordingConfiguration => write!(f, "AWS::IVS::RecordingConfiguration"),
@@ -3575,12 +3556,22 @@
             ResourceType::ImageBuilderLifecyclePolicy => write!(f, "AWS::ImageBuilder::LifecyclePolicy"),
             ResourceType::InspectorV2Activation => write!(f, "AWS::InspectorV2::Activation"),
             ResourceType::InspectorV2Filter => write!(f, "AWS::InspectorV2::Filter"),
+            ResourceType::Instance => write!(f, "AWS::EC2::Instance"),
+            ResourceType::InternetGateway => write!(f, "AWS::EC2::InternetGateway"),
             ResourceType::IoTAccountAuditConfiguration => write!(f, "AWS::IoT::AccountAuditConfiguration"),
+            ResourceType::IoTAnalyticsChannel => write!(f, "AWS::IoTAnalytics::Channel"),
+            ResourceType::IoTAnalyticsDataset => write!(f, "AWS::IoTAnalytics::Dataset"),
+            ResourceType::IoTAnalyticsDatastore => write!(f, "AWS::IoTAnalytics::Datastore"),
+            ResourceType::IoTAnalyticsPipeline => write!(f, "AWS::IoTAnalytics::Pipeline"),
             ResourceType::IoTAuthorizer => write!(f, "AWS::IoT::Authorizer"),
             ResourceType::IoTcaCertificate => write!(f, "AWS::IoT::CACertificate"),
+            ResourceType::IoTCoreDeviceAdvisorSuiteDefinition => write!(f, "AWS::IoTCoreDeviceAdvisor::SuiteDefinition"),
             ResourceType::IoTCustomMetric => write!(f, "AWS::IoT::CustomMetric"),
             ResourceType::IoTDimension => write!(f, "AWS::IoT::Dimension"),
             ResourceType::IoTDomainConfiguration => write!(f, "AWS::IoT::DomainConfiguration"),
+            ResourceType::IoTEventsAlarmModel => write!(f, "AWS::IoTEvents::AlarmModel"),
+            ResourceType::IoTEventsDetectorModel => write!(f, "AWS::IoTEvents::DetectorModel"),
+            ResourceType::IoTEventsInput => write!(f, "AWS::IoTEvents::Input"),
             ResourceType::IoTFleetMetric => write!(f, "AWS::IoT::FleetMetric"),
             ResourceType::IoTJobTemplate => write!(f, "AWS::IoT::JobTemplate"),
             ResourceType::IoTMitigationAction => write!(f, "AWS::IoT::MitigationAction"),
@@ -3589,15 +3580,6 @@
             ResourceType::IoTRoleAlias => write!(f, "AWS::IoT::RoleAlias"),
             ResourceType::IoTScheduledAudit => write!(f, "AWS::IoT::ScheduledAudit"),
             ResourceType::IoTSecurityProfile => write!(f, "AWS::IoT::SecurityProfile"),
-            ResourceType::IoTThingGroup => write!(f, "AWS::IoT::ThingGroup"),
-            ResourceType::IoTAnalyticsChannel => write!(f, "AWS::IoTAnalytics::Channel"),
-            ResourceType::IoTAnalyticsDataset => write!(f, "AWS::IoTAnalytics::Dataset"),
-            ResourceType::IoTAnalyticsDatastore => write!(f, "AWS::IoTAnalytics::Datastore"),
-            ResourceType::IoTAnalyticsPipeline => write!(f, "AWS::IoTAnalytics::Pipeline"),
-            ResourceType::IoTCoreDeviceAdvisorSuiteDefinition => write!(f, "AWS::IoTCoreDeviceAdvisor::SuiteDefinition"),
-            ResourceType::IoTEventsAlarmModel => write!(f, "AWS::IoTEvents::AlarmModel"),
-            ResourceType::IoTEventsDetectorModel => write!(f, "AWS::IoTEvents::DetectorModel"),
-            ResourceType::IoTEventsInput => write!(f, "AWS::IoTEvents::Input"),
             ResourceType::IoTSiteWiseAsset => write!(f, "AWS::IoTSiteWise::Asset"),
             ResourceType::IoTSiteWiseAssetModel => write!(f, "AWS::IoTSiteWise::AssetModel"),
             ResourceType::IoTSiteWiseDashboard => write!(f, "AWS::IoTSiteWise::Dashboard"),
@@ -3604,6 +3586,7 @@
             ResourceType::IoTSiteWiseGateway => write!(f, "AWS::IoTSiteWise::Gateway"),
             ResourceType::IoTSiteWisePortal => write!(f, "AWS::IoTSiteWise::Portal"),
             ResourceType::IoTSiteWiseProject => write!(f, "AWS::IoTSiteWise::Project"),
+            ResourceType::IoTThingGroup => write!(f, "AWS::IoT::ThingGroup"),
             ResourceType::IoTTwinMakerComponentType => write!(f, "AWS::IoTTwinMaker::ComponentType"),
             ResourceType::IoTTwinMakerEntity => write!(f, "AWS::IoTTwinMaker::Entity"),
             ResourceType::IoTTwinMakerScene => write!(f, "AWS::IoTTwinMaker::Scene"),
@@ -3613,18 +3596,19 @@
             ResourceType::IoTWirelessMulticastGroup => write!(f, "AWS::IoTWireless::MulticastGroup"),
             ResourceType::IoTWirelessServiceProfile => write!(f, "AWS::IoTWireless::ServiceProfile"),
             ResourceType::KmsAlias => write!(f, "AWS::KMS::Alias"),
-            ResourceType::Key => write!(f, "AWS::KMS::Key"),
             ResourceType::KafkaConnectConnector => write!(f, "AWS::KafkaConnect::Connector"),
             ResourceType::KafkaConnectCustomPlugin => write!(f, "AWS::KafkaConnect::CustomPlugin"),
             ResourceType::KendraIndex => write!(f, "AWS::Kendra::Index"),
+            ResourceType::Key => write!(f, "AWS::KMS::Key"),
+            ResourceType::KinesisAnalyticsV2Application => write!(f, "AWS::KinesisAnalyticsV2::Application"),
+            ResourceType::KinesisFirehoseDeliveryStream => write!(f, "AWS::KinesisFirehose::DeliveryStream"),
             ResourceType::KinesisStream => write!(f, "AWS::Kinesis::Stream"),
             ResourceType::KinesisStreamConsumer => write!(f, "AWS::Kinesis::StreamConsumer"),
-            ResourceType::KinesisAnalyticsV2Application => write!(f, "AWS::KinesisAnalyticsV2::Application"),
-            ResourceType::KinesisFirehoseDeliveryStream => write!(f, "AWS::KinesisFirehose::DeliveryStream"),
             ResourceType::KinesisVideoSignalingChannel => write!(f, "AWS::KinesisVideo::SignalingChannel"),
             ResourceType::KinesisVideoStream => write!(f, "AWS::KinesisVideo::Stream"),
             ResourceType::LambdaCodeSigningConfig => write!(f, "AWS::Lambda::CodeSigningConfig"),
-            ResourceType::Function => write!(f, "AWS::Lambda::Function"),
+            ResourceType::LaunchConfiguration => write!(f, "AWS::AutoScaling::LaunchConfiguration"),
+            ResourceType::LaunchTemplate => write!(f, "AWS::EC2::LaunchTemplate"),
             ResourceType::LexBot => write!(f, "AWS::Lex::Bot"),
             ResourceType::LexBotAlias => write!(f, "AWS::Lex::BotAlias"),
             ResourceType::LightsailBucket => write!(f, "AWS::Lightsail::Bucket"),
@@ -3631,6 +3615,9 @@
             ResourceType::LightsailCertificate => write!(f, "AWS::Lightsail::Certificate"),
             ResourceType::LightsailDisk => write!(f, "AWS::Lightsail::Disk"),
             ResourceType::LightsailStaticIp => write!(f, "AWS::Lightsail::StaticIp"),
+            ResourceType::ListenerV2 => write!(f, "AWS::ElasticLoadBalancingV2::Listener"),
+            ResourceType::LoadBalancer => write!(f, "AWS::ElasticLoadBalancing::LoadBalancer"),
+            ResourceType::LoadBalancerV2 => write!(f, "AWS::ElasticLoadBalancingV2::LoadBalancer"),
             ResourceType::LocationApiKey => write!(f, "AWS::Location::APIKey"),
             ResourceType::LogsDestination => write!(f, "AWS::Logs::Destination"),
             ResourceType::LookoutMetricsAlert => write!(f, "AWS::LookoutMetrics::Alert"),
@@ -3642,6 +3629,8 @@
             ResourceType::MskConfiguration => write!(f, "AWS::MSK::Configuration"),
             ResourceType::MskServerlessCluster => write!(f, "AWS::MSK::ServerlessCluster"),
             ResourceType::MskVpcConnection => write!(f, "AWS::MSK::VpcConnection"),
+            ResourceType::ManagedInstanceInventory => write!(f, "AWS::SSM::ManagedInstanceInventory"),
+            ResourceType::ManagedRuleSetV2 => write!(f, "AWS::WAFv2::ManagedRuleSet"),
             ResourceType::MediaConnectFlowEntitlement => write!(f, "AWS::MediaConnect::FlowEntitlement"),
             ResourceType::MediaConnectFlowSource => write!(f, "AWS::MediaConnect::FlowSource"),
             ResourceType::MediaConnectFlowVpcInterface => write!(f, "AWS::MediaConnect::FlowVpcInterface"),
@@ -3653,9 +3642,13 @@
             ResourceType::MediaTailorLiveSource => write!(f, "AWS::MediaTailor::LiveSource"),
             ResourceType::MediaTailorPlaybackConfiguration => write!(f, "AWS::MediaTailor::PlaybackConfiguration"),
             ResourceType::MemoryDbSubnetGroup => write!(f, "AWS::MemoryDB::SubnetGroup"),
+            ResourceType::NatGateway => write!(f, "AWS::EC2::NatGateway"),
+            ResourceType::NetworkAcl => write!(f, "AWS::EC2::NetworkAcl"),
             ResourceType::NetworkFirewallFirewall => write!(f, "AWS::NetworkFirewall::Firewall"),
             ResourceType::NetworkFirewallFirewallPolicy => write!(f, "AWS::NetworkFirewall::FirewallPolicy"),
             ResourceType::NetworkFirewallRuleGroup => write!(f, "AWS::NetworkFirewall::RuleGroup"),
+            ResourceType::NetworkInsightsAccessScopeAnalysis => write!(f, "AWS::EC2::NetworkInsightsAccessScopeAnalysis"),
+            ResourceType::NetworkInterface => write!(f, "AWS::EC2::NetworkInterface"),
             ResourceType::NetworkManagerConnectPeer => write!(f, "AWS::NetworkManager::ConnectPeer"),
             ResourceType::NetworkManagerCustomerGatewayAssociation => write!(f, "AWS::NetworkManager::CustomerGatewayAssociation"),
             ResourceType::NetworkManagerDevice => write!(f, "AWS::NetworkManager::Device"),
@@ -3673,6 +3666,7 @@
             ResourceType::PcaConnectorAdConnector => write!(f, "AWS::PCAConnectorAD::Connector"),
             ResourceType::PcaConnectorAdDirectoryRegistration => write!(f, "AWS::PCAConnectorAD::DirectoryRegistration"),
             ResourceType::PanoramaPackage => write!(f, "AWS::Panorama::Package"),
+            ResourceType::PatchCompliance => write!(f, "AWS::SSM::PatchCompliance"),
             ResourceType::PersonalizeDataset => write!(f, "AWS::Personalize::Dataset"),
             ResourceType::PersonalizeDatasetGroup => write!(f, "AWS::Personalize::DatasetGroup"),
             ResourceType::PersonalizeSchema => write!(f, "AWS::Personalize::Schema"),
@@ -3685,37 +3679,42 @@
             ResourceType::PinpointEventStream => write!(f, "AWS::Pinpoint::EventStream"),
             ResourceType::PinpointInAppTemplate => write!(f, "AWS::Pinpoint::InAppTemplate"),
             ResourceType::PinpointSegment => write!(f, "AWS::Pinpoint::Segment"),
+            ResourceType::Pipeline => write!(f, "AWS::CodePipeline::Pipeline"),
+            ResourceType::Policy => write!(f, "AWS::IAM::Policy"),
+            ResourceType::Portfolio => write!(f, "AWS::ServiceCatalog::Portfolio"),
+            ResourceType::Project => write!(f, "AWS::CodeBuild::Project"),
+            ResourceType::Protection => write!(f, "AWS::Shield::Protection"),
             ResourceType::QldbLedger => write!(f, "AWS::QLDB::Ledger"),
+            ResourceType::Queue => write!(f, "AWS::SQS::Queue"),
             ResourceType::QuickSightDataSource => write!(f, "AWS::QuickSight::DataSource"),
             ResourceType::QuickSightTemplate => write!(f, "AWS::QuickSight::Template"),
             ResourceType::QuickSightTheme => write!(f, "AWS::QuickSight::Theme"),
-            ResourceType::DbCluster => write!(f, "AWS::RDS::DBCluster"),
-            ResourceType::DbClusterSnapshot => write!(f, "AWS::RDS::DBClusterSnapshot"),
-            ResourceType::DbInstance => write!(f, "AWS::RDS::DBInstance"),
-            ResourceType::DbSecurityGroup => write!(f, "AWS::RDS::DBSecurityGroup"),
-            ResourceType::DbSnapshot => write!(f, "AWS::RDS::DBSnapshot"),
-            ResourceType::DbSubnetGroup => write!(f, "AWS::RDS::DBSubnetGroup"),
-            ResourceType::EventSubscription => write!(f, "AWS::RDS::EventSubscription"),
             ResourceType::RdsGlobalCluster => write!(f, "AWS::RDS::GlobalCluster"),
             ResourceType::RdsIntegration => write!(f, "AWS::RDS::Integration"),
             ResourceType::RdsOptionGroup => write!(f, "AWS::RDS::OptionGroup"),
             ResourceType::RumAppMonitor => write!(f, "AWS::RUM::AppMonitor"),
-            ResourceType::Cluster => write!(f, "AWS::Redshift::Cluster"),
-            ResourceType::ClusterParameterGroup => write!(f, "AWS::Redshift::ClusterParameterGroup"),
-            ResourceType::ClusterSecurityGroup => write!(f, "AWS::Redshift::ClusterSecurityGroup"),
-            ResourceType::ClusterSnapshot => write!(f, "AWS::Redshift::ClusterSnapshot"),
-            ResourceType::ClusterSubnetGroup => write!(f, "AWS::Redshift::ClusterSubnetGroup"),
+            ResourceType::RateBasedRule => write!(f, "AWS::WAF::RateBasedRule"),
             ResourceType::RedshiftEndpointAccess => write!(f, "AWS::Redshift::EndpointAccess"),
             ResourceType::RedshiftEndpointAuthorization => write!(f, "AWS::Redshift::EndpointAuthorization"),
             ResourceType::RedshiftEventSubscription => write!(f, "AWS::Redshift::EventSubscription"),
             ResourceType::RedshiftIntegration => write!(f, "AWS::Redshift::Integration"),
             ResourceType::RedshiftScheduledAction => write!(f, "AWS::Redshift::ScheduledAction"),
+            ResourceType::RegexPatternSetV2 => write!(f, "AWS::WAFv2::RegexPatternSet"),
+            ResourceType::RegionalProtection => write!(f, "AWS::ShieldRegional::Protection"),
+            ResourceType::RegionalRateBasedRule => write!(f, "AWS::WAFRegional::RateBasedRule"),
+            ResourceType::RegionalRule => write!(f, "AWS::WAFRegional::Rule"),
+            ResourceType::RegionalRuleGroup => write!(f, "AWS::WAFRegional::RuleGroup"),
+            ResourceType::RegionalWebAcl => write!(f, "AWS::WAFRegional::WebACL"),
+            ResourceType::RegisteredHaInstance => write!(f, "AWS::EC2::RegisteredHAInstance"),
             ResourceType::ResilienceHubApp => write!(f, "AWS::ResilienceHub::App"),
             ResourceType::ResilienceHubResiliencyPolicy => write!(f, "AWS::ResilienceHub::ResiliencyPolicy"),
+            ResourceType::ResourceCompliance => write!(f, "AWS::Config::ResourceCompliance"),
             ResourceType::ResourceExplorer2Index => write!(f, "AWS::ResourceExplorer2::Index"),
+            ResourceType::RestApi => write!(f, "AWS::ApiGateway::RestApi"),
             ResourceType::RoboMakerRobotApplication => write!(f, "AWS::RoboMaker::RobotApplication"),
             ResourceType::RoboMakerRobotApplicationVersion => write!(f, "AWS::RoboMaker::RobotApplicationVersion"),
             ResourceType::RoboMakerSimulationApplication => write!(f, "AWS::RoboMaker::SimulationApplication"),
+            ResourceType::Role => write!(f, "AWS::IAM::Role"),
             ResourceType::RolesAnywhereProfile => write!(f, "AWS::RolesAnywhere::Profile"),
             ResourceType::RolesAnywhereTrustAnchor => write!(f, "AWS::RolesAnywhere::TrustAnchor"),
             ResourceType::Route53Dnssec => write!(f, "AWS::Route53::DNSSEC"),
@@ -3740,17 +3739,19 @@
             }
             ResourceType::Route53ResolverResolverRule => write!(f, "AWS::Route53Resolver::ResolverRule"),
             ResourceType::Route53ResolverResolverRuleAssociation => write!(f, "AWS::Route53Resolver::ResolverRuleAssociation"),
+            ResourceType::RouteTable => write!(f, "AWS::EC2::RouteTable"),
+            ResourceType::Rule => write!(f, "AWS::WAF::Rule"),
+            ResourceType::RuleGroup => write!(f, "AWS::WAF::RuleGroup"),
+            ResourceType::RuleGroupV2 => write!(f, "AWS::WAFv2::RuleGroup"),
             ResourceType::S3AccessGrant => write!(f, "AWS::S3::AccessGrant"),
             ResourceType::S3AccessGrantsInstance => write!(f, "AWS::S3::AccessGrantsInstance"),
             ResourceType::S3AccessGrantsLocation => write!(f, "AWS::S3::AccessGrantsLocation"),
             ResourceType::S3AccessPoint => write!(f, "AWS::S3::AccessPoint"),
-            ResourceType::AccountPublicAccessBlock => write!(f, "AWS::S3::AccountPublicAccessBlock"),
-            ResourceType::Bucket => write!(f, "AWS::S3::Bucket"),
+            ResourceType::S3ExpressBucketPolicy => write!(f, "AWS::S3Express::BucketPolicy"),
+            ResourceType::S3ExpressDirectoryBucket => write!(f, "AWS::S3Express::DirectoryBucket"),
             ResourceType::S3MultiRegionAccessPoint => write!(f, "AWS::S3::MultiRegionAccessPoint"),
             ResourceType::S3StorageLens => write!(f, "AWS::S3::StorageLens"),
             ResourceType::S3StorageLensGroup => write!(f, "AWS::S3::StorageLensGroup"),
-            ResourceType::S3ExpressBucketPolicy => write!(f, "AWS::S3Express::BucketPolicy"),
-            ResourceType::S3ExpressDirectoryBucket => write!(f, "AWS::S3Express::DirectoryBucket"),
             ResourceType::S3TablesTableBucket => write!(f, "AWS::S3Tables::TableBucket"),
             ResourceType::S3TablesTableBucketPolicy => write!(f, "AWS::S3Tables::TableBucketPolicy"),
             ResourceType::SesConfigurationSet => write!(f, "AWS::SES::ConfigurationSet"),
@@ -3760,16 +3761,10 @@
             ResourceType::SesReceiptFilter => write!(f, "AWS::SES::ReceiptFilter"),
             ResourceType::SesReceiptRuleSet => write!(f, "AWS::SES::ReceiptRuleSet"),
             ResourceType::SesTemplate => write!(f, "AWS::SES::Template"),
-            ResourceType::Topic => write!(f, "AWS::SNS::Topic"),
-            ResourceType::Queue => write!(f, "AWS::SQS::Queue"),
-            ResourceType::AssociationCompliance => write!(f, "AWS::SSM::AssociationCompliance"),
+            ResourceType::SsmContactsContact => write!(f, "AWS::SSMContacts::Contact"),
             ResourceType::SsmDocument => write!(f, "AWS::SSM::Document"),
-            ResourceType::FileData => write!(f, "AWS::SSM::FileData"),
-            ResourceType::ManagedInstanceInventory => write!(f, "AWS::SSM::ManagedInstanceInventory"),
-            ResourceType::PatchCompliance => write!(f, "AWS::SSM::PatchCompliance"),
-            ResourceType::SsmResourceDataSync => write!(f, "AWS::SSM::ResourceDataSync"),
-            ResourceType::SsmContactsContact => write!(f, "AWS::SSMContacts::Contact"),
             ResourceType::SsmIncidentsResponsePlan => write!(f, "AWS::SSMIncidents::ResponsePlan"),
+            ResourceType::SsmResourceDataSync => write!(f, "AWS::SSM::ResourceDataSync"),
             ResourceType::SageMakerAppImageConfig => write!(f, "AWS::SageMaker::AppImageConfig"),
             ResourceType::SageMakerCodeRepository => write!(f, "AWS::SageMaker::CodeRepository"),
             ResourceType::SageMakerDataQualityJobDefinition => write!(f, "AWS::SageMaker::DataQualityJobDefinition"),
@@ -3786,22 +3781,28 @@
             ResourceType::SageMakerStudioLifecycleConfig => write!(f, "AWS::SageMaker::StudioLifecycleConfig"),
             ResourceType::SageMakerUserProfile => write!(f, "AWS::SageMaker::UserProfile"),
             ResourceType::SageMakerWorkteam => write!(f, "AWS::SageMaker::Workteam"),
+            ResourceType::ScalingPolicy => write!(f, "AWS::AutoScaling::ScalingPolicy"),
+            ResourceType::ScheduledAction => write!(f, "AWS::AutoScaling::ScheduledAction"),
+            ResourceType::Secret => write!(f, "AWS::SecretsManager::Secret"),
             ResourceType::SecretsManagerResourcePolicy => write!(f, "AWS::SecretsManager::ResourcePolicy"),
             ResourceType::SecretsManagerRotationSchedule => write!(f, "AWS::SecretsManager::RotationSchedule"),
-            ResourceType::Secret => write!(f, "AWS::SecretsManager::Secret"),
+            ResourceType::SecurityGroup => write!(f, "AWS::EC2::SecurityGroup"),
             ResourceType::SecurityHubStandard => write!(f, "AWS::SecurityHub::Standard"),
-            ResourceType::CloudFormationProduct => write!(f, "AWS::ServiceCatalog::CloudFormationProduct"),
-            ResourceType::CloudFormationProvisionedProduct => write!(f, "AWS::ServiceCatalog::CloudFormationProvisionedProduct"),
-            ResourceType::Portfolio => write!(f, "AWS::ServiceCatalog::Portfolio"),
             ResourceType::ServiceDiscoveryHttpNamespace => write!(f, "AWS::ServiceDiscovery::HttpNamespace"),
             ResourceType::ServiceDiscoveryInstance => write!(f, "AWS::ServiceDiscovery::Instance"),
             ResourceType::ServiceDiscoveryPublicDnsNamespace => write!(f, "AWS::ServiceDiscovery::PublicDnsNamespace"),
             ResourceType::ServiceDiscoveryService => write!(f, "AWS::ServiceDiscovery::Service"),
-            ResourceType::Protection => write!(f, "AWS::Shield::Protection"),
-            ResourceType::RegionalProtection => write!(f, "AWS::ShieldRegional::Protection"),
             ResourceType::SignerSigningProfile => write!(f, "AWS::Signer::SigningProfile"),
+            ResourceType::Stack => write!(f, "AWS::CloudFormation::Stack"),
+            ResourceType::Stage => write!(f, "AWS::ApiGateway::Stage"),
+            ResourceType::StageV2 => write!(f, "AWS::ApiGatewayV2::Stage"),
             ResourceType::StepFunctionsActivity => write!(f, "AWS::StepFunctions::Activity"),
             ResourceType::StepFunctionsStateMachine => write!(f, "AWS::StepFunctions::StateMachine"),
+            ResourceType::StreamingDistribution => write!(f, "AWS::CloudFront::StreamingDistribution"),
+            ResourceType::Subnet => write!(f, "AWS::EC2::Subnet"),
+            ResourceType::Table => write!(f, "AWS::DynamoDB::Table"),
+            ResourceType::Topic => write!(f, "AWS::SNS::Topic"),
+            ResourceType::Trail => write!(f, "AWS::CloudTrail::Trail"),
             ResourceType::TransferAgreement => write!(f, "AWS::Transfer::Agreement"),
             ResourceType::TransferCertificate => write!(f, "AWS::Transfer::Certificate"),
             ResourceType::TransferConnector => write!(f, "AWS::Transfer::Connector"),
@@ -3808,22 +3809,21 @@
             ResourceType::TransferProfile => write!(f, "AWS::Transfer::Profile"),
             ResourceType::TransferServer => write!(f, "AWS::Transfer::Server"),
             ResourceType::TransferWorkflow => write!(f, "AWS::Transfer::Workflow"),
-            ResourceType::RateBasedRule => write!(f, "AWS::WAF::RateBasedRule"),
-            ResourceType::Rule => write!(f, "AWS::WAF::Rule"),
-            ResourceType::RuleGroup => write!(f, "AWS::WAF::RuleGroup"),
+            ResourceType::TransitGateway => write!(f, "AWS::EC2::TransitGateway"),
+            ResourceType::TransitGatewayAttachment => write!(f, "AWS::EC2::TransitGatewayAttachment"),
+            ResourceType::TransitGatewayRouteTable => write!(f, "AWS::EC2::TransitGatewayRouteTable"),
+            ResourceType::User => write!(f, "AWS::IAM::User"),
+            ResourceType::Vpc => write!(f, "AWS::EC2::VPC"),
+            ResourceType::VpcEndpoint => write!(f, "AWS::EC2::VPCEndpoint"),
+            ResourceType::VpcEndpointService => write!(f, "AWS::EC2::VPCEndpointService"),
+            ResourceType::VpcPeeringConnection => write!(f, "AWS::EC2::VPCPeeringConnection"),
+            ResourceType::VpnConnection => write!(f, "AWS::EC2::VPNConnection"),
+            ResourceType::VpnGateway => write!(f, "AWS::EC2::VPNGateway"),
+            ResourceType::Volume => write!(f, "AWS::EC2::Volume"),
             ResourceType::WebAcl => write!(f, "AWS::WAF::WebACL"),
-            ResourceType::RegionalRateBasedRule => write!(f, "AWS::WAFRegional::RateBasedRule"),
-            ResourceType::RegionalRule => write!(f, "AWS::WAFRegional::Rule"),
-            ResourceType::RegionalRuleGroup => write!(f, "AWS::WAFRegional::RuleGroup"),
-            ResourceType::RegionalWebAcl => write!(f, "AWS::WAFRegional::WebACL"),
-            ResourceType::IpSetV2 => write!(f, "AWS::WAFv2::IPSet"),
-            ResourceType::ManagedRuleSetV2 => write!(f, "AWS::WAFv2::ManagedRuleSet"),
-            ResourceType::RegexPatternSetV2 => write!(f, "AWS::WAFv2::RegexPatternSet"),
-            ResourceType::RuleGroupV2 => write!(f, "AWS::WAFv2::RuleGroup"),
             ResourceType::WebAclv2 => write!(f, "AWS::WAFv2::WebACL"),
             ResourceType::WorkSpacesConnectionAlias => write!(f, "AWS::WorkSpaces::ConnectionAlias"),
             ResourceType::WorkSpacesWorkspace => write!(f, "AWS::WorkSpaces::Workspace"),
-            ResourceType::EncryptionConfig => write!(f, "AWS::XRay::EncryptionConfig"),
             ResourceType::Unknown(value) => write!(f, "{value}"),
         }
     }
```
