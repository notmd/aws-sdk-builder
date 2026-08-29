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
- `src/operation/add_permission/_add_permission_input.rs`
- `src/operation/add_permission/_add_permission_output.rs`
- `src/operation/add_permission/builders.rs`
- `src/operation/cancel_message_move_task/_cancel_message_move_task_input.rs`
- `src/operation/cancel_message_move_task/_cancel_message_move_task_output.rs`
- `src/operation/cancel_message_move_task/builders.rs`
- `src/operation/change_message_visibility/_change_message_visibility_input.rs`
- `src/operation/change_message_visibility/_change_message_visibility_output.rs`
- `src/operation/change_message_visibility/builders.rs`
- `src/operation/change_message_visibility_batch/_change_message_visibility_batch_input.rs`
- `src/operation/change_message_visibility_batch/_change_message_visibility_batch_output.rs`
- `src/operation/change_message_visibility_batch/builders.rs`
- `src/operation/create_queue/_create_queue_input.rs`
- `src/operation/create_queue/_create_queue_output.rs`
- `src/operation/create_queue/builders.rs`
- `src/operation/delete_message/_delete_message_input.rs`
- `src/operation/delete_message/_delete_message_output.rs`
- `src/operation/delete_message/builders.rs`
- `src/operation/delete_message_batch/_delete_message_batch_input.rs`
- `src/operation/delete_message_batch/_delete_message_batch_output.rs`
- `src/operation/delete_message_batch/builders.rs`
- `src/operation/delete_queue/_delete_queue_input.rs`
- `src/operation/delete_queue/_delete_queue_output.rs`
- `src/operation/delete_queue/builders.rs`
- `src/operation/get_queue_attributes/_get_queue_attributes_input.rs`
- `src/operation/get_queue_attributes/_get_queue_attributes_output.rs`
- `src/operation/get_queue_attributes/builders.rs`
- `src/operation/get_queue_url/_get_queue_url_input.rs`
- `src/operation/get_queue_url/_get_queue_url_output.rs`
- `src/operation/get_queue_url/builders.rs`
- `src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_input.rs`
- `src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_output.rs`
- `src/operation/list_dead_letter_source_queues/builders.rs`
- `src/operation/list_dead_letter_source_queues/paginator.rs`
- `src/operation/list_message_move_tasks/_list_message_move_tasks_input.rs`
- `src/operation/list_message_move_tasks/_list_message_move_tasks_output.rs`
- `src/operation/list_message_move_tasks/builders.rs`
- `src/operation/list_queue_tags/_list_queue_tags_input.rs`
- `src/operation/list_queue_tags/_list_queue_tags_output.rs`
- `src/operation/list_queue_tags/builders.rs`
- `src/operation/list_queues/_list_queues_input.rs`
- `src/operation/list_queues/_list_queues_output.rs`
- `src/operation/list_queues/builders.rs`
- `src/operation/list_queues/paginator.rs`
- `src/operation/purge_queue/_purge_queue_input.rs`
- `src/operation/purge_queue/_purge_queue_output.rs`
- `src/operation/purge_queue/builders.rs`
- `src/operation/receive_message/_receive_message_input.rs`
- `src/operation/receive_message/_receive_message_output.rs`
- `src/operation/receive_message/builders.rs`
- `src/operation/remove_permission/_remove_permission_input.rs`
- `src/operation/remove_permission/_remove_permission_output.rs`
- `src/operation/remove_permission/builders.rs`
- `src/operation/send_message/_send_message_input.rs`
- `src/operation/send_message/_send_message_output.rs`
- `src/operation/send_message/builders.rs`
- `src/operation/send_message_batch/_send_message_batch_input.rs`
- `src/operation/send_message_batch/_send_message_batch_output.rs`
- `src/operation/send_message_batch/builders.rs`
- `src/operation/set_queue_attributes/_set_queue_attributes_input.rs`
- `src/operation/set_queue_attributes/_set_queue_attributes_output.rs`
- `src/operation/set_queue_attributes/builders.rs`
- `src/operation/start_message_move_task/_start_message_move_task_input.rs`
- `src/operation/start_message_move_task/_start_message_move_task_output.rs`
- `src/operation/start_message_move_task/builders.rs`
- `src/operation/tag_queue/_tag_queue_input.rs`
- `src/operation/tag_queue/_tag_queue_output.rs`
- `src/operation/tag_queue/builders.rs`
- `src/operation/untag_queue/_untag_queue_input.rs`
- `src/operation/untag_queue/_untag_queue_output.rs`
- `src/operation/untag_queue/builders.rs`
- `src/protocol_serde.rs`
- `src/serde_util.rs`

## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
