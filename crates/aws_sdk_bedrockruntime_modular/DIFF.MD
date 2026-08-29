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
- `src/config.rs`
- `src/error_meta.rs`
- `src/event_stream_serde.rs`
- `src/lens.rs`
- `src/lib.rs`
- `src/operation.rs`
- `src/protocol_serde.rs`
- `src/serde_util.rs`
- `src/types.rs`
- `src/types/builders.rs`
- `src/types/error.rs`
- `src/types/error/builders.rs`

## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
