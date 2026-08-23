# AWS SDK Conformance Report: sqs

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sqs
**Progress:** `299/299` files compared · `150` matched · `35` mismatches · `114` missing · `0` extra · `50.17%` match (100.00% means fully matched)

### `src/client/change_message_visibility.rs`

```diff
--- reference/src/client/change_message_visibility.rs
+++ generated/src/client/change_message_visibility.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_url(impl Into<String>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue whose message's visibility is changed.</p> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`receipt_handle(impl Into<String>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::receipt_handle) / [`set_receipt_handle(Option<String>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::set_receipt_handle):<br>required: **true**<br><p>The receipt handle associated with the message, whose visibility timeout is changed. This parameter is returned by the <code> <code>ReceiveMessage</code> </code> action.</p><br>
+    ///   - [`receipt_handle(impl Into<String>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::receipt_handle) / [`set_receipt_handle(Option<String>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::set_receipt_handle):<br>required: **true**<br><p>The receipt handle associated with the message, whose visibility timeout is changed. This parameter is returned by the <code> <a>ReceiveMessage</a> </code> action.</p><br>
     ///   - [`visibility_timeout(i32)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::visibility_timeout) / [`set_visibility_timeout(Option<i32>)`](crate::operation::change_message_visibility::builders::ChangeMessageVisibilityFluentBuilder::set_visibility_timeout):<br>required: **true**<br><p>The new value for the message's visibility timeout (in seconds). Values range: <code>0</code> to <code>43200</code>. Maximum: 12 hours.</p><br>
     /// - On success, responds with [`ChangeMessageVisibilityOutput`](crate::operation::change_message_visibility::ChangeMessageVisibilityOutput)
     /// - On failure, responds with [`SdkError<ChangeMessageVisibilityError>`](crate::operation::change_message_visibility::ChangeMessageVisibilityError)
```

### `src/client/change_message_visibility_batch.rs`

```diff
--- reference/src/client/change_message_visibility_batch.rs
+++ generated/src/client/change_message_visibility_batch.rs
@@ -6,8 +6,8 @@
     ///   - [`queue_url(impl Into<String>)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p> <p>Queue URLs and names are case-sensitive.</p><br>
     ///   - [`entries(ChangeMessageVisibilityBatchRequestEntry)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::entries) / [`set_entries(Option<Vec::<ChangeMessageVisibilityBatchRequestEntry>>)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::set_entries):<br>required: **true**<br><p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p><br>
     /// - On success, responds with [`ChangeMessageVisibilityBatchOutput`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput) with field(s):
-    ///   - [`successful(Vec::<ChangeMessageVisibilityBatchResultEntry>)`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput::successful): <p>A list of <code> <code>ChangeMessageVisibilityBatchResultEntry</code> </code> items.</p>
-    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput::failed): <p>A list of <code> <code>BatchResultErrorEntry</code> </code> items.</p>
+    ///   - [`successful(Vec::<ChangeMessageVisibilityBatchResultEntry>)`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput::successful): <p>A list of <code> <a>ChangeMessageVisibilityBatchResultEntry</a> </code> items.</p>
+    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput::failed): <p>A list of <code> <a>BatchResultErrorEntry</a> </code> items.</p>
     /// - On failure, responds with [`SdkError<ChangeMessageVisibilityBatchError>`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError)
     pub fn change_message_visibility_batch(
         &self,
```

### `src/client/create_queue.rs`

```diff
--- reference/src/client/create_queue.rs
+++ generated/src/client/create_queue.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_name(impl Into<String>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::queue_name) / [`set_queue_name(Option<String>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::set_queue_name):<br>required: **true**<br><p>The name of the new queue. The following limits apply to this name:</p> <ul>  <li>   <p>A queue name can have up to 80 characters.</p></li>  <li>   <p>Valid values: alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p></li>  <li>   <p>A FIFO queue name must end with the <code>.fifo</code> suffix.</p></li> </ul> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`attributes(QueueAttributeName, impl Into<String>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<QueueAttributeName, String>>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::set_attributes):<br>required: **false**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>CreateQueue</code> action uses:</p> <ul>  <li>   <p><code>DelaySeconds</code> – The length of time, in seconds, for which the delivery of all messages in the queue is delayed. Valid values: An integer from 0 to 900 seconds (15 minutes). Default: 0.</p></li>  <li>   <p><code>MaximumMessageSize</code> – The limit of how many bytes a message can contain before Amazon SQS rejects it. Valid values: An integer from 1,024 bytes (1 KiB) to 1,048,576 bytes (1 MiB). Default: 1,048,576 bytes (1 MiB).</p></li>  <li>   <p><code>MessageRetentionPeriod</code> – The length of time, in seconds, for which Amazon SQS retains a message. Valid values: An integer from 60 seconds (1 minute) to 1,209,600 seconds (14 days). Default: 345,600 (4 days). When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes and will impact existing messages in the queue potentially causing them to be expired and deleted if the <code>MessageRetentionPeriod</code> is reduced below the age of existing messages.</p></li>  <li>   <p><code>Policy</code> – The queue's policy. A valid Amazon Web Services policy. For more information about policy structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html">Overview of Amazon Web Services IAM Policies</a> in the <i>IAM User Guide</i>.</p></li>  <li>   <p><code>ReceiveMessageWaitTimeSeconds</code> – The length of time, in seconds, for which a <code> <code>ReceiveMessage</code> </code> action waits for a message to arrive. Valid values: An integer from 0 to 20 (seconds). Default: 0.</p></li>  <li>   <p><code>VisibilityTimeout</code> – The visibility timeout for the queue, in seconds. Valid values: An integer from 0 to 43,200 (12 hours). Default: 30. For more information about the visibility timeout, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues:</a></p> <ul>  <li>   <p><code>RedrivePolicy</code> – The string that includes the parameters for the dead-letter queue functionality of the source queue as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>deadLetterTargetArn</code> – The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p></li>    <li>     <p><code>maxReceiveCount</code> – The number of times a message is delivered to the source queue before being moved to the dead-letter queue. Default: 10. When the <code>ReceiveCount</code> for a message exceeds the <code>maxReceiveCount</code> for a queue, Amazon SQS moves the message to the dead-letter-queue.</p></li>   </ul></li>  <li>   <p><code>RedriveAllowPolicy</code> – The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>redrivePermission</code> – The permission type that defines which source queues can specify the current queue as the dead-letter queue. Valid values are:</p>     <ul>      <li>       <p><code>allowAll</code> – (Default) Any source queues in this Amazon Web Services account in the same Region can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>denyAll</code> – No source queues can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>byQueue</code> – Only queues specified by the <code>sourceQueueArns</code> parameter can specify this queue as the dead-letter queue.</p></li>     </ul></li>    <li>     <p><code>sourceQueueArns</code> – The Amazon Resource Names (ARN)s of the source queues that can specify this queue as the dead-letter queue and redrive messages. You can specify this parameter only when the <code>redrivePermission</code> parameter is set to <code>byQueue</code>. You can specify up to 10 source queue ARNs. To allow more than 10 source queues to specify dead-letter queues, set the <code>redrivePermission</code> parameter to <code>allowAll</code>.</p></li>   </ul></li> </ul><note>  <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. While the alias of the Amazon Web Services managed CMK for Amazon SQS is always <code>alias/aws/sqs</code>, the alias of a custom CMK can, for example, be <code>alias/<i>MyAlias</i> </code>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>  <li>   <p><code>KmsDataKeyReusePeriodSeconds</code> – The length of time, in seconds, for which Amazon SQS can reuse a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data key</a> to encrypt or decrypt messages before calling KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). Default: 300 (5 minutes). A shorter time period provides better security but results in more calls to KMS which might incur charges after Free Tier. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a></p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul>  <li>   <p><code>FifoQueue</code> – Designates a queue as FIFO. Valid values are <code>true</code> and <code>false</code>. If you don't specify the <code>FifoQueue</code> attribute, Amazon SQS creates a standard queue. You can provide this attribute only during queue creation. You can't change it for an existing queue. When you set this attribute, you must also provide the <code>MessageGroupId</code> for your messages explicitly.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-understanding-logic.html">FIFO queue logic</a> in the <i>Amazon SQS Developer Guide</i>.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication. Valid values are <code>true</code> and <code>false</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html">Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>. Note the following:</p>   <ul>    <li>     <p>Every message must have a unique <code>MessageDeduplicationId</code>.</p>     <ul>      <li>       <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>      <li>       <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>      <li>       <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>      <li>       <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>     </ul></li>    <li>     <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>    <li>     <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>   </ul></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/high-throughput-fifo.html">high throughput for FIFO queues</a>:</p> <ul>  <li>   <p><code>DeduplicationScope</code> – Specifies whether message deduplication occurs at the message group or queue level. Valid values are <code>messageGroup</code> and <code>queue</code>.</p></li>  <li>   <p><code>FifoThroughputLimit</code> – Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are <code>perQueue</code> and <code>perMessageGroupId</code>. The <code>perMessageGroupId</code> value is allowed only when the value for <code>DeduplicationScope</code> is <code>messageGroup</code>.</p></li> </ul> <p>To enable high throughput for FIFO queues, do the following:</p> <ul>  <li>   <p>Set <code>DeduplicationScope</code> to <code>messageGroup</code>.</p></li>  <li>   <p>Set <code>FifoThroughputLimit</code> to <code>perMessageGroupId</code>.</p></li> </ul> <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p> <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
+    ///   - [`attributes(QueueAttributeName, impl Into<String>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<QueueAttributeName, String>>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::set_attributes):<br>required: **false**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>CreateQueue</code> action uses:</p> <ul>  <li>   <p><code>DelaySeconds</code> – The length of time, in seconds, for which the delivery of all messages in the queue is delayed. Valid values: An integer from 0 to 900 seconds (15 minutes). Default: 0.</p></li>  <li>   <p><code>MaximumMessageSize</code> – The limit of how many bytes a message can contain before Amazon SQS rejects it. Valid values: An integer from 1,024 bytes (1 KiB) to 1,048,576 bytes (1 MiB). Default: 1,048,576 bytes (1 MiB).</p></li>  <li>   <p><code>MessageRetentionPeriod</code> – The length of time, in seconds, for which Amazon SQS retains a message. Valid values: An integer from 60 seconds (1 minute) to 1,209,600 seconds (14 days). Default: 345,600 (4 days). When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes and will impact existing messages in the queue potentially causing them to be expired and deleted if the <code>MessageRetentionPeriod</code> is reduced below the age of existing messages.</p></li>  <li>   <p><code>Policy</code> – The queue's policy. A valid Amazon Web Services policy. For more information about policy structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html">Overview of Amazon Web Services IAM Policies</a> in the <i>IAM User Guide</i>.</p></li>  <li>   <p><code>ReceiveMessageWaitTimeSeconds</code> – The length of time, in seconds, for which a <code> <a>ReceiveMessage</a> </code> action waits for a message to arrive. Valid values: An integer from 0 to 20 (seconds). Default: 0.</p></li>  <li>   <p><code>VisibilityTimeout</code> – The visibility timeout for the queue, in seconds. Valid values: An integer from 0 to 43,200 (12 hours). Default: 30. For more information about the visibility timeout, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues:</a></p> <ul>  <li>   <p><code>RedrivePolicy</code> – The string that includes the parameters for the dead-letter queue functionality of the source queue as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>deadLetterTargetArn</code> – The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p></li>    <li>     <p><code>maxReceiveCount</code> – The number of times a message is delivered to the source queue before being moved to the dead-letter queue. Default: 10. When the <code>ReceiveCount</code> for a message exceeds the <code>maxReceiveCount</code> for a queue, Amazon SQS moves the message to the dead-letter-queue.</p></li>   </ul></li>  <li>   <p><code>RedriveAllowPolicy</code> – The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>redrivePermission</code> – The permission type that defines which source queues can specify the current queue as the dead-letter queue. Valid values are:</p>     <ul>      <li>       <p><code>allowAll</code> – (Default) Any source queues in this Amazon Web Services account in the same Region can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>denyAll</code> – No source queues can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>byQueue</code> – Only queues specified by the <code>sourceQueueArns</code> parameter can specify this queue as the dead-letter queue.</p></li>     </ul></li>    <li>     <p><code>sourceQueueArns</code> – The Amazon Resource Names (ARN)s of the source queues that can specify this queue as the dead-letter queue and redrive messages. You can specify this parameter only when the <code>redrivePermission</code> parameter is set to <code>byQueue</code>. You can specify up to 10 source queue ARNs. To allow more than 10 source queues to specify dead-letter queues, set the <code>redrivePermission</code> parameter to <code>allowAll</code>.</p></li>   </ul></li> </ul> <note>  <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. While the alias of the Amazon Web Services managed CMK for Amazon SQS is always <code>alias/aws/sqs</code>, the alias of a custom CMK can, for example, be <code>alias/<i>MyAlias</i> </code>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>  <li>   <p><code>KmsDataKeyReusePeriodSeconds</code> – The length of time, in seconds, for which Amazon SQS can reuse a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data key</a> to encrypt or decrypt messages before calling KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). Default: 300 (5 minutes). A shorter time period provides better security but results in more calls to KMS which might incur charges after Free Tier. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a></p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul>  <li>   <p><code>FifoQueue</code> – Designates a queue as FIFO. Valid values are <code>true</code> and <code>false</code>. If you don't specify the <code>FifoQueue</code> attribute, Amazon SQS creates a standard queue. You can provide this attribute only during queue creation. You can't change it for an existing queue. When you set this attribute, you must also provide the <code>MessageGroupId</code> for your messages explicitly.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-understanding-logic.html">FIFO queue logic</a> in the <i>Amazon SQS Developer Guide</i>.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication. Valid values are <code>true</code> and <code>false</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html">Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>. Note the following:</p>   <ul>    <li>     <p>Every message must have a unique <code>MessageDeduplicationId</code>.</p>     <ul>      <li>       <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>      <li>       <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>      <li>       <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>      <li>       <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>     </ul></li>    <li>     <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>    <li>     <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>   </ul></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/high-throughput-fifo.html">high throughput for FIFO queues</a>:</p> <ul>  <li>   <p><code>DeduplicationScope</code> – Specifies whether message deduplication occurs at the message group or queue level. Valid values are <code>messageGroup</code> and <code>queue</code>.</p></li>  <li>   <p><code>FifoThroughputLimit</code> – Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are <code>perQueue</code> and <code>perMessageGroupId</code>. The <code>perMessageGroupId</code> value is allowed only when the value for <code>DeduplicationScope</code> is <code>messageGroup</code>.</p></li> </ul> <p>To enable high throughput for FIFO queues, do the following:</p> <ul>  <li>   <p>Set <code>DeduplicationScope</code> to <code>messageGroup</code>.</p></li>  <li>   <p>Set <code>FifoThroughputLimit</code> to <code>perMessageGroupId</code>.</p></li> </ul> <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p> <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_queue::builders::CreateQueueFluentBuilder::set_tags):<br>required: **false**<br><p>Add cost allocation tags to the specified Amazon SQS queue. For an overview, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-tags.html">Tagging Your Amazon SQS Queues</a> in the <i>Amazon SQS Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul>  <li>   <p>Adding more than 50 tags to a queue isn't recommended.</p></li>  <li>   <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p></li>  <li>   <p>Tags are case-sensitive.</p></li>  <li>   <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p></li> </ul> <p>For a full list of tag restrictions, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-limits.html#limits-queues">Quotas related to queues</a> in the <i>Amazon SQS Developer Guide</i>.</p><note>  <p>To be able to tag a queue on creation, you must have the <code>sqs:CreateQueue</code> and <code>sqs:TagQueue</code> permissions.</p>  <p>Cross-account permissions don't apply to this action. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-customer-managed-policy-examples.html#grant-cross-account-permissions-to-role-and-user-name">Grant cross-account permissions to a role and a username</a> in the <i>Amazon SQS Developer Guide</i>.</p> </note><br>
     /// - On success, responds with [`CreateQueueOutput`](crate::operation::create_queue::CreateQueueOutput) with field(s):
     ///   - [`queue_url(Option<String>)`](crate::operation::create_queue::CreateQueueOutput::queue_url): <p>The URL of the created Amazon SQS queue.</p>
```

### `src/client/delete_message_batch.rs`

```diff
--- reference/src/client/delete_message_batch.rs
+++ generated/src/client/delete_message_batch.rs
@@ -6,8 +6,8 @@
     ///   - [`queue_url(impl Into<String>)`](crate::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue from which messages are deleted.</p> <p>Queue URLs and names are case-sensitive.</p><br>
     ///   - [`entries(DeleteMessageBatchRequestEntry)`](crate::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder::entries) / [`set_entries(Option<Vec::<DeleteMessageBatchRequestEntry>>)`](crate::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder::set_entries):<br>required: **true**<br><p>Lists the receipt handles for the messages to be deleted.</p><br>
     /// - On success, responds with [`DeleteMessageBatchOutput`](crate::operation::delete_message_batch::DeleteMessageBatchOutput) with field(s):
-    ///   - [`successful(Vec::<DeleteMessageBatchResultEntry>)`](crate::operation::delete_message_batch::DeleteMessageBatchOutput::successful): <p>A list of <code> <code>DeleteMessageBatchResultEntry</code> </code> items.</p>
-    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::delete_message_batch::DeleteMessageBatchOutput::failed): <p>A list of <code> <code>BatchResultErrorEntry</code> </code> items.</p>
+    ///   - [`successful(Vec::<DeleteMessageBatchResultEntry>)`](crate::operation::delete_message_batch::DeleteMessageBatchOutput::successful): <p>A list of <code> <a>DeleteMessageBatchResultEntry</a> </code> items.</p>
+    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::delete_message_batch::DeleteMessageBatchOutput::failed): <p>A list of <code> <a>BatchResultErrorEntry</a> </code> items.</p>
     /// - On failure, responds with [`SdkError<DeleteMessageBatchError>`](crate::operation::delete_message_batch::DeleteMessageBatchError)
     pub fn delete_message_batch(&self) -> crate::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder {
         crate::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder::new(self.handle.clone())
```

### `src/client/get_queue_attributes.rs`

```diff
--- reference/src/client/get_queue_attributes.rs
+++ generated/src/client/get_queue_attributes.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_url(impl Into<String>)`](crate::operation::get_queue_attributes::builders::GetQueueAttributesFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::get_queue_attributes::builders::GetQueueAttributesFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue whose attribute information is retrieved.</p> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`attribute_names(QueueAttributeName)`](crate::operation::get_queue_attributes::builders::GetQueueAttributesFluentBuilder::attribute_names) / [`set_attribute_names(Option<Vec::<QueueAttributeName>>)`](crate::operation::get_queue_attributes::builders::GetQueueAttributesFluentBuilder::set_attribute_names):<br>required: **false**<br><p>A list of attributes for which to retrieve information.</p> <p>The <code>AttributeNames</code> parameter is optional, but if you don't specify values for this parameter, the request returns empty results.</p><note>  <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note> <p>The following attributes are supported:</p><important>  <p>The <code>ApproximateNumberOfMessagesDelayed</code>, <code>ApproximateNumberOfMessagesNotVisible</code>, and <code>ApproximateNumberOfMessages</code> metrics may not achieve consistency until at least 1 minute after the producers stop sending messages. This period is required for the queue metadata to reach eventual consistency.</p> </important> <ul>  <li>   <p><code>All</code> – Returns all values.</p></li>  <li>   <p><code>ApproximateNumberOfMessages</code> – Returns the approximate number of messages available for retrieval from the queue.</p></li>  <li>   <p><code>ApproximateNumberOfMessagesDelayed</code> – Returns the approximate number of messages in the queue that are delayed and not available for reading immediately. This can happen when the queue is configured as a delay queue or when a message has been sent with a delay parameter.</p></li>  <li>   <p><code>ApproximateNumberOfMessagesNotVisible</code> – Returns the approximate number of messages that are in flight. Messages are considered to be <i>in flight</i> if they have been sent to a client but have not yet been deleted or have not yet reached the end of their visibility window.</p></li>  <li>   <p><code>CreatedTimestamp</code> – Returns the time when the queue was created in seconds (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a>).</p></li>  <li>   <p><code>DelaySeconds</code> – Returns the default delay on the queue in seconds.</p></li>  <li>   <p><code>LastModifiedTimestamp</code> – Returns the time when the queue was last changed in seconds (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a>).</p></li>  <li>   <p><code>MaximumMessageSize</code> – Returns the limit of how many bytes a message can contain before Amazon SQS rejects it.</p></li>  <li>   <p><code>MessageRetentionPeriod</code> – Returns the length of time, in seconds, for which Amazon SQS retains a message. When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes and will impact existing messages in the queue potentially causing them to be expired and deleted if the <code>MessageRetentionPeriod</code> is reduced below the age of existing messages.</p></li>  <li>   <p><code>Policy</code> – Returns the policy of the queue.</p></li>  <li>   <p><code>QueueArn</code> – Returns the Amazon resource name (ARN) of the queue.</p></li>  <li>   <p><code>ReceiveMessageWaitTimeSeconds</code> – Returns the length of time, in seconds, for which the <code>ReceiveMessage</code> action waits for a message to arrive.</p></li>  <li>   <p><code>VisibilityTimeout</code> – Returns the visibility timeout for the queue. For more information about the visibility timeout, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues:</a></p> <ul>  <li>   <p><code>RedrivePolicy</code> – The string that includes the parameters for the dead-letter queue functionality of the source queue as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>deadLetterTargetArn</code> – The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p></li>    <li>     <p><code>maxReceiveCount</code> – The number of times a message is delivered to the source queue before being moved to the dead-letter queue. Default: 10. When the <code>ReceiveCount</code> for a message exceeds the <code>maxReceiveCount</code> for a queue, Amazon SQS moves the message to the dead-letter-queue.</p></li>   </ul></li>  <li>   <p><code>RedriveAllowPolicy</code> – The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>redrivePermission</code> – The permission type that defines which source queues can specify the current queue as the dead-letter queue. Valid values are:</p>     <ul>      <li>       <p><code>allowAll</code> – (Default) Any source queues in this Amazon Web Services account in the same Region can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>denyAll</code> – No source queues can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>byQueue</code> – Only queues specified by the <code>sourceQueueArns</code> parameter can specify this queue as the dead-letter queue.</p></li>     </ul></li>    <li>     <p><code>sourceQueueArns</code> – The Amazon Resource Names (ARN)s of the source queues that can specify this queue as the dead-letter queue and redrive messages. You can specify this parameter only when the <code>redrivePermission</code> parameter is set to <code>byQueue</code>. You can specify up to 10 source queue ARNs. To allow more than 10 source queues to specify dead-letter queues, set the <code>redrivePermission</code> parameter to <code>allowAll</code>.</p></li>   </ul></li> </ul><note>  <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – Returns the ID of an Amazon Web Services managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>.</p></li>  <li>   <p><code>KmsDataKeyReusePeriodSeconds</code> – Returns the length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling KMS again. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>.</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Returns information about whether the queue is using SSE-SQS encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul>  <li>   <p><code>FifoQueue</code> – Returns information about whether the queue is FIFO. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-understanding-logic.html">FIFO queue logic</a> in the <i>Amazon SQS Developer Guide</i>.</p><note>    <p>To determine whether a queue is <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO</a>, you can check whether <code>QueueName</code> ends with the <code>.fifo</code> suffix.</p>   </note></li>  <li>   <p><code>ContentBasedDeduplication</code> – Returns whether content-based deduplication is enabled for the queue. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html">Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/high-throughput-fifo.html">high throughput for FIFO queues</a>:</p> <ul>  <li>   <p><code>DeduplicationScope</code> – Specifies whether message deduplication occurs at the message group or queue level. Valid values are <code>messageGroup</code> and <code>queue</code>.</p></li>  <li>   <p><code>FifoThroughputLimit</code> – Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are <code>perQueue</code> and <code>perMessageGroupId</code>. The <code>perMessageGroupId</code> value is allowed only when the value for <code>DeduplicationScope</code> is <code>messageGroup</code>.</p></li> </ul> <p>To enable high throughput for FIFO queues, do the following:</p> <ul>  <li>   <p>Set <code>DeduplicationScope</code> to <code>messageGroup</code>.</p></li>  <li>   <p>Set <code>FifoThroughputLimit</code> to <code>perMessageGroupId</code>.</p></li> </ul> <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p> <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
+    ///   - [`attribute_names(QueueAttributeName)`](crate::operation::get_queue_attributes::builders::GetQueueAttributesFluentBuilder::attribute_names) / [`set_attribute_names(Option<Vec::<QueueAttributeName>>)`](crate::operation::get_queue_attributes::builders::GetQueueAttributesFluentBuilder::set_attribute_names):<br>required: **false**<br><p>A list of attributes for which to retrieve information.</p> <p>The <code>AttributeNames</code> parameter is optional, but if you don't specify values for this parameter, the request returns empty results.</p><note>  <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note> <p>The following attributes are supported:</p><important>  <p>The <code>ApproximateNumberOfMessagesDelayed</code>, <code>ApproximateNumberOfMessagesNotVisible</code>, and <code>ApproximateNumberOfMessages</code> metrics may not achieve consistency until at least 1 minute after the producers stop sending messages. This period is required for the queue metadata to reach eventual consistency.</p> </important> <ul>  <li>   <p><code>All</code> – Returns all values.</p></li>  <li>   <p><code>ApproximateNumberOfMessages</code> – Returns the approximate number of messages available for retrieval from the queue.</p></li>  <li>   <p><code>ApproximateNumberOfMessagesDelayed</code> – Returns the approximate number of messages in the queue that are delayed and not available for reading immediately. This can happen when the queue is configured as a delay queue or when a message has been sent with a delay parameter.</p></li>  <li>   <p><code>ApproximateNumberOfMessagesNotVisible</code> – Returns the approximate number of messages that are in flight. Messages are considered to be <i>in flight</i> if they have been sent to a client but have not yet been deleted or have not yet reached the end of their visibility window.</p></li>  <li>   <p><code>CreatedTimestamp</code> – Returns the time when the queue was created in seconds (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a>).</p></li>  <li>   <p><code>DelaySeconds</code> – Returns the default delay on the queue in seconds.</p></li>  <li>   <p><code>LastModifiedTimestamp</code> – Returns the time when the queue was last changed in seconds (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a>).</p></li>  <li>   <p><code>MaximumMessageSize</code> – Returns the limit of how many bytes a message can contain before Amazon SQS rejects it.</p></li>  <li>   <p><code>MessageRetentionPeriod</code> – Returns the length of time, in seconds, for which Amazon SQS retains a message. When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes and will impact existing messages in the queue potentially causing them to be expired and deleted if the <code>MessageRetentionPeriod</code> is reduced below the age of existing messages.</p></li>  <li>   <p><code>Policy</code> – Returns the policy of the queue.</p></li>  <li>   <p><code>QueueArn</code> – Returns the Amazon resource name (ARN) of the queue.</p></li>  <li>   <p><code>ReceiveMessageWaitTimeSeconds</code> – Returns the length of time, in seconds, for which the <code>ReceiveMessage</code> action waits for a message to arrive.</p></li>  <li>   <p><code>VisibilityTimeout</code> – Returns the visibility timeout for the queue. For more information about the visibility timeout, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues:</a></p> <ul>  <li>   <p><code>RedrivePolicy</code> – The string that includes the parameters for the dead-letter queue functionality of the source queue as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>deadLetterTargetArn</code> – The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p></li>    <li>     <p><code>maxReceiveCount</code> – The number of times a message is delivered to the source queue before being moved to the dead-letter queue. Default: 10. When the <code>ReceiveCount</code> for a message exceeds the <code>maxReceiveCount</code> for a queue, Amazon SQS moves the message to the dead-letter-queue.</p></li>   </ul></li>  <li>   <p><code>RedriveAllowPolicy</code> – The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>redrivePermission</code> – The permission type that defines which source queues can specify the current queue as the dead-letter queue. Valid values are:</p>     <ul>      <li>       <p><code>allowAll</code> – (Default) Any source queues in this Amazon Web Services account in the same Region can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>denyAll</code> – No source queues can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>byQueue</code> – Only queues specified by the <code>sourceQueueArns</code> parameter can specify this queue as the dead-letter queue.</p></li>     </ul></li>    <li>     <p><code>sourceQueueArns</code> – The Amazon Resource Names (ARN)s of the source queues that can specify this queue as the dead-letter queue and redrive messages. You can specify this parameter only when the <code>redrivePermission</code> parameter is set to <code>byQueue</code>. You can specify up to 10 source queue ARNs. To allow more than 10 source queues to specify dead-letter queues, set the <code>redrivePermission</code> parameter to <code>allowAll</code>.</p></li>   </ul></li> </ul> <note>  <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – Returns the ID of an Amazon Web Services managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>.</p></li>  <li>   <p><code>KmsDataKeyReusePeriodSeconds</code> – Returns the length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling KMS again. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>.</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Returns information about whether the queue is using SSE-SQS encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul>  <li>   <p><code>FifoQueue</code> – Returns information about whether the queue is FIFO. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-understanding-logic.html">FIFO queue logic</a> in the <i>Amazon SQS Developer Guide</i>.</p><note>    <p>To determine whether a queue is <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO</a>, you can check whether <code>QueueName</code> ends with the <code>.fifo</code> suffix.</p>   </note></li>  <li>   <p><code>ContentBasedDeduplication</code> – Returns whether content-based deduplication is enabled for the queue. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html">Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/high-throughput-fifo.html">high throughput for FIFO queues</a>:</p> <ul>  <li>   <p><code>DeduplicationScope</code> – Specifies whether message deduplication occurs at the message group or queue level. Valid values are <code>messageGroup</code> and <code>queue</code>.</p></li>  <li>   <p><code>FifoThroughputLimit</code> – Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are <code>perQueue</code> and <code>perMessageGroupId</code>. The <code>perMessageGroupId</code> value is allowed only when the value for <code>DeduplicationScope</code> is <code>messageGroup</code>.</p></li> </ul> <p>To enable high throughput for FIFO queues, do the following:</p> <ul>  <li>   <p>Set <code>DeduplicationScope</code> to <code>messageGroup</code>.</p></li>  <li>   <p>Set <code>FifoThroughputLimit</code> to <code>perMessageGroupId</code>.</p></li> </ul> <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p> <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
     /// - On success, responds with [`GetQueueAttributesOutput`](crate::operation::get_queue_attributes::GetQueueAttributesOutput) with field(s):
     ///   - [`attributes(Option<HashMap::<QueueAttributeName, String>>)`](crate::operation::get_queue_attributes::GetQueueAttributesOutput::attributes): <p>A map of attributes to their respective values.</p>
     /// - On failure, responds with [`SdkError<GetQueueAttributesError>`](crate::operation::get_queue_attributes::GetQueueAttributesError)
```

### `src/client/receive_message.rs`

```diff
--- reference/src/client/receive_message.rs
+++ generated/src/client/receive_message.rs
@@ -4,8 +4,8 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_url(impl Into<String>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue from which messages are received.</p> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`attribute_names(QueueAttributeName)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::attribute_names) / [`set_attribute_names(Option<Vec::<QueueAttributeName>>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_attribute_names):<br>required: **false**<br><important>  <p>This parameter has been discontinued but will be supported for backward compatibility. To provide attribute names, you are encouraged to use <code>MessageSystemAttributeNames</code>.</p> </important> <p>A list of attributes that need to be returned along with each message. These attributes include:</p> <ul>  <li>   <p><code>All</code> – Returns all values.</p></li>  <li>   <p><code>ApproximateFirstReceiveTimestamp</code> – Returns the time the message was first received from the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>ApproximateReceiveCount</code> – Returns the number of times a message has been received across all queues but not deleted.</p></li>  <li>   <p><code>AWSTraceHeader</code> – Returns the X-Ray trace header string.</p></li>  <li>   <p><code>SenderId</code></p>   <ul>    <li>     <p>For a user, returns the user ID, for example <code>ABCDEFGHI1JKLMNOPQ23R</code>.</p></li>    <li>     <p>For an IAM role, returns the IAM role ID, for example <code>ABCDE1F2GH3I4JK5LMNOP:i-a123b456</code>.</p></li>   </ul></li>  <li>   <p><code>SentTimestamp</code> – Returns the time the message was sent to the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li>  <li>   <p><code>MessageDeduplicationId</code> – Returns the value provided by the producer that calls the <code> <code>SendMessage</code> </code> action.</p></li>  <li>   <p><code>MessageGroupId</code> – Returns the value provided by the producer that calls the <code> <code>SendMessage</code> </code> action.</p></li>  <li>   <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li> </ul><br>
-    ///   - [`message_system_attribute_names(MessageSystemAttributeName)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::message_system_attribute_names) / [`set_message_system_attribute_names(Option<Vec::<MessageSystemAttributeName>>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_message_system_attribute_names):<br>required: **false**<br><p>A list of attributes that need to be returned along with each message. These attributes include:</p> <ul>  <li>   <p><code>All</code> – Returns all values.</p></li>  <li>   <p><code>ApproximateFirstReceiveTimestamp</code> – Returns the time the message was first received from the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>ApproximateReceiveCount</code> – Returns the number of times a message has been received across all queues but not deleted.</p></li>  <li>   <p><code>AWSTraceHeader</code> – Returns the X-Ray trace header string.</p></li>  <li>   <p><code>SenderId</code></p>   <ul>    <li>     <p>For a user, returns the user ID, for example <code>ABCDEFGHI1JKLMNOPQ23R</code>.</p></li>    <li>     <p>For an IAM role, returns the IAM role ID, for example <code>ABCDE1F2GH3I4JK5LMNOP:i-a123b456</code>.</p></li>   </ul></li>  <li>   <p><code>SentTimestamp</code> – Returns the time the message was sent to the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li>  <li>   <p><code>MessageDeduplicationId</code> – Returns the value provided by the producer that calls the <code> <code>SendMessage</code> </code> action.</p></li>  <li>   <p><code>MessageGroupId</code> – Returns the value provided by the producer that calls the <code> <code>SendMessage</code> </code> action.</p></li>  <li>   <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li> </ul><br>
+    ///   - [`attribute_names(QueueAttributeName)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::attribute_names) / [`set_attribute_names(Option<Vec::<QueueAttributeName>>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_attribute_names):<br>required: **false**<br><important>  <p>This parameter has been discontinued but will be supported for backward compatibility. To provide attribute names, you are encouraged to use <code>MessageSystemAttributeNames</code>.</p> </important> <p>A list of attributes that need to be returned along with each message. These attributes include:</p> <ul>  <li>   <p><code>All</code> – Returns all values.</p></li>  <li>   <p><code>ApproximateFirstReceiveTimestamp</code> – Returns the time the message was first received from the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>ApproximateReceiveCount</code> – Returns the number of times a message has been received across all queues but not deleted.</p></li>  <li>   <p><code>AWSTraceHeader</code> – Returns the X-Ray trace header string.</p></li>  <li>   <p><code>SenderId</code></p>   <ul>    <li>     <p>For a user, returns the user ID, for example <code>ABCDEFGHI1JKLMNOPQ23R</code>.</p></li>    <li>     <p>For an IAM role, returns the IAM role ID, for example <code>ABCDE1F2GH3I4JK5LMNOP:i-a123b456</code>.</p></li>   </ul></li>  <li>   <p><code>SentTimestamp</code> – Returns the time the message was sent to the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li>  <li>   <p><code>MessageDeduplicationId</code> – Returns the value provided by the producer that calls the <code> <a>SendMessage</a> </code> action.</p></li>  <li>   <p><code>MessageGroupId</code> – Returns the value provided by the producer that calls the <code> <a>SendMessage</a> </code> action.</p></li>  <li>   <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li> </ul><br>
+    ///   - [`message_system_attribute_names(MessageSystemAttributeName)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::message_system_attribute_names) / [`set_message_system_attribute_names(Option<Vec::<MessageSystemAttributeName>>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_message_system_attribute_names):<br>required: **false**<br><p>A list of attributes that need to be returned along with each message. These attributes include:</p> <ul>  <li>   <p><code>All</code> – Returns all values.</p></li>  <li>   <p><code>ApproximateFirstReceiveTimestamp</code> – Returns the time the message was first received from the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>ApproximateReceiveCount</code> – Returns the number of times a message has been received across all queues but not deleted.</p></li>  <li>   <p><code>AWSTraceHeader</code> – Returns the X-Ray trace header string.</p></li>  <li>   <p><code>SenderId</code></p>   <ul>    <li>     <p>For a user, returns the user ID, for example <code>ABCDEFGHI1JKLMNOPQ23R</code>.</p></li>    <li>     <p>For an IAM role, returns the IAM role ID, for example <code>ABCDE1F2GH3I4JK5LMNOP:i-a123b456</code>.</p></li>   </ul></li>  <li>   <p><code>SentTimestamp</code> – Returns the time the message was sent to the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li>  <li>   <p><code>MessageDeduplicationId</code> – Returns the value provided by the producer that calls the <code> <a>SendMessage</a> </code> action.</p></li>  <li>   <p><code>MessageGroupId</code> – Returns the value provided by the producer that calls the <code> <a>SendMessage</a> </code> action.</p></li>  <li>   <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li> </ul><br>
     ///   - [`message_attribute_names(impl Into<String>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::message_attribute_names) / [`set_message_attribute_names(Option<Vec::<String>>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_message_attribute_names):<br>required: **false**<br><p>The name of the message attribute, where <i>N</i> is the index.</p> <ul>  <li>   <p>The name can contain alphanumeric characters and the underscore (<code>_</code>), hyphen (<code>-</code>), and period (<code>.</code>).</p></li>  <li>   <p>The name is case-sensitive and must be unique among all attribute names for the message.</p></li>  <li>   <p>The name must not start with AWS-reserved prefixes such as <code>AWS.</code> or <code>Amazon.</code> (or any casing variants).</p></li>  <li>   <p>The name must not start or end with a period (<code>.</code>), and it should not have periods in succession (<code>..</code>).</p></li>  <li>   <p>The name can be up to 256 characters long.</p></li> </ul> <p>When using <code>ReceiveMessage</code>, you can send a list of attribute names to receive, or you can return all of the attributes by specifying <code>All</code> or <code>.*</code> in your request. You can also use all message attributes starting with a prefix, for example <code>bar.*</code>.</p><br>
     ///   - [`max_number_of_messages(i32)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::max_number_of_messages) / [`set_max_number_of_messages(Option<i32>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_max_number_of_messages):<br>required: **false**<br><p>The maximum number of messages to return. Amazon SQS never returns more messages than this value (however, fewer messages might be returned). Valid values: 1 to 10. Default: 1.</p><br>
     ///   - [`visibility_timeout(i32)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::visibility_timeout) / [`set_visibility_timeout(Option<i32>)`](crate::operation::receive_message::builders::ReceiveMessageFluentBuilder::set_visibility_timeout):<br>required: **false**<br><p>The duration (in seconds) that the received messages are hidden from subsequent retrieve requests after being retrieved by a <code>ReceiveMessage</code> request. If not specified, the default visibility timeout for the queue is used, which is 30 seconds.</p> <p>Understanding <code>VisibilityTimeout</code>:</p> <ul>  <li>   <p>When a message is received from a queue, it becomes temporarily invisible to other consumers for the duration of the visibility timeout. This prevents multiple consumers from processing the same message simultaneously. If the message is not deleted or its visibility timeout is not extended before the timeout expires, it becomes visible again and can be retrieved by other consumers.</p></li>  <li>   <p>Setting an appropriate visibility timeout is crucial. If it's too short, the message might become visible again before processing is complete, leading to duplicate processing. If it's too long, it delays the reprocessing of messages if the initial processing fails.</p></li>  <li>   <p>You can adjust the visibility timeout using the <code>--visibility-timeout</code> parameter in the <code>receive-message</code> command to match the processing time required by your application.</p></li>  <li>   <p>A message that isn't deleted or a message whose visibility isn't extended before the visibility timeout expires counts as a failed receive. Depending on the configuration of the queue, the message might be sent to the dead-letter queue.</p></li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
```

### `src/client/remove_permission.rs`

```diff
--- reference/src/client/remove_permission.rs
+++ generated/src/client/remove_permission.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_url(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue from which permissions are removed.</p> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`label(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::label) / [`set_label(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_label):<br>required: **true**<br><p>The identification of the permission to remove. This is the label added using the <code> <code>AddPermission</code> </code> action.</p><br>
+    ///   - [`label(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::label) / [`set_label(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_label):<br>required: **true**<br><p>The identification of the permission to remove. This is the label added using the <code> <a>AddPermission</a> </code> action.</p><br>
     /// - On success, responds with [`RemovePermissionOutput`](crate::operation::remove_permission::RemovePermissionOutput)
     /// - On failure, responds with [`SdkError<RemovePermissionError>`](crate::operation::remove_permission::RemovePermissionError)
     pub fn remove_permission(&self) -> crate::operation::remove_permission::builders::RemovePermissionFluentBuilder {
```

### `src/client/send_message.rs`

```diff
--- reference/src/client/send_message.rs
+++ generated/src/client/send_message.rs
@@ -8,7 +8,7 @@
     ///   - [`delay_seconds(i32)`](crate::operation::send_message::builders::SendMessageFluentBuilder::delay_seconds) / [`set_delay_seconds(Option<i32>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::set_delay_seconds):<br>required: **false**<br><p>The length of time, in seconds, for which to delay a specific message. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don't specify a value, the default value for the queue applies.</p><note>  <p>When you set <code>FifoQueue</code>, you can't set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p> </note><br>
     ///   - [`message_attributes(impl Into<String>, MessageAttributeValue)`](crate::operation::send_message::builders::SendMessageFluentBuilder::message_attributes) / [`set_message_attributes(Option<HashMap::<String, MessageAttributeValue>>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::set_message_attributes):<br>required: **false**<br><p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
     ///   - [`message_system_attributes(MessageSystemAttributeNameForSends, MessageSystemAttributeValue)`](crate::operation::send_message::builders::SendMessageFluentBuilder::message_system_attributes) / [`set_message_system_attributes(Option<HashMap::<MessageSystemAttributeNameForSends, MessageSystemAttributeValue>>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::set_message_system_attributes):<br>required: **false**<br><p>The message system attribute to send. Each message system attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>.</p><important>  <ul>   <li>    <p>Currently, the only supported message system attribute is <code>AWSTraceHeader</code>. Its type must be <code>String</code> and its value must be a correctly formatted X-Ray trace header string.</p></li>   <li>    <p>The size of a message system attribute doesn't count towards the total size of a message.</p></li>  </ul> </important><br>
-    ///   - [`message_deduplication_id(impl Into<String>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::message_deduplication_id) / [`set_message_deduplication_id(Option<String>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::set_message_deduplication_id):<br>required: **false**<br><p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p> <ul>  <li>   <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p>   <ul>    <li>     <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>    <li>     <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>    <li>     <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>    <li>     <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>   </ul></li>  <li>   <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>  <li>   <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li> </ul><note>  <p>The <code>MessageDeduplicationId</code> is available to the consumer of the message (this can be useful for troubleshooting delivery issues).</p>  <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p>  <p>Amazon SQS continues to keep track of the message deduplication ID even after the message is received and deleted.</p> </note> <p>The maximum length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~</code>).</p> <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html">Using the MessageDeduplicationId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
+    ///   - [`message_deduplication_id(impl Into<String>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::message_deduplication_id) / [`set_message_deduplication_id(Option<String>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::set_message_deduplication_id):<br>required: **false**<br><p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p> <ul>  <li>   <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p>   <ul>    <li>     <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>    <li>     <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>    <li>     <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>    <li>     <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>   </ul></li>  <li>   <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>  <li>   <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li> </ul> <note>  <p>The <code>MessageDeduplicationId</code> is available to the consumer of the message (this can be useful for troubleshooting delivery issues).</p>  <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p>  <p>Amazon SQS continues to keep track of the message deduplication ID even after the message is received and deleted.</p> </note> <p>The maximum length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~</code>).</p> <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html">Using the MessageDeduplicationId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
     ///   - [`message_group_id(impl Into<String>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::message_group_id) / [`set_message_group_id(Option<String>)`](crate::operation::send_message::builders::SendMessageFluentBuilder::set_message_group_id):<br>required: **false**<br><p><code>MessageGroupId</code> is an attribute used in Amazon SQS FIFO (First-In-First-Out) and standard queues. In FIFO queues, <code>MessageGroupId</code> organizes messages into distinct groups. Messages within the same message group are always processed one at a time, in strict order, ensuring that no two messages from the same group are processed simultaneously. In standard queues, using <code>MessageGroupId</code> enables fair queues. It is used to identify the tenant a message belongs to, helping maintain consistent message dwell time across all tenants during noisy neighbor events. Unlike FIFO queues, messages with the same <code>MessageGroupId</code> can be processed in parallel, maintaining the high throughput of standard queues.</p> <ul>  <li>   <p><b>FIFO queues:</b> <code>MessageGroupId</code> acts as the tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple consumers can process the queue, but the session data of each user is processed in a FIFO fashion.</p>   <p>If you do not provide a <code>MessageGroupId</code> when sending a message to a FIFO queue, the action fails.</p>   <p><code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent.</p></li>  <li>   <p><b>Standard queues:</b>Use <code>MessageGroupId</code> in standard queues to enable fair queues. The <code>MessageGroupId</code> identifies the tenant a message belongs to. A tenant can be any entity that shares a queue with others, such as your customer, a client application, or a request type. When one tenant sends a disproportionately large volume of messages or has messages that require longer processing time, fair queues ensure other tenants' messages maintain low dwell time. This preserves quality of service for all tenants while maintaining the scalability and throughput of standard queues. We recommend that you include a <code>MessageGroupId</code> in all messages when using fair queues.</p></li> </ul> <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values: alphanumeric characters and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p> <p>For best practices of using <code>MessageGroupId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagegroupid-property.html">Using the MessageGroupId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
     /// - On success, responds with [`SendMessageOutput`](crate::operation::send_message::SendMessageOutput) with field(s):
     ///   - [`md5_of_message_body(Option<String>)`](crate::operation::send_message::SendMessageOutput::md5_of_message_body): <p>An MD5 digest of the non-URL-encoded message body string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
```

### `src/client/send_message_batch.rs`

```diff
--- reference/src/client/send_message_batch.rs
+++ generated/src/client/send_message_batch.rs
@@ -4,10 +4,10 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_url(impl Into<String>)`](crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue to which batched messages are sent.</p> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`entries(SendMessageBatchRequestEntry)`](crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::entries) / [`set_entries(Option<Vec::<SendMessageBatchRequestEntry>>)`](crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::set_entries):<br>required: **true**<br><p>A list of <code> <code>SendMessageBatchRequestEntry</code> </code> items.</p><br>
+    ///   - [`entries(SendMessageBatchRequestEntry)`](crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::entries) / [`set_entries(Option<Vec::<SendMessageBatchRequestEntry>>)`](crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::set_entries):<br>required: **true**<br><p>A list of <code> <a>SendMessageBatchRequestEntry</a> </code> items.</p><br>
     /// - On success, responds with [`SendMessageBatchOutput`](crate::operation::send_message_batch::SendMessageBatchOutput) with field(s):
-    ///   - [`successful(Vec::<SendMessageBatchResultEntry>)`](crate::operation::send_message_batch::SendMessageBatchOutput::successful): <p>A list of <code> <code>SendMessageBatchResultEntry</code> </code> items.</p>
-    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::send_message_batch::SendMessageBatchOutput::failed): <p>A list of <code> <code>BatchResultErrorEntry</code> </code> items with error details about each message that can't be enqueued.</p>
+    ///   - [`successful(Vec::<SendMessageBatchResultEntry>)`](crate::operation::send_message_batch::SendMessageBatchOutput::successful): <p>A list of <code> <a>SendMessageBatchResultEntry</a> </code> items.</p>
+    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::send_message_batch::SendMessageBatchOutput::failed): <p>A list of <code> <a>BatchResultErrorEntry</a> </code> items with error details about each message that can't be enqueued.</p>
     /// - On failure, responds with [`SdkError<SendMessageBatchError>`](crate::operation::send_message_batch::SendMessageBatchError)
     pub fn send_message_batch(&self) -> crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder {
         crate::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::new(self.handle.clone())
```

### `src/client/set_queue_attributes.rs`

```diff
--- reference/src/client/set_queue_attributes.rs
+++ generated/src/client/set_queue_attributes.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`queue_url(impl Into<String>)`](crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue whose attributes are set.</p> <p>Queue URLs and names are case-sensitive.</p><br>
-    ///   - [`attributes(QueueAttributeName, impl Into<String>)`](crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<QueueAttributeName, String>>)`](crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder::set_attributes):<br>required: **true**<br><p>A map of attributes to set.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetQueueAttributes</code> action uses:</p> <ul>  <li>   <p><code>DelaySeconds</code> – The length of time, in seconds, for which the delivery of all messages in the queue is delayed. Valid values: An integer from 0 to 900 (15 minutes). Default: 0.</p></li>  <li>   <p><code>MaximumMessageSize</code> – The limit of how many bytes a message can contain before Amazon SQS rejects it. Valid values: An integer from 1,024 bytes (1 KiB) up to 1,048,576 bytes (1 MiB). Default: 1,048,576 bytes (1 MiB).</p></li>  <li>   <p><code>MessageRetentionPeriod</code> – The length of time, in seconds, for which Amazon SQS retains a message. Valid values: An integer representing seconds, from 60 (1 minute) to 1,209,600 (14 days). Default: 345,600 (4 days). When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes and will impact existing messages in the queue potentially causing them to be expired and deleted if the <code>MessageRetentionPeriod</code> is reduced below the age of existing messages.</p></li>  <li>   <p><code>Policy</code> – The queue's policy. A valid Amazon Web Services policy. For more information about policy structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html">Overview of Amazon Web Services IAM Policies</a> in the <i>Identity and Access Management User Guide</i>.</p></li>  <li>   <p><code>ReceiveMessageWaitTimeSeconds</code> – The length of time, in seconds, for which a <code> <code>ReceiveMessage</code> </code> action waits for a message to arrive. Valid values: An integer from 0 to 20 (seconds). Default: 0.</p></li>  <li>   <p><code>VisibilityTimeout</code> – The visibility timeout for the queue, in seconds. Valid values: An integer from 0 to 43,200 (12 hours). Default: 30. For more information about the visibility timeout, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues:</a></p> <ul>  <li>   <p><code>RedrivePolicy</code> – The string that includes the parameters for the dead-letter queue functionality of the source queue as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>deadLetterTargetArn</code> – The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p></li>    <li>     <p><code>maxReceiveCount</code> – The number of times a message is delivered to the source queue before being moved to the dead-letter queue. Default: 10. When the <code>ReceiveCount</code> for a message exceeds the <code>maxReceiveCount</code> for a queue, Amazon SQS moves the message to the dead-letter-queue.</p></li>   </ul></li>  <li>   <p><code>RedriveAllowPolicy</code> – The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>redrivePermission</code> – The permission type that defines which source queues can specify the current queue as the dead-letter queue. Valid values are:</p>     <ul>      <li>       <p><code>allowAll</code> – (Default) Any source queues in this Amazon Web Services account in the same Region can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>denyAll</code> – No source queues can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>byQueue</code> – Only queues specified by the <code>sourceQueueArns</code> parameter can specify this queue as the dead-letter queue.</p></li>     </ul></li>    <li>     <p><code>sourceQueueArns</code> – The Amazon Resource Names (ARN)s of the source queues that can specify this queue as the dead-letter queue and redrive messages. You can specify this parameter only when the <code>redrivePermission</code> parameter is set to <code>byQueue</code>. You can specify up to 10 source queue ARNs. To allow more than 10 source queues to specify dead-letter queues, set the <code>redrivePermission</code> parameter to <code>allowAll</code>.</p></li>   </ul></li> </ul><note>  <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. While the alias of the AWS-managed CMK for Amazon SQS is always <code>alias/aws/sqs</code>, the alias of a custom CMK can, for example, be <code>alias/<i>MyAlias</i> </code>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>  <li>   <p><code>KmsDataKeyReusePeriodSeconds</code> – The length of time, in seconds, for which Amazon SQS can reuse a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data key</a> to encrypt or decrypt messages before calling KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). Default: 300 (5 minutes). A shorter time period provides better security but results in more calls to KMS which might incur charges after Free Tier. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>.</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html">Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>. Note the following:</p>   <ul>    <li>     <p>Every message must have a unique <code>MessageDeduplicationId</code>.</p>     <ul>      <li>       <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>      <li>       <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>      <li>       <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>      <li>       <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>     </ul></li>    <li>     <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>    <li>     <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>   </ul></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/high-throughput-fifo.html">high throughput for FIFO queues</a>:</p> <ul>  <li>   <p><code>DeduplicationScope</code> – Specifies whether message deduplication occurs at the message group or queue level. Valid values are <code>messageGroup</code> and <code>queue</code>.</p></li>  <li>   <p><code>FifoThroughputLimit</code> – Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are <code>perQueue</code> and <code>perMessageGroupId</code>. The <code>perMessageGroupId</code> value is allowed only when the value for <code>DeduplicationScope</code> is <code>messageGroup</code>.</p></li> </ul> <p>To enable high throughput for FIFO queues, do the following:</p> <ul>  <li>   <p>Set <code>DeduplicationScope</code> to <code>messageGroup</code>.</p></li>  <li>   <p>Set <code>FifoThroughputLimit</code> to <code>perMessageGroupId</code>.</p></li> </ul> <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p> <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
+    ///   - [`attributes(QueueAttributeName, impl Into<String>)`](crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<QueueAttributeName, String>>)`](crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder::set_attributes):<br>required: **true**<br><p>A map of attributes to set.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetQueueAttributes</code> action uses:</p> <ul>  <li>   <p><code>DelaySeconds</code> – The length of time, in seconds, for which the delivery of all messages in the queue is delayed. Valid values: An integer from 0 to 900 (15 minutes). Default: 0.</p></li>  <li>   <p><code>MaximumMessageSize</code> – The limit of how many bytes a message can contain before Amazon SQS rejects it. Valid values: An integer from 1,024 bytes (1 KiB) up to 1,048,576 bytes (1 MiB). Default: 1,048,576 bytes (1 MiB).</p></li>  <li>   <p><code>MessageRetentionPeriod</code> – The length of time, in seconds, for which Amazon SQS retains a message. Valid values: An integer representing seconds, from 60 (1 minute) to 1,209,600 (14 days). Default: 345,600 (4 days). When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes and will impact existing messages in the queue potentially causing them to be expired and deleted if the <code>MessageRetentionPeriod</code> is reduced below the age of existing messages.</p></li>  <li>   <p><code>Policy</code> – The queue's policy. A valid Amazon Web Services policy. For more information about policy structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html">Overview of Amazon Web Services IAM Policies</a> in the <i>Identity and Access Management User Guide</i>.</p></li>  <li>   <p><code>ReceiveMessageWaitTimeSeconds</code> – The length of time, in seconds, for which a <code> <a>ReceiveMessage</a> </code> action waits for a message to arrive. Valid values: An integer from 0 to 20 (seconds). Default: 0.</p></li>  <li>   <p><code>VisibilityTimeout</code> – The visibility timeout for the queue, in seconds. Valid values: An integer from 0 to 43,200 (12 hours). Default: 30. For more information about the visibility timeout, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon SQS Developer Guide</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues:</a></p> <ul>  <li>   <p><code>RedrivePolicy</code> – The string that includes the parameters for the dead-letter queue functionality of the source queue as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>deadLetterTargetArn</code> – The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p></li>    <li>     <p><code>maxReceiveCount</code> – The number of times a message is delivered to the source queue before being moved to the dead-letter queue. Default: 10. When the <code>ReceiveCount</code> for a message exceeds the <code>maxReceiveCount</code> for a queue, Amazon SQS moves the message to the dead-letter-queue.</p></li>   </ul></li>  <li>   <p><code>RedriveAllowPolicy</code> – The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object. The parameters are as follows:</p>   <ul>    <li>     <p><code>redrivePermission</code> – The permission type that defines which source queues can specify the current queue as the dead-letter queue. Valid values are:</p>     <ul>      <li>       <p><code>allowAll</code> – (Default) Any source queues in this Amazon Web Services account in the same Region can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>denyAll</code> – No source queues can specify this queue as the dead-letter queue.</p></li>      <li>       <p><code>byQueue</code> – Only queues specified by the <code>sourceQueueArns</code> parameter can specify this queue as the dead-letter queue.</p></li>     </ul></li>    <li>     <p><code>sourceQueueArns</code> – The Amazon Resource Names (ARN)s of the source queues that can specify this queue as the dead-letter queue and redrive messages. You can specify this parameter only when the <code>redrivePermission</code> parameter is set to <code>byQueue</code>. You can specify up to 10 source queue ARNs. To allow more than 10 source queues to specify dead-letter queues, set the <code>redrivePermission</code> parameter to <code>allowAll</code>.</p></li>   </ul></li> </ul> <note>  <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. While the alias of the AWS-managed CMK for Amazon SQS is always <code>alias/aws/sqs</code>, the alias of a custom CMK can, for example, be <code>alias/<i>MyAlias</i> </code>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>  <li>   <p><code>KmsDataKeyReusePeriodSeconds</code> – The length of time, in seconds, for which Amazon SQS can reuse a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data key</a> to encrypt or decrypt messages before calling KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). Default: 300 (5 minutes). A shorter time period provides better security but results in more calls to KMS which might incur charges after Free Tier. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>.</p></li>  <li>   <p><code>SqsManagedSseEnabled</code> – Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (for example, <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html">SSE-KMS</a> or <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html">SSE-SQS</a>).</p></li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html">Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>. Note the following:</p>   <ul>    <li>     <p>Every message must have a unique <code>MessageDeduplicationId</code>.</p>     <ul>      <li>       <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>      <li>       <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>      <li>       <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>      <li>       <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>     </ul></li>    <li>     <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>    <li>     <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>   </ul></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/high-throughput-fifo.html">high throughput for FIFO queues</a>:</p> <ul>  <li>   <p><code>DeduplicationScope</code> – Specifies whether message deduplication occurs at the message group or queue level. Valid values are <code>messageGroup</code> and <code>queue</code>.</p></li>  <li>   <p><code>FifoThroughputLimit</code> – Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are <code>perQueue</code> and <code>perMessageGroupId</code>. The <code>perMessageGroupId</code> value is allowed only when the value for <code>DeduplicationScope</code> is <code>messageGroup</code>.</p></li> </ul> <p>To enable high throughput for FIFO queues, do the following:</p> <ul>  <li>   <p>Set <code>DeduplicationScope</code> to <code>messageGroup</code>.</p></li>  <li>   <p>Set <code>FifoThroughputLimit</code> to <code>perMessageGroupId</code>.</p></li> </ul> <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p> <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p><br>
     /// - On success, responds with [`SetQueueAttributesOutput`](crate::operation::set_queue_attributes::SetQueueAttributesOutput)
     /// - On failure, responds with [`SdkError<SetQueueAttributesError>`](crate::operation::set_queue_attributes::SetQueueAttributesError)
     pub fn set_queue_attributes(&self) -> crate::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder {
```

### `src/operation/add_permission.rs`

```diff
--- reference/src/operation/add_permission.rs
+++ generated/src/operation/add_permission.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AddPermission", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::add_permission::AddPermissionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::add_permission::AddPermissionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::add_permission::AddPermissionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,17 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.AddPermission",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_add_permission::ser_add_permission_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_add_permission_input::ser_add_permission_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -296,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -465,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::add_permission::AddPermissionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::add_permission::AddPermissionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/cancel_message_move_task.rs`

```diff
--- reference/src/operation/cancel_message_move_task.rs
+++ generated/src/operation/cancel_message_move_task.rs
@@ -107,9 +107,9 @@
             "SQS",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CancelMessageMoveTask")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CancelMessageMoveTaskTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CancelMessageMoveTaskEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::cancel_message_move_task::CancelMessageMoveTaskError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::cancel_message_move_task::CancelMessageMoveTaskError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::cancel_message_move_task::CancelMessageMoveTaskError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CancelMessageMoveTask")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    CancelMessageMoveTaskTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    CancelMessageMoveTaskEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::cancel_message_move_task::CancelMessageMoveTaskError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::cancel_message_move_task::CancelMessageMoveTaskError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::cancel_message_move_task::CancelMessageMoveTaskError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,18 +259,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.CancelMessageMoveTask",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_cancel_message_move_task::ser_cancel_message_move_task_input(&input)?,
+            crate::protocol_serde::shape_cancel_message_move_task_input::ser_cancel_message_move_task_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -296,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +457,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::cancel_message_move_task::CancelMessageMoveTaskError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::cancel_message_move_task::CancelMessageMoveTaskError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/change_message_visibility/builders.rs`

```diff
--- reference/src/operation/change_message_visibility/builders.rs
+++ generated/src/operation/change_message_visibility/builders.rs
@@ -35,7 +35,7 @@
 /// </ol>
 /// <p>A message is considered to be <i>stored</i> after it is sent to a queue by a producer, but not yet received from the queue by a consumer (that is, between states 1 and 2). There is no limit to the number of stored messages. A message is considered to be <i>in flight</i> after it is received from a queue by a consumer, but not yet deleted from the queue (that is, between states 2 and 3). There is a limit to the number of in flight messages.</p>
 /// <p>Limits that apply to in flight messages are unrelated to the <i>unlimited</i> number of stored messages.</p>
-/// <p>For most standard queues (depending on queue traffic and message backlog), there can be a maximum of approximately 120,000 in flight messages (received from a queue by a consumer, but not yet deleted from the queue). If you reach this limit, Amazon SQS returns the <code>OverLimit</code> error message. To avoid reaching the limit, you should delete messages from the queue after they're processed. You can also increase the number of queues you use to process your messages. To request a limit increase, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sqs">file a support request</a>.</p>
+/// <p>For most standard queues (depending on queue traffic and message backlog), there can be a maximum of approximately 120,000 in flight messages (received from a queue by a consumer, but not yet deleted from the queue). If you reach this limit, Amazon SQS returns the <code>OverLimit</code> error message. To avoid reaching the limit, you should delete messages from the queue after they're processed. You can also increase the number of queues you use to process your messages. To request a limit increase, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sqs">file a support request</a>.</p>
 /// <p>For FIFO queues, there can be a maximum of 120,000 in flight messages (received from a queue by a consumer, but not yet deleted from the queue). If you reach this limit, Amazon SQS returns no error messages.</p><important>
 /// <p>If you attempt to set the <code>VisibilityTimeout</code> to a value greater than the maximum time left, Amazon SQS returns an error. Amazon SQS doesn't automatically recalculate and increase the timeout to the maximum remaining time.</p>
 /// <p>Unlike with a queue, when you change the visibility timeout for a specific message the timeout value is applied immediately but isn't saved in memory for that message. If you don't delete a message after it is received, the visibility timeout for the message reverts to the original timeout value (not to the value you set using the <code>ChangeMessageVisibility</code> action) the next time the message is received.</p>
```

### `src/operation/change_message_visibility.rs`

```diff
--- reference/src/operation/change_message_visibility.rs
+++ generated/src/operation/change_message_visibility.rs
@@ -107,9 +107,9 @@
             "SQS",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::change_message_visibility::ChangeMessageVisibilityError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::change_message_visibility::ChangeMessageVisibilityError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::change_message_visibility::ChangeMessageVisibilityError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,18 +263,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ChangeMessageVisibility",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_change_message_visibility::ser_change_message_visibility_input(&input)?,
+            crate::protocol_serde::shape_change_message_visibility_input::ser_change_message_visibility_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -301,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -480,6 +481,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::change_message_visibility::ChangeMessageVisibilityError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::change_message_visibility::ChangeMessageVisibilityError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/change_message_visibility_batch.rs`

```diff
--- reference/src/operation/change_message_visibility_batch.rs
+++ generated/src/operation/change_message_visibility_batch.rs
@@ -107,9 +107,9 @@
             "SQS",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,18 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ChangeMessageVisibilityBatch",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_change_message_visibility_batch::ser_change_message_visibility_batch_input(&input)?,
+            crate::protocol_serde::shape_change_message_visibility_batch_input::ser_change_message_visibility_batch_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -296,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -495,6 +496,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_queue.rs`

```diff
--- reference/src/operation/create_queue.rs
+++ generated/src/operation/create_queue.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateQueue", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_queue::CreateQueueError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_queue::CreateQueueError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_queue::CreateQueueError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,17 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.CreateQueue",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_queue::ser_create_queue_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_queue_input::ser_create_queue_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -291,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -480,6 +479,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_queue::CreateQueueError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_queue::CreateQueueError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_message.rs`

```diff
--- reference/src/operation/delete_message.rs
+++ generated/src/operation/delete_message.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteMessage", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_message::DeleteMessageError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_message::DeleteMessageError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_message::DeleteMessageError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,17 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.DeleteMessage",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_message::ser_delete_message_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_message_input::ser_delete_message_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -296,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -319,7 +318,6 @@
     /// <p>The specified ID is invalid.</p>
     InvalidAddress(crate::types::error::InvalidAddress),
     /// <p>The specified receipt handle isn't valid for the current version.</p>
-    #[deprecated(note = "exception has been included in ReceiptHandleIsInvalid")]
     InvalidIdFormat(crate::types::error::InvalidIdFormat),
     /// <p>The request was not made over HTTPS or did not use SigV4 for signing.</p>
     InvalidSecurity(crate::types::error::InvalidSecurity),
@@ -476,6 +474,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_message::DeleteMessageError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_message::DeleteMessageError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_message_batch.rs`

```diff
--- reference/src/operation/delete_message_batch.rs
+++ generated/src/operation/delete_message_batch.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteMessageBatch", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -122,25 +122,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteMessageBatch")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteMessageBatchTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteMessageBatchEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::delete_message_batch::DeleteMessageBatchError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::delete_message_batch::DeleteMessageBatchError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_message_batch::DeleteMessageBatchError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteMessageBatch")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteMessageBatchTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteMessageBatchEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::delete_message_batch::DeleteMessageBatchError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::delete_message_batch::DeleteMessageBatchError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_message_batch::DeleteMessageBatchError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,18 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.DeleteMessageBatch",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_message_batch::ser_delete_message_batch_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_delete_message_batch_input::ser_delete_message_batch_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -491,6 +477,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_message_batch::DeleteMessageBatchError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_message_batch::DeleteMessageBatchError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_queue.rs`

```diff
--- reference/src/operation/delete_queue.rs
+++ generated/src/operation/delete_queue.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteQueue", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_queue::DeleteQueueError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_queue::DeleteQueueError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_queue::DeleteQueueError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,17 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.DeleteQueue",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_queue::ser_delete_queue_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_queue_input::ser_delete_queue_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -291,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +449,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_queue::DeleteQueueError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_queue::DeleteQueueError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_queue_attributes.rs`

```diff
--- reference/src/operation/get_queue_attributes.rs
+++ generated/src/operation/get_queue_attributes.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetQueueAttributes", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -122,25 +122,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetQueueAttributes")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetQueueAttributesTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetQueueAttributesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_queue_attributes::GetQueueAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_queue_attributes::GetQueueAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_queue_attributes::GetQueueAttributesError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetQueueAttributes")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetQueueAttributesTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetQueueAttributesEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_queue_attributes::GetQueueAttributesError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_queue_attributes::GetQueueAttributesError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_queue_attributes::GetQueueAttributesError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,18 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.GetQueueAttributes",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_queue_attributes::ser_get_queue_attributes_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_get_queue_attributes_input::ser_get_queue_attributes_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -461,6 +447,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_queue_attributes::GetQueueAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_queue_attributes::GetQueueAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_queue_url.rs`

```diff
--- reference/src/operation/get_queue_url.rs
+++ generated/src/operation/get_queue_url.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetQueueUrl", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_queue_url::GetQueueUrlError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_queue_url::GetQueueUrlError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_queue_url::GetQueueUrlError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,17 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.GetQueueUrl",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_queue_url::ser_get_queue_url_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_queue_url_input::ser_get_queue_url_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -296,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +454,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_queue_url::GetQueueUrlError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_queue_url::GetQueueUrlError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_dead_letter_source_queues.rs`

```diff
--- reference/src/operation/list_dead_letter_source_queues.rs
+++ generated/src/operation/list_dead_letter_source_queues.rs
@@ -107,9 +107,9 @@
             "SQS",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,18 +263,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ListDeadLetterSourceQueues",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_dead_letter_source_queues::ser_list_dead_letter_source_queues_input(&input)?,
+            crate::protocol_serde::shape_list_dead_letter_source_queues_input::ser_list_dead_letter_source_queues_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -301,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +461,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_message_move_tasks.rs`

```diff
--- reference/src/operation/list_message_move_tasks.rs
+++ generated/src/operation/list_message_move_tasks.rs
@@ -107,9 +107,9 @@
             "SQS",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListMessageMoveTasks")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListMessageMoveTasksTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListMessageMoveTasksEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_message_move_tasks::ListMessageMoveTasksError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_message_move_tasks::ListMessageMoveTasksError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_message_move_tasks::ListMessageMoveTasksError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListMessageMoveTasks")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListMessageMoveTasksTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListMessageMoveTasksEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::list_message_move_tasks::ListMessageMoveTasksError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::list_message_move_tasks::ListMessageMoveTasksError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_message_move_tasks::ListMessageMoveTasksError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,19 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ListMessageMoveTasks",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_message_move_tasks::ser_list_message_move_tasks_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_list_message_move_tasks_input::ser_list_message_move_tasks_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -296,8 +281,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +440,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_message_move_tasks::ListMessageMoveTasksError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_message_move_tasks::ListMessageMoveTasksError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_queue_tags.rs`

```diff
--- reference/src/operation/list_queue_tags.rs
+++ generated/src/operation/list_queue_tags.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListQueueTags", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_queue_tags::ListQueueTagsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_queue_tags::ListQueueTagsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_queue_tags::ListQueueTagsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,17 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ListQueueTags",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_queue_tags::ser_list_queue_tags_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_queue_tags_input::ser_list_queue_tags_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -291,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +449,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_queue_tags::ListQueueTagsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_queue_tags::ListQueueTagsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_queues.rs`

```diff
--- reference/src/operation/list_queues.rs
+++ generated/src/operation/list_queues.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListQueues", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_queues::ListQueuesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_queues::ListQueuesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_queues::ListQueuesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,17 +256,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ListQueues",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_queues::ser_list_queues_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_queues_input::ser_list_queues_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +293,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +442,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_queues::ListQueuesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_queues::ListQueuesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/purge_queue.rs`

```diff
--- reference/src/operation/purge_queue.rs
+++ generated/src/operation/purge_queue.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("PurgeQueue", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::purge_queue::PurgeQueueError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::purge_queue::PurgeQueueError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::purge_queue::PurgeQueueError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,17 +251,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.PurgeQueue",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_purge_queue::ser_purge_queue_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_purge_queue_input::ser_purge_queue_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -458,6 +457,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::purge_queue::PurgeQueueError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::purge_queue::PurgeQueueError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/receive_message/_receive_message_input.rs`

```diff
--- reference/src/operation/receive_message/_receive_message_input.rs
+++ generated/src/operation/receive_message/_receive_message_input.rs
@@ -167,9 +167,9 @@
     /// <li>
     /// <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li>
     /// </ul>
+    #[deprecated(note = "AttributeNames has been replaced by MessageSystemAttributeNames")]
     ///
     /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.attribute_names.is_none()`.
-    #[deprecated(note = "AttributeNames has been replaced by MessageSystemAttributeNames")]
     pub fn attribute_names(&self) -> &[crate::types::QueueAttributeName] {
         self.attribute_names.as_deref().unwrap_or_default()
     }
```

### `src/operation/receive_message.rs`

```diff
--- reference/src/operation/receive_message.rs
+++ generated/src/operation/receive_message.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ReceiveMessage", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -130,9 +130,6 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                crate::long_polling::LongPollingInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ReceiveMessageEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
@@ -141,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::receive_message::ReceiveMessageError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::receive_message::ReceiveMessageError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::receive_message::ReceiveMessageError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,17 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.ReceiveMessage",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_receive_message::ser_receive_message_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_receive_message_input::ser_receive_message_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -544,6 +540,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::receive_message::ReceiveMessageError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::receive_message::ReceiveMessageError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/remove_permission.rs`

```diff
--- reference/src/operation/remove_permission.rs
+++ generated/src/operation/remove_permission.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("RemovePermission", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::remove_permission::RemovePermissionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::remove_permission::RemovePermissionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::remove_permission::RemovePermissionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,17 +258,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.RemovePermission",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_remove_permission::ser_remove_permission_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_remove_permission_input::ser_remove_permission_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -296,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::remove_permission::RemovePermissionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::remove_permission::RemovePermissionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/send_message.rs`

```diff
--- reference/src/operation/send_message.rs
+++ generated/src/operation/send_message.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("SendMessage", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::send_message::SendMessageError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::send_message::SendMessageError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::send_message::SendMessageError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -262,17 +268,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.SendMessage",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_send_message::ser_send_message_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_send_message_input::ser_send_message_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -306,8 +305,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -551,6 +550,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::send_message::SendMessageError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::send_message::SendMessageError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/send_message_batch.rs`

```diff
--- reference/src/operation/send_message_batch.rs
+++ generated/src/operation/send_message_batch.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("SendMessageBatch", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::send_message_batch::SendMessageBatchError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::send_message_batch::SendMessageBatchError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::send_message_batch::SendMessageBatchError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,17 +253,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.SendMessageBatch",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_send_message_batch::ser_send_message_batch_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_send_message_batch_input::ser_send_message_batch_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -291,8 +292,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -576,6 +577,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::send_message_batch::SendMessageBatchError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::send_message_batch::SendMessageBatchError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/set_queue_attributes.rs`

```diff
--- reference/src/operation/set_queue_attributes.rs
+++ generated/src/operation/set_queue_attributes.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("SetQueueAttributes", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -122,25 +122,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetQueueAttributes")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetQueueAttributesTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetQueueAttributesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::set_queue_attributes::SetQueueAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::set_queue_attributes::SetQueueAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::set_queue_attributes::SetQueueAttributesError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetQueueAttributes")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(SetQueueAttributesTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(SetQueueAttributesEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::set_queue_attributes::SetQueueAttributesError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::set_queue_attributes::SetQueueAttributesError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::set_queue_attributes::SetQueueAttributesError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,18 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.SetQueueAttributes",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_set_queue_attributes::ser_set_queue_attributes_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_set_queue_attributes_input::ser_set_queue_attributes_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -481,6 +467,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::set_queue_attributes::SetQueueAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::set_queue_attributes::SetQueueAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/start_message_move_task.rs`

```diff
--- reference/src/operation/start_message_move_task.rs
+++ generated/src/operation/start_message_move_task.rs
@@ -107,9 +107,9 @@
             "SQS",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StartMessageMoveTask")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                StartMessageMoveTaskTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                StartMessageMoveTaskEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::start_message_move_task::StartMessageMoveTaskError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::start_message_move_task::StartMessageMoveTaskError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::start_message_move_task::StartMessageMoveTaskError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StartMessageMoveTask")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(StartMessageMoveTaskTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(StartMessageMoveTaskEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::start_message_move_task::StartMessageMoveTaskError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::start_message_move_task::StartMessageMoveTaskError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::start_message_move_task::StartMessageMoveTaskError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,19 +247,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.StartMessageMoveTask",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_start_message_move_task::ser_start_message_move_task_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_start_message_move_task_input::ser_start_message_move_task_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -301,8 +286,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +445,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::start_message_move_task::StartMessageMoveTaskError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::start_message_move_task::StartMessageMoveTaskError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/tag_queue.rs`

```diff
--- reference/src/operation/tag_queue.rs
+++ generated/src/operation/tag_queue.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("TagQueue", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -134,9 +134,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::tag_queue::TagQueueError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::tag_queue::TagQueueError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::tag_queue::TagQueueError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -241,17 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.TagQueue",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_queue::ser_tag_queue_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_queue_input::ser_tag_queue_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -285,8 +284,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -444,6 +443,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::tag_queue::TagQueueError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::tag_queue::TagQueueError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/untag_queue.rs`

```diff
--- reference/src/operation/untag_queue.rs
+++ generated/src/operation/untag_queue.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UntagQueue", "SQS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::untag_queue::UntagQueueError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::untag_queue::UntagQueueError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::untag_queue::UntagQueueError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,17 +251,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "AmazonSQS.UntagQueue",
-            );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_queue::ser_untag_queue_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_queue_input::ser_untag_queue_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -448,6 +447,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::untag_queue::UntagQueueError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::untag_queue::UntagQueueError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### Missing reference files

- `Cargo.toml`
- `LICENSE`
- `README.md`
- `src/aws_query_compatible_errors.rs`
- `src/config/endpoint.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/diagnostic.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/partition.rs`
- `src/endpoint_lib.rs`
- `src/json_errors.rs`
- `src/long_polling.rs`
- `src/protocol_serde/shape_add_permission.rs`
- `src/protocol_serde/shape_add_permission_input.rs`
- `src/protocol_serde/shape_batch_entry_ids_not_distinct.rs`
- `src/protocol_serde/shape_batch_request_too_long.rs`
- `src/protocol_serde/shape_batch_result_error_entry.rs`
- `src/protocol_serde/shape_batch_result_error_entry_list.rs`
- `src/protocol_serde/shape_binary_list.rs`
- `src/protocol_serde/shape_cancel_message_move_task.rs`
- `src/protocol_serde/shape_cancel_message_move_task_input.rs`
- `src/protocol_serde/shape_change_message_visibility.rs`
- `src/protocol_serde/shape_change_message_visibility_batch.rs`
- `src/protocol_serde/shape_change_message_visibility_batch_input.rs`
- `src/protocol_serde/shape_change_message_visibility_batch_request_entry.rs`
- `src/protocol_serde/shape_change_message_visibility_batch_result_entry.rs`
- `src/protocol_serde/shape_change_message_visibility_batch_result_entry_list.rs`
- `src/protocol_serde/shape_change_message_visibility_input.rs`
- `src/protocol_serde/shape_create_queue.rs`
- `src/protocol_serde/shape_create_queue_input.rs`
- `src/protocol_serde/shape_delete_message.rs`
- `src/protocol_serde/shape_delete_message_batch.rs`
- `src/protocol_serde/shape_delete_message_batch_input.rs`
- `src/protocol_serde/shape_delete_message_batch_request_entry.rs`
- `src/protocol_serde/shape_delete_message_batch_result_entry.rs`
- `src/protocol_serde/shape_delete_message_batch_result_entry_list.rs`
- `src/protocol_serde/shape_delete_message_input.rs`
- `src/protocol_serde/shape_delete_queue.rs`
- `src/protocol_serde/shape_delete_queue_input.rs`
- `src/protocol_serde/shape_empty_batch_request.rs`
- `src/protocol_serde/shape_get_queue_attributes.rs`
- `src/protocol_serde/shape_get_queue_attributes_input.rs`
- `src/protocol_serde/shape_get_queue_url.rs`
- `src/protocol_serde/shape_get_queue_url_input.rs`
- `src/protocol_serde/shape_invalid_address.rs`
- `src/protocol_serde/shape_invalid_attribute_name.rs`
- `src/protocol_serde/shape_invalid_attribute_value.rs`
- `src/protocol_serde/shape_invalid_batch_entry_id.rs`
- `src/protocol_serde/shape_invalid_id_format.rs`
- `src/protocol_serde/shape_invalid_message_contents.rs`
- `src/protocol_serde/shape_invalid_security.rs`
- `src/protocol_serde/shape_kms_access_denied.rs`
- `src/protocol_serde/shape_kms_disabled.rs`
- `src/protocol_serde/shape_kms_invalid_key_usage.rs`
- `src/protocol_serde/shape_kms_invalid_state.rs`
- `src/protocol_serde/shape_kms_not_found.rs`
- `src/protocol_serde/shape_kms_opt_in_required.rs`
- `src/protocol_serde/shape_kms_throttled.rs`
- `src/protocol_serde/shape_list_dead_letter_source_queues.rs`
- `src/protocol_serde/shape_list_dead_letter_source_queues_input.rs`
- `src/protocol_serde/shape_list_message_move_tasks.rs`
- `src/protocol_serde/shape_list_message_move_tasks_input.rs`
- `src/protocol_serde/shape_list_message_move_tasks_result_entry.rs`
- `src/protocol_serde/shape_list_message_move_tasks_result_entry_list.rs`
- `src/protocol_serde/shape_list_queue_tags.rs`
- `src/protocol_serde/shape_list_queue_tags_input.rs`
- `src/protocol_serde/shape_list_queues.rs`
- `src/protocol_serde/shape_list_queues_input.rs`
- `src/protocol_serde/shape_message.rs`
- `src/protocol_serde/shape_message_attribute_value.rs`
- `src/protocol_serde/shape_message_body_attribute_map.rs`
- `src/protocol_serde/shape_message_list.rs`
- `src/protocol_serde/shape_message_not_inflight.rs`
- `src/protocol_serde/shape_message_system_attribute_map.rs`
- `src/protocol_serde/shape_message_system_attribute_value.rs`
- `src/protocol_serde/shape_over_limit.rs`
- `src/protocol_serde/shape_purge_queue.rs`
- `src/protocol_serde/shape_purge_queue_in_progress.rs`
- `src/protocol_serde/shape_purge_queue_input.rs`
- `src/protocol_serde/shape_queue_attribute_map.rs`
- `src/protocol_serde/shape_queue_deleted_recently.rs`
- `src/protocol_serde/shape_queue_does_not_exist.rs`
- `src/protocol_serde/shape_queue_name_exists.rs`
- `src/protocol_serde/shape_queue_url_list.rs`
- `src/protocol_serde/shape_receipt_handle_is_invalid.rs`
- `src/protocol_serde/shape_receive_message.rs`
- `src/protocol_serde/shape_receive_message_input.rs`
- `src/protocol_serde/shape_remove_permission.rs`
- `src/protocol_serde/shape_remove_permission_input.rs`
- `src/protocol_serde/shape_request_throttled.rs`
- `src/protocol_serde/shape_resource_not_found_exception.rs`
- `src/protocol_serde/shape_send_message.rs`
- `src/protocol_serde/shape_send_message_batch.rs`
- `src/protocol_serde/shape_send_message_batch_input.rs`
- `src/protocol_serde/shape_send_message_batch_request_entry.rs`
- `src/protocol_serde/shape_send_message_batch_result_entry.rs`
- `src/protocol_serde/shape_send_message_batch_result_entry_list.rs`
- `src/protocol_serde/shape_send_message_input.rs`
- `src/protocol_serde/shape_set_queue_attributes.rs`
- `src/protocol_serde/shape_set_queue_attributes_input.rs`
- `src/protocol_serde/shape_start_message_move_task.rs`
- `src/protocol_serde/shape_start_message_move_task_input.rs`
- `src/protocol_serde/shape_string_list.rs`
- `src/protocol_serde/shape_tag_map.rs`
- `src/protocol_serde/shape_tag_queue.rs`
- `src/protocol_serde/shape_tag_queue_input.rs`
- `src/protocol_serde/shape_too_many_entries_in_batch_request.rs`
- `src/protocol_serde/shape_unsupported_operation.rs`
- `src/protocol_serde/shape_untag_queue.rs`
- `src/protocol_serde/shape_untag_queue_input.rs`
- `src/protocol_serde.rs`
- `src/serialization_settings.rs`
- `tests/endpoint_tests.rs`
- `tests/long-polling.rs`

### Rust token differences

- `src/client/change_message_visibility.rs`
- `src/client/change_message_visibility_batch.rs`
- `src/client/create_queue.rs`
- `src/client/delete_message_batch.rs`
- `src/client/get_queue_attributes.rs`
- `src/client/receive_message.rs`
- `src/client/remove_permission.rs`
- `src/client/send_message.rs`
- `src/client/send_message_batch.rs`
- `src/client/set_queue_attributes.rs`
- `src/operation/add_permission.rs`
- `src/operation/cancel_message_move_task.rs`
- `src/operation/change_message_visibility/builders.rs`
- `src/operation/change_message_visibility.rs`
- `src/operation/change_message_visibility_batch.rs`
- `src/operation/create_queue.rs`
- `src/operation/delete_message.rs`
- `src/operation/delete_message_batch.rs`
- `src/operation/delete_queue.rs`
- `src/operation/get_queue_attributes.rs`
- `src/operation/get_queue_url.rs`
- `src/operation/list_dead_letter_source_queues.rs`
- `src/operation/list_message_move_tasks.rs`
- `src/operation/list_queue_tags.rs`
- `src/operation/list_queues.rs`
- `src/operation/purge_queue.rs`
- `src/operation/receive_message/_receive_message_input.rs`
- `src/operation/receive_message.rs`
- `src/operation/remove_permission.rs`
- `src/operation/send_message.rs`
- `src/operation/send_message_batch.rs`
- `src/operation/set_queue_attributes.rs`
- `src/operation/start_message_move_task.rs`
- `src/operation/tag_queue.rs`
- `src/operation/untag_queue.rs`
