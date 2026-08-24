# AWS SDK Conformance Report: sqs

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sqs
**Progress:** `294/294` files compared · `187` matched · `105` mismatches · `2` missing · `0` extra · `63.61%` match (100.00% means fully matched)

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
     pub fn delete_message_batch(&self) -> super::super::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder {
         super::super::operation::delete_message_batch::builders::DeleteMessageBatchFluentBuilder::new(self.handle.clone())
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
     pub fn remove_permission(&self) -> super::super::operation::remove_permission::builders::RemovePermissionFluentBuilder {
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
     pub fn send_message_batch(&self) -> super::super::operation::send_message_batch::builders::SendMessageBatchFluentBuilder {
         super::super::operation::send_message_batch::builders::SendMessageBatchFluentBuilder::new(self.handle.clone())
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
     pub fn set_queue_attributes(&self) -> super::super::operation::set_queue_attributes::builders::SetQueueAttributesFluentBuilder {
```

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -29,7 +29,10 @@
 /// Endpoint resolver trait specific to Amazon Simple Queue Service
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -292,7 +295,10 @@
 }

 impl super::super::config::endpoint::ResolveEndpoint for DefaultResolver {
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         // Check single-entry cache (lock-free read via ArcSwap)
         let cached = self.endpoint_cache.load();
         if let Some((cached_params, cached_endpoint)) = cached.as_ref() {
```

### `src/operation/add_permission.rs`

```diff
--- reference/src/operation/add_permission.rs
+++ generated/src/operation/add_permission.rs
@@ -258,8 +258,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.AddPermission",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_add_permission::ser_add_permission_input(&input)?);
```

### `src/operation/cancel_message_move_task/_cancel_message_move_task_input.rs`

```diff
--- reference/src/operation/cancel_message_move_task/_cancel_message_move_task_input.rs
+++ generated/src/operation/cancel_message_move_task/_cancel_message_move_task_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`CancelMessageMoveTaskInput`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::cancel_message_move_task::CancelMessageMoveTaskInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::cancel_message_move_task::CancelMessageMoveTaskInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::cancel_message_move_task::CancelMessageMoveTaskInput {
             task_handle: self.task_handle,
         })
```

### `src/operation/cancel_message_move_task.rs`

```diff
--- reference/src/operation/cancel_message_move_task.rs
+++ generated/src/operation/cancel_message_move_task.rs
@@ -256,8 +256,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.CancelMessageMoveTask",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
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
@@ -261,8 +261,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ChangeMessageVisibility",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
```

### `src/operation/change_message_visibility_batch/_change_message_visibility_batch_input.rs`

```diff
--- reference/src/operation/change_message_visibility_batch/_change_message_visibility_batch_input.rs
+++ generated/src/operation/change_message_visibility_batch/_change_message_visibility_batch_input.rs
@@ -68,7 +68,10 @@
         self
     }
     /// <p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p>
-    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ChangeMessageVisibilityBatchRequestEntry>>) -> Self {
+    pub fn set_entries(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ChangeMessageVisibilityBatchRequestEntry>>,
+    ) -> Self {
         self.entries = input;
         self
     }
@@ -83,9 +86,11 @@
         super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchInput {
-            queue_url: self.queue_url,
-            entries: self.entries,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchInput {
+                queue_url: self.queue_url,
+                entries: self.entries,
+            },
+        )
     }
 }
```

### `src/operation/change_message_visibility_batch/_change_message_visibility_batch_output.rs`

```diff
--- reference/src/operation/change_message_visibility_batch/_change_message_visibility_batch_output.rs
+++ generated/src/operation/change_message_visibility_batch/_change_message_visibility_batch_output.rs
@@ -55,7 +55,10 @@
         self
     }
     /// <p>A list of <code> <code>ChangeMessageVisibilityBatchResultEntry</code> </code> items.</p>
-    pub fn set_successful(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ChangeMessageVisibilityBatchResultEntry>>) -> Self {
+    pub fn set_successful(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ChangeMessageVisibilityBatchResultEntry>>,
+    ) -> Self {
         self.successful = input;
         self
     }
@@ -102,20 +105,22 @@
         super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput {
-            successful: self.successful.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "successful",
-                    "successful was not specified but it is required when building ChangeMessageVisibilityBatchOutput",
-                )
-            })?,
-            failed: self.failed.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "failed",
-                    "failed was not specified but it is required when building ChangeMessageVisibilityBatchOutput",
-                )
-            })?,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput {
+                successful: self.successful.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "successful",
+                        "successful was not specified but it is required when building ChangeMessageVisibilityBatchOutput",
+                    )
+                })?,
+                failed: self.failed.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "failed",
+                        "failed was not specified but it is required when building ChangeMessageVisibilityBatchOutput",
+                    )
+                })?,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/change_message_visibility_batch/builders.rs`

```diff
--- reference/src/operation/change_message_visibility_batch/builders.rs
+++ generated/src/operation/change_message_visibility_batch/builders.rs
@@ -83,11 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatch::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatch::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatch::orchestrate(&runtime_plugins, input).await
     }

@@ -138,7 +139,10 @@
         self
     }
     /// <p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p>
-    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ChangeMessageVisibilityBatchRequestEntry>>) -> Self {
+    pub fn set_entries(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ChangeMessageVisibilityBatchRequestEntry>>,
+    ) -> Self {
         self.inner = self.inner.set_entries(input);
         self
     }
```

### `src/operation/change_message_visibility_batch.rs`

```diff
--- reference/src/operation/change_message_visibility_batch.rs
+++ generated/src/operation/change_message_visibility_batch.rs
@@ -209,7 +209,9 @@
         let parse_result = if !success && status != 200 || force_error {
             super::super::protocol_serde::shape_change_message_visibility_batch::de_change_message_visibility_batch_http_error(status, headers, body)
         } else {
-            super::super::protocol_serde::shape_change_message_visibility_batch::de_change_message_visibility_batch_http_response(status, headers, body)
+            super::super::protocol_serde::shape_change_message_visibility_batch::de_change_message_visibility_batch_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -256,8 +258,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ChangeMessageVisibilityBatch",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
```

### `src/operation/create_queue/_create_queue_input.rs`

```diff
--- reference/src/operation/create_queue/_create_queue_input.rs
+++ generated/src/operation/create_queue/_create_queue_input.rs
@@ -240,7 +240,9 @@
     /// </ul>
     /// <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p>
     /// <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         self.attributes.as_ref()
     }
     /// <p>Add cost allocation tags to the specified Amazon SQS queue. For an overview, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-tags.html">Tagging Your Amazon SQS Queues</a> in the <i>Amazon SQS Developer Guide</i>.</p>
@@ -626,7 +628,9 @@
     /// </ul>
     /// <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p>
     /// <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         &self.attributes
     }
     /// Adds a key-value pair to `tags`.
@@ -695,7 +699,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`CreateQueueInput`](crate::operation::create_queue::CreateQueueInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_queue::CreateQueueInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_queue::CreateQueueInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_queue::CreateQueueInput {
             queue_name: self.queue_name,
             attributes: self.attributes,
```

### `src/operation/create_queue/builders.rs`

```diff
--- reference/src/operation/create_queue/builders.rs
+++ generated/src/operation/create_queue/builders.rs
@@ -476,7 +476,9 @@
     /// </ul>
     /// <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p>
     /// <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         self.inner.get_attributes()
     }
     ///
```

### `src/operation/create_queue.rs`

```diff
--- reference/src/operation/create_queue.rs
+++ generated/src/operation/create_queue.rs
@@ -253,8 +253,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.CreateQueue",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_queue::ser_create_queue_input(&input)?);
```

### `src/operation/delete_message.rs`

```diff
--- reference/src/operation/delete_message.rs
+++ generated/src/operation/delete_message.rs
@@ -258,8 +258,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.DeleteMessage",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_message::ser_delete_message_input(&input)?);
@@ -319,7 +317,6 @@
     /// <p>The specified ID is invalid.</p>
     InvalidAddress(super::super::types::error::InvalidAddress),
     /// <p>The specified receipt handle isn't valid for the current version.</p>
-    #[deprecated(note = "exception has been included in ReceiptHandleIsInvalid")]
     InvalidIdFormat(super::super::types::error::InvalidIdFormat),
     /// <p>The request was not made over HTTPS or did not use SigV4 for signing.</p>
     InvalidSecurity(super::super::types::error::InvalidSecurity),
```

### `src/operation/delete_message_batch/_delete_message_batch_input.rs`

```diff
--- reference/src/operation/delete_message_batch/_delete_message_batch_input.rs
+++ generated/src/operation/delete_message_batch/_delete_message_batch_input.rs
@@ -79,8 +79,10 @@
     /// Consumes the builder and constructs a [`DeleteMessageBatchInput`](crate::operation::delete_message_batch::DeleteMessageBatchInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_message_batch::DeleteMessageBatchInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_message_batch::DeleteMessageBatchInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_message_batch::DeleteMessageBatchInput {
             queue_url: self.queue_url,
             entries: self.entries,
```

### `src/operation/delete_message_batch/_delete_message_batch_output.rs`

```diff
--- reference/src/operation/delete_message_batch/_delete_message_batch_output.rs
+++ generated/src/operation/delete_message_batch/_delete_message_batch_output.rs
@@ -55,7 +55,10 @@
         self
     }
     /// <p>A list of <code> <code>DeleteMessageBatchResultEntry</code> </code> items.</p>
-    pub fn set_successful(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::DeleteMessageBatchResultEntry>>) -> Self {
+    pub fn set_successful(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::DeleteMessageBatchResultEntry>>,
+    ) -> Self {
         self.successful = input;
         self
     }
@@ -98,8 +101,10 @@
     /// - [`failed`](crate::operation::delete_message_batch::builders::DeleteMessageBatchOutputBuilder::failed)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_message_batch::DeleteMessageBatchOutput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_message_batch::DeleteMessageBatchOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_message_batch::DeleteMessageBatchOutput {
             successful: self.successful.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/delete_message_batch.rs`

```diff
--- reference/src/operation/delete_message_batch.rs
+++ generated/src/operation/delete_message_batch.rs
@@ -253,12 +253,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.DeleteMessageBatch",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_message_batch::ser_delete_message_batch_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_message_batch::ser_delete_message_batch_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_queue/_delete_queue_input.rs`

```diff
--- reference/src/operation/delete_queue/_delete_queue_input.rs
+++ generated/src/operation/delete_queue/_delete_queue_input.rs
@@ -48,7 +48,9 @@
         &self.queue_url
     }
     /// Consumes the builder and constructs a [`DeleteQueueInput`](crate::operation::delete_queue::DeleteQueueInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_queue::DeleteQueueInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_queue::DeleteQueueInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_queue::DeleteQueueInput { queue_url: self.queue_url })
     }
 }
```

### `src/operation/delete_queue.rs`

```diff
--- reference/src/operation/delete_queue.rs
+++ generated/src/operation/delete_queue.rs
@@ -253,8 +253,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.DeleteQueue",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_queue::ser_delete_queue_input(&input)?);
```

### `src/operation/get_queue_attributes/_get_queue_attributes_input.rs`

```diff
--- reference/src/operation/get_queue_attributes/_get_queue_attributes_input.rs
+++ generated/src/operation/get_queue_attributes/_get_queue_attributes_input.rs
@@ -564,8 +564,10 @@
     /// Consumes the builder and constructs a [`GetQueueAttributesInput`](crate::operation::get_queue_attributes::GetQueueAttributesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_queue_attributes::GetQueueAttributesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_queue_attributes::GetQueueAttributesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_queue_attributes::GetQueueAttributesInput {
             queue_url: self.queue_url,
             attribute_names: self.attribute_names,
```

### `src/operation/get_queue_attributes/_get_queue_attributes_output.rs`

```diff
--- reference/src/operation/get_queue_attributes/_get_queue_attributes_output.rs
+++ generated/src/operation/get_queue_attributes/_get_queue_attributes_output.rs
@@ -10,7 +10,9 @@
 }
 impl GetQueueAttributesOutput {
     /// <p>A map of attributes to their respective values.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         self.attributes.as_ref()
     }
 }
@@ -54,7 +56,9 @@
         self
     }
     /// <p>A map of attributes to their respective values.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         &self.attributes
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/get_queue_attributes.rs`

```diff
--- reference/src/operation/get_queue_attributes.rs
+++ generated/src/operation/get_queue_attributes.rs
@@ -253,12 +253,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.GetQueueAttributes",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_queue_attributes::ser_get_queue_attributes_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_queue_attributes::ser_get_queue_attributes_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_queue_url/_get_queue_url_input.rs`

```diff
--- reference/src/operation/get_queue_url/_get_queue_url_input.rs
+++ generated/src/operation/get_queue_url/_get_queue_url_input.rs
@@ -64,7 +64,10 @@
         &self.queue_owner_aws_account_id
     }
     /// Consumes the builder and constructs a [`GetQueueUrlInput`](crate::operation::get_queue_url::GetQueueUrlInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_queue_url::GetQueueUrlInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_queue_url::GetQueueUrlInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::get_queue_url::GetQueueUrlInput {
             queue_name: self.queue_name,
             queue_owner_aws_account_id: self.queue_owner_aws_account_id,
```

### `src/operation/get_queue_url.rs`

```diff
--- reference/src/operation/get_queue_url.rs
+++ generated/src/operation/get_queue_url.rs
@@ -258,8 +258,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.GetQueueUrl",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_queue_url::ser_get_queue_url_input(&input)?);
```

### `src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_input.rs`

```diff
--- reference/src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_input.rs
+++ generated/src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_input.rs
@@ -96,10 +96,12 @@
         super::super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesInput {
-            queue_url: self.queue_url,
-            next_token: self.next_token,
-            max_results: self.max_results,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesInput {
+                queue_url: self.queue_url,
+                next_token: self.next_token,
+                max_results: self.max_results,
+            },
+        )
     }
 }
```

### `src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_output.rs`

```diff
--- reference/src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_output.rs
+++ generated/src/operation/list_dead_letter_source_queues/_list_dead_letter_source_queues_output.rs
@@ -94,15 +94,17 @@
         super::super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput {
-            queue_urls: self.queue_urls.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "queue_urls",
-                    "queue_urls was not specified but it is required when building ListDeadLetterSourceQueuesOutput",
-                )
-            })?,
-            next_token: self.next_token,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput {
+                queue_urls: self.queue_urls.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "queue_urls",
+                        "queue_urls was not specified but it is required when building ListDeadLetterSourceQueuesOutput",
+                    )
+                })?,
+                next_token: self.next_token,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/list_dead_letter_source_queues.rs`

```diff
--- reference/src/operation/list_dead_letter_source_queues.rs
+++ generated/src/operation/list_dead_letter_source_queues.rs
@@ -261,8 +261,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ListDeadLetterSourceQueues",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
```

### `src/operation/list_message_move_tasks/_list_message_move_tasks_input.rs`

```diff
--- reference/src/operation/list_message_move_tasks/_list_message_move_tasks_input.rs
+++ generated/src/operation/list_message_move_tasks/_list_message_move_tasks_input.rs
@@ -65,8 +65,10 @@
     /// Consumes the builder and constructs a [`ListMessageMoveTasksInput`](crate::operation::list_message_move_tasks::ListMessageMoveTasksInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_message_move_tasks::ListMessageMoveTasksInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_message_move_tasks::ListMessageMoveTasksInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_message_move_tasks::ListMessageMoveTasksInput {
             source_arn: self.source_arn,
             max_results: self.max_results,
```

### `src/operation/list_message_move_tasks.rs`

```diff
--- reference/src/operation/list_message_move_tasks.rs
+++ generated/src/operation/list_message_move_tasks.rs
@@ -256,13 +256,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ListMessageMoveTasks",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_message_move_tasks::ser_list_message_move_tasks_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_list_message_move_tasks::ser_list_message_move_tasks_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_queue_tags.rs`

```diff
--- reference/src/operation/list_queue_tags.rs
+++ generated/src/operation/list_queue_tags.rs
@@ -253,8 +253,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ListQueueTags",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_queue_tags::ser_list_queue_tags_input(&input)?);
```

### `src/operation/list_queues/_list_queues_input.rs`

```diff
--- reference/src/operation/list_queues/_list_queues_input.rs
+++ generated/src/operation/list_queues/_list_queues_input.rs
@@ -89,7 +89,9 @@
         &self.max_results
     }
     /// Consumes the builder and constructs a [`ListQueuesInput`](crate::operation::list_queues::ListQueuesInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_queues::ListQueuesInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_queues::ListQueuesInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_queues::ListQueuesInput {
             queue_name_prefix: self.queue_name_prefix,
             next_token: self.next_token,
```

### `src/operation/list_queues.rs`

```diff
--- reference/src/operation/list_queues.rs
+++ generated/src/operation/list_queues.rs
@@ -225,7 +225,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::list_queues::ListQueuesInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::list_queues::ListQueuesInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -256,8 +258,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ListQueues",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_queues::ser_list_queues_input(&input)?);
```

### `src/operation/purge_queue/_purge_queue_input.rs`

```diff
--- reference/src/operation/purge_queue/_purge_queue_input.rs
+++ generated/src/operation/purge_queue/_purge_queue_input.rs
@@ -48,7 +48,9 @@
         &self.queue_url
     }
     /// Consumes the builder and constructs a [`PurgeQueueInput`](crate::operation::purge_queue::PurgeQueueInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::purge_queue::PurgeQueueInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::purge_queue::PurgeQueueInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::purge_queue::PurgeQueueInput { queue_url: self.queue_url })
     }
 }
```

### `src/operation/purge_queue.rs`

```diff
--- reference/src/operation/purge_queue.rs
+++ generated/src/operation/purge_queue.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::purge_queue::PurgeQueueInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::purge_queue::PurgeQueueInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -251,8 +253,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.PurgeQueue",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_purge_queue::ser_purge_queue_input(&input)?);
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
     pub fn attribute_names(&self) -> &[super::super::super::types::QueueAttributeName] {
         self.attribute_names.as_deref().unwrap_or_default()
     }
@@ -536,7 +536,9 @@
     /// <li>
     /// <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li>
     /// </ul>
-    pub fn get_message_system_attribute_names(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::MessageSystemAttributeName>> {
+    pub fn get_message_system_attribute_names(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::MessageSystemAttributeName>> {
         &self.message_system_attribute_names
     }
     /// Appends an item to `message_attribute_names`.
@@ -759,7 +761,8 @@
     /// Consumes the builder and constructs a [`ReceiveMessageInput`](crate::operation::receive_message::ReceiveMessageInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::receive_message::ReceiveMessageInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::receive_message::ReceiveMessageInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::receive_message::ReceiveMessageInput {
             queue_url: self.queue_url,
             attribute_names: self.attribute_names,
```

### `src/operation/receive_message/builders.rs`

```diff
--- reference/src/operation/receive_message/builders.rs
+++ generated/src/operation/receive_message/builders.rs
@@ -363,7 +363,9 @@
     /// <li>
     /// <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li>
     /// </ul>
-    pub fn get_message_system_attribute_names(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::MessageSystemAttributeName>> {
+    pub fn get_message_system_attribute_names(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::MessageSystemAttributeName>> {
         self.inner.get_message_system_attribute_names()
     }
     ///
```

### `src/operation/receive_message.rs`

```diff
--- reference/src/operation/receive_message.rs
+++ generated/src/operation/receive_message.rs
@@ -130,9 +130,6 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                super::super::long_polling::LongPollingInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ReceiveMessageEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
@@ -261,8 +258,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.ReceiveMessage",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_receive_message::ser_receive_message_input(&input)?);
```

### `src/operation/remove_permission/_remove_permission_input.rs`

```diff
--- reference/src/operation/remove_permission/_remove_permission_input.rs
+++ generated/src/operation/remove_permission/_remove_permission_input.rs
@@ -72,7 +72,10 @@
     /// Consumes the builder and constructs a [`RemovePermissionInput`](crate::operation::remove_permission::RemovePermissionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::remove_permission::RemovePermissionInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::remove_permission::RemovePermissionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::remove_permission::RemovePermissionInput {
             queue_url: self.queue_url,
             label: self.label,
```

### `src/operation/remove_permission.rs`

```diff
--- reference/src/operation/remove_permission.rs
+++ generated/src/operation/remove_permission.rs
@@ -258,11 +258,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.RemovePermission",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_remove_permission::ser_remove_permission_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_remove_permission::ser_remove_permission_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/send_message/_send_message_input.rs`

```diff
--- reference/src/operation/send_message/_send_message_input.rs
+++ generated/src/operation/send_message/_send_message_input.rs
@@ -18,7 +18,8 @@
     /// </note>
     pub delay_seconds: ::std::option::Option<i32>,
     /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::MessageAttributeValue>>,
+    pub message_attributes:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::MessageAttributeValue>>,
     /// <p>The message system attribute to send. Each message system attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>.</p><important>
     /// <ul>
     /// <li>
@@ -28,7 +29,10 @@
     /// </ul>
     /// </important>
     pub message_system_attributes: ::std::option::Option<
-        ::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+        ::std::collections::HashMap<
+            super::super::super::types::MessageSystemAttributeNameForSends,
+            super::super::super::types::MessageSystemAttributeValue,
+        >,
     >,
     /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>
     /// <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p>
@@ -107,7 +111,10 @@
     pub fn message_system_attributes(
         &self,
     ) -> ::std::option::Option<
-        &::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+        &::std::collections::HashMap<
+            super::super::super::types::MessageSystemAttributeNameForSends,
+            super::super::super::types::MessageSystemAttributeValue,
+        >,
     > {
         self.message_system_attributes.as_ref()
     }
@@ -169,9 +176,13 @@
     pub(crate) queue_url: ::std::option::Option<::std::string::String>,
     pub(crate) message_body: ::std::option::Option<::std::string::String>,
     pub(crate) delay_seconds: ::std::option::Option<i32>,
-    pub(crate) message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::MessageAttributeValue>>,
+    pub(crate) message_attributes:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::MessageAttributeValue>>,
     pub(crate) message_system_attributes: ::std::option::Option<
-        ::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+        ::std::collections::HashMap<
+            super::super::super::types::MessageSystemAttributeNameForSends,
+            super::super::super::types::MessageSystemAttributeValue,
+        >,
     >,
     pub(crate) message_deduplication_id: ::std::option::Option<::std::string::String>,
     pub(crate) message_group_id: ::std::option::Option<::std::string::String>,
@@ -247,7 +258,11 @@
     /// To override the contents of this collection use [`set_message_attributes`](Self::set_message_attributes).
     ///
     /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn message_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::MessageAttributeValue) -> Self {
+    pub fn message_attributes(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::MessageAttributeValue,
+    ) -> Self {
         let mut hash_map = self.message_attributes.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.message_attributes = ::std::option::Option::Some(hash_map);
@@ -300,7 +315,10 @@
     pub fn set_message_system_attributes(
         mut self,
         input: ::std::option::Option<
-            ::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+            ::std::collections::HashMap<
+                super::super::super::types::MessageSystemAttributeNameForSends,
+                super::super::super::types::MessageSystemAttributeValue,
+            >,
         >,
     ) -> Self {
         self.message_system_attributes = input;
@@ -317,7 +335,10 @@
     pub fn get_message_system_attributes(
         &self,
     ) -> &::std::option::Option<
-        ::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+        ::std::collections::HashMap<
+            super::super::super::types::MessageSystemAttributeNameForSends,
+            super::super::super::types::MessageSystemAttributeValue,
+        >,
     > {
         &self.message_system_attributes
     }
@@ -455,7 +476,9 @@
         &self.message_group_id
     }
     /// Consumes the builder and constructs a [`SendMessageInput`](crate::operation::send_message::SendMessageInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::send_message::SendMessageInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::send_message::SendMessageInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::send_message::SendMessageInput {
             queue_url: self.queue_url,
             message_body: self.message_body,
```

### `src/operation/send_message/builders.rs`

```diff
--- reference/src/operation/send_message/builders.rs
+++ generated/src/operation/send_message/builders.rs
@@ -181,7 +181,11 @@
     /// To override the contents of this collection use [`set_message_attributes`](Self::set_message_attributes).
     ///
     /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn message_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::MessageAttributeValue) -> Self {
+    pub fn message_attributes(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::MessageAttributeValue,
+    ) -> Self {
         self.inner = self.inner.message_attributes(k.into(), v);
         self
     }
@@ -231,7 +235,10 @@
     pub fn set_message_system_attributes(
         mut self,
         input: ::std::option::Option<
-            ::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+            ::std::collections::HashMap<
+                super::super::super::types::MessageSystemAttributeNameForSends,
+                super::super::super::types::MessageSystemAttributeValue,
+            >,
         >,
     ) -> Self {
         self.inner = self.inner.set_message_system_attributes(input);
@@ -248,7 +255,10 @@
     pub fn get_message_system_attributes(
         &self,
     ) -> &::std::option::Option<
-        ::std::collections::HashMap<super::super::super::types::MessageSystemAttributeNameForSends, super::super::super::types::MessageSystemAttributeValue>,
+        ::std::collections::HashMap<
+            super::super::super::types::MessageSystemAttributeNameForSends,
+            super::super::super::types::MessageSystemAttributeValue,
+        >,
     > {
         self.inner.get_message_system_attributes()
     }
```

### `src/operation/send_message.rs`

```diff
--- reference/src/operation/send_message.rs
+++ generated/src/operation/send_message.rs
@@ -268,8 +268,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.SendMessage",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_send_message::ser_send_message_input(&input)?);
```

### `src/operation/send_message_batch/_send_message_batch_input.rs`

```diff
--- reference/src/operation/send_message_batch/_send_message_batch_input.rs
+++ generated/src/operation/send_message_batch/_send_message_batch_input.rs
@@ -79,7 +79,10 @@
     /// Consumes the builder and constructs a [`SendMessageBatchInput`](crate::operation::send_message_batch::SendMessageBatchInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::send_message_batch::SendMessageBatchInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::send_message_batch::SendMessageBatchInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::send_message_batch::SendMessageBatchInput {
             queue_url: self.queue_url,
             entries: self.entries,
```

### `src/operation/send_message_batch/_send_message_batch_output.rs`

```diff
--- reference/src/operation/send_message_batch/_send_message_batch_output.rs
+++ generated/src/operation/send_message_batch/_send_message_batch_output.rs
@@ -98,7 +98,10 @@
     /// - [`failed`](crate::operation::send_message_batch::builders::SendMessageBatchOutputBuilder::failed)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::send_message_batch::SendMessageBatchOutput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::send_message_batch::SendMessageBatchOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::send_message_batch::SendMessageBatchOutput {
             successful: self.successful.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/send_message_batch.rs`

```diff
--- reference/src/operation/send_message_batch.rs
+++ generated/src/operation/send_message_batch.rs
@@ -253,11 +253,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.SendMessageBatch",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_send_message_batch::ser_send_message_batch_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_send_message_batch::ser_send_message_batch_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/set_queue_attributes/_set_queue_attributes_input.rs`

```diff
--- reference/src/operation/set_queue_attributes/_set_queue_attributes_input.rs
+++ generated/src/operation/set_queue_attributes/_set_queue_attributes_input.rs
@@ -201,7 +201,9 @@
     /// </ul>
     /// <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p>
     /// <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         self.attributes.as_ref()
     }
 }
@@ -534,14 +536,18 @@
     /// </ul>
     /// <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p>
     /// <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         &self.attributes
     }
     /// Consumes the builder and constructs a [`SetQueueAttributesInput`](crate::operation::set_queue_attributes::SetQueueAttributesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::set_queue_attributes::SetQueueAttributesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::set_queue_attributes::SetQueueAttributesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::set_queue_attributes::SetQueueAttributesInput {
             queue_url: self.queue_url,
             attributes: self.attributes,
```

### `src/operation/set_queue_attributes/builders.rs`

```diff
--- reference/src/operation/set_queue_attributes/builders.rs
+++ generated/src/operation/set_queue_attributes/builders.rs
@@ -429,7 +429,9 @@
     /// </ul>
     /// <p>If you set these attributes to anything other than the values shown for enabling high throughput, normal throughput is in effect and deduplication occurs as specified.</p>
     /// <p>For information on throughput quotas, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/quotas-messages.html">Quotas related to messages</a> in the <i>Amazon SQS Developer Guide</i>.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<super::super::super::types::QueueAttributeName, ::std::string::String>> {
         self.inner.get_attributes()
     }
 }
```

### `src/operation/set_queue_attributes.rs`

```diff
--- reference/src/operation/set_queue_attributes.rs
+++ generated/src/operation/set_queue_attributes.rs
@@ -253,12 +253,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.SetQueueAttributes",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_set_queue_attributes::ser_set_queue_attributes_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_set_queue_attributes::ser_set_queue_attributes_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/start_message_move_task/_start_message_move_task_input.rs`

```diff
--- reference/src/operation/start_message_move_task/_start_message_move_task_input.rs
+++ generated/src/operation/start_message_move_task/_start_message_move_task_input.rs
@@ -86,8 +86,10 @@
     /// Consumes the builder and constructs a [`StartMessageMoveTaskInput`](crate::operation::start_message_move_task::StartMessageMoveTaskInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::start_message_move_task::StartMessageMoveTaskInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::start_message_move_task::StartMessageMoveTaskInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::start_message_move_task::StartMessageMoveTaskInput {
             source_arn: self.source_arn,
             destination_arn: self.destination_arn,
```

### `src/operation/start_message_move_task.rs`

```diff
--- reference/src/operation/start_message_move_task.rs
+++ generated/src/operation/start_message_move_task.rs
@@ -261,13 +261,11 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.StartMessageMoveTask",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_start_message_move_task::ser_start_message_move_task_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_start_message_move_task::ser_start_message_move_task_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/tag_queue/_tag_queue_input.rs`

```diff
--- reference/src/operation/tag_queue/_tag_queue_input.rs
+++ generated/src/operation/tag_queue/_tag_queue_input.rs
@@ -69,7 +69,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`TagQueueInput`](crate::operation::tag_queue::TagQueueInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::tag_queue::TagQueueInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::tag_queue::TagQueueInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::tag_queue::TagQueueInput {
             queue_url: self.queue_url,
             tags: self.tags,
```

### `src/operation/tag_queue/builders.rs`

```diff
--- reference/src/operation/tag_queue/builders.rs
+++ generated/src/operation/tag_queue/builders.rs
@@ -43,14 +43,20 @@
     inner: super::super::super::operation::tag_queue::builders::TagQueueInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::tag_queue::TagQueueOutput, super::super::super::operation::tag_queue::TagQueueError>
-    for TagQueueFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::tag_queue::TagQueueOutput,
+        super::super::super::operation::tag_queue::TagQueueError,
+    > for TagQueueFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::tag_queue::TagQueueOutput, super::super::super::operation::tag_queue::TagQueueError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::tag_queue::TagQueueOutput,
+            super::super::super::operation::tag_queue::TagQueueError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -100,8 +106,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::tag_queue::TagQueueOutput, super::super::super::operation::tag_queue::TagQueueError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::tag_queue::TagQueueOutput,
+        super::super::super::operation::tag_queue::TagQueueError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/tag_queue.rs`

```diff
--- reference/src/operation/tag_queue.rs
+++ generated/src/operation/tag_queue.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::tag_queue::TagQueueError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::tag_queue::TagQueueError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -216,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::tag_queue::TagQueueInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::tag_queue::TagQueueInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -247,8 +253,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.TagQueue",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_queue::ser_tag_queue_input(&input)?);
```

### `src/operation/untag_queue/_untag_queue_input.rs`

```diff
--- reference/src/operation/untag_queue/_untag_queue_input.rs
+++ generated/src/operation/untag_queue/_untag_queue_input.rs
@@ -71,7 +71,9 @@
         &self.tag_keys
     }
     /// Consumes the builder and constructs a [`UntagQueueInput`](crate::operation::untag_queue::UntagQueueInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::untag_queue::UntagQueueInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::untag_queue::UntagQueueInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::untag_queue::UntagQueueInput {
             queue_url: self.queue_url,
             tag_keys: self.tag_keys,
```

### `src/operation/untag_queue.rs`

```diff
--- reference/src/operation/untag_queue.rs
+++ generated/src/operation/untag_queue.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::untag_queue::UntagQueueInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::untag_queue::UntagQueueInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -251,8 +253,6 @@
                 ::http_1x::header::HeaderName::from_static("x-amz-target"),
                 "AmazonSQS.UntagQueue",
             );
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::HeaderName::from_static("x-amzn-query-mode"), "true");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_queue::ser_untag_queue_input(&input)?);
```

### `src/protocol_serde/shape_add_permission.rs`

```diff
--- reference/src/protocol_serde/shape_add_permission.rs
+++ generated/src/protocol_serde/shape_add_permission.rs
@@ -62,7 +62,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::add_permission::AddPermissionError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::add_permission::AddPermissionError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -92,7 +92,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::add_permission::AddPermissionError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::add_permission::AddPermissionError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -134,3 +134,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_add_permission(
+    _value: &[u8],
+    mut builder: super::super::operation::add_permission::builders::AddPermissionOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::add_permission::builders::AddPermissionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_batch_entry_ids_not_distinct.rs`

```diff
--- reference/src/protocol_serde/shape_batch_entry_ids_not_distinct.rs
+++ generated/src/protocol_serde/shape_batch_entry_ids_not_distinct.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_batch_entry_ids_not_distinct_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_cancel_message_move_task.rs`

```diff
--- reference/src/protocol_serde/shape_cancel_message_move_task.rs
+++ generated/src/protocol_serde/shape_cancel_message_move_task.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -70,8 +74,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,23 +87,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => {
-            super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::UnsupportedOperation({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
-                    output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
-                        .map_err(super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
+                output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
+                    .map_err(super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         _ => super::super::operation::cancel_message_move_task::CancelMessageMoveTaskError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_change_message_visibility.rs`

```diff
--- reference/src/protocol_serde/shape_change_message_visibility.rs
+++ generated/src/protocol_serde/shape_change_message_visibility.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::change_message_visibility::ChangeMessageVisibilityError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::change_message_visibility::ChangeMessageVisibilityError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -54,24 +50,22 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.MessageNotInflight" => {
-            super::super::operation::change_message_visibility::ChangeMessageVisibilityError::MessageNotInflight({
+        "MessageNotInflight" => super::super::operation::change_message_visibility::ChangeMessageVisibilityError::MessageNotInflight({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::MessageNotInflightBuilder::default();
-                    output = super::super::protocol_serde::shape_message_not_inflight::de_message_not_inflight_json_err(_response_body, output)
-                        .map_err(super::super::operation::change_message_visibility::ChangeMessageVisibilityError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::change_message_visibility::ChangeMessageVisibilityError::QueueDoesNotExist({
+                let mut output = super::super::types::error::builders::MessageNotInflightBuilder::default();
+                output = super::super::protocol_serde::shape_message_not_inflight::de_message_not_inflight_json_err(_response_body, output)
+                    .map_err(super::super::operation::change_message_visibility::ChangeMessageVisibilityError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
+        "QueueDoesNotExist" => super::super::operation::change_message_visibility::ChangeMessageVisibilityError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -116,23 +110,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => {
-            super::super::operation::change_message_visibility::ChangeMessageVisibilityError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::change_message_visibility::ChangeMessageVisibilityError::UnsupportedOperation({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
-                    output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
-                        .map_err(super::super::operation::change_message_visibility::ChangeMessageVisibilityError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
+                output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
+                    .map_err(super::super::operation::change_message_visibility::ChangeMessageVisibilityError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         _ => super::super::operation::change_message_visibility::ChangeMessageVisibilityError::generic(generic),
     })
 }
@@ -163,3 +155,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_change_message_visibility(
+    _value: &[u8],
+    mut builder: super::super::operation::change_message_visibility::builders::ChangeMessageVisibilityOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::change_message_visibility::builders::ChangeMessageVisibilityOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_change_message_visibility_batch.rs`

```diff
--- reference/src/protocol_serde/shape_change_message_visibility_batch.rs
+++ generated/src/protocol_serde/shape_change_message_visibility_batch.rs
@@ -20,15 +20,17 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => {
+        "BatchEntryIdsNotDistinct" => {
             super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::BatchEntryIdsNotDistinct({
                 #[allow(unused_mut)]
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(_response_body, output)
-                            .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
+                    output = super::super::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -38,23 +40,21 @@
                 tmp
             })
         }
-        "AWS.SimpleQueueService.EmptyBatchRequest" => {
-            super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::EmptyBatchRequest({
+        "EmptyBatchRequest" => super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::EmptyBatchRequest({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::EmptyBatchRequestBuilder::default();
-                    output = super::super::protocol_serde::shape_empty_batch_request::de_empty_batch_request_json_err(_response_body, output)
-                        .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::EmptyBatchRequestBuilder::default();
+                output = super::super::protocol_serde::shape_empty_batch_request::de_empty_batch_request_json_err(_response_body, output)
+                    .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "InvalidAddress" => super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::InvalidAddress({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -70,23 +70,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.InvalidBatchEntryId" => {
-            super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::InvalidBatchEntryId({
+        "InvalidBatchEntryId" => super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::InvalidBatchEntryId({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::InvalidBatchEntryIdBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_batch_entry_id::de_invalid_batch_entry_id_json_err(_response_body, output)
-                        .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::InvalidBatchEntryIdBuilder::default();
+                output = super::super::protocol_serde::shape_invalid_batch_entry_id::de_invalid_batch_entry_id_json_err(_response_body, output)
+                    .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "InvalidSecurity" => super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::InvalidSecurity({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -102,23 +100,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => {
-            super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::QueueDoesNotExist({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::QueueDoesNotExistBuilder::default();
-                    output = super::super::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_json_err(_response_body, output)
-                        .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::QueueDoesNotExistBuilder::default();
+                output = super::super::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_json_err(_response_body, output)
+                    .map_err(super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "RequestThrottled" => super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::RequestThrottled({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -134,7 +130,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
+        "TooManyEntriesInBatchRequest" => {
             super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::TooManyEntriesInBatchRequest({
                 #[allow(unused_mut)]
                 let mut tmp = {
@@ -154,7 +150,7 @@
                 tmp
             })
         }
-        "AWS.SimpleQueueService.UnsupportedOperation" => {
+        "UnsupportedOperation" => {
             super::super::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError::UnsupportedOperation({
                 #[allow(unused_mut)]
                 let mut tmp = {
@@ -223,13 +219,15 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "Successful" => {
-                    builder = builder.set_successful(
-                            super::super::protocol_serde::shape_change_message_visibility_batch_result_entry_list::de_change_message_visibility_batch_result_entry_list(tokens, _value, depth + 1)?
-                        );
+                    builder = builder.set_successful(super::super::protocol_serde::shape_change_message_visibility_batch_result_entry_list::de_change_message_visibility_batch_result_entry_list(tokens, _value, depth + 1)?);
                 }
                 "Failed" => {
                     builder = builder.set_failed(
-                        super::super::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_change_message_visibility_batch_result_entry.rs`

```diff
--- reference/src/protocol_serde/shape_change_message_visibility_batch_result_entry.rs
+++ generated/src/protocol_serde/shape_change_message_visibility_batch_result_entry.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<super::super::types::ChangeMessageVisibilityBatchResultEntry>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<super::super::types::ChangeMessageVisibilityBatchResultEntry>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_create_queue.rs`

```diff
--- reference/src/protocol_serde/shape_create_queue.rs
+++ generated/src/protocol_serde/shape_create_queue.rs
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.QueueDeletedRecently" => super::super::operation::create_queue::CreateQueueError::QueueDeletedRecently({
+        "QueueDeletedRecently" => super::super::operation::create_queue::CreateQueueError::QueueDeletedRecently({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -92,7 +92,7 @@
             }
             tmp
         }),
-        "QueueAlreadyExists" => super::super::operation::create_queue::CreateQueueError::QueueNameExists({
+        "QueueNameExists" => super::super::operation::create_queue::CreateQueueError::QueueNameExists({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -122,7 +122,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::create_queue::CreateQueueError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::create_queue::CreateQueueError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -170,8 +170,10 @@
 pub(crate) fn de_create_queue(
     _value: &[u8],
     mut builder: super::super::operation::create_queue::builders::CreateQueueOutputBuilder,
-) -> ::std::result::Result<super::super::operation::create_queue::builders::CreateQueueOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::create_queue::builders::CreateQueueOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_delete_message.rs`

```diff
--- reference/src/protocol_serde/shape_delete_message.rs
+++ generated/src/protocol_serde/shape_delete_message.rs
@@ -62,7 +62,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::delete_message::DeleteMessageError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::delete_message::DeleteMessageError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -107,7 +107,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::delete_message::DeleteMessageError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::delete_message::DeleteMessageError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -149,3 +149,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_message(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_message::builders::DeleteMessageOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_message::builders::DeleteMessageOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_message_batch.rs`

```diff
--- reference/src/protocol_serde/shape_delete_message_batch.rs
+++ generated/src/protocol_serde/shape_delete_message_batch.rs
@@ -20,25 +20,25 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => {
-            super::super::operation::delete_message_batch::DeleteMessageBatchError::BatchEntryIdsNotDistinct({
+        "BatchEntryIdsNotDistinct" => super::super::operation::delete_message_batch::DeleteMessageBatchError::BatchEntryIdsNotDistinct({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(_response_body, output)
-                            .map_err(super::super::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
-        "AWS.SimpleQueueService.EmptyBatchRequest" => super::super::operation::delete_message_batch::DeleteMessageBatchError::EmptyBatchRequest({
+                let mut output = super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder::default();
+                output = super::super::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
+        "EmptyBatchRequest" => super::super::operation::delete_message_batch::DeleteMessageBatchError::EmptyBatchRequest({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -68,7 +68,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.InvalidBatchEntryId" => super::super::operation::delete_message_batch::DeleteMessageBatchError::InvalidBatchEntryId({
+        "InvalidBatchEntryId" => super::super::operation::delete_message_batch::DeleteMessageBatchError::InvalidBatchEntryId({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -98,7 +98,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::delete_message_batch::DeleteMessageBatchError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::delete_message_batch::DeleteMessageBatchError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -128,27 +128,25 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
-            super::super::operation::delete_message_batch::DeleteMessageBatchError::TooManyEntriesInBatchRequest({
+        "TooManyEntriesInBatchRequest" => super::super::operation::delete_message_batch::DeleteMessageBatchError::TooManyEntriesInBatchRequest({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder::default();
-                    output = super::super::protocol_serde::shape_too_many_entries_in_batch_request::de_too_many_entries_in_batch_request_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::delete_message_batch::DeleteMessageBatchError::UnsupportedOperation({
+                let mut output = super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder::default();
+                output = super::super::protocol_serde::shape_too_many_entries_in_batch_request::de_too_many_entries_in_batch_request_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
+        "UnsupportedOperation" => super::super::operation::delete_message_batch::DeleteMessageBatchError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -225,7 +223,11 @@
                 }
                 "Failed" => {
                     builder = builder.set_failed(
-                        super::super::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_delete_message_batch_input.rs`

```diff
--- reference/src/protocol_serde/shape_delete_message_batch_input.rs
+++ generated/src/protocol_serde/shape_delete_message_batch_input.rs
@@ -12,7 +12,10 @@
             {
                 #[allow(unused_mut)]
                 let mut object_5 = array_3.value().start_object();
-                super::super::protocol_serde::shape_delete_message_batch_request_entry::ser_delete_message_batch_request_entry(&mut object_5, item_4)?;
+                super::super::protocol_serde::shape_delete_message_batch_request_entry::ser_delete_message_batch_request_entry(
+                    &mut object_5,
+                    item_4,
+                )?;
                 object_5.finish();
             }
         }
```

### `src/protocol_serde/shape_delete_queue.rs`

```diff
--- reference/src/protocol_serde/shape_delete_queue.rs
+++ generated/src/protocol_serde/shape_delete_queue.rs
@@ -47,7 +47,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::delete_queue::DeleteQueueError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::delete_queue::DeleteQueueError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::delete_queue::DeleteQueueError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::delete_queue::DeleteQueueError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -119,3 +119,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_queue(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_queue::builders::DeleteQueueOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_queue::builders::DeleteQueueOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_get_queue_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_get_queue_attributes.rs
+++ generated/src/protocol_serde/shape_get_queue_attributes.rs
@@ -65,7 +65,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::get_queue_attributes::GetQueueAttributesError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::get_queue_attributes::GetQueueAttributesError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -95,7 +95,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::get_queue_attributes::GetQueueAttributesError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::get_queue_attributes::GetQueueAttributesError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
```

### `src/protocol_serde/shape_get_queue_url.rs`

```diff
--- reference/src/protocol_serde/shape_get_queue_url.rs
+++ generated/src/protocol_serde/shape_get_queue_url.rs
@@ -47,7 +47,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::get_queue_url::GetQueueUrlError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::get_queue_url::GetQueueUrlError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::get_queue_url::GetQueueUrlError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::get_queue_url::GetQueueUrlError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -125,8 +125,10 @@
 pub(crate) fn de_get_queue_url(
     _value: &[u8],
     mut builder: super::super::operation::get_queue_url::builders::GetQueueUrlOutputBuilder,
-) -> ::std::result::Result<super::super::operation::get_queue_url::builders::GetQueueUrlOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::get_queue_url::builders::GetQueueUrlOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_id_format.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_id_format.rs
+++ generated/src/protocol_serde/shape_invalid_id_format.rs
@@ -13,11 +13,7 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "Message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                    builder = builder.set_message(::aws_smithy_json::deserialize::token::skip_value(tokens)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_list_dead_letter_source_queues.rs`

```diff
--- reference/src/protocol_serde/shape_list_dead_letter_source_queues.rs
+++ generated/src/protocol_serde/shape_list_dead_letter_source_queues.rs
@@ -50,23 +50,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => {
-            super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::QueueDoesNotExist({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::QueueDoesNotExistBuilder::default();
-                    output = super::super::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_json_err(_response_body, output)
-                        .map_err(super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::QueueDoesNotExistBuilder::default();
+                output = super::super::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_json_err(_response_body, output)
+                    .map_err(super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "RequestThrottled" => super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::RequestThrottled({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -82,23 +80,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => {
-            super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::UnsupportedOperation({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
-                    output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
-                        .map_err(super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
+                output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
+                    .map_err(super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         _ => super::super::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::generic(generic),
     })
 }
@@ -151,7 +147,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "queueUrls" => {
-                    builder = builder.set_queue_urls(super::super::protocol_serde::shape_queue_url_list::de_queue_url_list(tokens, _value, depth + 1)?);
+                    builder = builder.set_queue_urls(super::super::protocol_serde::shape_queue_url_list::de_queue_url_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "NextToken" => {
                     builder = builder.set_next_token(
```

### `src/protocol_serde/shape_list_message_move_tasks.rs`

```diff
--- reference/src/protocol_serde/shape_list_message_move_tasks.rs
+++ generated/src/protocol_serde/shape_list_message_move_tasks.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -70,8 +74,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,23 +87,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => {
-            super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::UnsupportedOperation({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
-                    output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
-                        .map_err(super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
+                output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
+                    .map_err(super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         _ => super::super::operation::list_message_move_tasks::ListMessageMoveTasksError::generic(generic),
     })
 }
@@ -145,18 +150,14 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Results" => {
-                    builder = builder.set_results(
-                        super::super::protocol_serde::shape_list_message_move_tasks_result_entry_list::de_list_message_move_tasks_result_entry_list(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
-                    );
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "Results" => {
+                        builder = builder.set_results(super::super::protocol_serde::shape_list_message_move_tasks_result_entry_list::de_list_message_move_tasks_result_entry_list(tokens, _value, depth + 1)?);
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_list_queue_tags.rs`

```diff
--- reference/src/protocol_serde/shape_list_queue_tags.rs
+++ generated/src/protocol_serde/shape_list_queue_tags.rs
@@ -4,7 +4,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_queue_tags::ListQueueTagsOutput, super::super::operation::list_queue_tags::ListQueueTagsError> {
+) -> std::result::Result<super::super::operation::list_queue_tags::ListQueueTagsOutput, super::super::operation::list_queue_tags::ListQueueTagsError>
+{
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::list_queue_tags::ListQueueTagsError::unhandled)?;
@@ -47,7 +48,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::list_queue_tags::ListQueueTagsError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::list_queue_tags::ListQueueTagsError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +78,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::list_queue_tags::ListQueueTagsError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::list_queue_tags::ListQueueTagsError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -101,7 +102,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_queue_tags::ListQueueTagsOutput, super::super::operation::list_queue_tags::ListQueueTagsError> {
+) -> std::result::Result<super::super::operation::list_queue_tags::ListQueueTagsOutput, super::super::operation::list_queue_tags::ListQueueTagsError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::list_queue_tags::builders::ListQueueTagsOutputBuilder::default();
```

### `src/protocol_serde/shape_list_queues.rs`

```diff
--- reference/src/protocol_serde/shape_list_queues.rs
+++ generated/src/protocol_serde/shape_list_queues.rs
@@ -62,7 +62,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::list_queues::ListQueuesError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::list_queues::ListQueuesError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -110,8 +110,10 @@
 pub(crate) fn de_list_queues(
     _value: &[u8],
     mut builder: super::super::operation::list_queues::builders::ListQueuesOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_queues::builders::ListQueuesOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_queues::builders::ListQueuesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -122,7 +124,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "QueueUrls" => {
-                    builder = builder.set_queue_urls(super::super::protocol_serde::shape_queue_url_list::de_queue_url_list(tokens, _value, depth + 1)?);
+                    builder = builder.set_queue_urls(super::super::protocol_serde::shape_queue_url_list::de_queue_url_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "NextToken" => {
                     builder = builder.set_next_token(
```

### `src/protocol_serde/shape_message.rs`

```diff
--- reference/src/protocol_serde/shape_message.rs
+++ generated/src/protocol_serde/shape_message.rs
@@ -67,7 +67,11 @@
                         }
                         "MessageAttributes" => {
                             builder = builder.set_message_attributes(
-                                super::super::protocol_serde::shape_message_body_attribute_map::de_message_body_attribute_map(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_message_body_attribute_map::de_message_body_attribute_map(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_message_attribute_value.rs`

```diff
--- reference/src/protocol_serde/shape_message_attribute_value.rs
+++ generated/src/protocol_serde/shape_message_attribute_value.rs
@@ -66,12 +66,18 @@
                             builder = builder.set_binary_value(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
                         }
                         "StringListValues" => {
-                            builder =
-                                builder.set_string_list_values(super::super::protocol_serde::shape_string_list::de_string_list(tokens, _value, depth + 1)?);
+                            builder = builder.set_string_list_values(super::super::protocol_serde::shape_string_list::de_string_list(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "BinaryListValues" => {
-                            builder =
-                                builder.set_binary_list_values(super::super::protocol_serde::shape_binary_list::de_binary_list(tokens, _value, depth + 1)?);
+                            builder = builder.set_binary_list_values(super::super::protocol_serde::shape_binary_list::de_binary_list(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "DataType" => {
                             builder = builder.set_data_type(
@@ -89,9 +95,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::message_attribute_value_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::message_attribute_value_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_message_not_inflight.rs`

```diff
--- reference/src/protocol_serde/shape_message_not_inflight.rs
+++ generated/src/protocol_serde/shape_message_not_inflight.rs
@@ -13,11 +13,7 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "Message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                    builder = builder.set_message(::aws_smithy_json::deserialize::token::skip_value(tokens)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_message_system_attribute_map.rs`

```diff
--- reference/src/protocol_serde/shape_message_system_attribute_map.rs
+++ generated/src/protocol_serde/shape_message_system_attribute_map.rs
@@ -23,7 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key.to_unescaped().map(|u| super::super::types::MessageSystemAttributeName::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?;
```

### `src/protocol_serde/shape_purge_queue.rs`

```diff
--- reference/src/protocol_serde/shape_purge_queue.rs
+++ generated/src/protocol_serde/shape_purge_queue.rs
@@ -47,7 +47,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.PurgeQueueInProgress" => super::super::operation::purge_queue::PurgeQueueError::PurgeQueueInProgress({
+        "PurgeQueueInProgress" => super::super::operation::purge_queue::PurgeQueueError::PurgeQueueInProgress({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -62,7 +62,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::purge_queue::PurgeQueueError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::purge_queue::PurgeQueueError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -92,7 +92,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::purge_queue::PurgeQueueError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::purge_queue::PurgeQueueError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -134,3 +134,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_purge_queue(
+    _value: &[u8],
+    mut builder: super::super::operation::purge_queue::builders::PurgeQueueOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::purge_queue::builders::PurgeQueueOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_queue_attribute_map.rs`

```diff
--- reference/src/protocol_serde/shape_queue_attribute_map.rs
+++ generated/src/protocol_serde/shape_queue_attribute_map.rs
@@ -23,7 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key.to_unescaped().map(|u| super::super::types::QueueAttributeName::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?;
```

### `src/protocol_serde/shape_receive_message.rs`

```diff
--- reference/src/protocol_serde/shape_receive_message.rs
+++ generated/src/protocol_serde/shape_receive_message.rs
@@ -4,7 +4,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::receive_message::ReceiveMessageOutput, super::super::operation::receive_message::ReceiveMessageError> {
+) -> std::result::Result<super::super::operation::receive_message::ReceiveMessageOutput, super::super::operation::receive_message::ReceiveMessageError>
+{
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::receive_message::ReceiveMessageError::unhandled)?;
@@ -47,7 +48,7 @@
             }
             tmp
         }),
-        "KMS.AccessDeniedException" => super::super::operation::receive_message::ReceiveMessageError::KmsAccessDenied({
+        "KmsAccessDenied" => super::super::operation::receive_message::ReceiveMessageError::KmsAccessDenied({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -62,7 +63,7 @@
             }
             tmp
         }),
-        "KMS.DisabledException" => super::super::operation::receive_message::ReceiveMessageError::KmsDisabled({
+        "KmsDisabled" => super::super::operation::receive_message::ReceiveMessageError::KmsDisabled({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +78,7 @@
             }
             tmp
         }),
-        "KMS.InvalidKeyUsageException" => super::super::operation::receive_message::ReceiveMessageError::KmsInvalidKeyUsage({
+        "KmsInvalidKeyUsage" => super::super::operation::receive_message::ReceiveMessageError::KmsInvalidKeyUsage({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -92,7 +93,7 @@
             }
             tmp
         }),
-        "KMS.InvalidStateException" => super::super::operation::receive_message::ReceiveMessageError::KmsInvalidState({
+        "KmsInvalidState" => super::super::operation::receive_message::ReceiveMessageError::KmsInvalidState({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -107,7 +108,7 @@
             }
             tmp
         }),
-        "KMS.NotFoundException" => super::super::operation::receive_message::ReceiveMessageError::KmsNotFound({
+        "KmsNotFound" => super::super::operation::receive_message::ReceiveMessageError::KmsNotFound({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -122,7 +123,7 @@
             }
             tmp
         }),
-        "KMS.OptInRequired" => super::super::operation::receive_message::ReceiveMessageError::KmsOptInRequired({
+        "KmsOptInRequired" => super::super::operation::receive_message::ReceiveMessageError::KmsOptInRequired({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -137,7 +138,7 @@
             }
             tmp
         }),
-        "KMS.ThrottlingException" => super::super::operation::receive_message::ReceiveMessageError::KmsThrottled({
+        "KmsThrottled" => super::super::operation::receive_message::ReceiveMessageError::KmsThrottled({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -167,7 +168,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::receive_message::ReceiveMessageError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::receive_message::ReceiveMessageError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -197,7 +198,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::receive_message::ReceiveMessageError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::receive_message::ReceiveMessageError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -221,7 +222,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::receive_message::ReceiveMessageOutput, super::super::operation::receive_message::ReceiveMessageError> {
+) -> std::result::Result<super::super::operation::receive_message::ReceiveMessageOutput, super::super::operation::receive_message::ReceiveMessageError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::receive_message::builders::ReceiveMessageOutputBuilder::default();
@@ -259,7 +261,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "Messages" => {
-                    builder = builder.set_messages(super::super::protocol_serde::shape_message_list::de_message_list(tokens, _value, depth + 1)?);
+                    builder = builder.set_messages(super::super::protocol_serde::shape_message_list::de_message_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_remove_permission.rs`

```diff
--- reference/src/protocol_serde/shape_remove_permission.rs
+++ generated/src/protocol_serde/shape_remove_permission.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::remove_permission::RemovePermissionOutput, super::super::operation::remove_permission::RemovePermissionError> {
+) -> std::result::Result<
+    super::super::operation::remove_permission::RemovePermissionOutput,
+    super::super::operation::remove_permission::RemovePermissionError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::remove_permission::RemovePermissionError::unhandled)?;
@@ -47,7 +50,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::remove_permission::RemovePermissionError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::remove_permission::RemovePermissionError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +80,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::remove_permission::RemovePermissionError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::remove_permission::RemovePermissionError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -101,7 +104,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::remove_permission::RemovePermissionOutput, super::super::operation::remove_permission::RemovePermissionError> {
+) -> std::result::Result<
+    super::super::operation::remove_permission::RemovePermissionOutput,
+    super::super::operation::remove_permission::RemovePermissionError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::remove_permission::builders::RemovePermissionOutputBuilder::default();
@@ -119,3 +125,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_remove_permission(
+    _value: &[u8],
+    mut builder: super::super::operation::remove_permission::builders::RemovePermissionOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::remove_permission::builders::RemovePermissionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_resource_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_resource_not_found_exception.rs
+++ generated/src/protocol_serde/shape_resource_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_resource_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ResourceNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ResourceNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ResourceNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_send_message.rs`

```diff
--- reference/src/protocol_serde/shape_send_message.rs
+++ generated/src/protocol_serde/shape_send_message.rs
@@ -62,7 +62,7 @@
             }
             tmp
         }),
-        "KMS.AccessDeniedException" => super::super::operation::send_message::SendMessageError::KmsAccessDenied({
+        "KmsAccessDenied" => super::super::operation::send_message::SendMessageError::KmsAccessDenied({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        "KMS.DisabledException" => super::super::operation::send_message::SendMessageError::KmsDisabled({
+        "KmsDisabled" => super::super::operation::send_message::SendMessageError::KmsDisabled({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -92,7 +92,7 @@
             }
             tmp
         }),
-        "KMS.InvalidKeyUsageException" => super::super::operation::send_message::SendMessageError::KmsInvalidKeyUsage({
+        "KmsInvalidKeyUsage" => super::super::operation::send_message::SendMessageError::KmsInvalidKeyUsage({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -107,7 +107,7 @@
             }
             tmp
         }),
-        "KMS.InvalidStateException" => super::super::operation::send_message::SendMessageError::KmsInvalidState({
+        "KmsInvalidState" => super::super::operation::send_message::SendMessageError::KmsInvalidState({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -122,7 +122,7 @@
             }
             tmp
         }),
-        "KMS.NotFoundException" => super::super::operation::send_message::SendMessageError::KmsNotFound({
+        "KmsNotFound" => super::super::operation::send_message::SendMessageError::KmsNotFound({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -137,7 +137,7 @@
             }
             tmp
         }),
-        "KMS.OptInRequired" => super::super::operation::send_message::SendMessageError::KmsOptInRequired({
+        "KmsOptInRequired" => super::super::operation::send_message::SendMessageError::KmsOptInRequired({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -152,7 +152,7 @@
             }
             tmp
         }),
-        "KMS.ThrottlingException" => super::super::operation::send_message::SendMessageError::KmsThrottled({
+        "KmsThrottled" => super::super::operation::send_message::SendMessageError::KmsThrottled({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -167,7 +167,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::send_message::SendMessageError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::send_message::SendMessageError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -197,7 +197,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::send_message::SendMessageError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::send_message::SendMessageError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -245,8 +245,10 @@
 pub(crate) fn de_send_message(
     _value: &[u8],
     mut builder: super::super::operation::send_message::builders::SendMessageOutputBuilder,
-) -> ::std::result::Result<super::super::operation::send_message::builders::SendMessageOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::send_message::builders::SendMessageOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_send_message_batch.rs`

```diff
--- reference/src/protocol_serde/shape_send_message_batch.rs
+++ generated/src/protocol_serde/shape_send_message_batch.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::send_message_batch::SendMessageBatchOutput, super::super::operation::send_message_batch::SendMessageBatchError> {
+) -> std::result::Result<
+    super::super::operation::send_message_batch::SendMessageBatchOutput,
+    super::super::operation::send_message_batch::SendMessageBatchError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::send_message_batch::SendMessageBatchError::unhandled)?;
@@ -17,13 +20,16 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => super::super::operation::send_message_batch::SendMessageBatchError::BatchEntryIdsNotDistinct({
+        "BatchEntryIdsNotDistinct" => super::super::operation::send_message_batch::SendMessageBatchError::BatchEntryIdsNotDistinct({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::BatchEntryIdsNotDistinctBuilder::default();
-                output = super::super::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(_response_body, output)
-                    .map_err(super::super::operation::send_message_batch::SendMessageBatchError::unhandled)?;
+                output = super::super::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::send_message_batch::SendMessageBatchError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,7 +38,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.BatchRequestTooLong" => super::super::operation::send_message_batch::SendMessageBatchError::BatchRequestTooLong({
+        "BatchRequestTooLong" => super::super::operation::send_message_batch::SendMessageBatchError::BatchRequestTooLong({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -47,7 +53,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.EmptyBatchRequest" => super::super::operation::send_message_batch::SendMessageBatchError::EmptyBatchRequest({
+        "EmptyBatchRequest" => super::super::operation::send_message_batch::SendMessageBatchError::EmptyBatchRequest({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +83,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.InvalidBatchEntryId" => super::super::operation::send_message_batch::SendMessageBatchError::InvalidBatchEntryId({
+        "InvalidBatchEntryId" => super::super::operation::send_message_batch::SendMessageBatchError::InvalidBatchEntryId({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -107,7 +113,7 @@
             }
             tmp
         }),
-        "KMS.AccessDeniedException" => super::super::operation::send_message_batch::SendMessageBatchError::KmsAccessDenied({
+        "KmsAccessDenied" => super::super::operation::send_message_batch::SendMessageBatchError::KmsAccessDenied({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -122,7 +128,7 @@
             }
             tmp
         }),
-        "KMS.DisabledException" => super::super::operation::send_message_batch::SendMessageBatchError::KmsDisabled({
+        "KmsDisabled" => super::super::operation::send_message_batch::SendMessageBatchError::KmsDisabled({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -137,7 +143,7 @@
             }
             tmp
         }),
-        "KMS.InvalidKeyUsageException" => super::super::operation::send_message_batch::SendMessageBatchError::KmsInvalidKeyUsage({
+        "KmsInvalidKeyUsage" => super::super::operation::send_message_batch::SendMessageBatchError::KmsInvalidKeyUsage({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -152,7 +158,7 @@
             }
             tmp
         }),
-        "KMS.InvalidStateException" => super::super::operation::send_message_batch::SendMessageBatchError::KmsInvalidState({
+        "KmsInvalidState" => super::super::operation::send_message_batch::SendMessageBatchError::KmsInvalidState({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -167,7 +173,7 @@
             }
             tmp
         }),
-        "KMS.NotFoundException" => super::super::operation::send_message_batch::SendMessageBatchError::KmsNotFound({
+        "KmsNotFound" => super::super::operation::send_message_batch::SendMessageBatchError::KmsNotFound({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -182,7 +188,7 @@
             }
             tmp
         }),
-        "KMS.OptInRequired" => super::super::operation::send_message_batch::SendMessageBatchError::KmsOptInRequired({
+        "KmsOptInRequired" => super::super::operation::send_message_batch::SendMessageBatchError::KmsOptInRequired({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -197,7 +203,7 @@
             }
             tmp
         }),
-        "KMS.ThrottlingException" => super::super::operation::send_message_batch::SendMessageBatchError::KmsThrottled({
+        "KmsThrottled" => super::super::operation::send_message_batch::SendMessageBatchError::KmsThrottled({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -212,7 +218,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::send_message_batch::SendMessageBatchError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::send_message_batch::SendMessageBatchError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -242,27 +248,25 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
-            super::super::operation::send_message_batch::SendMessageBatchError::TooManyEntriesInBatchRequest({
+        "TooManyEntriesInBatchRequest" => super::super::operation::send_message_batch::SendMessageBatchError::TooManyEntriesInBatchRequest({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder::default();
-                    output = super::super::protocol_serde::shape_too_many_entries_in_batch_request::de_too_many_entries_in_batch_request_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::send_message_batch::SendMessageBatchError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::send_message_batch::SendMessageBatchError::UnsupportedOperation({
+                let mut output = super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder::default();
+                output = super::super::protocol_serde::shape_too_many_entries_in_batch_request::de_too_many_entries_in_batch_request_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::send_message_batch::SendMessageBatchError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
+        "UnsupportedOperation" => super::super::operation::send_message_batch::SendMessageBatchError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -286,7 +290,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::send_message_batch::SendMessageBatchOutput, super::super::operation::send_message_batch::SendMessageBatchError> {
+) -> std::result::Result<
+    super::super::operation::send_message_batch::SendMessageBatchOutput,
+    super::super::operation::send_message_batch::SendMessageBatchError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::send_message_batch::builders::SendMessageBatchOutputBuilder::default();
@@ -336,7 +343,11 @@
                 }
                 "Failed" => {
                     builder = builder.set_failed(
-                        super::super::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_send_message_batch_result_entry_list.rs`

```diff
--- reference/src/protocol_serde/shape_send_message_batch_result_entry_list.rs
+++ generated/src/protocol_serde/shape_send_message_batch_result_entry_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::SendMessageBatchResultEntry>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::SendMessageBatchResultEntry>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_set_queue_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_set_queue_attributes.rs
+++ generated/src/protocol_serde/shape_set_queue_attributes.rs
@@ -95,7 +95,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::set_queue_attributes::SetQueueAttributesError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::set_queue_attributes::SetQueueAttributesError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -125,7 +125,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::set_queue_attributes::SetQueueAttributesError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::set_queue_attributes::SetQueueAttributesError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -170,3 +170,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_set_queue_attributes(
+    _value: &[u8],
+    mut builder: super::super::operation::set_queue_attributes::builders::SetQueueAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::set_queue_attributes::builders::SetQueueAttributesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_start_message_move_task.rs`

```diff
--- reference/src/protocol_serde/shape_start_message_move_task.rs
+++ generated/src/protocol_serde/shape_start_message_move_task.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::start_message_move_task::StartMessageMoveTaskError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::start_message_move_task::StartMessageMoveTaskError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -70,8 +74,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::start_message_move_task::StartMessageMoveTaskError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::start_message_move_task::StartMessageMoveTaskError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,23 +87,21 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => {
-            super::super::operation::start_message_move_task::StartMessageMoveTaskError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::start_message_move_task::StartMessageMoveTaskError::UnsupportedOperation({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
-                    output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
-                        .map_err(super::super::operation::start_message_move_task::StartMessageMoveTaskError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::UnsupportedOperationBuilder::default();
+                output = super::super::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
+                    .map_err(super::super::operation::start_message_move_task::StartMessageMoveTaskError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         _ => super::super::operation::start_message_move_task::StartMessageMoveTaskError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_tag_queue.rs`

```diff
--- reference/src/protocol_serde/shape_tag_queue.rs
+++ generated/src/protocol_serde/shape_tag_queue.rs
@@ -47,7 +47,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::tag_queue::TagQueueError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::tag_queue::TagQueueError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::tag_queue::TagQueueError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::tag_queue::TagQueueError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -119,3 +119,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_tag_queue(
+    _value: &[u8],
+    mut builder: super::super::operation::tag_queue::builders::TagQueueOutputBuilder,
+) -> ::std::result::Result<super::super::operation::tag_queue::builders::TagQueueOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_too_many_entries_in_batch_request.rs`

```diff
--- reference/src/protocol_serde/shape_too_many_entries_in_batch_request.rs
+++ generated/src/protocol_serde/shape_too_many_entries_in_batch_request.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_too_many_entries_in_batch_request_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::TooManyEntriesInBatchRequestBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_untag_queue.rs`

```diff
--- reference/src/protocol_serde/shape_untag_queue.rs
+++ generated/src/protocol_serde/shape_untag_queue.rs
@@ -47,7 +47,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.NonExistentQueue" => super::super::operation::untag_queue::UntagQueueError::QueueDoesNotExist({
+        "QueueDoesNotExist" => super::super::operation::untag_queue::UntagQueueError::QueueDoesNotExist({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        "AWS.SimpleQueueService.UnsupportedOperation" => super::super::operation::untag_queue::UntagQueueError::UnsupportedOperation({
+        "UnsupportedOperation" => super::super::operation::untag_queue::UntagQueueError::UnsupportedOperation({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -119,3 +119,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_untag_queue(
+    _value: &[u8],
+    mut builder: super::super::operation::untag_queue::builders::UntagQueueOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::untag_queue::builders::UntagQueueOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -20,12 +20,7 @@
     response_headers: &::aws_smithy_runtime_api::http::Headers,
     response_body: &[u8],
 ) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut builder = super::json_errors::parse_error_metadata(response_body, response_headers)?;
-    if let Some((error_code, error_type)) = super::aws_query_compatible_errors::parse_aws_query_compatible_error(response_headers) {
-        builder = builder.code(error_code);
-        builder = builder.custom("type", error_type);
-    }
-    Ok(builder)
+    super::json_errors::parse_error_metadata(response_body, response_headers)
 }

 pub(crate) mod shape_add_permission;
@@ -74,8 +69,6 @@

 pub(crate) mod shape_untag_queue;

-pub(crate) mod shape_add_permission_input;
-
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -84,6 +77,8 @@
     }
 }

+pub(crate) mod shape_add_permission_input;
+
 pub(crate) mod shape_batch_entry_ids_not_distinct;

 pub(crate) mod shape_batch_request_too_long;
```

### `src/types/_delete_message_batch_request_entry.rs`

```diff
--- reference/src/types/_delete_message_batch_request_entry.rs
+++ generated/src/types/_delete_message_batch_request_entry.rs
@@ -85,7 +85,9 @@
     /// This method will fail if any of the following fields are not set:
     /// - [`id`](crate::types::builders::DeleteMessageBatchRequestEntryBuilder::id)
     /// - [`receipt_handle`](crate::types::builders::DeleteMessageBatchRequestEntryBuilder::receipt_handle)
-    pub fn build(self) -> ::std::result::Result<super::super::types::DeleteMessageBatchRequestEntry, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::DeleteMessageBatchRequestEntry, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::DeleteMessageBatchRequestEntry {
             id: self.id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_delete_message_batch_result_entry.rs`

```diff
--- reference/src/types/_delete_message_batch_result_entry.rs
+++ generated/src/types/_delete_message_batch_result_entry.rs
@@ -46,7 +46,9 @@
     /// Consumes the builder and constructs a [`DeleteMessageBatchResultEntry`](crate::types::DeleteMessageBatchResultEntry).
     /// This method will fail if any of the following fields are not set:
     /// - [`id`](crate::types::builders::DeleteMessageBatchResultEntryBuilder::id)
-    pub fn build(self) -> ::std::result::Result<super::super::types::DeleteMessageBatchResultEntry, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::DeleteMessageBatchResultEntry, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::DeleteMessageBatchResultEntry {
             id: self.id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_message.rs`

```diff
--- reference/src/types/_message.rs
+++ generated/src/types/_message.rs
@@ -71,7 +71,9 @@
     /// <p><code>SequenceNumber</code></p></li>
     /// </ul>
     /// <p><code>ApproximateFirstReceiveTimestamp</code> and <code>SentTimestamp</code> are each returned as an integer representing the <a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<super::super::types::MessageSystemAttributeName, ::std::string::String>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<super::super::types::MessageSystemAttributeName, ::std::string::String>> {
         self.attributes.as_ref()
     }
     /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
@@ -102,7 +104,8 @@
     pub(crate) body: ::std::option::Option<::std::string::String>,
     pub(crate) attributes: ::std::option::Option<::std::collections::HashMap<super::super::types::MessageSystemAttributeName, ::std::string::String>>,
     pub(crate) md5_of_message_attributes: ::std::option::Option<::std::string::String>,
-    pub(crate) message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::MessageAttributeValue>>,
+    pub(crate) message_attributes:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::MessageAttributeValue>>,
 }
 impl MessageBuilder {
     /// <p>A unique identifier for the message. A <code>MessageId</code>is considered unique across all Amazon Web Services accounts for an extended period of time.</p>
```

### `src/types/_message_system_attribute_name_for_sends.rs`

```diff
--- reference/src/types/_message_system_attribute_name_for_sends.rs
+++ generated/src/types/_message_system_attribute_name_for_sends.rs
@@ -51,7 +51,9 @@
     fn from(s: &str) -> Self {
         match s {
             "AWSTraceHeader" => MessageSystemAttributeNameForSends::AwsTraceHeader,
-            other => MessageSystemAttributeNameForSends::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => {
+                MessageSystemAttributeNameForSends::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()))
+            }
         }
     }
 }
```

### Missing reference files

- `src/aws_query_compatible_errors.rs`
- `src/long_polling.rs`
