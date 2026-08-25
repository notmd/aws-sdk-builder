# AWS SDK Conformance Report: lambda

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## lambda
**Progress:** `1077/1077` files compared · `1064` matched · `12` mismatches · `0` missing · `1` extra · `98.79%` match (100.00% means fully matched)

### `src/operation/invoke.rs`

```diff
--- reference/src/operation/invoke.rs
+++ generated/src/operation/invoke.rs
@@ -120,6 +120,9 @@
                 InvokeTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 InvokeEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
```

### `src/operation/invoke_async.rs`

```diff
--- reference/src/operation/invoke_async.rs
+++ generated/src/operation/invoke_async.rs
@@ -127,6 +127,9 @@
                 InvokeAsyncTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 InvokeAsyncEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
@@ -259,7 +262,9 @@
             builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
             builder
         };
-        let body = super::super::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(input.invoke_args)?.into_inner();
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(
+            input.invoke_args,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/protocol_serde/shape_event_source_mapping_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_event_source_mapping_configuration.rs
+++ generated/src/protocol_serde/shape_event_source_mapping_configuration.rs
@@ -20,231 +20,231 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "UUID" => {
-                            builder = builder.set_uuid(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "StartingPosition" => {
-                            builder = builder.set_starting_position(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| super::super::types::EventSourcePosition::from(u.as_ref())))
-                                    .transpose()?,
-                            );
-                        }
-                        "StartingPositionTimestamp" => {
-                            builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                tokens.next(),
-                                ::aws_smithy_types::date_time::Format::EpochSeconds,
-                            )?);
-                        }
-                        "BatchSize" => {
-                            builder = builder.set_batch_size(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "MaximumBatchingWindowInSeconds" => {
-                            builder = builder.set_maximum_batching_window_in_seconds(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "ParallelizationFactor" => {
-                            builder = builder.set_parallelization_factor(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "EventSourceArn" => {
-                            builder = builder.set_event_source_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "FilterCriteria" => {
-                            builder = builder.set_filter_criteria(super::super::protocol_serde::shape_filter_criteria::de_filter_criteria(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "FilterCriteriaError" => {
-                            builder = builder.set_filter_criteria_error(
-                                super::super::protocol_serde::shape_filter_criteria_error::de_filter_criteria_error(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "KMSKeyArn" => {
-                            builder = builder.set_kms_key_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "MetricsConfig" => {
-                            builder = builder.set_metrics_config(
-                                super::super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "UUID" => {
+                                builder = builder.set_uuid(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "StartingPosition" => {
+                                builder = builder.set_starting_position(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| super::super::types::EventSourcePosition::from(u.as_ref())))
+                                        .transpose()?,
+                                );
+                            }
+                            "StartingPositionTimestamp" => {
+                                builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                    tokens.next(),
+                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
+                                )?);
+                            }
+                            "BatchSize" => {
+                                builder = builder.set_batch_size(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "MaximumBatchingWindowInSeconds" => {
+                                builder = builder.set_maximum_batching_window_in_seconds(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "ParallelizationFactor" => {
+                                builder = builder.set_parallelization_factor(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "EventSourceArn" => {
+                                builder = builder.set_event_source_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "FilterCriteria" => {
+                                builder = builder.set_filter_criteria(super::super::protocol_serde::shape_filter_criteria::de_filter_criteria(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "LoggingConfig" => {
-                            builder = builder.set_logging_config(
-                                super::super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                                )?);
+                            }
+                            "FilterCriteriaError" => {
+                                builder = builder.set_filter_criteria_error(
+                                    super::super::protocol_serde::shape_filter_criteria_error::de_filter_criteria_error(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            "KMSKeyArn" => {
+                                builder = builder.set_kms_key_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "MetricsConfig" => {
+                                builder = builder.set_metrics_config(
+                                    super::super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "LoggingConfig" => {
+                                builder = builder.set_logging_config(
+                                    super::super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "ScalingConfig" => {
+                                builder = builder.set_scaling_config(super::super::protocol_serde::shape_scaling_config::de_scaling_config(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "ScalingConfig" => {
-                            builder = builder.set_scaling_config(super::super::protocol_serde::shape_scaling_config::de_scaling_config(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "FunctionArn" => {
-                            builder = builder.set_function_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "LastModified" => {
-                            builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                tokens.next(),
-                                ::aws_smithy_types::date_time::Format::EpochSeconds,
-                            )?);
-                        }
-                        "LastProcessingResult" => {
-                            builder = builder.set_last_processing_result(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "State" => {
-                            builder = builder.set_state(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "StateTransitionReason" => {
-                            builder = builder.set_state_transition_reason(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "DestinationConfig" => {
-                            builder = builder.set_destination_config(super::super::protocol_serde::shape_destination_config::de_destination_config(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "Topics" => {
-                            builder = builder.set_topics(super::super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                        }
-                        "Queues" => {
-                            builder = builder.set_queues(super::super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                        }
-                        "SourceAccessConfigurations" => {
-                            builder = builder.set_source_access_configurations(
-                                super::super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "SelfManagedEventSource" => {
-                            builder = builder.set_self_managed_event_source(
-                                super::super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "MaximumRecordAgeInSeconds" => {
-                            builder = builder.set_maximum_record_age_in_seconds(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "BisectBatchOnFunctionError" => {
-                            builder = builder
-                                .set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                        }
-                        "MaximumRetryAttempts" => {
-                            builder = builder.set_maximum_retry_attempts(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "TumblingWindowInSeconds" => {
-                            builder = builder.set_tumbling_window_in_seconds(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "FunctionResponseTypes" => {
-                            builder = builder.set_function_response_types(
-                                super::super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "AmazonManagedKafkaEventSourceConfig" => {
-                            builder = builder.set_amazon_managed_kafka_event_source_config(
-                                super::super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "SelfManagedKafkaEventSourceConfig" => {
-                            builder = builder.set_self_managed_kafka_event_source_config(
-                                super::super::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "DocumentDBEventSourceConfig" => {
-                            builder = builder.set_document_db_event_source_config(
-                                super::super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(
+                                )?);
+                            }
+                            "FunctionArn" => {
+                                builder = builder.set_function_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "LastModified" => {
+                                builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                    tokens.next(),
+                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
+                                )?);
+                            }
+                            "LastProcessingResult" => {
+                                builder = builder.set_last_processing_result(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "State" => {
+                                builder = builder.set_state(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "StateTransitionReason" => {
+                                builder = builder.set_state_transition_reason(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "DestinationConfig" => {
+                                builder = builder.set_destination_config(super::super::protocol_serde::shape_destination_config::de_destination_config(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "EventSourceMappingArn" => {
-                            builder = builder.set_event_source_mapping_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "ProvisionedPollerConfig" => {
-                            builder = builder.set_provisioned_poller_config(
-                                super::super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
-                            );
+                                )?);
+                            }
+                            "Topics" => {
+                                builder = builder.set_topics(super::super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                            }
+                            "Queues" => {
+                                builder = builder.set_queues(super::super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                            }
+                            "SourceAccessConfigurations" => {
+                                builder = builder.set_source_access_configurations(
+                                    super::super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "SelfManagedEventSource" => {
+                                builder = builder.set_self_managed_event_source(
+                                    super::super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            "MaximumRecordAgeInSeconds" => {
+                                builder = builder.set_maximum_record_age_in_seconds(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "BisectBatchOnFunctionError" => {
+                                builder = builder
+                                    .set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                            }
+                            "MaximumRetryAttempts" => {
+                                builder = builder.set_maximum_retry_attempts(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "TumblingWindowInSeconds" => {
+                                builder = builder.set_tumbling_window_in_seconds(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "FunctionResponseTypes" => {
+                                builder = builder.set_function_response_types(
+                                    super::super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "AmazonManagedKafkaEventSourceConfig" => {
+                                builder = builder.set_amazon_managed_kafka_event_source_config(super::super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(tokens, _value, depth + 1)?);
+                            }
+                            "SelfManagedKafkaEventSourceConfig" => {
+                                builder = builder.set_self_managed_kafka_event_source_config(
+                                    super::super::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "DocumentDBEventSourceConfig" => {
+                                builder = builder.set_document_db_event_source_config(
+                                    super::super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "EventSourceMappingArn" => {
+                                builder = builder.set_event_source_mapping_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "ProvisionedPollerConfig" => {
+                                builder = builder.set_provisioned_poller_config(
+                                    super::super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_invoke_async_input.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_async_input.rs
+++ generated/src/protocol_serde/shape_invoke_async_input.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_invoke_args_http_payload(
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

### `src/protocol_serde/shape_invoke_output.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_output.rs
+++ generated/src/protocol_serde/shape_invoke_output.rs
@@ -1,34 +1,33 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_durable_execution_arn_header(
+pub(crate) fn de_payload_payload(
+    body: &[u8],
+) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, super::super::operation::invoke::InvokeError> {
+    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
+}
+pub(crate) fn de_function_error_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Durable-Execution-Arn");
+    let headers = header_map.get_all("X-Amz-Function-Error");
     ::aws_smithy_http::header::one_or_none(headers)
 }

-pub(crate) fn de_executed_version_header(
+pub(crate) fn de_log_result_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Executed-Version");
+    let headers = header_map.get_all("X-Amz-Log-Result");
     ::aws_smithy_http::header::one_or_none(headers)
 }

-pub(crate) fn de_function_error_header(
+pub(crate) fn de_executed_version_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Function-Error");
+    let headers = header_map.get_all("X-Amz-Executed-Version");
     ::aws_smithy_http::header::one_or_none(headers)
 }

-pub(crate) fn de_log_result_header(
+pub(crate) fn de_durable_execution_arn_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Log-Result");
+    let headers = header_map.get_all("X-Amz-Durable-Execution-Arn");
     ::aws_smithy_http::header::one_or_none(headers)
 }
-
-pub(crate) fn de_payload_payload(
-    body: &[u8],
-) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, super::super::operation::invoke::InvokeError> {
-    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
-}
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -115,14 +115,8 @@

 pub(crate) mod shape_invoke_async;

-pub(crate) mod shape_invoke_async_input;
-
-pub(crate) mod shape_invoke_input;
-
 pub(crate) mod shape_invoke_with_response_stream;

-pub(crate) mod shape_invoke_with_response_stream_input;
-
 pub(crate) mod shape_list_aliases;

 pub(crate) mod shape_list_capacity_providers;
@@ -179,22 +173,12 @@

 pub(crate) mod shape_send_durable_execution_callback_failure;

-pub fn rest_json_unset_struct_payload() -> ::std::vec::Vec<u8> {
-    b"{}"[..].into()
-}
-
-pub(crate) mod shape_send_durable_execution_callback_failure_input;
-
 pub(crate) mod shape_send_durable_execution_callback_heartbeat;

 pub(crate) mod shape_send_durable_execution_callback_success;

-pub(crate) mod shape_send_durable_execution_callback_success_input;
-
 pub(crate) mod shape_stop_durable_execution;

-pub(crate) mod shape_stop_durable_execution_input;
-
 pub(crate) mod shape_tag_resource;

 pub(crate) mod shape_untag_resource;
@@ -295,8 +279,14 @@

 pub(crate) mod shape_invalid_zip_file_exception;

+pub(crate) mod shape_invoke_async_input;
+
+pub(crate) mod shape_invoke_input;
+
 pub(crate) mod shape_invoke_output;

+pub(crate) mod shape_invoke_with_response_stream_input;
+
 pub(crate) mod shape_invoke_with_response_stream_output;

 pub(crate) mod shape_kms_access_denied_exception;
@@ -357,6 +347,14 @@

 pub(crate) mod shape_s3_files_mount_timeout_exception;

+pub fn rest_json_unset_struct_payload() -> ::std::vec::Vec<u8> {
+    b"{}"[..].into()
+}
+
+pub(crate) mod shape_send_durable_execution_callback_failure_input;
+
+pub(crate) mod shape_send_durable_execution_callback_success_input;
+
 pub(crate) mod shape_serialized_request_entity_too_large_exception;

 pub(crate) mod shape_service_exception;
@@ -371,6 +369,8 @@

 pub(crate) mod shape_snap_start_timeout_exception;

+pub(crate) mod shape_stop_durable_execution_input;
+
 pub(crate) mod shape_subnet_ip_address_limit_reached_exception;

 pub(crate) mod shape_tag_resource_input;
@@ -505,6 +505,10 @@

 pub(crate) mod shape_instance_requirements;

+pub(crate) mod shape_invoke_response_stream_update;
+
+pub(crate) mod shape_invoke_with_response_stream_complete_event;
+
 pub(crate) mod shape_layer_version_content_input;

 pub(crate) mod shape_layer_version_content_output;
@@ -609,8 +613,6 @@

 pub(crate) mod shape_image_config_error;

-pub(crate) mod shape_invoke_with_response_stream_complete_event;
-
 pub(crate) mod shape_kafka_schema_registry_config;

 pub(crate) mod shape_lambda_managed_instances_capacity_provider_config;
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -295,6 +295,27 @@
     builder
 }

+pub(crate) fn capacity_provider_vpc_config_correct_errors(
+    mut builder: super::types::builders::CapacityProviderVpcConfigBuilder,
+) -> super::types::builders::CapacityProviderVpcConfigBuilder {
+    if builder.subnet_ids.is_none() {
+        builder.subnet_ids = Some(Default::default())
+    }
+    if builder.security_group_ids.is_none() {
+        builder.security_group_ids = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn capacity_provider_permissions_config_correct_errors(
+    mut builder: super::types::builders::CapacityProviderPermissionsConfigBuilder,
+) -> super::types::builders::CapacityProviderPermissionsConfigBuilder {
+    if builder.capacity_provider_operator_role_arn.is_none() {
+        builder.capacity_provider_operator_role_arn = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn code_signing_config_correct_errors(
     mut builder: super::types::builders::CodeSigningConfigBuilder,
 ) -> super::types::builders::CodeSigningConfigBuilder {
@@ -322,6 +343,15 @@
     builder
 }

+pub(crate) fn allowed_publishers_correct_errors(
+    mut builder: super::types::builders::AllowedPublishersBuilder,
+) -> super::types::builders::AllowedPublishersBuilder {
+    if builder.signing_profile_version_arns.is_none() {
+        builder.signing_profile_version_arns = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn capacity_provider_config_correct_errors(
     mut builder: super::types::builders::CapacityProviderConfigBuilder,
 ) -> super::types::builders::CapacityProviderConfigBuilder {
@@ -364,36 +394,6 @@
     builder
 }

-pub(crate) fn capacity_provider_vpc_config_correct_errors(
-    mut builder: super::types::builders::CapacityProviderVpcConfigBuilder,
-) -> super::types::builders::CapacityProviderVpcConfigBuilder {
-    if builder.subnet_ids.is_none() {
-        builder.subnet_ids = Some(Default::default())
-    }
-    if builder.security_group_ids.is_none() {
-        builder.security_group_ids = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn capacity_provider_permissions_config_correct_errors(
-    mut builder: super::types::builders::CapacityProviderPermissionsConfigBuilder,
-) -> super::types::builders::CapacityProviderPermissionsConfigBuilder {
-    if builder.capacity_provider_operator_role_arn.is_none() {
-        builder.capacity_provider_operator_role_arn = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn allowed_publishers_correct_errors(
-    mut builder: super::types::builders::AllowedPublishersBuilder,
-) -> super::types::builders::AllowedPublishersBuilder {
-    if builder.signing_profile_version_arns.is_none() {
-        builder.signing_profile_version_arns = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn execution_correct_errors(mut builder: super::types::builders::ExecutionBuilder) -> super::types::builders::ExecutionBuilder {
     if builder.durable_execution_arn.is_none() {
         builder.durable_execution_arn = Some(Default::default())
```

### `src/types/_capacity_provider_logging_config.rs`

```diff
--- reference/src/types/_capacity_provider_logging_config.rs
+++ generated/src/types/_capacity_provider_logging_config.rs
@@ -6,7 +6,7 @@
 pub struct CapacityProviderLoggingConfig {
     /// <p>Set this property to filter the system logs for your capacity provider that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level of detail and lower, where <code>DEBUG</code> is the highest level and <code>WARN</code> is the lowest.</p>
     pub system_log_level: ::std::option::Option<super::super::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl CapacityProviderLoggingConfig {
@@ -14,7 +14,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&super::super::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -48,17 +48,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<super::super::types::SystemLogLevel> {
         &self.system_log_level
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.log_group = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.log_group = input;
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn get_log_group(&self) -> &::std::option::Option<::std::string::String> {
         &self.log_group
     }
```

### `src/types/_event.rs`

```diff
--- reference/src/types/_event.rs
+++ generated/src/types/_event.rs
@@ -676,7 +676,7 @@
         super::super::types::Event {
             event_type: self.event_type,
             sub_type: self.sub_type,
-            event_id: self.event_id.unwrap_or(1),
+            event_id: self.event_id.unwrap_or_default(),
             id: self.id,
             name: self.name,
             event_timestamp: self.event_timestamp,
```

### `src/types/_logging_config.rs`

```diff
--- reference/src/types/_logging_config.rs
+++ generated/src/types/_logging_config.rs
@@ -10,7 +10,7 @@
     pub application_log_level: ::std::option::Option<super::super::types::ApplicationLogLevel>,
     /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level of detail and lower, where <code>DEBUG</code> is the highest level and <code>WARN</code> is the lowest.</p>
     pub system_log_level: ::std::option::Option<super::super::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl LoggingConfig {
@@ -26,7 +26,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&super::super::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -90,17 +90,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<super::super::types::SystemLogLevel> {
         &self.system_log_level
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.log_group = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.log_group = input;
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn get_log_group(&self) -> &::std::option::Option<::std::string::String> {
         &self.log_group
     }
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use super::super::super::types::error::_invalid_parameter_value_exception::InvalidParameterValueExceptionBuilder;

+pub use super::super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
+
+pub use super::super::super::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
+
 pub use super::super::super::types::error::_resource_conflict_exception::ResourceConflictExceptionBuilder;

 pub use super::super::super::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;
@@ -9,14 +13,8 @@

 pub use super::super::super::types::error::_too_many_requests_exception::TooManyRequestsExceptionBuilder;

-pub use super::super::super::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
-
-pub use super::super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
-
 pub use super::super::super::types::error::_public_policy_exception::PublicPolicyExceptionBuilder;

-pub use super::super::super::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
-
 pub use super::super::super::types::error::_kms_access_denied_exception::KmsAccessDeniedExceptionBuilder;

 pub use super::super::super::types::error::_kms_disabled_exception::KmsDisabledExceptionBuilder;
@@ -25,10 +23,10 @@

 pub use super::super::super::types::error::_kms_not_found_exception::KmsNotFoundExceptionBuilder;

+pub use super::super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
+
 pub use super::super::super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededExceptionBuilder;

-pub use super::super::super::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
-
 pub use super::super::super::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundExceptionBuilder;

 pub use super::super::super::types::error::_code_storage_exceeded_exception::CodeStorageExceededExceptionBuilder;
@@ -39,6 +37,10 @@

 pub use super::super::super::types::error::_invalid_code_signature_exception::InvalidCodeSignatureExceptionBuilder;

+pub use super::super::super::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
+
+pub use super::super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+
 pub use super::super::super::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedExceptionBuilder;

 pub use super::super::super::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedExceptionBuilder;
@@ -107,6 +109,4 @@

 pub use super::super::super::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeExceptionBuilder;

-pub use super::super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
-
-pub use super::super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+pub use super::super::super::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use super::super::types::error::_invalid_parameter_value_exception::InvalidParameterValueException;

+pub use super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
+
+pub use super::super::types::error::_precondition_failed_exception::PreconditionFailedException;
+
 pub use super::super::types::error::_resource_conflict_exception::ResourceConflictException;

 pub use super::super::types::error::_resource_not_found_exception::ResourceNotFoundException;
@@ -9,14 +13,8 @@

 pub use super::super::types::error::_too_many_requests_exception::TooManyRequestsException;

-pub use super::super::types::error::_precondition_failed_exception::PreconditionFailedException;
-
-pub use super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
-
 pub use super::super::types::error::_public_policy_exception::PublicPolicyException;

-pub use super::super::types::error::_callback_timeout_exception::CallbackTimeoutException;
-
 pub use super::super::types::error::_kms_access_denied_exception::KmsAccessDeniedException;

 pub use super::super::types::error::_kms_disabled_exception::KmsDisabledException;
@@ -25,9 +23,9 @@

 pub use super::super::types::error::_kms_not_found_exception::KmsNotFoundException;

-pub use super::super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;
+pub use super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;

-pub use super::super::types::error::_resource_in_use_exception::ResourceInUseException;
+pub use super::super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;

 pub use super::super::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundException;

@@ -39,6 +37,10 @@

 pub use super::super::types::error::_invalid_code_signature_exception::InvalidCodeSignatureException;

+pub use super::super::types::error::_resource_in_use_exception::ResourceInUseException;
+
+pub use super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+
 pub use super::super::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedException;

 pub use super::super::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedException;
@@ -107,9 +109,7 @@

 pub use super::super::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeException;

-pub use super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;
-
-pub use super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+pub use super::super::types::error::_callback_timeout_exception::CallbackTimeoutException;

 /// Error type for the `InvokeWithResponseStreamResponseEventError` operation.
 #[non_exhaustive]
@@ -203,7 +203,6 @@
         self.meta().request_id()
     }
 }
-
 mod _alias_limit_exceeded_exception;

 mod _callback_timeout_exception;
```

### Unexpected generated files

- `src/protocol_serde/shape_invoke_response_stream_update.rs`
