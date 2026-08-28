# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/bedrockruntime`
- model: `models/bedrockruntime.json`
- package: `aws-sdk-bedrockruntime-modular`
- library: `aws_sdk_bedrockruntime_modular`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_get_async_invoke`: `GetAsyncInvoke`
- `op_list_async_invokes`: `ListAsyncInvokes`
- `op_start_async_invoke`: `StartAsyncInvoke`
- `op_apply_guardrail`: `ApplyGuardrail`
- `op_invoke_guardrail_checks`: `InvokeGuardrailChecks`
- `op_converse`: `Converse`
- `op_converse_stream`: `ConverseStream`
- `op_invoke_model`: `InvokeModel`
- `op_invoke_model_with_bidirectional_stream`: `InvokeModelWithBidirectionalStream`
- `op_invoke_model_with_response_stream`: `InvokeModelWithResponseStream`
- `op_count_tokens`: `CountTokens`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/client/apply_guardrail.rs`
- `src/client/converse.rs`
- `src/client/converse_stream.rs`
- `src/client/count_tokens.rs`
- `src/client/get_async_invoke.rs`
- `src/client/invoke_guardrail_checks.rs`
- `src/client/invoke_model.rs`
- `src/client/invoke_model_with_bidirectional_stream.rs`
- `src/client/invoke_model_with_response_stream.rs`
- `src/client/list_async_invokes.rs`
- `src/client/start_async_invoke.rs`
- `src/error_meta.rs`
- `src/event_stream_serde.rs`
- `src/lens.rs`
- `src/operation.rs`
- `src/operation/apply_guardrail.rs`
- `src/operation/apply_guardrail/_apply_guardrail_input.rs`
- `src/operation/apply_guardrail/_apply_guardrail_output.rs`
- `src/operation/apply_guardrail/builders.rs`
- `src/operation/converse.rs`
- `src/operation/converse/_converse_input.rs`
- `src/operation/converse/_converse_output.rs`
- `src/operation/converse/builders.rs`
- `src/operation/converse_stream.rs`
- `src/operation/converse_stream/_converse_stream_input.rs`
- `src/operation/converse_stream/_converse_stream_output.rs`
- `src/operation/converse_stream/builders.rs`
- `src/operation/count_tokens.rs`
- `src/operation/count_tokens/_count_tokens_input.rs`
- `src/operation/count_tokens/_count_tokens_output.rs`
- `src/operation/count_tokens/builders.rs`
- `src/operation/get_async_invoke.rs`
- `src/operation/get_async_invoke/_get_async_invoke_input.rs`
- `src/operation/get_async_invoke/_get_async_invoke_output.rs`
- `src/operation/get_async_invoke/builders.rs`
- `src/operation/invoke_guardrail_checks.rs`
- `src/operation/invoke_guardrail_checks/_invoke_guardrail_checks_input.rs`
- `src/operation/invoke_guardrail_checks/_invoke_guardrail_checks_output.rs`
- `src/operation/invoke_guardrail_checks/builders.rs`
- `src/operation/invoke_model.rs`
- `src/operation/invoke_model/_invoke_model_input.rs`
- `src/operation/invoke_model/_invoke_model_output.rs`
- `src/operation/invoke_model/builders.rs`
- `src/operation/invoke_model_with_bidirectional_stream.rs`
- `src/operation/invoke_model_with_bidirectional_stream/_invoke_model_with_bidirectional_stream_input.rs`
- `src/operation/invoke_model_with_bidirectional_stream/_invoke_model_with_bidirectional_stream_output.rs`
- `src/operation/invoke_model_with_bidirectional_stream/builders.rs`
- `src/operation/invoke_model_with_response_stream.rs`
- `src/operation/invoke_model_with_response_stream/_invoke_model_with_response_stream_input.rs`
- `src/operation/invoke_model_with_response_stream/_invoke_model_with_response_stream_output.rs`
- `src/operation/invoke_model_with_response_stream/builders.rs`
- `src/operation/list_async_invokes.rs`
- `src/operation/list_async_invokes/_list_async_invokes_input.rs`
- `src/operation/list_async_invokes/_list_async_invokes_output.rs`
- `src/operation/list_async_invokes/builders.rs`
- `src/operation/list_async_invokes/paginator.rs`
- `src/operation/start_async_invoke.rs`
- `src/operation/start_async_invoke/_start_async_invoke_input.rs`
- `src/operation/start_async_invoke/_start_async_invoke_output.rs`
- `src/operation/start_async_invoke/builders.rs`
- `src/protocol_serde.rs`
- `src/protocol_serde/shape_apply_guardrail.rs`
- `src/protocol_serde/shape_apply_guardrail_input.rs`
- `src/protocol_serde/shape_converse.rs`
- `src/protocol_serde/shape_converse_input.rs`
- `src/protocol_serde/shape_converse_metrics.rs`
- `src/protocol_serde/shape_converse_output.rs`
- `src/protocol_serde/shape_converse_stream.rs`
- `src/protocol_serde/shape_converse_stream_input.rs`
- `src/protocol_serde/shape_converse_stream_metadata_event.rs`
- `src/protocol_serde/shape_converse_stream_metrics.rs`
- `src/protocol_serde/shape_converse_stream_output.rs`
- `src/protocol_serde/shape_converse_stream_trace.rs`
- `src/protocol_serde/shape_converse_tokens_request.rs`
- `src/protocol_serde/shape_converse_trace.rs`
- `src/protocol_serde/shape_count_tokens.rs`
- `src/protocol_serde/shape_count_tokens_input.rs`
- `src/protocol_serde/shape_get_async_invoke.rs`
- `src/protocol_serde/shape_invoke_guardrail_checks.rs`
- `src/protocol_serde/shape_invoke_guardrail_checks_input.rs`
- `src/protocol_serde/shape_invoke_model.rs`
- `src/protocol_serde/shape_invoke_model_input.rs`
- `src/protocol_serde/shape_invoke_model_output.rs`
- `src/protocol_serde/shape_invoke_model_tokens_request.rs`
- `src/protocol_serde/shape_invoke_model_with_bidirectional_stream.rs`
- `src/protocol_serde/shape_invoke_model_with_bidirectional_stream_input.rs`
- `src/protocol_serde/shape_invoke_model_with_bidirectional_stream_output.rs`
- `src/protocol_serde/shape_invoke_model_with_response_stream.rs`
- `src/protocol_serde/shape_invoke_model_with_response_stream_input.rs`
- `src/protocol_serde/shape_invoke_model_with_response_stream_output.rs`
- `src/protocol_serde/shape_list_async_invokes.rs`
- `src/protocol_serde/shape_start_async_invoke.rs`
- `src/protocol_serde/shape_start_async_invoke_input.rs`
- `src/serde_util.rs`

## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
