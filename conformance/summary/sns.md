# AWS SDK Conformance Report: sns

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sns
**Progress:** `449/449` files compared · `240` matched · `63` mismatches · `146` missing · `0` extra · `53.45%` match (100.00% means fully matched)

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
     pub fn set_sms_attributes(&self) -> crate::operation::set_sms_attributes::builders::SetSMSAttributesFluentBuilder {
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

### `src/operation/add_permission.rs`

```diff
--- reference/src/operation/add_permission.rs
+++ generated/src/operation/add_permission.rs
@@ -252,13 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_add_permission_input::ser_add_permission_input_input_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_add_permission_input::ser_add_permission_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/check_if_phone_number_is_opted_out.rs`

```diff
--- reference/src/operation/check_if_phone_number_is_opted_out.rs
+++ generated/src/operation/check_if_phone_number_is_opted_out.rs
@@ -206,12 +206,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_check_if_phone_number_is_opted_out_input::ser_check_if_phone_number_is_opted_out_input_input_input(&input)?,
+            crate::protocol_serde::shape_check_if_phone_number_is_opted_out_input::ser_check_if_phone_number_is_opted_out_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/confirm_subscription.rs`

```diff
--- reference/src/operation/confirm_subscription.rs
+++ generated/src/operation/confirm_subscription.rs
@@ -260,12 +260,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_confirm_subscription_input::ser_confirm_subscription_input_input_input(&input)?,
+            crate::protocol_serde::shape_confirm_subscription_input::ser_confirm_subscription_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_platform_application.rs`

```diff
--- reference/src/operation/create_platform_application.rs
+++ generated/src/operation/create_platform_application.rs
@@ -255,12 +255,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_platform_application_input::ser_create_platform_application_input_input_input(&input)?,
+            crate::protocol_serde::shape_create_platform_application_input::ser_create_platform_application_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_platform_endpoint.rs`

```diff
--- reference/src/operation/create_platform_endpoint.rs
+++ generated/src/operation/create_platform_endpoint.rs
@@ -260,12 +260,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_platform_endpoint_input::ser_create_platform_endpoint_input_input_input(&input)?,
+            crate::protocol_serde::shape_create_platform_endpoint_input::ser_create_platform_endpoint_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_sms_sandbox_phone_number/builders.rs`

```diff
--- reference/src/operation/create_sms_sandbox_phone_number/builders.rs
+++ generated/src/operation/create_sms_sandbox_phone_number/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+            crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-        crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
-    > for CreateSMSSandboxPhoneNumberFluentBuilder
+        crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
+    > for CreateSmsSandboxPhoneNumberFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+            crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl CreateSMSSandboxPhoneNumberFluentBuilder {
-    /// Creates a new `CreateSMSSandboxPhoneNumberFluentBuilder`.
+impl CreateSmsSandboxPhoneNumberFluentBuilder {
+    /// Creates a new `CreateSmsSandboxPhoneNumberFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the CreateSMSSandboxPhoneNumber as a reference.
+    /// Access the CreateSmsSandboxPhoneNumber as a reference.
     pub fn as_input(&self) -> &crate::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+            crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumber::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumber::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
+        crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
-        crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
+        crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
@@ -204,12 +204,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_sms_sandbox_phone_number_input::ser_create_sms_sandbox_phone_number_input_input_input(&input)?,
+            crate::protocol_serde::shape_create_sms_sandbox_phone_number_input::ser_create_sms_sandbox_phone_number_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -219,12 +218,12 @@
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

### `src/operation/create_topic.rs`

```diff
--- reference/src/operation/create_topic.rs
+++ generated/src/operation/create_topic.rs
@@ -252,13 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_topic_input::ser_create_topic_input_input_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_topic_input::ser_create_topic_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_endpoint.rs`

```diff
--- reference/src/operation/delete_endpoint.rs
+++ generated/src/operation/delete_endpoint.rs
@@ -247,13 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_delete_endpoint_input::ser_delete_endpoint_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_endpoint_input::ser_delete_endpoint_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_platform_application.rs`

```diff
--- reference/src/operation/delete_platform_application.rs
+++ generated/src/operation/delete_platform_application.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_delete_platform_application_input::ser_delete_platform_application_input_input_input(&input)?,
+            crate::protocol_serde::shape_delete_platform_application_input::ser_delete_platform_application_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_sms_sandbox_phone_number/builders.rs`

```diff
--- reference/src/operation/delete_sms_sandbox_phone_number/builders.rs
+++ generated/src/operation/delete_sms_sandbox_phone_number/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+            crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::delete_sms_sandbox_phone_number::builders::DeleteSmsSandboxPhoneNumberInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-        crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
-    > for DeleteSMSSandboxPhoneNumberFluentBuilder
+        crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
+    > for DeleteSmsSandboxPhoneNumberFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-            crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+            crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeleteSMSSandboxPhoneNumberFluentBuilder {
-    /// Creates a new `DeleteSMSSandboxPhoneNumberFluentBuilder`.
+impl DeleteSmsSandboxPhoneNumberFluentBuilder {
+    /// Creates a new `DeleteSmsSandboxPhoneNumberFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeleteSMSSandboxPhoneNumber as a reference.
+    /// Access the DeleteSmsSandboxPhoneNumber as a reference.
     pub fn as_input(&self) -> &crate::operation::delete_sms_sandbox_phone_number::builders::DeleteSmsSandboxPhoneNumberInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+            crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumber::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumber::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
+        crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberOutput,
-        crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
+        crate::operation::delete_sms_sandbox_phone_number::DeleteSmsSandboxPhoneNumberError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
@@ -204,12 +204,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_delete_sms_sandbox_phone_number_input::ser_delete_sms_sandbox_phone_number_input_input_input(&input)?,
+            crate::protocol_serde::shape_delete_sms_sandbox_phone_number_input::ser_delete_sms_sandbox_phone_number_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -219,12 +218,12 @@
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

### `src/operation/delete_topic.rs`

```diff
--- reference/src/operation/delete_topic.rs
+++ generated/src/operation/delete_topic.rs
@@ -247,13 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_topic_input::ser_delete_topic_input_input_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_topic_input::ser_delete_topic_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_data_protection_policy.rs`

```diff
--- reference/src/operation/get_data_protection_policy.rs
+++ generated/src/operation/get_data_protection_policy.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_data_protection_policy_input::ser_get_data_protection_policy_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_data_protection_policy_input::ser_get_data_protection_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_endpoint_attributes.rs`

```diff
--- reference/src/operation/get_endpoint_attributes.rs
+++ generated/src/operation/get_endpoint_attributes.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_endpoint_attributes_input::ser_get_endpoint_attributes_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_endpoint_attributes_input::ser_get_endpoint_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_platform_application_attributes.rs`

```diff
--- reference/src/operation/get_platform_application_attributes.rs
+++ generated/src/operation/get_platform_application_attributes.rs
@@ -258,14 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_platform_application_attributes_input::ser_get_platform_application_attributes_input_input_input(
-                &input,
-            )?,
+            crate::protocol_serde::shape_get_platform_application_attributes_input::ser_get_platform_application_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_sms_attributes/builders.rs`

```diff
--- reference/src/operation/get_sms_attributes/builders.rs
+++ generated/src/operation/get_sms_attributes/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         crate::operation::get_sms_attributes::GetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::get_sms_attributes::GetSMSAttributesError,
+            crate::operation::get_sms_attributes::GetSmsAttributesError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::get_sms_attributes::builders::GetSmsAttributesInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::get_sms_attributes::GetSmsAttributesOutput,
-        crate::operation::get_sms_attributes::GetSMSAttributesError,
-    > for GetSMSAttributesFluentBuilder
+        crate::operation::get_sms_attributes::GetSmsAttributesError,
+    > for GetSmsAttributesFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::get_sms_attributes::GetSmsAttributesOutput,
-            crate::operation::get_sms_attributes::GetSMSAttributesError,
+            crate::operation::get_sms_attributes::GetSmsAttributesError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetSMSAttributesFluentBuilder {
-    /// Creates a new `GetSMSAttributesFluentBuilder`.
+impl GetSmsAttributesFluentBuilder {
+    /// Creates a new `GetSmsAttributesFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetSMSAttributes as a reference.
+    /// Access the GetSmsAttributes as a reference.
     pub fn as_input(&self) -> &crate::operation::get_sms_attributes::builders::GetSmsAttributesInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         crate::operation::get_sms_attributes::GetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::get_sms_attributes::GetSMSAttributesError,
+            crate::operation::get_sms_attributes::GetSmsAttributesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::get_sms_attributes::GetSMSAttributes::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::get_sms_attributes::GetSmsAttributes::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::get_sms_attributes::GetSMSAttributes::orchestrate(&runtime_plugins, input).await
+        crate::operation::get_sms_attributes::GetSmsAttributes::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::get_sms_attributes::GetSmsAttributesOutput,
-        crate::operation::get_sms_attributes::GetSMSAttributesError,
+        crate::operation::get_sms_attributes::GetSmsAttributesError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::get_sms_attributes::GetSMSAttributesError,
@@ -201,13 +201,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_sms_attributes_input::ser_get_sms_attributes_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_sms_attributes_input::ser_get_sms_attributes_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -216,12 +215,12 @@
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
         crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+            crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-        crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
-    > for GetSMSSandboxAccountStatusFluentBuilder
+        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
+    > for GetSmsSandboxAccountStatusFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-            crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+            crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetSMSSandboxAccountStatusFluentBuilder {
-    /// Creates a new `GetSMSSandboxAccountStatusFluentBuilder`.
+impl GetSmsSandboxAccountStatusFluentBuilder {
+    /// Creates a new `GetSmsSandboxAccountStatusFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetSMSSandboxAccountStatus as a reference.
+    /// Access the GetSmsSandboxAccountStatus as a reference.
     pub fn as_input(&self) -> &crate::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+            crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatus::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatus::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatus::orchestrate(&runtime_plugins, input).await
+        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatus::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
-        crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
+        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
@@ -204,24 +204,20 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_sms_sandbox_account_status_input::ser_get_sms_sandbox_account_status_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

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

### `src/operation/get_subscription_attributes.rs`

```diff
--- reference/src/operation/get_subscription_attributes.rs
+++ generated/src/operation/get_subscription_attributes.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_subscription_attributes_input::ser_get_subscription_attributes_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_subscription_attributes_input::ser_get_subscription_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_topic_attributes.rs`

```diff
--- reference/src/operation/get_topic_attributes.rs
+++ generated/src/operation/get_topic_attributes.rs
@@ -247,12 +247,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_topic_attributes_input::ser_get_topic_attributes_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_topic_attributes_input::ser_get_topic_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_endpoints_by_platform_application.rs`

```diff
--- reference/src/operation/list_endpoints_by_platform_application.rs
+++ generated/src/operation/list_endpoints_by_platform_application.rs
@@ -265,14 +265,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_endpoints_by_platform_application_input::ser_list_endpoints_by_platform_application_input_input_input(
-                &input,
-            )?,
+            crate::protocol_serde::shape_list_endpoints_by_platform_application_input::ser_list_endpoints_by_platform_application_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_origination_numbers.rs`

```diff
--- reference/src/operation/list_origination_numbers.rs
+++ generated/src/operation/list_origination_numbers.rs
@@ -251,12 +251,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_origination_numbers_input::ser_list_origination_numbers_input_input_input(&input)?,
+            crate::protocol_serde::shape_list_origination_numbers_input::ser_list_origination_numbers_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
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

### `src/operation/list_phone_numbers_opted_out.rs`

```diff
--- reference/src/operation/list_phone_numbers_opted_out.rs
+++ generated/src/operation/list_phone_numbers_opted_out.rs
@@ -251,12 +251,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_phone_numbers_opted_out_input::ser_list_phone_numbers_opted_out_input_input_input(&input)?,
+            crate::protocol_serde::shape_list_phone_numbers_opted_out_input::ser_list_phone_numbers_opted_out_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_platform_applications.rs`

```diff
--- reference/src/operation/list_platform_applications.rs
+++ generated/src/operation/list_platform_applications.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_platform_applications_input::ser_list_platform_applications_input_input_input(&input)?,
+            crate::protocol_serde::shape_list_platform_applications_input::ser_list_platform_applications_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_sms_sandbox_phone_numbers/builders.rs`

```diff
--- reference/src/operation/list_sms_sandbox_phone_numbers/builders.rs
+++ generated/src/operation/list_sms_sandbox_phone_numbers/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+            crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-        crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
-    > for ListSMSSandboxPhoneNumbersFluentBuilder
+        crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
+    > for ListSmsSandboxPhoneNumbersFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-            crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+            crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListSMSSandboxPhoneNumbersFluentBuilder {
-    /// Creates a new `ListSMSSandboxPhoneNumbersFluentBuilder`.
+impl ListSmsSandboxPhoneNumbersFluentBuilder {
+    /// Creates a new `ListSmsSandboxPhoneNumbersFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListSMSSandboxPhoneNumbers as a reference.
+    /// Access the ListSmsSandboxPhoneNumbers as a reference.
     pub fn as_input(&self) -> &crate::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+            crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbers::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbers::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbers::orchestrate(&runtime_plugins, input).await
+        crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbers::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
-        crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
+        crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
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
@@ -251,12 +251,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_sms_sandbox_phone_numbers_input::ser_list_sms_sandbox_phone_numbers_input_input_input(&input)?,
+            crate::protocol_serde::shape_list_sms_sandbox_phone_numbers_input::ser_list_sms_sandbox_phone_numbers_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -266,12 +265,12 @@
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

### `src/operation/list_subscriptions.rs`

```diff
--- reference/src/operation/list_subscriptions.rs
+++ generated/src/operation/list_subscriptions.rs
@@ -247,13 +247,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_subscriptions_input::ser_list_subscriptions_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_subscriptions_input::ser_list_subscriptions_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_subscriptions_by_topic.rs`

```diff
--- reference/src/operation/list_subscriptions_by_topic.rs
+++ generated/src/operation/list_subscriptions_by_topic.rs
@@ -255,12 +255,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_subscriptions_by_topic_input::ser_list_subscriptions_by_topic_input_input_input(&input)?,
+            crate::protocol_serde::shape_list_subscriptions_by_topic_input::ser_list_subscriptions_by_topic_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_tags_for_resource.rs`

```diff
--- reference/src/operation/list_tags_for_resource.rs
+++ generated/src/operation/list_tags_for_resource.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_tags_for_resource_input::ser_list_tags_for_resource_input_input_input(&input)?,
+            crate::protocol_serde::shape_list_tags_for_resource_input::ser_list_tags_for_resource_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_topics.rs`

```diff
--- reference/src/operation/list_topics.rs
+++ generated/src/operation/list_topics.rs
@@ -245,12 +245,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_topics_input::ser_list_topics_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_topics_input::ser_list_topics_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/opt_in_phone_number.rs`

```diff
--- reference/src/operation/opt_in_phone_number.rs
+++ generated/src/operation/opt_in_phone_number.rs
@@ -201,13 +201,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_opt_in_phone_number_input::ser_opt_in_phone_number_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_opt_in_phone_number_input::ser_opt_in_phone_number_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/publish.rs`

```diff
--- reference/src/operation/publish.rs
+++ generated/src/operation/publish.rs
@@ -271,11 +271,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_input::ser_publish_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_input::ser_publish_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/publish_batch.rs`

```diff
--- reference/src/operation/publish_batch.rs
+++ generated/src/operation/publish_batch.rs
@@ -247,13 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_batch_input::ser_publish_batch_input_input_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_batch_input::ser_publish_batch_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/put_data_protection_policy.rs`

```diff
--- reference/src/operation/put_data_protection_policy.rs
+++ generated/src/operation/put_data_protection_policy.rs
@@ -255,12 +255,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_data_protection_policy_input::ser_put_data_protection_policy_input_input_input(&input)?,
+            crate::protocol_serde::shape_put_data_protection_policy_input::ser_put_data_protection_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/remove_permission.rs`

```diff
--- reference/src/operation/remove_permission.rs
+++ generated/src/operation/remove_permission.rs
@@ -252,13 +252,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_remove_permission_input::ser_remove_permission_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_remove_permission_input::ser_remove_permission_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/set_endpoint_attributes.rs`

```diff
--- reference/src/operation/set_endpoint_attributes.rs
+++ generated/src/operation/set_endpoint_attributes.rs
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_set_endpoint_attributes_input::ser_set_endpoint_attributes_input_input_input(&input)?,
+            crate::protocol_serde::shape_set_endpoint_attributes_input::ser_set_endpoint_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/set_platform_application_attributes.rs`

```diff
--- reference/src/operation/set_platform_application_attributes.rs
+++ generated/src/operation/set_platform_application_attributes.rs
@@ -258,14 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_set_platform_application_attributes_input::ser_set_platform_application_attributes_input_input_input(
-                &input,
-            )?,
+            crate::protocol_serde::shape_set_platform_application_attributes_input::ser_set_platform_application_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
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
         crate::operation::set_sms_attributes::SetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::set_sms_attributes::SetSMSAttributesError,
+            crate::operation::set_sms_attributes::SetSmsAttributesError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::set_sms_attributes::builders::SetSmsAttributesInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -35,8 +35,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::set_sms_attributes::SetSmsAttributesOutput,
-        crate::operation::set_sms_attributes::SetSMSAttributesError,
-    > for SetSMSAttributesFluentBuilder
+        crate::operation::set_sms_attributes::SetSmsAttributesError,
+    > for SetSmsAttributesFluentBuilder
 {
     fn send(
         self,
@@ -44,14 +44,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::set_sms_attributes::SetSmsAttributesOutput,
-            crate::operation::set_sms_attributes::SetSMSAttributesError,
+            crate::operation::set_sms_attributes::SetSmsAttributesError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl SetSMSAttributesFluentBuilder {
-    /// Creates a new `SetSMSAttributesFluentBuilder`.
+impl SetSmsAttributesFluentBuilder {
+    /// Creates a new `SetSmsAttributesFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -59,7 +59,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the SetSMSAttributes as a reference.
+    /// Access the SetSmsAttributes as a reference.
     pub fn as_input(&self) -> &crate::operation::set_sms_attributes::builders::SetSmsAttributesInputBuilder {
         &self.inner
     }
@@ -76,7 +76,7 @@
     ) -> ::std::result::Result<
         crate::operation::set_sms_attributes::SetSmsAttributesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::set_sms_attributes::SetSMSAttributesError,
+            crate::operation::set_sms_attributes::SetSmsAttributesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,12 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::set_sms_attributes::SetSMSAttributes::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::set_sms_attributes::SetSmsAttributes::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::set_sms_attributes::SetSMSAttributes::orchestrate(&runtime_plugins, input).await
+        crate::operation::set_sms_attributes::SetSmsAttributes::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -97,7 +97,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::set_sms_attributes::SetSmsAttributesOutput,
-        crate::operation::set_sms_attributes::SetSMSAttributesError,
+        crate::operation::set_sms_attributes::SetSmsAttributesError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::set_sms_attributes::SetSMSAttributesError,
@@ -201,13 +201,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_set_sms_attributes_input::ser_set_sms_attributes_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_set_sms_attributes_input::ser_set_sms_attributes_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -216,12 +215,12 @@
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

### `src/operation/set_subscription_attributes.rs`

```diff
--- reference/src/operation/set_subscription_attributes.rs
+++ generated/src/operation/set_subscription_attributes.rs
@@ -260,12 +260,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_set_subscription_attributes_input::ser_set_subscription_attributes_input_input_input(&input)?,
+            crate::protocol_serde::shape_set_subscription_attributes_input::ser_set_subscription_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/set_topic_attributes.rs`

```diff
--- reference/src/operation/set_topic_attributes.rs
+++ generated/src/operation/set_topic_attributes.rs
@@ -257,12 +257,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_set_topic_attributes_input::ser_set_topic_attributes_input_input_input(&input)?,
+            crate::protocol_serde::shape_set_topic_attributes_input::ser_set_topic_attributes_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
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

### `src/operation/subscribe.rs`

```diff
--- reference/src/operation/subscribe.rs
+++ generated/src/operation/subscribe.rs
@@ -252,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_subscribe_input::ser_subscribe_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_subscribe_input::ser_subscribe_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/tag_resource.rs`

```diff
--- reference/src/operation/tag_resource.rs
+++ generated/src/operation/tag_resource.rs
@@ -247,13 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_input_input_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/unsubscribe.rs`

```diff
--- reference/src/operation/unsubscribe.rs
+++ generated/src/operation/unsubscribe.rs
@@ -245,12 +245,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_unsubscribe_input::ser_unsubscribe_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_unsubscribe_input::ser_unsubscribe_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -247,13 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_resource_input::ser_untag_resource_input_input_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_resource_input::ser_untag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/verify_sms_sandbox_phone_number/builders.rs`

```diff
--- reference/src/operation/verify_sms_sandbox_phone_number/builders.rs
+++ generated/src/operation/verify_sms_sandbox_phone_number/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+            crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
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
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::verify_sms_sandbox_phone_number::builders::VerifySmsSandboxPhoneNumberInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-        crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
-    > for VerifySMSSandboxPhoneNumberFluentBuilder
+        crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
+    > for VerifySmsSandboxPhoneNumberFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-            crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+            crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl VerifySMSSandboxPhoneNumberFluentBuilder {
-    /// Creates a new `VerifySMSSandboxPhoneNumberFluentBuilder`.
+impl VerifySmsSandboxPhoneNumberFluentBuilder {
+    /// Creates a new `VerifySmsSandboxPhoneNumberFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the VerifySMSSandboxPhoneNumber as a reference.
+    /// Access the VerifySmsSandboxPhoneNumber as a reference.
     pub fn as_input(&self) -> &crate::operation::verify_sms_sandbox_phone_number::builders::VerifySmsSandboxPhoneNumberInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+            crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumber::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumber::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
+        crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumber::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput,
-        crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
+        crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
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
                 crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
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
@@ -250,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_verify_sms_sandbox_phone_number_input::ser_verify_sms_sandbox_phone_number_input_input_input(&input)?,
+            crate::protocol_serde::shape_verify_sms_sandbox_phone_number_input::ser_verify_sms_sandbox_phone_number_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +264,12 @@
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

### `src/types/error/_kms_access_denied_exception.rs`

```diff
--- reference/src/types/error/_kms_access_denied_exception.rs
+++ generated/src/types/error/_kms_access_denied_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsAccessDeniedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsAccessDeniedException [KMSAccessDeniedException]")?;
+        ::std::write!(f, "KmsAccessDeniedException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_disabled_exception.rs`

```diff
--- reference/src/types/error/_kms_disabled_exception.rs
+++ generated/src/types/error/_kms_disabled_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsDisabledException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsDisabledException [KMSDisabledException]")?;
+        ::std::write!(f, "KmsDisabledException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_invalid_state_exception.rs`

```diff
--- reference/src/types/error/_kms_invalid_state_exception.rs
+++ generated/src/types/error/_kms_invalid_state_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsInvalidStateException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsInvalidStateException [KMSInvalidStateException]")?;
+        ::std::write!(f, "KmsInvalidStateException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_not_found_exception.rs`

```diff
--- reference/src/types/error/_kms_not_found_exception.rs
+++ generated/src/types/error/_kms_not_found_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsNotFoundException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsNotFoundException [KMSNotFoundException]")?;
+        ::std::write!(f, "KmsNotFoundException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_opt_in_required.rs`

```diff
--- reference/src/types/error/_kms_opt_in_required.rs
+++ generated/src/types/error/_kms_opt_in_required.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsOptInRequired {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsOptInRequired [KMSOptInRequired]")?;
+        ::std::write!(f, "KmsOptInRequired")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_throttling_exception.rs`

```diff
--- reference/src/types/error/_kms_throttling_exception.rs
+++ generated/src/types/error/_kms_throttling_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsThrottlingException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsThrottlingException [KMSThrottlingException]")?;
+        ::std::write!(f, "KmsThrottlingException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_validation_exception.rs`

```diff
--- reference/src/types/error/_validation_exception.rs
+++ generated/src/types/error/_validation_exception.rs
@@ -10,15 +10,17 @@
 }
 impl ValidationException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for ValidationException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "ValidationException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### `src/types/error/_verification_exception.rs`

```diff
--- reference/src/types/error/_verification_exception.rs
+++ generated/src/types/error/_verification_exception.rs
@@ -19,15 +19,17 @@
 }
 impl VerificationException {
     /// Returns the error message.
-    pub fn message(&self) -> &str {
-        &self.message
+    pub fn message(&self) -> ::std::option::Option<&str> {
+        self.message.as_deref()
     }
 }
 impl ::std::fmt::Display for VerificationException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         ::std::write!(f, "VerificationException")?;
-        {
-            ::std::write!(f, ": {}", &self.message)?;
+        if let ::std::option::Option::Some(inner_1) = &self.message {
+            {
+                ::std::write!(f, ": {inner_1}")?;
+            }
         }
         Ok(())
     }
```

### Missing reference files

- `Cargo.toml`
- `src/protocol_serde/shape_add_permission.rs`
- `src/protocol_serde/shape_add_permission_input.rs`
- `src/protocol_serde/shape_authorization_error_exception.rs`
- `src/protocol_serde/shape_batch_entry_ids_not_distinct_exception.rs`
- `src/protocol_serde/shape_batch_request_too_long_exception.rs`
- `src/protocol_serde/shape_batch_result_error_entry.rs`
- `src/protocol_serde/shape_batch_result_error_entry_list.rs`
- `src/protocol_serde/shape_check_if_phone_number_is_opted_out.rs`
- `src/protocol_serde/shape_check_if_phone_number_is_opted_out_input.rs`
- `src/protocol_serde/shape_concurrent_access_exception.rs`
- `src/protocol_serde/shape_confirm_subscription.rs`
- `src/protocol_serde/shape_confirm_subscription_input.rs`
- `src/protocol_serde/shape_create_platform_application.rs`
- `src/protocol_serde/shape_create_platform_application_input.rs`
- `src/protocol_serde/shape_create_platform_endpoint.rs`
- `src/protocol_serde/shape_create_platform_endpoint_input.rs`
- `src/protocol_serde/shape_create_sms_sandbox_phone_number.rs`
- `src/protocol_serde/shape_create_sms_sandbox_phone_number_input.rs`
- `src/protocol_serde/shape_create_topic.rs`
- `src/protocol_serde/shape_create_topic_input.rs`
- `src/protocol_serde/shape_delete_endpoint.rs`
- `src/protocol_serde/shape_delete_endpoint_input.rs`
- `src/protocol_serde/shape_delete_platform_application.rs`
- `src/protocol_serde/shape_delete_platform_application_input.rs`
- `src/protocol_serde/shape_delete_sms_sandbox_phone_number.rs`
- `src/protocol_serde/shape_delete_sms_sandbox_phone_number_input.rs`
- `src/protocol_serde/shape_delete_topic.rs`
- `src/protocol_serde/shape_delete_topic_input.rs`
- `src/protocol_serde/shape_empty_batch_request_exception.rs`
- `src/protocol_serde/shape_endpoint.rs`
- `src/protocol_serde/shape_endpoint_disabled_exception.rs`
- `src/protocol_serde/shape_filter_policy_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_get_data_protection_policy.rs`
- `src/protocol_serde/shape_get_data_protection_policy_input.rs`
- `src/protocol_serde/shape_get_endpoint_attributes.rs`
- `src/protocol_serde/shape_get_endpoint_attributes_input.rs`
- `src/protocol_serde/shape_get_platform_application_attributes.rs`
- `src/protocol_serde/shape_get_platform_application_attributes_input.rs`
- `src/protocol_serde/shape_get_sms_attributes.rs`
- `src/protocol_serde/shape_get_sms_attributes_input.rs`
- `src/protocol_serde/shape_get_sms_sandbox_account_status.rs`
- `src/protocol_serde/shape_get_sms_sandbox_account_status_input.rs`
- `src/protocol_serde/shape_get_subscription_attributes.rs`
- `src/protocol_serde/shape_get_subscription_attributes_input.rs`
- `src/protocol_serde/shape_get_topic_attributes.rs`
- `src/protocol_serde/shape_get_topic_attributes_input.rs`
- `src/protocol_serde/shape_internal_error_exception.rs`
- `src/protocol_serde/shape_invalid_batch_entry_id_exception.rs`
- `src/protocol_serde/shape_invalid_parameter_exception.rs`
- `src/protocol_serde/shape_invalid_parameter_value_exception.rs`
- `src/protocol_serde/shape_invalid_security_exception.rs`
- `src/protocol_serde/shape_invalid_state_exception.rs`
- `src/protocol_serde/shape_kms_access_denied_exception.rs`
- `src/protocol_serde/shape_kms_disabled_exception.rs`
- `src/protocol_serde/shape_kms_invalid_state_exception.rs`
- `src/protocol_serde/shape_kms_not_found_exception.rs`
- `src/protocol_serde/shape_kms_opt_in_required.rs`
- `src/protocol_serde/shape_kms_throttling_exception.rs`
- `src/protocol_serde/shape_list_endpoints_by_platform_application.rs`
- `src/protocol_serde/shape_list_endpoints_by_platform_application_input.rs`
- `src/protocol_serde/shape_list_of_endpoints.rs`
- `src/protocol_serde/shape_list_of_platform_applications.rs`
- `src/protocol_serde/shape_list_origination_numbers.rs`
- `src/protocol_serde/shape_list_origination_numbers_input.rs`
- `src/protocol_serde/shape_list_phone_numbers_opted_out.rs`
- `src/protocol_serde/shape_list_phone_numbers_opted_out_input.rs`
- `src/protocol_serde/shape_list_platform_applications.rs`
- `src/protocol_serde/shape_list_platform_applications_input.rs`
- `src/protocol_serde/shape_list_sms_sandbox_phone_numbers.rs`
- `src/protocol_serde/shape_list_sms_sandbox_phone_numbers_input.rs`
- `src/protocol_serde/shape_list_subscriptions.rs`
- `src/protocol_serde/shape_list_subscriptions_by_topic.rs`
- `src/protocol_serde/shape_list_subscriptions_by_topic_input.rs`
- `src/protocol_serde/shape_list_subscriptions_input.rs`
- `src/protocol_serde/shape_list_tags_for_resource.rs`
- `src/protocol_serde/shape_list_tags_for_resource_input.rs`
- `src/protocol_serde/shape_list_topics.rs`
- `src/protocol_serde/shape_list_topics_input.rs`
- `src/protocol_serde/shape_map_string_to_string.rs`
- `src/protocol_serde/shape_message_attribute_value.rs`
- `src/protocol_serde/shape_not_found_exception.rs`
- `src/protocol_serde/shape_number_capability_list.rs`
- `src/protocol_serde/shape_opt_in_phone_number.rs`
- `src/protocol_serde/shape_opt_in_phone_number_input.rs`
- `src/protocol_serde/shape_opted_out_exception.rs`
- `src/protocol_serde/shape_phone_number_information.rs`
- `src/protocol_serde/shape_phone_number_information_list.rs`
- `src/protocol_serde/shape_phone_number_list.rs`
- `src/protocol_serde/shape_platform_application.rs`
- `src/protocol_serde/shape_platform_application_disabled_exception.rs`
- `src/protocol_serde/shape_publish.rs`
- `src/protocol_serde/shape_publish_batch.rs`
- `src/protocol_serde/shape_publish_batch_input.rs`
- `src/protocol_serde/shape_publish_batch_request_entry.rs`
- `src/protocol_serde/shape_publish_batch_result_entry.rs`
- `src/protocol_serde/shape_publish_batch_result_entry_list.rs`
- `src/protocol_serde/shape_publish_input.rs`
- `src/protocol_serde/shape_put_data_protection_policy.rs`
- `src/protocol_serde/shape_put_data_protection_policy_input.rs`
- `src/protocol_serde/shape_remove_permission.rs`
- `src/protocol_serde/shape_remove_permission_input.rs`
- `src/protocol_serde/shape_replay_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_resource_not_found_exception.rs`
- `src/protocol_serde/shape_set_endpoint_attributes.rs`
- `src/protocol_serde/shape_set_endpoint_attributes_input.rs`
- `src/protocol_serde/shape_set_platform_application_attributes.rs`
- `src/protocol_serde/shape_set_platform_application_attributes_input.rs`
- `src/protocol_serde/shape_set_sms_attributes.rs`
- `src/protocol_serde/shape_set_sms_attributes_input.rs`
- `src/protocol_serde/shape_set_subscription_attributes.rs`
- `src/protocol_serde/shape_set_subscription_attributes_input.rs`
- `src/protocol_serde/shape_set_topic_attributes.rs`
- `src/protocol_serde/shape_set_topic_attributes_input.rs`
- `src/protocol_serde/shape_sms_sandbox_phone_number.rs`
- `src/protocol_serde/shape_sms_sandbox_phone_number_list.rs`
- `src/protocol_serde/shape_stale_tag_exception.rs`
- `src/protocol_serde/shape_subscribe.rs`
- `src/protocol_serde/shape_subscribe_input.rs`
- `src/protocol_serde/shape_subscription.rs`
- `src/protocol_serde/shape_subscription_attributes_map.rs`
- `src/protocol_serde/shape_subscription_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_subscriptions_list.rs`
- `src/protocol_serde/shape_tag.rs`
- `src/protocol_serde/shape_tag_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_tag_list.rs`
- `src/protocol_serde/shape_tag_policy_exception.rs`
- `src/protocol_serde/shape_tag_resource.rs`
- `src/protocol_serde/shape_tag_resource_input.rs`
- `src/protocol_serde/shape_throttled_exception.rs`
- `src/protocol_serde/shape_too_many_entries_in_batch_request_exception.rs`
- `src/protocol_serde/shape_topic.rs`
- `src/protocol_serde/shape_topic_attributes_map.rs`
- `src/protocol_serde/shape_topic_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_topics_list.rs`
- `src/protocol_serde/shape_unsubscribe.rs`
- `src/protocol_serde/shape_unsubscribe_input.rs`
- `src/protocol_serde/shape_untag_resource.rs`
- `src/protocol_serde/shape_untag_resource_input.rs`
- `src/protocol_serde/shape_user_error_exception.rs`
- `src/protocol_serde/shape_validation_exception.rs`
- `src/protocol_serde/shape_verification_exception.rs`
- `src/protocol_serde/shape_verify_sms_sandbox_phone_number.rs`
- `src/protocol_serde/shape_verify_sms_sandbox_phone_number_input.rs`
- `src/protocol_serde.rs`
- `src/rest_xml_wrapped_errors.rs`

### Rust token differences

- `src/client/create_topic.rs`
- `src/client/set_sms_attributes.rs`
- `src/client/set_topic_attributes.rs`
- `src/operation/add_permission.rs`
- `src/operation/check_if_phone_number_is_opted_out.rs`
- `src/operation/confirm_subscription.rs`
- `src/operation/create_platform_application.rs`
- `src/operation/create_platform_endpoint.rs`
- `src/operation/create_sms_sandbox_phone_number/builders.rs`
- `src/operation/create_sms_sandbox_phone_number.rs`
- `src/operation/create_topic.rs`
- `src/operation/delete_endpoint.rs`
- `src/operation/delete_platform_application.rs`
- `src/operation/delete_sms_sandbox_phone_number/builders.rs`
- `src/operation/delete_sms_sandbox_phone_number.rs`
- `src/operation/delete_topic.rs`
- `src/operation/get_data_protection_policy.rs`
- `src/operation/get_endpoint_attributes.rs`
- `src/operation/get_platform_application_attributes.rs`
- `src/operation/get_sms_attributes/builders.rs`
- `src/operation/get_sms_attributes.rs`
- `src/operation/get_sms_sandbox_account_status/builders.rs`
- `src/operation/get_sms_sandbox_account_status.rs`
- `src/operation/get_subscription_attributes.rs`
- `src/operation/get_topic_attributes.rs`
- `src/operation/list_endpoints_by_platform_application.rs`
- `src/operation/list_origination_numbers.rs`
- `src/operation/list_phone_numbers_opted_out/_list_phone_numbers_opted_out_output.rs`
- `src/operation/list_phone_numbers_opted_out.rs`
- `src/operation/list_platform_applications.rs`
- `src/operation/list_sms_sandbox_phone_numbers/builders.rs`
- `src/operation/list_sms_sandbox_phone_numbers.rs`
- `src/operation/list_subscriptions.rs`
- `src/operation/list_subscriptions_by_topic.rs`
- `src/operation/list_tags_for_resource.rs`
- `src/operation/list_topics.rs`
- `src/operation/opt_in_phone_number.rs`
- `src/operation/publish.rs`
- `src/operation/publish_batch.rs`
- `src/operation/put_data_protection_policy.rs`
- `src/operation/remove_permission.rs`
- `src/operation/set_endpoint_attributes.rs`
- `src/operation/set_platform_application_attributes.rs`
- `src/operation/set_sms_attributes/_set_sms_attributes_input.rs`
- `src/operation/set_sms_attributes/builders.rs`
- `src/operation/set_sms_attributes.rs`
- `src/operation/set_subscription_attributes.rs`
- `src/operation/set_topic_attributes.rs`
- `src/operation/subscribe/_subscribe_input.rs`
- `src/operation/subscribe.rs`
- `src/operation/tag_resource.rs`
- `src/operation/unsubscribe.rs`
- `src/operation/untag_resource.rs`
- `src/operation/verify_sms_sandbox_phone_number/builders.rs`
- `src/operation/verify_sms_sandbox_phone_number.rs`
- `src/types/error/_kms_access_denied_exception.rs`
- `src/types/error/_kms_disabled_exception.rs`
- `src/types/error/_kms_invalid_state_exception.rs`
- `src/types/error/_kms_not_found_exception.rs`
- `src/types/error/_kms_opt_in_required.rs`
- `src/types/error/_kms_throttling_exception.rs`
- `src/types/error/_validation_exception.rs`
- `src/types/error/_verification_exception.rs`
