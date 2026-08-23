# AWS SDK Conformance Report: sns

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sns
**Progress:** `449/449` files compared · `223` matched · `69` mismatches · `157` missing · `0` extra · `49.67%` match (100.00% means fully matched)

### `src/client/create_platform_application.rs`

```diff
--- reference/src/client/create_platform_application.rs
+++ generated/src/client/create_platform_application.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`name(impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_name):<br>required: **true**<br><p>Application names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, hyphens, and periods, and must be between 1 and 256 characters long.</p><br>
     ///   - [`platform(impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::platform) / [`set_platform(Option<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_platform):<br>required: **true**<br><p>The following platforms are supported: ADM (Amazon Device Messaging), APNS (Apple Push Notification Service), APNS_SANDBOX, and GCM (Firebase Cloud Messaging).</p><br>
-    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_attributes):<br>required: **true**<br><p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html"> <code>SetPlatformApplicationAttributes</code> </a>.</p><br>
+    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_attributes):<br>required: **true**<br><p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html"> <code>SetPlatformApplicationAttributes</code></a>.</p><br>
     /// - On success, responds with [`CreatePlatformApplicationOutput`](crate::operation::create_platform_application::CreatePlatformApplicationOutput) with field(s):
     ///   - [`platform_application_arn(Option<String>)`](crate::operation::create_platform_application::CreatePlatformApplicationOutput::platform_application_arn): <p><code>PlatformApplicationArn</code> is returned.</p>
     /// - On failure, responds with [`SdkError<CreatePlatformApplicationError>`](crate::operation::create_platform_application::CreatePlatformApplicationError)
```

### `src/client/create_platform_endpoint.rs`

```diff
--- reference/src/client/create_platform_endpoint.rs
+++ generated/src/client/create_platform_endpoint.rs
@@ -6,7 +6,7 @@
     ///   - [`platform_application_arn(impl Into<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::platform_application_arn) / [`set_platform_application_arn(Option<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::set_platform_application_arn):<br>required: **true**<br><p><code>PlatformApplicationArn</code> returned from CreatePlatformApplication is used to create a an endpoint.</p><br>
     ///   - [`token(impl Into<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::token) / [`set_token(Option<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::set_token):<br>required: **true**<br><p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token equivalent is called the registration ID.</p><br>
     ///   - [`custom_user_data(impl Into<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::custom_user_data) / [`set_custom_user_data(Option<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::set_custom_user_data):<br>required: **false**<br><p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p><br>
-    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::set_attributes):<br>required: **false**<br><p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html"> <code>SetEndpointAttributes</code> </a>.</p><br>
+    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointFluentBuilder::set_attributes):<br>required: **false**<br><p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html"> <code>SetEndpointAttributes</code></a>.</p><br>
     /// - On success, responds with [`CreatePlatformEndpointOutput`](crate::operation::create_platform_endpoint::CreatePlatformEndpointOutput) with field(s):
     ///   - [`endpoint_arn(Option<String>)`](crate::operation::create_platform_endpoint::CreatePlatformEndpointOutput::endpoint_arn): <p>EndpointArn returned from CreateEndpoint action.</p>
     /// - On failure, responds with [`SdkError<CreatePlatformEndpointError>`](crate::operation::create_platform_endpoint::CreatePlatformEndpointError)
```

### `src/client/create_topic.rs`

```diff
--- reference/src/client/create_topic.rs
+++ generated/src/client/create_topic.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`name(impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_name):<br>required: **true**<br><p>The name of the topic you want to create.</p> <p>Constraints: Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long.</p> <p>For a FIFO (first-in-first-out) topic, the name must end with the <code>.fifo</code> suffix.</p><br>
-    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_attributes):<br>required: **false**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists names, descriptions, and values of the special request parameters that the <code>CreateTopic</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul><note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>    <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>   </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul><note>  <p>The <endpoint>    SuccessFeedbackRoleArn and     <endpoint>     FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The      <endpoint>      SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the       <endpoint>       FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.      </endpoint>     </endpoint>    </endpoint>   </endpoint></p> </note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li> </ul> <ul>  <li>   <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>   <ul>    <li>     <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>    <li>     <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>   </ul></li> </ul><br>
+    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_attributes):<br>required: **false**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists names, descriptions, and values of the special request parameters that the <code>CreateTopic</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul> <note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p> <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p> </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul> <note>  <p>The <endpoint>SuccessFeedbackRoleArn and <endpoint>FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The <endpoint>SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the <endpoint>FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.</p></note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side encryption</a>:</p> <ul>        <li>         <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>       </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>        <li>         <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>        <li>         <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>         <ul>          <li>           <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>          <li>           <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>           <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>         </ul></li>       </ul> <ul>        <li>         <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>         <ul>          <li>           <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>          <li>           <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>         </ul></li>       </ul><br>
     ///   - [`tags(Tag)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_tags):<br>required: **false**<br><p>The list of tags to add to a new topic.</p><note>  <p>To be able to tag a topic on creation, you must have the <code>sns:CreateTopic</code> and <code>sns:TagResource</code> permissions.</p> </note><br>
     ///   - [`data_protection_policy(impl Into<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::data_protection_policy) / [`set_data_protection_policy(Option<String>)`](crate::operation::create_topic::builders::CreateTopicFluentBuilder::set_data_protection_policy):<br>required: **false**<br><p>The body of the policy document you want to use for this topic.</p> <p>You can only add one policy per topic.</p> <p>The policy must be in JSON string format.</p> <p>Length Constraints: Maximum length of 30,720.</p><br>
     /// - On success, responds with [`CreateTopicOutput`](crate::operation::create_topic::CreateTopicOutput) with field(s):
```

### `src/client/get_endpoint_attributes.rs`

```diff
--- reference/src/client/get_endpoint_attributes.rs
+++ generated/src/client/get_endpoint_attributes.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`endpoint_arn(impl Into<String>)`](crate::operation::get_endpoint_attributes::builders::GetEndpointAttributesFluentBuilder::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::operation::get_endpoint_attributes::builders::GetEndpointAttributesFluentBuilder::set_endpoint_arn):<br>required: **true**<br><p><code>EndpointArn</code> for <code>GetEndpointAttributes</code> input.</p><br>
     /// - On success, responds with [`GetEndpointAttributesOutput`](crate::operation::get_endpoint_attributes::GetEndpointAttributesOutput) with field(s):
-    ///   - [`attributes(Option<HashMap::<String, String>>)`](crate::operation::get_endpoint_attributes::GetEndpointAttributesOutput::attributes): <p>Attributes include the following:</p> <ul>  <li>   <p><code>CustomUserData</code> – arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p></li>  <li>   <p><code>Enabled</code> – flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p></li>  <li>   <p><code>Token</code> – device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p><note>    <p>The device token for the iOS platform is returned in lowercase.</p>   </note></li> </ul>
+    ///   - [`attributes(Option<HashMap::<String, String>>)`](crate::operation::get_endpoint_attributes::GetEndpointAttributesOutput::attributes): <p>Attributes include the following:</p> <ul>  <li>   <p><code>CustomUserData</code> – arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p></li>  <li>   <p><code>Enabled</code> – flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p></li>  <li>   <p><code>Token</code> – device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p><note>    <p>The device token for the iOS platform is returned in lowercase.</p> </note></li> </ul>
     /// - On failure, responds with [`SdkError<GetEndpointAttributesError>`](crate::operation::get_endpoint_attributes::GetEndpointAttributesError)
     pub fn get_endpoint_attributes(&self) -> crate::operation::get_endpoint_attributes::builders::GetEndpointAttributesFluentBuilder {
         crate::operation::get_endpoint_attributes::builders::GetEndpointAttributesFluentBuilder::new(self.handle.clone())
```

### `src/client/get_topic_attributes.rs`

```diff
--- reference/src/client/get_topic_attributes.rs
+++ generated/src/client/get_topic_attributes.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`topic_arn(impl Into<String>)`](crate::operation::get_topic_attributes::builders::GetTopicAttributesFluentBuilder::topic_arn) / [`set_topic_arn(Option<String>)`](crate::operation::get_topic_attributes::builders::GetTopicAttributesFluentBuilder::set_topic_arn):<br>required: **true**<br><p>The ARN of the topic whose properties you want to get.</p><br>
     /// - On success, responds with [`GetTopicAttributesOutput`](crate::operation::get_topic_attributes::GetTopicAttributesOutput) with field(s):
-    ///   - [`attributes(Option<HashMap::<String, String>>)`](crate::operation::get_topic_attributes::GetTopicAttributesOutput::attributes): <p>A map of the topic's attributes. Attributes in this map include the following:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The JSON serialization of the topic's delivery policy.</p></li>  <li>   <p><code>DisplayName</code> – The human-readable name used in the <code>From</code> field for notifications to <code>email</code> and <code>email-json</code> endpoints.</p></li>  <li>   <p><code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy, taking system defaults into account.</p></li>  <li>   <p><code>Owner</code> – The Amazon Web Services account ID of the topic's owner.</p></li>  <li>   <p><code>Policy</code> – The JSON serialization of the topic's access control policy.</p></li>  <li>   <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.</p>   <ul>    <li>     <p>By default, <code>SignatureVersion</code> is set to <b>1</b>. The signature is a Base64-encoded <b>SHA1withRSA</b> signature.</p></li>    <li>     <p>When you set <code>SignatureVersion</code> to <b>2</b>. Amazon SNS uses a Base64-encoded <b>SHA256withRSA</b> signature.</p><note>      <p>If the API response does not include the <code>SignatureVersion</code> attribute, it means that the <code>SignatureVersion</code> for the topic has value <b>1</b>.</p>     </note></li>   </ul></li>  <li>   <p><code>SubscriptionsConfirmed</code> – The number of confirmed subscriptions for the topic.</p></li>  <li>   <p><code>SubscriptionsDeleted</code> – The number of deleted subscriptions for the topic.</p></li>  <li>   <p><code>SubscriptionsPending</code> – The number of subscriptions pending confirmation for the topic.</p></li>  <li>   <p><code>TopicArn</code> – The topic's ARN.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> - The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>BeginningArchiveTime</code> – The earliest starting point at which a message in the topic’s archive can be replayed from. This point in time is based on the configured message retention period set by the topic’s message archiving policy.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li>  <li>   <p><code>FifoTopic</code> – When this is set to <code>true</code>, a FIFO topic is created.</p></li> </ul>
+    ///   - [`attributes(Option<HashMap::<String, String>>)`](crate::operation::get_topic_attributes::GetTopicAttributesOutput::attributes): <p>A map of the topic's attributes. Attributes in this map include the following:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The JSON serialization of the topic's delivery policy.</p></li>  <li>   <p><code>DisplayName</code> – The human-readable name used in the <code>From</code> field for notifications to <code>email</code> and <code>email-json</code> endpoints.</p></li>  <li>   <p><code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy, taking system defaults into account.</p></li>  <li>   <p><code>Owner</code> – The Amazon Web Services account ID of the topic's owner.</p></li>  <li>   <p><code>Policy</code> – The JSON serialization of the topic's access control policy.</p></li>  <li>   <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.</p>   <ul>    <li>     <p>By default, <code>SignatureVersion</code> is set to <b>1</b>. The signature is a Base64-encoded <b>SHA1withRSA</b> signature.</p></li>    <li>     <p>When you set <code>SignatureVersion</code> to <b>2</b>. Amazon SNS uses a Base64-encoded <b>SHA256withRSA</b> signature.</p><note>      <p>If the API response does not include the <code>SignatureVersion</code> attribute, it means that the <code>SignatureVersion</code> for the topic has value <b>1</b>.</p> </note></li>   </ul></li>  <li>   <p><code>SubscriptionsConfirmed</code> – The number of confirmed subscriptions for the topic.</p></li>  <li>   <p><code>SubscriptionsDeleted</code> – The number of deleted subscriptions for the topic.</p></li>  <li>   <p><code>SubscriptionsPending</code> – The number of subscriptions pending confirmation for the topic.</p></li>  <li>   <p><code>TopicArn</code> – The topic's ARN.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul>  <li>   <p><code>KmsMasterKeyId</code> - The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li> </ul> <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>  <li>   <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>  <li>   <p><code>BeginningArchiveTime</code> – The earliest starting point at which a message in the topic’s archive can be replayed from. This point in time is based on the configured message retention period set by the topic’s message archiving policy.</p></li>  <li>   <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>   <ul>    <li>     <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>    <li>     <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>     <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>   </ul></li>  <li>   <p><code>FifoTopic</code> – When this is set to <code>true</code>, a FIFO topic is created.</p></li> </ul>
     /// - On failure, responds with [`SdkError<GetTopicAttributesError>`](crate::operation::get_topic_attributes::GetTopicAttributesError)
     pub fn get_topic_attributes(&self) -> crate::operation::get_topic_attributes::builders::GetTopicAttributesFluentBuilder {
         crate::operation::get_topic_attributes::builders::GetTopicAttributesFluentBuilder::new(self.handle.clone())
```

### `src/client/publish.rs`

```diff
--- reference/src/client/publish.rs
+++ generated/src/client/publish.rs
@@ -10,8 +10,8 @@
     ///   - [`subject(impl Into<String>)`](crate::operation::publish::builders::PublishFluentBuilder::subject) / [`set_subject(Option<String>)`](crate::operation::publish::builders::PublishFluentBuilder::set_subject):<br>required: **false**<br><p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p> <p>Constraints: Subjects must be UTF-8 text with no line breaks or control characters, and less than 100 characters long.</p><br>
     ///   - [`message_structure(impl Into<String>)`](crate::operation::publish::builders::PublishFluentBuilder::message_structure) / [`set_message_structure(Option<String>)`](crate::operation::publish::builders::PublishFluentBuilder::set_message_structure):<br>required: **false**<br><p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p> <ul>  <li>   <p>be a syntactically valid JSON object; and</p></li>  <li>   <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li> </ul> <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p> <p>Valid value: <code>json</code></p><br>
     ///   - [`message_attributes(impl Into<String>, MessageAttributeValue)`](crate::operation::publish::builders::PublishFluentBuilder::message_attributes) / [`set_message_attributes(Option<HashMap::<String, MessageAttributeValue>>)`](crate::operation::publish::builders::PublishFluentBuilder::set_message_attributes):<br>required: **false**<br><p>Message attributes for Publish action.</p><br>
-    ///   - [`message_deduplication_id(impl Into<String>)`](crate::operation::publish::builders::PublishFluentBuilder::message_deduplication_id) / [`set_message_deduplication_id(Option<String>)`](crate::operation::publish::builders::PublishFluentBuilder::set_message_deduplication_id):<br>required: **false**<br><ul>  <li>   <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p></li>  <li>   <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages within the 5 minute minimum deduplication interval.</p></li>  <li>   <p>The scope of deduplication depends on the <code>FifoThroughputScope</code> attribute, when set to <code>Topic</code> the message deduplication scope is across the entire topic, when set to <code>MessageGroup</code> the message deduplication scope is within each individual message group.</p></li>  <li>   <p>If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, subsequent messages within the deduplication scope and interval, with the same <code>MessageDeduplicationId</code>, are accepted successfully but aren't delivered.</p></li>  <li>   <p>Every message must have a unique <code>MessageDeduplicationId</code>:</p>   <ul>    <li>     <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>    <li>     <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your topic, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>    <li>     <p>If you don't provide a <code>MessageDeduplicationId</code> and the topic doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>    <li>     <p>If the topic has a <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>   </ul></li>  <li>   <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication scope and interval are treated as duplicates and only one copy of the message is delivered.</p></li>  <li>   <p>If you send one message with <code>ContentBasedDeduplication</code> enabled, and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates, within the deduplication scope and interval, and only one copy of the message is delivered.</p></li> </ul><br>
-    ///   - [`message_group_id(impl Into<String>)`](crate::operation::publish::builders::PublishFluentBuilder::message_group_id) / [`set_message_group_id(Option<String>)`](crate::operation::publish::builders::PublishFluentBuilder::set_message_group_id):<br>required: **false**<br><p>The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p> <p>For FIFO topics: The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p> <p>For standard topics: The <code>MessageGroupId</code> is optional and is forwarded only to Amazon SQS standard subscriptions to activate <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-fair-queues.html">fair queues</a>. The <code>MessageGroupId</code> is not used for, or sent to, any other endpoint types. When provided, the same validation rules apply as for FIFO topics.</p><br>
+    ///   - [`message_deduplication_id(impl Into<String>)`](crate::operation::publish::builders::PublishFluentBuilder::message_deduplication_id) / [`set_message_deduplication_id(Option<String>)`](crate::operation::publish::builders::PublishFluentBuilder::set_message_deduplication_id):<br>required: **false**<br><ul>  <li>   <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p></li>  <li>   <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages within the 5 minute minimum deduplication interval.</p></li>  <li>   <p>The scope of deduplication depends on the <code>FifoThroughputScope</code> attribute, when set to <code>Topic</code> the message deduplication scope is across the entire topic, when set to <code>MessageGroup</code> the message deduplication scope is within each individual message group.</p></li>  <li>   <p>If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, subsequent messages within the deduplication scope and interval, with the same <code>MessageDeduplicationId</code>, are accepted successfully but aren't delivered.</p></li>  <li>   <p>Every message must have a unique <code>MessageDeduplicationId</code>:</p>   <ul>    <li>     <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>    <li>     <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your topic, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>    <li>     <p>If you don't provide a <code>MessageDeduplicationId</code> and the topic doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>    <li>     <p>If the topic has a <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>   </ul></li>  <li>   <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication scope and interval are treated as duplicates and only one copy of the message is delivered.</p></li>  <li>   <p>If you send one message with <code>ContentBasedDeduplication</code> enabled, and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates, within the deduplication scope and interval, and only one copy of the message is delivered.</p></li> </ul><br>
+    ///   - [`message_group_id(impl Into<String>)`](crate::operation::publish::builders::PublishFluentBuilder::message_group_id) / [`set_message_group_id(Option<String>)`](crate::operation::publish::builders::PublishFluentBuilder::set_message_group_id):<br>required: **false**<br><p>The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p> <p>For FIFO topics: The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p> <p>For standard topics: The <code>MessageGroupId</code> is optional and is forwarded only to Amazon SQS standard subscriptions to activate <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-fair-queues.html">fair queues</a>. The <code>MessageGroupId</code> is not used for, or sent to, any other endpoint types. When provided, the same validation rules apply as for FIFO topics.</p><br>
     /// - On success, responds with [`PublishOutput`](crate::operation::publish::PublishOutput) with field(s):
     ///   - [`message_id(Option<String>)`](crate::operation::publish::PublishOutput::message_id): <p>Unique identifier assigned to the published message.</p> <p>Length Constraint: Maximum 100 characters</p>
     ///   - [`sequence_number(Option<String>)`](crate::operation::publish::PublishOutput::sequence_number): <p>This response element applies only to FIFO (first-in-first-out) topics.</p> <p>The sequence number is a large, non-consecutive number that Amazon SNS assigns to each message. The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for each <code>MessageGroupId</code>.</p>
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
+    ///   - [`attribute_name(impl Into<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::attribute_name) / [`set_attribute_name(Option<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::set_attribute_name):<br>required: **true**<br><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p> <ul>  <li>   <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>  <li>   <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>  <li>   <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>  <li>   <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>  <li>   <p>HTTP</p>   <ul>    <li>     <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>    <li>     <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>   </ul></li>  <li>   <p>Amazon Data Firehose</p>   <ul>    <li>     <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>    <li>     <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Data Firehose endpoint.</p></li>   </ul></li>  <li>   <p>Lambda</p>   <ul>    <li>     <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>    <li>     <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>   </ul></li>  <li>   <p>Platform application endpoint</p>   <ul>    <li>     <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>    <li>     <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an platform application endpoint.</p></li>   </ul> <note>    <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p> <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p> </note></li>  <li>   <p>Amazon SQS</p>   <ul>    <li>     <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>    <li>     <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>   </ul></li> </ul> <note>  <p>The <endpoint>SuccessFeedbackRoleArn and <endpoint>FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The <endpoint>SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the <endpoint>FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.</p></note> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul>        <li>         <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>        <li>         <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS. By default, <code>SignatureVersion</code> is set to <code>1</code>.</p></li>       </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p> <ul>        <li>         <p><code>ArchivePolicy</code> – The policy that sets the retention period for messages stored in the message archive of an Amazon SNS FIFO topic.</p></li>        <li>         <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>         <ul>          <li>           <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>          <li>           <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>           <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>         </ul></li>       </ul> <ul>        <li>         <p><code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>         <ul>          <li>           <p><code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p></li>          <li>           <p><code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p></li>         </ul></li>       </ul><br>
     ///   - [`attribute_value(impl Into<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::attribute_value) / [`set_attribute_value(Option<String>)`](crate::operation::set_topic_attributes::builders::SetTopicAttributesFluentBuilder::set_attribute_value):<br>required: **false**<br><p>The new value for the attribute.</p><br>
     /// - On success, responds with [`SetTopicAttributesOutput`](crate::operation::set_topic_attributes::SetTopicAttributesOutput)
     /// - On failure, responds with [`SdkError<SetTopicAttributesError>`](crate::operation::set_topic_attributes::SetTopicAttributesError)
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -1,1719 +1,45 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#![allow(clippy::empty_line_after_doc_comments)]
-/// Configuration for a aws_sdk_sns service client.
-///
-/// Service configuration allows for customization of endpoints, region, credentials providers,
-/// and retry configuration. Generally, it is constructed automatically for you from a shared
-/// configuration loaded by the `aws-config` crate. For example:
-///
-/// ```ignore
-/// // Load a shared config from the environment
-/// let shared_config = aws_config::from_env().load().await;
-/// // The client constructor automatically converts the shared config into the service config
-/// let client = Client::new(&shared_config);
-/// ```
-///
-/// The service config can also be constructed manually using its builder.
-///
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
+
+#[derive(Clone, Debug)]
 pub struct Config {
-    // Both `config` and `cloneable` are the same config, but the cloneable one
-    // is kept around so that it is possible to convert back into a builder. This can be
-    // optimized in the future.
-    pub(crate) config: crate::config::FrozenLayer,
-    cloneable: ::aws_smithy_types::config_bag::CloneableLayer,
-    pub(crate) runtime_components: crate::config::RuntimeComponentsBuilder,
-    pub(crate) runtime_plugins: ::std::vec::Vec<crate::config::SharedRuntimePlugin>,
-    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
+    pub(crate) endpoint_url: ::std::string::String,
 }
-impl Config {
-    ///
-    /// Constructs a config builder.
-    /// <div class="warning">
-    /// Note that a config created from this builder will not have the same safe defaults as one created by
-    /// the <a href="https://crates.io/crates/aws-config" target="_blank">aws-config</a> crate.
-    /// </div>
-    ///
-    pub fn builder() -> Builder {
-        Builder::default()
-    }
-    /// Converts this config back into a builder so that it can be tweaked.
-    pub fn to_builder(&self) -> Builder {
-        Builder {
-            config: self.cloneable.clone(),
-            runtime_components: self.runtime_components.clone(),
-            runtime_plugins: self.runtime_plugins.clone(),
-            behavior_version: self.behavior_version,
-        }
-    }
-    /// Return a reference to the stalled stream protection configuration contained in this config, if any.
-    pub fn stalled_stream_protection(&self) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
-        self.config.load::<crate::config::StalledStreamProtectionConfig>()
-    }
-    /// Return the [`SharedHttpClient`](crate::config::SharedHttpClient) to use when making requests, if any.
-    pub fn http_client(&self) -> Option<crate::config::SharedHttpClient> {
-        self.runtime_components.http_client()
-    }
-    /// Return the auth schemes configured on this service config
-    pub fn auth_schemes(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
-        self.runtime_components.auth_schemes()
-    }

-    /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
-        self.runtime_components.auth_scheme_option_resolver()
-    }
-    /// Returns the configured auth scheme preference
-    pub fn auth_scheme_preference(&self) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
-        self.config.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
-    }
-    /// Returns the endpoint resolver.
-    pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
-    }
-    /// Return a reference to the retry configuration contained in this config, if any.
-    pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
-        self.config.load::<::aws_smithy_types::retry::RetryConfig>()
-    }
-
-    /// Return a cloned shared async sleep implementation from this config, if any.
-    pub fn sleep_impl(&self) -> ::std::option::Option<crate::config::SharedAsyncSleep> {
-        self.runtime_components.sleep_impl()
-    }
-
-    /// Return a reference to the timeout configuration contained in this config, if any.
-    pub fn timeout_config(&self) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
-        self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>()
-    }
-
-    /// Returns a reference to the retry partition contained in this config, if any.
-    ///
-    /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
-    /// method for anything!
-    pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
-    }
-    /// Returns the configured identity cache for auth.
-    pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
-        self.runtime_components.identity_cache()
-    }
-    /// Returns interceptors currently registered by the user.
-    pub fn interceptors(&self) -> impl Iterator<Item = crate::config::SharedInterceptor> + '_ {
-        self.runtime_components.interceptors()
-    }
-    /// Return time source used for this service.
-    pub fn time_source(&self) -> ::std::option::Option<::aws_smithy_async::time::SharedTimeSource> {
-        self.runtime_components.time_source()
-    }
-    /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
-        self.runtime_components.retry_classifiers()
-    }
-    /// Returns the name of the app that is using the client, if it was provided.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn app_name(&self) -> ::std::option::Option<&::aws_types::app_name::AppName> {
-        self.config.load::<::aws_types::app_name::AppName>()
-    }
-    /// Returns the framework metadata that has been configured, if any.
-    ///
-    /// This _optional_ metadata identifies software frameworks or third-party libraries
-    /// being used with the client, rendered into the user agent as `lib/{name}/{version}`.
-    /// Entries are returned in first-seen (insertion) order, matching the order they are
-    /// rendered into the user agent.
-    pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
-        // `StoreAppend` loads entries newest-first; reverse to first-seen order so
-        // this getter agrees with both the user agent and `SdkConfig::framework_metadata`.
-        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> =
-            self.config.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>().collect();
-        entries.reverse();
-        entries
-    }
-    /// Returns the invocation ID generator if one was given in config.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(&self) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
-        self.config.load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>().cloned()
-    }
-    /// Creates a new [service config](crate::Config) from a [shared `config`](::aws_types::sdk_config::SdkConfig).
-    pub fn new(config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Builder::from(config).build()
-    }
-    /// The signature version 4 service signing name to use in the credential scope when signing requests.
-    ///
-    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
-    /// [`SigningName`](aws_types::SigningName) during operation construction
-    pub fn signing_name(&self) -> &'static str {
-        "sns"
-    }
-    /// Returns the AWS region, if it was provided.
-    pub fn region(&self) -> ::std::option::Option<&crate::config::Region> {
-        self.config.load::<crate::config::Region>()
-    }
-    /// This function was intended to be removed, and has been broken since release-2023-11-15 as it always returns a `None`. Do not use.
-    #[deprecated(
-        note = "This function was intended to be removed, and has been broken since release-2023-11-15 as it always returns a `None`. Do not use."
-    )]
-    pub fn credentials_provider(&self) -> Option<crate::config::SharedCredentialsProvider> {
-        ::std::option::Option::None
-    }
-}
-/// Builder for creating a `Config`.
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct Builder {
-    pub(crate) config: ::aws_smithy_types::config_bag::CloneableLayer,
-    pub(crate) runtime_components: crate::config::RuntimeComponentsBuilder,
-    pub(crate) runtime_plugins: ::std::vec::Vec<crate::config::SharedRuntimePlugin>,
-    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
-}
-impl ::std::default::Default for Builder {
+impl ::std::default::Default for Config {
     fn default() -> Self {
         Self {
-            config: ::std::default::Default::default(),
-            runtime_components: crate::config::RuntimeComponentsBuilder::new("service config"),
-            runtime_plugins: ::std::default::Default::default(),
-            behavior_version: ::std::default::Default::default(),
+            endpoint_url: ::std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localhost:4566".to_owned()),
         }
     }
 }
-impl Builder {
-    ///
-    /// Constructs a config builder.
-    /// <div class="warning">
-    /// Note that a config created from this builder will not have the same safe defaults as one created by
-    /// the <a href="https://crates.io/crates/aws-config" target="_blank">aws-config</a> crate.
-    /// </div>
-    ///
-    pub fn new() -> Self {
-        Self::default()
-    }
-    /// Constructs a config builder from the given `config_bag`, setting only fields stored in the config bag,
-    /// but not those in runtime components.
-    #[allow(unused)]
-    pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
-        let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
-        builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
-        builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
-        builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
-        for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-        builder.set_endpoint_url(config_bag.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()));
-        builder.set_use_dual_stack(config_bag.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0));
-        builder.set_use_fips(config_bag.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0));
-        builder.set_region(config_bag.load::<crate::config::Region>().cloned());
-        builder
-    }
-    /// Names operation-input members whose values are captured *and* emitted as
-    /// attributes on the client's built-in metrics (e.g. `["Bucket"]`).
-    ///
-    /// Emitting implies capture, so an emitted member is also readable in-process
-    /// via `CapturedTelemetryAttributes` on the config bag. Names are Smithy input
-    /// member names; only string-valued, non-sensitive members are eligible, and
-    /// naming any other member has no effect. Off by default.
-    ///
-    /// Prefer bounded identifiers here: an emitted member becomes a metric label, so
-    /// high-cardinality values (like object keys) fragment the metrics and inflate
-    /// cost. Use [`Self::capture_input_attributes`] for values you want to read
-    /// in-process without emitting them on the metrics.
-    pub fn emit_input_attributes(mut self, names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>) -> Self {
-        let mut requested = self
-            .config
-            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
-            .cloned()
-            .unwrap_or_default();
-        requested.emit(names.into_iter().map(|n| n.into()));
-        self.config.store_put(requested);
-        self
-    }
-
-    /// Names operation-input members whose values are captured into
-    /// `CapturedTelemetryAttributes` for in-process reads (e.g. from a custom
-    /// interceptor), but are **not** emitted on the built-in metrics.
-    ///
-    /// Use this for values you need during the operation lifecycle but do not want on
-    /// the metric label set (for example, high-cardinality identifiers). Names follow
-    /// the same eligibility rules as [`Self::emit_input_attributes`]. Off by default.
-    pub fn capture_input_attributes(
-        mut self,
-        names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>,
-    ) -> Self {
-        let mut requested = self
-            .config
-            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
-            .cloned()
-            .unwrap_or_default();
-        requested.capture_only(names.into_iter().map(|n| n.into()));
-        self.config.store_put(requested);
-        self
-    }
-    /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
-    /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
-        self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
-        self
-    }
-    /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
-    /// to configure protection for stalled streams.
-    pub fn set_stalled_stream_protection(
-        &mut self,
-        stalled_stream_protection_config: ::std::option::Option<crate::config::StalledStreamProtectionConfig>,
-    ) -> &mut Self {
-        self.config.store_or_unset(stalled_stream_protection_config);
-        self
-    }
-    /// Sets the HTTP client to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # #[cfg(test)]
-    /// # mod tests {
-    /// # #[test]
-    /// # fn example() {
-    /// use std::time::Duration;
-    /// use aws_sdk_sns::config::Config;
-    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
-    ///
-    /// let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
-    ///     .with_webpki_roots()
-    ///     .https_only()
-    ///     .enable_http1()
-    ///     .enable_http2()
-    ///     .build();
-    /// let hyper_client = HyperClientBuilder::new().build(https_connector);
-    ///
-    /// // This connector can then be given to a generated service Config
-    /// let config = my_service_client::Config::builder()
-    ///     .endpoint_url("https://example.com")
-    ///     .http_client(hyper_client)
-    ///     .build();
-    /// let client = my_service_client::Client::from_conf(config);
-    /// # }
-    /// # }
-    /// ```
-    pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
-        self
-    }

-    /// Sets the HTTP client to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # #[cfg(test)]
-    /// # mod tests {
-    /// # #[test]
-    /// # fn example() {
-    /// use std::time::Duration;
-    /// use aws_sdk_sns::config::{Builder, Config};
-    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
-    ///
-    /// fn override_http_client(builder: &mut Builder) {
-    ///     let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
-    ///         .with_webpki_roots()
-    ///         .https_only()
-    ///         .enable_http1()
-    ///         .enable_http2()
-    ///         .build();
-    ///     let hyper_client = HyperClientBuilder::new().build(https_connector);
-    ///     builder.set_http_client(Some(hyper_client));
-    /// }
-    ///
-    /// let mut builder = aws_sdk_sns::Config::builder();
-    /// override_http_client(&mut builder);
-    /// let config = builder.build();
-    /// # }
-    /// # }
-    /// ```
-    pub fn set_http_client(&mut self, http_client: Option<crate::config::SharedHttpClient>) -> &mut Self {
-        self.runtime_components.set_http_client(http_client);
-        self
+pub mod config {
+    #[derive(Clone, Debug, Default)]
+    pub struct Builder {
+        endpoint_url: ::std::option::Option<::std::string::String>,
     }
-    /// Adds an auth scheme to the builder
-    ///
-    /// If `auth_scheme` has an existing [AuthSchemeId](aws_smithy_runtime_api::client::auth::AuthSchemeId) in the runtime, the current identity
-    /// resolver and signer for that scheme will be replaced by those from `auth_scheme`.
-    ///
-    /// _Important:_ When introducing a custom auth scheme, ensure you override either
-    /// [`Self::auth_scheme_resolver`] or [`Self::set_auth_scheme_resolver`]
-    /// so that the custom auth scheme is included in the list of resolved auth scheme options.
-    /// [The default auth scheme resolver](crate::config::auth::DefaultAuthSchemeResolver) will not recognize your custom auth scheme.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::{
-    /// #     box_error::BoxError,
-    /// #     client::{
-    /// #         auth::{
-    /// #             AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, AuthSchemeOption,
-    /// #             AuthSchemeOptionsFuture, Sign,
-    /// #         },
-    /// #         identity::{Identity, IdentityFuture, ResolveIdentity, SharedIdentityResolver},
-    /// #         orchestrator::HttpRequest,
-    /// #         runtime_components::{GetIdentityResolver, RuntimeComponents},
-    /// #   },
-    /// #   shared::IntoShared,
-    /// # };
-    /// # use aws_smithy_types::config_bag::ConfigBag;
-    /// // Auth scheme with customer identity resolver and signer
-    /// #[derive(Debug)]
-    /// struct CustomAuthScheme {
-    ///     id: AuthSchemeId,
-    ///     identity_resolver: SharedIdentityResolver,
-    ///     signer: CustomSigner,
-    /// }
-    /// impl Default for CustomAuthScheme {
-    ///     fn default() -> Self {
-    ///         Self {
-    ///             id: AuthSchemeId::new("custom"),
-    ///             identity_resolver: CustomIdentityResolver.into_shared(),
-    ///             signer: CustomSigner,
-    ///         }
-    ///     }
-    /// }
-    /// impl AuthScheme for CustomAuthScheme {
-    ///     fn scheme_id(&self) -> AuthSchemeId {
-    ///         self.id.clone()
-    ///     }
-    ///     fn identity_resolver(
-    ///         &self,
-    ///         _identity_resolvers: &dyn GetIdentityResolver,
-    ///     ) -> Option<SharedIdentityResolver> {
-    ///         Some(self.identity_resolver.clone())
-    ///     }
-    ///     fn signer(&self) -> &dyn Sign {
-    ///         &self.signer
-    ///     }
-    /// }
-    ///
-    /// #[derive(Debug, Default)]
-    /// struct CustomSigner;
-    /// impl Sign for CustomSigner {
-    ///     fn sign_http_request(
-    ///         &self,
-    ///         _request: &mut HttpRequest,
-    ///         _identity: &Identity,
-    ///         _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
-    ///         _runtime_components: &RuntimeComponents,
-    ///         _config_bag: &ConfigBag,
-    ///     ) -> Result<(), BoxError> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// #[derive(Debug)]
-    /// struct CustomIdentityResolver;
-    /// impl ResolveIdentity for CustomIdentityResolver {
-    ///     fn resolve_identity<'a>(
-    ///         &'a self,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///         _config_bag: &'a ConfigBag,
-    ///     ) -> IdentityFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// // Auth scheme resolver that favors `CustomAuthScheme`
-    /// #[derive(Debug)]
-    /// struct CustomAuthSchemeResolver;
-    /// impl aws_sdk_sns::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_sns::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         AuthSchemeOptionsFuture::ready(Ok(vec![AuthSchemeOption::from(AuthSchemeId::new(
-    ///             "custom",
-    ///         ))]))
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .push_auth_scheme(CustomAuthScheme::default())
-    ///     .auth_scheme_resolver(CustomAuthSchemeResolver)
-    ///     // other configurations
-    ///     .build();
-    /// ```
-    pub fn push_auth_scheme(mut self, auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static) -> Self {
-        self.runtime_components.push_auth_scheme(auth_scheme);
-        self
-    }
-
-    /// Set the auth scheme resolver for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::{
-    /// #     client::{
-    /// #         auth::AuthSchemeOptionsFuture,
-    /// #         runtime_components::RuntimeComponents,
-    /// #   },
-    /// # };
-    /// # use aws_smithy_types::config_bag::ConfigBag;
-    /// #[derive(Debug)]
-    /// struct CustomAuthSchemeResolver;
-    /// impl aws_sdk_sns::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_sns::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .auth_scheme_resolver(CustomAuthSchemeResolver)
-    ///     // other configurations
-    ///     .build();
-    /// ```
-    pub fn auth_scheme_resolver(mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> Self {
-        self.set_auth_scheme_resolver(auth_scheme_resolver);
-        self
-    }
-
-    /// Set the auth scheme resolver for the builder
-    ///
-    /// # Examples
-    /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
-        self.runtime_components
-            .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
-        self
-    }
-
-    /// Enable no authentication regardless of what authentication mechanisms operations support
-    ///
-    /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
-    /// and the auth scheme resolver will use it when no other auth schemes are applicable.
-    pub fn allow_no_auth(mut self) -> Self {
-        self.set_allow_no_auth();
-        self
-    }
-
-    /// Enable no authentication regardless of what authentication mechanisms operations support
-    ///
-    /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
-    /// and the auth scheme resolver will use it when no other auth schemes are applicable.
-    pub fn set_allow_no_auth(&mut self) -> &mut Self {
-        self.push_runtime_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared());
-        self
-    }
-    /// Set the auth scheme preference for an auth scheme resolver
-    /// (typically the default auth scheme resolver).
-    ///
-    /// Each operation has a predefined order of auth schemes, as determined by the service,
-    /// for auth scheme resolution. By using the auth scheme preference, customers
-    /// can reorder the schemes resolved by the auth scheme resolver.
-    ///
-    /// The preference list is intended as a hint rather than a strict override.
-    /// Any schemes not present in the originally resolved auth schemes will be ignored.
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-
-    pub fn auth_scheme_preference(
-        mut self,
-        preference: impl ::std::convert::Into<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
-    ) -> Self {
-        self.set_auth_scheme_preference(::std::option::Option::Some(preference.into()));
-        self
-    }
-
-    /// Set the auth scheme preference for an auth scheme resolver
-    /// (typically the default auth scheme resolver).
-    ///
-    /// Each operation has a predefined order of auth schemes, as determined by the service,
-    /// for auth scheme resolution. By using the auth scheme preference, customers
-    /// can reorder the schemes resolved by the auth scheme resolver.
-    ///
-    /// The preference list is intended as a hint rather than a strict override.
-    /// Any schemes not present in the originally resolved auth schemes will be ignored.
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-
-    pub fn set_auth_scheme_preference(
-        &mut self,
-        preference: ::std::option::Option<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
-    ) -> &mut Self {
-        self.config.store_or_unset(preference);
-        self
-    }
-    /// Sets the endpoint resolver to use when making requests.
-    ///
-    ///
-    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
-    /// rules for `aws_sdk_sns`.
-    ///
-    ///
-    /// Note: setting an endpoint resolver will replace any endpoint URL that has been set.
-    /// This method accepts an endpoint resolver [specific to this service](crate::config::endpoint::ResolveEndpoint). If you want to
-    /// provide a shared endpoint resolver, use [`Self::set_endpoint_resolver`].
-    ///
-    /// # Examples
-    /// Create a custom endpoint resolver that resolves a different endpoing per-stage, e.g. staging vs. production.
-    /// ```no_run
-    /// use aws_sdk_sns::config::endpoint::{ResolveEndpoint, EndpointFuture, Params, Endpoint};
-    /// #[derive(Debug)]
-    /// struct StageResolver { stage: String }
-    /// impl ResolveEndpoint for StageResolver {
-    ///     fn resolve_endpoint(&self, params: &Params) -> EndpointFuture<'_> {
-    ///         let stage = &self.stage;
-    ///         EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{stage}.myservice.com")).build()))
-    ///     }
-    /// }
-    /// let resolver = StageResolver { stage: std::env::var("STAGE").unwrap() };
-    /// let config = aws_sdk_sns::Config::builder().endpoint_resolver(resolver).build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
-        self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
-        self
-    }
-
-    /// Sets the endpoint resolver to use when making requests.
-    ///
-    ///
-    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
-    /// rules for `aws_sdk_sns`.
-    ///
-    pub fn set_endpoint_resolver(
-        &mut self,
-        endpoint_resolver: ::std::option::Option<::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver>,
-    ) -> &mut Self {
-        self.runtime_components.set_endpoint_resolver(endpoint_resolver);
-        self
-    }
-    /// Set the retry_config for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_sdk_sns::config::Config;
-    /// use aws_sdk_sns::config::retry::RetryConfig;
-    ///
-    /// let retry_config = RetryConfig::standard().with_max_attempts(5);
-    /// let config = Config::builder().retry_config(retry_config).build();
-    /// ```
-    ///
-    /// # Retry token bucket
-    ///
-    /// [`RetryConfig`](::aws_smithy_types::retry::RetryConfig) controls *how many* times to retry and *how long* to back
-    /// off. Retries are **also** gated by a retry token bucket (also called the retry quota) that
-    /// is shared across a [`RetryPartition`](::aws_smithy_runtime::client::retries::RetryPartition). To configure the token bucket — for
-    /// example, to set
-    /// its capacity or to give a workload its own bucket — see [`Self::retry_partition`] and
-    /// [`RetryPartition::custom`](::aws_smithy_runtime::client::retries::RetryPartition::custom).
-    pub fn retry_config(mut self, retry_config: ::aws_smithy_types::retry::RetryConfig) -> Self {
-        self.set_retry_config(Some(retry_config));
-        self
-    }
-
-    /// Set the retry_config for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_sdk_sns::config::{Builder, Config};
-    /// use aws_sdk_sns::config::retry::RetryConfig;
-    ///
-    /// fn disable_retries(builder: &mut Builder) {
-    ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
-    ///     builder.set_retry_config(Some(retry_config));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// disable_retries(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_retry_config(&mut self, retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>) -> &mut Self {
-        retry_config.map(|r| self.config.store_put(r));
-        self
-    }
-    /// Set the sleep_impl for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// use aws_sdk_sns::config::{AsyncSleep, Config, SharedAsyncSleep, Sleep};
-    ///
-    /// #[derive(Debug)]
-    /// pub struct ForeverSleep;
-    ///
-    /// impl AsyncSleep for ForeverSleep {
-    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
-    ///         Sleep::new(std::future::pending())
-    ///     }
-    /// }
-    ///
-    /// let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
-    /// let config = Config::builder().sleep_impl(sleep_impl).build();
-    /// ```
-    pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
-        self
-    }
-
-    /// Set the sleep_impl for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// use aws_sdk_sns::config::{AsyncSleep, Builder, Config, SharedAsyncSleep, Sleep};
-    ///
-    /// #[derive(Debug)]
-    /// pub struct ForeverSleep;
-    ///
-    /// impl AsyncSleep for ForeverSleep {
-    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
-    ///         Sleep::new(std::future::pending())
-    ///     }
-    /// }
-    ///
-    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
-    ///     let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
-    ///     builder.set_sleep_impl(Some(sleep_impl));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// set_never_ending_sleep_impl(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_sleep_impl(&mut self, sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>) -> &mut Self {
-        self.runtime_components.set_sleep_impl(sleep_impl);
-        self
-    }
-    /// Set the timeout_config for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use std::time::Duration;
-    /// use aws_sdk_sns::config::Config;
-    /// use aws_sdk_sns::config::timeout::TimeoutConfig;
-    ///
-    /// let timeout_config = TimeoutConfig::builder()
-    ///     .operation_attempt_timeout(Duration::from_secs(1))
-    ///     .build();
-    /// let config = Config::builder().timeout_config(timeout_config).build();
-    /// ```
-    pub fn timeout_config(mut self, timeout_config: ::aws_smithy_types::timeout::TimeoutConfig) -> Self {
-        self.set_timeout_config(Some(timeout_config));
-        self
-    }
-
-    /// Set the timeout_config for the builder.
-    ///
-    /// Setting this to `None` has no effect if another source of configuration has set timeouts. If you
-    /// are attempting to disable timeouts, use [`TimeoutConfig::disabled`](::aws_smithy_types::timeout::TimeoutConfig::disabled)
-    ///
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use std::time::Duration;
-    /// use aws_sdk_sns::config::{Builder, Config};
-    /// use aws_sdk_sns::config::timeout::TimeoutConfig;
-    ///
-    /// fn set_request_timeout(builder: &mut Builder) {
-    ///     let timeout_config = TimeoutConfig::builder()
-    ///         .operation_attempt_timeout(Duration::from_secs(1))
-    ///         .build();
-    ///     builder.set_timeout_config(Some(timeout_config));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// set_request_timeout(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_timeout_config(&mut self, timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>) -> &mut Self {
-        // passing None has no impact.
-        let Some(mut timeout_config) = timeout_config else { return self };
-
-        if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
-            timeout_config.take_defaults_from(base);
+    impl Builder {
+        pub fn endpoint_url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
+            self.endpoint_url = Some(value.into());
+            self
         }
-        self.config.store_put(timeout_config);
-        self
-    }
-    /// Set the partition for retry-related state. When clients share a retry partition, they will
-    /// also share components such as token buckets and client rate limiters.
-    /// See the [`RetryPartition`](::aws_smithy_runtime::client::retries::RetryPartition) documentation for more details.
-    ///
-    /// # Default Behavior
-    ///
-    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `sns`
-    /// (or `sns-<region>` if a region is configured).
-    /// All SNS clients without an explicit retry partition will share this default partition.
-    ///
-    /// # Notes
-    ///
-    /// - This is an advanced setting. A common reason to set it is to size or isolate the retry
-    ///   token bucket — for example, giving a high-throughput workload its own bucket. Otherwise
-    ///   most users won't need to modify it.
-    /// - A configured client rate limiter has no effect unless [`RetryConfig::adaptive`](::aws_smithy_types::retry::RetryConfig::adaptive) is used.
-    ///
-    /// # Examples
-    ///
-    /// Creating a custom retry partition with a token bucket:
-    /// ```no_run
-    /// use aws_sdk_sns::config::Config;
-    /// use aws_sdk_sns::config::retry::{RetryPartition, TokenBucket};
-    ///
-    /// let token_bucket = TokenBucket::new(10);
-    /// let config = Config::builder()
-    ///     .retry_partition(RetryPartition::custom("custom")
-    ///         .token_bucket(token_bucket)
-    ///         .build()
-    ///     )
-    ///     .build();
-    /// ```
-    ///
-    /// Sizing the retry token bucket (for example, for a high-throughput workload), or giving a
-    /// workload its own bucket:
-    /// ```no_run
-    /// use aws_sdk_sns::config::Config;
-    /// use aws_sdk_sns::config::retry::{RetryPartition, TokenBucket};
-    ///
-    /// let config = Config::builder()
-    ///     .retry_partition(
-    ///         RetryPartition::custom("high-throughput")
-    ///             .token_bucket(TokenBucket::builder().capacity(5000).build())
-    ///             .build(),
-    ///     )
-    ///     .build();
-    /// ```
-    ///
-    /// Configuring a client rate limiter with adaptive retry mode:
-    /// ```no_run
-    /// use aws_sdk_sns::config::Config;
-    /// use aws_sdk_sns::config::retry::{ClientRateLimiter, RetryConfig, RetryPartition};
-    ///
-    /// let client_rate_limiter = ClientRateLimiter::new(10.0);
-    /// let config = Config::builder()
-    ///     .retry_partition(RetryPartition::custom("custom")
-    ///         .client_rate_limiter(client_rate_limiter)
-    ///         .build()
-    ///     )
-    ///     .retry_config(RetryConfig::adaptive())
-    ///     .build();
-    /// ```
-    pub fn retry_partition(mut self, retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition) -> Self {
-        self.set_retry_partition(Some(retry_partition));
-        self
-    }
-    /// Like [`Self::retry_partition`], but takes a mutable reference to the builder and an optional `RetryPartition`
-    pub fn set_retry_partition(
-        &mut self,
-        retry_partition: ::std::option::Option<::aws_smithy_runtime::client::retries::RetryPartition>,
-    ) -> &mut Self {
-        retry_partition.map(|r| self.config.store_put(r));
-        self
-    }
-    /// Set the identity cache for auth.
-    ///
-    /// The identity cache defaults to a lazy caching implementation that will resolve
-    /// an identity when it is requested, and place it in the cache thereafter. Subsequent
-    /// requests will take the value from the cache while it is still valid. Once it expires,
-    /// the next request will result in refreshing the identity.
-    ///
-    /// This configuration allows you to disable or change the default caching mechanism.
-    /// To use a custom caching mechanism, implement the [`ResolveCachedIdentity`](crate::config::ResolveCachedIdentity)
-    /// trait and pass that implementation into this function.
-    ///
-    /// # Examples
-    ///
-    /// Disabling identity caching:
-    /// ```no_run
-    /// use aws_sdk_sns::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_sns::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn identity_cache(mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> Self {
-        self.set_identity_cache(identity_cache);
-        self
-    }
-
-    /// Set the identity cache for auth.
-    ///
-    /// The identity cache defaults to a lazy caching implementation that will resolve
-    /// an identity when it is requested, and place it in the cache thereafter. Subsequent
-    /// requests will take the value from the cache while it is still valid. Once it expires,
-    /// the next request will result in refreshing the identity.
-    ///
-    /// This configuration allows you to disable or change the default caching mechanism.
-    /// To use a custom caching mechanism, implement the [`ResolveCachedIdentity`](crate::config::ResolveCachedIdentity)
-    /// trait and pass that implementation into this function.
-    ///
-    /// # Examples
-    ///
-    /// Disabling identity caching:
-    /// ```no_run
-    /// use aws_sdk_sns::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_sns::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn set_identity_cache(&mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> &mut Self {
-        self.runtime_components.set_identity_cache(::std::option::Option::Some(identity_cache));
-        self
-    }
-    /// Add an [interceptor](crate::config::Intercept) that runs at specific stages of the request execution pipeline.
-    ///
-    /// Interceptors targeted at a certain stage are executed according to the pre-defined priority.
-    /// The SDK provides a default set of interceptors. An interceptor configured by this method
-    /// will run after those default interceptors.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # fn example() {
-    /// use aws_smithy_runtime_api::box_error::BoxError;
-    /// use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
-    /// use aws_smithy_runtime_api::client::interceptors::Intercept;
-    /// use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
-    /// use aws_smithy_types::config_bag::ConfigBag;
-    /// use aws_sdk_sns::config::Config;
-    /// use ::http::uri::Uri;
-    ///
-    /// fn base_url() -> String {
-    ///     // ...
-    ///     # String::new()
-    /// }
-    ///
-    /// #[derive(Debug)]
-    /// pub struct UriModifierInterceptor;
-    /// impl Intercept for UriModifierInterceptor {
-    ///     fn name(&self) -> &'static str {
-    ///         "UriModifierInterceptor"
-    ///     }
-    ///     fn modify_before_signing(
-    ///         &self,
-    ///         context: &mut BeforeTransmitInterceptorContextMut<'_>,
-    ///         _runtime_components: &RuntimeComponents,
-    ///         _cfg: &mut ConfigBag,
-    ///     ) -> Result<(), BoxError> {
-    ///         let request = context.request_mut();
-    ///         let uri = format!("{}{}", base_url(), request.uri());
-    ///         *request.uri_mut() = uri.parse::<Uri>()?.into();
-    ///
-    ///         Ok(())
-    ///     }
-    /// }
-    ///
-    /// let config = Config::builder()
-    ///     .interceptor(UriModifierInterceptor)
-    ///     .build();
-    /// # }
-    /// ```
-    pub fn interceptor(mut self, interceptor: impl crate::config::Intercept + 'static) -> Self {
-        self.push_interceptor(crate::config::SharedInterceptor::new(interceptor));
-        self
-    }
-
-    /// Like [`Self::interceptor`], but takes a [`SharedInterceptor`](crate::config::SharedInterceptor).
-    pub fn push_interceptor(&mut self, interceptor: crate::config::SharedInterceptor) -> &mut Self {
-        self.runtime_components.push_interceptor(interceptor);
-        self
-    }
-
-    /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
-        self.runtime_components.set_interceptors(interceptors.into_iter());
-        self
-    }
-    /// Sets the time source used for this service
-    pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
-        self
-    }
-    /// Sets the time source used for this service
-    pub fn set_time_source(&mut self, time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>) -> &mut Self {
-        self.runtime_components.set_time_source(time_source);
-        self
-    }
-    /// Add type implementing [`ClassifyRetry`](::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry) that will be used by the
-    /// [`RetryStrategy`](::aws_smithy_runtime_api::client::retries::RetryStrategy) to determine what responses should be retried.
-    ///
-    /// A retry classifier configured by this method will run according to its [priority](::aws_smithy_runtime_api::client::retries::classifiers::RetryClassifierPriority).
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # fn example() {
-    /// use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
-    /// use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
-    /// use aws_smithy_runtime_api::client::retries::classifiers::{
-    ///     ClassifyRetry, RetryAction, RetryClassifierPriority,
-    /// };
-    /// use aws_smithy_types::error::metadata::ProvideErrorMetadata;
-    /// use aws_smithy_types::retry::ErrorKind;
-    /// use std::error::Error as StdError;
-    /// use std::marker::PhantomData;
-    /// use std::fmt;
-    /// use aws_sdk_sns::config::Config;
-    /// # #[derive(Debug)]
-    /// # struct SomeOperationError {}
-    /// # impl StdError for SomeOperationError {}
-    /// # impl fmt::Display for SomeOperationError {
-    /// #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
-    /// # }
-    /// # impl ProvideErrorMetadata for SomeOperationError {
-    /// #    fn meta(&self) -> &aws_sdk_sns::error::ErrorMetadata { todo!() }
-    /// # }
-    ///
-    /// const RETRYABLE_ERROR_CODES: &[&str] = &[
-    ///     // List error codes to be retried here...
-    /// ];
-    ///
-    /// // When classifying at an operation's error type, classifiers require a generic parameter.
-    /// // When classifying the HTTP response alone, no generic is needed.
-    /// #[derive(Debug, Default)]
-    /// pub struct ExampleErrorCodeClassifier<E> {
-    ///     _inner: PhantomData<E>,
-    /// }
-    ///
-    /// impl<E> ExampleErrorCodeClassifier<E> {
-    ///     pub fn new() -> Self {
-    ///         Self {
-    ///             _inner: PhantomData,
-    ///         }
-    ///     }
-    /// }
-    ///
-    /// impl<E> ClassifyRetry for ExampleErrorCodeClassifier<E>
-    /// where
-    ///     // Adding a trait bound for ProvideErrorMetadata allows us to inspect the error code.
-    ///     E: StdError + ProvideErrorMetadata + Send + Sync + 'static,
-    /// {
-    ///     fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
-    ///         // Check for a result
-    ///         let output_or_error = ctx.output_or_error();
-    ///         // Check for an error
-    ///         let error = match output_or_error {
-    ///             Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
-    ///               Some(Err(err)) => err,
-    ///         };
-    ///
-    ///         // Downcast the generic error and extract the code
-    ///         let error_code = OrchestratorError::as_operation_error(error)
-    ///             .and_then(|err| err.downcast_ref::<E>())
-    ///             .and_then(|err| err.code());
-    ///
-    ///         // If this error's code is in our list, return an action that tells the RetryStrategy to retry this request.
-    ///         if let Some(error_code) = error_code {
-    ///             if RETRYABLE_ERROR_CODES.contains(&error_code) {
-    ///                 return RetryAction::transient_error();
-    ///             }
-    ///         }
-    ///
-    ///         // Otherwise, return that no action is indicated i.e. that this classifier doesn't require a retry.
-    ///         // Another classifier may still classify this response as retryable.
-    ///         RetryAction::NoActionIndicated
-    ///     }
-    ///
-    ///     fn name(&self) -> &'static str { "Example Error Code Classifier" }
-    /// }
-    ///
-    /// let config = Config::builder()
-    ///     .retry_classifier(ExampleErrorCodeClassifier::<SomeOperationError>::new())
-    ///     .build();
-    /// # }
-    /// ```
-    pub fn retry_classifier(
-        mut self,
-        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
-    ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
-        self
-    }
-
-    /// Like [`Self::retry_classifier`], but takes a [`SharedRetryClassifier`](::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier).
-    pub fn push_retry_classifier(
-        &mut self,
-        retry_classifier: ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
-    ) -> &mut Self {
-        self.runtime_components.push_retry_classifier(retry_classifier);
-        self
-    }
-
-    /// Set [`SharedRetryClassifier`](::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier)s for the builder, replacing any that
-    /// were previously set.
-    pub fn set_retry_classifiers(
-        &mut self,
-        retry_classifiers: impl IntoIterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier>,
-    ) -> &mut Self {
-        self.runtime_components.set_retry_classifiers(retry_classifiers.into_iter());
-        self
-    }
-    /// Sets the name of the app that is using the client.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn app_name(mut self, app_name: ::aws_types::app_name::AppName) -> Self {
-        self.set_app_name(Some(app_name));
-        self
-    }
-    /// Sets the name of the app that is using the client.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn set_app_name(&mut self, app_name: ::std::option::Option<::aws_types::app_name::AppName>) -> &mut Self {
-        self.config.store_or_unset(app_name);
-        self
-    }
-    /// Appends framework metadata to the user agent.
-    ///
-    /// This _optional_ metadata identifies a software framework or third-party library
-    /// that is being used with the client. It is rendered into the user agent string
-    /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
-    /// self-identify in the requests they make. Multiple entries may be added; each call
-    /// appends another entry rather than replacing previous ones.
-    ///
-    /// Entries are de-duplicated on `(name, version)`, rendered in first-seen order, and
-    /// the total number of unique entries included in the user agent is capped (currently
-    /// at 10); additional entries beyond the cap are dropped with a warning.
-    pub fn framework_metadata(mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> Self {
-        self.push_framework_metadata(framework_metadata);
-        self
-    }
-    /// Appends framework metadata to the user agent.
-    ///
-    /// This _optional_ metadata identifies a software framework or third-party library
-    /// that is being used with the client. It is rendered into the user agent string
-    /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
-    /// self-identify in the requests they make. Multiple entries may be added; each call
-    /// appends another entry rather than replacing previous ones.
-    pub fn push_framework_metadata(&mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> &mut Self {
-        self.config.store_append(framework_metadata);
-        self
-    }
-    /// Overrides the default invocation ID generator.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(mut self, gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static) -> Self {
-        self.set_invocation_id_generator(::std::option::Option::Some(
-            ::aws_runtime::invocation_id::SharedInvocationIdGenerator::new(gen),
-        ));
-        self
-    }
-    /// Overrides the default invocation ID generator.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn set_invocation_id_generator(
-        &mut self,
-        gen: ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator>,
-    ) -> &mut Self {
-        self.config.store_or_unset(gen);
-        self
-    }
-    /// Sets the endpoint URL used to communicate with this service.
-    ///
-    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
-    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
-    /// [`Builder::endpoint_resolver`].
-    pub fn endpoint_url(mut self, endpoint_url: impl Into<::std::string::String>) -> Self {
-        self.set_endpoint_url(Some(endpoint_url.into()));
-        self
-    }
-    /// Sets the endpoint URL used to communicate with this service.
-    ///
-    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
-    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
-    /// [`Builder::endpoint_resolver`].
-    pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
-        self
-    }
-    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
-    pub fn use_dual_stack(mut self, use_dual_stack: impl Into<bool>) -> Self {
-        self.set_use_dual_stack(Some(use_dual_stack.into()));
-        self
-    }
-    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
-    pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
-        self
-    }
-    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
-    pub fn use_fips(mut self, use_fips: impl Into<bool>) -> Self {
-        self.set_use_fips(Some(use_fips.into()));
-        self
-    }
-    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
-    pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_types::region::Region;
-    /// use aws_sdk_sns::config::{Builder, Config};
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .region(Region::new("us-east-1"))
-    ///     .build();
-    /// ```
-    pub fn region(mut self, region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>) -> Self {
-        self.set_region(region.into());
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    pub fn set_region(&mut self, region: ::std::option::Option<crate::config::Region>) -> &mut Self {
-        self.config.store_or_unset(region);
-        self
-    }
-    /// Sets the credentials provider for this service
-    pub fn credentials_provider(mut self, credentials_provider: impl crate::config::ProvideCredentials + 'static) -> Self {
-        self.set_credentials_provider(::std::option::Option::Some(crate::config::SharedCredentialsProvider::new(
-            credentials_provider,
-        )));
-        self
-    }
-    /// Sets the credentials provider for this service
-    pub fn set_credentials_provider(&mut self, credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>) -> &mut Self {
-        if let Some(credentials_provider) = credentials_provider {
-            self.runtime_components
-                .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
+        pub fn build(self) -> super::Config {
+            super::Config {
+                endpoint_url: self.endpoint_url.unwrap_or_else(|| super::Config::default().endpoint_url),
+            }
         }
-        self
     }
-    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
-    ///
-    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
-    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
-    /// all operations might be the ideal behavior but could break existing applications.
-    ///
-    /// # Examples
-    ///
-    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
-    /// ```no_run
-    /// use aws_sdk_sns::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_sns::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn behavior_version(mut self, behavior_version: crate::config::BehaviorVersion) -> Self {
-        self.set_behavior_version(Some(behavior_version));
-        self
-    }
-
-    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
-    ///
-    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
-    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
-    /// all operations might be the ideal behavior but could break existing applications.
-    ///
-    /// # Examples
-    ///
-    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
-    /// ```no_run
-    /// use aws_sdk_sns::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_sns::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sns::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sns::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn set_behavior_version(&mut self, behavior_version: Option<crate::config::BehaviorVersion>) -> &mut Self {
-        self.behavior_version = behavior_version;
-        self
-    }
-
-    /// Convenience method to set the latest behavior major version
-    ///
-    /// This is equivalent to enabling the `behavior-version-latest` Cargo feature
-    pub fn behavior_version_latest(mut self) -> Self {
-        self.set_behavior_version(Some(crate::config::BehaviorVersion::latest()));
-        self
-    }
-    /// Adds a runtime plugin to the config.
-    #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, plugin: impl crate::config::RuntimePlugin + 'static) -> Self {
-        self.push_runtime_plugin(crate::config::SharedRuntimePlugin::new(plugin));
-        self
-    }
-    /// Adds a runtime plugin to the config.
-    #[allow(unused)]
-    pub(crate) fn push_runtime_plugin(&mut self, plugin: crate::config::SharedRuntimePlugin) -> &mut Self {
-        self.runtime_plugins.push(plugin);
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
-    pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
-        self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
-            ::aws_credential_types::Credentials::for_tests(),
-        )));
-        self.behavior_version = ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. NOTE: Consider migrating to use `with_test_defaults_v2` instead.
-    pub fn with_test_defaults(mut self) -> Self {
-        self.apply_test_defaults();
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. V2 of this function sets additional test defaults such as region configuration (if applicable).
-    pub fn apply_test_defaults_v2(&mut self) -> &mut Self {
-        self.apply_test_defaults();
-        if self.config.load::<crate::config::Region>().is_none() {
-            self.set_region(::std::option::Option::Some(crate::config::Region::new("us-east-1")));
-        }
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. V2 of this function sets additional test defaults such as region configuration (if applicable).
-    pub fn with_test_defaults_v2(mut self) -> Self {
-        self.apply_test_defaults_v2();
-        self
-    }
-    /// Builds a [`Config`].
-    #[allow(unused_mut)]
-    pub fn build(mut self) -> Config {
-        let mut layer = self.config;
-        if self.runtime_components.time_source().is_none() {
-            self.runtime_components
-                .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
-        }
-        layer.store_put(crate::meta::API_METADATA.clone());
-        layer.store_put(::aws_types::SigningName::from_static("sns"));
-        layer
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| layer.store_put(::aws_types::region::SigningRegion::from(r)));
-        Config {
-            config: crate::config::Layer::from(layer.clone())
-                .with_name("aws_sdk_sns::config::Config")
-                .freeze(),
-            cloneable: layer,
-            runtime_components: self.runtime_components,
-            runtime_plugins: self.runtime_plugins,
-            behavior_version: self.behavior_version,
+    impl From<&super::Config> for Builder {
+        fn from(config: &super::Config) -> Self {
+            Self {
+                endpoint_url: Some(config.endpoint_url.clone()),
+            }
         }
     }
 }
-#[derive(::std::fmt::Debug)]
-pub(crate) struct ServiceRuntimePlugin {
-    config: ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer>,
-    runtime_components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}
-
-impl ServiceRuntimePlugin {
-    pub fn new(_service_config: crate::config::Config) -> Self {
-        let config = {
-            let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AmazonSimpleNotificationService");
-            cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
-            ::std::option::Option::Some(cfg.freeze())
-        };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
-        runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
-            use crate::config::auth::ResolveAuthScheme;
-            crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
-        }));
-        runtime_components.set_endpoint_resolver(::std::option::Option::Some({
-            use crate::config::endpoint::ResolveEndpoint;
-            crate::config::endpoint::DefaultResolver::new().into_shared_resolver()
-        }));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
-        ));
-        runtime_components.push_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
-        runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
-        runtime_components.push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
-        ));
-        runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
-            ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
-        ));
-        Self { config, runtime_components }
-    }
-}
-
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ServiceRuntimePlugin {
-    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
-        self.config.clone()
-    }
-
-    fn order(&self) -> ::aws_smithy_runtime_api::client::runtime_plugin::Order {
-        ::aws_smithy_runtime_api::client::runtime_plugin::Order::Defaults
-    }
-
-    fn runtime_components(
-        &self,
-        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
-        ::std::borrow::Cow::Borrowed(&self.runtime_components)
-    }
-}
-
-// Cross-operation shared-state singletons
-
-/// A plugin that enables configuration for a single operation invocation
-///
-/// The `config` method will return a `FrozenLayer` by storing values from `config_override`.
-/// In the case of default values requested, they will be obtained from `client_config`.
-#[derive(Debug)]
-pub(crate) struct ConfigOverrideRuntimePlugin {
-    pub(crate) config: ::aws_smithy_types::config_bag::FrozenLayer,
-    pub(crate) components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}

-impl ConfigOverrideRuntimePlugin {
-    #[allow(dead_code)] // unused when a service does not provide any operations
-    pub(crate) fn new(
-        config_override: Builder,
-        initial_config: ::aws_smithy_types::config_bag::FrozenLayer,
-        initial_components: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> Self {
-        let mut layer = config_override.config;
-        let mut components = config_override.runtime_components;
-        #[allow(unused_mut)]
-        let mut resolver =
-            ::aws_smithy_runtime::client::config_override::Resolver::overrid(initial_config, initial_components, &mut layer, &mut components);
-
-        resolver
-            .config_mut()
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| resolver.config_mut().store_put(::aws_types::region::SigningRegion::from(r)));
-
-        let _ = resolver;
-
-        // When the config override supplies an identity resolver for any auth scheme
-        // known to the client or the override itself, we give this operation its own
-        // short-lived identity cache so that new partitions don't accumulate in the
-        // shared client cache. A lazy cache (not `no_cache`) is used so that resolved
-        // identities are served from the short-lived identity cache on retries.
-        //
-        // This is skipped if the override already sets its own identity cache.
-        if components.has_identity_resolvers() && components.identity_cache().is_none() {
-            components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
-            ));
-        }
-
-        Self {
-            config: ::aws_smithy_types::config_bag::Layer::from(layer)
-                .with_name("aws_sdk_sns::config::ConfigOverrideRuntimePlugin")
-                .freeze(),
-            components,
-        }
-    }
-}
-
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ConfigOverrideRuntimePlugin {
-    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
-        Some(self.config.clone())
-    }
-
-    fn runtime_components(
-        &self,
-        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
-        ::std::borrow::Cow::Borrowed(&self.components)
-    }
-}
-
-pub use ::aws_smithy_runtime::client::identity::IdentityCache;
-pub use ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
-pub use ::aws_smithy_types::config_bag::ConfigBag;
-
-pub use ::aws_credential_types::Credentials;
-
-impl From<&::aws_types::sdk_config::SdkConfig> for Builder {
-    fn from(input: &::aws_types::sdk_config::SdkConfig) -> Self {
-        let mut builder = Builder::default();
-        builder.set_credentials_provider(input.credentials_provider());
-        builder = builder.region(input.region().cloned());
-        builder.set_use_fips(input.use_fips());
-        builder.set_use_dual_stack(input.use_dual_stack());
-        if input.get_origin("endpoint_url").is_client_config() {
-            builder.set_endpoint_url(input.endpoint_url().map(|s| s.to_string()));
-        } else {
-            builder.set_endpoint_url(
-                input
-                    .service_config()
-                    .and_then(|conf| {
-                        conf.load_config(service_config_key("SNS", "AWS_ENDPOINT_URL", "endpoint_url"))
-                            .map(|it| it.parse().unwrap())
-                    })
-                    .or_else(|| input.endpoint_url().map(|s| s.to_string())),
-            );
-        }
-        // resiliency
-        builder.set_retry_config(input.retry_config().cloned());
-        builder.set_timeout_config(input.timeout_config().cloned());
-        builder.set_sleep_impl(input.sleep_impl());
-
-        builder.set_http_client(input.http_client());
-        builder.set_time_source(input.time_source());
-        builder.set_behavior_version(input.behavior_version());
-        builder.set_auth_scheme_preference(input.auth_scheme_preference().cloned());
-        // setting `None` here removes the default
-        if let Some(config) = input.stalled_stream_protection() {
-            builder.set_stalled_stream_protection(Some(config));
-        }
-
-        if let Some(cache) = input.identity_cache() {
-            builder.set_identity_cache(cache);
-        }
-        builder.set_app_name(input.app_name().cloned());
-        for framework_metadata in input.framework_metadata() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-
-        builder
-    }
-}
-
-impl From<&::aws_types::sdk_config::SdkConfig> for Config {
-    fn from(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Builder::from(sdk_config).build()
-    }
-}
-
-pub use ::aws_types::app_name::AppName;
-pub use ::aws_types::sdk_ua_metadata::FrameworkMetadata;
-
-#[allow(dead_code)]
-fn service_config_key<'a>(service_id: &'a str, env: &'a str, profile: &'a str) -> aws_types::service_config::ServiceConfigKey<'a> {
-    ::aws_types::service_config::ServiceConfigKey::builder()
-        .service_id(service_id)
-        .env(env)
-        .profile(profile)
-        .build()
-        .expect("all field sets explicitly, can't fail")
-}
-
-pub use ::aws_smithy_async::rt::sleep::Sleep;
-
-pub(crate) fn base_client_runtime_plugins(mut config: crate::Config) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
-    let mut configured_plugins = ::std::vec::Vec::new();
-    ::std::mem::swap(&mut config.runtime_plugins, &mut configured_plugins);
-    #[cfg(feature = "behavior-version-latest")]
-    {
-        if config.behavior_version.is_none() {
-            config.behavior_version = Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
-        }
-    }
-
-    let default_retry_partition = "sns";
-    let default_retry_partition = match config.region() {
-        Some(region) => ::std::borrow::Cow::from(format!("{default_retry_partition}-{region}")),
-        None => ::std::borrow::Cow::from(default_retry_partition),
-    };
-
-    let scope = "aws-sdk-sns";
-
-    #[allow(deprecated)]
-                    let mut plugins = ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins::new()
-                        // defaults
-                        .with_client_plugins(::aws_smithy_runtime::client::defaults::default_plugins(
-                            ::aws_smithy_runtime::client::defaults::DefaultPluginParams::new()
-                                .with_retry_partition_name(default_retry_partition)
-                                .with_behavior_version(config.behavior_version.expect("Invalid client configuration: A behavior major version must be set when sending a request or constructing a client. You must set it during client construction or by enabling the `behavior-version-latest` cargo feature."))
-                                .with_is_aws_sdk(true)
-                        ))
-                        // user config
-                        .with_client_plugin(
-                            ::aws_smithy_runtime_api::client::runtime_plugin::StaticRuntimePlugin::new()
-                                .with_config(config.config.clone())
-                                .with_runtime_components(config.runtime_components.clone())
-                        )
-                        // codegen config
-                        .with_client_plugin(crate::config::ServiceRuntimePlugin::new(config.clone()))
-                        .with_client_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePlugin::new())
-                        .with_client_plugin(
-                            ::aws_smithy_runtime::client::metrics::MetricsRuntimePlugin::builder()
-                                .with_scope(scope)
-                                .with_time_source(config.runtime_components.time_source().unwrap_or_default())
-                                .build()
-                                .expect("All required fields have been set")
-                        );
-
-    for plugin in configured_plugins {
-        plugins = plugins.with_client_plugin(plugin);
+impl Config {
+    pub fn builder() -> config::Builder {
+        config::Builder::default()
     }
-    plugins
 }
-
-pub use ::aws_smithy_types::config_bag::FrozenLayer;
-
-pub use ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
-
-pub use ::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin;
-
-pub use ::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
-
-pub use ::aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
-
-pub use ::aws_smithy_runtime_api::client::http::SharedHttpClient;
-
-pub use ::aws_smithy_async::rt::sleep::SharedAsyncSleep;
-
-pub use ::aws_smithy_runtime_api::client::identity::SharedIdentityCache;
-
-pub use ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor;
-
-pub use ::aws_types::region::Region;
-
-pub use ::aws_credential_types::provider::SharedCredentialsProvider;
-
-pub use ::aws_smithy_runtime_api::client::http::HttpClient;
-
-pub use ::aws_smithy_runtime_api::shared::IntoShared;
-
-pub use ::aws_smithy_async::rt::sleep::AsyncSleep;
-
-pub use ::aws_smithy_runtime_api::client::identity::ResolveCachedIdentity;
-
-pub use ::aws_smithy_runtime_api::client::interceptors::Intercept;
-
-pub use ::aws_credential_types::provider::ProvideCredentials;
-
-pub use ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
-
-pub use ::aws_smithy_types::config_bag::Layer;
-
-/// Types needed to configure endpoint resolution.
-pub mod endpoint;
-
-/// HTTP request and response types.
-pub mod http;
-
-/// Types needed to implement [`Intercept`](crate::config::Intercept).
-pub mod interceptors;
-
-/// Retry configuration.
-///
-/// [`RetryConfig`](crate::config::retry::RetryConfig) sets the number of retry attempts and the backoff between them. Retries are additionally bounded by a retry token bucket (a shared retry quota): [`TokenBucket`](crate::config::retry::TokenBucket) holds the tokens and [`RetryPartition`](crate::config::retry::RetryPartition) determines which clients and operations share one. To size the token bucket or give a workload its own, use [`Builder::retry_partition`](crate::config::Builder::retry_partition).
-pub mod retry;
-
-/// Timeout configuration.
-pub mod timeout;
-
-/// Types needed to configure auth scheme resolution.
-pub mod auth;
```

### `src/operation/add_permission.rs`

```diff
--- reference/src/operation/add_permission.rs
+++ generated/src/operation/add_permission.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AddPermission", "SNS"));
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
@@ -252,13 +258,10 @@
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
@@ -292,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -435,6 +438,11 @@
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

### `src/operation/check_if_phone_number_is_opted_out.rs`

```diff
--- reference/src/operation/check_if_phone_number_is_opted_out.rs
+++ generated/src/operation/check_if_phone_number_is_opted_out.rs
@@ -107,9 +107,9 @@
             "SNS",
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
@@ -127,6 +127,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CheckIfPhoneNumberIsOptedOut")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                CheckIfPhoneNumberIsOptedOutTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -138,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
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
@@ -147,6 +158,44 @@
 }

 #[derive(Debug)]
+struct CheckIfPhoneNumberIsOptedOutTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CheckIfPhoneNumberIsOptedOutTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "CheckIfPhoneNumberIsOptedOutTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<CheckIfPhoneNumberIsOptedOutInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct CheckIfPhoneNumberIsOptedOutResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for CheckIfPhoneNumberIsOptedOutResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -206,12 +255,11 @@
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
@@ -246,8 +294,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -389,6 +437,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/confirm_subscription.rs`

```diff
--- reference/src/operation/confirm_subscription.rs
+++ generated/src/operation/confirm_subscription.rs
@@ -107,9 +107,9 @@
             "SNS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ConfirmSubscription")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ConfirmSubscriptionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ConfirmSubscriptionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::confirm_subscription::ConfirmSubscriptionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::confirm_subscription::ConfirmSubscriptionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::confirm_subscription::ConfirmSubscriptionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ConfirmSubscription")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ConfirmSubscriptionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ConfirmSubscriptionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::confirm_subscription::ConfirmSubscriptionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::confirm_subscription::ConfirmSubscriptionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::confirm_subscription::ConfirmSubscriptionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -260,12 +252,11 @@
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
@@ -300,8 +291,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -473,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::confirm_subscription::ConfirmSubscriptionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::confirm_subscription::ConfirmSubscriptionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_platform_application.rs`

```diff
--- reference/src/operation/create_platform_application.rs
+++ generated/src/operation/create_platform_application.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::create_platform_application::CreatePlatformApplicationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_platform_application::CreatePlatformApplicationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::create_platform_application::CreatePlatformApplicationError,
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
@@ -255,12 +263,11 @@
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
@@ -295,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -428,6 +435,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_platform_application::CreatePlatformApplicationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_platform_application::CreatePlatformApplicationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_platform_endpoint.rs`

```diff
--- reference/src/operation/create_platform_endpoint.rs
+++ generated/src/operation/create_platform_endpoint.rs
@@ -107,9 +107,9 @@
             "SNS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreatePlatformEndpoint")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreatePlatformEndpointTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreatePlatformEndpointEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::create_platform_endpoint::CreatePlatformEndpointError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::create_platform_endpoint::CreatePlatformEndpointError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_platform_endpoint::CreatePlatformEndpointError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreatePlatformEndpoint")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    CreatePlatformEndpointTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    CreatePlatformEndpointEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::create_platform_endpoint::CreatePlatformEndpointError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::create_platform_endpoint::CreatePlatformEndpointError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::create_platform_endpoint::CreatePlatformEndpointError,
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
@@ -260,12 +269,11 @@
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
@@ -300,8 +308,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +451,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_platform_endpoint::CreatePlatformEndpointError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_platform_endpoint::CreatePlatformEndpointError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -107,9 +107,9 @@
             "SNS",
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
@@ -127,10 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateSMSSandboxPhoneNumber")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                CreateSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateSMSSandboxPhoneNumberEndpointParamsInterceptor,
+                CreateSmsSandboxPhoneNumberEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
@@ -138,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
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
@@ -147,6 +158,44 @@
 }

 #[derive(Debug)]
+struct CreateSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "CreateSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<CreateSmsSandboxPhoneNumberInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct CreateSMSSandboxPhoneNumberResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for CreateSMSSandboxPhoneNumberResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,12 +253,11 @@
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
@@ -219,12 +267,12 @@
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
@@ -244,8 +292,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -407,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_topic.rs`

```diff
--- reference/src/operation/create_topic.rs
+++ generated/src/operation/create_topic.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateTopic", "SNS"));
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
                 crate::operation::create_topic::CreateTopicError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_topic::CreateTopicError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_topic::CreateTopicError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,13 +258,10 @@
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
@@ -292,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -485,6 +488,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_topic::CreateTopicError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_topic::CreateTopicError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_endpoint.rs`

```diff
--- reference/src/operation/delete_endpoint.rs
+++ generated/src/operation/delete_endpoint.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteEndpoint", "SNS"));
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
                 crate::operation::delete_endpoint::DeleteEndpointError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_endpoint::DeleteEndpointError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_endpoint::DeleteEndpointError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,13 +253,10 @@
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
@@ -287,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -420,6 +423,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_endpoint::DeleteEndpointError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_endpoint::DeleteEndpointError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_platform_application.rs`

```diff
--- reference/src/operation/delete_platform_application.rs
+++ generated/src/operation/delete_platform_application.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::delete_platform_application::DeletePlatformApplicationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_platform_application::DeletePlatformApplicationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_platform_application::DeletePlatformApplicationError,
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
@@ -250,12 +258,11 @@
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
@@ -290,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -423,6 +430,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_platform_application::DeletePlatformApplicationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_platform_application::DeletePlatformApplicationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -107,9 +107,9 @@
             "SNS",
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
@@ -127,10 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteSMSSandboxPhoneNumber")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                DeleteSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteSMSSandboxPhoneNumberEndpointParamsInterceptor,
+                DeleteSmsSandboxPhoneNumberEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
@@ -138,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError,
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
@@ -147,6 +158,44 @@
 }

 #[derive(Debug)]
+struct DeleteSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "DeleteSmsSandboxPhoneNumberTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DeleteSmsSandboxPhoneNumberInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct DeleteSMSSandboxPhoneNumberResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for DeleteSMSSandboxPhoneNumberResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,12 +253,11 @@
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
@@ -219,12 +267,12 @@
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
@@ -244,8 +292,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -407,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_sms_sandbox_phone_number::DeleteSMSSandboxPhoneNumberError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_topic.rs`

```diff
--- reference/src/operation/delete_topic.rs
+++ generated/src/operation/delete_topic.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteTopic", "SNS"));
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
                 crate::operation::delete_topic::DeleteTopicError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_topic::DeleteTopicError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_topic::DeleteTopicError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,13 +253,10 @@
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
@@ -287,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +473,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_topic::DeleteTopicError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_topic::DeleteTopicError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_data_protection_policy.rs`

```diff
--- reference/src/operation/get_data_protection_policy.rs
+++ generated/src/operation/get_data_protection_policy.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::get_data_protection_policy::GetDataProtectionPolicyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_data_protection_policy::GetDataProtectionPolicyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_data_protection_policy::GetDataProtectionPolicyError,
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
@@ -250,12 +258,11 @@
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
@@ -290,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +450,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_data_protection_policy::GetDataProtectionPolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_data_protection_policy::GetDataProtectionPolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_endpoint_attributes.rs`

```diff
--- reference/src/operation/get_endpoint_attributes.rs
+++ generated/src/operation/get_endpoint_attributes.rs
@@ -107,9 +107,9 @@
             "SNS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetEndpointAttributes")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetEndpointAttributesTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetEndpointAttributesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_endpoint_attributes::GetEndpointAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_endpoint_attributes::GetEndpointAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_endpoint_attributes::GetEndpointAttributesError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetEndpointAttributes")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetEndpointAttributesTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetEndpointAttributesEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::get_endpoint_attributes::GetEndpointAttributesError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::get_endpoint_attributes::GetEndpointAttributesError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::get_endpoint_attributes::GetEndpointAttributesError,
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
@@ -250,12 +259,11 @@
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
@@ -290,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -433,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_endpoint_attributes::GetEndpointAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_endpoint_attributes::GetEndpointAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_platform_application_attributes.rs`

```diff
--- reference/src/operation/get_platform_application_attributes.rs
+++ generated/src/operation/get_platform_application_attributes.rs
@@ -113,9 +113,9 @@
             "SNS",
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
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_platform_application_attributes::GetPlatformApplicationAttributesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_platform_application_attributes::GetPlatformApplicationAttributesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_platform_application_attributes::GetPlatformApplicationAttributesError,
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
@@ -258,14 +266,11 @@
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
@@ -300,8 +305,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +448,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_platform_application_attributes::GetPlatformApplicationAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_platform_application_attributes::GetPlatformApplicationAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetSMSAttributes", "SNS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -124,10 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetSMSAttributes")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetSmsAttributesTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSMSAttributesEndpointParamsInterceptor,
+                GetSmsAttributesEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_sms_attributes::GetSMSAttributesError,
@@ -135,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_sms_attributes::GetSMSAttributesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_sms_attributes::GetSMSAttributesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_sms_attributes::GetSMSAttributesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -144,6 +153,44 @@
 }

 #[derive(Debug)]
+struct GetSmsAttributesTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSmsAttributesTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "GetSmsAttributesTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetSmsAttributesInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct GetSMSAttributesResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetSMSAttributesResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -201,13 +248,12 @@
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
@@ -216,12 +262,12 @@
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
@@ -241,8 +287,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -384,6 +430,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_sms_attributes::GetSMSAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_sms_attributes::GetSMSAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -107,9 +107,9 @@
             "SNS",
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
@@ -127,10 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetSMSSandboxAccountStatus")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetSmsSandboxAccountStatusTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSMSSandboxAccountStatusEndpointParamsInterceptor,
+                GetSmsSandboxAccountStatusEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
@@ -138,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
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
@@ -147,6 +158,44 @@
 }

 #[derive(Debug)]
+struct GetSmsSandboxAccountStatusTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSmsSandboxAccountStatusTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "GetSmsSandboxAccountStatusTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetSmsSandboxAccountStatusInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct GetSMSSandboxAccountStatusResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetSMSSandboxAccountStatusResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,24 +253,20 @@
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
@@ -241,8 +286,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -374,6 +419,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_subscription_attributes.rs`

```diff
--- reference/src/operation/get_subscription_attributes.rs
+++ generated/src/operation/get_subscription_attributes.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::get_subscription_attributes::GetSubscriptionAttributesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_subscription_attributes::GetSubscriptionAttributesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_subscription_attributes::GetSubscriptionAttributesError,
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
@@ -250,12 +258,11 @@
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
@@ -290,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -433,6 +440,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_subscription_attributes::GetSubscriptionAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_subscription_attributes::GetSubscriptionAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_topic_attributes.rs`

```diff
--- reference/src/operation/get_topic_attributes.rs
+++ generated/src/operation/get_topic_attributes.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetTopicAttributes", "SNS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetTopicAttributes")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetTopicAttributesTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetTopicAttributesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_topic_attributes::GetTopicAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_topic_attributes::GetTopicAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_topic_attributes::GetTopicAttributesError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetTopicAttributes")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetTopicAttributesTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetTopicAttributesEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_topic_attributes::GetTopicAttributesError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_topic_attributes::GetTopicAttributesError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_topic_attributes::GetTopicAttributesError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,12 +239,11 @@
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
@@ -287,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -440,6 +431,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_topic_attributes::GetTopicAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_topic_attributes::GetTopicAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_endpoints_by_platform_application.rs`

```diff
--- reference/src/operation/list_endpoints_by_platform_application.rs
+++ generated/src/operation/list_endpoints_by_platform_application.rs
@@ -113,9 +113,9 @@
             "SNS",
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
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationError,
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
@@ -265,14 +273,11 @@
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
@@ -307,8 +312,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_origination_numbers.rs`

```diff
--- reference/src/operation/list_origination_numbers.rs
+++ generated/src/operation/list_origination_numbers.rs
@@ -108,9 +108,9 @@
             "SNS",
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
@@ -126,25 +126,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListOriginationNumbers")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListOriginationNumbersTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListOriginationNumbersEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_origination_numbers::ListOriginationNumbersError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_origination_numbers::ListOriginationNumbersError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_origination_numbers::ListOriginationNumbersError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListOriginationNumbers")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListOriginationNumbersTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListOriginationNumbersEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::list_origination_numbers::ListOriginationNumbersError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::list_origination_numbers::ListOriginationNumbersError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::list_origination_numbers::ListOriginationNumbersError,
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
@@ -251,12 +260,11 @@
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
@@ -291,8 +299,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -444,6 +452,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_origination_numbers::ListOriginationNumbersError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_origination_numbers::ListOriginationNumbersError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -108,9 +108,9 @@
             "SNS",
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
@@ -142,9 +142,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
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
@@ -251,12 +259,11 @@
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
@@ -291,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -434,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_platform_applications.rs`

```diff
--- reference/src/operation/list_platform_applications.rs
+++ generated/src/operation/list_platform_applications.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::list_platform_applications::ListPlatformApplicationsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_platform_applications::ListPlatformApplicationsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_platform_applications::ListPlatformApplicationsError,
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
@@ -250,12 +258,11 @@
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
@@ -290,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -423,6 +430,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_platform_applications::ListPlatformApplicationsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_platform_applications::ListPlatformApplicationsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -108,9 +108,9 @@
             "SNS",
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
@@ -142,9 +142,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError,
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
@@ -151,12 +159,12 @@
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
@@ -251,12 +259,11 @@
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
@@ -266,12 +273,12 @@
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
@@ -291,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -444,6 +451,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_subscriptions.rs`

```diff
--- reference/src/operation/list_subscriptions.rs
+++ generated/src/operation/list_subscriptions.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListSubscriptions", "SNS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,16 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_subscriptions::ListSubscriptionsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_subscriptions::ListSubscriptionsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_subscriptions::ListSubscriptionsError>::builder(
+                )
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,13 +254,12 @@
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
@@ -287,8 +293,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -420,6 +426,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_subscriptions::ListSubscriptionsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_subscriptions::ListSubscriptionsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_subscriptions_by_topic.rs`

```diff
--- reference/src/operation/list_subscriptions_by_topic.rs
+++ generated/src/operation/list_subscriptions_by_topic.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicError,
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
@@ -255,12 +263,11 @@
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
@@ -295,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -438,6 +445,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_tags_for_resource.rs`

```diff
--- reference/src/operation/list_tags_for_resource.rs
+++ generated/src/operation/list_tags_for_resource.rs
@@ -107,9 +107,9 @@
             "SNS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListTagsForResource")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListTagsForResourceTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListTagsForResourceEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_tags_for_resource::ListTagsForResourceError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_tags_for_resource::ListTagsForResourceError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_tags_for_resource::ListTagsForResourceError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListTagsForResource")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListTagsForResourceTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListTagsForResourceEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::list_tags_for_resource::ListTagsForResourceError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::list_tags_for_resource::ListTagsForResourceError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_tags_for_resource::ListTagsForResourceError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,12 +242,11 @@
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
@@ -290,8 +281,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +434,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_tags_for_resource::ListTagsForResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_tags_for_resource::ListTagsForResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_topics.rs`

```diff
--- reference/src/operation/list_topics.rs
+++ generated/src/operation/list_topics.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListTopics", "SNS"));
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
                 crate::operation::list_topics::ListTopicsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_topics::ListTopicsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_topics::ListTopicsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,12 +251,10 @@
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
@@ -284,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -417,6 +421,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_topics::ListTopicsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_topics::ListTopicsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/opt_in_phone_number.rs`

```diff
--- reference/src/operation/opt_in_phone_number.rs
+++ generated/src/operation/opt_in_phone_number.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("OptInPhoneNumber", "SNS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -124,6 +124,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("OptInPhoneNumber")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                OptInPhoneNumberTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -135,9 +138,16 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::opt_in_phone_number::OptInPhoneNumberError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::opt_in_phone_number::OptInPhoneNumberError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::opt_in_phone_number::OptInPhoneNumberError>::builder(
+                )
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -144,6 +154,44 @@
 }

 #[derive(Debug)]
+struct OptInPhoneNumberTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for OptInPhoneNumberTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "OptInPhoneNumberTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<OptInPhoneNumberInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct OptInPhoneNumberResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for OptInPhoneNumberResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -201,13 +249,12 @@
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
@@ -241,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -384,6 +431,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::opt_in_phone_number::OptInPhoneNumberError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::opt_in_phone_number::OptInPhoneNumberError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/publish.rs`

```diff
--- reference/src/operation/publish.rs
+++ generated/src/operation/publish.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Publish", "SNS"));
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
                 crate::operation::publish::PublishError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::publish::PublishError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::publish::PublishError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -271,11 +277,10 @@
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
@@ -309,8 +314,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -562,6 +567,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::publish::PublishError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::publish::PublishError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/publish_batch.rs`

```diff
--- reference/src/operation/publish_batch.rs
+++ generated/src/operation/publish_batch.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("PublishBatch", "SNS"));
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
                 crate::operation::publish_batch::PublishBatchError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::publish_batch::PublishBatchError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::publish_batch::PublishBatchError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,13 +253,10 @@
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
@@ -287,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -590,6 +593,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::publish_batch::PublishBatchError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::publish_batch::PublishBatchError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_data_protection_policy.rs`

```diff
--- reference/src/operation/put_data_protection_policy.rs
+++ generated/src/operation/put_data_protection_policy.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::put_data_protection_policy::PutDataProtectionPolicyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_data_protection_policy::PutDataProtectionPolicyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_data_protection_policy::PutDataProtectionPolicyError,
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
@@ -255,12 +263,11 @@
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
@@ -295,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -448,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_data_protection_policy::PutDataProtectionPolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_data_protection_policy::PutDataProtectionPolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/remove_permission.rs`

```diff
--- reference/src/operation/remove_permission.rs
+++ generated/src/operation/remove_permission.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("RemovePermission", "SNS"));
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
@@ -252,13 +258,12 @@
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
@@ -292,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -435,6 +440,11 @@
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

### `src/operation/set_endpoint_attributes.rs`

```diff
--- reference/src/operation/set_endpoint_attributes.rs
+++ generated/src/operation/set_endpoint_attributes.rs
@@ -107,9 +107,9 @@
             "SNS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetEndpointAttributes")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetEndpointAttributesTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetEndpointAttributesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::set_endpoint_attributes::SetEndpointAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::set_endpoint_attributes::SetEndpointAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::set_endpoint_attributes::SetEndpointAttributesError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetEndpointAttributes")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    SetEndpointAttributesTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    SetEndpointAttributesEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::set_endpoint_attributes::SetEndpointAttributesError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::set_endpoint_attributes::SetEndpointAttributesError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::set_endpoint_attributes::SetEndpointAttributesError,
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
@@ -250,12 +259,11 @@
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
@@ -290,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -433,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::set_endpoint_attributes::SetEndpointAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::set_endpoint_attributes::SetEndpointAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/set_platform_application_attributes.rs`

```diff
--- reference/src/operation/set_platform_application_attributes.rs
+++ generated/src/operation/set_platform_application_attributes.rs
@@ -113,9 +113,9 @@
             "SNS",
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
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::set_platform_application_attributes::SetPlatformApplicationAttributesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::set_platform_application_attributes::SetPlatformApplicationAttributesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::set_platform_application_attributes::SetPlatformApplicationAttributesError,
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
@@ -258,14 +266,11 @@
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
@@ -300,8 +305,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +448,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::set_platform_application_attributes::SetPlatformApplicationAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::set_platform_application_attributes::SetPlatformApplicationAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("SetSMSAttributes", "SNS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -124,10 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetSMSAttributes")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                SetSmsAttributesTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetSMSAttributesEndpointParamsInterceptor,
+                SetSmsAttributesEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::set_sms_attributes::SetSMSAttributesError,
@@ -135,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::set_sms_attributes::SetSMSAttributesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::set_sms_attributes::SetSMSAttributesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::set_sms_attributes::SetSMSAttributesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -144,6 +153,44 @@
 }

 #[derive(Debug)]
+struct SetSmsAttributesTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetSmsAttributesTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "SetSmsAttributesTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<SetSmsAttributesInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct SetSMSAttributesResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for SetSMSAttributesResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -201,13 +248,12 @@
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
@@ -216,12 +262,12 @@
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
@@ -241,8 +287,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -384,6 +430,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::set_sms_attributes::SetSMSAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::set_sms_attributes::SetSMSAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/set_subscription_attributes.rs`

```diff
--- reference/src/operation/set_subscription_attributes.rs
+++ generated/src/operation/set_subscription_attributes.rs
@@ -107,9 +107,9 @@
             "SNS",
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
                 crate::operation::set_subscription_attributes::SetSubscriptionAttributesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::set_subscription_attributes::SetSubscriptionAttributesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::set_subscription_attributes::SetSubscriptionAttributesError,
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
@@ -260,12 +268,11 @@
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
@@ -300,8 +307,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -463,6 +470,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::set_subscription_attributes::SetSubscriptionAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::set_subscription_attributes::SetSubscriptionAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/set_topic_attributes.rs`

```diff
--- reference/src/operation/set_topic_attributes.rs
+++ generated/src/operation/set_topic_attributes.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("SetTopicAttributes", "SNS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetTopicAttributes")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetTopicAttributesTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetTopicAttributesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::set_topic_attributes::SetTopicAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::set_topic_attributes::SetTopicAttributesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::set_topic_attributes::SetTopicAttributesError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetTopicAttributes")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(SetTopicAttributesTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(SetTopicAttributesEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::set_topic_attributes::SetTopicAttributesError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::set_topic_attributes::SetTopicAttributesError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::set_topic_attributes::SetTopicAttributesError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,12 +249,11 @@
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
@@ -297,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::set_topic_attributes::SetTopicAttributesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::set_topic_attributes::SetTopicAttributesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -101,9 +101,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Subscribe", "SNS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -135,9 +135,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::subscribe::SubscribeError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::subscribe::SubscribeError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::subscribe::SubscribeError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,11 +258,10 @@
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
@@ -290,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -473,6 +478,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::subscribe::SubscribeError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::subscribe::SubscribeError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/tag_resource.rs`

```diff
--- reference/src/operation/tag_resource.rs
+++ generated/src/operation/tag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("TagResource", "SNS"));
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
                 crate::operation::tag_resource::TagResourceError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::tag_resource::TagResourceError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::tag_resource::TagResourceError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,13 +253,10 @@
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
@@ -287,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +463,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::tag_resource::TagResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::tag_resource::TagResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/unsubscribe.rs`

```diff
--- reference/src/operation/unsubscribe.rs
+++ generated/src/operation/unsubscribe.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Unsubscribe", "SNS"));
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
                 crate::operation::unsubscribe::UnsubscribeError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::unsubscribe::UnsubscribeError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::unsubscribe::UnsubscribeError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,12 +251,10 @@
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
@@ -284,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -437,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::unsubscribe::UnsubscribeError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::unsubscribe::UnsubscribeError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UntagResource", "SNS"));
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
                 crate::operation::untag_resource::UntagResourceError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::untag_resource::UntagResourceError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::untag_resource::UntagResourceError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,13 +253,10 @@
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
@@ -287,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +463,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::untag_resource::UntagResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::untag_resource::UntagResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
@@ -107,9 +107,9 @@
             "SNS",
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
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError,
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
@@ -150,12 +158,12 @@
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
@@ -250,12 +258,11 @@
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
@@ -265,12 +272,12 @@
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
@@ -290,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -453,6 +460,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
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
- `LICENSE`
- `README.md`
- `src/config/auth.rs`
- `src/config/endpoint.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/diagnostic.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/partition.rs`
- `src/endpoint_lib.rs`
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
- `src/serialization_settings.rs`
- `tests/endpoint_tests.rs`

### Rust token differences

- `src/client/create_platform_application.rs`
- `src/client/create_platform_endpoint.rs`
- `src/client/create_topic.rs`
- `src/client/get_endpoint_attributes.rs`
- `src/client/get_topic_attributes.rs`
- `src/client/publish.rs`
- `src/client/set_sms_attributes.rs`
- `src/client/set_topic_attributes.rs`
- `src/config.rs`
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
