# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sqs`
- model: `models/sqs.json`
- package: `aws-sdk-sqs-modular`
- library: `aws_sdk_sqs_modular`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_add_permission`: `AddPermission`
- `op_cancel_message_move_task`: `CancelMessageMoveTask`
- `op_change_message_visibility`: `ChangeMessageVisibility`
- `op_change_message_visibility_batch`: `ChangeMessageVisibilityBatch`
- `op_create_queue`: `CreateQueue`
- `op_delete_message`: `DeleteMessage`
- `op_delete_message_batch`: `DeleteMessageBatch`
- `op_delete_queue`: `DeleteQueue`
- `op_get_queue_attributes`: `GetQueueAttributes`
- `op_get_queue_url`: `GetQueueUrl`
- `op_list_dead_letter_source_queues`: `ListDeadLetterSourceQueues`
- `op_list_message_move_tasks`: `ListMessageMoveTasks`
- `op_list_queues`: `ListQueues`
- `op_list_queue_tags`: `ListQueueTags`
- `op_purge_queue`: `PurgeQueue`
- `op_receive_message`: `ReceiveMessage`
- `op_remove_permission`: `RemovePermission`
- `op_send_message`: `SendMessage`
- `op_send_message_batch`: `SendMessageBatch`
- `op_set_queue_attributes`: `SetQueueAttributes`
- `op_start_message_move_task`: `StartMessageMoveTask`
- `op_tag_queue`: `TagQueue`
- `op_untag_queue`: `UntagQueue`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/error_meta.rs`
- `src/lens.rs`
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
