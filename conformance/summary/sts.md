# AWS SDK Conformance Report: sts

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sts
**Progress:** `152/152` files compared · `63` matched · `24` mismatches · `65` missing · `0` extra · `41.45%` match (100.00% means fully matched)

### `src/client/assume_role.rs`

```diff
--- reference/src/client/assume_role.rs
+++ generated/src/client/assume_role.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`role_arn(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_role_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the role to assume.</p><br>
-    ///   - [`role_session_name(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::role_session_name) / [`set_role_session_name(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_role_session_name):<br>required: **true**<br><p>An identifier for the assumed role session.</p> <p>Use the role session name to uniquely identify a session when the same role is assumed by different principals or for different reasons. In cross-account scenarios, the role session name is visible to, and can be logged by the account that owns the role. The role session name is also used in the ARN of the assumed role principal. This means that subsequent cross-account API requests that use the temporary security credentials will expose the role session name to the external account in their CloudTrail logs.</p> <p>For security purposes, administrators can view this field in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/cloudtrail-integration.html#cloudtrail-integration_signin-tempcreds">CloudTrail logs</a> to help identify who performed an action in Amazon Web Services. Your administrator might require that you specify your user name as the session name when you assume the role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_iam-condition-keys.html#ck_rolesessionname"> <code>sts:RoleSessionName</code> </a>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: +=,.@-</p><br>
+    ///   - [`role_session_name(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::role_session_name) / [`set_role_session_name(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_role_session_name):<br>required: **true**<br><p>An identifier for the assumed role session.</p> <p>Use the role session name to uniquely identify a session when the same role is assumed by different principals or for different reasons. In cross-account scenarios, the role session name is visible to, and can be logged by the account that owns the role. The role session name is also used in the ARN of the assumed role principal. This means that subsequent cross-account API requests that use the temporary security credentials will expose the role session name to the external account in their CloudTrail logs.</p> <p>For security purposes, administrators can view this field in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/cloudtrail-integration.html#cloudtrail-integration_signin-tempcreds">CloudTrail logs</a> to help identify who performed an action in Amazon Web Services. Your administrator might require that you specify your user name as the session name when you assume the role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_iam-condition-keys.html#ck_rolesessionname"> <code>sts:RoleSessionName</code></a>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: +=,.@-</p><br>
     ///   - [`policy_arns(PolicyDescriptorType)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::policy_arns) / [`set_policy_arns(Option<Vec::<PolicyDescriptorType>>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_policy_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p> <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p><note>  <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p> </note> <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p><br>
     ///   - [`policy(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_policy):<br>required: **false**<br><p>An IAM policy in JSON format that you want to use as an inline session policy.</p> <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p> <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p><note>  <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p> </note> <p>For more information about role session permissions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session policies</a>.</p><br>
     ///   - [`duration_seconds(i32)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_duration_seconds):<br>required: **false**<br><p>The duration, in seconds, of the role session. The value specified can range from 900 seconds (15 minutes) up to the maximum session duration set for the role. The maximum session duration setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting or the administrator setting (whichever is lower), the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails.</p> <p>Role chaining limits your Amazon Web Services CLI or Amazon Web Services API role session to a maximum of one hour. When you use the <code>AssumeRole</code> API operation to assume a role, you can specify the duration of your role session with the <code>DurationSeconds</code> parameter. You can specify a parameter value of up to 43200 seconds (12 hours), depending on the maximum session duration setting for your role. However, if you assume a role using role chaining and provide a <code>DurationSeconds</code> parameter value greater than one hour, the operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_update-role-settings.html#id_roles_update-session-duration">Update the maximum session duration for a role</a>.</p> <p>By default, the value is set to <code>3600</code> seconds.</p><note>  <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p> </note><br>
@@ -13,8 +13,8 @@
     ///   - [`external_id(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::external_id) / [`set_external_id(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_external_id):<br>required: **false**<br><p>A unique identifier that might be required when you assume a role in another account. If the administrator of the account to which the role belongs provided you with an external ID, then provide that value in the <code>ExternalId</code> parameter. This value can be any string, such as a passphrase or account number. A cross-account role is usually set up to trust everyone in an account. Therefore, the administrator of the trusting account might send an external ID to the administrator of the trusted account. That way, only someone with the ID can assume the role, rather than everyone in the account. For more information about the external ID, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user_externalid.html">How to Use an External ID When Granting Access to Your Amazon Web Services Resources to a Third Party</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: +=,.@:\/-</p><br>
     ///   - [`serial_number(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::serial_number) / [`set_serial_number(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_serial_number):<br>required: **false**<br><p>The identification number of the MFA device that is associated with the user who is making the <code>AssumeRole</code> call. Specify this value if the trust policy of the role being assumed includes a condition that requires MFA authentication. The value is either the serial number for a hardware device (such as <code>GAHT12345678</code>) or an Amazon Resource Name (ARN) for a virtual device (such as <code>arn:aws:iam::123456789012:mfa/user</code>).</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: +=/:,.@-</p><br>
     ///   - [`token_code(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::token_code) / [`set_token_code(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_token_code):<br>required: **false**<br><p>The value provided by the MFA device, if the trust policy of the role being assumed requires MFA. (In other words, if the policy includes a condition that tests for MFA). If the role being assumed requires MFA and if the <code>TokenCode</code> value is missing or expired, the <code>AssumeRole</code> call returns an "access denied" error.</p> <p>The format for this parameter, as described by its regex pattern, is a sequence of six numeric digits.</p><br>
-    ///   - [`source_identity(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::source_identity) / [`set_source_identity(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_source_identity):<br>required: **false**<br><p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation. The source identity value persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#iam-term-role-chaining">chained role</a> sessions.</p> <p>You can require users to specify a source identity when they assume a role. You do this by using the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceidentity"> <code>sts:SourceIdentity</code> </a> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: +=,.@-. You cannot use a value that begins with the text <code>aws:</code>. This prefix is reserved for Amazon Web Services internal use.</p><br>
-    ///   - [`provided_contexts(ProvidedContext)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::provided_contexts) / [`set_provided_contexts(Option<Vec::<ProvidedContext>>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_provided_contexts):<br>required: **false**<br><p>A list of previously acquired trusted context assertions in the format of a JSON array. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p> <p>The following is an example of a <code>ProvidedContext</code> value that includes a single trusted context assertion and the ARN of the context provider from which the trusted context assertion was generated.</p> <p><code>\[{"ProviderArn":"arn:aws:iam::aws:contextProvider/IdentityCenter","ContextAssertion":"trusted-context-assertion"}\]</code></p><br>
+    ///   - [`source_identity(impl Into<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::source_identity) / [`set_source_identity(Option<String>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_source_identity):<br>required: **false**<br><p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation. The source identity value persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#iam-term-role-chaining">chained role</a> sessions.</p> <p>You can require users to specify a source identity when they assume a role. You do this by using the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceidentity"> <code>sts:SourceIdentity</code></a> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: +=,.@-. You cannot use a value that begins with the text <code>aws:</code>. This prefix is reserved for Amazon Web Services internal use.</p><br>
+    ///   - [`provided_contexts(ProvidedContext)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::provided_contexts) / [`set_provided_contexts(Option<Vec::<ProvidedContext>>)`](crate::operation::assume_role::builders::AssumeRoleFluentBuilder::set_provided_contexts):<br>required: **false**<br><p>A list of previously acquired trusted context assertions in the format of a JSON array. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p> <p>The following is an example of a <code>ProvidedContext</code> value that includes a single trusted context assertion and the ARN of the context provider from which the trusted context assertion was generated.</p> <p><code>[{"ProviderArn":"arn:aws:iam::aws:contextProvider/IdentityCenter","ContextAssertion":"trusted-context-assertion"}]</code></p><br>
     /// - On success, responds with [`AssumeRoleOutput`](crate::operation::assume_role::AssumeRoleOutput) with field(s):
     ///   - [`credentials(Option<Credentials>)`](crate::operation::assume_role::AssumeRoleOutput::credentials): <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p><note>  <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p> </note>
     ///   - [`assumed_role_user(Option<AssumedRoleUser>)`](crate::operation::assume_role::AssumeRoleOutput::assumed_role_user): <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
```

### `src/client/assume_role_with_web_identity.rs`

```diff
--- reference/src/client/assume_role_with_web_identity.rs
+++ generated/src/client/assume_role_with_web_identity.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`role_arn(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_role_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p><note>  <p>Additional considerations apply to Amazon Cognito identity pools that assume <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies-cross-account-resource-access.html">cross-account IAM roles</a>. The trust policies of these roles must accept the <code>cognito-identity.amazonaws.com</code> service principal and must contain the <code>cognito-identity.amazonaws.com:aud</code> condition key to restrict role assumption to users from your intended identity pools. A policy that trusts Amazon Cognito identity pools without this condition creates a risk that a user from an unintended identity pool can assume the role. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/iam-roles.html#trust-policies"> Trust policies for IAM roles in Basic (Classic) authentication </a> in the <i>Amazon Cognito Developer Guide</i>.</p> </note><br>
-    ///   - [`role_session_name(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::role_session_name) / [`set_role_session_name(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_role_session_name):<br>required: **true**<br><p>An identifier for the assumed role session. Typically, you pass the name or identifier that is associated with the user who is using your application. That way, the temporary security credentials that your application will use are associated with that user. This session name is included as part of the ARN and assumed role ID in the <code>AssumedRoleUser</code> response element.</p> <p>For security purposes, administrators can view this field in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/cloudtrail-integration.html#cloudtrail-integration_signin-tempcreds">CloudTrail logs</a> to help identify who performed an action in Amazon Web Services. Your administrator might require that you specify your user name as the session name when you assume the role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_iam-condition-keys.html#ck_rolesessionname"> <code>sts:RoleSessionName</code> </a>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p><br>
+    ///   - [`role_session_name(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::role_session_name) / [`set_role_session_name(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_role_session_name):<br>required: **true**<br><p>An identifier for the assumed role session. Typically, you pass the name or identifier that is associated with the user who is using your application. That way, the temporary security credentials that your application will use are associated with that user. This session name is included as part of the ARN and assumed role ID in the <code>AssumedRoleUser</code> response element.</p> <p>For security purposes, administrators can view this field in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/cloudtrail-integration.html#cloudtrail-integration_signin-tempcreds">CloudTrail logs</a> to help identify who performed an action in Amazon Web Services. Your administrator might require that you specify your user name as the session name when you assume the role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_iam-condition-keys.html#ck_rolesessionname"> <code>sts:RoleSessionName</code></a>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p><br>
     ///   - [`web_identity_token(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::web_identity_token) / [`set_web_identity_token(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_web_identity_token):<br>required: **true**<br><p>The OAuth 2.0 access token or OpenID Connect ID token that is provided by the identity provider. Your application must get this token by authenticating the user who is using your application with a web identity provider before the application makes an <code>AssumeRoleWithWebIdentity</code> call. Timestamps in the token must be formatted as either an integer or a long integer. Tokens must be signed using either RSA keys (RS256, RS384, or RS512) or ECDSA keys (ES256, ES384, or ES512).</p><br>
     ///   - [`provider_id(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::provider_id) / [`set_provider_id(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_provider_id):<br>required: **false**<br><p>The fully qualified host component of the domain name of the OAuth 2.0 identity provider. Do not specify this value for an OpenID Connect identity provider.</p> <p>Currently <code>www.amazon.com</code> and <code>graph.facebook.com</code> are the only supported identity providers for OAuth 2.0 access tokens. Do not include URL schemes and port numbers.</p> <p>Do not specify this value for OpenID Connect ID tokens.</p><br>
     ///   - [`policy_arns(PolicyDescriptorType)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::policy_arns) / [`set_policy_arns(Option<Vec::<PolicyDescriptorType>>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_policy_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p> <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p><note>  <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p> </note> <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p><br>
```

### `src/client.rs`

```diff
--- reference/src/client.rs
+++ generated/src/client.rs
@@ -1,140 +1,245 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#[derive(Debug)]
-pub(crate) struct Handle {
-    pub(crate) conf: crate::Config,
-    #[allow(dead_code)] // unused when a service does not provide any operations
-    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
-}

-/// Client for AWS Security Token Service
-///
-/// Client for invoking operations on AWS Security Token Service. Each operation on AWS Security Token Service is a method on this
-/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
-/// # Using the `Client`
-///
-/// A client has a function for every operation that can be performed by the service.
-/// For example, the [`AssumeRole`](crate::operation::assume_role) operation has
-/// a [`Client::assume_role`], function which returns a builder for that operation.
-/// The fluent builder ultimately has a `send()` function that returns an async future that
-/// returns a result, as illustrated below:
-///
-/// ```rust,ignore
-/// let result = client.assume_role()
-///     .role_arn("example")
-///     .send()
-///     .await;
-/// ```
-///
-/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
-/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
-/// information.
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct Client {
-    handle: ::std::sync::Arc<Handle>,
-}
+#[allow(dead_code)]
+pub(crate) mod transport {
+    use ::std::collections::BTreeMap;
+    use ::std::fmt;
+    use ::std::io::{Read, Write};
+    use ::std::net::TcpStream;

-impl Client {
-    /// Creates a new client from the service [`Config`](crate::Config).
-    ///
-    /// # Panics
-    ///
-    /// This method will panic in the following cases:
-    ///
-    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
-    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
-    /// - No `behavior_version` is provided.
-    ///
-    /// The panic message for each of these will have instructions on how to resolve them.
-    #[track_caller]
-    pub fn from_conf(conf: crate::Config) -> Self {
-        let handle = Handle {
-            conf: conf.clone(),
-            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
-        };
-        if let Err(err) = Self::validate_config(&handle) {
-            panic!("Invalid client configuration: {err}");
-        }
-        Self {
-            handle: ::std::sync::Arc::new(handle),
-        }
+    #[derive(Clone, Copy, Debug)]
+    pub(crate) enum Method {
+        Get,
+        Put,
+        Post,
+        Delete,
+        Head,
+        Patch,
     }

-    /// Returns the client's configuration.
-    pub fn config(&self) -> &crate::Config {
-        &self.handle.conf
+    impl Method {
+        fn as_str(self) -> &'static str {
+            match self {
+                Self::Get => "GET",
+                Self::Put => "PUT",
+                Self::Post => "POST",
+                Self::Delete => "DELETE",
+                Self::Head => "HEAD",
+                Self::Patch => "PATCH",
+            }
+        }
     }

-    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
-        handle
-            .runtime_plugins
-            .apply_client_configuration(&mut cfg)?
-            .validate_base_client_config(&cfg)?;
-        Ok(())
+    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
+    pub(crate) struct StatusCode(u16);
+
+    impl StatusCode {
+        pub(crate) const CONFLICT: Self = Self(409);
+        pub(crate) fn is_success(self) -> bool {
+            (200..300).contains(&self.0)
+        }
     }
-}

-impl Client {
-    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
-    ///
-    /// # Panics
-    ///
-    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
-    ///   the `sleep_impl` on the Config passed into this function to fix it.
-    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
-    ///   `http_connector` on the Config passed into this function to fix it.
-    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.
-    #[track_caller]
-    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Self::from_conf(sdk_config.into())
+    impl fmt::Display for StatusCode {
+        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
+            self.0.fmt(formatter)
+        }
     }
-}

-mod assume_role;
-
-mod assume_role_with_saml;
+    #[derive(Clone, Debug)]
+    pub(crate) struct Response {
+        status: StatusCode,
+        headers: BTreeMap<String, String>,
+        body: Vec<u8>,
+    }

-mod assume_role_with_web_identity;
-
-mod assume_root;
+    impl Response {
+        pub(crate) fn status(&self) -> StatusCode {
+            self.status
+        }
+        pub(crate) fn header(&self, name: &str) -> Option<&str> {
+            self.headers.get(&name.to_ascii_lowercase()).map(String::as_str)
+        }
+        pub(crate) fn body(&self) -> &[u8] {
+            &self.body
+        }
+        pub(crate) async fn text(&self) -> Result<String, String> {
+            String::from_utf8(self.body.clone()).map_err(|error| error.to_string())
+        }
+    }

-/// Operation customization and supporting types.
-///
-/// The underlying HTTP requests made during an operation can be customized
-/// by calling the `customize()` method on the builder returned from a client
-/// operation call. For example, this can be used to add an additional HTTP header:
-///
-/// ```ignore
-/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_sts::Error> {
-/// # let client: aws_sdk_sts::Client = unimplemented!();
-/// use ::http_1x::header::{HeaderName, HeaderValue};
-///
-/// let result = client.assume_role()
-///     .customize()
-///     .mutate_request(|req| {
-///         // Add `x-example-header` with value
-///         req.headers_mut()
-///             .insert(
-///                 HeaderName::from_static("x-example-header"),
-///                 HeaderValue::from_static("1"),
-///             );
-///     })
-///     .send()
-///     .await;
-/// # }
-/// ```
-pub mod customize;
+    #[derive(Clone, Debug, Default)]
+    pub(crate) struct HttpClient;

-mod decode_authorization_message;
+    impl HttpClient {
+        pub(crate) fn new() -> Self {
+            Self
+        }
+        pub(crate) async fn request(&self, method: Method, url: &str, headers: &[(&str, &str)], body: &[u8]) -> Result<Response, String> {
+            let (host, port, path) = parse_http_url(url)?;
+            let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|error| format!("failed to connect to {host}:{port}: {error}"))?;
+            let mut request = format!(
+                "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Length: {}\r\n",
+                method.as_str(),
+                path,
+                host,
+                body.len()
+            );
+            for (name, value) in headers {
+                request.push_str(name);
+                request.push_str(": ");
+                request.push_str(value);
+                request.push_str("\r\n");
+            }
+            request.push_str("\r\n");
+            let mut request_bytes = request.into_bytes();
+            request_bytes.extend_from_slice(body);
+            stream
+                .write_all(&request_bytes)
+                .map_err(|error| format!("failed to write HTTP request: {error}"))?;
+            let mut bytes = Vec::new();
+            stream
+                .read_to_end(&mut bytes)
+                .map_err(|error| format!("failed to read HTTP response: {error}"))?;
+            parse_response(&bytes)
+        }
+    }

-mod get_access_key_info;
+    fn parse_http_url(url: &str) -> Result<(String, u16, String), String> {
+        let authority_and_path = url
+            .strip_prefix("http://")
+            .ok_or_else(|| format!("only http:// endpoints are supported: {url}"))?;
+        let (authority, path) = authority_and_path
+            .split_once('/')
+            .map_or((authority_and_path, "/"), |(authority, _path)| {
+                (authority, &authority_and_path[authority.len()..])
+            });
+        if authority.is_empty() {
+            return Err(format!("endpoint has no host: {url}"));
+        }
+        let (host, port) = if let Some((host, port)) = authority.rsplit_once(':') {
+            let port = port.parse::<u16>().map_err(|error| format!("invalid endpoint port in {url}: {error}"))?;
+            (host.to_owned(), port)
+        } else {
+            (authority.to_owned(), 80)
+        };
+        Ok((host, port, path.to_owned()))
+    }

-mod get_caller_identity;
+    fn parse_response(bytes: &[u8]) -> Result<Response, String> {
+        let header_end = bytes
+            .windows(4)
+            .position(|window| window == b"\r\n\r\n")
+            .ok_or_else(|| "HTTP response did not contain a header terminator".to_owned())?;
+        let header = ::std::str::from_utf8(&bytes[..header_end]).map_err(|error| format!("HTTP response headers were not UTF-8: {error}"))?;
+        let status = header
+            .lines()
+            .next()
+            .and_then(|line| line.split_whitespace().nth(1))
+            .ok_or_else(|| "HTTP response did not contain a status code".to_owned())?
+            .parse::<u16>()
+            .map_err(|error| format!("HTTP response status was invalid: {error}"))?;
+        let mut headers = BTreeMap::new();
+        for line in header.lines().skip(1) {
+            if let Some((name, value)) = line.split_once(':') {
+                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
+            }
+        }
+        Ok(Response {
+            status: StatusCode(status),
+            headers,
+            body: bytes[header_end + 4..].to_vec(),
+        })
+    }

-mod get_delegated_access_token;
+    pub(crate) fn encode_path(value: &str) -> String {
+        value.bytes().fold(String::new(), |mut result, byte| {
+            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/') {
+                result.push(byte as char);
+            } else {
+                result.push('%');
+                result.push(hex(byte >> 4));
+                result.push(hex(byte & 0x0f));
+            }
+            result
+        })
+    }
+    fn hex(value: u8) -> char {
+        match value {
+            0..=9 => (b'0' + value) as char,
+            _ => (b'A' + value - 10) as char,
+        }
+    }
+    pub(crate) fn xml_escape(value: &str) -> String {
+        value
+            .replace('&', "&amp;")
+            .replace('<', "&lt;")
+            .replace('>', "&gt;")
+            .replace('\"', "&quot;")
+            .replace('\'', "&apos;")
+    }
+    pub(crate) fn xml_unescape(value: &str) -> String {
+        value
+            .replace("&lt;", "<")
+            .replace("&gt;", ">")
+            .replace("&apos;", "'")
+            .replace("&amp;", "&")
+    }
+    pub(crate) fn xml_first(xml: &str, tag: &str) -> Option<String> {
+        xml_tags(xml, tag).into_iter().next().map(|value| xml_unescape(&value))
+    }
+    pub(crate) fn xml_tags(xml: &str, tag: &str) -> Vec<String> {
+        let open = format!("<{tag}>");
+        let close = format!("</{tag}>");
+        let mut values = Vec::new();
+        let mut remaining = xml;
+        while let Some(start) = remaining.find(&open) {
+            let value_start = start + open.len();
+            let Some(end) = remaining[value_start..].find(&close) else { break };
+            values.push(remaining[value_start..value_start + end].to_owned());
+            remaining = &remaining[value_start + end + close.len()..];
+        }
+        values
+    }
+}

-mod get_federation_token;
+// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

-mod get_session_token;
+#[derive(Clone, Debug, Default)]
+pub struct Client {
+    config: Config,
+    http: transport::HttpClient,
+}
+impl Client {
+    pub fn new(config: &Config) -> Self {
+        Self {
+            config: config.clone(),
+            http: transport::HttpClient::new(),
+        }
+    }
+    pub fn config(&self) -> &Config {
+        &self.config
+    }
+    pub(crate) async fn request(
+        &self,
+        method: transport::Method,
+        path: &str,
+        headers: &[(&str, &str)],
+        body: &[u8],
+    ) -> ::std::result::Result<transport::Response, ::std::string::String> {
+        let url = format!("{}{}", self.config.endpoint_url.trim_end_matches('/'), path);
+        self.http.request(method, &url, headers, body).await
+    }
+}

-mod get_web_identity_token;
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/assume_role.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/assume_role_with_saml.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/assume_role_with_web_identity.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/assume_root.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/decode_authorization_message.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/get_access_key_info.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/get_caller_identity.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/get_delegated_access_token.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/get_federation_token.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/get_session_token.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client/get_web_identity_token.rs"));
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -1,1747 +1,45 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#![allow(clippy::empty_line_after_doc_comments)]
-/// Configuration for a aws_sdk_sts service client.
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
-
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
-        "sts"
-    }
-    /// Returns the SigV4a signing region set, if configured.
-    pub fn sigv4a_signing_region_set(&self) -> Option<&::aws_types::region::SigningRegionSet> {
-        self.config.load::<::aws_types::region::SigningRegionSet>()
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
-        builder.set_sigv4a_signing_region_set(config_bag.load::<::aws_types::region::SigningRegionSet>().cloned());
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
-    /// use aws_sdk_sts::config::Config;
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
-    /// use aws_sdk_sts::config::{Builder, Config};
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
-    /// let mut builder = aws_sdk_sts::Config::builder();
-    /// override_http_client(&mut builder);
-    /// let config = builder.build();
-    /// # }
-    /// # }
-    /// ```
-    pub fn set_http_client(&mut self, http_client: Option<crate::config::SharedHttpClient>) -> &mut Self {
-        self.runtime_components.set_http_client(http_client);
-        self
-    }
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
-    /// impl aws_sdk_sts::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_sts::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         AuthSchemeOptionsFuture::ready(Ok(vec![AuthSchemeOption::from(AuthSchemeId::new(
-    ///             "custom",
-    ///         ))]))
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
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
-    /// impl aws_sdk_sts::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_sts::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
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
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
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
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
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
-    /// rules for `aws_sdk_sts`.
-    ///
-    ///
-    /// Note: setting an endpoint resolver will replace any endpoint URL that has been set.
-    /// This method accepts an endpoint resolver [specific to this service](crate::config::endpoint::ResolveEndpoint). If you want to
-    /// provide a shared endpoint resolver, use [`Self::set_endpoint_resolver`].
-    ///
-    /// # Examples
-    /// Create a custom endpoint resolver that resolves a different endpoing per-stage, e.g. staging vs. production.
-    /// ```no_run
-    /// use aws_sdk_sts::config::endpoint::{ResolveEndpoint, EndpointFuture, Params, Endpoint};
-    /// #[derive(Debug)]
-    /// struct StageResolver { stage: String }
-    /// impl ResolveEndpoint for StageResolver {
-    ///     fn resolve_endpoint(&self, params: &Params) -> EndpointFuture<'_> {
-    ///         let stage = &self.stage;
-    ///         EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{stage}.myservice.com")).build()))
-    ///     }
-    /// }
-    /// let resolver = StageResolver { stage: std::env::var("STAGE").unwrap() };
-    /// let config = aws_sdk_sts::Config::builder().endpoint_resolver(resolver).build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
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
-    /// rules for `aws_sdk_sts`.
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
-    /// use aws_sdk_sts::config::Config;
-    /// use aws_sdk_sts::config::retry::RetryConfig;
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
-    /// use aws_sdk_sts::config::{Builder, Config};
-    /// use aws_sdk_sts::config::retry::RetryConfig;
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
-    /// use aws_sdk_sts::config::{AsyncSleep, Config, SharedAsyncSleep, Sleep};
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
-    /// use aws_sdk_sts::config::{AsyncSleep, Builder, Config, SharedAsyncSleep, Sleep};
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
-    /// use aws_sdk_sts::config::Config;
-    /// use aws_sdk_sts::config::timeout::TimeoutConfig;
-    ///
-    /// let timeout_config = TimeoutConfig::builder()
-    ///     .operation_attempt_timeout(Duration::from_secs(1))
-    ///     .build();
-    /// let config = Config::builder().timeout_config(timeout_config).build();
-    /// ```
-    pub fn timeout_config(mut self, timeout_config: ::aws_smithy_types::timeout::TimeoutConfig) -> Self {
-        self.set_timeout_config(Some(timeout_config));
-        self
+pub mod config {
+    #[derive(Clone, Debug, Default)]
+    pub struct Builder {
+        endpoint_url: ::std::option::Option<::std::string::String>,
     }
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
-    /// use aws_sdk_sts::config::{Builder, Config};
-    /// use aws_sdk_sts::config::timeout::TimeoutConfig;
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
-    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `sts`
-    /// (or `sts-<region>` if a region is configured).
-    /// All STS clients without an explicit retry partition will share this default partition.
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
-    /// use aws_sdk_sts::config::Config;
-    /// use aws_sdk_sts::config::retry::{RetryPartition, TokenBucket};
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
-    /// use aws_sdk_sts::config::Config;
-    /// use aws_sdk_sts::config::retry::{RetryPartition, TokenBucket};
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
-    /// use aws_sdk_sts::config::Config;
-    /// use aws_sdk_sts::config::retry::{ClientRateLimiter, RetryConfig, RetryPartition};
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
-    /// use aws_sdk_sts::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_sts::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
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
-    /// use aws_sdk_sts::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_sts::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
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
-    /// use aws_sdk_sts::config::Config;
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
-    /// use aws_sdk_sts::config::Config;
-    /// # #[derive(Debug)]
-    /// # struct SomeOperationError {}
-    /// # impl StdError for SomeOperationError {}
-    /// # impl fmt::Display for SomeOperationError {
-    /// #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
-    /// # }
-    /// # impl ProvideErrorMetadata for SomeOperationError {
-    /// #    fn meta(&self) -> &aws_sdk_sts::error::ErrorMetadata { todo!() }
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
-    /// Sets the SigV4a signing region set.
-    pub fn sigv4a_signing_region_set(mut self, v: impl Into<::aws_types::region::SigningRegionSet>) -> Self {
-        self.set_sigv4a_signing_region_set(Some(v.into()));
-        self
-    }
-
-    /// Sets the SigV4a signing region set.
-    pub fn set_sigv4a_signing_region_set(&mut self, v: Option<::aws_types::region::SigningRegionSet>) -> &mut Self {
-        self.config.store_or_unset(v);
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_types::region::Region;
-    /// use aws_sdk_sts::config::{Builder, Config};
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
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
-            #[cfg(feature = "sigv4a")]
-            {
-                self.runtime_components
-                    .set_identity_resolver(::aws_runtime::auth::sigv4a::SCHEME_ID, credentials_provider.clone());
+        pub fn build(self) -> super::Config {
+            super::Config {
+                endpoint_url: self.endpoint_url.unwrap_or_else(|| super::Config::default().endpoint_url),
             }
-            self.runtime_components
-                .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
         }
-        self
-    }
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
-    /// use aws_sdk_sts::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_sts::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn behavior_version(mut self, behavior_version: crate::config::BehaviorVersion) -> Self {
-        self.set_behavior_version(Some(behavior_version));
-        self
     }
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
-    /// use aws_sdk_sts::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_sts::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_sts::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_sts::Client::from_conf(config);
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
-        layer.store_put(::aws_types::SigningName::from_static("sts"));
-        layer
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| layer.store_put(::aws_types::region::SigningRegion::from(r)));
-        Config {
-            config: crate::config::Layer::from(layer.clone())
-                .with_name("aws_sdk_sts::config::Config")
-                .freeze(),
-            cloneable: layer,
-            runtime_components: self.runtime_components,
-            runtime_plugins: self.runtime_plugins,
-            behavior_version: self.behavior_version,
-        }
-    }
-}
-#[derive(::std::fmt::Debug)]
-pub(crate) struct ServiceRuntimePlugin {
-    config: ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer>,
-    runtime_components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}
-
-impl ServiceRuntimePlugin {
-    pub fn new(_service_config: crate::config::Config) -> Self {
-        let config = {
-            let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AWSSecurityTokenServiceV20110615");
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
-        #[cfg(feature = "sigv4a")]
-        {
-            runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
-                ::aws_runtime::auth::sigv4a::SigV4aAuthScheme::new(),
-            ));
-        }
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
-
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
-                .with_name("aws_sdk_sts::config::ConfigOverrideRuntimePlugin")
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
-        builder.set_sigv4a_signing_region_set(input.sigv4a_signing_region_set().cloned());
-        builder.set_use_fips(input.use_fips());
-        builder.set_use_dual_stack(input.use_dual_stack());
-        if input.get_origin("endpoint_url").is_client_config() {
-            builder.set_endpoint_url(input.endpoint_url().map(|s| s.to_string()));
-        } else {
-            builder.set_endpoint_url(
-                input
-                    .service_config()
-                    .and_then(|conf| {
-                        conf.load_config(service_config_key("STS", "AWS_ENDPOINT_URL", "endpoint_url"))
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
+    impl From<&super::Config> for Builder {
+        fn from(config: &super::Config) -> Self {
+            Self {
+                endpoint_url: Some(config.endpoint_url.clone()),
+            }
         }
-        builder.set_app_name(input.app_name().cloned());
-        for framework_metadata in input.framework_metadata() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-
-        builder
     }
 }

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
+impl Config {
+    pub fn builder() -> config::Builder {
+        config::Builder::default()
     }
-
-    let default_retry_partition = "sts";
-    let default_retry_partition = match config.region() {
-        Some(region) => ::std::borrow::Cow::from(format!("{default_retry_partition}-{region}")),
-        None => ::std::borrow::Cow::from(default_retry_partition),
-    };
-
-    let scope = "aws-sdk-sts";
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
-    }
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

### `src/error.rs`

```diff
--- reference/src/error.rs
+++ generated/src/error.rs
@@ -1,13 +1,80 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use ::aws_smithy_runtime_api::box_error::BoxError;

-/// Error type returned by the client.
-pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> = ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
-pub use ::aws_smithy_runtime_api::client::result::ConnectorError;
-pub use ::aws_smithy_types::error::operation::BuildError;
-
-pub use ::aws_smithy_types::error::display::DisplayErrorContext;
-pub use ::aws_smithy_types::error::metadata::ErrorMetadata;
-pub use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
+#[derive(Clone, Debug)]
+pub struct Error {
+    message: ::std::string::String,
+}
+impl Error {
+    pub fn unhandled(message: impl ::std::convert::Into<::std::string::String>) -> Self {
+        Self { message: message.into() }
+    }
+    pub fn meta(&self) -> ErrorMetadata {
+        ErrorMetadata::default()
+    }
+}
+impl ::std::fmt::Display for Error {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        f.write_str(&self.message)
+    }
+}
+impl ::std::error::Error for Error {}
+#[derive(Clone, Debug, Default)]
+pub struct ErrorMetadata {
+    request_id: ::std::option::Option<::std::string::String>,
+    extended_request_id: ::std::option::Option<::std::string::String>,
+}
+impl ErrorMetadata {
+    pub(crate) fn from_request_ids(
+        request_id: ::std::option::Option<::std::string::String>,
+        extended_request_id: ::std::option::Option<::std::string::String>,
+    ) -> Self {
+        Self {
+            request_id,
+            extended_request_id,
+        }
+    }
+    pub fn request_id(&self) -> ::std::option::Option<&str> {
+        self.request_id.as_deref()
+    }
+    pub fn extended_request_id(&self) -> ::std::option::Option<&str> {
+        self.extended_request_id.as_deref()
+    }
+}
+#[derive(Clone, Debug)]
+pub struct UnknownVariantError {
+    value: ::std::string::String,
+}
+impl UnknownVariantError {
+    pub(crate) fn new(value: impl ::std::convert::Into<::std::string::String>) -> Self {
+        Self { value: value.into() }
+    }
+}
+impl ::std::fmt::Display for UnknownVariantError {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        write!(f, "unknown enum variant: '{}'", self.value)
+    }
+}
+impl ::std::error::Error for UnknownVariantError {}
+pub mod error {
+    pub use super::{BuildError, Error, ErrorMetadata, UnknownVariantError};
+}

-pub(crate) mod sealed_unhandled;
+#[derive(Clone, Debug)]
+pub struct BuildError {
+    field: ::std::string::String,
+    message: ::std::string::String,
+}
+impl BuildError {
+    pub fn missing_field(field: impl ::std::convert::Into<::std::string::String>, message: impl ::std::convert::Into<::std::string::String>) -> Self {
+        Self {
+            field: field.into(),
+            message: message.into(),
+        }
+    }
+}
+impl ::std::fmt::Display for BuildError {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        write!(f, "{}: {}", self.field, self.message)
+    }
+}
+impl ::std::error::Error for BuildError {}
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -1,161 +1,14 @@
-#![allow(deprecated)]
-#![allow(unknown_lints)]
-#![allow(clippy::module_inception)]
-#![allow(clippy::upper_case_acronyms)]
-#![allow(clippy::large_enum_variant)]
-#![allow(clippy::wrong_self_convention)]
-#![allow(clippy::should_implement_trait)]
-#![allow(clippy::disallowed_names)]
-#![allow(clippy::vec_init_then_push)]
-#![allow(clippy::type_complexity)]
-#![allow(clippy::needless_return)]
-#![allow(clippy::derive_partial_eq_without_eq)]
-#![allow(clippy::result_large_err)]
-#![allow(clippy::unnecessary_map_on_constructor)]
-#![allow(clippy::useless_conversion)]
-#![allow(clippy::deprecated_semver)]
-#![allow(rustdoc::bare_urls)]
-#![allow(rustdoc::redundant_explicit_links)]
-#![allow(rustdoc::broken_intra_doc_links)]
-#![allow(rustdoc::invalid_html_tags)]
-#![forbid(unsafe_code)]
-#![warn(missing_docs)]
-#![cfg_attr(docsrs, feature(doc_cfg))]
-//! Security Token Service (STS) enables you to request temporary, limited-privilege credentials for users. This guide provides descriptions of the STS API. For more information about using this service, see [Temporary Security Credentials](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html).
-//!
-//! ## Getting Started
-//!
-//! > Examples are available for many services and operations, check out the
-//! > [usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1).
-//!
-//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
-//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-sts` to
-//! your project, add the following to your **Cargo.toml** file:
-//!
-//! ```toml
-//! [dependencies]
-//! aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
-//! aws-sdk-sts = "1.112.0"
-//! tokio = { version = "1", features = ["full"] }
-//! ```
-//!
-//! Then in code, a client can be created with the following:
-//!
-//! ```rust,ignore
-//! use aws_sdk_sts as sts;
-//!
-//! #[::tokio::main]
-//! async fn main() -> Result<(), sts::Error> {
-//!     let config = aws_config::load_from_env().await;
-//!     let client = aws_sdk_sts::Client::new(&config);
-//!
-//!     // ... make some calls with the client
-//!
-//!     Ok(())
-//! }
-//! ```
-//!
-//! See the [client documentation](https://docs.rs/aws-sdk-sts/latest/aws_sdk_sts/client/struct.Client.html)
-//! for information on what calls can be made, and the inputs and outputs for each of those calls.
-//!
-//! ## Using the SDK
-//!
-//! Until the SDK is released, we will be adding information about using the SDK to the
-//! [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
-//! additional sections for the guide by opening an issue and describing what you are trying to do.
-//!
-//! ## Getting Help
-//!
-//! * [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
-//! * [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
-//! * [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
-//! * [Usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1)
-//!
-//!
-//! # Crate Organization
-//!
-//! The entry point for most customers will be [`Client`], which exposes one method for each API
-//! offered by AWS Security Token Service. The return value of each of these methods is a "fluent builder",
-//! where the different inputs for that API are added by builder-style function call chaining,
-//! followed by calling `send()` to get a [`Future`](std::future::Future) that will result in
-//! either a successful output or a [`SdkError`](crate::error::SdkError).
-//!
-//! Some of these API inputs may be structs or enums to provide more complex structured information.
-//! These structs and enums live in [`types`](crate::types). There are some simpler types for
-//! representing data such as date times or binary blobs that live in [`primitives`](crate::primitives).
-//!
-//! All types required to configure a client via the [`Config`](crate::Config) struct live
-//! in [`config`](crate::config).
-//!
-//! The [`operation`](crate::operation) module has a submodule for every API, and in each submodule
-//! is the input, output, and error type for that API, as well as builders to construct each of those.
-//!
-//! There is a top-level [`Error`](crate::Error) type that encompasses all the errors that the
-//! client can return. Any other error type can be converted to this `Error` type via the
-//! [`From`](std::convert::From) trait.
-//!
-//! The other modules within this crate are not required for normal usage.
-
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use error_meta::Error;
-
-#[doc(inline)]
-pub use config::Config;
-
-/// Client for calling AWS Security Token Service.
-/// # Using the `Client`
-///
-/// A client has a function for every operation that can be performed by the service.
-/// For example, the [`AssumeRole`](crate::operation::assume_role) operation has
-/// a [`Client::assume_role`], function which returns a builder for that operation.
-/// The fluent builder ultimately has a `send()` function that returns an async future that
-/// returns a result, as illustrated below:
-///
-/// ```rust,ignore
-/// let result = client.assume_role()
-///     .role_arn("example")
-///     .send()
-///     .await;
-/// ```
-///
-/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
-/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
-/// information.
-pub mod client;
-
-/// Configuration for AWS Security Token Service.
-pub mod config;
-
-/// Common errors and error handling utilities.
-pub mod error;

-mod error_meta;
-
-/// Information about this crate.
-pub mod meta;
-
-/// All operations that this crate can perform.
-pub mod operation;
-
-/// Primitives such as `Blob` or `DateTime` used by other types.
-pub mod primitives;
-
-/// Data structures used by operation inputs/outputs.
-pub mod types;
-
-mod observability_feature;
-
-pub(crate) mod protocol_serde;
-
-mod sdk_feature_tracker;
-
-mod serialization_settings;
-
-mod endpoint_lib;
-
-mod rest_xml_wrapped_errors;
-
-mod serde_util;
-
-#[doc(inline)]
-pub use client::Client;
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/primitives.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/error.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/meta.rs"));
+pub mod types {
+    include!(concat!(env!("OUT_DIR"), "/generated/sts/src/types.rs"));
+}
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/sts/src/client.rs"));
+mod serde_util {
+    include!(concat!(env!("OUT_DIR"), "/generated/sts/src/serde_util.rs"));
+}
```

### `src/operation/assume_role.rs`

```diff
--- reference/src/operation/assume_role.rs
+++ generated/src/operation/assume_role.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRole", "STS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -143,7 +143,7 @@
                 ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::assume_role::AssumeRoleError>::builder()
                     .transient_errors({
                         let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
+                        transient_errors.push("InternalError");
                         ::std::borrow::Cow::Owned(transient_errors)
                     })
                     .build(),
@@ -282,12 +282,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_role_input::ser_assume_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_role_input::ser_assume_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -321,8 +319,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -465,6 +463,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::assume_role::AssumeRoleError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::assume_role::AssumeRoleError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/assume_role_with_saml/builders.rs`

```diff
--- reference/src/operation/assume_role_with_saml/builders.rs
+++ generated/src/operation/assume_role_with_saml/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            crate::operation::assume_role_with_saml::AssumeRoleWithSamlError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `AssumeRoleWithSAML`.
+/// Fluent builder constructing a request to `AssumeRoleWithSaml`.
 ///
 /// <p>Returns a set of temporary security credentials for users who have been authenticated via a SAML authentication response. This operation provides a mechanism for tying an enterprise identity store or directory to role-based Amazon Web Services access without user-specific credentials or configuration. For a comparison of <code>AssumeRoleWithSAML</code> with the other API operations that produce temporary credentials, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_request.html">Requesting Temporary Security Credentials</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_sts-comparison.html">Compare STS credentials</a> in the <i>IAM User Guide</i>.</p>
 /// <p>The temporary security credentials returned by this operation consist of an access key ID, a secret access key, and a security token. Applications can use these temporary security credentials to sign calls to Amazon Web Services services.</p><note>
@@ -58,7 +58,7 @@
 /// <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-idp_saml.html">Creating a Role for SAML 2.0 Federation</a> in the <i>IAM User Guide</i>.</p></li>
 /// </ul>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct AssumeRoleWithSAMLFluentBuilder {
+pub struct AssumeRoleWithSamlFluentBuilder {
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
@@ -66,8 +66,8 @@
 impl
     crate::client::customize::internal::CustomizableSend<
         crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-        crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
-    > for AssumeRoleWithSAMLFluentBuilder
+        crate::operation::assume_role_with_saml::AssumeRoleWithSamlError,
+    > for AssumeRoleWithSamlFluentBuilder
 {
     fn send(
         self,
@@ -75,14 +75,14 @@
     ) -> crate::client::customize::internal::BoxFuture<
         crate::client::customize::internal::SendResult<
             crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            crate::operation::assume_role_with_saml::AssumeRoleWithSamlError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl AssumeRoleWithSAMLFluentBuilder {
-    /// Creates a new `AssumeRoleWithSAMLFluentBuilder`.
+impl AssumeRoleWithSamlFluentBuilder {
+    /// Creates a new `AssumeRoleWithSamlFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
         Self {
             handle,
@@ -90,7 +90,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the AssumeRoleWithSAML as a reference.
+    /// Access the AssumeRoleWithSaml as a reference.
     pub fn as_input(&self) -> &crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlInputBuilder {
         &self.inner
     }
@@ -107,7 +107,7 @@
     ) -> ::std::result::Result<
         crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            crate::operation::assume_role_with_saml::AssumeRoleWithSamlError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -115,12 +115,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::assume_role_with_saml::AssumeRoleWithSAML::operation_runtime_plugins(
+        let runtime_plugins = crate::operation::assume_role_with_saml::AssumeRoleWithSaml::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        crate::operation::assume_role_with_saml::AssumeRoleWithSAML::orchestrate(&runtime_plugins, input).await
+        crate::operation::assume_role_with_saml::AssumeRoleWithSaml::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -128,7 +128,7 @@
         self,
     ) -> crate::client::customize::CustomizableOperation<
         crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
-        crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+        crate::operation::assume_role_with_saml::AssumeRoleWithSamlError,
         Self,
     > {
         crate::client::customize::CustomizableOperation::new(self)
```

### `src/operation/assume_role_with_saml.rs`

```diff
--- reference/src/operation/assume_role_with_saml.rs
+++ generated/src/operation/assume_role_with_saml.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `AssumeRoleWithSAML`.
+/// Orchestration and serialization glue logic for `AssumeRoleWithSaml`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct AssumeRoleWithSAML;
-impl AssumeRoleWithSAML {
-    /// Creates a new `AssumeRoleWithSAML`
+pub struct AssumeRoleWithSaml;
+impl AssumeRoleWithSaml {
+    /// Creates a new `AssumeRoleWithSaml`
     pub fn new() -> Self {
         Self
     }
@@ -14,7 +14,7 @@
     ) -> ::std::result::Result<
         crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            crate::operation::assume_role_with_saml::AssumeRoleWithSamlError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -23,7 +23,7 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >| {
             err.map_service_error(|err| {
-                err.downcast::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>()
+                err.downcast::<crate::operation::assume_role_with_saml::AssumeRoleWithSamlError>()
                     .expect("correct error type")
             })
         };
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AssumeRoleWithSAML {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AssumeRoleWithSaml {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AssumeRoleWithSAML");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            AssumeRoleWithSAMLRequestSerializer,
+            AssumeRoleWithSamlRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleWithSAMLResponseDeserializer,
+            AssumeRoleWithSamlResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -104,6 +104,16 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoleWithSAML", "STS"));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -114,14 +124,14 @@
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
                     let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithSAML")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLTelemetryInputCaptureInterceptor))
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSamlTelemetryInputCaptureInterceptor))
 .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::builder().transient_errors({
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSamlEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSamlError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSamlError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSamlError>::builder().transient_errors({
                                             let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
+                                            transient_errors.push("InternalError");
                                             ::std::borrow::Cow::Owned(transient_errors)
                                             }).build());

@@ -130,12 +140,12 @@
 }

 #[derive(Debug)]
-struct AssumeRoleWithSAMLTelemetryInputCaptureInterceptor;
+struct AssumeRoleWithSamlTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSAMLTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSamlTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "AssumeRoleWithSAMLTelemetryInputCaptureInterceptor"
+        "AssumeRoleWithSamlTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -240,12 +250,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_input_input_input(&input)?,
+            crate::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -255,12 +264,12 @@
     }
 }
 #[derive(Debug)]
-struct AssumeRoleWithSAMLEndpointParamsInterceptor;
+struct AssumeRoleWithSamlEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSAMLEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithSamlEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "AssumeRoleWithSAMLEndpointParamsInterceptor"
+        "AssumeRoleWithSamlEndpointParamsInterceptor"
     }

     fn read_before_execution(
@@ -280,8 +289,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -296,10 +305,10 @@
 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.

-/// Error type for the `AssumeRoleWithSAMLError` operation.
+/// Error type for the `AssumeRoleWithSamlError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
-pub enum AssumeRoleWithSAMLError {
+pub enum AssumeRoleWithSamlError {
     /// <p>The web identity token that was passed is expired or is not valid. Get a new identity token from the identity provider and then retry the request.</p>
     ExpiredTokenException(crate::types::error::ExpiredTokenException),
     /// <p>The identity provider (IdP) reported that authentication failed. This might be because the claim is invalid.</p>
@@ -320,11 +329,11 @@
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithSAMLError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithSamlError) for what information is available for the error.")]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
-impl AssumeRoleWithSAMLError {
-    /// Creates the `AssumeRoleWithSAMLError::Unhandled` variant from any error type.
+impl AssumeRoleWithSamlError {
+    /// Creates the `AssumeRoleWithSamlError::Unhandled` variant from any error type.
     pub fn unhandled(
         err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
     ) -> Self {
@@ -334,7 +343,7 @@
         })
     }

-    /// Creates the `AssumeRoleWithSAMLError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
+    /// Creates the `AssumeRoleWithSamlError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
     pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.clone().into(),
@@ -356,32 +365,32 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `AssumeRoleWithSAMLError::ExpiredTokenException`.
+    /// Returns `true` if the error kind is `AssumeRoleWithSamlError::ExpiredTokenException`.
     pub fn is_expired_token_exception(&self) -> bool {
         matches!(self, Self::ExpiredTokenException(_))
     }
-    /// Returns `true` if the error kind is `AssumeRoleWithSAMLError::IdpRejectedClaimException`.
+    /// Returns `true` if the error kind is `AssumeRoleWithSamlError::IdpRejectedClaimException`.
     pub fn is_idp_rejected_claim_exception(&self) -> bool {
         matches!(self, Self::IdpRejectedClaimException(_))
     }
-    /// Returns `true` if the error kind is `AssumeRoleWithSAMLError::InvalidIdentityTokenException`.
+    /// Returns `true` if the error kind is `AssumeRoleWithSamlError::InvalidIdentityTokenException`.
     pub fn is_invalid_identity_token_exception(&self) -> bool {
         matches!(self, Self::InvalidIdentityTokenException(_))
     }
-    /// Returns `true` if the error kind is `AssumeRoleWithSAMLError::MalformedPolicyDocumentException`.
+    /// Returns `true` if the error kind is `AssumeRoleWithSamlError::MalformedPolicyDocumentException`.
     pub fn is_malformed_policy_document_exception(&self) -> bool {
         matches!(self, Self::MalformedPolicyDocumentException(_))
     }
-    /// Returns `true` if the error kind is `AssumeRoleWithSAMLError::PackedPolicyTooLargeException`.
+    /// Returns `true` if the error kind is `AssumeRoleWithSamlError::PackedPolicyTooLargeException`.
     pub fn is_packed_policy_too_large_exception(&self) -> bool {
         matches!(self, Self::PackedPolicyTooLargeException(_))
     }
-    /// Returns `true` if the error kind is `AssumeRoleWithSAMLError::RegionDisabledException`.
+    /// Returns `true` if the error kind is `AssumeRoleWithSamlError::RegionDisabledException`.
     pub fn is_region_disabled_exception(&self) -> bool {
         matches!(self, Self::RegionDisabledException(_))
     }
 }
-impl ::std::error::Error for AssumeRoleWithSAMLError {
+impl ::std::error::Error for AssumeRoleWithSamlError {
     fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
             Self::ExpiredTokenException(_inner) => ::std::option::Option::Some(_inner),
@@ -394,7 +403,7 @@
         }
     }
 }
-impl ::std::fmt::Display for AssumeRoleWithSAMLError {
+impl ::std::fmt::Display for AssumeRoleWithSamlError {
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         match self {
             Self::ExpiredTokenException(_inner) => _inner.fmt(f),
@@ -413,7 +422,7 @@
         }
     }
 }
-impl ::aws_smithy_types::retry::ProvideErrorKind for AssumeRoleWithSAMLError {
+impl ::aws_smithy_types::retry::ProvideErrorKind for AssumeRoleWithSamlError {
     fn code(&self) -> ::std::option::Option<&str> {
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
@@ -421,7 +430,7 @@
         ::std::option::Option::None
     }
 }
-impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithSAMLError {
+impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithSamlError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
@@ -434,7 +443,7 @@
         }
     }
 }
-impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for AssumeRoleWithSAMLError {
+impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for AssumeRoleWithSamlError {
     fn create_unhandled_error(
         source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
@@ -445,7 +454,12 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError {
+impl crate::s3_request_id::RequestIdExt for crate::operation::assume_role_with_saml::AssumeRoleWithSamlError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
+impl ::aws_types::request_id::RequestId for crate::operation::assume_role_with_saml::AssumeRoleWithSamlError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/assume_role_with_web_identity.rs`

```diff
--- reference/src/operation/assume_role_with_web_identity.rs
+++ generated/src/operation/assume_role_with_web_identity.rs
@@ -107,7 +107,17 @@
             "AssumeRoleWithWebIdentity",
             "STS",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
+        signing_options.payload_override = None;

+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });
+
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -138,7 +148,7 @@
                 >::builder()
                 .transient_errors({
                     let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
+                    transient_errors.push("InternalError");
                     ::std::borrow::Cow::Owned(transient_errors)
                 })
                 .build(),
@@ -264,12 +274,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_assume_role_with_web_identity_input::ser_assume_role_with_web_identity_input_input_input(&input)?,
+            crate::protocol_serde::shape_assume_role_with_web_identity_input::ser_assume_role_with_web_identity_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -304,8 +313,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -451,10 +460,7 @@
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
     fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
-        match self {
-            Self::IdpCommunicationErrorException(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
-            _ => ::std::option::Option::None,
-        }
+        ::std::option::Option::None
     }
 }
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithWebIdentityError {
@@ -482,6 +488,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/assume_root.rs`

```diff
--- reference/src/operation/assume_root.rs
+++ generated/src/operation/assume_root.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoot", "STS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -143,7 +143,7 @@
                 ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::assume_root::AssumeRootError>::builder()
                     .transient_errors({
                         let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
+                        transient_errors.push("InternalError");
                         ::std::borrow::Cow::Owned(transient_errors)
                     })
                     .build(),
@@ -252,12 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_root_input::ser_assume_root_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_root_input::ser_assume_root_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -291,8 +289,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -414,6 +412,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::assume_root::AssumeRootError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::assume_root::AssumeRootError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/decode_authorization_message.rs`

```diff
--- reference/src/operation/decode_authorization_message.rs
+++ generated/src/operation/decode_authorization_message.rs
@@ -107,9 +107,9 @@
             "STS",
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
@@ -147,7 +147,7 @@
                 >::builder()
                 .transient_errors({
                     let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
+                    transient_errors.push("InternalError");
                     ::std::borrow::Cow::Owned(transient_errors)
                 })
                 .build(),
@@ -258,12 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_decode_authorization_message_input::ser_decode_authorization_message_input_input_input(&input)?,
+            crate::protocol_serde::shape_decode_authorization_message_input::ser_decode_authorization_message_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -298,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -411,6 +410,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::decode_authorization_message::DecodeAuthorizationMessageError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::decode_authorization_message::DecodeAuthorizationMessageError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_access_key_info.rs`

```diff
--- reference/src/operation/get_access_key_info.rs
+++ generated/src/operation/get_access_key_info.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetAccessKeyInfo", "STS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -143,7 +143,7 @@
                 )
                 .transient_errors({
                     let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
+                    transient_errors.push("InternalError");
                     ::std::borrow::Cow::Owned(transient_errors)
                 })
                 .build(),
@@ -254,13 +254,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_access_key_info_input::ser_get_access_key_info_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_access_key_info_input::ser_get_access_key_info_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +293,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -397,6 +396,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_access_key_info::GetAccessKeyInfoError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_access_key_info::GetAccessKeyInfoError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_caller_identity.rs`

```diff
--- reference/src/operation/get_caller_identity.rs
+++ generated/src/operation/get_caller_identity.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetCallerIdentity", "STS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -123,13 +123,14 @@
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
                     let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCallerIdentity")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCallerIdentityTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
 .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCallerIdentityEndpointParamsInterceptor))
                             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::new())
 .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::new())
 .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::builder().transient_errors({
                                             let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
+                                            transient_errors.push("InternalError");
                                             ::std::borrow::Cow::Owned(transient_errors)
                                             }).build());

@@ -138,6 +139,44 @@
 }

 #[derive(Debug)]
+struct GetCallerIdentityTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetCallerIdentityTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "GetCallerIdentityTelemetryInputCaptureInterceptor"
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
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetCallerIdentityInput>() else {
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
 struct GetCallerIdentityResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetCallerIdentityResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -195,13 +234,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_caller_identity_input::ser_get_caller_identity_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -232,8 +267,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -335,6 +370,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_caller_identity::GetCallerIdentityError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_caller_identity::GetCallerIdentityError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_delegated_access_token.rs`

```diff
--- reference/src/operation/get_delegated_access_token.rs
+++ generated/src/operation/get_delegated_access_token.rs
@@ -108,9 +108,9 @@
             "STS",
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
@@ -128,6 +128,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetDelegatedAccessToken")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetDelegatedAccessTokenTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -145,7 +148,7 @@
                 >::builder()
                 .transient_errors({
                     let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
+                    transient_errors.push("InternalError");
                     ::std::borrow::Cow::Owned(transient_errors)
                 })
                 .build(),
@@ -156,6 +159,44 @@
 }

 #[derive(Debug)]
+struct GetDelegatedAccessTokenTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetDelegatedAccessTokenTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "GetDelegatedAccessTokenTelemetryInputCaptureInterceptor"
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
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetDelegatedAccessTokenInput>() else {
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
 struct GetDelegatedAccessTokenResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetDelegatedAccessTokenResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -213,12 +254,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_delegated_access_token_input::ser_get_delegated_access_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_delegated_access_token_input::ser_get_delegated_access_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -253,8 +293,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -387,6 +427,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_federation_token.rs`

```diff
--- reference/src/operation/get_federation_token.rs
+++ generated/src/operation/get_federation_token.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetFederationToken", "STS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -131,7 +131,7 @@
 .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::new())
 .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::builder().transient_errors({
                                             let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
+                                            transient_errors.push("InternalError");
                                             ::std::borrow::Cow::Owned(transient_errors)
                                             }).build());

@@ -245,12 +245,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -285,8 +284,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -419,6 +418,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_federation_token::GetFederationTokenError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_federation_token::GetFederationTokenError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_session_token.rs`

```diff
--- reference/src/operation/get_session_token.rs
+++ generated/src/operation/get_session_token.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetSessionToken", "STS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -143,7 +143,7 @@
                 ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_session_token::GetSessionTokenError>::builder()
                     .transient_errors({
                         let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
+                        transient_errors.push("InternalError");
                         ::std::borrow::Cow::Owned(transient_errors)
                     })
                     .build(),
@@ -259,13 +259,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_session_token_input::ser_get_session_token_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_session_token_input::ser_get_session_token_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -412,6 +411,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_session_token::GetSessionTokenError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_session_token::GetSessionTokenError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_web_identity_token.rs`

```diff
--- reference/src/operation/get_web_identity_token.rs
+++ generated/src/operation/get_web_identity_token.rs
@@ -108,9 +108,9 @@
             "STS",
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
@@ -134,7 +134,7 @@
 .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
 .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::builder().transient_errors({
                                             let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
+                                            transient_errors.push("InternalError");
                                             ::std::borrow::Cow::Owned(transient_errors)
                                             }).build());

@@ -243,12 +243,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -283,8 +282,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -416,6 +415,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_web_identity_token::GetWebIdentityTokenError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_web_identity_token::GetWebIdentityTokenError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation.rs`

```diff
--- reference/src/operation.rs
+++ generated/src/operation.rs
@@ -1,35 +1,39 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+
 pub use ::aws_types::request_id::RequestId;

-/// Types for the `AssumeRole` operation.
-pub mod assume_role;
-
-/// Types for the `AssumeRoleWithSAML` operation.
-pub mod assume_role_with_saml;
-
-/// Types for the `AssumeRoleWithWebIdentity` operation.
-pub mod assume_role_with_web_identity;
-
-/// Types for the `AssumeRoot` operation.
-pub mod assume_root;
-
-/// Types for the `DecodeAuthorizationMessage` operation.
-pub mod decode_authorization_message;
-
-/// Types for the `GetAccessKeyInfo` operation.
-pub mod get_access_key_info;
-
-/// Types for the `GetCallerIdentity` operation.
-pub mod get_caller_identity;
-
-/// Types for the `GetDelegatedAccessToken` operation.
-pub mod get_delegated_access_token;
-
-/// Types for the `GetFederationToken` operation.
-pub mod get_federation_token;
-
-/// Types for the `GetSessionToken` operation.
-pub mod get_session_token;
-
-/// Types for the `GetWebIdentityToken` operation.
-pub mod get_web_identity_token;
+pub mod operation {
+    pub mod assume_role {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/assume_role.rs"));
+    }
+    pub mod assume_role_with_saml {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/assume_role_with_saml.rs"));
+    }
+    pub mod assume_role_with_web_identity {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/assume_role_with_web_identity.rs"));
+    }
+    pub mod assume_root {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/assume_root.rs"));
+    }
+    pub mod decode_authorization_message {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/decode_authorization_message.rs"));
+    }
+    pub mod get_access_key_info {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/get_access_key_info.rs"));
+    }
+    pub mod get_caller_identity {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/get_caller_identity.rs"));
+    }
+    pub mod get_delegated_access_token {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/get_delegated_access_token.rs"));
+    }
+    pub mod get_federation_token {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/get_federation_token.rs"));
+    }
+    pub mod get_session_token {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/get_session_token.rs"));
+    }
+    pub mod get_web_identity_token {
+        include!(concat!(env!("OUT_DIR"), "/generated/sts/src/operation/get_web_identity_token.rs"));
+    }
+}
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -1,16 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn assumed_role_user_correct_errors(
-    mut builder: crate::types::builders::AssumedRoleUserBuilder,
-) -> crate::types::builders::AssumedRoleUserBuilder {
-    if builder.assumed_role_id.is_none() {
-        builder.assumed_role_id = Some(Default::default())
-    }
-    if builder.arn.is_none() {
-        builder.arn = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn credentials_correct_errors(mut builder: crate::types::builders::CredentialsBuilder) -> crate::types::builders::CredentialsBuilder {
     if builder.access_key_id.is_none() {
         builder.access_key_id = Some(Default::default())
@@ -27,6 +15,18 @@
     builder
 }

+pub(crate) fn assumed_role_user_correct_errors(
+    mut builder: crate::types::builders::AssumedRoleUserBuilder,
+) -> crate::types::builders::AssumedRoleUserBuilder {
+    if builder.assumed_role_id.is_none() {
+        builder.assumed_role_id = Some(Default::default())
+    }
+    if builder.arn.is_none() {
+        builder.arn = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn federated_user_correct_errors(
     mut builder: crate::types::builders::FederatedUserBuilder,
 ) -> crate::types::builders::FederatedUserBuilder {
```

### `src/types/_credentials.rs`

```diff
--- reference/src/types/_credentials.rs
+++ generated/src/types/_credentials.rs
@@ -37,10 +37,10 @@
 impl ::std::fmt::Debug for Credentials {
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("Credentials");
-        formatter.field("access_key_id", &"*** Sensitive Data Redacted ***");
+        formatter.field("access_key_id", &self.access_key_id);
         formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
-        formatter.field("session_token", &"*** Sensitive Data Redacted ***");
-        formatter.field("expiration", &"*** Sensitive Data Redacted ***");
+        formatter.field("session_token", &self.session_token);
+        formatter.field("expiration", &self.expiration);
         formatter.finish()
     }
 }
@@ -159,10 +159,10 @@
 impl ::std::fmt::Debug for CredentialsBuilder {
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("CredentialsBuilder");
-        formatter.field("access_key_id", &"*** Sensitive Data Redacted ***");
+        formatter.field("access_key_id", &self.access_key_id);
         formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
-        formatter.field("session_token", &"*** Sensitive Data Redacted ***");
-        formatter.field("expiration", &"*** Sensitive Data Redacted ***");
+        formatter.field("session_token", &self.session_token);
+        formatter.field("expiration", &self.expiration);
         formatter.finish()
     }
 }
```

### `src/types/error/_idp_communication_error_exception.rs`

```diff
--- reference/src/types/error/_idp_communication_error_exception.rs
+++ generated/src/types/error/_idp_communication_error_exception.rs
@@ -9,10 +9,6 @@
     pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
 }
 impl IdpCommunicationErrorException {
-    /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
-    pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
-        ::aws_smithy_types::retry::ErrorKind::ServerError
-    }
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
         self.message.as_deref()
@@ -20,7 +16,7 @@
 }
 impl ::std::fmt::Display for IdpCommunicationErrorException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "IdpCommunicationErrorException [IDPCommunicationErrorException]")?;
+        ::std::write!(f, "IdpCommunicationErrorException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_idp_rejected_claim_exception.rs`

```diff
--- reference/src/types/error/_idp_rejected_claim_exception.rs
+++ generated/src/types/error/_idp_rejected_claim_exception.rs
@@ -17,7 +17,7 @@
 }
 impl ::std::fmt::Display for IdpRejectedClaimException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "IdpRejectedClaimException [IDPRejectedClaimException]")?;
+        ::std::write!(f, "IdpRejectedClaimException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_jwt_payload_size_exceeded_exception.rs`

```diff
--- reference/src/types/error/_jwt_payload_size_exceeded_exception.rs
+++ generated/src/types/error/_jwt_payload_size_exceeded_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for JwtPayloadSizeExceededException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "JwtPayloadSizeExceededException [JWTPayloadSizeExceededException]")?;
+        ::std::write!(f, "JwtPayloadSizeExceededException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### Missing reference files

- `Cargo.toml`
- `LICENSE`
- `README.md`
- `src/client/customize/internal.rs`
- `src/client/customize.rs`
- `src/config/auth.rs`
- `src/config/endpoint.rs`
- `src/config/http.rs`
- `src/config/interceptors.rs`
- `src/config/retry.rs`
- `src/config/timeout.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/diagnostic.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/partition.rs`
- `src/endpoint_lib.rs`
- `src/error/sealed_unhandled.rs`
- `src/error_meta.rs`
- `src/protocol_serde/shape_assume_role.rs`
- `src/protocol_serde/shape_assume_role_input.rs`
- `src/protocol_serde/shape_assume_role_with_saml.rs`
- `src/protocol_serde/shape_assume_role_with_saml_input.rs`
- `src/protocol_serde/shape_assume_role_with_web_identity.rs`
- `src/protocol_serde/shape_assume_role_with_web_identity_input.rs`
- `src/protocol_serde/shape_assume_root.rs`
- `src/protocol_serde/shape_assume_root_input.rs`
- `src/protocol_serde/shape_assumed_role_user.rs`
- `src/protocol_serde/shape_credentials.rs`
- `src/protocol_serde/shape_decode_authorization_message.rs`
- `src/protocol_serde/shape_decode_authorization_message_input.rs`
- `src/protocol_serde/shape_expired_token_exception.rs`
- `src/protocol_serde/shape_expired_trade_in_token_exception.rs`
- `src/protocol_serde/shape_federated_user.rs`
- `src/protocol_serde/shape_get_access_key_info.rs`
- `src/protocol_serde/shape_get_access_key_info_input.rs`
- `src/protocol_serde/shape_get_caller_identity.rs`
- `src/protocol_serde/shape_get_caller_identity_input.rs`
- `src/protocol_serde/shape_get_delegated_access_token.rs`
- `src/protocol_serde/shape_get_delegated_access_token_input.rs`
- `src/protocol_serde/shape_get_federation_token.rs`
- `src/protocol_serde/shape_get_federation_token_input.rs`
- `src/protocol_serde/shape_get_session_token.rs`
- `src/protocol_serde/shape_get_session_token_input.rs`
- `src/protocol_serde/shape_get_web_identity_token.rs`
- `src/protocol_serde/shape_get_web_identity_token_input.rs`
- `src/protocol_serde/shape_idp_communication_error_exception.rs`
- `src/protocol_serde/shape_idp_rejected_claim_exception.rs`
- `src/protocol_serde/shape_invalid_authorization_message_exception.rs`
- `src/protocol_serde/shape_invalid_identity_token_exception.rs`
- `src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs`
- `src/protocol_serde/shape_malformed_policy_document_exception.rs`
- `src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs`
- `src/protocol_serde/shape_packed_policy_too_large_exception.rs`
- `src/protocol_serde/shape_policy_descriptor_type.rs`
- `src/protocol_serde/shape_provided_context.rs`
- `src/protocol_serde/shape_region_disabled_exception.rs`
- `src/protocol_serde/shape_session_duration_escalation_exception.rs`
- `src/protocol_serde/shape_tag.rs`
- `src/protocol_serde.rs`
- `src/rest_xml_wrapped_errors.rs`
- `src/sdk_feature_tracker.rs`
- `src/serialization_settings.rs`
- `tests/endpoint_tests.rs`
- `tests/retry_idp_comms_err.rs`
- `tests/signing-it.rs`

### Rust token differences

- `src/client/assume_role.rs`
- `src/client/assume_role_with_web_identity.rs`
- `src/client.rs`
- `src/config.rs`
- `src/error.rs`
- `src/lib.rs`
- `src/operation/assume_role.rs`
- `src/operation/assume_role_with_saml/builders.rs`
- `src/operation/assume_role_with_saml.rs`
- `src/operation/assume_role_with_web_identity.rs`
- `src/operation/assume_root.rs`
- `src/operation/decode_authorization_message.rs`
- `src/operation/get_access_key_info.rs`
- `src/operation/get_caller_identity.rs`
- `src/operation/get_delegated_access_token.rs`
- `src/operation/get_federation_token.rs`
- `src/operation/get_session_token.rs`
- `src/operation/get_web_identity_token.rs`
- `src/operation.rs`
- `src/serde_util.rs`
- `src/types/_credentials.rs`
- `src/types/error/_idp_communication_error_exception.rs`
- `src/types/error/_idp_rejected_claim_exception.rs`
- `src/types/error/_jwt_payload_size_exceeded_exception.rs`
