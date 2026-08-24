# AWS SDK Conformance Report: config

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## config
**Progress:** `1262/1262` files compared · `1236` matched · `26` mismatches · `0` missing · `0` extra · `97.94%` match (100.00% means fully matched)

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
