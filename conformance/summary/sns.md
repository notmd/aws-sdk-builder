# AWS SDK Conformance Report: sns

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sns
**Progress:** `445/445` files compared · `414` matched · `31` mismatches · `0` missing · `0` extra · `93.03%` match (100.00% means fully matched)

### `src/client/create_topic.rs`

```diff
--- reference/src/client/create_topic.rs
+++ generated/src/client/create_topic.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`name(impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_name):<br>required: **true**<br><p>The name of the topic you want to create.</p> <p>Constraints: Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long.</p> <p>For a FIFO (first-in-first-out) topic, the name must end with the <code>.fifo</code> suffix.</p><br>
-    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_attributes):<br>required: **false**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists names, descriptions, and values of the special request parameters that the <code>CreateTopic</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul><note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>    <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>   </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul><note>  <p>The <endpoint>    SuccessFeedbackRoleArn and     <endpoint>     FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The      <endpoint>      SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the       <endpoint>       FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.      </endpoint>     </endpoint>    </endpoint>   </endpoint></p> </note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li> </ul> <ul>  <li>   <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>   <ul>    <li>     <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>    <li>     <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>   </ul></li> </ul><br>
+    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_attributes):<br>required: **false**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists names, descriptions, and values of the special request parameters that the <code>CreateTopic</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul> <note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>    <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>   </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul> <note>  <p>The <endpoint>    SuccessFeedbackRoleArn and     <endpoint>     FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The      <endpoint>      SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the       <endpoint>       FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.       </endpoint>       </endpoint>      </endpoint>     </endpoint></p> </note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li> </ul> <ul>  <li>   <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>   <ul>    <li>     <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>    <li>     <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>   </ul></li> </ul><br>
     ///   - [`tags(Tag)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_tags):<br>required: **false**<br><p>The list of tags to add to a new topic.</p><note>  <p>To be able to tag a topic on creation, you must have the <code>sns:CreateTopic</code> and <code>sns:TagResource</code> permissions.</p> </note><br>
     ///   - [`data_protection_policy(impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::data_protection_policy) / [`set_data_protection_policy(Option<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_data_protection_policy):<br>required: **false**<br><p>The body of the policy document you want to use for this topic.</p> <p>You can only add one policy per topic.</p> <p>The policy must be in JSON string format.</p> <p>Length Constraints: Maximum length of 30,720.</p><br>
     /// - On success, responds with [`CreateTopicOutput`](crate::operation::create_topic::CreateTopicOutput) with field(s):
```

### `src/client/set_sms_attributes.rs`

```diff
--- reference/src/client/set_sms_attributes.rs
+++ generated/src/client/set_sms_attributes.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`SetSMSAttributes`](crate::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder::set_attributes):<br>required: **true**<br><p>The default settings for sending SMS messages from your Amazon Web Services account. You can set values for the following attribute names:</p> <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>  <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p> </important> <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p> <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p> <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p> <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p> <p><code>DefaultSMSType</code> – The type of SMS message that you will send by default. You can assign the following values:</p> <ul>  <li>   <p><code>Promotional</code> – (Default) Noncritical messages, such as marketing messages. Amazon SNS optimizes the message delivery to incur the lowest cost.</p></li>  <li>   <p><code>Transactional</code> – Critical messages that support customer transactions, such as one-time passcodes for multi-factor authentication. Amazon SNS optimizes the message delivery to achieve the highest reliability.</p></li> </ul> <p><code>UsageReportS3Bucket</code> – The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS. Each day, Amazon SNS will deliver a usage report as a CSV file to the bucket. The report includes the following information for each SMS message that was successfully delivered by your Amazon Web Services account:</p> <ul>  <li>   <p>Time that the message was published (in UTC)</p></li>  <li>   <p>Message ID</p></li>  <li>   <p>Destination phone number</p></li>  <li>   <p>Message type</p></li>  <li>   <p>Delivery status</p></li>  <li>   <p>Message price (in USD)</p></li>  <li>   <p>Part number (a message is split into multiple parts if it is too long for a single message)</p></li>  <li>   <p>Total number of parts</p></li> </ul> <p>To receive the report, the bucket must have a policy that allows the Amazon SNS service principal to perform the <code>s3:PutObject</code> and <code>s3:GetBucketLocation</code> actions.</p> <p>For an example bucket policy and usage report, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_stats.html">Monitoring SMS Activity</a> in the <i>Amazon SNS Developer Guide</i>.</p><br>
+    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder::set_attributes):<br>required: **true**<br><p>The default settings for sending SMS messages from your Amazon Web Services account. You can set values for the following attribute names:</p> <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>  <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p> </important> <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p> <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p> <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p> <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p> <p><code>DefaultSMSType</code> – The type of SMS message that you will send by default. You can assign the following values:</p> <ul>  <li>   <p><code>Promotional</code> – (Default) Noncritical messages, such as marketing messages. Amazon SNS optimizes the message delivery to incur the lowest cost.</p></li>  <li>   <p><code>Transactional</code> – Critical messages that support customer transactions, such as one-time passcodes for multi-factor authentication. Amazon SNS optimizes the message delivery to achieve the highest reliability.</p></li> </ul> <p><code>UsageReportS3Bucket</code> – The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS. Each day, Amazon SNS will deliver a usage report as a CSV file to the bucket. The report includes the following information for each SMS message that was successfully delivered by your Amazon Web Services account:</p> <ul>  <li>   <p>Time that the message was published (in UTC)</p></li>  <li>   <p>Message ID</p></li>  <li>   <p>Destination phone number</p></li>  <li>   <p>Message type</p></li>  <li>   <p>Delivery status</p></li>  <li>   <p>Message price (in USD)</p></li>  <li>   <p>Part number (a message is split into multiple parts if it is too long for a single message)</p></li>  <li>   <p>Total number of parts</p></li> </ul> <p>To receive the report, the bucket must have a policy that allows the Amazon SNS service principal to perform the <code>s3:PutObject</code> and <code>s3:GetBucketLocation</code> actions.</p> <p>For an example bucket policy and usage report, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_stats.html">Monitoring SMS Activity</a> in the <i>Amazon SNS Developer Guide</i>.</p><br>
     /// - On success, responds with [`SetSmsAttributesOutput`](crate::operation::set_sms_attributes::SetSmsAttributesOutput)
     /// - On failure, responds with [`SdkError<SetSMSAttributesError>`](crate::operation::set_sms_attributes::SetSMSAttributesError)
     pub fn set_sms_attributes(&self) -> super::super::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder {
```

### `src/client/set_topic_attributes.rs`

```diff
--- reference/src/client/set_topic_attributes.rs
+++ generated/src/client/set_topic_attributes.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`topic_arn(impl Into<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::topic_arn) / [`set_topic_arn(Option<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::set_topic_arn):<br>required: **true**<br><p>The ARN of the topic to modify.</p><br>
-    ///   - [`attribute_name(impl Into<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::attribute_name) / [`set_attribute_name(Option<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::set_attribute_name):<br>required: **true**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul><note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>    <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>   </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul><note>  <p>The <endpoint>    SuccessFeedbackRoleArn and     <endpoint>     FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The      <endpoint>      SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the       <endpoint>       FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.      </endpoint>     </endpoint>    </endpoint>   </endpoint></p> </note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>  <li>   <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS. By default, <code>SignatureVersion</code> is set to <code>1</code>.</p></li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li> </ul> <ul>  <li>   <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>   <ul>    <li>     <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>    <li>     <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>   </ul></li> </ul><br>
+    ///   - [`attribute_name(impl Into<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::attribute_name) / [`set_attribute_name(Option<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::set_attribute_name):<br>required: **true**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul> <note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>    <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>   </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul> <note>  <p>The <endpoint>    SuccessFeedbackRoleArn and     <endpoint>     FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The      <endpoint>      SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the       <endpoint>       FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.       </endpoint>       </endpoint>      </endpoint>     </endpoint></p> </note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>  <li>   <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS. By default, <code>SignatureVersion</code> is set to <code>1</code>.</p></li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li> </ul> <ul>  <li>   <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>   <ul>    <li>     <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>    <li>     <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>   </ul></li> </ul><br>
     ///   - [`attribute_value(impl Into<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::attribute_value) / [`set_attribute_value(Option<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::set_attribute_value):<br>required: **false**<br><p>The new value for the attribute.</p><br>
     /// - On success, responds with [`SetTopicAttributesOutput`](crate::operation::set_topic_attributes::SetTopicAttributesOutput)
     /// - On failure, responds with [`SdkError<SetTopicAttributesError>`](crate::operation::set_topic_attributes::SetTopicAttributesError)
```

### `src/operation/create_sms_sandbox_phone_number/builders.rs`

```diff
--- reference/src/operation/create_sms_sandbox_phone_number/builders.rs
+++ generated/src/operation/create_sms_sandbox_phone_number/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+            super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `CreateSMSSandboxPhoneNumber`.
+/// Fluent builder constructing a request to `CreateSmsSandboxPhoneNumber`.
 ///
 /// <p>Adds a destination phone number to an Amazon Web Services account in the SMS sandbox and sends a one-time password (OTP) to that phone number.</p>
 /// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct CreateSMSSandboxPhoneNumberFluentBuilder {
+pub struct CreateSmsSandboxPhoneNumberFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-        super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
-    > for CreateSMSSandboxPhoneNumberFluentBuilder
+        super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
+    > for CreateSmsSandboxPhoneNumberFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-            super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+            super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl CreateSMSSandboxPhoneNumberFluentBuilder {
-    /// Creates a new `CreateSMSSandboxPhoneNumberFluentBuilder`.
+impl CreateSmsSandboxPhoneNumberFluentBuilder {
+    /// Creates a new `CreateSmsSandboxPhoneNumberFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the CreateSMSSandboxPhoneNumber as a reference.
+    /// Access the CreateSmsSandboxPhoneNumber as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+            super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumber::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumber::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-        super::super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+        super::super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/create_sms_sandbox_phone_number.rs`

```diff
--- reference/src/operation/create_sms_sandbox_phone_number.rs
+++ generated/src/operation/create_sms_sandbox_phone_number.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `CreateSMSSandboxPhoneNumber`.
+/// Orchestration and serialization glue logic for `CreateSmsSandboxPhoneNumber`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct CreateSMSSandboxPhoneNumber;
-impl CreateSMSSandboxPhoneNumber {
-    /// Creates a new `CreateSMSSandboxPhoneNumber`
+pub struct CreateSmsSandboxPhoneNumber;
+impl CreateSmsSandboxPhoneNumber {
+    /// Creates a new `CreateSmsSandboxPhoneNumber`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateSMSSandboxPhoneNumber {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateSmsSandboxPhoneNumber {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateSMSSandboxPhoneNumber");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            CreateSMSSandboxPhoneNumberRequestSerializer,
+            CreateSmsSandboxPhoneNumberRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateSMSSandboxPhoneNumberResponseDeserializer,
+            CreateSmsSandboxPhoneNumberResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -130,7 +130,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateSMSSandboxPhoneNumberEndpointParamsInterceptor,
+                CreateSmsSandboxPhoneNumberEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
@@ -219,12 +219,12 @@
     }
 }
 #[derive(Debug)]
-struct CreateSMSSandboxPhoneNumberEndpointParamsInterceptor;
+struct CreateSmsSandboxPhoneNumberEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSMSSandboxPhoneNumberEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSmsSandboxPhoneNumberEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "CreateSMSSandboxPhoneNumberEndpointParamsInterceptor"
+        "CreateSmsSandboxPhoneNumberEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/delete_sms_sandbox_phone_number/builders.rs`

```diff
--- reference/src/operation/delete_sms_sandbox_phone_number/builders.rs
+++ generated/src/operation/delete_sms_sandbox_phone_number/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+            super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `DeleteSMSSandboxPhoneNumber`.
+/// Fluent builder constructing a request to `DeleteSmsSandboxPhoneNumber`.
 ///
 /// <p>Deletes an Amazon Web Services account's verified or pending phone number from the SMS sandbox.</p>
 /// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct DeleteSMSSandboxPhoneNumberFluentBuilder {
+pub struct DeleteSmsSandboxPhoneNumberFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::delete_sms_sandbox_phone_number::builders::DeleteSmsSandboxPhoneNumberInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-        super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
-    > for DeleteSMSSandboxPhoneNumberFluentBuilder
+        super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
+    > for DeleteSmsSandboxPhoneNumberFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-            super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+            super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeleteSMSSandboxPhoneNumberFluentBuilder {
-    /// Creates a new `DeleteSMSSandboxPhoneNumberFluentBuilder`.
+impl DeleteSmsSandboxPhoneNumberFluentBuilder {
+    /// Creates a new `DeleteSmsSandboxPhoneNumberFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeleteSMSSandboxPhoneNumber as a reference.
+    /// Access the DeleteSmsSandboxPhoneNumber as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::delete_sms_sandbox_phone_number::builders::DeleteSmsSandboxPhoneNumberInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+            super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumber::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumber::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-        super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+        super::super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/delete_sms_sandbox_phone_number.rs`

```diff
--- reference/src/operation/delete_sms_sandbox_phone_number.rs
+++ generated/src/operation/delete_sms_sandbox_phone_number.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `DeleteSMSSandboxPhoneNumber`.
+/// Orchestration and serialization glue logic for `DeleteSmsSandboxPhoneNumber`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct DeleteSMSSandboxPhoneNumber;
-impl DeleteSMSSandboxPhoneNumber {
-    /// Creates a new `DeleteSMSSandboxPhoneNumber`
+pub struct DeleteSmsSandboxPhoneNumber;
+impl DeleteSmsSandboxPhoneNumber {
+    /// Creates a new `DeleteSmsSandboxPhoneNumber`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteSMSSandboxPhoneNumber {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteSmsSandboxPhoneNumber {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteSMSSandboxPhoneNumber");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            DeleteSMSSandboxPhoneNumberRequestSerializer,
+            DeleteSmsSandboxPhoneNumberRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DeleteSMSSandboxPhoneNumberResponseDeserializer,
+            DeleteSmsSandboxPhoneNumberResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -130,7 +130,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteSMSSandboxPhoneNumberEndpointParamsInterceptor,
+                DeleteSmsSandboxPhoneNumberEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
@@ -219,12 +219,12 @@
     }
 }
 #[derive(Debug)]
-struct DeleteSMSSandboxPhoneNumberEndpointParamsInterceptor;
+struct DeleteSmsSandboxPhoneNumberEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSMSSandboxPhoneNumberEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSmsSandboxPhoneNumberEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteSMSSandboxPhoneNumberEndpointParamsInterceptor"
+        "DeleteSmsSandboxPhoneNumberEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_sms_attributes/builders.rs`

```diff
--- reference/src/operation/get_sms_attributes/builders.rs
+++ generated/src/operation/get_sms_attributes/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_sms_attributes::GetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_sms_attributes::GetSMSAttributesError,
+            super::super::super::operation::get_sms_attributes::GetSmsAttributesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetSMSAttributes`.
+/// Fluent builder constructing a request to `GetSmsAttributes`.
 ///
 /// <p>Returns the settings for sending SMS messages from your Amazon Web Services account.</p>
 /// <p>These settings are set with the <code>SetSMSAttributes</code> action.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetSMSAttributesFluentBuilder {
+pub struct GetSmsAttributesFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_sms_attributes::builders::GetSmsAttributesInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_sms_attributes::GetSmsAttributesOutput,
-        super::super::super::operation::get_sms_attributes::GetSMSAttributesError,
-    > for GetSMSAttributesFluentBuilder
+        super::super::super::operation::get_sms_attributes::GetSmsAttributesError,
+    > for GetSmsAttributesFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_sms_attributes::GetSmsAttributesOutput,
-            super::super::super::operation::get_sms_attributes::GetSMSAttributesError,
+            super::super::super::operation::get_sms_attributes::GetSmsAttributesError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetSMSAttributesFluentBuilder {
-    /// Creates a new `GetSMSAttributesFluentBuilder`.
+impl GetSmsAttributesFluentBuilder {
+    /// Creates a new `GetSmsAttributesFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetSMSAttributes as a reference.
+    /// Access the GetSmsAttributes as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_sms_attributes::builders::GetSmsAttributesInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_sms_attributes::GetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_sms_attributes::GetSMSAttributesError,
+            super::super::super::operation::get_sms_attributes::GetSmsAttributesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_sms_attributes::GetSMSAttributes::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_sms_attributes::GetSmsAttributes::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_sms_attributes::GetSMSAttributes::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_sms_attributes::GetSmsAttributes::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_sms_attributes::GetSmsAttributesOutput,
-        super::super::super::operation::get_sms_attributes::GetSMSAttributesError,
+        super::super::super::operation::get_sms_attributes::GetSmsAttributesError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_sms_attributes.rs`

```diff
--- reference/src/operation/get_sms_attributes.rs
+++ generated/src/operation/get_sms_attributes.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetSMSAttributes`.
+/// Orchestration and serialization glue logic for `GetSmsAttributes`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetSMSAttributes;
-impl GetSMSAttributes {
-    /// Creates a new `GetSMSAttributes`
+pub struct GetSmsAttributes;
+impl GetSmsAttributes {
+    /// Creates a new `GetSmsAttributes`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSMSAttributes {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSmsAttributes {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetSMSAttributes");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetSMSAttributesRequestSerializer,
+            GetSmsAttributesRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetSMSAttributesResponseDeserializer,
+            GetSmsAttributesResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,7 +127,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSMSAttributesEndpointParamsInterceptor,
+                GetSmsAttributesEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_sms_attributes::GetSMSAttributesError,
@@ -216,12 +216,12 @@
     }
 }
 #[derive(Debug)]
-struct GetSMSAttributesEndpointParamsInterceptor;
+struct GetSmsAttributesEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSMSAttributesEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSmsAttributesEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetSMSAttributesEndpointParamsInterceptor"
+        "GetSmsAttributesEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_sms_sandbox_account_status/builders.rs`

```diff
--- reference/src/operation/get_sms_sandbox_account_status/builders.rs
+++ generated/src/operation/get_sms_sandbox_account_status/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+            super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetSMSSandboxAccountStatus`.
+/// Fluent builder constructing a request to `GetSmsSandboxAccountStatus`.
 ///
 /// <p>Retrieves the SMS sandbox status for the calling Amazon Web Services account in the target Amazon Web Services Region.</p>
 /// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetSMSSandboxAccountStatusFluentBuilder {
+pub struct GetSmsSandboxAccountStatusFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-        super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
-    > for GetSMSSandboxAccountStatusFluentBuilder
+        super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
+    > for GetSmsSandboxAccountStatusFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-            super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+            super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetSMSSandboxAccountStatusFluentBuilder {
-    /// Creates a new `GetSMSSandboxAccountStatusFluentBuilder`.
+impl GetSmsSandboxAccountStatusFluentBuilder {
+    /// Creates a new `GetSmsSandboxAccountStatusFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetSMSSandboxAccountStatus as a reference.
+    /// Access the GetSmsSandboxAccountStatus as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+            super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatus::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatus::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatus::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatus::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-        super::super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+        super::super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_sms_sandbox_account_status.rs`

```diff
--- reference/src/operation/get_sms_sandbox_account_status.rs
+++ generated/src/operation/get_sms_sandbox_account_status.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetSMSSandboxAccountStatus`.
+/// Orchestration and serialization glue logic for `GetSmsSandboxAccountStatus`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetSMSSandboxAccountStatus;
-impl GetSMSSandboxAccountStatus {
-    /// Creates a new `GetSMSSandboxAccountStatus`
+pub struct GetSmsSandboxAccountStatus;
+impl GetSmsSandboxAccountStatus {
+    /// Creates a new `GetSmsSandboxAccountStatus`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSMSSandboxAccountStatus {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSmsSandboxAccountStatus {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetSMSSandboxAccountStatus");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetSMSSandboxAccountStatusRequestSerializer,
+            GetSmsSandboxAccountStatusRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetSMSSandboxAccountStatusResponseDeserializer,
+            GetSmsSandboxAccountStatusResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -130,7 +130,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSMSSandboxAccountStatusEndpointParamsInterceptor,
+                GetSmsSandboxAccountStatusEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
@@ -211,17 +211,20 @@
         let body = ::aws_smithy_types::body::SdkBody::from(
             super::super::protocol_serde::shape_get_sms_sandbox_account_status_input::ser_get_sms_sandbox_account_status_input_input_input(&input)?,
         );
-
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
 #[derive(Debug)]
-struct GetSMSSandboxAccountStatusEndpointParamsInterceptor;
+struct GetSmsSandboxAccountStatusEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSMSSandboxAccountStatusEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSmsSandboxAccountStatusEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetSMSSandboxAccountStatusEndpointParamsInterceptor"
+        "GetSmsSandboxAccountStatusEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_phone_numbers_opted_out/_list_phone_numbers_opted_out_output.rs`

```diff
--- reference/src/operation/list_phone_numbers_opted_out/_list_phone_numbers_opted_out_output.rs
+++ generated/src/operation/list_phone_numbers_opted_out/_list_phone_numbers_opted_out_output.rs
@@ -2,7 +2,7 @@

 /// <p>The response from the <code>ListPhoneNumbersOptedOut</code> action.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ListPhoneNumbersOptedOutOutput {
     /// <p>A list of phone numbers that are opted out of receiving SMS messages. The list is paginated, and each page can contain up to 100 phone numbers.</p>
     pub phone_numbers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -22,6 +22,15 @@
         self.next_token.as_deref()
     }
 }
+impl ::std::fmt::Debug for ListPhoneNumbersOptedOutOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ListPhoneNumbersOptedOutOutput");
+        formatter.field("phone_numbers", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_token", &self.next_token);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for ListPhoneNumbersOptedOutOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -35,7 +44,7 @@
 }

 /// A builder for [`ListPhoneNumbersOptedOutOutput`](crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ListPhoneNumbersOptedOutOutputBuilder {
     pub(crate) phone_numbers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -95,3 +104,12 @@
         }
     }
 }
+impl ::std::fmt::Debug for ListPhoneNumbersOptedOutOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ListPhoneNumbersOptedOutOutputBuilder");
+        formatter.field("phone_numbers", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_token", &self.next_token);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
```

### `src/operation/list_sms_sandbox_phone_numbers/builders.rs`

```diff
--- reference/src/operation/list_sms_sandbox_phone_numbers/builders.rs
+++ generated/src/operation/list_sms_sandbox_phone_numbers/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+            super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListSMSSandboxPhoneNumbers`.
+/// Fluent builder constructing a request to `ListSmsSandboxPhoneNumbers`.
 ///
 /// <p>Lists the calling Amazon Web Services account's current verified and pending destination phone numbers in the SMS sandbox.</p>
 /// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListSMSSandboxPhoneNumbersFluentBuilder {
+pub struct ListSmsSandboxPhoneNumbersFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-        super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
-    > for ListSMSSandboxPhoneNumbersFluentBuilder
+        super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
+    > for ListSmsSandboxPhoneNumbersFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-            super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+            super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListSMSSandboxPhoneNumbersFluentBuilder {
-    /// Creates a new `ListSMSSandboxPhoneNumbersFluentBuilder`.
+impl ListSmsSandboxPhoneNumbersFluentBuilder {
+    /// Creates a new `ListSmsSandboxPhoneNumbersFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListSMSSandboxPhoneNumbers as a reference.
+    /// Access the ListSmsSandboxPhoneNumbers as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+            super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbers::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbers::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbers::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbers::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-        super::super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+        super::super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_sms_sandbox_phone_numbers.rs`

```diff
--- reference/src/operation/list_sms_sandbox_phone_numbers.rs
+++ generated/src/operation/list_sms_sandbox_phone_numbers.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListSMSSandboxPhoneNumbers`.
+/// Orchestration and serialization glue logic for `ListSmsSandboxPhoneNumbers`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListSMSSandboxPhoneNumbers;
-impl ListSMSSandboxPhoneNumbers {
-    /// Creates a new `ListSMSSandboxPhoneNumbers`
+pub struct ListSmsSandboxPhoneNumbers;
+impl ListSmsSandboxPhoneNumbers {
+    /// Creates a new `ListSmsSandboxPhoneNumbers`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSMSSandboxPhoneNumbers {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSmsSandboxPhoneNumbers {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListSMSSandboxPhoneNumbers");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListSMSSandboxPhoneNumbersRequestSerializer,
+            ListSmsSandboxPhoneNumbersRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListSMSSandboxPhoneNumbersResponseDeserializer,
+            ListSmsSandboxPhoneNumbersResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -128,13 +128,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListSMSSandboxPhoneNumbers")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSMSSandboxPhoneNumbersTelemetryInputCaptureInterceptor,
+                ListSmsSandboxPhoneNumbersTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSMSSandboxPhoneNumbersEndpointParamsInterceptor,
+                ListSmsSandboxPhoneNumbersEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
@@ -151,12 +151,12 @@
 }

 #[derive(Debug)]
-struct ListSMSSandboxPhoneNumbersTelemetryInputCaptureInterceptor;
+struct ListSmsSandboxPhoneNumbersTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSMSSandboxPhoneNumbersTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSmsSandboxPhoneNumbersTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListSMSSandboxPhoneNumbersTelemetryInputCaptureInterceptor"
+        "ListSmsSandboxPhoneNumbersTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -266,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct ListSMSSandboxPhoneNumbersEndpointParamsInterceptor;
+struct ListSmsSandboxPhoneNumbersEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSMSSandboxPhoneNumbersEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSmsSandboxPhoneNumbersEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListSMSSandboxPhoneNumbersEndpointParamsInterceptor"
+        "ListSmsSandboxPhoneNumbersEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/set_sms_attributes/_set_sms_attributes_input.rs`

```diff
--- reference/src/operation/set_sms_attributes/_set_sms_attributes_input.rs
+++ generated/src/operation/set_sms_attributes/_set_sms_attributes_input.rs
@@ -8,7 +8,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
@@ -47,7 +47,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
@@ -105,7 +105,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
@@ -147,7 +147,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
@@ -187,7 +187,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
```

### `src/operation/set_sms_attributes/builders.rs`

```diff
--- reference/src/operation/set_sms_attributes/builders.rs
+++ generated/src/operation/set_sms_attributes/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::set_sms_attributes::SetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::set_sms_attributes::SetSMSAttributesError,
+            super::super::super::operation::set_sms_attributes::SetSmsAttributesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `SetSMSAttributes`.
+/// Fluent builder constructing a request to `SetSmsAttributes`.
 ///
 /// <p>Use this request to set the default settings for sending SMS messages and receiving daily SMS usage reports.</p>
 /// <p>You can override some of these settings for a single message when you use the <code>Publish</code> action with the <code>MessageAttributes.entry.N</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Publishing to a mobile phone</a> in the <i>Amazon SNS Developer Guide</i>.</p><note>
@@ -27,7 +27,7 @@
 /// <p>To use this operation, you must grant the Amazon SNS service principal (<code>sns.amazonaws.com</code>) permission to perform the <code>s3:ListBucket</code> action.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct SetSMSAttributesFluentBuilder {
+pub struct SetSmsAttributesFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::set_sms_attributes::builders::SetSmsAttributesInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -35,8 +35,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::set_sms_attributes::SetSmsAttributesOutput,
-        super::super::super::operation::set_sms_attributes::SetSMSAttributesError,
-    > for SetSMSAttributesFluentBuilder
+        super::super::super::operation::set_sms_attributes::SetSmsAttributesError,
+    > for SetSmsAttributesFluentBuilder
 {
     fn send(
         self,
@@ -44,14 +44,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::set_sms_attributes::SetSmsAttributesOutput,
-            super::super::super::operation::set_sms_attributes::SetSMSAttributesError,
+            super::super::super::operation::set_sms_attributes::SetSmsAttributesError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl SetSMSAttributesFluentBuilder {
-    /// Creates a new `SetSMSAttributesFluentBuilder`.
+impl SetSmsAttributesFluentBuilder {
+    /// Creates a new `SetSmsAttributesFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -59,7 +59,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the SetSMSAttributes as a reference.
+    /// Access the SetSmsAttributes as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::set_sms_attributes::builders::SetSmsAttributesInputBuilder {
         &self.inner
     }
@@ -76,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::set_sms_attributes::SetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::set_sms_attributes::SetSMSAttributesError,
+            super::super::super::operation::set_sms_attributes::SetSmsAttributesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,12 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::set_sms_attributes::SetSMSAttributes::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::set_sms_attributes::SetSmsAttributes::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::set_sms_attributes::SetSMSAttributes::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::set_sms_attributes::SetSmsAttributes::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -97,7 +97,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::set_sms_attributes::SetSmsAttributesOutput,
-        super::super::super::operation::set_sms_attributes::SetSMSAttributesError,
+        super::super::super::operation::set_sms_attributes::SetSmsAttributesError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
@@ -120,7 +120,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
@@ -160,7 +160,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
@@ -200,7 +200,7 @@
     /// <p><code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p><important>
     /// <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p>
     /// </important>
-    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
+    /// <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p>
     /// <p><code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p>
     /// <p><code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p>
     /// <p><code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p>
```

### `src/operation/set_sms_attributes.rs`

```diff
--- reference/src/operation/set_sms_attributes.rs
+++ generated/src/operation/set_sms_attributes.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `SetSMSAttributes`.
+/// Orchestration and serialization glue logic for `SetSmsAttributes`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct SetSMSAttributes;
-impl SetSMSAttributes {
-    /// Creates a new `SetSMSAttributes`
+pub struct SetSmsAttributes;
+impl SetSmsAttributes {
+    /// Creates a new `SetSmsAttributes`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SetSMSAttributes {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SetSmsAttributes {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("SetSMSAttributes");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            SetSMSAttributesRequestSerializer,
+            SetSmsAttributesRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            SetSMSAttributesResponseDeserializer,
+            SetSmsAttributesResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,7 +127,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetSMSAttributesEndpointParamsInterceptor,
+                SetSmsAttributesEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::set_sms_attributes::SetSMSAttributesError,
@@ -216,12 +216,12 @@
     }
 }
 #[derive(Debug)]
-struct SetSMSAttributesEndpointParamsInterceptor;
+struct SetSmsAttributesEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetSMSAttributesEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetSmsAttributesEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "SetSMSAttributesEndpointParamsInterceptor"
+        "SetSmsAttributesEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/subscribe/_subscribe_input.rs`

```diff
--- reference/src/operation/subscribe/_subscribe_input.rs
+++ generated/src/operation/subscribe/_subscribe_input.rs
@@ -595,7 +595,7 @@
             protocol: self.protocol,
             endpoint: self.endpoint,
             attributes: self.attributes,
-            return_subscription_arn: self.return_subscription_arn,
+            return_subscription_arn: self.return_subscription_arn.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/verify_sms_sandbox_phone_number/builders.rs`

```diff
--- reference/src/operation/verify_sms_sandbox_phone_number/builders.rs
+++ generated/src/operation/verify_sms_sandbox_phone_number/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+            super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `VerifySMSSandboxPhoneNumber`.
+/// Fluent builder constructing a request to `VerifySmsSandboxPhoneNumber`.
 ///
 /// <p>Verifies a destination phone number with a one-time password (OTP) for the calling Amazon Web Services account.</p>
 /// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct VerifySMSSandboxPhoneNumberFluentBuilder {
+pub struct VerifySmsSandboxPhoneNumberFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::verify_sms_sandbox_phone_number::builders::VerifySmsSandboxPhoneNumberInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-        super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
-    > for VerifySMSSandboxPhoneNumberFluentBuilder
+        super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
+    > for VerifySmsSandboxPhoneNumberFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-            super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+            super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl VerifySMSSandboxPhoneNumberFluentBuilder {
-    /// Creates a new `VerifySMSSandboxPhoneNumberFluentBuilder`.
+impl VerifySmsSandboxPhoneNumberFluentBuilder {
+    /// Creates a new `VerifySmsSandboxPhoneNumberFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the VerifySMSSandboxPhoneNumber as a reference.
+    /// Access the VerifySmsSandboxPhoneNumber as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::verify_sms_sandbox_phone_number::builders::VerifySmsSandboxPhoneNumberInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+            super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumber::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumber::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-        super::super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+        super::super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/verify_sms_sandbox_phone_number.rs`

```diff
--- reference/src/operation/verify_sms_sandbox_phone_number.rs
+++ generated/src/operation/verify_sms_sandbox_phone_number.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `VerifySMSSandboxPhoneNumber`.
+/// Orchestration and serialization glue logic for `VerifySmsSandboxPhoneNumber`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct VerifySMSSandboxPhoneNumber;
-impl VerifySMSSandboxPhoneNumber {
-    /// Creates a new `VerifySMSSandboxPhoneNumber`
+pub struct VerifySmsSandboxPhoneNumber;
+impl VerifySmsSandboxPhoneNumber {
+    /// Creates a new `VerifySmsSandboxPhoneNumber`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for VerifySMSSandboxPhoneNumber {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for VerifySmsSandboxPhoneNumber {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("VerifySMSSandboxPhoneNumber");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            VerifySMSSandboxPhoneNumberRequestSerializer,
+            VerifySmsSandboxPhoneNumberRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            VerifySMSSandboxPhoneNumberResponseDeserializer,
+            VerifySmsSandboxPhoneNumberResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("VerifySMSSandboxPhoneNumber")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                VerifySMSSandboxPhoneNumberTelemetryInputCaptureInterceptor,
+                VerifySmsSandboxPhoneNumberTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                VerifySMSSandboxPhoneNumberEndpointParamsInterceptor,
+                VerifySmsSandboxPhoneNumberEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct VerifySMSSandboxPhoneNumberTelemetryInputCaptureInterceptor;
+struct VerifySmsSandboxPhoneNumberTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for VerifySMSSandboxPhoneNumberTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for VerifySmsSandboxPhoneNumberTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "VerifySMSSandboxPhoneNumberTelemetryInputCaptureInterceptor"
+        "VerifySmsSandboxPhoneNumberTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -265,12 +265,12 @@
     }
 }
 #[derive(Debug)]
-struct VerifySMSSandboxPhoneNumberEndpointParamsInterceptor;
+struct VerifySmsSandboxPhoneNumberEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for VerifySMSSandboxPhoneNumberEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for VerifySmsSandboxPhoneNumberEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "VerifySMSSandboxPhoneNumberEndpointParamsInterceptor"
+        "VerifySmsSandboxPhoneNumberEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/protocol_serde/shape_create_sms_sandbox_phone_number.rs`

```diff
--- reference/src/protocol_serde/shape_create_sms_sandbox_phone_number.rs
+++ generated/src/protocol_serde/shape_create_sms_sandbox_phone_number.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-    super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+    super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+        .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled(generic)),
+        None => return Err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::InternalErrorException({
+        "InternalError" => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "InvalidParameter" => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::InvalidParameterException({
+        "InvalidParameter" => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,13 +65,13 @@
             }
             tmp
         }),
-        "OptedOut" => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::OptedOutException({
+        "OptedOut" => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::OptedOutException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::OptedOutExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_opted_out_exception::de_opted_out_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,13 +80,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::ThrottledException({
+        "Throttled" => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -95,13 +95,13 @@
             }
             tmp
         }),
-        "UserError" => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::UserErrorException({
+        "UserError" => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::UserErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UserErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_user_error_exception::de_user_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -110,7 +110,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError::generic(generic),
+        _ => super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError::generic(generic),
     })
 }

@@ -121,7 +121,7 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-    super::super::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+    super::super::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
 > {
     Ok({
         #[allow(unused_mut)]
```

### `src/protocol_serde/shape_delete_sms_sandbox_phone_number.rs`

```diff
--- reference/src/protocol_serde/shape_delete_sms_sandbox_phone_number.rs
+++ generated/src/protocol_serde/shape_delete_sms_sandbox_phone_number.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-    super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+    super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+        .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled(generic)),
+        None => return Err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::InternalErrorException({
+        "InternalError" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "InvalidParameter" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::InvalidParameterException({
+        "InvalidParameter" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,13 +65,13 @@
             }
             tmp
         }),
-        "ResourceNotFound" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::ResourceNotFoundException({
+        "ResourceNotFound" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,13 +80,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::ThrottledException({
+        "Throttled" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -95,13 +95,13 @@
             }
             tmp
         }),
-        "UserError" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::UserErrorException({
+        "UserError" => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::UserErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UserErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_user_error_exception::de_user_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -110,7 +110,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError::generic(generic),
+        _ => super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError::generic(generic),
     })
 }

@@ -121,7 +121,7 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-    super::super::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+    super::super::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
 > {
     Ok({
         #[allow(unused_mut)]
```

### `src/protocol_serde/shape_get_sms_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_get_sms_attributes.rs
+++ generated/src/protocol_serde/shape_get_sms_attributes.rs
@@ -4,26 +4,26 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_sms_attributes::GetSmsAttributesOutput, super::super::operation::get_sms_attributes::GetSMSAttributesError> {
+) -> std::result::Result<super::super::operation::get_sms_attributes::GetSmsAttributesOutput, super::super::operation::get_sms_attributes::GetSmsAttributesError> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled)?;
+        .map_err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled(generic)),
+        None => return Err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::get_sms_attributes::GetSMSAttributesError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::get_sms_attributes::GetSmsAttributesError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,13 +32,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::get_sms_attributes::GetSMSAttributesError::InternalErrorException({
+        "InternalError" => super::super::operation::get_sms_attributes::GetSmsAttributesError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,13 +47,13 @@
             }
             tmp
         }),
-        "InvalidParameter" => super::super::operation::get_sms_attributes::GetSMSAttributesError::InvalidParameterException({
+        "InvalidParameter" => super::super::operation::get_sms_attributes::GetSmsAttributesError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -62,13 +62,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::get_sms_attributes::GetSMSAttributesError::ThrottledException({
+        "Throttled" => super::super::operation::get_sms_attributes::GetSmsAttributesError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::get_sms_attributes::GetSMSAttributesError::generic(generic),
+        _ => super::super::operation::get_sms_attributes::GetSmsAttributesError::generic(generic),
     })
 }

@@ -86,12 +86,12 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_sms_attributes::GetSmsAttributesOutput, super::super::operation::get_sms_attributes::GetSMSAttributesError> {
+) -> std::result::Result<super::super::operation::get_sms_attributes::GetSmsAttributesOutput, super::super::operation::get_sms_attributes::GetSmsAttributesError> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::get_sms_attributes::builders::GetSmsAttributesOutputBuilder::default();
         output = super::super::protocol_serde::shape_get_sms_attributes::de_get_sms_attributes(_response_body, output)
-            .map_err(super::super::operation::get_sms_attributes::GetSMSAttributesError::unhandled)?;
+            .map_err(super::super::operation::get_sms_attributes::GetSmsAttributesError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -124,7 +124,7 @@
         }
         while let Some(mut tag) = result_tag.next_tag() {
             match tag.start_el() {
-            s if s.matches("attributes") /* attributes com.amazonaws.sns.synthetic#GetSMSAttributesOutput$attributes */ =>  {
+            s if s.matches("attributes") /* attributes com.amazonaws.sns.synthetic#GetSmsAttributesOutput$attributes */ =>  {
                 let var_1 =
                     Some(
                         super::super::protocol_serde::shape_map_string_to_string::de_map_string_to_string(&mut tag, depth + 1)
```

### `src/protocol_serde/shape_get_sms_sandbox_account_status.rs`

```diff
--- reference/src/protocol_serde/shape_get_sms_sandbox_account_status.rs
+++ generated/src/protocol_serde/shape_get_sms_sandbox_account_status.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-    super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+    super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::unhandled)?;
+        .map_err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::unhandled(generic)),
+        None => return Err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::InternalErrorException({
+        "InternalError" => super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::ThrottledException({
+        "Throttled" => super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::unhandled)?;
+                    .map_err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,7 +65,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::generic(generic),
+        _ => super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::generic(generic),
     })
 }

@@ -76,15 +76,17 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-    super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+    super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
 > {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusOutputBuilder::default();
         output = super::super::protocol_serde::shape_get_sms_sandbox_account_status::de_get_sms_sandbox_account_status(_response_body, output)
-            .map_err(super::super::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError::unhandled)?;
+            .map_err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::get_sms_sandbox_account_status_output_output_correct_errors(output).build()
+        super::super::serde_util::get_sms_sandbox_account_status_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError::unhandled)?
     })
 }

@@ -118,7 +120,7 @@
         }
         while let Some(mut tag) = result_tag.next_tag() {
             match tag.start_el() {
-            s if s.matches("IsInSandbox") /* IsInSandbox com.amazonaws.sns.synthetic#GetSMSSandboxAccountStatusOutput$IsInSandbox */ =>  {
+            s if s.matches("IsInSandbox") /* IsInSandbox com.amazonaws.sns.synthetic#GetSmsSandboxAccountStatusOutput$IsInSandbox */ =>  {
                 let var_1 =
                     Some(
                          {
```

### `src/protocol_serde/shape_list_origination_numbers.rs`

```diff
--- reference/src/protocol_serde/shape_list_origination_numbers.rs
+++ generated/src/protocol_serde/shape_list_origination_numbers.rs
@@ -96,6 +96,9 @@
                     .build()
                     .map_err(super::super::operation::list_origination_numbers::ListOriginationNumbersError::unhandled)?
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::super::operation::list_origination_numbers::ListOriginationNumbersError::generic(generic),
```

### `src/protocol_serde/shape_list_sms_sandbox_phone_numbers.rs`

```diff
--- reference/src/protocol_serde/shape_list_sms_sandbox_phone_numbers.rs
+++ generated/src/protocol_serde/shape_list_sms_sandbox_phone_numbers.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-    super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+    super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+        .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled(generic)),
+        None => return Err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::InternalErrorException({
+        "InternalError" => super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "InvalidParameter" => super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::InvalidParameterException({
+        "InvalidParameter" => super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,13 +65,13 @@
             }
             tmp
         }),
-        "ResourceNotFound" => super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::ResourceNotFoundException({
+        "ResourceNotFound" => super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,13 +80,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::ThrottledException({
+        "Throttled" => super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+                    .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -95,7 +95,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::generic(generic),
+        _ => super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::generic(generic),
     })
 }

@@ -106,17 +106,17 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-    super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+    super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
 > {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersOutputBuilder::default();
         output = super::super::protocol_serde::shape_list_sms_sandbox_phone_numbers::de_list_sms_sandbox_phone_numbers(_response_body, output)
-            .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?;
+            .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         super::super::serde_util::list_sms_sandbox_phone_numbers_output_output_correct_errors(output)
             .build()
-            .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError::unhandled)?
+            .map_err(super::super::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError::unhandled)?
     })
 }

@@ -150,7 +150,7 @@
         }
         while let Some(mut tag) = result_tag.next_tag() {
             match tag.start_el() {
-            s if s.matches("PhoneNumbers") /* PhoneNumbers com.amazonaws.sns.synthetic#ListSMSSandboxPhoneNumbersOutput$PhoneNumbers */ =>  {
+            s if s.matches("PhoneNumbers") /* PhoneNumbers com.amazonaws.sns.synthetic#ListSmsSandboxPhoneNumbersOutput$PhoneNumbers */ =>  {
                 let var_1 =
                     Some(
                         super::super::protocol_serde::shape_sms_sandbox_phone_number_list::de_sms_sandbox_phone_number_list(&mut tag, depth + 1)
@@ -160,7 +160,7 @@
                 builder = builder.set_phone_numbers(var_1);
             }
             ,
-            s if s.matches("NextToken") /* NextToken com.amazonaws.sns.synthetic#ListSMSSandboxPhoneNumbersOutput$NextToken */ =>  {
+            s if s.matches("NextToken") /* NextToken com.amazonaws.sns.synthetic#ListSmsSandboxPhoneNumbersOutput$NextToken */ =>  {
                 let var_2 =
                     Some(
                         Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
```

### `src/protocol_serde/shape_publish.rs`

```diff
--- reference/src/protocol_serde/shape_publish.rs
+++ generated/src/protocol_serde/shape_publish.rs
@@ -245,6 +245,9 @@
                     .build()
                     .map_err(super::super::operation::publish::PublishError::unhandled)?
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::super::operation::publish::PublishError::generic(generic),
```

### `src/protocol_serde/shape_publish_batch.rs`

```diff
--- reference/src/protocol_serde/shape_publish_batch.rs
+++ generated/src/protocol_serde/shape_publish_batch.rs
@@ -308,7 +308,8 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::TooManyEntriesInBatchRequestExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_too_many_entries_in_batch_request_exception::de_too_many_entries_in_batch_request_exception_xml_err(_response_body, output).map_err(super::super::operation::publish_batch::PublishBatchError::unhandled)?;
+                    output = super::super::protocol_serde::shape_too_many_entries_in_batch_request_exception::de_too_many_entries_in_batch_request_exception_xml_err(_response_body, output)
+                    .map_err(super::super::operation::publish_batch::PublishBatchError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -330,6 +331,9 @@
                     .build()
                     .map_err(super::super::operation::publish_batch::PublishBatchError::unhandled)?
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
         _ => super::super::operation::publish_batch::PublishBatchError::generic(generic),
```

### `src/protocol_serde/shape_set_sms_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_set_sms_attributes.rs
+++ generated/src/protocol_serde/shape_set_sms_attributes.rs
@@ -4,26 +4,26 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::set_sms_attributes::SetSmsAttributesOutput, super::super::operation::set_sms_attributes::SetSMSAttributesError> {
+) -> std::result::Result<super::super::operation::set_sms_attributes::SetSmsAttributesOutput, super::super::operation::set_sms_attributes::SetSmsAttributesError> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::set_sms_attributes::SetSMSAttributesError::unhandled)?;
+        .map_err(super::super::operation::set_sms_attributes::SetSmsAttributesError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::set_sms_attributes::SetSMSAttributesError::unhandled(generic)),
+        None => return Err(super::super::operation::set_sms_attributes::SetSmsAttributesError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::set_sms_attributes::SetSMSAttributesError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::set_sms_attributes::SetSmsAttributesError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::set_sms_attributes::SetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::set_sms_attributes::SetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,13 +32,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::set_sms_attributes::SetSMSAttributesError::InternalErrorException({
+        "InternalError" => super::super::operation::set_sms_attributes::SetSmsAttributesError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::set_sms_attributes::SetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::set_sms_attributes::SetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,13 +47,13 @@
             }
             tmp
         }),
-        "InvalidParameter" => super::super::operation::set_sms_attributes::SetSMSAttributesError::InvalidParameterException({
+        "InvalidParameter" => super::super::operation::set_sms_attributes::SetSmsAttributesError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::set_sms_attributes::SetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::set_sms_attributes::SetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -62,13 +62,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::set_sms_attributes::SetSMSAttributesError::ThrottledException({
+        "Throttled" => super::super::operation::set_sms_attributes::SetSmsAttributesError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::set_sms_attributes::SetSMSAttributesError::unhandled)?;
+                    .map_err(super::super::operation::set_sms_attributes::SetSmsAttributesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -77,7 +77,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::set_sms_attributes::SetSMSAttributesError::generic(generic),
+        _ => super::super::operation::set_sms_attributes::SetSmsAttributesError::generic(generic),
     })
 }

@@ -86,7 +86,7 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::set_sms_attributes::SetSmsAttributesOutput, super::super::operation::set_sms_attributes::SetSMSAttributesError> {
+) -> std::result::Result<super::super::operation::set_sms_attributes::SetSmsAttributesOutput, super::super::operation::set_sms_attributes::SetSmsAttributesError> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::set_sms_attributes::builders::SetSmsAttributesOutputBuilder::default();
```

### `src/protocol_serde/shape_verify_sms_sandbox_phone_number.rs`

```diff
--- reference/src/protocol_serde/shape_verify_sms_sandbox_phone_number.rs
+++ generated/src/protocol_serde/shape_verify_sms_sandbox_phone_number.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-    super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+    super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+        .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled(generic)),
+        None => return Err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "AuthorizationError" => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::AuthorizationErrorException({
+        "AuthorizationError" => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::AuthorizationErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InternalError" => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::InternalErrorException({
+        "InternalError" => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "InvalidParameter" => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::InvalidParameterException({
+        "InvalidParameter" => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,13 +65,13 @@
             }
             tmp
         }),
-        "ResourceNotFound" => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::ResourceNotFoundException({
+        "ResourceNotFound" => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,13 +80,13 @@
             }
             tmp
         }),
-        "Throttled" => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::ThrottledException({
+        "Throttled" => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::ThrottledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ThrottledExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_throttled_exception::de_throttled_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -95,21 +95,24 @@
             }
             tmp
         }),
-        "VerificationException" => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::VerificationException({
+        "VerificationException" => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::VerificationException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::VerificationExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_verification_exception::de_verification_exception_xml_err(_response_body, output)
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?;
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?;
                 let output = output.meta(generic);
                 super::super::serde_util::verification_exception_correct_errors(output)
                     .build()
-                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::unhandled)?
+                    .map_err(super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::unhandled)?
             };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
             tmp
         }),
-        _ => super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError::generic(generic),
+        _ => super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError::generic(generic),
     })
 }

@@ -120,7 +123,7 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-    super::super::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+    super::super::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
 > {
     Ok({
         #[allow(unused_mut)]
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -229,8 +229,6 @@

 pub(crate) mod shape_kms_throttling_exception;

-pub(crate) mod shape_message_attribute_value;
-
 pub(crate) mod shape_not_found_exception;

 pub(crate) mod shape_opted_out_exception;
@@ -237,8 +235,6 @@

 pub(crate) mod shape_platform_application_disabled_exception;

-pub(crate) mod shape_publish_batch_request_entry;
-
 pub(crate) mod shape_replay_limit_exceeded_exception;

 pub(crate) mod shape_resource_not_found_exception;
@@ -247,8 +243,6 @@

 pub(crate) mod shape_subscription_limit_exceeded_exception;

-pub(crate) mod shape_tag;
-
 pub(crate) mod shape_tag_limit_exceeded_exception;

 pub(crate) mod shape_tag_policy_exception;
@@ -273,10 +267,14 @@

 pub(crate) mod shape_map_string_to_string;

+pub(crate) mod shape_message_attribute_value;
+
 pub(crate) mod shape_phone_number_information_list;

 pub(crate) mod shape_phone_number_list;

+pub(crate) mod shape_publish_batch_request_entry;
+
 pub(crate) mod shape_publish_batch_result_entry_list;

 pub(crate) mod shape_sms_sandbox_phone_number_list;
@@ -285,6 +283,8 @@

 pub(crate) mod shape_subscriptions_list;

+pub(crate) mod shape_tag;
+
 pub(crate) mod shape_tag_list;

 pub(crate) mod shape_topic_attributes_map;
```
