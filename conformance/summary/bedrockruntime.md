# AWS SDK Conformance Report: bedrockruntime

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## bedrockruntime
**Progress:** `536/536` files compared · `531` matched · `5` mismatches · `0` missing · `0` extra · `99.07%` match (100.00% means fully matched)

### `src/protocol_serde/shape_guardrail_automated_reasoning_finding.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_automated_reasoning_finding.rs
+++ generated/src/protocol_serde/shape_guardrail_automated_reasoning_finding.rs
@@ -36,53 +36,18 @@
                         ));
                     }
                     variant = match key.as_ref() {
-                            "valid" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Valid(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_valid_finding::de_guardrail_automated_reasoning_valid_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'valid' cannot be null"))?
-                                ))
-                            }
-                            "invalid" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Invalid(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_invalid_finding::de_guardrail_automated_reasoning_invalid_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'invalid' cannot be null"))?
-                                ))
-                            }
-                            "satisfiable" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Satisfiable(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_satisfiable_finding::de_guardrail_automated_reasoning_satisfiable_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'satisfiable' cannot be null"))?
-                                ))
-                            }
-                            "impossible" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Impossible(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_impossible_finding::de_guardrail_automated_reasoning_impossible_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'impossible' cannot be null"))?
-                                ))
-                            }
-                            "translationAmbiguous" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::TranslationAmbiguous(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_translation_ambiguous_finding::de_guardrail_automated_reasoning_translation_ambiguous_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'translationAmbiguous' cannot be null"))?
-                                ))
-                            }
-                            "tooComplex" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::TooComplex(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_too_complex_finding::de_guardrail_automated_reasoning_too_complex_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'tooComplex' cannot be null"))?
-                                ))
-                            }
-                            "noTranslations" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::NoTranslations(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_no_translations_finding::de_guardrail_automated_reasoning_no_translations_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'noTranslations' cannot be null"))?
-                                ))
-                            }
-                            _ => {
-                                                                              ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
-                                                                              Some(super::super::types::GuardrailAutomatedReasoningFinding::Unknown)
-                                                                            }
-                        };
+                        "valid" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Valid(super::super::protocol_serde::shape_guardrail_automated_reasoning_valid_finding::de_guardrail_automated_reasoning_valid_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'valid' cannot be null"))?)),
+                        "invalid" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Invalid(super::super::protocol_serde::shape_guardrail_automated_reasoning_invalid_finding::de_guardrail_automated_reasoning_invalid_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'invalid' cannot be null"))?)),
+                        "satisfiable" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Satisfiable(super::super::protocol_serde::shape_guardrail_automated_reasoning_satisfiable_finding::de_guardrail_automated_reasoning_satisfiable_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'satisfiable' cannot be null"))?)),
+                        "impossible" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Impossible(super::super::protocol_serde::shape_guardrail_automated_reasoning_impossible_finding::de_guardrail_automated_reasoning_impossible_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'impossible' cannot be null"))?)),
+                        "translationAmbiguous" => Some(super::super::types::GuardrailAutomatedReasoningFinding::TranslationAmbiguous(super::super::protocol_serde::shape_guardrail_automated_reasoning_translation_ambiguous_finding::de_guardrail_automated_reasoning_translation_ambiguous_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'translationAmbiguous' cannot be null"))?)),
+                        "tooComplex" => Some(super::super::types::GuardrailAutomatedReasoningFinding::TooComplex(super::super::protocol_serde::shape_guardrail_automated_reasoning_too_complex_finding::de_guardrail_automated_reasoning_too_complex_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'tooComplex' cannot be null"))?)),
+                        "noTranslations" => Some(super::super::types::GuardrailAutomatedReasoningFinding::NoTranslations(super::super::protocol_serde::shape_guardrail_automated_reasoning_no_translations_finding::de_guardrail_automated_reasoning_no_translations_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'noTranslations' cannot be null"))?)),
+                        _ => {
+                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
+                            Some(super::super::types::GuardrailAutomatedReasoningFinding::Unknown)
+                        }
+                    };
                 }
                 other => {
                     return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -249,8 +249,16 @@

 pub(crate) mod shape_citations_content_block;

+pub(crate) mod shape_content_block_delta;
+
+pub(crate) mod shape_content_block_start;
+
 pub(crate) mod shape_content_blocks;

+pub(crate) mod shape_converse_stream_metrics;
+
+pub(crate) mod shape_converse_stream_trace;
+
 pub(crate) mod shape_document_block;

 pub(crate) mod shape_guardrail_assessment_list_map;
@@ -323,13 +331,7 @@

 pub(crate) mod shape_citations_config;

-pub(crate) mod shape_content_block_delta;
-
-pub(crate) mod shape_content_block_start;
-
-pub(crate) mod shape_converse_stream_metrics;
-
-pub(crate) mod shape_converse_stream_trace;
+pub(crate) mod shape_citations_delta;

 pub(crate) mod shape_document_source;

@@ -361,10 +363,16 @@

 pub(crate) mod shape_guardrail_topic_list;

+pub(crate) mod shape_image_block_delta;
+
+pub(crate) mod shape_image_block_start;
+
 pub(crate) mod shape_image_source;

 pub(crate) mod shape_json_schema_definition;

+pub(crate) mod shape_reasoning_content_block_delta;
+
 pub(crate) mod shape_reasoning_text_block;

 pub(crate) mod shape_search_result_content_block;
@@ -373,8 +381,16 @@

 pub(crate) mod shape_tool_reference;

+pub(crate) mod shape_tool_result_block_start;
+
+pub(crate) mod shape_tool_result_blocks_delta;
+
 pub(crate) mod shape_tool_result_content_block;

+pub(crate) mod shape_tool_use_block_delta;
+
+pub(crate) mod shape_tool_use_block_start;
+
 pub(crate) mod shape_video_source;

 pub(crate) mod shape_citation_location;
@@ -381,7 +397,7 @@

 pub(crate) mod shape_citation_source_content;

-pub(crate) mod shape_citations_delta;
+pub(crate) mod shape_citation_source_content_list_delta;

 pub(crate) mod shape_document_content_block;

@@ -401,25 +417,13 @@

 pub(crate) mod shape_guardrail_topic;

-pub(crate) mod shape_image_block_delta;
-
-pub(crate) mod shape_image_block_start;
-
-pub(crate) mod shape_reasoning_content_block_delta;
-
 pub(crate) mod shape_s3_location;

-pub(crate) mod shape_tool_result_block_start;
-
-pub(crate) mod shape_tool_result_blocks_delta;
-
-pub(crate) mod shape_tool_use_block_delta;
-
-pub(crate) mod shape_tool_use_block_start;
+pub(crate) mod shape_tool_result_block_delta;

 pub(crate) mod shape_citation_generated_content_list;

-pub(crate) mod shape_citation_source_content_list_delta;
+pub(crate) mod shape_citation_source_content_delta;

 pub(crate) mod shape_citations;

@@ -447,14 +451,10 @@

 pub(crate) mod shape_search_result_location;

-pub(crate) mod shape_tool_result_block_delta;
-
 pub(crate) mod shape_tool_result_content_blocks;

 pub(crate) mod shape_web_location;

-pub(crate) mod shape_citation_source_content_delta;
-
 pub(crate) mod shape_document_content_blocks;

 pub(crate) mod shape_guardrail_automated_reasoning_difference_scenario_list;
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -202,6 +202,57 @@
     builder
 }

+pub(crate) fn content_block_delta_event_correct_errors(
+    mut builder: super::types::builders::ContentBlockDeltaEventBuilder,
+) -> super::types::builders::ContentBlockDeltaEventBuilder {
+    if builder.delta.is_none() {
+        builder.delta = Some(super::types::ContentBlockDelta::Unknown)
+    }
+    if builder.content_block_index.is_none() {
+        builder.content_block_index = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn content_block_start_event_correct_errors(
+    mut builder: super::types::builders::ContentBlockStartEventBuilder,
+) -> super::types::builders::ContentBlockStartEventBuilder {
+    if builder.start.is_none() {
+        builder.start = Some(super::types::ContentBlockStart::Unknown)
+    }
+    if builder.content_block_index.is_none() {
+        builder.content_block_index = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn content_block_stop_event_correct_errors(
+    mut builder: super::types::builders::ContentBlockStopEventBuilder,
+) -> super::types::builders::ContentBlockStopEventBuilder {
+    if builder.content_block_index.is_none() {
+        builder.content_block_index = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn converse_stream_metadata_event_correct_errors(
+    mut builder: super::types::builders::ConverseStreamMetadataEventBuilder,
+) -> super::types::builders::ConverseStreamMetadataEventBuilder {
+    if builder.usage.is_none() {
+        builder.usage = {
+            let builder = super::types::builders::TokenUsageBuilder::default();
+            super::serde_util::token_usage_correct_errors(builder).build().ok()
+        }
+    }
+    if builder.metrics.is_none() {
+        builder.metrics = {
+            let builder = super::types::builders::ConverseStreamMetricsBuilder::default();
+            super::serde_util::converse_stream_metrics_correct_errors(builder).build().ok()
+        }
+    }
+    builder
+}
+
 pub(crate) fn guardrail_checks_content_filter_result_correct_errors(
     mut builder: super::types::builders::GuardrailChecksContentFilterResultBuilder,
 ) -> super::types::builders::GuardrailChecksContentFilterResultBuilder {
@@ -266,63 +317,39 @@
     builder
 }

-pub(crate) fn cache_detail_correct_errors(mut builder: super::types::builders::CacheDetailBuilder) -> super::types::builders::CacheDetailBuilder {
-    if builder.ttl.is_none() {
-        builder.ttl = "no value was set".parse::<super::types::CacheTtl>().ok()
-    }
-    if builder.input_tokens.is_none() {
-        builder.input_tokens = Some(Default::default())
+pub(crate) fn message_start_event_correct_errors(
+    mut builder: super::types::builders::MessageStartEventBuilder,
+) -> super::types::builders::MessageStartEventBuilder {
+    if builder.role.is_none() {
+        builder.role = "no value was set".parse::<super::types::ConversationRole>().ok()
     }
     builder
 }

-pub(crate) fn content_block_delta_event_correct_errors(
-    mut builder: super::types::builders::ContentBlockDeltaEventBuilder,
-) -> super::types::builders::ContentBlockDeltaEventBuilder {
-    if builder.delta.is_none() {
-        builder.delta = Some(super::types::ContentBlockDelta::Unknown)
+pub(crate) fn message_stop_event_correct_errors(
+    mut builder: super::types::builders::MessageStopEventBuilder,
+) -> super::types::builders::MessageStopEventBuilder {
+    if builder.stop_reason.is_none() {
+        builder.stop_reason = "no value was set".parse::<super::types::StopReason>().ok()
     }
-    if builder.content_block_index.is_none() {
-        builder.content_block_index = Some(Default::default())
-    }
     builder
 }

-pub(crate) fn content_block_start_event_correct_errors(
-    mut builder: super::types::builders::ContentBlockStartEventBuilder,
-) -> super::types::builders::ContentBlockStartEventBuilder {
-    if builder.start.is_none() {
-        builder.start = Some(super::types::ContentBlockStart::Unknown)
-    }
-    if builder.content_block_index.is_none() {
-        builder.content_block_index = Some(Default::default())
+pub(crate) fn cache_detail_correct_errors(mut builder: super::types::builders::CacheDetailBuilder) -> super::types::builders::CacheDetailBuilder {
+    if builder.ttl.is_none() {
+        builder.ttl = "no value was set".parse::<super::types::CacheTtl>().ok()
     }
-    builder
-}
-
-pub(crate) fn content_block_stop_event_correct_errors(
-    mut builder: super::types::builders::ContentBlockStopEventBuilder,
-) -> super::types::builders::ContentBlockStopEventBuilder {
-    if builder.content_block_index.is_none() {
-        builder.content_block_index = Some(Default::default())
+    if builder.input_tokens.is_none() {
+        builder.input_tokens = Some(Default::default())
     }
     builder
 }

-pub(crate) fn converse_stream_metadata_event_correct_errors(
-    mut builder: super::types::builders::ConverseStreamMetadataEventBuilder,
-) -> super::types::builders::ConverseStreamMetadataEventBuilder {
-    if builder.usage.is_none() {
-        builder.usage = {
-            let builder = super::types::builders::TokenUsageBuilder::default();
-            super::serde_util::token_usage_correct_errors(builder).build().ok()
-        }
-    }
-    if builder.metrics.is_none() {
-        builder.metrics = {
-            let builder = super::types::builders::ConverseStreamMetricsBuilder::default();
-            super::serde_util::converse_stream_metrics_correct_errors(builder).build().ok()
-        }
+pub(crate) fn converse_stream_metrics_correct_errors(
+    mut builder: super::types::builders::ConverseStreamMetricsBuilder,
+) -> super::types::builders::ConverseStreamMetricsBuilder {
+    if builder.latency_ms.is_none() {
+        builder.latency_ms = Some(Default::default())
     }
     builder
 }
@@ -369,33 +396,6 @@
     builder
 }

-pub(crate) fn message_start_event_correct_errors(
-    mut builder: super::types::builders::MessageStartEventBuilder,
-) -> super::types::builders::MessageStartEventBuilder {
-    if builder.role.is_none() {
-        builder.role = "no value was set".parse::<super::types::ConversationRole>().ok()
-    }
-    builder
-}
-
-pub(crate) fn message_stop_event_correct_errors(
-    mut builder: super::types::builders::MessageStopEventBuilder,
-) -> super::types::builders::MessageStopEventBuilder {
-    if builder.stop_reason.is_none() {
-        builder.stop_reason = "no value was set".parse::<super::types::StopReason>().ok()
-    }
-    builder
-}
-
-pub(crate) fn converse_stream_metrics_correct_errors(
-    mut builder: super::types::builders::ConverseStreamMetricsBuilder,
-) -> super::types::builders::ConverseStreamMetricsBuilder {
-    if builder.latency_ms.is_none() {
-        builder.latency_ms = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn guardrail_checks_content_filter_result_entry_correct_errors(
     mut builder: super::types::builders::GuardrailChecksContentFilterResultEntryBuilder,
 ) -> super::types::builders::GuardrailChecksContentFilterResultEntryBuilder {
@@ -446,6 +446,45 @@
     builder
 }

+pub(crate) fn image_block_start_correct_errors(
+    mut builder: super::types::builders::ImageBlockStartBuilder,
+) -> super::types::builders::ImageBlockStartBuilder {
+    if builder.format.is_none() {
+        builder.format = "no value was set".parse::<super::types::ImageFormat>().ok()
+    }
+    builder
+}
+
+pub(crate) fn tool_result_block_start_correct_errors(
+    mut builder: super::types::builders::ToolResultBlockStartBuilder,
+) -> super::types::builders::ToolResultBlockStartBuilder {
+    if builder.tool_use_id.is_none() {
+        builder.tool_use_id = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn tool_use_block_delta_correct_errors(
+    mut builder: super::types::builders::ToolUseBlockDeltaBuilder,
+) -> super::types::builders::ToolUseBlockDeltaBuilder {
+    if builder.input.is_none() {
+        builder.input = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn tool_use_block_start_correct_errors(
+    mut builder: super::types::builders::ToolUseBlockStartBuilder,
+) -> super::types::builders::ToolUseBlockStartBuilder {
+    if builder.tool_use_id.is_none() {
+        builder.tool_use_id = Some(Default::default())
+    }
+    if builder.name.is_none() {
+        builder.name = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn audio_block_correct_errors(mut builder: super::types::builders::AudioBlockBuilder) -> super::types::builders::AudioBlockBuilder {
     if builder.format.is_none() {
         builder.format = "no value was set".parse::<super::types::AudioFormat>().ok()
@@ -586,15 +625,6 @@
     builder
 }

-pub(crate) fn image_block_start_correct_errors(
-    mut builder: super::types::builders::ImageBlockStartBuilder,
-) -> super::types::builders::ImageBlockStartBuilder {
-    if builder.format.is_none() {
-        builder.format = "no value was set".parse::<super::types::ImageFormat>().ok()
-    }
-    builder
-}
-
 pub(crate) fn search_result_block_correct_errors(
     mut builder: super::types::builders::SearchResultBlockBuilder,
 ) -> super::types::builders::SearchResultBlockBuilder {
@@ -646,15 +676,6 @@
     builder
 }

-pub(crate) fn tool_result_block_start_correct_errors(
-    mut builder: super::types::builders::ToolResultBlockStartBuilder,
-) -> super::types::builders::ToolResultBlockStartBuilder {
-    if builder.tool_use_id.is_none() {
-        builder.tool_use_id = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn tool_use_block_correct_errors(mut builder: super::types::builders::ToolUseBlockBuilder) -> super::types::builders::ToolUseBlockBuilder {
     if builder.tool_use_id.is_none() {
         builder.tool_use_id = Some(Default::default())
@@ -668,27 +689,6 @@
     builder
 }

-pub(crate) fn tool_use_block_delta_correct_errors(
-    mut builder: super::types::builders::ToolUseBlockDeltaBuilder,
-) -> super::types::builders::ToolUseBlockDeltaBuilder {
-    if builder.input.is_none() {
-        builder.input = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn tool_use_block_start_correct_errors(
-    mut builder: super::types::builders::ToolUseBlockStartBuilder,
-) -> super::types::builders::ToolUseBlockStartBuilder {
-    if builder.tool_use_id.is_none() {
-        builder.tool_use_id = Some(Default::default())
-    }
-    if builder.name.is_none() {
-        builder.name = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn video_block_correct_errors(mut builder: super::types::builders::VideoBlockBuilder) -> super::types::builders::VideoBlockBuilder {
     if builder.format.is_none() {
         builder.format = "no value was set".parse::<super::types::VideoFormat>().ok()
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -3,12 +3,6 @@

 pub use super::super::super::types::error::_internal_server_exception::InternalServerExceptionBuilder;

-pub use super::super::super::types::error::_throttling_exception::ThrottlingExceptionBuilder;
-
-pub use super::super::super::types::error::_validation_exception::ValidationExceptionBuilder;
-
-pub use super::super::super::types::error::_conflict_exception::ConflictExceptionBuilder;
-
 pub use super::super::super::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;

 pub use super::super::super::types::error::_service_quota_exceeded_exception::ServiceQuotaExceededExceptionBuilder;
@@ -15,6 +9,10 @@

 pub use super::super::super::types::error::_service_unavailable_exception::ServiceUnavailableExceptionBuilder;

+pub use super::super::super::types::error::_throttling_exception::ThrottlingExceptionBuilder;
+
+pub use super::super::super::types::error::_validation_exception::ValidationExceptionBuilder;
+
 pub use super::super::super::types::error::_model_error_exception::ModelErrorExceptionBuilder;

 pub use super::super::super::types::error::_model_not_ready_exception::ModelNotReadyExceptionBuilder;
@@ -22,3 +20,5 @@
 pub use super::super::super::types::error::_model_timeout_exception::ModelTimeoutExceptionBuilder;

 pub use super::super::super::types::error::_model_stream_error_exception::ModelStreamErrorExceptionBuilder;
+
+pub use super::super::super::types::error::_conflict_exception::ConflictExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -3,12 +3,6 @@

 pub use super::super::types::error::_internal_server_exception::InternalServerException;

-pub use super::super::types::error::_throttling_exception::ThrottlingException;
-
-pub use super::super::types::error::_validation_exception::ValidationException;
-
-pub use super::super::types::error::_conflict_exception::ConflictException;
-
 pub use super::super::types::error::_resource_not_found_exception::ResourceNotFoundException;

 pub use super::super::types::error::_service_quota_exceeded_exception::ServiceQuotaExceededException;
@@ -15,6 +9,10 @@

 pub use super::super::types::error::_service_unavailable_exception::ServiceUnavailableException;

+pub use super::super::types::error::_throttling_exception::ThrottlingException;
+
+pub use super::super::types::error::_validation_exception::ValidationException;
+
 pub use super::super::types::error::_model_error_exception::ModelErrorException;

 pub use super::super::types::error::_model_not_ready_exception::ModelNotReadyException;
@@ -23,6 +21,8 @@

 pub use super::super::types::error::_model_stream_error_exception::ModelStreamErrorException;

+pub use super::super::types::error::_conflict_exception::ConflictException;
+
 /// Error type for the `ConverseStreamOutputError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -78,23 +78,23 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::InternalServerException`.
+    /// Returns `true` if the error kind is `InternalServerException::InternalServerException`.
     pub fn is_internal_server_exception(&self) -> bool {
         matches!(self, Self::InternalServerException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ModelStreamErrorException`.
+    /// Returns `true` if the error kind is `ModelStreamErrorException::ModelStreamErrorException`.
     pub fn is_model_stream_error_exception(&self) -> bool {
         matches!(self, Self::ModelStreamErrorException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ValidationException`.
+    /// Returns `true` if the error kind is `ValidationException::ValidationException`.
     pub fn is_validation_exception(&self) -> bool {
         matches!(self, Self::ValidationException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ThrottlingException`.
+    /// Returns `true` if the error kind is `ThrottlingException::ThrottlingException`.
     pub fn is_throttling_exception(&self) -> bool {
         matches!(self, Self::ThrottlingException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ServiceUnavailableException`.
+    /// Returns `true` if the error kind is `ServiceUnavailableException::ServiceUnavailableException`.
     pub fn is_service_unavailable_exception(&self) -> bool {
         matches!(self, Self::ServiceUnavailableException(_))
     }
@@ -140,11 +140,11 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ConverseStreamOutputError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelStreamErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelStreamErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ThrottlingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -165,7 +165,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `InvokeModelWithBidirectionalStreamInputError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -258,7 +257,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `InvokeModelWithBidirectionalStreamOutputError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -317,27 +315,27 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::InternalServerException`.
+    /// Returns `true` if the error kind is `InternalServerException::InternalServerException`.
     pub fn is_internal_server_exception(&self) -> bool {
         matches!(self, Self::InternalServerException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ModelStreamErrorException`.
+    /// Returns `true` if the error kind is `ModelStreamErrorException::ModelStreamErrorException`.
     pub fn is_model_stream_error_exception(&self) -> bool {
         matches!(self, Self::ModelStreamErrorException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ValidationException`.
+    /// Returns `true` if the error kind is `ValidationException::ValidationException`.
     pub fn is_validation_exception(&self) -> bool {
         matches!(self, Self::ValidationException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ThrottlingException`.
+    /// Returns `true` if the error kind is `ThrottlingException::ThrottlingException`.
     pub fn is_throttling_exception(&self) -> bool {
         matches!(self, Self::ThrottlingException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ModelTimeoutException`.
+    /// Returns `true` if the error kind is `ModelTimeoutException::ModelTimeoutException`.
     pub fn is_model_timeout_exception(&self) -> bool {
         matches!(self, Self::ModelTimeoutException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ServiceUnavailableException`.
+    /// Returns `true` if the error kind is `ServiceUnavailableException::ServiceUnavailableException`.
     pub fn is_service_unavailable_exception(&self) -> bool {
         matches!(self, Self::ServiceUnavailableException(_))
     }
@@ -385,12 +383,12 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for InvokeModelWithBidirectionalStreamOutputError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelStreamErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelTimeoutException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelStreamErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ThrottlingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelTimeoutException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -411,7 +409,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `ResponseStreamError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -470,27 +467,27 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::InternalServerException`.
+    /// Returns `true` if the error kind is `InternalServerException::InternalServerException`.
     pub fn is_internal_server_exception(&self) -> bool {
         matches!(self, Self::InternalServerException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ModelStreamErrorException`.
+    /// Returns `true` if the error kind is `ModelStreamErrorException::ModelStreamErrorException`.
     pub fn is_model_stream_error_exception(&self) -> bool {
         matches!(self, Self::ModelStreamErrorException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ValidationException`.
+    /// Returns `true` if the error kind is `ValidationException::ValidationException`.
     pub fn is_validation_exception(&self) -> bool {
         matches!(self, Self::ValidationException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ThrottlingException`.
+    /// Returns `true` if the error kind is `ThrottlingException::ThrottlingException`.
     pub fn is_throttling_exception(&self) -> bool {
         matches!(self, Self::ThrottlingException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ModelTimeoutException`.
+    /// Returns `true` if the error kind is `ModelTimeoutException::ModelTimeoutException`.
     pub fn is_model_timeout_exception(&self) -> bool {
         matches!(self, Self::ModelTimeoutException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ServiceUnavailableException`.
+    /// Returns `true` if the error kind is `ServiceUnavailableException::ServiceUnavailableException`.
     pub fn is_service_unavailable_exception(&self) -> bool {
         matches!(self, Self::ServiceUnavailableException(_))
     }
@@ -538,12 +535,12 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ResponseStreamError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelStreamErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelTimeoutException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelStreamErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ThrottlingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelTimeoutException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -564,7 +561,6 @@
         self.meta().request_id()
     }
 }
-
 mod _access_denied_exception;

 mod _conflict_exception;
```
