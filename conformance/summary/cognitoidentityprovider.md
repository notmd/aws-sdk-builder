# AWS SDK Conformance Report: cognitoidentityprovider

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## cognitoidentityprovider
**Progress:** `1361/1361` files compared · `1217` matched · `144` mismatches · `0` missing · `0` extra · `89.42%` match (100.00% means fully matched)

### `src/client/add_custom_attributes.rs`

```diff
--- reference/src/client/add_custom_attributes.rs
+++ generated/src/client/add_custom_attributes.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool where you want to add custom attributes.</p><br>
-    ///   - [`custom_attributes(SchemaAttributeType)`](crate::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder::custom_attributes) / [`set_custom_attributes(Option<Vec::<SchemaAttributeType>>)`](crate::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder::set_custom_attributes):<br>required: **true**<br><p>An array of custom attribute names and other properties. Sets the following characteristics:</p> <dl>  <dt>   AttributeDataType  </dt>  <dd>   <p>The expected data type. Can be a string, a number, a date and time, or a boolean.</p>  </dd>  <dt>   Mutable  </dt>  <dd>   <p>If true, you can grant app clients write access to the attribute value. If false, the attribute value can only be set up on sign-up or administrator creation of users.</p>  </dd>  <dt>   Name  </dt>  <dd>   <p>The attribute name. For an attribute like <code>custom:myAttribute</code>, enter <code>myAttribute</code> for this field.</p>  </dd>  <dt>   Required  </dt>  <dd>   <p>When true, users who sign up or are created must set a value for the attribute.</p>  </dd>  <dt>   NumberAttributeConstraints  </dt>  <dd>   <p>The minimum and maximum length of accepted values for a <code>Number</code>-type attribute.</p>  </dd>  <dt>   StringAttributeConstraints  </dt>  <dd>   <p>The minimum and maximum length of accepted values for a <code>String</code>-type attribute.</p>  </dd>  <dt>   DeveloperOnlyAttribute  </dt>  <dd>   <p>This legacy option creates an attribute with a <code>dev:</code> prefix. You can only set the value of a developer-only attribute with administrative IAM credentials.</p>  </dd> </dl><br>
+    ///   - [`custom_attributes(SchemaAttributeType)`](crate::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder::custom_attributes) / [`set_custom_attributes(Option<Vec::<SchemaAttributeType>>)`](crate::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder::set_custom_attributes):<br>required: **true**<br><p>An array of custom attribute names and other properties. Sets the following characteristics:</p> <dl> <dt>AttributeDataType</dt> <dd> <p>The expected data type. Can be a string, a number, a date and time, or a boolean.</p></dd> <dt>Mutable</dt> <dd> <p>If true, you can grant app clients write access to the attribute value. If false, the attribute value can only be set up on sign-up or administrator creation of users.</p></dd> <dt>Name</dt> <dd> <p>The attribute name. For an attribute like <code>custom:myAttribute</code>, enter <code>myAttribute</code> for this field.</p></dd> <dt>Required</dt> <dd> <p>When true, users who sign up or are created must set a value for the attribute.</p></dd> <dt>NumberAttributeConstraints</dt> <dd> <p>The minimum and maximum length of accepted values for a <code>Number</code>-type attribute.</p></dd> <dt>StringAttributeConstraints</dt> <dd> <p>The minimum and maximum length of accepted values for a <code>String</code>-type attribute.</p></dd> <dt>DeveloperOnlyAttribute</dt> <dd> <p>This legacy option creates an attribute with a <code>dev:</code> prefix. You can only set the value of a developer-only attribute with administrative IAM credentials.</p></dd></dl><br>
     /// - On success, responds with [`AddCustomAttributesOutput`](crate::operation::add_custom_attributes::AddCustomAttributesOutput)
     /// - On failure, responds with [`SdkError<AddCustomAttributesError>`](crate::operation::add_custom_attributes::AddCustomAttributesError)
     pub fn add_custom_attributes(&self) -> super::super::operation::add_custom_attributes::builders::AddCustomAttributesFluentBuilder {
```

### `src/client/admin_create_user.rs`

```diff
--- reference/src/client/admin_create_user.rs
+++ generated/src/client/admin_create_user.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool where you want to create a user.</p><br>
     ///   - [`username(impl Into<String>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::username) / [`set_username(Option<String>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_username):<br>required: **true**<br><p>The value that you want to set as the username sign-in attribute. The following conditions apply to the username parameter.</p> <ul>  <li>   <p>The username can't be a duplicate of another username in the same user pool.</p></li>  <li>   <p>You can't change the value of a username after you create it.</p></li>  <li>   <p>You can only provide a value if usernames are a valid sign-in attribute for your user pool. If your user pool only supports phone numbers or email addresses as sign-in attributes, Amazon Cognito automatically generates a username value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#user-pool-settings-aliases">Customizing sign-in attributes</a>.</p></li> </ul><br>
-    ///   - [`user_attributes(AttributeType)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::user_attributes) / [`set_user_attributes(Option<Vec::<AttributeType>>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_user_attributes):<br>required: **false**<br><p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p> <p>You must also provide an email address or phone number when you expect the user to do passwordless sign-in with an email or SMS OTP. These attributes must be provided when passwordless options are the only available, or when you don't submit a <code>TemporaryPassword</code>.</p> <p>In your <code>AdminCreateUser</code> request, you can set the <code>email_verified</code> and <code>phone_number_verified</code> attributes to <code>true</code>. The following conditions apply:</p> <dl>  <dt>   email  </dt>  <dd>   <p>The email address where you want the user to receive their confirmation code and username. You must provide a value for <code>email</code> when you want to set <code>email_verified</code> to <code>true</code>, or if you set <code>EMAIL</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>  </dd>  <dt>   phone_number  </dt>  <dd>   <p>The phone number where you want the user to receive their confirmation code and username. You must provide a value for <code>phone_number</code> when you want to set <code>phone_number_verified</code> to <code>true</code>, or if you set <code>SMS</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>  </dd> </dl><br>
+    ///   - [`user_attributes(AttributeType)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::user_attributes) / [`set_user_attributes(Option<Vec::<AttributeType>>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_user_attributes):<br>required: **false**<br><p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p> <p>You must also provide an email address or phone number when you expect the user to do passwordless sign-in with an email or SMS OTP. These attributes must be provided when passwordless options are the only available, or when you don't submit a <code>TemporaryPassword</code>.</p> <p>In your <code>AdminCreateUser</code> request, you can set the <code>email_verified</code> and <code>phone_number_verified</code> attributes to <code>true</code>. The following conditions apply:</p> <dl> <dt>email</dt> <dd> <p>The email address where you want the user to receive their confirmation code and username. You must provide a value for <code>email</code> when you want to set <code>email_verified</code> to <code>true</code>, or if you set <code>EMAIL</code> in the <code>DesiredDeliveryMediums</code> parameter.</p></dd> <dt>phone_number</dt> <dd> <p>The phone number where you want the user to receive their confirmation code and username. You must provide a value for <code>phone_number</code> when you want to set <code>phone_number_verified</code> to <code>true</code>, or if you set <code>SMS</code> in the <code>DesiredDeliveryMediums</code> parameter.</p></dd></dl><br>
     ///   - [`validation_data(AttributeType)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::validation_data) / [`set_validation_data(Option<Vec::<AttributeType>>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_validation_data):<br>required: **false**<br><p>Temporary user attributes that contribute to the outcomes of your pre sign-up Lambda trigger. This set of key-value pairs are for custom validation of information that you collect from your users but don't need to retain.</p> <p>Your Lambda function can analyze this additional data and act on it. Your function can automatically confirm and verify select users or perform external API operations like logging user attributes and validation data to Amazon CloudWatch Logs.</p> <p>For more information about the pre sign-up Lambda trigger, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-pre-sign-up.html">Pre sign-up Lambda trigger</a>.</p><br>
     ///   - [`temporary_password(impl Into<String>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::temporary_password) / [`set_temporary_password(Option<String>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_temporary_password):<br>required: **false**<br><p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p> <p>The exception to the requirement for a password is when your user pool supports passwordless sign-in with email or SMS OTPs. To create a user with no password, omit this parameter or submit a blank value. You can only create a passwordless user when passwordless sign-in is available.</p> <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p> <p>If you don't specify a value, Amazon Cognito generates one for you unless you have passwordless options active for your user pool.</p> <p>The temporary password can only be used until the user account expiration limit that you set for your user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again and specify <code>RESEND</code> for the <code>MessageAction</code> parameter.</p><br>
     ///   - [`force_alias_creation(bool)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::force_alias_creation) / [`set_force_alias_creation(Option<bool>)`](crate::operation::admin_create_user::builders::AdminCreateUserFluentBuilder::set_force_alias_creation):<br>required: **false**<br><p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p> <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the <code>UserAttributes</code> parameter already exists as an alias with a different user, this request migrates the alias from the previous user to the newly-created user. The previous user will no longer be able to log in using that alias.</p> <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p><br>
```

### `src/client/admin_initiate_auth.rs`

```diff
--- reference/src/client/admin_initiate_auth.rs
+++ generated/src/client/admin_initiate_auth.rs
@@ -5,9 +5,9 @@
     /// - The fluent builder is configurable:
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool where the user wants to sign in.</p><br>
     ///   - [`client_id(impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::client_id) / [`set_client_id(Option<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_client_id):<br>required: **true**<br><p>The ID of the app client where the user wants to sign in.</p><br>
-    ///   - [`auth_flow(AuthFlowType)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_flow) / [`set_auth_flow(Option<AuthFlowType>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_flow):<br>required: **true**<br><p>The authentication flow that you want to initiate. Each <code>AuthFlow</code> has linked <code>AuthParameters</code> that you must submit. The following are some example flows.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <p>The entry point for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a> with passwords, one-time passwords, and WebAuthn authenticators. Request a preferred authentication type or review available authentication types. From the offered authentication types, select one in a challenge response and then authenticate with that method in an additional challenge response. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <p>Username-password authentication with the Secure Remote Password (SRP) protocol. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow.html#Using-SRP-password-verification-in-custom-authentication-flow">Use SRP password verification in custom authentication flow</a>.</p>  </dd>  <dt>   REFRESH_TOKEN_AUTH and REFRESH_TOKEN  </dt>  <dd>   <p>Receive new ID and access tokens when you pass a <code>REFRESH_TOKEN</code> parameter with a valid refresh token as the value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html">Using the refresh token</a>.</p>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <p>Custom authentication with Lambda triggers. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-challenge.html">Custom authentication challenge Lambda triggers</a>.</p>  </dd>  <dt>   ADMIN_USER_PASSWORD_AUTH  </dt>  <dd>   <p>Server-side username-password authentication with the password sent directly in the request. For more information about client-side and server-side authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-public-server-side.html">SDK authorization models</a>.</p>  </dd> </dl><br>
-    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li>   </ul>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>SRP_A</code> (required)</p></li>   </ul>  </dd>  <dt>   ADMIN_USER_PASSWORD_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PASSWORD</code> (required)</p></li>   </ul>  </dd>  <dt>   REFRESH_TOKEN_AUTH/REFRESH_TOKEN  </dt>  <dd>   <ul>    <li>     <p><code>REFRESH_TOKEN</code>(required)</p></li>   </ul>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>ChallengeName: SRP_A</code> (when preceding custom authentication with SRP authentication)</p></li>    <li>     <p><code>SRP_A: (An SRP_A value)</code> (when preceding custom authentication with SRP authentication)</p></li>   </ul>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
-    ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The <code>ClientMetadata</code> value is passed as input to the functions for only the following triggers:</p> <ul>  <li>   <p>Pre signup</p></li>  <li>   <p>Pre authentication</p></li>  <li>   <p>User migration</p></li> </ul> <p>This request also invokes the functions for the following triggers, but doesn't pass <code>ClientMetadata</code>:</p> <ul>  <li>   <p>Post authentication</p></li>  <li>   <p>Custom message</p></li>  <li>   <p>Pre token generation</p></li>  <li>   <p>Create auth challenge</p></li>  <li>   <p>Define auth challenge</p></li>  <li>   <p>Custom email sender</p></li>  <li>   <p>Custom SMS sender</p></li> </ul><note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
+    ///   - [`auth_flow(AuthFlowType)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_flow) / [`set_auth_flow(Option<AuthFlowType>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_flow):<br>required: **true**<br><p>The authentication flow that you want to initiate. Each <code>AuthFlow</code> has linked <code>AuthParameters</code> that you must submit. The following are some example flows.</p> <dl> <dt>USER_AUTH</dt> <dd> <p>The entry point for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a> with passwords, one-time passwords, and WebAuthn authenticators. Request a preferred authentication type or review available authentication types. From the offered authentication types, select one in a challenge response and then authenticate with that method in an additional challenge response. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p></dd> <dt>USER_SRP_AUTH</dt> <dd> <p>Username-password authentication with the Secure Remote Password (SRP) protocol. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow.html#Using-SRP-password-verification-in-custom-authentication-flow">Use SRP password verification in custom authentication flow</a>.</p></dd> <dt>REFRESH_TOKEN_AUTH and REFRESH_TOKEN</dt> <dd> <p>Receive new ID and access tokens when you pass a <code>REFRESH_TOKEN</code> parameter with a valid refresh token as the value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html">Using the refresh token</a>.</p></dd> <dt>CUSTOM_AUTH</dt> <dd> <p>Custom authentication with Lambda triggers. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-challenge.html">Custom authentication challenge Lambda triggers</a>.</p></dd> <dt>ADMIN_USER_PASSWORD_AUTH</dt> <dd> <p>Server-side username-password authentication with the password sent directly in the request. For more information about client-side and server-side authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-public-server-side.html">SDK authorization models</a>.</p></dd></dl><br>
+    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl> <dt>USER_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li>   </ul></dd> <dt>USER_SRP_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>SRP_A</code> (required)</p></li>   </ul></dd> <dt>ADMIN_USER_PASSWORD_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PASSWORD</code> (required)</p></li>   </ul></dd> <dt>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</dt> <dd> <ul>    <li>     <p><code>REFRESH_TOKEN</code>(required)</p></li>   </ul></dd> <dt>CUSTOM_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>ChallengeName: SRP_A</code> (when preceding custom authentication with SRP authentication)</p></li>    <li>     <p><code>SRP_A: (An SRP_A value)</code> (when preceding custom authentication with SRP authentication)</p></li>   </ul></dd></dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
+    ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The <code>ClientMetadata</code> value is passed as input to the functions for only the following triggers:</p> <ul>  <li>   <p>Pre signup</p></li>  <li>   <p>Pre authentication</p></li>  <li>   <p>User migration</p></li> </ul> <p>This request also invokes the functions for the following triggers, but doesn't pass <code>ClientMetadata</code>:</p> <ul>  <li>   <p>Post authentication</p></li>  <li>   <p>Custom message</p></li>  <li>   <p>Pre token generation</p></li>  <li>   <p>Create auth challenge</p></li>  <li>   <p>Define auth challenge</p></li>  <li>   <p>Custom email sender</p></li>  <li>   <p>Custom SMS sender</p></li> </ul> <note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
     ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_analytics_metadata):<br>required: **false**<br><p>Information that supports analytics outcomes with Amazon Pinpoint, including the user's endpoint ID. The endpoint ID is a destination for Amazon Pinpoint push notifications, for example a device identifier, email address, or phone number.</p><br>
     ///   - [`context_data(ContextDataType)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::context_data) / [`set_context_data(Option<ContextDataType>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_context_data):<br>required: **false**<br><p>Contextual data about your user session like the device fingerprint, IP address, or location. Amazon Cognito threat protection evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-viewing-threat-protection-app.html">Collecting data for threat protection in applications</a>.</p><br>
     ///   - [`session(impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::session) / [`set_session(Option<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_session):<br>required: **false**<br><p>The optional session ID from a <code>ConfirmSignUp</code> API request. You can sign in a user directly from the sign-up process with an <code>AuthFlow</code> of <code>USER_AUTH</code> and <code>AuthParameters</code> of <code>EMAIL_OTP</code> or <code>SMS_OTP</code>, depending on how your user pool sent the confirmation-code message.</p><br>
```

### `src/client/admin_respond_to_auth_challenge.rs`

```diff
--- reference/src/client/admin_respond_to_auth_challenge.rs
+++ generated/src/client/admin_respond_to_auth_challenge.rs
@@ -6,7 +6,7 @@
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool where you want to respond to an authentication challenge.</p><br>
     ///   - [`client_id(impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::client_id) / [`set_client_id(Option<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_client_id):<br>required: **true**<br><p>The ID of the app client where you initiated sign-in.</p><br>
     ///   - [`challenge_name(ChallengeNameType)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::challenge_name) / [`set_challenge_name(Option<ChallengeNameType>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_challenge_name):<br>required: **true**<br><p>The name of the challenge that you are responding to.</p> <p>Possible challenges include the following:</p><note>  <p>All of the following challenges require <code>USERNAME</code> and, when the app client has a client secret, <code>SECRET_HASH</code> in the parameters. Include a <code>DEVICE_KEY</code> for device authentication.</p> </note> <ul>  <li>   <p><code>WEB_AUTHN</code>: Respond to the challenge with the results of a successful authentication with a WebAuthn authenticator, or passkey, as <code>CREDENTIAL</code>. Examples of WebAuthn authenticators include biometric devices and security keys.</p></li>  <li>   <p><code>PASSWORD</code>: Respond with the user's password as <code>PASSWORD</code>.</p></li>  <li>   <p><code>PASSWORD_SRP</code>: Respond with the initial SRP secret as <code>SRP_A</code>.</p></li>  <li>   <p><code>SELECT_CHALLENGE</code>: Respond with a challenge selection as <code>ANSWER</code>. It must be one of the challenge types in the <code>AvailableChallenges</code> response parameter. Add the parameters of the selected challenge, for example <code>USERNAME</code> and <code>SMS_OTP</code>.</p></li>  <li>   <p><code>SMS_MFA</code>: Respond with the code that your user pool delivered in an SMS message, as <code>SMS_MFA_CODE</code></p></li>  <li>   <p><code>EMAIL_MFA</code>: Respond with the code that your user pool delivered in an email message, as <code>EMAIL_MFA_CODE</code></p></li>  <li>   <p><code>EMAIL_OTP</code>: Respond with the code that your user pool delivered in an email message, as <code>EMAIL_OTP_CODE</code> .</p></li>  <li>   <p><code>SMS_OTP</code>: Respond with the code that your user pool delivered in an SMS message, as <code>SMS_OTP_CODE</code>.</p></li>  <li>   <p><code>PASSWORD_VERIFIER</code>: Respond with the second stage of SRP secrets as <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, and <code>TIMESTAMP</code>.</p></li>  <li>   <p><code>CUSTOM_CHALLENGE</code>: This is returned if your custom authentication flow determines that the user should pass another challenge before tokens are issued. The parameters of the challenge are determined by your Lambda function and issued in the <code>ChallengeParameters</code> of a challenge response.</p></li>  <li>   <p><code>DEVICE_SRP_AUTH</code>: Respond with the initial parameters of device SRP authentication. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html#user-pools-remembered-devices-signing-in-with-a-device">Signing in with a device</a>.</p></li>  <li>   <p><code>DEVICE_PASSWORD_VERIFIER</code>: Respond with <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, and <code>TIMESTAMP</code> after client-side SRP calculations. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html#user-pools-remembered-devices-signing-in-with-a-device">Signing in with a device</a>.</p></li>  <li>   <p><code>NEW_PASSWORD_REQUIRED</code>: For users who are required to change their passwords after successful first login. Respond to this challenge with <code>NEW_PASSWORD</code> and any required attributes that Amazon Cognito returned in the <code>requiredAttributes</code> parameter. You can also set values for attributes that aren't required by your user pool and that your app client can write.</p>   <p>Amazon Cognito only returns this challenge for users who have temporary passwords. When you create passwordless users, you must provide values for all required attributes.</p><note>    <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p>   </note></li>  <li>   <p><code>MFA_SETUP</code>: For users who are required to setup an MFA factor before they can sign in. The MFA types activated for the user pool will be listed in the challenge parameters <code>MFAS_CAN_SETUP</code> value.</p>   <p>To set up time-based one-time password (TOTP) MFA, use the session returned in this challenge from <code>InitiateAuth</code> or <code>AdminInitiateAuth</code> as an input to <code>AssociateSoftwareToken</code>. Then, use the session returned by <code>VerifySoftwareToken</code> as an input to <code>RespondToAuthChallenge</code> or <code>AdminRespondToAuthChallenge</code> with challenge name <code>MFA_SETUP</code> to complete sign-in.</p>   <p>To set up SMS or email MFA, collect a <code>phone_number</code> or <code>email</code> attribute for the user. Then restart the authentication flow with an <code>InitiateAuth</code> or <code>AdminInitiateAuth</code> request.</p></li> </ul><br>
-    ///   - [`challenge_responses(impl Into<String>, impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::challenge_responses) / [`set_challenge_responses(Option<HashMap::<String, String>>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_challenge_responses):<br>required: **false**<br><p>The responses to the challenge that you received in the previous request. Each challenge has its own required response parameters. The following examples are partial JSON request bodies that highlight challenge-response parameters.</p><important>  <p>You must provide a SECRET_HASH parameter in all challenge responses to an app client that has a client secret. Include a <code>DEVICE_KEY</code> for device authentication.</p> </important> <dl>  <dt>   SELECT_CHALLENGE  </dt>  <dd>   <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "USERNAME": "\[username\]", "ANSWER": "\[Challenge name\]"}</code></p>   <p>Available challenges are <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, <code>EMAIL_OTP</code>, <code>SMS_OTP</code>, and <code>WEB_AUTHN</code>.</p>   <p>Complete authentication in the <code>SELECT_CHALLENGE</code> response for <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, and <code>WEB_AUTHN</code>:</p>   <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "WEB_AUTHN", "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>     <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD", "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD_SRP", "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></li>   </ul>   <p>For <code>SMS_OTP</code> and <code>EMAIL_OTP</code>, respond with the username and answer. Your user pool will send a code for the user to submit in the next challenge response.</p>   <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "SMS_OTP", "USERNAME": "\[username\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "EMAIL_OTP", "USERNAME": "\[username\]"}</code></p></li>   </ul>  </dd>  <dt>   WEB_AUTHN  </dt>  <dd>   <p><code>"ChallengeName": "WEB_AUTHN", "ChallengeResponses": { "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>   <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p>  </dd>  <dt>   PASSWORD  </dt>  <dd>   <p><code>"ChallengeName": "PASSWORD", "ChallengeResponses": { "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p>  </dd>  <dt>   PASSWORD_SRP  </dt>  <dd>   <p><code>"ChallengeName": "PASSWORD_SRP", "ChallengeResponses": { "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p>  </dd>  <dt>   SMS_OTP  </dt>  <dd>   <p><code>"ChallengeName": "SMS_OTP", "ChallengeResponses": {"SMS_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   EMAIL_OTP  </dt>  <dd>   <p><code>"ChallengeName": "EMAIL_OTP", "ChallengeResponses": {"EMAIL_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   SMS_MFA  </dt>  <dd>   <p><code>"ChallengeName": "SMS_MFA", "ChallengeResponses": {"SMS_MFA_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   PASSWORD_VERIFIER  </dt>  <dd>   <p>This challenge response is part of the SRP flow. Amazon Cognito requires that your application respond to this challenge within a few seconds. When the response time exceeds this period, your user pool returns a <code>NotAuthorizedException</code> error.</p>   <p><code>"ChallengeName": "PASSWORD_VERIFIER", "ChallengeResponses": {"PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   CUSTOM_CHALLENGE  </dt>  <dd>   <p><code>"ChallengeName": "CUSTOM_CHALLENGE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[challenge_answer\]"}</code></p>  </dd>  <dt>   NEW_PASSWORD_REQUIRED  </dt>  <dd>   <p><code>"ChallengeName": "NEW_PASSWORD_REQUIRED", "ChallengeResponses": {"NEW_PASSWORD": "\[new_password\]", "USERNAME": "\[username\]"}</code></p>   <p>To set any required attributes that <code>InitiateAuth</code> returned in an <code>requiredAttributes</code> parameter, add <code>"userAttributes.\[attribute_name\]": "\[attribute_value\]"</code>. This parameter can also set values for writable attributes that aren't required by your user pool.</p><note>    <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p>   </note>  </dd>  <dt>   SOFTWARE_TOKEN_MFA  </dt>  <dd>   <p><code>"ChallengeName": "SOFTWARE_TOKEN_MFA", "ChallengeResponses": {"USERNAME": "\[username\]", "SOFTWARE_TOKEN_MFA_CODE": \[authenticator_code\]}</code></p>  </dd>  <dt>   DEVICE_SRP_AUTH  </dt>  <dd>   <p><code>"ChallengeName": "DEVICE_SRP_AUTH", "ChallengeResponses": {"USERNAME": "\[username\]", "DEVICE_KEY": "\[device_key\]", "SRP_A": "\[srp_a\]"}</code></p>  </dd>  <dt>   DEVICE_PASSWORD_VERIFIER  </dt>  <dd>   <p><code>"ChallengeName": "DEVICE_PASSWORD_VERIFIER", "ChallengeResponses": {"DEVICE_KEY": "\[device_key\]", "PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   MFA_SETUP  </dt>  <dd>   <p><code>"ChallengeName": "MFA_SETUP", "ChallengeResponses": {"USERNAME": "\[username\]"}, "SESSION": "\[Session ID from VerifySoftwareToken\]"</code></p>  </dd>  <dt>   SELECT_MFA_TYPE  </dt>  <dd>   <p><code>"ChallengeName": "SELECT_MFA_TYPE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[SMS_MFA|EMAIL_MFA|SOFTWARE_TOKEN_MFA\]"}</code></p>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
+    ///   - [`challenge_responses(impl Into<String>, impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::challenge_responses) / [`set_challenge_responses(Option<HashMap::<String, String>>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_challenge_responses):<br>required: **false**<br><p>The responses to the challenge that you received in the previous request. Each challenge has its own required response parameters. The following examples are partial JSON request bodies that highlight challenge-response parameters.</p><important>  <p>You must provide a SECRET_HASH parameter in all challenge responses to an app client that has a client secret. Include a <code>DEVICE_KEY</code> for device authentication.</p> </important> <dl> <dt>SELECT_CHALLENGE</dt> <dd> <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "USERNAME": "\[username\]", "ANSWER": "\[Challenge name\]"}</code></p> <p>Available challenges are <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, <code>EMAIL_OTP</code>, <code>SMS_OTP</code>, and <code>WEB_AUTHN</code>.</p> <p>Complete authentication in the <code>SELECT_CHALLENGE</code> response for <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, and <code>WEB_AUTHN</code>:</p> <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "WEB_AUTHN", "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>     <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD", "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD_SRP", "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></li>   </ul> <p>For <code>SMS_OTP</code> and <code>EMAIL_OTP</code>, respond with the username and answer. Your user pool will send a code for the user to submit in the next challenge response.</p> <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "SMS_OTP", "USERNAME": "\[username\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "EMAIL_OTP", "USERNAME": "\[username\]"}</code></p></li>   </ul></dd> <dt>WEB_AUTHN</dt> <dd> <p><code>"ChallengeName": "WEB_AUTHN", "ChallengeResponses": { "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p> <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></dd> <dt>PASSWORD</dt> <dd> <p><code>"ChallengeName": "PASSWORD", "ChallengeResponses": { "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></dd> <dt>PASSWORD_SRP</dt> <dd> <p><code>"ChallengeName": "PASSWORD_SRP", "ChallengeResponses": { "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></dd> <dt>SMS_OTP</dt> <dd> <p><code>"ChallengeName": "SMS_OTP", "ChallengeResponses": {"SMS_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p></dd> <dt>EMAIL_OTP</dt> <dd> <p><code>"ChallengeName": "EMAIL_OTP", "ChallengeResponses": {"EMAIL_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p></dd> <dt>SMS_MFA</dt> <dd> <p><code>"ChallengeName": "SMS_MFA", "ChallengeResponses": {"SMS_MFA_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p></dd> <dt>PASSWORD_VERIFIER</dt> <dd> <p>This challenge response is part of the SRP flow. Amazon Cognito requires that your application respond to this challenge within a few seconds. When the response time exceeds this period, your user pool returns a <code>NotAuthorizedException</code> error.</p> <p><code>"ChallengeName": "PASSWORD_VERIFIER", "ChallengeResponses": {"PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p></dd> <dt>CUSTOM_CHALLENGE</dt> <dd> <p><code>"ChallengeName": "CUSTOM_CHALLENGE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[challenge_answer\]"}</code></p></dd> <dt>NEW_PASSWORD_REQUIRED</dt> <dd> <p><code>"ChallengeName": "NEW_PASSWORD_REQUIRED", "ChallengeResponses": {"NEW_PASSWORD": "\[new_password\]", "USERNAME": "\[username\]"}</code></p> <p>To set any required attributes that <code>InitiateAuth</code> returned in an <code>requiredAttributes</code> parameter, add <code>"userAttributes.\[attribute_name\]": "\[attribute_value\]"</code>. This parameter can also set values for writable attributes that aren't required by your user pool.</p><note>    <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p>   </note></dd> <dt>SOFTWARE_TOKEN_MFA</dt> <dd> <p><code>"ChallengeName": "SOFTWARE_TOKEN_MFA", "ChallengeResponses": {"USERNAME": "\[username\]", "SOFTWARE_TOKEN_MFA_CODE": \[authenticator_code\]}</code></p></dd> <dt>DEVICE_SRP_AUTH</dt> <dd> <p><code>"ChallengeName": "DEVICE_SRP_AUTH", "ChallengeResponses": {"USERNAME": "\[username\]", "DEVICE_KEY": "\[device_key\]", "SRP_A": "\[srp_a\]"}</code></p></dd> <dt>DEVICE_PASSWORD_VERIFIER</dt> <dd> <p><code>"ChallengeName": "DEVICE_PASSWORD_VERIFIER", "ChallengeResponses": {"DEVICE_KEY": "\[device_key\]", "PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p></dd> <dt>MFA_SETUP</dt> <dd> <p><code>"ChallengeName": "MFA_SETUP", "ChallengeResponses": {"USERNAME": "\[username\]"}, "SESSION": "\[Session ID from VerifySoftwareToken\]"</code></p></dd> <dt>SELECT_MFA_TYPE</dt> <dd> <p><code>"ChallengeName": "SELECT_MFA_TYPE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[SMS_MFA|EMAIL_MFA|SOFTWARE_TOKEN_MFA\]"}</code></p></dd></dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
     ///   - [`session(impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::session) / [`set_session(Option<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_session):<br>required: **false**<br><p>The session identifier that maintains the state of authentication requests and challenge responses. If an <code>AdminInitiateAuth</code> or <code>AdminRespondToAuthChallenge</code> API request results in a determination that your application must pass another challenge, Amazon Cognito returns a session with other challenge parameters. Send this session identifier, unmodified, to the next <code>AdminRespondToAuthChallenge</code> request.</p><br>
     ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_analytics_metadata):<br>required: **false**<br><p>Information that supports analytics outcomes with Amazon Pinpoint, including the user's endpoint ID. The endpoint ID is a destination for Amazon Pinpoint push notifications, for example a device identifier, email address, or phone number.</p><br>
     ///   - [`context_data(ContextDataType)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::context_data) / [`set_context_data(Option<ContextDataType>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_context_data):<br>required: **false**<br><p>Contextual data about your user session like the device fingerprint, IP address, or location. Amazon Cognito threat protection evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-viewing-threat-protection-app.html">Collecting data for threat protection in applications</a>.</p><br>
```

### `src/client/create_identity_provider.rs`

```diff
--- reference/src/client/create_identity_provider.rs
+++ generated/src/client/create_identity_provider.rs
@@ -6,7 +6,7 @@
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The Id of the user pool where you want to create an IdP.</p><br>
     ///   - [`provider_name(impl Into<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::provider_name) / [`set_provider_name(Option<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_provider_name):<br>required: **true**<br><p>The name that you want to assign to the IdP. You can pass the identity provider name in the <code>identity_provider</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated IdP.</p><br>
     ///   - [`provider_type(IdentityProviderTypeType)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::provider_type) / [`set_provider_type(Option<IdentityProviderTypeType>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_provider_type):<br>required: **true**<br><p>The type of IdP that you want to add. Amazon Cognito supports OIDC, SAML 2.0, Login With Amazon, Sign In With Apple, Google, and Facebook IdPs.</p><br>
-    ///   - [`provider_details(impl Into<String>, impl Into<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::provider_details) / [`set_provider_details(Option<HashMap::<String, String>>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_provider_details):<br>required: **true**<br><p>The scopes, URLs, and identifiers for your external identity provider. The following examples describe the provider detail keys for each IdP type. These values and their schema are subject to change. Social IdP <code>authorize_scopes</code> values must match the values listed here.</p> <dl>  <dt>   OpenID Connect (OIDC)  </dt>  <dd>   <p>Amazon Cognito accepts the following elements when it can't discover endpoint URLs from <code>oidc_issuer</code>: <code>attributes_url</code>, <code>authorize_url</code>, <code>jwks_uri</code>, <code>token_url</code>.</p>   <p>Create or update request: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "attributes_url_add_attributes": "false", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p>  </dd>  <dt>   SAML  </dt>  <dd>   <p>Create or update request with Metadata URL: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256" }</code></p>   <p>Create or update request with Metadata file: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataFile": "\[metadata XML\]", "RequestSigningAlgorithm": "rsa-sha256" }</code></p>   <p>The value of <code>MetadataFile</code> must be the plaintext metadata document with all quote (") characters escaped by backslashes.</p>   <p>Describe response: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "ActiveEncryptionCertificate": "\[certificate\]", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256", "SLORedirectBindingURI": "https://auth.example.com/slo/saml", "SSORedirectBindingURI": "https://auth.example.com/sso/saml" }</code></p>  </dd>  <dt>   LoginWithAmazon  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "profile postal_code", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret"</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://api.amazon.com/user/profile", "attributes_url_add_attributes": "false", "authorize_scopes": "profile postal_code", "authorize_url": "https://www.amazon.com/ap/oa", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "POST", "token_url": "https://api.amazon.com/auth/o2/token" }</code></p>  </dd>  <dt>   Google  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email profile openid", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://people.googleapis.com/v1/people/me?personFields=", "attributes_url_add_attributes": "true", "authorize_scopes": "email profile openid", "authorize_url": "https://accounts.google.com/o/oauth2/v2/auth", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret", "oidc_issuer": "https://accounts.google.com", "token_request_method": "POST", "token_url": "https://www.googleapis.com/oauth2/v4/token" }</code></p>  </dd>  <dt>   SignInWithApple  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email name", "client_id": "com.example.cognito", "private_key": "1EXAMPLE", "key_id": "2EXAMPLE", "team_id": "3EXAMPLE" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_url_add_attributes": "false", "authorize_scopes": "email name", "authorize_url": "https://appleid.apple.com/auth/authorize", "client_id": "com.example.cognito", "key_id": "1EXAMPLE", "oidc_issuer": "https://appleid.apple.com", "team_id": "2EXAMPLE", "token_request_method": "POST", "token_url": "https://appleid.apple.com/auth/token" }</code></p>  </dd>  <dt>   Facebook  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "api_version": "v17.0", "authorize_scopes": "public_profile, email", "client_id": "1example23456789", "client_secret": "provider-app-client-secret" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "api_version": "v17.0", "attributes_url": "https://graph.facebook.com/v17.0/me?fields=", "attributes_url_add_attributes": "true", "authorize_scopes": "public_profile, email", "authorize_url": "https://www.facebook.com/v17.0/dialog/oauth", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "GET", "token_url": "https://graph.facebook.com/v17.0/oauth/access_token" }</code></p>  </dd> </dl><br>
+    ///   - [`provider_details(impl Into<String>, impl Into<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::provider_details) / [`set_provider_details(Option<HashMap::<String, String>>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_provider_details):<br>required: **true**<br><p>The scopes, URLs, and identifiers for your external identity provider. The following examples describe the provider detail keys for each IdP type. These values and their schema are subject to change. Social IdP <code>authorize_scopes</code> values must match the values listed here.</p> <dl> <dt>OpenID Connect (OIDC)</dt> <dd> <p>Amazon Cognito accepts the following elements when it can't discover endpoint URLs from <code>oidc_issuer</code>: <code>attributes_url</code>, <code>authorize_url</code>, <code>jwks_uri</code>, <code>token_url</code>.</p> <p>Create or update request: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "attributes_url_add_attributes": "false", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p></dd> <dt>SAML</dt> <dd> <p>Create or update request with Metadata URL: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256" }</code></p> <p>Create or update request with Metadata file: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataFile": "\[metadata XML\]", "RequestSigningAlgorithm": "rsa-sha256" }</code></p> <p>The value of <code>MetadataFile</code> must be the plaintext metadata document with all quote (") characters escaped by backslashes.</p> <p>Describe response: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "ActiveEncryptionCertificate": "\[certificate\]", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256", "SLORedirectBindingURI": "https://auth.example.com/slo/saml", "SSORedirectBindingURI": "https://auth.example.com/sso/saml" }</code></p></dd> <dt>LoginWithAmazon</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "profile postal_code", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret"</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://api.amazon.com/user/profile", "attributes_url_add_attributes": "false", "authorize_scopes": "profile postal_code", "authorize_url": "https://www.amazon.com/ap/oa", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "POST", "token_url": "https://api.amazon.com/auth/o2/token" }</code></p></dd> <dt>Google</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email profile openid", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret" }</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://people.googleapis.com/v1/people/me?personFields=", "attributes_url_add_attributes": "true", "authorize_scopes": "email profile openid", "authorize_url": "https://accounts.google.com/o/oauth2/v2/auth", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret", "oidc_issuer": "https://accounts.google.com", "token_request_method": "POST", "token_url": "https://www.googleapis.com/oauth2/v4/token" }</code></p></dd> <dt>SignInWithApple</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email name", "client_id": "com.example.cognito", "private_key": "1EXAMPLE", "key_id": "2EXAMPLE", "team_id": "3EXAMPLE" }</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_url_add_attributes": "false", "authorize_scopes": "email name", "authorize_url": "https://appleid.apple.com/auth/authorize", "client_id": "com.example.cognito", "key_id": "1EXAMPLE", "oidc_issuer": "https://appleid.apple.com", "team_id": "2EXAMPLE", "token_request_method": "POST", "token_url": "https://appleid.apple.com/auth/token" }</code></p></dd> <dt>Facebook</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "api_version": "v17.0", "authorize_scopes": "public_profile, email", "client_id": "1example23456789", "client_secret": "provider-app-client-secret" }</code></p> <p>Describe response: <code>"ProviderDetails": { "api_version": "v17.0", "attributes_url": "https://graph.facebook.com/v17.0/me?fields=", "attributes_url_add_attributes": "true", "authorize_scopes": "public_profile, email", "authorize_url": "https://www.facebook.com/v17.0/dialog/oauth", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "GET", "token_url": "https://graph.facebook.com/v17.0/oauth/access_token" }</code></p></dd></dl><br>
     ///   - [`attribute_mapping(impl Into<String>, impl Into<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::attribute_mapping) / [`set_attribute_mapping(Option<HashMap::<String, String>>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_attribute_mapping):<br>required: **false**<br><p>A mapping of IdP attributes to standard and custom user pool attributes. Specify a user pool attribute as the key of the key-value pair, and the IdP attribute claim name as the value.</p><br>
     ///   - [`idp_identifiers(impl Into<String>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::idp_identifiers) / [`set_idp_identifiers(Option<Vec::<String>>)`](crate::operation::create_identity_provider::builders::CreateIdentityProviderFluentBuilder::set_idp_identifiers):<br>required: **false**<br><p>An array of IdP identifiers, for example <code>"IdPIdentifiers": \[ "MyIdP", "MyIdP2" \]</code>. Identifiers are friendly names that you can pass in the <code>idp_identifier</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated IdP. Identifiers in a domain format also enable the use of <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managing-saml-idp-naming.html">email-address matching with SAML providers</a>.</p><br>
     /// - On success, responds with [`CreateIdentityProviderOutput`](crate::operation::create_identity_provider::CreateIdentityProviderOutput) with field(s):
```

### `src/client/create_user_pool_client.rs`

```diff
--- reference/src/client/create_user_pool_client.rs
+++ generated/src/client/create_user_pool_client.rs
@@ -15,10 +15,10 @@
     ///   - [`write_attributes(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::write_attributes) / [`set_write_attributes(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_write_attributes):<br>required: **false**<br><p>The list of user attributes that you want your app client to have write access to. After your user authenticates in your app, their access token authorizes them to set or modify their own attribute value for any attribute in this list.</p> <p>When you don't specify the <code>WriteAttributes</code> for your app client, your app can write the values of the Standard attributes of your user pool. When your user pool has write access to these default attributes, <code>WriteAttributes</code> doesn't return any information. Amazon Cognito only populates <code>WriteAttributes</code> in the API response if you have specified your own custom set of write attributes.</p> <p>If your app client allows users to sign in through an IdP, this array must include all attributes that you have mapped to IdP attributes. Amazon Cognito updates mapped attributes when users sign in to your application through an IdP. If your app client does not have write access to a mapped attribute, Amazon Cognito throws an error when it tries to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying IdP Attribute Mappings for Your user pool</a>.</p><br>
     ///   - [`explicit_auth_flows(ExplicitAuthFlowsType)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::explicit_auth_flows) / [`set_explicit_auth_flows(Option<Vec::<ExplicitAuthFlowsType>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_explicit_auth_flows):<br>required: **false**<br><p>The <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow-methods.html">authentication flows</a> that you want your user pool client to support. For each app client in your user pool, you can sign in your users with any combination of one or more flows, including with a user name and Secure Remote Password (SRP), a user name and password, or a custom authentication process that you define with Lambda functions.</p><note>  <p>If you don't specify a value for <code>ExplicitAuthFlows</code>, your app client supports <code>ALLOW_REFRESH_TOKEN_AUTH</code>, <code>ALLOW_USER_SRP_AUTH</code>, and <code>ALLOW_CUSTOM_AUTH</code>.</p> </note> <p>The values for authentication flow options include the following.</p> <ul>  <li>   <p><code>ALLOW_USER_AUTH</code>: Enable selection-based sign-in with <code>USER_AUTH</code>. This setting covers username-password, secure remote password (SRP), passwordless, and passkey authentication. This authentiation flow can do username-password and SRP authentication without other <code>ExplicitAuthFlows</code> permitting them. For example users can complete an SRP challenge through <code>USER_AUTH</code> without the flow <code>USER_SRP_AUTH</code> being active for the app client. This flow doesn't include <code>CUSTOM_AUTH</code>.</p>   <p>To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p></li>  <li>   <p><code>ALLOW_ADMIN_USER_PASSWORD_AUTH</code>: Enable admin based user password authentication flow <code>ADMIN_USER_PASSWORD_AUTH</code>. This setting replaces the <code>ADMIN_NO_SRP_AUTH</code> setting. With this authentication flow, your app passes a user name and password to Amazon Cognito in the request, instead of using the Secure Remote Password (SRP) protocol to securely transmit the password.</p></li>  <li>   <p><code>ALLOW_CUSTOM_AUTH</code>: Enable Lambda trigger based authentication.</p></li>  <li>   <p><code>ALLOW_USER_PASSWORD_AUTH</code>: Enable user password-based authentication. In this flow, Amazon Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p></li>  <li>   <p><code>ALLOW_USER_SRP_AUTH</code>: Enable SRP-based authentication.</p></li>  <li>   <p><code>ALLOW_REFRESH_TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p></li> </ul> <p>In some environments, you will see the values <code>ADMIN_NO_SRP_AUTH</code>, <code>CUSTOM_AUTH_FLOW_ONLY</code>, or <code>USER_PASSWORD_AUTH</code>. You can't assign these legacy <code>ExplicitAuthFlows</code> values to user pool clients at the same time as values that begin with <code>ALLOW_</code>, like <code>ALLOW_USER_SRP_AUTH</code>.</p><br>
     ///   - [`supported_identity_providers(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::supported_identity_providers) / [`set_supported_identity_providers(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_supported_identity_providers):<br>required: **false**<br><p>A list of provider names for the identity providers (IdPs) that are supported on this client. The following are supported: <code>COGNITO</code>, <code>Facebook</code>, <code>Google</code>, <code>SignInWithApple</code>, and <code>LoginWithAmazon</code>. You can also specify the names that you configured for the SAML and OIDC IdPs in your user pool, for example <code>MySAMLIdP</code> or <code>MyOIDCIdP</code>.</p> <p>This parameter sets the IdPs that <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html">managed login</a> will display on the login page for your app client. The removal of <code>COGNITO</code> from this list doesn't prevent authentication operations for local users with the user pools API in an Amazon Web Services SDK. The only way to prevent SDK-based authentication is to block access with a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-waf.html">WAF rule</a>.</p><br>
-    ///   - [`callback_urls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::callback_urls) / [`set_callback_urls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_callback_urls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
-    ///   - [`logout_urls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::logout_urls) / [`set_logout_urls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_logout_urls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
+    ///   - [`callback_ur_ls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::callback_ur_ls) / [`set_callback_ur_ls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_callback_ur_ls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
+    ///   - [`logout_ur_ls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::logout_ur_ls) / [`set_logout_ur_ls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_logout_ur_ls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
     ///   - [`default_redirect_uri(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::default_redirect_uri) / [`set_default_redirect_uri(Option<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_default_redirect_uri):<br>required: **false**<br><p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p><br>
-    ///   - [`allowed_o_auth_flows(OAuthFlowType)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::allowed_o_auth_flows) / [`set_allowed_o_auth_flows(Option<Vec::<OAuthFlowType>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_allowed_o_auth_flows):<br>required: **false**<br><p>The OAuth grant types that you want your app client to generate for clients in managed login authentication. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p> <dl>  <dt>   code  </dt>  <dd>   <p>Use a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the <code>/oauth2/token</code> endpoint.</p>  </dd>  <dt>   implicit  </dt>  <dd>   <p>Issue the access token, and the ID token when scopes like <code>openid</code> and <code>profile</code> are requested, directly to your user.</p>  </dd>  <dt>   client_credentials  </dt>  <dd>   <p>Issue the access token from the <code>/oauth2/token</code> endpoint directly to a non-person user, authorized by a combination of the client ID and client secret.</p>  </dd> </dl><br>
+    ///   - [`allowed_o_auth_flows(OAuthFlowType)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::allowed_o_auth_flows) / [`set_allowed_o_auth_flows(Option<Vec::<OAuthFlowType>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_allowed_o_auth_flows):<br>required: **false**<br><p>The OAuth grant types that you want your app client to generate for clients in managed login authentication. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p> <dl> <dt>code</dt> <dd> <p>Use a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the <code>/oauth2/token</code> endpoint.</p></dd> <dt>implicit</dt> <dd> <p>Issue the access token, and the ID token when scopes like <code>openid</code> and <code>profile</code> are requested, directly to your user.</p></dd> <dt>client_credentials</dt> <dd> <p>Issue the access token from the <code>/oauth2/token</code> endpoint directly to a non-person user, authorized by a combination of the client ID and client secret.</p></dd></dl><br>
     ///   - [`allowed_o_auth_scopes(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::allowed_o_auth_scopes) / [`set_allowed_o_auth_scopes(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_allowed_o_auth_scopes):<br>required: **false**<br><p>The OAuth, OpenID Connect (OIDC), and custom scopes that you want to permit your app client to authorize access with. Scopes govern access control to user pool self-service API operations, user data from the <code>userInfo</code> endpoint, and third-party APIs. Scope values include <code>phone</code>, <code>email</code>, <code>openid</code>, and <code>profile</code>. The <code>aws.cognito.signin.user.admin</code> scope authorizes user self-service operations. Custom scopes with resource servers authorize access to external APIs.</p><br>
     ///   - [`allowed_o_auth_flows_user_pool_client(bool)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::allowed_o_auth_flows_user_pool_client) / [`set_allowed_o_auth_flows_user_pool_client(Option<bool>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_allowed_o_auth_flows_user_pool_client):<br>required: **false**<br><p>Set to <code>true</code> to use OAuth 2.0 authorization server features in your app client.</p> <p>This parameter must have a value of <code>true</code> before you can configure the following features in your app client.</p> <ul>  <li>   <p><code>CallBackURLs</code>: Callback URLs.</p></li>  <li>   <p><code>LogoutURLs</code>: Sign-out redirect URLs.</p></li>  <li>   <p><code>AllowedOAuthScopes</code>: OAuth 2.0 scopes.</p></li>  <li>   <p><code>AllowedOAuthFlows</code>: Support for authorization code, implicit, and client credentials OAuth 2.0 grants.</p></li> </ul> <p>To use authorization server features, configure one of these features in the Amazon Cognito console or set <code>AllowedOAuthFlowsUserPoolClient</code> to <code>true</code> in a <code>CreateUserPoolClient</code> or <code>UpdateUserPoolClient</code> API request. If you don't set a value for <code>AllowedOAuthFlowsUserPoolClient</code> in a request with the CLI or SDKs, it defaults to <code>false</code>. When <code>false</code>, only SDK-based API sign-in is permitted.</p><br>
     ///   - [`analytics_configuration(AnalyticsConfigurationType)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::analytics_configuration) / [`set_analytics_configuration(Option<AnalyticsConfigurationType>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_analytics_configuration):<br>required: **false**<br><p>The user pool analytics configuration for collecting metrics and sending them to your Amazon Pinpoint campaign.</p> <p>In Amazon Web Services Regions where Amazon Pinpoint isn't available, user pools might not have access to analytics or might be configurable with campaigns in the US East (N. Virginia) Region. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-pinpoint-integration.html">Using Amazon Pinpoint analytics</a>.</p><br>
```

### `src/client/initiate_auth.rs`

```diff
--- reference/src/client/initiate_auth.rs
+++ generated/src/client/initiate_auth.rs
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`InitiateAuth`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`auth_flow(AuthFlowType)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_flow) / [`set_auth_flow(Option<AuthFlowType>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_flow):<br>required: **true**<br><p>The authentication flow that you want to initiate. Each <code>AuthFlow</code> has linked <code>AuthParameters</code> that you must submit. The following are some example flows.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <p>The entry point for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a> with passwords, one-time passwords, and WebAuthn authenticators. Request a preferred authentication type or review available authentication types. From the offered authentication types, select one in a challenge response and then authenticate with that method in an additional challenge response. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <p>Username-password authentication with the Secure Remote Password (SRP) protocol. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow.html#Using-SRP-password-verification-in-custom-authentication-flow">Use SRP password verification in custom authentication flow</a>.</p>  </dd>  <dt>   REFRESH_TOKEN_AUTH and REFRESH_TOKEN  </dt>  <dd>   <p>Receive new ID and access tokens when you pass a <code>REFRESH_TOKEN</code> parameter with a valid refresh token as the value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html">Using the refresh token</a>.</p>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <p>Custom authentication with Lambda triggers. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-challenge.html">Custom authentication challenge Lambda triggers</a>.</p>  </dd>  <dt>   USER_PASSWORD_AUTH  </dt>  <dd>   <p>Client-side username-password authentication with the password sent directly in the request. For more information about client-side and server-side authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-public-server-side.html">SDK authorization models</a>.</p>  </dd> </dl> <p><code>ADMIN_USER_PASSWORD_AUTH</code> is a flow type of <code>AdminInitiateAuth</code> and isn't valid for InitiateAuth. <code>ADMIN_NO_SRP_AUTH</code> is a legacy server-side username-password flow and isn't valid for InitiateAuth.</p><br>
-    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li>   </ul>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>SRP_A</code> (required)</p></li>   </ul>  </dd>  <dt>   USER_PASSWORD_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PASSWORD</code> (required)</p></li>   </ul>  </dd>  <dt>   REFRESH_TOKEN_AUTH/REFRESH_TOKEN  </dt>  <dd>   <ul>    <li>     <p><code>REFRESH_TOKEN</code>(required)</p></li>   </ul>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>ChallengeName: SRP_A</code> (when doing SRP authentication before custom challenges)</p></li>    <li>     <p><code>SRP_A: (An SRP_A value)</code> (when doing SRP authentication before custom challenges)</p></li>   </ul>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
-    ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The <code>ClientMetadata</code> value is passed as input to the functions for only the following triggers:</p> <ul>  <li>   <p>Pre signup</p></li>  <li>   <p>Pre authentication</p></li>  <li>   <p>User migration</p></li> </ul> <p>This request also invokes the functions for the following triggers, but doesn't pass <code>ClientMetadata</code>:</p> <ul>  <li>   <p>Post authentication</p></li>  <li>   <p>Custom message</p></li>  <li>   <p>Pre token generation</p></li>  <li>   <p>Create auth challenge</p></li>  <li>   <p>Define auth challenge</p></li>  <li>   <p>Custom email sender</p></li>  <li>   <p>Custom SMS sender</p></li> </ul><note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
+    ///   - [`auth_flow(AuthFlowType)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_flow) / [`set_auth_flow(Option<AuthFlowType>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_flow):<br>required: **true**<br><p>The authentication flow that you want to initiate. Each <code>AuthFlow</code> has linked <code>AuthParameters</code> that you must submit. The following are some example flows.</p> <dl> <dt>USER_AUTH</dt> <dd> <p>The entry point for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a> with passwords, one-time passwords, and WebAuthn authenticators. Request a preferred authentication type or review available authentication types. From the offered authentication types, select one in a challenge response and then authenticate with that method in an additional challenge response. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p></dd> <dt>USER_SRP_AUTH</dt> <dd> <p>Username-password authentication with the Secure Remote Password (SRP) protocol. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow.html#Using-SRP-password-verification-in-custom-authentication-flow">Use SRP password verification in custom authentication flow</a>.</p></dd> <dt>REFRESH_TOKEN_AUTH and REFRESH_TOKEN</dt> <dd> <p>Receive new ID and access tokens when you pass a <code>REFRESH_TOKEN</code> parameter with a valid refresh token as the value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html">Using the refresh token</a>.</p></dd> <dt>CUSTOM_AUTH</dt> <dd> <p>Custom authentication with Lambda triggers. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-challenge.html">Custom authentication challenge Lambda triggers</a>.</p></dd> <dt>USER_PASSWORD_AUTH</dt> <dd> <p>Client-side username-password authentication with the password sent directly in the request. For more information about client-side and server-side authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-public-server-side.html">SDK authorization models</a>.</p></dd></dl> <p><code>ADMIN_USER_PASSWORD_AUTH</code> is a flow type of <code>AdminInitiateAuth</code> and isn't valid for InitiateAuth. <code>ADMIN_NO_SRP_AUTH</code> is a legacy server-side username-password flow and isn't valid for InitiateAuth.</p><br>
+    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl> <dt>USER_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li>   </ul></dd> <dt>USER_SRP_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>SRP_A</code> (required)</p></li>   </ul></dd> <dt>USER_PASSWORD_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PASSWORD</code> (required)</p></li>   </ul></dd> <dt>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</dt> <dd> <ul>    <li>     <p><code>REFRESH_TOKEN</code>(required)</p></li>   </ul></dd> <dt>CUSTOM_AUTH</dt> <dd> <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>ChallengeName: SRP_A</code> (when doing SRP authentication before custom challenges)</p></li>    <li>     <p><code>SRP_A: (An SRP_A value)</code> (when doing SRP authentication before custom challenges)</p></li>   </ul></dd></dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
+    ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The <code>ClientMetadata</code> value is passed as input to the functions for only the following triggers:</p> <ul>  <li>   <p>Pre signup</p></li>  <li>   <p>Pre authentication</p></li>  <li>   <p>User migration</p></li> </ul> <p>This request also invokes the functions for the following triggers, but doesn't pass <code>ClientMetadata</code>:</p> <ul>  <li>   <p>Post authentication</p></li>  <li>   <p>Custom message</p></li>  <li>   <p>Pre token generation</p></li>  <li>   <p>Create auth challenge</p></li>  <li>   <p>Define auth challenge</p></li>  <li>   <p>Custom email sender</p></li>  <li>   <p>Custom SMS sender</p></li> </ul> <note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
     ///   - [`client_id(impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::client_id) / [`set_client_id(Option<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_client_id):<br>required: **true**<br><p>The ID of the app client that your user wants to sign in to.</p><br>
     ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_analytics_metadata):<br>required: **false**<br><p>Information that supports analytics outcomes with Amazon Pinpoint, including the user's endpoint ID. The endpoint ID is a destination for Amazon Pinpoint push notifications, for example a device identifier, email address, or phone number.</p><br>
     ///   - [`user_context_data(UserContextDataType)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::user_context_data) / [`set_user_context_data(Option<UserContextDataType>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_user_context_data):<br>required: **false**<br><p>Contextual data about your user session like the device fingerprint, IP address, or location. Amazon Cognito threat protection evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-viewing-threat-protection-app.html">Collecting data for threat protection in applications</a>.</p><br>
```

### `src/client/respond_to_auth_challenge.rs`

```diff
--- reference/src/client/respond_to_auth_challenge.rs
+++ generated/src/client/respond_to_auth_challenge.rs
@@ -6,7 +6,7 @@
     ///   - [`client_id(impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::client_id) / [`set_client_id(Option<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_client_id):<br>required: **true**<br><p>The ID of the app client where the user is signing in.</p><br>
     ///   - [`challenge_name(ChallengeNameType)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::challenge_name) / [`set_challenge_name(Option<ChallengeNameType>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_challenge_name):<br>required: **true**<br><p>The name of the challenge that you are responding to.</p><note>  <p>You can't respond to an <code>ADMIN_NO_SRP_AUTH</code> challenge with this operation.</p> </note> <p>Possible challenges include the following:</p><note>  <p>All of the following challenges require <code>USERNAME</code> and, when the app client has a client secret, <code>SECRET_HASH</code> in the parameters. Include a <code>DEVICE_KEY</code> for device authentication.</p> </note> <ul>  <li>   <p><code>WEB_AUTHN</code>: Respond to the challenge with the results of a successful authentication with a WebAuthn authenticator, or passkey, as <code>CREDENTIAL</code>. Examples of WebAuthn authenticators include biometric devices and security keys.</p></li>  <li>   <p><code>PASSWORD</code>: Respond with the user's password as <code>PASSWORD</code>.</p></li>  <li>   <p><code>PASSWORD_SRP</code>: Respond with the initial SRP secret as <code>SRP_A</code>.</p></li>  <li>   <p><code>SELECT_CHALLENGE</code>: Respond with a challenge selection as <code>ANSWER</code>. It must be one of the challenge types in the <code>AvailableChallenges</code> response parameter. Add the parameters of the selected challenge, for example <code>USERNAME</code> and <code>SMS_OTP</code>.</p></li>  <li>   <p><code>SMS_MFA</code>: Respond with the code that your user pool delivered in an SMS message, as <code>SMS_MFA_CODE</code></p></li>  <li>   <p><code>EMAIL_MFA</code>: Respond with the code that your user pool delivered in an email message, as <code>EMAIL_MFA_CODE</code></p></li>  <li>   <p><code>EMAIL_OTP</code>: Respond with the code that your user pool delivered in an email message, as <code>EMAIL_OTP_CODE</code> .</p></li>  <li>   <p><code>SMS_OTP</code>: Respond with the code that your user pool delivered in an SMS message, as <code>SMS_OTP_CODE</code>.</p></li>  <li>   <p><code>PASSWORD_VERIFIER</code>: Respond with the second stage of SRP secrets as <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, and <code>TIMESTAMP</code>.</p></li>  <li>   <p><code>CUSTOM_CHALLENGE</code>: This is returned if your custom authentication flow determines that the user should pass another challenge before tokens are issued. The parameters of the challenge are determined by your Lambda function and issued in the <code>ChallengeParameters</code> of a challenge response.</p></li>  <li>   <p><code>DEVICE_SRP_AUTH</code>: Respond with the initial parameters of device SRP authentication. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html#user-pools-remembered-devices-signing-in-with-a-device">Signing in with a device</a>.</p></li>  <li>   <p><code>DEVICE_PASSWORD_VERIFIER</code>: Respond with <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, and <code>TIMESTAMP</code> after client-side SRP calculations. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html#user-pools-remembered-devices-signing-in-with-a-device">Signing in with a device</a>.</p></li>  <li>   <p><code>NEW_PASSWORD_REQUIRED</code>: For users who are required to change their passwords after successful first login. Respond to this challenge with <code>NEW_PASSWORD</code> and any required attributes that Amazon Cognito returned in the <code>requiredAttributes</code> parameter. You can also set values for attributes that aren't required by your user pool and that your app client can write.</p>   <p>Amazon Cognito only returns this challenge for users who have temporary passwords. When you create passwordless users, you must provide values for all required attributes.</p><note>    <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p>   </note></li>  <li>   <p><code>MFA_SETUP</code>: For users who are required to setup an MFA factor before they can sign in. The MFA types activated for the user pool will be listed in the challenge parameters <code>MFAS_CAN_SETUP</code> value.</p>   <p>To set up time-based one-time password (TOTP) MFA, use the session returned in this challenge from <code>InitiateAuth</code> or <code>AdminInitiateAuth</code> as an input to <code>AssociateSoftwareToken</code>. Then, use the session returned by <code>VerifySoftwareToken</code> as an input to <code>RespondToAuthChallenge</code> or <code>AdminRespondToAuthChallenge</code> with challenge name <code>MFA_SETUP</code> to complete sign-in.</p>   <p>To set up SMS or email MFA, collect a <code>phone_number</code> or <code>email</code> attribute for the user. Then restart the authentication flow with an <code>InitiateAuth</code> or <code>AdminInitiateAuth</code> request.</p></li> </ul><br>
     ///   - [`session(impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::session) / [`set_session(Option<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_session):<br>required: **false**<br><p>The session identifier that maintains the state of authentication requests and challenge responses. If an <code>AdminInitiateAuth</code> or <code>AdminRespondToAuthChallenge</code> API request results in a determination that your application must pass another challenge, Amazon Cognito returns a session with other challenge parameters. Send this session identifier, unmodified, to the next <code>AdminRespondToAuthChallenge</code> request.</p><br>
-    ///   - [`challenge_responses(impl Into<String>, impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::challenge_responses) / [`set_challenge_responses(Option<HashMap::<String, String>>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_challenge_responses):<br>required: **false**<br><p>The responses to the challenge that you received in the previous request. Each challenge has its own required response parameters. The following examples are partial JSON request bodies that highlight challenge-response parameters.</p><important>  <p>You must provide a SECRET_HASH parameter in all challenge responses to an app client that has a client secret. Include a <code>DEVICE_KEY</code> for device authentication.</p> </important> <dl>  <dt>   SELECT_CHALLENGE  </dt>  <dd>   <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "USERNAME": "\[username\]", "ANSWER": "\[Challenge name\]"}</code></p>   <p>Available challenges are <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, <code>EMAIL_OTP</code>, <code>SMS_OTP</code>, and <code>WEB_AUTHN</code>.</p>   <p>Complete authentication in the <code>SELECT_CHALLENGE</code> response for <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, and <code>WEB_AUTHN</code>:</p>   <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "WEB_AUTHN", "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>     <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD", "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD_SRP", "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></li>   </ul>   <p>For <code>SMS_OTP</code> and <code>EMAIL_OTP</code>, respond with the username and answer. Your user pool will send a code for the user to submit in the next challenge response.</p>   <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "SMS_OTP", "USERNAME": "\[username\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "EMAIL_OTP", "USERNAME": "\[username\]"}</code></p></li>   </ul>  </dd>  <dt>   WEB_AUTHN  </dt>  <dd>   <p><code>"ChallengeName": "WEB_AUTHN", "ChallengeResponses": { "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>   <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p>  </dd>  <dt>   PASSWORD  </dt>  <dd>   <p><code>"ChallengeName": "PASSWORD", "ChallengeResponses": { "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p>  </dd>  <dt>   PASSWORD_SRP  </dt>  <dd>   <p><code>"ChallengeName": "PASSWORD_SRP", "ChallengeResponses": { "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p>  </dd>  <dt>   SMS_OTP  </dt>  <dd>   <p><code>"ChallengeName": "SMS_OTP", "ChallengeResponses": {"SMS_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   EMAIL_OTP  </dt>  <dd>   <p><code>"ChallengeName": "EMAIL_OTP", "ChallengeResponses": {"EMAIL_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   SMS_MFA  </dt>  <dd>   <p><code>"ChallengeName": "SMS_MFA", "ChallengeResponses": {"SMS_MFA_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   PASSWORD_VERIFIER  </dt>  <dd>   <p>This challenge response is part of the SRP flow. Amazon Cognito requires that your application respond to this challenge within a few seconds. When the response time exceeds this period, your user pool returns a <code>NotAuthorizedException</code> error.</p>   <p><code>"ChallengeName": "PASSWORD_VERIFIER", "ChallengeResponses": {"PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   CUSTOM_CHALLENGE  </dt>  <dd>   <p><code>"ChallengeName": "CUSTOM_CHALLENGE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[challenge_answer\]"}</code></p>  </dd>  <dt>   NEW_PASSWORD_REQUIRED  </dt>  <dd>   <p><code>"ChallengeName": "NEW_PASSWORD_REQUIRED", "ChallengeResponses": {"NEW_PASSWORD": "\[new_password\]", "USERNAME": "\[username\]"}</code></p>   <p>To set any required attributes that <code>InitiateAuth</code> returned in an <code>requiredAttributes</code> parameter, add <code>"userAttributes.\[attribute_name\]": "\[attribute_value\]"</code>. This parameter can also set values for writable attributes that aren't required by your user pool.</p><note>    <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p>   </note>  </dd>  <dt>   SOFTWARE_TOKEN_MFA  </dt>  <dd>   <p><code>"ChallengeName": "SOFTWARE_TOKEN_MFA", "ChallengeResponses": {"USERNAME": "\[username\]", "SOFTWARE_TOKEN_MFA_CODE": \[authenticator_code\]}</code></p>  </dd>  <dt>   DEVICE_SRP_AUTH  </dt>  <dd>   <p><code>"ChallengeName": "DEVICE_SRP_AUTH", "ChallengeResponses": {"USERNAME": "\[username\]", "DEVICE_KEY": "\[device_key\]", "SRP_A": "\[srp_a\]"}</code></p>  </dd>  <dt>   DEVICE_PASSWORD_VERIFIER  </dt>  <dd>   <p><code>"ChallengeName": "DEVICE_PASSWORD_VERIFIER", "ChallengeResponses": {"DEVICE_KEY": "\[device_key\]", "PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   MFA_SETUP  </dt>  <dd>   <p><code>"ChallengeName": "MFA_SETUP", "ChallengeResponses": {"USERNAME": "\[username\]"}, "SESSION": "\[Session ID from VerifySoftwareToken\]"</code></p>  </dd>  <dt>   SELECT_MFA_TYPE  </dt>  <dd>   <p><code>"ChallengeName": "SELECT_MFA_TYPE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[SMS_MFA|EMAIL_MFA|SOFTWARE_TOKEN_MFA\]"}</code></p>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
+    ///   - [`challenge_responses(impl Into<String>, impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::challenge_responses) / [`set_challenge_responses(Option<HashMap::<String, String>>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_challenge_responses):<br>required: **false**<br><p>The responses to the challenge that you received in the previous request. Each challenge has its own required response parameters. The following examples are partial JSON request bodies that highlight challenge-response parameters.</p><important>  <p>You must provide a SECRET_HASH parameter in all challenge responses to an app client that has a client secret. Include a <code>DEVICE_KEY</code> for device authentication.</p> </important> <dl> <dt>SELECT_CHALLENGE</dt> <dd> <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "USERNAME": "\[username\]", "ANSWER": "\[Challenge name\]"}</code></p> <p>Available challenges are <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, <code>EMAIL_OTP</code>, <code>SMS_OTP</code>, and <code>WEB_AUTHN</code>.</p> <p>Complete authentication in the <code>SELECT_CHALLENGE</code> response for <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, and <code>WEB_AUTHN</code>:</p> <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "WEB_AUTHN", "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>     <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD", "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD_SRP", "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></li>   </ul> <p>For <code>SMS_OTP</code> and <code>EMAIL_OTP</code>, respond with the username and answer. Your user pool will send a code for the user to submit in the next challenge response.</p> <ul>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "SMS_OTP", "USERNAME": "\[username\]"}</code></p></li>    <li>     <p><code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "EMAIL_OTP", "USERNAME": "\[username\]"}</code></p></li>   </ul></dd> <dt>WEB_AUTHN</dt> <dd> <p><code>"ChallengeName": "WEB_AUTHN", "ChallengeResponses": { "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p> <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></dd> <dt>PASSWORD</dt> <dd> <p><code>"ChallengeName": "PASSWORD", "ChallengeResponses": { "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></dd> <dt>PASSWORD_SRP</dt> <dd> <p><code>"ChallengeName": "PASSWORD_SRP", "ChallengeResponses": { "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></dd> <dt>SMS_OTP</dt> <dd> <p><code>"ChallengeName": "SMS_OTP", "ChallengeResponses": {"SMS_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p></dd> <dt>EMAIL_OTP</dt> <dd> <p><code>"ChallengeName": "EMAIL_OTP", "ChallengeResponses": {"EMAIL_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p></dd> <dt>SMS_MFA</dt> <dd> <p><code>"ChallengeName": "SMS_MFA", "ChallengeResponses": {"SMS_MFA_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p></dd> <dt>PASSWORD_VERIFIER</dt> <dd> <p>This challenge response is part of the SRP flow. Amazon Cognito requires that your application respond to this challenge within a few seconds. When the response time exceeds this period, your user pool returns a <code>NotAuthorizedException</code> error.</p> <p><code>"ChallengeName": "PASSWORD_VERIFIER", "ChallengeResponses": {"PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p></dd> <dt>CUSTOM_CHALLENGE</dt> <dd> <p><code>"ChallengeName": "CUSTOM_CHALLENGE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[challenge_answer\]"}</code></p></dd> <dt>NEW_PASSWORD_REQUIRED</dt> <dd> <p><code>"ChallengeName": "NEW_PASSWORD_REQUIRED", "ChallengeResponses": {"NEW_PASSWORD": "\[new_password\]", "USERNAME": "\[username\]"}</code></p> <p>To set any required attributes that <code>InitiateAuth</code> returned in an <code>requiredAttributes</code> parameter, add <code>"userAttributes.\[attribute_name\]": "\[attribute_value\]"</code>. This parameter can also set values for writable attributes that aren't required by your user pool.</p><note>    <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p>   </note></dd> <dt>SOFTWARE_TOKEN_MFA</dt> <dd> <p><code>"ChallengeName": "SOFTWARE_TOKEN_MFA", "ChallengeResponses": {"USERNAME": "\[username\]", "SOFTWARE_TOKEN_MFA_CODE": \[authenticator_code\]}</code></p></dd> <dt>DEVICE_SRP_AUTH</dt> <dd> <p><code>"ChallengeName": "DEVICE_SRP_AUTH", "ChallengeResponses": {"USERNAME": "\[username\]", "DEVICE_KEY": "\[device_key\]", "SRP_A": "\[srp_a\]"}</code></p></dd> <dt>DEVICE_PASSWORD_VERIFIER</dt> <dd> <p><code>"ChallengeName": "DEVICE_PASSWORD_VERIFIER", "ChallengeResponses": {"DEVICE_KEY": "\[device_key\]", "PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p></dd> <dt>MFA_SETUP</dt> <dd> <p><code>"ChallengeName": "MFA_SETUP", "ChallengeResponses": {"USERNAME": "\[username\]"}, "SESSION": "\[Session ID from VerifySoftwareToken\]"</code></p></dd> <dt>SELECT_MFA_TYPE</dt> <dd> <p><code>"ChallengeName": "SELECT_MFA_TYPE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[SMS_MFA|EMAIL_MFA|SOFTWARE_TOKEN_MFA\]"}</code></p></dd></dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
     ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_analytics_metadata):<br>required: **false**<br><p>Information that supports analytics outcomes with Amazon Pinpoint, including the user's endpoint ID. The endpoint ID is a destination for Amazon Pinpoint push notifications, for example a device identifier, email address, or phone number.</p><br>
     ///   - [`user_context_data(UserContextDataType)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::user_context_data) / [`set_user_context_data(Option<UserContextDataType>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_user_context_data):<br>required: **false**<br><p>Contextual data about your user session like the device fingerprint, IP address, or location. Amazon Cognito threat protection evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-viewing-threat-protection-app.html">Collecting data for threat protection in applications</a>.</p><br>
     ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
```

### `src/client/update_identity_provider.rs`

```diff
--- reference/src/client/update_identity_provider.rs
+++ generated/src/client/update_identity_provider.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The Id of the user pool where you want to update your IdP.</p><br>
     ///   - [`provider_name(impl Into<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::provider_name) / [`set_provider_name(Option<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::set_provider_name):<br>required: **true**<br><p>The name of the IdP that you want to update. You can pass the identity provider name in the <code>identity_provider</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated IdP.</p><br>
-    ///   - [`provider_details(impl Into<String>, impl Into<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::provider_details) / [`set_provider_details(Option<HashMap::<String, String>>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::set_provider_details):<br>required: **false**<br><p>The scopes, URLs, and identifiers for your external identity provider. The following examples describe the provider detail keys for each IdP type. These values and their schema are subject to change. Social IdP <code>authorize_scopes</code> values must match the values listed here.</p> <dl>  <dt>   OpenID Connect (OIDC)  </dt>  <dd>   <p>Amazon Cognito accepts the following elements when it can't discover endpoint URLs from <code>oidc_issuer</code>: <code>attributes_url</code>, <code>authorize_url</code>, <code>jwks_uri</code>, <code>token_url</code>.</p>   <p>Create or update request: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "attributes_url_add_attributes": "false", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p>  </dd>  <dt>   SAML  </dt>  <dd>   <p>Create or update request with Metadata URL: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256" }</code></p>   <p>Create or update request with Metadata file: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataFile": "\[metadata XML\]", "RequestSigningAlgorithm": "rsa-sha256" }</code></p>   <p>The value of <code>MetadataFile</code> must be the plaintext metadata document with all quote (") characters escaped by backslashes.</p>   <p>Describe response: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "ActiveEncryptionCertificate": "\[certificate\]", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256", "SLORedirectBindingURI": "https://auth.example.com/slo/saml", "SSORedirectBindingURI": "https://auth.example.com/sso/saml" }</code></p>  </dd>  <dt>   LoginWithAmazon  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "profile postal_code", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret"</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://api.amazon.com/user/profile", "attributes_url_add_attributes": "false", "authorize_scopes": "profile postal_code", "authorize_url": "https://www.amazon.com/ap/oa", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "POST", "token_url": "https://api.amazon.com/auth/o2/token" }</code></p>  </dd>  <dt>   Google  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email profile openid", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://people.googleapis.com/v1/people/me?personFields=", "attributes_url_add_attributes": "true", "authorize_scopes": "email profile openid", "authorize_url": "https://accounts.google.com/o/oauth2/v2/auth", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret", "oidc_issuer": "https://accounts.google.com", "token_request_method": "POST", "token_url": "https://www.googleapis.com/oauth2/v4/token" }</code></p>  </dd>  <dt>   SignInWithApple  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email name", "client_id": "com.example.cognito", "private_key": "1EXAMPLE", "key_id": "2EXAMPLE", "team_id": "3EXAMPLE" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "attributes_url_add_attributes": "false", "authorize_scopes": "email name", "authorize_url": "https://appleid.apple.com/auth/authorize", "client_id": "com.example.cognito", "key_id": "1EXAMPLE", "oidc_issuer": "https://appleid.apple.com", "team_id": "2EXAMPLE", "token_request_method": "POST", "token_url": "https://appleid.apple.com/auth/token" }</code></p>  </dd>  <dt>   Facebook  </dt>  <dd>   <p>Create or update request: <code>"ProviderDetails": { "api_version": "v17.0", "authorize_scopes": "public_profile, email", "client_id": "1example23456789", "client_secret": "provider-app-client-secret" }</code></p>   <p>Describe response: <code>"ProviderDetails": { "api_version": "v17.0", "attributes_url": "https://graph.facebook.com/v17.0/me?fields=", "attributes_url_add_attributes": "true", "authorize_scopes": "public_profile, email", "authorize_url": "https://www.facebook.com/v17.0/dialog/oauth", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "GET", "token_url": "https://graph.facebook.com/v17.0/oauth/access_token" }</code></p>  </dd> </dl><br>
+    ///   - [`provider_details(impl Into<String>, impl Into<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::provider_details) / [`set_provider_details(Option<HashMap::<String, String>>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::set_provider_details):<br>required: **false**<br><p>The scopes, URLs, and identifiers for your external identity provider. The following examples describe the provider detail keys for each IdP type. These values and their schema are subject to change. Social IdP <code>authorize_scopes</code> values must match the values listed here.</p> <dl> <dt>OpenID Connect (OIDC)</dt> <dd> <p>Amazon Cognito accepts the following elements when it can't discover endpoint URLs from <code>oidc_issuer</code>: <code>attributes_url</code>, <code>authorize_url</code>, <code>jwks_uri</code>, <code>token_url</code>.</p> <p>Create or update request: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_request_method": "GET", "attributes_url": "https://auth.example.com/userInfo", "attributes_url_add_attributes": "false", "authorize_scopes": "openid profile email", "authorize_url": "https://auth.example.com/authorize", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "jwks_uri": "https://auth.example.com/.well-known/jwks.json", "oidc_issuer": "https://auth.example.com", "token_url": "https://example.com/token" }</code></p></dd> <dt>SAML</dt> <dd> <p>Create or update request with Metadata URL: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256" }</code></p> <p>Create or update request with Metadata file: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataFile": "\[metadata XML\]", "RequestSigningAlgorithm": "rsa-sha256" }</code></p> <p>The value of <code>MetadataFile</code> must be the plaintext metadata document with all quote (") characters escaped by backslashes.</p> <p>Describe response: <code>"ProviderDetails": { "IDPInit": "true", "IDPSignout": "true", "EncryptedResponses" : "true", "ActiveEncryptionCertificate": "\[certificate\]", "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm": "rsa-sha256", "SLORedirectBindingURI": "https://auth.example.com/slo/saml", "SSORedirectBindingURI": "https://auth.example.com/sso/saml" }</code></p></dd> <dt>LoginWithAmazon</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "profile postal_code", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret"</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://api.amazon.com/user/profile", "attributes_url_add_attributes": "false", "authorize_scopes": "profile postal_code", "authorize_url": "https://www.amazon.com/ap/oa", "client_id": "amzn1.application-oa2-client.1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "POST", "token_url": "https://api.amazon.com/auth/o2/token" }</code></p></dd> <dt>Google</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email profile openid", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret" }</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_url": "https://people.googleapis.com/v1/people/me?personFields=", "attributes_url_add_attributes": "true", "authorize_scopes": "email profile openid", "authorize_url": "https://accounts.google.com/o/oauth2/v2/auth", "client_id": "1example23456789.apps.googleusercontent.com", "client_secret": "provider-app-client-secret", "oidc_issuer": "https://accounts.google.com", "token_request_method": "POST", "token_url": "https://www.googleapis.com/oauth2/v4/token" }</code></p></dd> <dt>SignInWithApple</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes": "email name", "client_id": "com.example.cognito", "private_key": "1EXAMPLE", "key_id": "2EXAMPLE", "team_id": "3EXAMPLE" }</code></p> <p>Describe response: <code>"ProviderDetails": { "attributes_url_add_attributes": "false", "authorize_scopes": "email name", "authorize_url": "https://appleid.apple.com/auth/authorize", "client_id": "com.example.cognito", "key_id": "1EXAMPLE", "oidc_issuer": "https://appleid.apple.com", "team_id": "2EXAMPLE", "token_request_method": "POST", "token_url": "https://appleid.apple.com/auth/token" }</code></p></dd> <dt>Facebook</dt> <dd> <p>Create or update request: <code>"ProviderDetails": { "api_version": "v17.0", "authorize_scopes": "public_profile, email", "client_id": "1example23456789", "client_secret": "provider-app-client-secret" }</code></p> <p>Describe response: <code>"ProviderDetails": { "api_version": "v17.0", "attributes_url": "https://graph.facebook.com/v17.0/me?fields=", "attributes_url_add_attributes": "true", "authorize_scopes": "public_profile, email", "authorize_url": "https://www.facebook.com/v17.0/dialog/oauth", "client_id": "1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": "GET", "token_url": "https://graph.facebook.com/v17.0/oauth/access_token" }</code></p></dd></dl><br>
     ///   - [`attribute_mapping(impl Into<String>, impl Into<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::attribute_mapping) / [`set_attribute_mapping(Option<HashMap::<String, String>>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::set_attribute_mapping):<br>required: **false**<br><p>A mapping of IdP attributes to standard and custom user pool attributes. Specify a user pool attribute as the key of the key-value pair, and the IdP attribute claim name as the value.</p><br>
     ///   - [`idp_identifiers(impl Into<String>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::idp_identifiers) / [`set_idp_identifiers(Option<Vec::<String>>)`](crate::operation::update_identity_provider::builders::UpdateIdentityProviderFluentBuilder::set_idp_identifiers):<br>required: **false**<br><p>An array of IdP identifiers, for example <code>"IdPIdentifiers": \[ "MyIdP", "MyIdP2" \]</code>. Identifiers are friendly names that you can pass in the <code>idp_identifier</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated IdP. Identifiers in a domain format also enable the use of <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managing-saml-idp-naming.html">email-address matching with SAML providers</a>.</p><br>
     /// - On success, responds with [`UpdateIdentityProviderOutput`](crate::operation::update_identity_provider::UpdateIdentityProviderOutput) with field(s):
```

### `src/client/update_user_pool_client.rs`

```diff
--- reference/src/client/update_user_pool_client.rs
+++ generated/src/client/update_user_pool_client.rs
@@ -14,10 +14,10 @@
     ///   - [`write_attributes(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::write_attributes) / [`set_write_attributes(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_write_attributes):<br>required: **false**<br><p>The list of user attributes that you want your app client to have write access to. After your user authenticates in your app, their access token authorizes them to set or modify their own attribute value for any attribute in this list.</p> <p>When you don't specify the <code>WriteAttributes</code> for your app client, your app can write the values of the Standard attributes of your user pool. When your user pool has write access to these default attributes, <code>WriteAttributes</code> doesn't return any information. Amazon Cognito only populates <code>WriteAttributes</code> in the API response if you have specified your own custom set of write attributes.</p> <p>If your app client allows users to sign in through an IdP, this array must include all attributes that you have mapped to IdP attributes. Amazon Cognito updates mapped attributes when users sign in to your application through an IdP. If your app client does not have write access to a mapped attribute, Amazon Cognito throws an error when it tries to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying IdP Attribute Mappings for Your user pool</a>.</p><br>
     ///   - [`explicit_auth_flows(ExplicitAuthFlowsType)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::explicit_auth_flows) / [`set_explicit_auth_flows(Option<Vec::<ExplicitAuthFlowsType>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_explicit_auth_flows):<br>required: **false**<br><p>The <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow-methods.html">authentication flows</a> that you want your user pool client to support. For each app client in your user pool, you can sign in your users with any combination of one or more flows, including with a user name and Secure Remote Password (SRP), a user name and password, or a custom authentication process that you define with Lambda functions.</p><note>  <p>If you don't specify a value for <code>ExplicitAuthFlows</code>, your app client supports <code>ALLOW_REFRESH_TOKEN_AUTH</code>, <code>ALLOW_USER_SRP_AUTH</code>, and <code>ALLOW_CUSTOM_AUTH</code>.</p> </note> <p>The values for authentication flow options include the following.</p> <ul>  <li>   <p><code>ALLOW_USER_AUTH</code>: Enable selection-based sign-in with <code>USER_AUTH</code>. This setting covers username-password, secure remote password (SRP), passwordless, and passkey authentication. This authentiation flow can do username-password and SRP authentication without other <code>ExplicitAuthFlows</code> permitting them. For example users can complete an SRP challenge through <code>USER_AUTH</code> without the flow <code>USER_SRP_AUTH</code> being active for the app client. This flow doesn't include <code>CUSTOM_AUTH</code>.</p>   <p>To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p></li>  <li>   <p><code>ALLOW_ADMIN_USER_PASSWORD_AUTH</code>: Enable admin based user password authentication flow <code>ADMIN_USER_PASSWORD_AUTH</code>. This setting replaces the <code>ADMIN_NO_SRP_AUTH</code> setting. With this authentication flow, your app passes a user name and password to Amazon Cognito in the request, instead of using the Secure Remote Password (SRP) protocol to securely transmit the password.</p></li>  <li>   <p><code>ALLOW_CUSTOM_AUTH</code>: Enable Lambda trigger based authentication.</p></li>  <li>   <p><code>ALLOW_USER_PASSWORD_AUTH</code>: Enable user password-based authentication. In this flow, Amazon Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p></li>  <li>   <p><code>ALLOW_USER_SRP_AUTH</code>: Enable SRP-based authentication.</p></li>  <li>   <p><code>ALLOW_REFRESH_TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p></li> </ul> <p>In some environments, you will see the values <code>ADMIN_NO_SRP_AUTH</code>, <code>CUSTOM_AUTH_FLOW_ONLY</code>, or <code>USER_PASSWORD_AUTH</code>. You can't assign these legacy <code>ExplicitAuthFlows</code> values to user pool clients at the same time as values that begin with <code>ALLOW_</code>, like <code>ALLOW_USER_SRP_AUTH</code>.</p><br>
     ///   - [`supported_identity_providers(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::supported_identity_providers) / [`set_supported_identity_providers(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_supported_identity_providers):<br>required: **false**<br><p>A list of provider names for the identity providers (IdPs) that are supported on this client. The following are supported: <code>COGNITO</code>, <code>Facebook</code>, <code>Google</code>, <code>SignInWithApple</code>, and <code>LoginWithAmazon</code>. You can also specify the names that you configured for the SAML and OIDC IdPs in your user pool, for example <code>MySAMLIdP</code> or <code>MyOIDCIdP</code>.</p> <p>This parameter sets the IdPs that <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html">managed login</a> will display on the login page for your app client. The removal of <code>COGNITO</code> from this list doesn't prevent authentication operations for local users with the user pools API in an Amazon Web Services SDK. The only way to prevent SDK-based authentication is to block access with a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-waf.html">WAF rule</a>.</p><br>
-    ///   - [`callback_urls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::callback_urls) / [`set_callback_urls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_callback_urls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
-    ///   - [`logout_urls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::logout_urls) / [`set_logout_urls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_logout_urls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
+    ///   - [`callback_ur_ls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::callback_ur_ls) / [`set_callback_ur_ls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_callback_ur_ls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
+    ///   - [`logout_ur_ls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::logout_ur_ls) / [`set_logout_ur_ls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_logout_ur_ls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
     ///   - [`default_redirect_uri(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::default_redirect_uri) / [`set_default_redirect_uri(Option<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_default_redirect_uri):<br>required: **false**<br><p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p><br>
-    ///   - [`allowed_o_auth_flows(OAuthFlowType)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::allowed_o_auth_flows) / [`set_allowed_o_auth_flows(Option<Vec::<OAuthFlowType>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_allowed_o_auth_flows):<br>required: **false**<br><p>The OAuth grant types that you want your app client to generate. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p> <dl>  <dt>   code  </dt>  <dd>   <p>Use a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the <code>/oauth2/token</code> endpoint.</p>  </dd>  <dt>   implicit  </dt>  <dd>   <p>Issue the access token (and, optionally, ID token, based on scopes) directly to your user.</p>  </dd>  <dt>   client_credentials  </dt>  <dd>   <p>Issue the access token from the <code>/oauth2/token</code> endpoint directly to a non-person user using a combination of the client ID and client secret.</p>  </dd> </dl><br>
+    ///   - [`allowed_o_auth_flows(OAuthFlowType)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::allowed_o_auth_flows) / [`set_allowed_o_auth_flows(Option<Vec::<OAuthFlowType>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_allowed_o_auth_flows):<br>required: **false**<br><p>The OAuth grant types that you want your app client to generate. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p> <dl> <dt>code</dt> <dd> <p>Use a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the <code>/oauth2/token</code> endpoint.</p></dd> <dt>implicit</dt> <dd> <p>Issue the access token (and, optionally, ID token, based on scopes) directly to your user.</p></dd> <dt>client_credentials</dt> <dd> <p>Issue the access token from the <code>/oauth2/token</code> endpoint directly to a non-person user using a combination of the client ID and client secret.</p></dd></dl><br>
     ///   - [`allowed_o_auth_scopes(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::allowed_o_auth_scopes) / [`set_allowed_o_auth_scopes(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_allowed_o_auth_scopes):<br>required: **false**<br><p>The OAuth, OpenID Connect (OIDC), and custom scopes that you want to permit your app client to authorize access with. Scopes govern access control to user pool self-service API operations, user data from the <code>userInfo</code> endpoint, and third-party APIs. Scope values include <code>phone</code>, <code>email</code>, <code>openid</code>, and <code>profile</code>. The <code>aws.cognito.signin.user.admin</code> scope authorizes user self-service operations. Custom scopes with resource servers authorize access to external APIs.</p><br>
     ///   - [`allowed_o_auth_flows_user_pool_client(bool)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::allowed_o_auth_flows_user_pool_client) / [`set_allowed_o_auth_flows_user_pool_client(Option<bool>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_allowed_o_auth_flows_user_pool_client):<br>required: **false**<br><p>Set to <code>true</code> to use OAuth 2.0 authorization server features in your app client.</p> <p>This parameter must have a value of <code>true</code> before you can configure the following features in your app client.</p> <ul>  <li>   <p><code>CallBackURLs</code>: Callback URLs.</p></li>  <li>   <p><code>LogoutURLs</code>: Sign-out redirect URLs.</p></li>  <li>   <p><code>AllowedOAuthScopes</code>: OAuth 2.0 scopes.</p></li>  <li>   <p><code>AllowedOAuthFlows</code>: Support for authorization code, implicit, and client credentials OAuth 2.0 grants.</p></li> </ul> <p>To use authorization server features, configure one of these features in the Amazon Cognito console or set <code>AllowedOAuthFlowsUserPoolClient</code> to <code>true</code> in a <code>CreateUserPoolClient</code> or <code>UpdateUserPoolClient</code> API request. If you don't set a value for <code>AllowedOAuthFlowsUserPoolClient</code> in a request with the CLI or SDKs, it defaults to <code>false</code>. When <code>false</code>, only SDK-based API sign-in is permitted.</p><br>
     ///   - [`analytics_configuration(AnalyticsConfigurationType)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::analytics_configuration) / [`set_analytics_configuration(Option<AnalyticsConfigurationType>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_analytics_configuration):<br>required: **false**<br><p>The user pool analytics configuration for collecting metrics and sending them to your Amazon Pinpoint campaign.</p> <p>In Amazon Web Services Regions where Amazon Pinpoint isn't available, user pools might not have access to analytics or might be configurable with campaigns in the US East (N. Virginia) Region. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-pinpoint-integration.html">Using Amazon Pinpoint analytics</a>.</p><br>
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -145,7 +145,7 @@
     /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
     /// [`SigningName`](aws_types::SigningName) during operation construction
     pub fn signing_name(&self) -> &'static str {
-        "cognito-idp"
+        "cognitoidentityprovider"
     }
     /// Returns the AWS region, if it was provided.
     pub fn region(&self) -> ::std::option::Option<&super::config::Region> {
@@ -1385,7 +1385,7 @@
                 .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
         }
         layer.store_put(super::meta::API_METADATA.clone());
-        layer.store_put(::aws_types::SigningName::from_static("cognito-idp"));
+        layer.store_put(::aws_types::SigningName::from_static("cognitoidentityprovider"));
         layer
             .load::<::aws_types::region::Region>()
             .cloned()
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -214,9 +214,9 @@

 mod lens;

+mod json_errors;
+
 mod serde_util;

-mod json_errors;
-
 #[doc(inline)]
 pub use client::Client;
```

### `src/operation/admin_create_user/_admin_create_user_input.rs`

```diff
--- reference/src/operation/admin_create_user/_admin_create_user_input.rs
+++ generated/src/operation/admin_create_user/_admin_create_user_input.rs
@@ -503,7 +503,7 @@
             user_attributes: self.user_attributes,
             validation_data: self.validation_data,
             temporary_password: self.temporary_password,
-            force_alias_creation: self.force_alias_creation,
+            force_alias_creation: self.force_alias_creation.unwrap_or_default(),
             message_action: self.message_action,
             desired_delivery_mediums: self.desired_delivery_mediums,
             client_metadata: self.client_metadata,
```

### `src/operation/admin_set_user_mfa_preference/builders.rs`

```diff
--- reference/src/operation/admin_set_user_mfa_preference/builders.rs
+++ generated/src/operation/admin_set_user_mfa_preference/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
+            super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `AdminSetUserMFAPreference`.
+/// Fluent builder constructing a request to `AdminSetUserMfaPreference`.
 ///
 /// <p>Sets the user's multi-factor authentication (MFA) preference, including which MFA options are activated, and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are activated. If multiple options are activated and no preference is set, a challenge to choose an MFA option will be returned during sign-in.</p><note>
 /// <p>Amazon Cognito evaluates Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you must use IAM credentials to authorize requests, and you must grant yourself the corresponding IAM permission in a policy.</p>
@@ -33,7 +33,7 @@
 /// </ul>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct AdminSetUserMFAPreferenceFluentBuilder {
+pub struct AdminSetUserMfaPreferenceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::admin_set_user_mfa_preference::builders::AdminSetUserMfaPreferenceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -41,8 +41,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
-        super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
-    > for AdminSetUserMFAPreferenceFluentBuilder
+        super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
+    > for AdminSetUserMfaPreferenceFluentBuilder
 {
     fn send(
         self,
@@ -50,14 +50,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
-            super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
+            super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl AdminSetUserMFAPreferenceFluentBuilder {
-    /// Creates a new `AdminSetUserMFAPreferenceFluentBuilder`.
+impl AdminSetUserMfaPreferenceFluentBuilder {
+    /// Creates a new `AdminSetUserMfaPreferenceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -65,7 +65,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the AdminSetUserMFAPreference as a reference.
+    /// Access the AdminSetUserMfaPreference as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::admin_set_user_mfa_preference::builders::AdminSetUserMfaPreferenceInputBuilder {
         &self.inner
     }
@@ -82,7 +82,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
+            super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -90,12 +90,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreference::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreference::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreference::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreference::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -103,7 +103,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
-        super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
+        super::super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/admin_set_user_mfa_preference.rs`

```diff
--- reference/src/operation/admin_set_user_mfa_preference.rs
+++ generated/src/operation/admin_set_user_mfa_preference.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `AdminSetUserMFAPreference`.
+/// Orchestration and serialization glue logic for `AdminSetUserMfaPreference`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct AdminSetUserMFAPreference;
-impl AdminSetUserMFAPreference {
-    /// Creates a new `AdminSetUserMFAPreference`
+pub struct AdminSetUserMfaPreference;
+impl AdminSetUserMfaPreference {
+    /// Creates a new `AdminSetUserMfaPreference`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AdminSetUserMFAPreference {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AdminSetUserMfaPreference {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AdminSetUserMFAPreference");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            AdminSetUserMFAPreferenceRequestSerializer,
+            AdminSetUserMfaPreferenceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AdminSetUserMFAPreferenceResponseDeserializer,
+            AdminSetUserMfaPreferenceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -133,13 +133,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AdminSetUserMFAPreference")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                AdminSetUserMFAPreferenceTelemetryInputCaptureInterceptor,
+                AdminSetUserMfaPreferenceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                AdminSetUserMFAPreferenceEndpointParamsInterceptor,
+                AdminSetUserMfaPreferenceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
@@ -156,12 +156,12 @@
 }

 #[derive(Debug)]
-struct AdminSetUserMFAPreferenceTelemetryInputCaptureInterceptor;
+struct AdminSetUserMfaPreferenceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AdminSetUserMFAPreferenceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AdminSetUserMfaPreferenceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "AdminSetUserMFAPreferenceTelemetryInputCaptureInterceptor"
+        "AdminSetUserMfaPreferenceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -275,12 +275,12 @@
     }
 }
 #[derive(Debug)]
-struct AdminSetUserMFAPreferenceEndpointParamsInterceptor;
+struct AdminSetUserMfaPreferenceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AdminSetUserMFAPreferenceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AdminSetUserMfaPreferenceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "AdminSetUserMFAPreferenceEndpointParamsInterceptor"
+        "AdminSetUserMfaPreferenceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/admin_set_user_password/_admin_set_user_password_input.rs`

```diff
--- reference/src/operation/admin_set_user_password/_admin_set_user_password_input.rs
+++ generated/src/operation/admin_set_user_password/_admin_set_user_password_input.rs
@@ -125,7 +125,7 @@
             user_pool_id: self.user_pool_id,
             username: self.username,
             password: self.password,
-            permanent: self.permanent,
+            permanent: self.permanent.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/associate_software_token.rs`

```diff
--- reference/src/operation/associate_software_token.rs
+++ generated/src/operation/associate_software_token.rs
@@ -113,6 +113,16 @@
             "AssociateSoftwareToken",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/change_password.rs`

```diff
--- reference/src/operation/change_password.rs
+++ generated/src/operation/change_password.rs
@@ -112,6 +112,16 @@
             "ChangePassword",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/complete_web_authn_registration/_complete_web_authn_registration_input.rs`

```diff
--- reference/src/operation/complete_web_authn_registration/_complete_web_authn_registration_input.rs
+++ generated/src/operation/complete_web_authn_registration/_complete_web_authn_registration_input.rs
@@ -6,7 +6,7 @@
     /// <p>A valid access token that Amazon Cognito issued to the currently signed-in user. Must include a scope claim for <code>aws.cognito.signin.user.admin</code>.</p>
     pub access_token: ::std::option::Option<::std::string::String>,
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub credential: ::std::option::Option<::aws_smithy_types::Document>,
+    pub credential: ::std::option::Option<::std::string::String>,
 }
 impl CompleteWebAuthnRegistrationInput {
     /// <p>A valid access token that Amazon Cognito issued to the currently signed-in user. Must include a scope claim for <code>aws.cognito.signin.user.admin</code>.</p>
@@ -14,7 +14,7 @@
         self.access_token.as_deref()
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub fn credential(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
+    pub fn credential(&self) -> ::std::option::Option<&::std::string::String> {
         self.credential.as_ref()
     }
 }
@@ -38,7 +38,7 @@
 #[non_exhaustive]
 pub struct CompleteWebAuthnRegistrationInputBuilder {
     pub(crate) access_token: ::std::option::Option<::std::string::String>,
-    pub(crate) credential: ::std::option::Option<::aws_smithy_types::Document>,
+    pub(crate) credential: ::std::option::Option<::std::string::String>,
 }
 impl CompleteWebAuthnRegistrationInputBuilder {
     /// <p>A valid access token that Amazon Cognito issued to the currently signed-in user. Must include a scope claim for <code>aws.cognito.signin.user.admin</code>.</p>
@@ -58,17 +58,17 @@
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
     /// This field is required.
-    pub fn credential(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn credential(mut self, input: ::std::string::String) -> Self {
         self.credential = ::std::option::Option::Some(input);
         self
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub fn set_credential(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_credential(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.credential = input;
         self
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub fn get_credential(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_credential(&self) -> &::std::option::Option<::std::string::String> {
         &self.credential
     }
     /// Consumes the builder and constructs a [`CompleteWebAuthnRegistrationInput`](crate::operation::complete_web_authn_registration::CompleteWebAuthnRegistrationInput).
```

### `src/operation/complete_web_authn_registration/builders.rs`

```diff
--- reference/src/operation/complete_web_authn_registration/builders.rs
+++ generated/src/operation/complete_web_authn_registration/builders.rs
@@ -124,17 +124,17 @@
         self.inner.get_access_token()
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub fn credential(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn credential(mut self, input: ::std::string::String) -> Self {
         self.inner = self.inner.credential(input);
         self
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub fn set_credential(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_credential(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.inner = self.inner.set_credential(input);
         self
     }
     /// <p>A <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-registrationresponsejson">RegistrationResponseJSON</a> public-key credential response from the user's passkey provider.</p>
-    pub fn get_credential(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_credential(&self) -> &::std::option::Option<::std::string::String> {
         self.inner.get_credential()
     }
 }
```

### `src/operation/complete_web_authn_registration.rs`

```diff
--- reference/src/operation/complete_web_authn_registration.rs
+++ generated/src/operation/complete_web_authn_registration.rs
@@ -112,6 +112,16 @@
             "CompleteWebAuthnRegistration",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/confirm_device.rs`

```diff
--- reference/src/operation/confirm_device.rs
+++ generated/src/operation/confirm_device.rs
@@ -112,6 +112,16 @@
             "ConfirmDevice",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/confirm_forgot_password.rs`

```diff
--- reference/src/operation/confirm_forgot_password.rs
+++ generated/src/operation/confirm_forgot_password.rs
@@ -112,6 +112,16 @@
             "ConfirmForgotPassword",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/confirm_sign_up/_confirm_sign_up_input.rs`

```diff
--- reference/src/operation/confirm_sign_up/_confirm_sign_up_input.rs
+++ generated/src/operation/confirm_sign_up/_confirm_sign_up_input.rs
@@ -329,7 +329,7 @@
             secret_hash: self.secret_hash,
             username: self.username,
             confirmation_code: self.confirmation_code,
-            force_alias_creation: self.force_alias_creation,
+            force_alias_creation: self.force_alias_creation.unwrap_or_default(),
             analytics_metadata: self.analytics_metadata,
             user_context_data: self.user_context_data,
             client_metadata: self.client_metadata,
```

### `src/operation/confirm_sign_up.rs`

```diff
--- reference/src/operation/confirm_sign_up.rs
+++ generated/src/operation/confirm_sign_up.rs
@@ -113,6 +113,16 @@
             "ConfirmSignUp",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/create_managed_login_branding/_create_managed_login_branding_input.rs`

```diff
--- reference/src/operation/create_managed_login_branding/_create_managed_login_branding_input.rs
+++ generated/src/operation/create_managed_login_branding/_create_managed_login_branding_input.rs
@@ -22,7 +22,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub settings: ::std::option::Option<::aws_smithy_types::Document>,
+    pub settings: ::std::option::Option<::std::string::String>,
     /// <p>An array of image files that you want to apply to functions like backgrounds, logos, and icons. Each object must also indicate whether it is for dark mode, light mode, or browser-adaptive mode.</p>
     pub assets: ::std::option::Option<::std::vec::Vec<super::super::super::types::AssetType>>,
 }
@@ -52,7 +52,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
+    pub fn settings(&self) -> ::std::option::Option<&::std::string::String> {
         self.settings.as_ref()
     }
     /// <p>An array of image files that you want to apply to functions like backgrounds, logos, and icons. Each object must also indicate whether it is for dark mode, light mode, or browser-adaptive mode.</p>
@@ -87,7 +87,7 @@
     pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
     pub(crate) client_id: ::std::option::Option<::std::string::String>,
     pub(crate) use_cognito_provided_values: ::std::option::Option<bool>,
-    pub(crate) settings: ::std::option::Option<::aws_smithy_types::Document>,
+    pub(crate) settings: ::std::option::Option<::std::string::String>,
     pub(crate) assets: ::std::option::Option<::std::vec::Vec<super::super::super::types::AssetType>>,
 }
 impl CreateManagedLoginBrandingInputBuilder {
@@ -150,7 +150,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn settings(mut self, input: ::std::string::String) -> Self {
         self.settings = ::std::option::Option::Some(input);
         self
     }
@@ -166,7 +166,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn set_settings(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_settings(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.settings = input;
         self
     }
@@ -182,7 +182,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn get_settings(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_settings(&self) -> &::std::option::Option<::std::string::String> {
         &self.settings
     }
     /// Appends an item to `assets`.
@@ -215,7 +215,7 @@
         ::std::result::Result::Ok(super::super::super::operation::create_managed_login_branding::CreateManagedLoginBrandingInput {
             user_pool_id: self.user_pool_id,
             client_id: self.client_id,
-            use_cognito_provided_values: self.use_cognito_provided_values,
+            use_cognito_provided_values: self.use_cognito_provided_values.unwrap_or_default(),
             settings: self.settings,
             assets: self.assets,
         })
```

### `src/operation/create_managed_login_branding/builders.rs`

```diff
--- reference/src/operation/create_managed_login_branding/builders.rs
+++ generated/src/operation/create_managed_login_branding/builders.rs
@@ -176,7 +176,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn settings(mut self, input: ::std::string::String) -> Self {
         self.inner = self.inner.settings(input);
         self
     }
@@ -192,7 +192,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn set_settings(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_settings(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.inner = self.inner.set_settings(input);
         self
     }
@@ -208,7 +208,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn get_settings(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_settings(&self) -> &::std::option::Option<::std::string::String> {
         self.inner.get_settings()
     }
     ///
```

### `src/operation/create_user_pool_client/_create_user_pool_client_input.rs`

```diff
--- reference/src/operation/create_user_pool_client/_create_user_pool_client_input.rs
+++ generated/src/operation/create_user_pool_client/_create_user_pool_client_input.rs
@@ -73,9 +73,9 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub callback_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub callback_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub logout_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub logout_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub default_redirect_uri: ::std::option::Option<::std::string::String>,
     /// <p>The OAuth grant types that you want your app client to generate for clients in managed login authentication. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p>
@@ -235,15 +235,15 @@
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
     ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.callback_urls.is_none()`.
-    pub fn callback_urls(&self) -> &[::std::string::String] {
-        self.callback_urls.as_deref().unwrap_or_default()
+    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.callback_ur_ls.is_none()`.
+    pub fn callback_ur_ls(&self) -> &[::std::string::String] {
+        self.callback_ur_ls.as_deref().unwrap_or_default()
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
     ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.logout_urls.is_none()`.
-    pub fn logout_urls(&self) -> &[::std::string::String] {
-        self.logout_urls.as_deref().unwrap_or_default()
+    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.logout_ur_ls.is_none()`.
+    pub fn logout_ur_ls(&self) -> &[::std::string::String] {
+        self.logout_ur_ls.as_deref().unwrap_or_default()
     }
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub fn default_redirect_uri(&self) -> ::std::option::Option<&str> {
@@ -340,8 +340,8 @@
         formatter.field("write_attributes", &self.write_attributes);
         formatter.field("explicit_auth_flows", &self.explicit_auth_flows);
         formatter.field("supported_identity_providers", &self.supported_identity_providers);
-        formatter.field("callback_urls", &self.callback_urls);
-        formatter.field("logout_urls", &self.logout_urls);
+        formatter.field("callback_ur_ls", &self.callback_ur_ls);
+        formatter.field("logout_ur_ls", &self.logout_ur_ls);
         formatter.field("default_redirect_uri", &self.default_redirect_uri);
         formatter.field("allowed_o_auth_flows", &self.allowed_o_auth_flows);
         formatter.field("allowed_o_auth_scopes", &self.allowed_o_auth_scopes);
@@ -381,8 +381,8 @@
     pub(crate) write_attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) explicit_auth_flows: ::std::option::Option<::std::vec::Vec<super::super::super::types::ExplicitAuthFlowsType>>,
     pub(crate) supported_identity_providers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) callback_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) logout_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub(crate) callback_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub(crate) logout_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) default_redirect_uri: ::std::option::Option<::std::string::String>,
     pub(crate) allowed_o_auth_flows: ::std::option::Option<::std::vec::Vec<super::super::super::types::OAuthFlowType>>,
     pub(crate) allowed_o_auth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -685,9 +685,9 @@
     pub fn get_supported_identity_providers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
         &self.supported_identity_providers
     }
-    /// Appends an item to `callback_urls`.
+    /// Appends an item to `callback_ur_ls`.
     ///
-    /// To override the contents of this collection use [`set_callback_urls`](Self::set_callback_urls).
+    /// To override the contents of this collection use [`set_callback_ur_ls`](Self::set_callback_ur_ls).
     ///
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
     /// <p>A redirect URI must meet the following requirements:</p>
@@ -702,10 +702,10 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn callback_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        let mut v = self.callback_urls.unwrap_or_default();
+    pub fn callback_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        let mut v = self.callback_ur_ls.unwrap_or_default();
         v.push(input.into());
-        self.callback_urls = ::std::option::Option::Some(v);
+        self.callback_ur_ls = ::std::option::Option::Some(v);
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -721,8 +721,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn set_callback_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.callback_urls = input;
+    pub fn set_callback_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.callback_ur_ls = input;
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -738,28 +738,28 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn get_callback_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        &self.callback_urls
+    pub fn get_callback_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        &self.callback_ur_ls
     }
-    /// Appends an item to `logout_urls`.
+    /// Appends an item to `logout_ur_ls`.
     ///
-    /// To override the contents of this collection use [`set_logout_urls`](Self::set_logout_urls).
+    /// To override the contents of this collection use [`set_logout_ur_ls`](Self::set_logout_ur_ls).
     ///
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn logout_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        let mut v = self.logout_urls.unwrap_or_default();
+    pub fn logout_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        let mut v = self.logout_ur_ls.unwrap_or_default();
         v.push(input.into());
-        self.logout_urls = ::std::option::Option::Some(v);
+        self.logout_ur_ls = ::std::option::Option::Some(v);
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn set_logout_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.logout_urls = input;
+    pub fn set_logout_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.logout_ur_ls = input;
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn get_logout_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        &self.logout_urls
+    pub fn get_logout_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        &self.logout_ur_ls
     }
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub fn default_redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
@@ -1026,9 +1026,9 @@
         ::std::result::Result::Ok(super::super::super::operation::create_user_pool_client::CreateUserPoolClientInput {
             user_pool_id: self.user_pool_id,
             client_name: self.client_name,
-            generate_secret: self.generate_secret,
+            generate_secret: self.generate_secret.unwrap_or_default(),
             client_secret: self.client_secret,
-            refresh_token_validity: self.refresh_token_validity,
+            refresh_token_validity: self.refresh_token_validity.unwrap_or_default(),
             access_token_validity: self.access_token_validity,
             id_token_validity: self.id_token_validity,
             token_validity_units: self.token_validity_units,
@@ -1036,12 +1036,12 @@
             write_attributes: self.write_attributes,
             explicit_auth_flows: self.explicit_auth_flows,
             supported_identity_providers: self.supported_identity_providers,
-            callback_urls: self.callback_urls,
-            logout_urls: self.logout_urls,
+            callback_ur_ls: self.callback_ur_ls,
+            logout_ur_ls: self.logout_ur_ls,
             default_redirect_uri: self.default_redirect_uri,
             allowed_o_auth_flows: self.allowed_o_auth_flows,
             allowed_o_auth_scopes: self.allowed_o_auth_scopes,
-            allowed_o_auth_flows_user_pool_client: self.allowed_o_auth_flows_user_pool_client,
+            allowed_o_auth_flows_user_pool_client: self.allowed_o_auth_flows_user_pool_client.unwrap_or_default(),
             analytics_configuration: self.analytics_configuration,
             prevent_user_existence_errors: self.prevent_user_existence_errors,
             enable_token_revocation: self.enable_token_revocation,
@@ -1066,8 +1066,8 @@
         formatter.field("write_attributes", &self.write_attributes);
         formatter.field("explicit_auth_flows", &self.explicit_auth_flows);
         formatter.field("supported_identity_providers", &self.supported_identity_providers);
-        formatter.field("callback_urls", &self.callback_urls);
-        formatter.field("logout_urls", &self.logout_urls);
+        formatter.field("callback_ur_ls", &self.callback_ur_ls);
+        formatter.field("logout_ur_ls", &self.logout_ur_ls);
         formatter.field("default_redirect_uri", &self.default_redirect_uri);
         formatter.field("allowed_o_auth_flows", &self.allowed_o_auth_flows);
         formatter.field("allowed_o_auth_scopes", &self.allowed_o_auth_scopes);
```

### `src/operation/create_user_pool_client/builders.rs`

```diff
--- reference/src/operation/create_user_pool_client/builders.rs
+++ generated/src/operation/create_user_pool_client/builders.rs
@@ -407,7 +407,7 @@
     ///
     /// Appends an item to `CallbackURLs`.
     ///
-    /// To override the contents of this collection use [`set_callback_urls`](Self::set_callback_urls).
+    /// To override the contents of this collection use [`set_callback_ur_ls`](Self::set_callback_ur_ls).
     ///
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
     /// <p>A redirect URI must meet the following requirements:</p>
@@ -422,8 +422,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn callback_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        self.inner = self.inner.callback_urls(input.into());
+    pub fn callback_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        self.inner = self.inner.callback_ur_ls(input.into());
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -439,8 +439,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn set_callback_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.inner = self.inner.set_callback_urls(input);
+    pub fn set_callback_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.inner = self.inner.set_callback_ur_ls(input);
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -456,27 +456,27 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn get_callback_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        self.inner.get_callback_urls()
+    pub fn get_callback_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        self.inner.get_callback_ur_ls()
     }
     ///
     /// Appends an item to `LogoutURLs`.
     ///
-    /// To override the contents of this collection use [`set_logout_urls`](Self::set_logout_urls).
+    /// To override the contents of this collection use [`set_logout_ur_ls`](Self::set_logout_ur_ls).
     ///
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn logout_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        self.inner = self.inner.logout_urls(input.into());
+    pub fn logout_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        self.inner = self.inner.logout_ur_ls(input.into());
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn set_logout_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.inner = self.inner.set_logout_urls(input);
+    pub fn set_logout_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.inner = self.inner.set_logout_ur_ls(input);
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn get_logout_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        self.inner.get_logout_urls()
+    pub fn get_logout_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        self.inner.get_logout_ur_ls()
     }
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub fn default_redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
```

### `src/operation/delete_user.rs`

```diff
--- reference/src/operation/delete_user.rs
+++ generated/src/operation/delete_user.rs
@@ -112,6 +112,16 @@
             "DeleteUser",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/delete_user_attributes.rs`

```diff
--- reference/src/operation/delete_user_attributes.rs
+++ generated/src/operation/delete_user_attributes.rs
@@ -112,6 +112,16 @@
             "DeleteUserAttributes",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/delete_web_authn_credential.rs`

```diff
--- reference/src/operation/delete_web_authn_credential.rs
+++ generated/src/operation/delete_web_authn_credential.rs
@@ -112,6 +112,16 @@
             "DeleteWebAuthnCredential",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/describe_managed_login_branding/_describe_managed_login_branding_input.rs`

```diff
--- reference/src/operation/describe_managed_login_branding/_describe_managed_login_branding_input.rs
+++ generated/src/operation/describe_managed_login_branding/_describe_managed_login_branding_input.rs
@@ -94,7 +94,7 @@
         ::std::result::Result::Ok(super::super::super::operation::describe_managed_login_branding::DescribeManagedLoginBrandingInput {
             user_pool_id: self.user_pool_id,
             managed_login_branding_id: self.managed_login_branding_id,
-            return_merged_resources: self.return_merged_resources,
+            return_merged_resources: self.return_merged_resources.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/describe_managed_login_branding_by_client/_describe_managed_login_branding_by_client_input.rs`

```diff
--- reference/src/operation/describe_managed_login_branding_by_client/_describe_managed_login_branding_by_client_input.rs
+++ generated/src/operation/describe_managed_login_branding_by_client/_describe_managed_login_branding_by_client_input.rs
@@ -104,7 +104,7 @@
             super::super::super::operation::describe_managed_login_branding_by_client::DescribeManagedLoginBrandingByClientInput {
                 user_pool_id: self.user_pool_id,
                 client_id: self.client_id,
-                return_merged_resources: self.return_merged_resources,
+                return_merged_resources: self.return_merged_resources.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/forget_device.rs`

```diff
--- reference/src/operation/forget_device.rs
+++ generated/src/operation/forget_device.rs
@@ -112,6 +112,16 @@
             "ForgetDevice",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/forgot_password.rs`

```diff
--- reference/src/operation/forgot_password.rs
+++ generated/src/operation/forgot_password.rs
@@ -112,6 +112,16 @@
             "ForgotPassword",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/get_csv_header/builders.rs`

```diff
--- reference/src/operation/get_csv_header/builders.rs
+++ generated/src/operation/get_csv_header/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_csv_header::GetCsvHeaderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_csv_header::GetCSVHeaderError,
+            super::super::super::operation::get_csv_header::GetCsvHeaderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetCSVHeader`.
+/// Fluent builder constructing a request to `GetCsvHeader`.
 ///
 /// <p>Given a user pool ID, generates a comma-separated value (CSV) list populated with available user attributes in the user pool. This list is the header for the CSV file that determines the users in a user import job. Save the content of <code>CSVHeader</code> in the response as a <code>.csv</code> file and populate it with the usernames and attributes of users that you want to import. For more information about CSV user import, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-using-import-tool.html">Importing users from a CSV file</a>.</p><note>
 /// <p>Amazon Cognito evaluates Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you must use IAM credentials to authorize requests, and you must grant yourself the corresponding IAM permission in a policy.</p>
@@ -33,7 +33,7 @@
 /// </ul>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetCSVHeaderFluentBuilder {
+pub struct GetCsvHeaderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_csv_header::builders::GetCsvHeaderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -41,8 +41,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_csv_header::GetCsvHeaderOutput,
-        super::super::super::operation::get_csv_header::GetCSVHeaderError,
-    > for GetCSVHeaderFluentBuilder
+        super::super::super::operation::get_csv_header::GetCsvHeaderError,
+    > for GetCsvHeaderFluentBuilder
 {
     fn send(
         self,
@@ -50,14 +50,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_csv_header::GetCsvHeaderOutput,
-            super::super::super::operation::get_csv_header::GetCSVHeaderError,
+            super::super::super::operation::get_csv_header::GetCsvHeaderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetCSVHeaderFluentBuilder {
-    /// Creates a new `GetCSVHeaderFluentBuilder`.
+impl GetCsvHeaderFluentBuilder {
+    /// Creates a new `GetCsvHeaderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -65,7 +65,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetCSVHeader as a reference.
+    /// Access the GetCsvHeader as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_csv_header::builders::GetCsvHeaderInputBuilder {
         &self.inner
     }
@@ -82,7 +82,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_csv_header::GetCsvHeaderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_csv_header::GetCSVHeaderError,
+            super::super::super::operation::get_csv_header::GetCsvHeaderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -90,12 +90,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_csv_header::GetCSVHeader::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_csv_header::GetCsvHeader::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_csv_header::GetCSVHeader::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_csv_header::GetCsvHeader::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -103,7 +103,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_csv_header::GetCsvHeaderOutput,
-        super::super::super::operation::get_csv_header::GetCSVHeaderError,
+        super::super::super::operation::get_csv_header::GetCsvHeaderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_csv_header.rs`

```diff
--- reference/src/operation/get_csv_header.rs
+++ generated/src/operation/get_csv_header.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetCSVHeader`.
+/// Orchestration and serialization glue logic for `GetCsvHeader`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetCSVHeader;
-impl GetCSVHeader {
-    /// Creates a new `GetCSVHeader`
+pub struct GetCsvHeader;
+impl GetCsvHeader {
+    /// Creates a new `GetCsvHeader`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetCSVHeader {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetCsvHeader {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetCSVHeader");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetCSVHeaderRequestSerializer,
+            GetCsvHeaderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetCSVHeaderResponseDeserializer,
+            GetCsvHeaderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -133,13 +133,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCSVHeader")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetCSVHeaderTelemetryInputCaptureInterceptor,
+                GetCsvHeaderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetCSVHeaderEndpointParamsInterceptor,
+                GetCsvHeaderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_csv_header::GetCSVHeaderError,
@@ -156,12 +156,12 @@
 }

 #[derive(Debug)]
-struct GetCSVHeaderTelemetryInputCaptureInterceptor;
+struct GetCsvHeaderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetCSVHeaderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetCsvHeaderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "GetCSVHeaderTelemetryInputCaptureInterceptor"
+        "GetCsvHeaderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -273,12 +273,12 @@
     }
 }
 #[derive(Debug)]
-struct GetCSVHeaderEndpointParamsInterceptor;
+struct GetCsvHeaderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetCSVHeaderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetCsvHeaderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetCSVHeaderEndpointParamsInterceptor"
+        "GetCsvHeaderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_device.rs`

```diff
--- reference/src/operation/get_device.rs
+++ generated/src/operation/get_device.rs
@@ -113,6 +113,16 @@
             "GetDevice",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/get_tokens_from_refresh_token.rs`

```diff
--- reference/src/operation/get_tokens_from_refresh_token.rs
+++ generated/src/operation/get_tokens_from_refresh_token.rs
@@ -113,6 +113,16 @@
             "GetTokensFromRefreshToken",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/get_ui_customization/builders.rs`

```diff
--- reference/src/operation/get_ui_customization/builders.rs
+++ generated/src/operation/get_ui_customization/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_ui_customization::GetUiCustomizationOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_ui_customization::GetUICustomizationError,
+            super::super::super::operation::get_ui_customization::GetUiCustomizationError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetUICustomization`.
+/// Fluent builder constructing a request to `GetUiCustomization`.
 ///
 /// <p>Given a user pool ID or app client, returns information about classic hosted UI branding that you applied, if any. Returns user-pool level branding information if no app client branding is applied, or if you don't specify an app client ID. Returns an empty object if you haven't applied hosted UI branding to either the client or the user pool. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/hosted-ui-classic-branding.html">Hosted UI (classic) branding</a>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetUICustomizationFluentBuilder {
+pub struct GetUiCustomizationFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_ui_customization::builders::GetUiCustomizationInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_ui_customization::GetUiCustomizationOutput,
-        super::super::super::operation::get_ui_customization::GetUICustomizationError,
-    > for GetUICustomizationFluentBuilder
+        super::super::super::operation::get_ui_customization::GetUiCustomizationError,
+    > for GetUiCustomizationFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_ui_customization::GetUiCustomizationOutput,
-            super::super::super::operation::get_ui_customization::GetUICustomizationError,
+            super::super::super::operation::get_ui_customization::GetUiCustomizationError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetUICustomizationFluentBuilder {
-    /// Creates a new `GetUICustomizationFluentBuilder`.
+impl GetUiCustomizationFluentBuilder {
+    /// Creates a new `GetUiCustomizationFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetUICustomization as a reference.
+    /// Access the GetUiCustomization as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_ui_customization::builders::GetUiCustomizationInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_ui_customization::GetUiCustomizationOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_ui_customization::GetUICustomizationError,
+            super::super::super::operation::get_ui_customization::GetUiCustomizationError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_ui_customization::GetUICustomization::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_ui_customization::GetUiCustomization::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_ui_customization::GetUICustomization::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_ui_customization::GetUiCustomization::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_ui_customization::GetUiCustomizationOutput,
-        super::super::super::operation::get_ui_customization::GetUICustomizationError,
+        super::super::super::operation::get_ui_customization::GetUiCustomizationError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_ui_customization.rs`

```diff
--- reference/src/operation/get_ui_customization.rs
+++ generated/src/operation/get_ui_customization.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetUICustomization`.
+/// Orchestration and serialization glue logic for `GetUiCustomization`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetUICustomization;
-impl GetUICustomization {
-    /// Creates a new `GetUICustomization`
+pub struct GetUiCustomization;
+impl GetUiCustomization {
+    /// Creates a new `GetUiCustomization`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetUICustomization {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetUiCustomization {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetUICustomization");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetUICustomizationRequestSerializer,
+            GetUiCustomizationRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetUICustomizationResponseDeserializer,
+            GetUiCustomizationResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -134,13 +134,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetUICustomization")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetUICustomizationTelemetryInputCaptureInterceptor,
+                GetUiCustomizationTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetUICustomizationEndpointParamsInterceptor,
+                GetUiCustomizationEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_ui_customization::GetUICustomizationError,
@@ -157,12 +157,12 @@
 }

 #[derive(Debug)]
-struct GetUICustomizationTelemetryInputCaptureInterceptor;
+struct GetUiCustomizationTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetUICustomizationTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetUiCustomizationTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "GetUICustomizationTelemetryInputCaptureInterceptor"
+        "GetUiCustomizationTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -275,12 +275,12 @@
     }
 }
 #[derive(Debug)]
-struct GetUICustomizationEndpointParamsInterceptor;
+struct GetUiCustomizationEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetUICustomizationEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetUiCustomizationEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetUICustomizationEndpointParamsInterceptor"
+        "GetUiCustomizationEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_user.rs`

```diff
--- reference/src/operation/get_user.rs
+++ generated/src/operation/get_user.rs
@@ -103,6 +103,16 @@
             "GetUser",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/get_user_attribute_verification_code.rs`

```diff
--- reference/src/operation/get_user_attribute_verification_code.rs
+++ generated/src/operation/get_user_attribute_verification_code.rs
@@ -112,6 +112,16 @@
             "GetUserAttributeVerificationCode",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/get_user_auth_factors.rs`

```diff
--- reference/src/operation/get_user_auth_factors.rs
+++ generated/src/operation/get_user_auth_factors.rs
@@ -113,6 +113,16 @@
             "GetUserAuthFactors",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/global_sign_out.rs`

```diff
--- reference/src/operation/global_sign_out.rs
+++ generated/src/operation/global_sign_out.rs
@@ -112,6 +112,16 @@
             "GlobalSignOut",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/initiate_auth.rs`

```diff
--- reference/src/operation/initiate_auth.rs
+++ generated/src/operation/initiate_auth.rs
@@ -113,6 +113,16 @@
             "InitiateAuth",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/list_devices.rs`

```diff
--- reference/src/operation/list_devices.rs
+++ generated/src/operation/list_devices.rs
@@ -113,6 +113,16 @@
             "ListDevices",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/list_web_authn_credentials.rs`

```diff
--- reference/src/operation/list_web_authn_credentials.rs
+++ generated/src/operation/list_web_authn_credentials.rs
@@ -112,6 +112,16 @@
             "ListWebAuthnCredentials",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/resend_confirmation_code.rs`

```diff
--- reference/src/operation/resend_confirmation_code.rs
+++ generated/src/operation/resend_confirmation_code.rs
@@ -112,6 +112,16 @@
             "ResendConfirmationCode",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/respond_to_auth_challenge.rs`

```diff
--- reference/src/operation/respond_to_auth_challenge.rs
+++ generated/src/operation/respond_to_auth_challenge.rs
@@ -113,6 +113,16 @@
             "RespondToAuthChallenge",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/revoke_token.rs`

```diff
--- reference/src/operation/revoke_token.rs
+++ generated/src/operation/revoke_token.rs
@@ -112,6 +112,16 @@
             "RevokeToken",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/set_ui_customization/builders.rs`

```diff
--- reference/src/operation/set_ui_customization/builders.rs
+++ generated/src/operation/set_ui_customization/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::set_ui_customization::SetUiCustomizationOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::set_ui_customization::SetUICustomizationError,
+            super::super::super::operation::set_ui_customization::SetUiCustomizationError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `SetUICustomization`.
+/// Fluent builder constructing a request to `SetUiCustomization`.
 ///
 /// <p>Configures UI branding settings for domains with the hosted UI (classic) branding version. Your user pool must have a domain. Configure a domain with .</p>
 /// <p>Set the default configuration for all clients with a <code>ClientId</code> of <code>ALL</code>. When the <code>ClientId</code> value is an app client ID, the settings you pass in this request apply to that app client and override the default <code>ALL</code> configuration.</p><note>
@@ -34,7 +34,7 @@
 /// </ul>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct SetUICustomizationFluentBuilder {
+pub struct SetUiCustomizationFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::set_ui_customization::builders::SetUiCustomizationInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -42,8 +42,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::set_ui_customization::SetUiCustomizationOutput,
-        super::super::super::operation::set_ui_customization::SetUICustomizationError,
-    > for SetUICustomizationFluentBuilder
+        super::super::super::operation::set_ui_customization::SetUiCustomizationError,
+    > for SetUiCustomizationFluentBuilder
 {
     fn send(
         self,
@@ -51,14 +51,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::set_ui_customization::SetUiCustomizationOutput,
-            super::super::super::operation::set_ui_customization::SetUICustomizationError,
+            super::super::super::operation::set_ui_customization::SetUiCustomizationError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl SetUICustomizationFluentBuilder {
-    /// Creates a new `SetUICustomizationFluentBuilder`.
+impl SetUiCustomizationFluentBuilder {
+    /// Creates a new `SetUiCustomizationFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -66,7 +66,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the SetUICustomization as a reference.
+    /// Access the SetUiCustomization as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::set_ui_customization::builders::SetUiCustomizationInputBuilder {
         &self.inner
     }
@@ -83,7 +83,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::set_ui_customization::SetUiCustomizationOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::set_ui_customization::SetUICustomizationError,
+            super::super::super::operation::set_ui_customization::SetUiCustomizationError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -91,12 +91,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::set_ui_customization::SetUICustomization::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::set_ui_customization::SetUiCustomization::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::set_ui_customization::SetUICustomization::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::set_ui_customization::SetUiCustomization::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -104,7 +104,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::set_ui_customization::SetUiCustomizationOutput,
-        super::super::super::operation::set_ui_customization::SetUICustomizationError,
+        super::super::super::operation::set_ui_customization::SetUiCustomizationError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/set_ui_customization.rs`

```diff
--- reference/src/operation/set_ui_customization.rs
+++ generated/src/operation/set_ui_customization.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `SetUICustomization`.
+/// Orchestration and serialization glue logic for `SetUiCustomization`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct SetUICustomization;
-impl SetUICustomization {
-    /// Creates a new `SetUICustomization`
+pub struct SetUiCustomization;
+impl SetUiCustomization {
+    /// Creates a new `SetUiCustomization`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SetUICustomization {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SetUiCustomization {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("SetUICustomization");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            SetUICustomizationRequestSerializer,
+            SetUiCustomizationRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            SetUICustomizationResponseDeserializer,
+            SetUiCustomizationResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -134,13 +134,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("SetUICustomization")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetUICustomizationTelemetryInputCaptureInterceptor,
+                SetUiCustomizationTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetUICustomizationEndpointParamsInterceptor,
+                SetUiCustomizationEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::set_ui_customization::SetUICustomizationError,
@@ -157,12 +157,12 @@
 }

 #[derive(Debug)]
-struct SetUICustomizationTelemetryInputCaptureInterceptor;
+struct SetUiCustomizationTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetUICustomizationTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetUiCustomizationTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "SetUICustomizationTelemetryInputCaptureInterceptor"
+        "SetUiCustomizationTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -280,12 +280,12 @@
     }
 }
 #[derive(Debug)]
-struct SetUICustomizationEndpointParamsInterceptor;
+struct SetUiCustomizationEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetUICustomizationEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetUiCustomizationEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "SetUICustomizationEndpointParamsInterceptor"
+        "SetUiCustomizationEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/set_user_mfa_preference/builders.rs`

```diff
--- reference/src/operation/set_user_mfa_preference/builders.rs
+++ generated/src/operation/set_user_mfa_preference/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
+            super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `SetUserMFAPreference`.
+/// Fluent builder constructing a request to `SetUserMfaPreference`.
 ///
 /// <p>Set the user's multi-factor authentication (MFA) method preference, including which MFA factors are activated and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are activated. If multiple options are activated and no preference is set, a challenge to choose an MFA option will be returned during sign-in. If an MFA type is activated for a user, the user will be prompted for MFA during all sign-in attempts unless device tracking is turned on and the device has been trusted. If you want MFA to be applied selectively based on the assessed risk level of sign-in attempts, deactivate MFA for users and turn on Adaptive Authentication for the user pool.</p>
 /// <p>Authorize this action with a signed-in user's access token. It must include the scope <code>aws.cognito.signin.user.admin</code>.</p><note>
@@ -27,7 +27,7 @@
 /// <p>Amazon Cognito doesn't evaluate Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you can't use IAM credentials to authorize requests, and you can't grant IAM permissions in policies. For more information about authorization models in Amazon Cognito, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pools-API-operations.html">Using the Amazon Cognito user pools API and user pool endpoints</a>.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct SetUserMFAPreferenceFluentBuilder {
+pub struct SetUserMfaPreferenceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::set_user_mfa_preference::builders::SetUserMfaPreferenceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -35,8 +35,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
-        super::super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
-    > for SetUserMFAPreferenceFluentBuilder
+        super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
+    > for SetUserMfaPreferenceFluentBuilder
 {
     fn send(
         self,
@@ -44,14 +44,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
-            super::super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
+            super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl SetUserMFAPreferenceFluentBuilder {
-    /// Creates a new `SetUserMFAPreferenceFluentBuilder`.
+impl SetUserMfaPreferenceFluentBuilder {
+    /// Creates a new `SetUserMfaPreferenceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -59,7 +59,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the SetUserMFAPreference as a reference.
+    /// Access the SetUserMfaPreference as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::set_user_mfa_preference::builders::SetUserMfaPreferenceInputBuilder {
         &self.inner
     }
@@ -76,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
+            super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,12 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::set_user_mfa_preference::SetUserMFAPreference::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::set_user_mfa_preference::SetUserMfaPreference::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::set_user_mfa_preference::SetUserMFAPreference::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::set_user_mfa_preference::SetUserMfaPreference::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -97,7 +97,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
-        super::super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
+        super::super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/set_user_mfa_preference.rs`

```diff
--- reference/src/operation/set_user_mfa_preference.rs
+++ generated/src/operation/set_user_mfa_preference.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `SetUserMFAPreference`.
+/// Orchestration and serialization glue logic for `SetUserMfaPreference`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct SetUserMFAPreference;
-impl SetUserMFAPreference {
-    /// Creates a new `SetUserMFAPreference`
+pub struct SetUserMfaPreference;
+impl SetUserMfaPreference {
+    /// Creates a new `SetUserMfaPreference`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SetUserMFAPreference {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SetUserMfaPreference {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("SetUserMFAPreference");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            SetUserMFAPreferenceRequestSerializer,
+            SetUserMfaPreferenceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            SetUserMFAPreferenceResponseDeserializer,
+            SetUserMfaPreferenceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -112,6 +112,16 @@
             "SetUserMFAPreference",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -126,7 +136,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                SetUserMFAPreferenceEndpointParamsInterceptor,
+                SetUserMfaPreferenceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
@@ -219,12 +229,12 @@
     }
 }
 #[derive(Debug)]
-struct SetUserMFAPreferenceEndpointParamsInterceptor;
+struct SetUserMfaPreferenceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetUserMFAPreferenceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SetUserMfaPreferenceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "SetUserMFAPreferenceEndpointParamsInterceptor"
+        "SetUserMfaPreferenceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/set_user_settings.rs`

```diff
--- reference/src/operation/set_user_settings.rs
+++ generated/src/operation/set_user_settings.rs
@@ -112,6 +112,16 @@
             "SetUserSettings",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/sign_up.rs`

```diff
--- reference/src/operation/sign_up.rs
+++ generated/src/operation/sign_up.rs
@@ -99,6 +99,16 @@
             "SignUp",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/start_web_authn_registration/_start_web_authn_registration_output.rs`

```diff
--- reference/src/operation/start_web_authn_registration/_start_web_authn_registration_output.rs
+++ generated/src/operation/start_web_authn_registration/_start_web_authn_registration_output.rs
@@ -4,12 +4,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct StartWebAuthnRegistrationOutput {
     /// <p>The information that a user can provide in their request to register with their passkey provider.</p>
-    pub credential_creation_options: ::aws_smithy_types::Document,
+    pub credential_creation_options: ::std::string::String,
     _request_id: Option<String>,
 }
 impl StartWebAuthnRegistrationOutput {
     /// <p>The information that a user can provide in their request to register with their passkey provider.</p>
-    pub fn credential_creation_options(&self) -> &::aws_smithy_types::Document {
+    pub fn credential_creation_options(&self) -> &::std::string::String {
         &self.credential_creation_options
     }
 }
@@ -29,23 +29,23 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct StartWebAuthnRegistrationOutputBuilder {
-    pub(crate) credential_creation_options: ::std::option::Option<::aws_smithy_types::Document>,
+    pub(crate) credential_creation_options: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
 }
 impl StartWebAuthnRegistrationOutputBuilder {
     /// <p>The information that a user can provide in their request to register with their passkey provider.</p>
     /// This field is required.
-    pub fn credential_creation_options(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn credential_creation_options(mut self, input: ::std::string::String) -> Self {
         self.credential_creation_options = ::std::option::Option::Some(input);
         self
     }
     /// <p>The information that a user can provide in their request to register with their passkey provider.</p>
-    pub fn set_credential_creation_options(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_credential_creation_options(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.credential_creation_options = input;
         self
     }
     /// <p>The information that a user can provide in their request to register with their passkey provider.</p>
-    pub fn get_credential_creation_options(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_credential_creation_options(&self) -> &::std::option::Option<::std::string::String> {
         &self.credential_creation_options
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/start_web_authn_registration.rs`

```diff
--- reference/src/operation/start_web_authn_registration.rs
+++ generated/src/operation/start_web_authn_registration.rs
@@ -112,6 +112,16 @@
             "StartWebAuthnRegistration",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/update_auth_event_feedback.rs`

```diff
--- reference/src/operation/update_auth_event_feedback.rs
+++ generated/src/operation/update_auth_event_feedback.rs
@@ -112,6 +112,16 @@
             "UpdateAuthEventFeedback",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/update_device_status.rs`

```diff
--- reference/src/operation/update_device_status.rs
+++ generated/src/operation/update_device_status.rs
@@ -112,6 +112,16 @@
             "UpdateDeviceStatus",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/update_managed_login_branding/_update_managed_login_branding_input.rs`

```diff
--- reference/src/operation/update_managed_login_branding/_update_managed_login_branding_input.rs
+++ generated/src/operation/update_managed_login_branding/_update_managed_login_branding_input.rs
@@ -22,7 +22,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub settings: ::std::option::Option<::aws_smithy_types::Document>,
+    pub settings: ::std::option::Option<::std::string::String>,
     /// <p>An array of image files that you want to apply to roles like backgrounds, logos, and icons. Each object must also indicate whether it is for dark mode, light mode, or browser-adaptive mode.</p>
     pub assets: ::std::option::Option<::std::vec::Vec<super::super::super::types::AssetType>>,
 }
@@ -52,7 +52,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
+    pub fn settings(&self) -> ::std::option::Option<&::std::string::String> {
         self.settings.as_ref()
     }
     /// <p>An array of image files that you want to apply to roles like backgrounds, logos, and icons. Each object must also indicate whether it is for dark mode, light mode, or browser-adaptive mode.</p>
@@ -76,7 +76,7 @@
     pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
     pub(crate) managed_login_branding_id: ::std::option::Option<::std::string::String>,
     pub(crate) use_cognito_provided_values: ::std::option::Option<bool>,
-    pub(crate) settings: ::std::option::Option<::aws_smithy_types::Document>,
+    pub(crate) settings: ::std::option::Option<::std::string::String>,
     pub(crate) assets: ::std::option::Option<::std::vec::Vec<super::super::super::types::AssetType>>,
 }
 impl UpdateManagedLoginBrandingInputBuilder {
@@ -137,7 +137,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn settings(mut self, input: ::std::string::String) -> Self {
         self.settings = ::std::option::Option::Some(input);
         self
     }
@@ -153,7 +153,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn set_settings(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_settings(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.settings = input;
         self
     }
@@ -169,7 +169,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn get_settings(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_settings(&self) -> &::std::option::Option<::std::string::String> {
         &self.settings
     }
     /// Appends an item to `assets`.
@@ -202,7 +202,7 @@
         ::std::result::Result::Ok(super::super::super::operation::update_managed_login_branding::UpdateManagedLoginBrandingInput {
             user_pool_id: self.user_pool_id,
             managed_login_branding_id: self.managed_login_branding_id,
-            use_cognito_provided_values: self.use_cognito_provided_values,
+            use_cognito_provided_values: self.use_cognito_provided_values.unwrap_or_default(),
             settings: self.settings,
             assets: self.assets,
         })
```

### `src/operation/update_managed_login_branding/builders.rs`

```diff
--- reference/src/operation/update_managed_login_branding/builders.rs
+++ generated/src/operation/update_managed_login_branding/builders.rs
@@ -176,7 +176,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn settings(mut self, input: ::std::string::String) -> Self {
         self.inner = self.inner.settings(input);
         self
     }
@@ -192,7 +192,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn set_settings(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_settings(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.inner = self.inner.set_settings(input);
         self
     }
@@ -208,7 +208,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn get_settings(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_settings(&self) -> &::std::option::Option<::std::string::String> {
         self.inner.get_settings()
     }
     ///
```

### `src/operation/update_provisioned_limit/_update_provisioned_limit_input.rs`

```diff
--- reference/src/operation/update_provisioned_limit/_update_provisioned_limit_input.rs
+++ generated/src/operation/update_provisioned_limit/_update_provisioned_limit_input.rs
@@ -72,7 +72,7 @@
     > {
         ::std::result::Result::Ok(super::super::super::operation::update_provisioned_limit::UpdateProvisionedLimitInput {
             limit_definition: self.limit_definition,
-            requested_limit_value: self.requested_limit_value,
+            requested_limit_value: self.requested_limit_value.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/update_user_attributes.rs`

```diff
--- reference/src/operation/update_user_attributes.rs
+++ generated/src/operation/update_user_attributes.rs
@@ -112,6 +112,16 @@
             "UpdateUserAttributes",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/update_user_pool_client/_update_user_pool_client_input.rs`

```diff
--- reference/src/operation/update_user_pool_client/_update_user_pool_client_input.rs
+++ generated/src/operation/update_user_pool_client/_update_user_pool_client_input.rs
@@ -71,9 +71,9 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub callback_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub callback_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub logout_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub logout_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub default_redirect_uri: ::std::option::Option<::std::string::String>,
     /// <p>The OAuth grant types that you want your app client to generate. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p>
@@ -228,15 +228,15 @@
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
     ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.callback_urls.is_none()`.
-    pub fn callback_urls(&self) -> &[::std::string::String] {
-        self.callback_urls.as_deref().unwrap_or_default()
+    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.callback_ur_ls.is_none()`.
+    pub fn callback_ur_ls(&self) -> &[::std::string::String] {
+        self.callback_ur_ls.as_deref().unwrap_or_default()
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
     ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.logout_urls.is_none()`.
-    pub fn logout_urls(&self) -> &[::std::string::String] {
-        self.logout_urls.as_deref().unwrap_or_default()
+    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.logout_ur_ls.is_none()`.
+    pub fn logout_ur_ls(&self) -> &[::std::string::String] {
+        self.logout_ur_ls.as_deref().unwrap_or_default()
     }
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub fn default_redirect_uri(&self) -> ::std::option::Option<&str> {
@@ -331,8 +331,8 @@
         formatter.field("write_attributes", &self.write_attributes);
         formatter.field("explicit_auth_flows", &self.explicit_auth_flows);
         formatter.field("supported_identity_providers", &self.supported_identity_providers);
-        formatter.field("callback_urls", &self.callback_urls);
-        formatter.field("logout_urls", &self.logout_urls);
+        formatter.field("callback_ur_ls", &self.callback_ur_ls);
+        formatter.field("logout_ur_ls", &self.logout_ur_ls);
         formatter.field("default_redirect_uri", &self.default_redirect_uri);
         formatter.field("allowed_o_auth_flows", &self.allowed_o_auth_flows);
         formatter.field("allowed_o_auth_scopes", &self.allowed_o_auth_scopes);
@@ -371,8 +371,8 @@
     pub(crate) write_attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) explicit_auth_flows: ::std::option::Option<::std::vec::Vec<super::super::super::types::ExplicitAuthFlowsType>>,
     pub(crate) supported_identity_providers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) callback_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) logout_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub(crate) callback_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub(crate) logout_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) default_redirect_uri: ::std::option::Option<::std::string::String>,
     pub(crate) allowed_o_auth_flows: ::std::option::Option<::std::vec::Vec<super::super::super::types::OAuthFlowType>>,
     pub(crate) allowed_o_auth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -661,9 +661,9 @@
     pub fn get_supported_identity_providers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
         &self.supported_identity_providers
     }
-    /// Appends an item to `callback_urls`.
+    /// Appends an item to `callback_ur_ls`.
     ///
-    /// To override the contents of this collection use [`set_callback_urls`](Self::set_callback_urls).
+    /// To override the contents of this collection use [`set_callback_ur_ls`](Self::set_callback_ur_ls).
     ///
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
     /// <p>A redirect URI must meet the following requirements:</p>
@@ -678,10 +678,10 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn callback_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        let mut v = self.callback_urls.unwrap_or_default();
+    pub fn callback_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        let mut v = self.callback_ur_ls.unwrap_or_default();
         v.push(input.into());
-        self.callback_urls = ::std::option::Option::Some(v);
+        self.callback_ur_ls = ::std::option::Option::Some(v);
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -697,8 +697,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn set_callback_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.callback_urls = input;
+    pub fn set_callback_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.callback_ur_ls = input;
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -714,28 +714,28 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn get_callback_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        &self.callback_urls
+    pub fn get_callback_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        &self.callback_ur_ls
     }
-    /// Appends an item to `logout_urls`.
+    /// Appends an item to `logout_ur_ls`.
     ///
-    /// To override the contents of this collection use [`set_logout_urls`](Self::set_logout_urls).
+    /// To override the contents of this collection use [`set_logout_ur_ls`](Self::set_logout_ur_ls).
     ///
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn logout_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        let mut v = self.logout_urls.unwrap_or_default();
+    pub fn logout_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        let mut v = self.logout_ur_ls.unwrap_or_default();
         v.push(input.into());
-        self.logout_urls = ::std::option::Option::Some(v);
+        self.logout_ur_ls = ::std::option::Option::Some(v);
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn set_logout_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.logout_urls = input;
+    pub fn set_logout_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.logout_ur_ls = input;
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn get_logout_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        &self.logout_urls
+    pub fn get_logout_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        &self.logout_ur_ls
     }
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub fn default_redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
@@ -1000,7 +1000,7 @@
             user_pool_id: self.user_pool_id,
             client_id: self.client_id,
             client_name: self.client_name,
-            refresh_token_validity: self.refresh_token_validity,
+            refresh_token_validity: self.refresh_token_validity.unwrap_or_default(),
             access_token_validity: self.access_token_validity,
             id_token_validity: self.id_token_validity,
             token_validity_units: self.token_validity_units,
@@ -1008,12 +1008,12 @@
             write_attributes: self.write_attributes,
             explicit_auth_flows: self.explicit_auth_flows,
             supported_identity_providers: self.supported_identity_providers,
-            callback_urls: self.callback_urls,
-            logout_urls: self.logout_urls,
+            callback_ur_ls: self.callback_ur_ls,
+            logout_ur_ls: self.logout_ur_ls,
             default_redirect_uri: self.default_redirect_uri,
             allowed_o_auth_flows: self.allowed_o_auth_flows,
             allowed_o_auth_scopes: self.allowed_o_auth_scopes,
-            allowed_o_auth_flows_user_pool_client: self.allowed_o_auth_flows_user_pool_client,
+            allowed_o_auth_flows_user_pool_client: self.allowed_o_auth_flows_user_pool_client.unwrap_or_default(),
             analytics_configuration: self.analytics_configuration,
             prevent_user_existence_errors: self.prevent_user_existence_errors,
             enable_token_revocation: self.enable_token_revocation,
@@ -1037,8 +1037,8 @@
         formatter.field("write_attributes", &self.write_attributes);
         formatter.field("explicit_auth_flows", &self.explicit_auth_flows);
         formatter.field("supported_identity_providers", &self.supported_identity_providers);
-        formatter.field("callback_urls", &self.callback_urls);
-        formatter.field("logout_urls", &self.logout_urls);
+        formatter.field("callback_ur_ls", &self.callback_ur_ls);
+        formatter.field("logout_ur_ls", &self.logout_ur_ls);
         formatter.field("default_redirect_uri", &self.default_redirect_uri);
         formatter.field("allowed_o_auth_flows", &self.allowed_o_auth_flows);
         formatter.field("allowed_o_auth_scopes", &self.allowed_o_auth_scopes);
```

### `src/operation/update_user_pool_client/builders.rs`

```diff
--- reference/src/operation/update_user_pool_client/builders.rs
+++ generated/src/operation/update_user_pool_client/builders.rs
@@ -393,7 +393,7 @@
     ///
     /// Appends an item to `CallbackURLs`.
     ///
-    /// To override the contents of this collection use [`set_callback_urls`](Self::set_callback_urls).
+    /// To override the contents of this collection use [`set_callback_ur_ls`](Self::set_callback_ur_ls).
     ///
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
     /// <p>A redirect URI must meet the following requirements:</p>
@@ -408,8 +408,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn callback_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        self.inner = self.inner.callback_urls(input.into());
+    pub fn callback_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        self.inner = self.inner.callback_ur_ls(input.into());
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -425,8 +425,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn set_callback_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.inner = self.inner.set_callback_urls(input);
+    pub fn set_callback_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.inner = self.inner.set_callback_ur_ls(input);
         self
     }
     /// <p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p>
@@ -442,27 +442,27 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p>
     /// <p>App callback URLs such as <code>myapp://example</code> are also supported.</p>
-    pub fn get_callback_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        self.inner.get_callback_urls()
+    pub fn get_callback_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        self.inner.get_callback_ur_ls()
     }
     ///
     /// Appends an item to `LogoutURLs`.
     ///
-    /// To override the contents of this collection use [`set_logout_urls`](Self::set_logout_urls).
+    /// To override the contents of this collection use [`set_logout_ur_ls`](Self::set_logout_ur_ls).
     ///
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn logout_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        self.inner = self.inner.logout_urls(input.into());
+    pub fn logout_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        self.inner = self.inner.logout_ur_ls(input.into());
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn set_logout_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.inner = self.inner.set_logout_urls(input);
+    pub fn set_logout_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.inner = self.inner.set_logout_ur_ls(input);
         self
     }
     /// <p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p>
-    pub fn get_logout_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        self.inner.get_logout_urls()
+    pub fn get_logout_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        self.inner.get_logout_ur_ls()
     }
     /// <p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p>
     pub fn default_redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
```

### `src/operation/verify_software_token.rs`

```diff
--- reference/src/operation/verify_software_token.rs
+++ generated/src/operation/verify_software_token.rs
@@ -113,6 +113,16 @@
             "VerifySoftwareToken",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/operation/verify_user_attribute.rs`

```diff
--- reference/src/operation/verify_user_attribute.rs
+++ generated/src/operation/verify_user_attribute.rs
@@ -112,6 +112,16 @@
             "VerifyUserAttribute",
             "Cognito Identity Provider",
         ));
+        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
+        signing_options.double_uri_encode = true;
+        signing_options.content_sha256_header = false;
+        signing_options.normalize_uri_path = true;
+        signing_options.payload_override = None;
+
+        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
+            signing_options,
+            ..::std::default::Default::default()
+        });

         ::std::option::Option::Some(cfg.freeze())
     }
```

### `src/protocol_serde/shape_add_custom_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_add_custom_attributes.rs
+++ generated/src/protocol_serde/shape_add_custom_attributes.rs
@@ -159,3 +159,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_add_custom_attributes(
+    _value: &[u8],
+    mut builder: super::super::operation::add_custom_attributes::builders::AddCustomAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::add_custom_attributes::builders::AddCustomAttributesOutputBuilder,
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

### `src/protocol_serde/shape_admin_add_user_to_group.rs`

```diff
--- reference/src/protocol_serde/shape_admin_add_user_to_group.rs
+++ generated/src/protocol_serde/shape_admin_add_user_to_group.rs
@@ -156,3 +156,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_add_user_to_group(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_add_user_to_group::builders::AdminAddUserToGroupOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_add_user_to_group::builders::AdminAddUserToGroupOutputBuilder,
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

### `src/protocol_serde/shape_admin_confirm_sign_up.rs`

```diff
--- reference/src/protocol_serde/shape_admin_confirm_sign_up.rs
+++ generated/src/protocol_serde/shape_admin_confirm_sign_up.rs
@@ -240,3 +240,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_confirm_sign_up(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_confirm_sign_up::builders::AdminConfirmSignUpOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_confirm_sign_up::builders::AdminConfirmSignUpOutputBuilder,
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

### `src/protocol_serde/shape_admin_create_user_config_type.rs`

```diff
--- reference/src/protocol_serde/shape_admin_create_user_config_type.rs
+++ generated/src/protocol_serde/shape_admin_create_user_config_type.rs
@@ -3,10 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::AdminCreateUserConfigType,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.allow_admin_create_user_only {
+    {
         object.key("AllowAdminCreateUserOnly").boolean(input.allow_admin_create_user_only);
     }
-    if input.unused_account_validity_days != 0 {
+    {
         object.key("UnusedAccountValidityDays").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((input.unused_account_validity_days).into()),
```

### `src/protocol_serde/shape_admin_delete_user.rs`

```diff
--- reference/src/protocol_serde/shape_admin_delete_user.rs
+++ generated/src/protocol_serde/shape_admin_delete_user.rs
@@ -150,3 +150,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_delete_user(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_delete_user::builders::AdminDeleteUserOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_delete_user::builders::AdminDeleteUserOutputBuilder,
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

### `src/protocol_serde/shape_admin_delete_user_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_admin_delete_user_attributes.rs
+++ generated/src/protocol_serde/shape_admin_delete_user_attributes.rs
@@ -164,3 +164,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_delete_user_attributes(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_delete_user_attributes::builders::AdminDeleteUserAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_delete_user_attributes::builders::AdminDeleteUserAttributesOutputBuilder,
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

### `src/protocol_serde/shape_admin_disable_provider_for_user.rs`

```diff
--- reference/src/protocol_serde/shape_admin_disable_provider_for_user.rs
+++ generated/src/protocol_serde/shape_admin_disable_provider_for_user.rs
@@ -184,3 +184,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_disable_provider_for_user(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_disable_provider_for_user::builders::AdminDisableProviderForUserOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_disable_provider_for_user::builders::AdminDisableProviderForUserOutputBuilder,
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

### `src/protocol_serde/shape_admin_disable_user.rs`

```diff
--- reference/src/protocol_serde/shape_admin_disable_user.rs
+++ generated/src/protocol_serde/shape_admin_disable_user.rs
@@ -150,3 +150,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_disable_user(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_disable_user::builders::AdminDisableUserOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_disable_user::builders::AdminDisableUserOutputBuilder,
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

### `src/protocol_serde/shape_admin_enable_user.rs`

```diff
--- reference/src/protocol_serde/shape_admin_enable_user.rs
+++ generated/src/protocol_serde/shape_admin_enable_user.rs
@@ -150,3 +150,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_enable_user(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_enable_user::builders::AdminEnableUserOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_enable_user::builders::AdminEnableUserOutputBuilder,
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

### `src/protocol_serde/shape_admin_forget_device.rs`

```diff
--- reference/src/protocol_serde/shape_admin_forget_device.rs
+++ generated/src/protocol_serde/shape_admin_forget_device.rs
@@ -168,3 +168,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_forget_device(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_forget_device::builders::AdminForgetDeviceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_forget_device::builders::AdminForgetDeviceOutputBuilder,
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

### `src/protocol_serde/shape_admin_link_provider_for_user.rs`

```diff
--- reference/src/protocol_serde/shape_admin_link_provider_for_user.rs
+++ generated/src/protocol_serde/shape_admin_link_provider_for_user.rs
@@ -194,3 +194,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_link_provider_for_user(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_link_provider_for_user::builders::AdminLinkProviderForUserOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_link_provider_for_user::builders::AdminLinkProviderForUserOutputBuilder,
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

### `src/protocol_serde/shape_admin_remove_user_from_group.rs`

```diff
--- reference/src/protocol_serde/shape_admin_remove_user_from_group.rs
+++ generated/src/protocol_serde/shape_admin_remove_user_from_group.rs
@@ -164,3 +164,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_remove_user_from_group(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_remove_user_from_group::builders::AdminRemoveUserFromGroupOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_remove_user_from_group::builders::AdminRemoveUserFromGroupOutputBuilder,
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

### `src/protocol_serde/shape_admin_reset_user_password.rs`

```diff
--- reference/src/protocol_serde/shape_admin_reset_user_password.rs
+++ generated/src/protocol_serde/shape_admin_reset_user_password.rs
@@ -281,3 +281,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_reset_user_password(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_reset_user_password::builders::AdminResetUserPasswordOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_reset_user_password::builders::AdminResetUserPasswordOutputBuilder,
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

### `src/protocol_serde/shape_admin_set_user_mfa_preference.rs`

```diff
--- reference/src/protocol_serde/shape_admin_set_user_mfa_preference.rs
+++ generated/src/protocol_serde/shape_admin_set_user_mfa_preference.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
-    super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
+    super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+        .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled(generic)),
+        None => return Err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalErrorException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::InternalErrorException({
+        "InternalErrorException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InvalidParameterException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::InvalidParameterException({
+        "InvalidParameterException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "NotAuthorizedException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::NotAuthorizedException({
+        "NotAuthorizedException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::NotAuthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::NotAuthorizedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -66,7 +66,7 @@
             tmp
         }),
         "OperationNotEnabledException" => {
-            super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::OperationNotEnabledException({
+            super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::OperationNotEnabledException({
                 #[allow(unused_mut)]
                 let mut tmp = {
                     #[allow(unused_mut)]
@@ -75,7 +75,7 @@
                         _response_body,
                         output,
                     )
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -86,7 +86,7 @@
             })
         }
         "PasswordResetRequiredException" => {
-            super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::PasswordResetRequiredException({
+            super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::PasswordResetRequiredException({
                 #[allow(unused_mut)]
                 let mut tmp = {
                     #[allow(unused_mut)]
@@ -95,7 +95,7 @@
                         _response_body,
                         output,
                     )
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -105,13 +105,13 @@
                 tmp
             })
         }
-        "ResourceNotFoundException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::ResourceNotFoundException({
+        "ResourceNotFoundException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -120,13 +120,13 @@
             }
             tmp
         }),
-        "UserNotConfirmedException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::UserNotConfirmedException({
+        "UserNotConfirmedException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::UserNotConfirmedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UserNotConfirmedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_user_not_confirmed_exception::de_user_not_confirmed_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -135,13 +135,13 @@
             }
             tmp
         }),
-        "UserNotFoundException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::UserNotFoundException({
+        "UserNotFoundException" => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::UserNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UserNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_user_not_found_exception::de_user_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -150,7 +150,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError::generic(generic),
+        _ => super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError::generic(generic),
     })
 }

@@ -161,7 +161,7 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
-    super::super::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
+    super::super::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceError,
 > {
     Ok({
         #[allow(unused_mut)]
@@ -180,3 +180,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_set_user_mfa_preference(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_set_user_mfa_preference::builders::AdminSetUserMfaPreferenceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_set_user_mfa_preference::builders::AdminSetUserMfaPreferenceOutputBuilder,
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

### `src/protocol_serde/shape_admin_set_user_password.rs`

```diff
--- reference/src/protocol_serde/shape_admin_set_user_password.rs
+++ generated/src/protocol_serde/shape_admin_set_user_password.rs
@@ -187,3 +187,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_set_user_password(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_set_user_password::builders::AdminSetUserPasswordOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_set_user_password::builders::AdminSetUserPasswordOutputBuilder,
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

### `src/protocol_serde/shape_admin_set_user_settings.rs`

```diff
--- reference/src/protocol_serde/shape_admin_set_user_settings.rs
+++ generated/src/protocol_serde/shape_admin_set_user_settings.rs
@@ -141,3 +141,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_set_user_settings(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_set_user_settings::builders::AdminSetUserSettingsOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_set_user_settings::builders::AdminSetUserSettingsOutputBuilder,
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

### `src/protocol_serde/shape_admin_update_auth_event_feedback.rs`

```diff
--- reference/src/protocol_serde/shape_admin_update_auth_event_feedback.rs
+++ generated/src/protocol_serde/shape_admin_update_auth_event_feedback.rs
@@ -189,3 +189,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_update_auth_event_feedback(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_update_auth_event_feedback::builders::AdminUpdateAuthEventFeedbackOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_update_auth_event_feedback::builders::AdminUpdateAuthEventFeedbackOutputBuilder,
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

### `src/protocol_serde/shape_admin_update_device_status.rs`

```diff
--- reference/src/protocol_serde/shape_admin_update_device_status.rs
+++ generated/src/protocol_serde/shape_admin_update_device_status.rs
@@ -176,3 +176,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_update_device_status(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_update_device_status::builders::AdminUpdateDeviceStatusOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_update_device_status::builders::AdminUpdateDeviceStatusOutputBuilder,
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

### `src/protocol_serde/shape_admin_update_user_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_admin_update_user_attributes.rs
+++ generated/src/protocol_serde/shape_admin_update_user_attributes.rs
@@ -287,3 +287,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_update_user_attributes(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_update_user_attributes::builders::AdminUpdateUserAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_update_user_attributes::builders::AdminUpdateUserAttributesOutputBuilder,
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

### `src/protocol_serde/shape_admin_user_global_sign_out.rs`

```diff
--- reference/src/protocol_serde/shape_admin_user_global_sign_out.rs
+++ generated/src/protocol_serde/shape_admin_user_global_sign_out.rs
@@ -160,3 +160,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_admin_user_global_sign_out(
+    _value: &[u8],
+    mut builder: super::super::operation::admin_user_global_sign_out::builders::AdminUserGlobalSignOutOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::admin_user_global_sign_out::builders::AdminUserGlobalSignOutOutputBuilder,
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

### `src/protocol_serde/shape_analytics_configuration_type.rs`

```diff
--- reference/src/protocol_serde/shape_analytics_configuration_type.rs
+++ generated/src/protocol_serde/shape_analytics_configuration_type.rs
@@ -15,7 +15,7 @@
     if let Some(var_4) = &input.external_id {
         object.key("ExternalId").string(var_4.as_str());
     }
-    if input.user_data_shared {
+    {
         object.key("UserDataShared").boolean(input.user_data_shared);
     }
     Ok(())
```

### `src/protocol_serde/shape_change_password.rs`

```diff
--- reference/src/protocol_serde/shape_change_password.rs
+++ generated/src/protocol_serde/shape_change_password.rs
@@ -244,3 +244,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_change_password(
+    _value: &[u8],
+    mut builder: super::super::operation::change_password::builders::ChangePasswordOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::change_password::builders::ChangePasswordOutputBuilder,
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

### `src/protocol_serde/shape_complete_web_authn_registration.rs`

```diff
--- reference/src/protocol_serde/shape_complete_web_authn_registration.rs
+++ generated/src/protocol_serde/shape_complete_web_authn_registration.rs
@@ -299,3 +299,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_complete_web_authn_registration(
+    _value: &[u8],
+    mut builder: super::super::operation::complete_web_authn_registration::builders::CompleteWebAuthnRegistrationOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::complete_web_authn_registration::builders::CompleteWebAuthnRegistrationOutputBuilder,
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

### `src/protocol_serde/shape_confirm_forgot_password.rs`

```diff
--- reference/src/protocol_serde/shape_confirm_forgot_password.rs
+++ generated/src/protocol_serde/shape_confirm_forgot_password.rs
@@ -331,3 +331,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_confirm_forgot_password(
+    _value: &[u8],
+    mut builder: super::super::operation::confirm_forgot_password::builders::ConfirmForgotPasswordOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::confirm_forgot_password::builders::ConfirmForgotPasswordOutputBuilder,
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

### `src/protocol_serde/shape_create_user_pool_client_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_user_pool_client_input.rs
+++ generated/src/protocol_serde/shape_create_user_pool_client_input.rs
@@ -75,7 +75,7 @@
         }
         array_20.finish();
     }
-    if let Some(var_22) = &input.callback_urls {
+    if let Some(var_22) = &input.callback_ur_ls {
         let mut array_23 = object.key("CallbackURLs").start_array();
         for item_24 in var_22 {
             {
@@ -84,7 +84,7 @@
         }
         array_23.finish();
     }
-    if let Some(var_25) = &input.logout_urls {
+    if let Some(var_25) = &input.logout_ur_ls {
         let mut array_26 = object.key("LogoutURLs").start_array();
         for item_27 in var_25 {
             {
```

### `src/protocol_serde/shape_delete_group.rs`

```diff
--- reference/src/protocol_serde/shape_delete_group.rs
+++ generated/src/protocol_serde/shape_delete_group.rs
@@ -135,3 +135,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_group(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_group::builders::DeleteGroupOutputBuilder,
+) -> ::std::result::Result<super::super::operation::delete_group::builders::DeleteGroupOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
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

### `src/protocol_serde/shape_delete_identity_provider.rs`

```diff
--- reference/src/protocol_serde/shape_delete_identity_provider.rs
+++ generated/src/protocol_serde/shape_delete_identity_provider.rs
@@ -170,3 +170,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_identity_provider(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_identity_provider::builders::DeleteIdentityProviderOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_identity_provider::builders::DeleteIdentityProviderOutputBuilder,
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

### `src/protocol_serde/shape_delete_managed_login_branding.rs`

```diff
--- reference/src/protocol_serde/shape_delete_managed_login_branding.rs
+++ generated/src/protocol_serde/shape_delete_managed_login_branding.rs
@@ -165,3 +165,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_managed_login_branding(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_managed_login_branding::builders::DeleteManagedLoginBrandingOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_managed_login_branding::builders::DeleteManagedLoginBrandingOutputBuilder,
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

### `src/protocol_serde/shape_delete_resource_server.rs`

```diff
--- reference/src/protocol_serde/shape_delete_resource_server.rs
+++ generated/src/protocol_serde/shape_delete_resource_server.rs
@@ -141,3 +141,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_resource_server(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_resource_server::builders::DeleteResourceServerOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_resource_server::builders::DeleteResourceServerOutputBuilder,
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

### `src/protocol_serde/shape_delete_terms.rs`

```diff
--- reference/src/protocol_serde/shape_delete_terms.rs
+++ generated/src/protocol_serde/shape_delete_terms.rs
@@ -153,3 +153,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_terms(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_terms::builders::DeleteTermsOutputBuilder,
+) -> ::std::result::Result<super::super::operation::delete_terms::builders::DeleteTermsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
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

### `src/protocol_serde/shape_delete_user.rs`

```diff
--- reference/src/protocol_serde/shape_delete_user.rs
+++ generated/src/protocol_serde/shape_delete_user.rs
@@ -198,3 +198,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_user(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_user::builders::DeleteUserOutputBuilder,
+) -> ::std::result::Result<super::super::operation::delete_user::builders::DeleteUserOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
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

### `src/protocol_serde/shape_delete_user_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_delete_user_attributes.rs
+++ generated/src/protocol_serde/shape_delete_user_attributes.rs
@@ -204,3 +204,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_user_attributes(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_user_attributes::builders::DeleteUserAttributesOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_user_attributes::builders::DeleteUserAttributesOutputBuilder,
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

### `src/protocol_serde/shape_delete_user_pool.rs`

```diff
--- reference/src/protocol_serde/shape_delete_user_pool.rs
+++ generated/src/protocol_serde/shape_delete_user_pool.rs
@@ -153,3 +153,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_user_pool(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_user_pool::builders::DeleteUserPoolOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_user_pool::builders::DeleteUserPoolOutputBuilder,
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

### `src/protocol_serde/shape_delete_user_pool_client.rs`

```diff
--- reference/src/protocol_serde/shape_delete_user_pool_client.rs
+++ generated/src/protocol_serde/shape_delete_user_pool_client.rs
@@ -159,3 +159,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_user_pool_client(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_user_pool_client::builders::DeleteUserPoolClientOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_user_pool_client::builders::DeleteUserPoolClientOutputBuilder,
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

### `src/protocol_serde/shape_delete_user_pool_client_secret.rs`

```diff
--- reference/src/protocol_serde/shape_delete_user_pool_client_secret.rs
+++ generated/src/protocol_serde/shape_delete_user_pool_client_secret.rs
@@ -131,3 +131,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_user_pool_client_secret(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_user_pool_client_secret::builders::DeleteUserPoolClientSecretOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_user_pool_client_secret::builders::DeleteUserPoolClientSecretOutputBuilder,
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

### `src/protocol_serde/shape_delete_user_pool_domain.rs`

```diff
--- reference/src/protocol_serde/shape_delete_user_pool_domain.rs
+++ generated/src/protocol_serde/shape_delete_user_pool_domain.rs
@@ -144,3 +144,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_user_pool_domain(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_user_pool_domain::builders::DeleteUserPoolDomainOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_user_pool_domain::builders::DeleteUserPoolDomainOutputBuilder,
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

### `src/protocol_serde/shape_delete_web_authn_credential.rs`

```diff
--- reference/src/protocol_serde/shape_delete_web_authn_credential.rs
+++ generated/src/protocol_serde/shape_delete_web_authn_credential.rs
@@ -199,3 +199,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_web_authn_credential(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_web_authn_credential::builders::DeleteWebAuthnCredentialOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_web_authn_credential::builders::DeleteWebAuthnCredentialOutputBuilder,
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

### `src/protocol_serde/shape_device_configuration_type.rs`

```diff
--- reference/src/protocol_serde/shape_device_configuration_type.rs
+++ generated/src/protocol_serde/shape_device_configuration_type.rs
@@ -3,10 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::DeviceConfigurationType,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.challenge_required_on_new_device {
+    {
         object.key("ChallengeRequiredOnNewDevice").boolean(input.challenge_required_on_new_device);
     }
-    if input.device_only_remembered_on_user_prompt {
+    {
         object
             .key("DeviceOnlyRememberedOnUserPrompt")
             .boolean(input.device_only_remembered_on_user_prompt);
```

### `src/protocol_serde/shape_email_mfa_config_type.rs`

```diff
--- reference/src/protocol_serde/shape_email_mfa_config_type.rs
+++ generated/src/protocol_serde/shape_email_mfa_config_type.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_email_mfa_config_type(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::EmailMfaConfigType,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.message {
+        object.key("Message").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.subject {
+        object.key("Subject").string(var_2.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_email_mfa_config_type<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -51,16 +64,3 @@
         )),
     }
 }
-
-pub fn ser_email_mfa_config_type(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::EmailMfaConfigType,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.message {
-        object.key("Message").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.subject {
-        object.key("Subject").string(var_2.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_email_mfa_settings_type.rs`

```diff
--- reference/src/protocol_serde/shape_email_mfa_settings_type.rs
+++ generated/src/protocol_serde/shape_email_mfa_settings_type.rs
@@ -3,10 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EmailMfaSettingsType,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.enabled {
+    {
         object.key("Enabled").boolean(input.enabled);
     }
-    if input.preferred_mfa {
+    {
         object.key("PreferredMfa").boolean(input.preferred_mfa);
     }
     Ok(())
```

### `src/protocol_serde/shape_failover_type.rs`

```diff
--- reference/src/protocol_serde/shape_failover_type.rs
+++ generated/src/protocol_serde/shape_failover_type.rs
@@ -1,4 +1,19 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_failover_type(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::FailoverType,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("SecondaryRegion").string(input.secondary_region.as_str());
+    }
+    {
+        object
+            .key("PrimaryRoute53HealthCheckId")
+            .string(input.primary_route53_health_check_id.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_failover_type<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -53,18 +68,3 @@
         )),
     }
 }
-
-pub fn ser_failover_type(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::FailoverType,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("SecondaryRegion").string(input.secondary_region.as_str());
-    }
-    {
-        object
-            .key("PrimaryRoute53HealthCheckId")
-            .string(input.primary_route53_health_check_id.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_forget_device.rs`

```diff
--- reference/src/protocol_serde/shape_forget_device.rs
+++ generated/src/protocol_serde/shape_forget_device.rs
@@ -214,3 +214,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_forget_device(
+    _value: &[u8],
+    mut builder: super::super::operation::forget_device::builders::ForgetDeviceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::forget_device::builders::ForgetDeviceOutputBuilder,
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

### `src/protocol_serde/shape_get_csv_header.rs`

```diff
--- reference/src/protocol_serde/shape_get_csv_header.rs
+++ generated/src/protocol_serde/shape_get_csv_header.rs
@@ -4,26 +4,26 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_csv_header::GetCsvHeaderOutput, super::super::operation::get_csv_header::GetCSVHeaderError> {
+) -> std::result::Result<super::super::operation::get_csv_header::GetCsvHeaderOutput, super::super::operation::get_csv_header::GetCsvHeaderError> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+        .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled(generic)),
+        None => return Err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalErrorException" => super::super::operation::get_csv_header::GetCSVHeaderError::InternalErrorException({
+        "InternalErrorException" => super::super::operation::get_csv_header::GetCsvHeaderError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+                    .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,13 +32,13 @@
             }
             tmp
         }),
-        "InvalidParameterException" => super::super::operation::get_csv_header::GetCSVHeaderError::InvalidParameterException({
+        "InvalidParameterException" => super::super::operation::get_csv_header::GetCsvHeaderError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+                    .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,13 +47,13 @@
             }
             tmp
         }),
-        "NotAuthorizedException" => super::super::operation::get_csv_header::GetCSVHeaderError::NotAuthorizedException({
+        "NotAuthorizedException" => super::super::operation::get_csv_header::GetCsvHeaderError::NotAuthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::NotAuthorizedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+                    .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -62,7 +62,7 @@
             }
             tmp
         }),
-        "OperationNotEnabledException" => super::super::operation::get_csv_header::GetCSVHeaderError::OperationNotEnabledException({
+        "OperationNotEnabledException" => super::super::operation::get_csv_header::GetCsvHeaderError::OperationNotEnabledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -69,7 +69,7 @@
                 let mut output = super::super::types::error::builders::OperationNotEnabledExceptionBuilder::default();
                 output =
                     super::super::protocol_serde::shape_operation_not_enabled_exception::de_operation_not_enabled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+                        .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -78,13 +78,13 @@
             }
             tmp
         }),
-        "ResourceNotFoundException" => super::super::operation::get_csv_header::GetCSVHeaderError::ResourceNotFoundException({
+        "ResourceNotFoundException" => super::super::operation::get_csv_header::GetCsvHeaderError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+                    .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -93,13 +93,13 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => super::super::operation::get_csv_header::GetCSVHeaderError::TooManyRequestsException({
+        "TooManyRequestsException" => super::super::operation::get_csv_header::GetCsvHeaderError::TooManyRequestsException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+                    .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -108,7 +108,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::get_csv_header::GetCSVHeaderError::generic(generic),
+        _ => super::super::operation::get_csv_header::GetCsvHeaderError::generic(generic),
     })
 }

@@ -117,12 +117,12 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_csv_header::GetCsvHeaderOutput, super::super::operation::get_csv_header::GetCSVHeaderError> {
+) -> std::result::Result<super::super::operation::get_csv_header::GetCsvHeaderOutput, super::super::operation::get_csv_header::GetCsvHeaderError> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::get_csv_header::builders::GetCsvHeaderOutputBuilder::default();
         output = super::super::protocol_serde::shape_get_csv_header::de_get_csv_header(_response_body, output)
-            .map_err(super::super::operation::get_csv_header::GetCSVHeaderError::unhandled)?;
+            .map_err(super::super::operation::get_csv_header::GetCsvHeaderError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_get_ui_customization.rs`

```diff
--- reference/src/protocol_serde/shape_get_ui_customization.rs
+++ generated/src/protocol_serde/shape_get_ui_customization.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::get_ui_customization::GetUiCustomizationOutput,
-    super::super::operation::get_ui_customization::GetUICustomizationError,
+    super::super::operation::get_ui_customization::GetUiCustomizationError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+        .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled(generic)),
+        None => return Err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalErrorException" => super::super::operation::get_ui_customization::GetUICustomizationError::InternalErrorException({
+        "InternalErrorException" => super::super::operation::get_ui_customization::GetUiCustomizationError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InvalidParameterException" => super::super::operation::get_ui_customization::GetUICustomizationError::InvalidParameterException({
+        "InvalidParameterException" => super::super::operation::get_ui_customization::GetUiCustomizationError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "NotAuthorizedException" => super::super::operation::get_ui_customization::GetUICustomizationError::NotAuthorizedException({
+        "NotAuthorizedException" => super::super::operation::get_ui_customization::GetUiCustomizationError::NotAuthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::NotAuthorizedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,7 +65,7 @@
             }
             tmp
         }),
-        "OperationNotEnabledException" => super::super::operation::get_ui_customization::GetUICustomizationError::OperationNotEnabledException({
+        "OperationNotEnabledException" => super::super::operation::get_ui_customization::GetUiCustomizationError::OperationNotEnabledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -72,7 +72,7 @@
                 let mut output = super::super::types::error::builders::OperationNotEnabledExceptionBuilder::default();
                 output =
                     super::super::protocol_serde::shape_operation_not_enabled_exception::de_operation_not_enabled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+                        .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -81,13 +81,13 @@
             }
             tmp
         }),
-        "ResourceNotFoundException" => super::super::operation::get_ui_customization::GetUICustomizationError::ResourceNotFoundException({
+        "ResourceNotFoundException" => super::super::operation::get_ui_customization::GetUiCustomizationError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -96,13 +96,13 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => super::super::operation::get_ui_customization::GetUICustomizationError::TooManyRequestsException({
+        "TooManyRequestsException" => super::super::operation::get_ui_customization::GetUiCustomizationError::TooManyRequestsException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -111,7 +111,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::get_ui_customization::GetUICustomizationError::generic(generic),
+        _ => super::super::operation::get_ui_customization::GetUiCustomizationError::generic(generic),
     })
 }

@@ -122,13 +122,13 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::get_ui_customization::GetUiCustomizationOutput,
-    super::super::operation::get_ui_customization::GetUICustomizationError,
+    super::super::operation::get_ui_customization::GetUiCustomizationError,
 > {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::get_ui_customization::builders::GetUiCustomizationOutputBuilder::default();
         output = super::super::protocol_serde::shape_get_ui_customization::de_get_ui_customization(_response_body, output)
-            .map_err(super::super::operation::get_ui_customization::GetUICustomizationError::unhandled)?;
+            .map_err(super::super::operation::get_ui_customization::GetUiCustomizationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         super::super::serde_util::get_ui_customization_output_output_correct_errors(output).build()
     })
```

### `src/protocol_serde/shape_global_sign_out.rs`

```diff
--- reference/src/protocol_serde/shape_global_sign_out.rs
+++ generated/src/protocol_serde/shape_global_sign_out.rs
@@ -183,3 +183,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_global_sign_out(
+    _value: &[u8],
+    mut builder: super::super::operation::global_sign_out::builders::GlobalSignOutOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::global_sign_out::builders::GlobalSignOutOutputBuilder,
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

### `src/protocol_serde/shape_limit_type.rs`

```diff
--- reference/src/protocol_serde/shape_limit_type.rs
+++ generated/src/protocol_serde/shape_limit_type.rs
@@ -51,7 +51,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::limit_type_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::limit_type_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_managed_login_branding_type.rs`

```diff
--- reference/src/protocol_serde/shape_managed_login_branding_type.rs
+++ generated/src/protocol_serde/shape_managed_login_branding_type.rs
@@ -40,7 +40,7 @@
                                 builder.set_use_cognito_provided_values(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                         }
                         "Settings" => {
-                            builder = builder.set_settings(Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?));
+                            builder = builder.set_settings(Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?));
                         }
                         "Assets" => {
                             builder = builder.set_assets(super::super::protocol_serde::shape_asset_list_type::de_asset_list_type(
```

### `src/protocol_serde/shape_password_policy_type.rs`

```diff
--- reference/src/protocol_serde/shape_password_policy_type.rs
+++ generated/src/protocol_serde/shape_password_policy_type.rs
@@ -9,16 +9,16 @@
             ::aws_smithy_types::Number::NegInt((*var_1).into()),
         );
     }
-    if input.require_uppercase {
+    {
         object.key("RequireUppercase").boolean(input.require_uppercase);
     }
-    if input.require_lowercase {
+    {
         object.key("RequireLowercase").boolean(input.require_lowercase);
     }
-    if input.require_numbers {
+    {
         object.key("RequireNumbers").boolean(input.require_numbers);
     }
-    if input.require_symbols {
+    {
         object.key("RequireSymbols").boolean(input.require_symbols);
     }
     if let Some(var_2) = &input.password_history_size {
@@ -27,7 +27,7 @@
             ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
-    if input.temporary_password_validity_days != 0 {
+    {
         object.key("TemporaryPasswordValidityDays").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((input.temporary_password_validity_days).into()),
```

### `src/protocol_serde/shape_revoke_token.rs`

```diff
--- reference/src/protocol_serde/shape_revoke_token.rs
+++ generated/src/protocol_serde/shape_revoke_token.rs
@@ -169,3 +169,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_revoke_token(
+    _value: &[u8],
+    mut builder: super::super::operation::revoke_token::builders::RevokeTokenOutputBuilder,
+) -> ::std::result::Result<super::super::operation::revoke_token::builders::RevokeTokenOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
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

### `src/protocol_serde/shape_routing_type.rs`

```diff
--- reference/src/protocol_serde/shape_routing_type.rs
+++ generated/src/protocol_serde/shape_routing_type.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_routing_type(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::RoutingType,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.failover {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("Failover").start_object();
+        super::super::protocol_serde::shape_failover_type::ser_failover_type(&mut object_2, var_1)?;
+        object_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_routing_type<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -40,16 +53,3 @@
         )),
     }
 }
-
-pub fn ser_routing_type(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::RoutingType,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.failover {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("Failover").start_object();
-        super::super::protocol_serde::shape_failover_type::ser_failover_type(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_schema_attribute_type.rs`

```diff
--- reference/src/protocol_serde/shape_schema_attribute_type.rs
+++ generated/src/protocol_serde/shape_schema_attribute_type.rs
@@ -9,26 +9,26 @@
     if let Some(var_2) = &input.attribute_data_type {
         object.key("AttributeDataType").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.developer_only_attribute {
-        object.key("DeveloperOnlyAttribute").boolean(*var_3);
+    {
+        object.key("DeveloperOnlyAttribute").boolean(input.developer_only_attribute);
     }
-    if let Some(var_4) = &input.mutable {
-        object.key("Mutable").boolean(*var_4);
+    {
+        object.key("Mutable").boolean(input.mutable);
     }
-    if let Some(var_5) = &input.required {
-        object.key("Required").boolean(*var_5);
+    {
+        object.key("Required").boolean(input.required);
     }
-    if let Some(var_6) = &input.number_attribute_constraints {
+    if let Some(var_3) = &input.number_attribute_constraints {
         #[allow(unused_mut)]
-        let mut object_7 = object.key("NumberAttributeConstraints").start_object();
-        super::super::protocol_serde::shape_number_attribute_constraints_type::ser_number_attribute_constraints_type(&mut object_7, var_6)?;
-        object_7.finish();
+        let mut object_4 = object.key("NumberAttributeConstraints").start_object();
+        super::super::protocol_serde::shape_number_attribute_constraints_type::ser_number_attribute_constraints_type(&mut object_4, var_3)?;
+        object_4.finish();
     }
-    if let Some(var_8) = &input.string_attribute_constraints {
+    if let Some(var_5) = &input.string_attribute_constraints {
         #[allow(unused_mut)]
-        let mut object_9 = object.key("StringAttributeConstraints").start_object();
-        super::super::protocol_serde::shape_string_attribute_constraints_type::ser_string_attribute_constraints_type(&mut object_9, var_8)?;
-        object_9.finish();
+        let mut object_6 = object.key("StringAttributeConstraints").start_object();
+        super::super::protocol_serde::shape_string_attribute_constraints_type::ser_string_attribute_constraints_type(&mut object_6, var_5)?;
+        object_6.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_set_ui_customization.rs`

```diff
--- reference/src/protocol_serde/shape_set_ui_customization.rs
+++ generated/src/protocol_serde/shape_set_ui_customization.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::set_ui_customization::SetUiCustomizationOutput,
-    super::super::operation::set_ui_customization::SetUICustomizationError,
+    super::super::operation::set_ui_customization::SetUiCustomizationError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+        .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled(generic)),
+        None => return Err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalErrorException" => super::super::operation::set_ui_customization::SetUICustomizationError::InternalErrorException({
+        "InternalErrorException" => super::super::operation::set_ui_customization::SetUiCustomizationError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InvalidParameterException" => super::super::operation::set_ui_customization::SetUICustomizationError::InvalidParameterException({
+        "InvalidParameterException" => super::super::operation::set_ui_customization::SetUiCustomizationError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "NotAuthorizedException" => super::super::operation::set_ui_customization::SetUICustomizationError::NotAuthorizedException({
+        "NotAuthorizedException" => super::super::operation::set_ui_customization::SetUiCustomizationError::NotAuthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::NotAuthorizedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,7 +65,7 @@
             }
             tmp
         }),
-        "OperationNotEnabledException" => super::super::operation::set_ui_customization::SetUICustomizationError::OperationNotEnabledException({
+        "OperationNotEnabledException" => super::super::operation::set_ui_customization::SetUiCustomizationError::OperationNotEnabledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -72,7 +72,7 @@
                 let mut output = super::super::types::error::builders::OperationNotEnabledExceptionBuilder::default();
                 output =
                     super::super::protocol_serde::shape_operation_not_enabled_exception::de_operation_not_enabled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+                        .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -81,13 +81,13 @@
             }
             tmp
         }),
-        "ResourceNotFoundException" => super::super::operation::set_ui_customization::SetUICustomizationError::ResourceNotFoundException({
+        "ResourceNotFoundException" => super::super::operation::set_ui_customization::SetUiCustomizationError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -96,13 +96,13 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => super::super::operation::set_ui_customization::SetUICustomizationError::TooManyRequestsException({
+        "TooManyRequestsException" => super::super::operation::set_ui_customization::SetUiCustomizationError::TooManyRequestsException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+                    .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -111,7 +111,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::set_ui_customization::SetUICustomizationError::generic(generic),
+        _ => super::super::operation::set_ui_customization::SetUiCustomizationError::generic(generic),
     })
 }

@@ -122,13 +122,13 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::set_ui_customization::SetUiCustomizationOutput,
-    super::super::operation::set_ui_customization::SetUICustomizationError,
+    super::super::operation::set_ui_customization::SetUiCustomizationError,
 > {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::set_ui_customization::builders::SetUiCustomizationOutputBuilder::default();
         output = super::super::protocol_serde::shape_set_ui_customization::de_set_ui_customization(_response_body, output)
-            .map_err(super::super::operation::set_ui_customization::SetUICustomizationError::unhandled)?;
+            .map_err(super::super::operation::set_ui_customization::SetUiCustomizationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         super::super::serde_util::set_ui_customization_output_output_correct_errors(output).build()
     })
```

### `src/protocol_serde/shape_set_user_mfa_preference.rs`

```diff
--- reference/src/protocol_serde/shape_set_user_mfa_preference.rs
+++ generated/src/protocol_serde/shape_set_user_mfa_preference.rs
@@ -6,27 +6,27 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
-    super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
+    super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
 > {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+        .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled(generic)),
+        None => return Err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ForbiddenException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::ForbiddenException({
+        "ForbiddenException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::ForbiddenException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ForbiddenExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -35,13 +35,13 @@
             }
             tmp
         }),
-        "InternalErrorException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::InternalErrorException({
+        "InternalErrorException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::InternalErrorException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InternalErrorExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -50,13 +50,13 @@
             }
             tmp
         }),
-        "InvalidParameterException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::InvalidParameterException({
+        "InvalidParameterException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::InvalidParameterException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidParameterExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,13 +65,13 @@
             }
             tmp
         }),
-        "NotAuthorizedException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::NotAuthorizedException({
+        "NotAuthorizedException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::NotAuthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::NotAuthorizedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,7 +80,7 @@
             }
             tmp
         }),
-        "OperationNotEnabledException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::OperationNotEnabledException({
+        "OperationNotEnabledException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::OperationNotEnabledException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -87,7 +87,7 @@
                 let mut output = super::super::types::error::builders::OperationNotEnabledExceptionBuilder::default();
                 output =
                     super::super::protocol_serde::shape_operation_not_enabled_exception::de_operation_not_enabled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                        .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -96,7 +96,7 @@
             }
             tmp
         }),
-        "PasswordResetRequiredException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::PasswordResetRequiredException({
+        "PasswordResetRequiredException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::PasswordResetRequiredException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
@@ -105,7 +105,7 @@
                     _response_body,
                     output,
                 )
-                .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -114,13 +114,13 @@
             }
             tmp
         }),
-        "ResourceNotFoundException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::ResourceNotFoundException({
+        "ResourceNotFoundException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::ResourceNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -129,13 +129,13 @@
             }
             tmp
         }),
-        "UserNotConfirmedException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::UserNotConfirmedException({
+        "UserNotConfirmedException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::UserNotConfirmedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UserNotConfirmedExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_user_not_confirmed_exception::de_user_not_confirmed_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -144,13 +144,13 @@
             }
             tmp
         }),
-        "UserNotFoundException" => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::UserNotFoundException({
+        "UserNotFoundException" => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::UserNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UserNotFoundExceptionBuilder::default();
                 output = super::super::protocol_serde::shape_user_not_found_exception::de_user_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::unhandled)?;
+                    .map_err(super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -159,7 +159,7 @@
             }
             tmp
         }),
-        _ => super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError::generic(generic),
+        _ => super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError::generic(generic),
     })
 }

@@ -170,7 +170,7 @@
     _response_body: &[u8],
 ) -> std::result::Result<
     super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceOutput,
-    super::super::operation::set_user_mfa_preference::SetUserMFAPreferenceError,
+    super::super::operation::set_user_mfa_preference::SetUserMfaPreferenceError,
 > {
     Ok({
         #[allow(unused_mut)]
@@ -189,3 +189,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_set_user_mfa_preference(
+    _value: &[u8],
+    mut builder: super::super::operation::set_user_mfa_preference::builders::SetUserMfaPreferenceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::set_user_mfa_preference::builders::SetUserMfaPreferenceOutputBuilder,
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

### `src/protocol_serde/shape_set_user_settings.rs`

```diff
--- reference/src/protocol_serde/shape_set_user_settings.rs
+++ generated/src/protocol_serde/shape_set_user_settings.rs
@@ -183,3 +183,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_set_user_settings(
+    _value: &[u8],
+    mut builder: super::super::operation::set_user_settings::builders::SetUserSettingsOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::set_user_settings::builders::SetUserSettingsOutputBuilder,
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

### `src/protocol_serde/shape_sms_mfa_config_type.rs`

```diff
--- reference/src/protocol_serde/shape_sms_mfa_config_type.rs
+++ generated/src/protocol_serde/shape_sms_mfa_config_type.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_sms_mfa_config_type(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::SmsMfaConfigType,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.sms_authentication_message {
+        object.key("SmsAuthenticationMessage").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.sms_configuration {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("SmsConfiguration").start_object();
+        super::super::protocol_serde::shape_sms_configuration_type::ser_sms_configuration_type(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_sms_mfa_config_type<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -51,19 +67,3 @@
         )),
     }
 }
-
-pub fn ser_sms_mfa_config_type(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::SmsMfaConfigType,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.sms_authentication_message {
-        object.key("SmsAuthenticationMessage").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.sms_configuration {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("SmsConfiguration").start_object();
-        super::super::protocol_serde::shape_sms_configuration_type::ser_sms_configuration_type(&mut object_3, var_2)?;
-        object_3.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_sms_mfa_settings_type.rs`

```diff
--- reference/src/protocol_serde/shape_sms_mfa_settings_type.rs
+++ generated/src/protocol_serde/shape_sms_mfa_settings_type.rs
@@ -3,10 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::SmsMfaSettingsType,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.enabled {
+    {
         object.key("Enabled").boolean(input.enabled);
     }
-    if input.preferred_mfa {
+    {
         object.key("PreferredMfa").boolean(input.preferred_mfa);
     }
     Ok(())
```

### `src/protocol_serde/shape_software_token_mfa_config_type.rs`

```diff
--- reference/src/protocol_serde/shape_software_token_mfa_config_type.rs
+++ generated/src/protocol_serde/shape_software_token_mfa_config_type.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_software_token_mfa_config_type(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::SoftwareTokenMfaConfigType,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("Enabled").boolean(input.enabled);
+    }
+    Ok(())
+}
+
 pub(crate) fn de_software_token_mfa_config_type<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -40,13 +50,3 @@
         )),
     }
 }
-
-pub fn ser_software_token_mfa_config_type(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::SoftwareTokenMfaConfigType,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.enabled {
-        object.key("Enabled").boolean(input.enabled);
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_software_token_mfa_settings_type.rs`

```diff
--- reference/src/protocol_serde/shape_software_token_mfa_settings_type.rs
+++ generated/src/protocol_serde/shape_software_token_mfa_settings_type.rs
@@ -3,10 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::SoftwareTokenMfaSettingsType,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.enabled {
+    {
         object.key("Enabled").boolean(input.enabled);
     }
-    if input.preferred_mfa {
+    {
         object.key("PreferredMfa").boolean(input.preferred_mfa);
     }
     Ok(())
```

### `src/protocol_serde/shape_start_web_authn_registration.rs`

```diff
--- reference/src/protocol_serde/shape_start_web_authn_registration.rs
+++ generated/src/protocol_serde/shape_start_web_authn_registration.rs
@@ -242,7 +242,7 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "CredentialCreationOptions" => {
-                    builder = builder.set_credential_creation_options(Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?));
+                    builder = builder.set_credential_creation_options(Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?));
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_tag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource.rs
+++ generated/src/protocol_serde/shape_tag_resource.rs
@@ -135,3 +135,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_tag_resource(
+    _value: &[u8],
+    mut builder: super::super::operation::tag_resource::builders::TagResourceOutputBuilder,
+) -> ::std::result::Result<super::super::operation::tag_resource::builders::TagResourceOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
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

### `src/protocol_serde/shape_untag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource.rs
+++ generated/src/protocol_serde/shape_untag_resource.rs
@@ -135,3 +135,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_untag_resource(
+    _value: &[u8],
+    mut builder: super::super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::untag_resource::builders::UntagResourceOutputBuilder,
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

### `src/protocol_serde/shape_update_auth_event_feedback.rs`

```diff
--- reference/src/protocol_serde/shape_update_auth_event_feedback.rs
+++ generated/src/protocol_serde/shape_update_auth_event_feedback.rs
@@ -180,3 +180,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_auth_event_feedback(
+    _value: &[u8],
+    mut builder: super::super::operation::update_auth_event_feedback::builders::UpdateAuthEventFeedbackOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_auth_event_feedback::builders::UpdateAuthEventFeedbackOutputBuilder,
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

### `src/protocol_serde/shape_update_device_status.rs`

```diff
--- reference/src/protocol_serde/shape_update_device_status.rs
+++ generated/src/protocol_serde/shape_update_device_status.rs
@@ -220,3 +220,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_device_status(
+    _value: &[u8],
+    mut builder: super::super::operation::update_device_status::builders::UpdateDeviceStatusOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_device_status::builders::UpdateDeviceStatusOutputBuilder,
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

### `src/protocol_serde/shape_update_user_pool.rs`

```diff
--- reference/src/protocol_serde/shape_update_user_pool.rs
+++ generated/src/protocol_serde/shape_update_user_pool.rs
@@ -272,3 +272,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_user_pool(
+    _value: &[u8],
+    mut builder: super::super::operation::update_user_pool::builders::UpdateUserPoolOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_user_pool::builders::UpdateUserPoolOutputBuilder,
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

### `src/protocol_serde/shape_update_user_pool_client_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_user_pool_client_input.rs
+++ generated/src/protocol_serde/shape_update_user_pool_client_input.rs
@@ -72,7 +72,7 @@
         }
         array_19.finish();
     }
-    if let Some(var_21) = &input.callback_urls {
+    if let Some(var_21) = &input.callback_ur_ls {
         let mut array_22 = object.key("CallbackURLs").start_array();
         for item_23 in var_21 {
             {
@@ -81,7 +81,7 @@
         }
         array_22.finish();
     }
-    if let Some(var_24) = &input.logout_urls {
+    if let Some(var_24) = &input.logout_ur_ls {
         let mut array_25 = object.key("LogoutURLs").start_array();
         for item_26 in var_24 {
             {
```

### `src/protocol_serde/shape_user_pool_client_type.rs`

```diff
--- reference/src/protocol_serde/shape_user_pool_client_type.rs
+++ generated/src/protocol_serde/shape_user_pool_client_type.rs
@@ -116,7 +116,7 @@
                             );
                         }
                         "CallbackURLs" => {
-                            builder = builder.set_callback_urls(super::super::protocol_serde::shape_callback_urls_list_type::de_callback_urls_list_type(
+                            builder = builder.set_callback_ur_ls(super::super::protocol_serde::shape_callback_urls_list_type::de_callback_urls_list_type(
                                 tokens,
                                 _value,
                                 depth + 1,
@@ -123,7 +123,7 @@
                             )?);
                         }
                         "LogoutURLs" => {
-                            builder = builder.set_logout_urls(super::super::protocol_serde::shape_logout_urls_list_type::de_logout_urls_list_type(
+                            builder = builder.set_logout_ur_ls(super::super::protocol_serde::shape_logout_urls_list_type::de_logout_urls_list_type(
                                 tokens,
                                 _value,
                                 depth + 1,
```

### `src/protocol_serde/shape_verify_user_attribute.rs`

```diff
--- reference/src/protocol_serde/shape_verify_user_attribute.rs
+++ generated/src/protocol_serde/shape_verify_user_attribute.rs
@@ -264,3 +264,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_verify_user_attribute(
+    _value: &[u8],
+    mut builder: super::super::operation::verify_user_attribute::builders::VerifyUserAttributeOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::verify_user_attribute::builders::VerifyUserAttributeOutputBuilder,
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

### `src/protocol_serde/shape_web_authn_configuration_type.rs`

```diff
--- reference/src/protocol_serde/shape_web_authn_configuration_type.rs
+++ generated/src/protocol_serde/shape_web_authn_configuration_type.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_web_authn_configuration_type(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::WebAuthnConfigurationType,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.relying_party_id {
+        object.key("RelyingPartyId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.user_verification {
+        object.key("UserVerification").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.factor_configuration {
+        object.key("FactorConfiguration").string(var_3.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_web_authn_configuration_type<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -58,19 +74,3 @@
         )),
     }
 }
-
-pub fn ser_web_authn_configuration_type(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::WebAuthnConfigurationType,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.relying_party_id {
-        object.key("RelyingPartyId").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.user_verification {
-        object.key("UserVerification").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.factor_configuration {
-        object.key("FactorConfiguration").string(var_3.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_web_authn_mfa_settings_type.rs`

```diff
--- reference/src/protocol_serde/shape_web_authn_mfa_settings_type.rs
+++ generated/src/protocol_serde/shape_web_authn_mfa_settings_type.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::WebAuthnMfaSettingsType,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.enabled {
+    {
         object.key("Enabled").boolean(input.enabled);
     }
     Ok(())
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -119,7 +119,7 @@
     if builder.limit.is_none() {
         builder.limit = {
             let builder = super::types::builders::LimitTypeBuilder::default();
-            Some(super::serde_util::limit_type_correct_errors(builder).build())
+            super::serde_util::limit_type_correct_errors(builder).build().ok()
         }
     }
     builder
@@ -257,7 +257,7 @@
     if builder.limit.is_none() {
         builder.limit = {
             let builder = super::types::builders::LimitTypeBuilder::default();
-            Some(super::serde_util::limit_type_correct_errors(builder).build())
+            super::serde_util::limit_type_correct_errors(builder).build().ok()
         }
     }
     builder
@@ -291,6 +291,18 @@
     builder
 }

+pub(crate) fn limit_definition_type_correct_errors(
+    mut builder: super::types::builders::LimitDefinitionTypeBuilder,
+) -> super::types::builders::LimitDefinitionTypeBuilder {
+    if builder.limit_class.is_none() {
+        builder.limit_class = "no value was set".parse::<super::types::LimitClass>().ok()
+    }
+    if builder.attributes.is_none() {
+        builder.attributes = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn log_delivery_configuration_type_correct_errors(
     mut builder: super::types::builders::LogDeliveryConfigurationTypeBuilder,
 ) -> super::types::builders::LogDeliveryConfigurationTypeBuilder {
@@ -334,18 +346,6 @@
     builder
 }

-pub(crate) fn limit_definition_type_correct_errors(
-    mut builder: super::types::builders::LimitDefinitionTypeBuilder,
-) -> super::types::builders::LimitDefinitionTypeBuilder {
-    if builder.limit_class.is_none() {
-        builder.limit_class = "no value was set".parse::<super::types::LimitClass>().ok()
-    }
-    if builder.attributes.is_none() {
-        builder.attributes = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn account_takeover_risk_configuration_type_correct_errors(
     mut builder: super::types::builders::AccountTakeoverRiskConfigurationTypeBuilder,
 ) -> super::types::builders::AccountTakeoverRiskConfigurationTypeBuilder {
```

### `src/types/_managed_login_branding_type.rs`

```diff
--- reference/src/types/_managed_login_branding_type.rs
+++ generated/src/types/_managed_login_branding_type.rs
@@ -23,7 +23,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub settings: ::std::option::Option<::aws_smithy_types::Document>,
+    pub settings: ::std::option::Option<::std::string::String>,
     /// <p>An array of image files that you want to apply to roles like backgrounds, logos, and icons. Each object must also indicate whether it is for dark mode, light mode, or browser-adaptive mode.</p>
     pub assets: ::std::option::Option<::std::vec::Vec<super::super::types::AssetType>>,
     /// <p>The date and time when the item was created. Amazon Cognito returns this timestamp in UNIX epoch time format. Your SDK might render the output in a human-readable format like ISO 8601 or a Java <code>Date</code> object.</p>
@@ -57,7 +57,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
+    pub fn settings(&self) -> ::std::option::Option<&::std::string::String> {
         self.settings.as_ref()
     }
     /// <p>An array of image files that you want to apply to roles like backgrounds, logos, and icons. Each object must also indicate whether it is for dark mode, light mode, or browser-adaptive mode.</p>
@@ -89,7 +89,7 @@
     pub(crate) managed_login_branding_id: ::std::option::Option<::std::string::String>,
     pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
     pub(crate) use_cognito_provided_values: ::std::option::Option<bool>,
-    pub(crate) settings: ::std::option::Option<::aws_smithy_types::Document>,
+    pub(crate) settings: ::std::option::Option<::std::string::String>,
     pub(crate) assets: ::std::option::Option<::std::vec::Vec<super::super::types::AssetType>>,
     pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
     pub(crate) last_modified_date: ::std::option::Option<::aws_smithy_types::DateTime>,
@@ -152,7 +152,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn settings(mut self, input: ::aws_smithy_types::Document) -> Self {
+    pub fn settings(mut self, input: ::std::string::String) -> Self {
         self.settings = ::std::option::Option::Some(input);
         self
     }
@@ -168,7 +168,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn set_settings(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
+    pub fn set_settings(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.settings = input;
         self
     }
@@ -184,7 +184,7 @@
     /// <li>
     /// <p><code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a></p></li>
     /// </ul>
-    pub fn get_settings(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
+    pub fn get_settings(&self) -> &::std::option::Option<::std::string::String> {
         &self.settings
     }
     /// Appends an item to `assets`.
```

### `src/types/_schema_attribute_type.rs`

```diff
--- reference/src/types/_schema_attribute_type.rs
+++ generated/src/types/_schema_attribute_type.rs
@@ -13,12 +13,12 @@
     /// <p>You should use <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_UserPoolClientType.html#CognitoUserPools-Type-UserPoolClientType-WriteAttributes">WriteAttributes</a> in the user pool client to control how attributes can be mutated for new use cases instead of using <code>DeveloperOnlyAttribute</code>.</p>
     /// </note>
     /// <p>Specifies whether the attribute type is developer only. This attribute can only be modified by an administrator. Users won't be able to modify this attribute using their access token. For example, <code>DeveloperOnlyAttribute</code> can be modified using AdminUpdateUserAttributes but can't be updated using UpdateUserAttributes.</p>
-    pub developer_only_attribute: ::std::option::Option<bool>,
+    pub developer_only_attribute: bool,
     /// <p>Specifies whether the value of the attribute can be changed.</p>
     /// <p>Any user pool attribute whose value you map from an IdP attribute must be mutable, with a parameter value of <code>true</code>. Amazon Cognito updates mapped attributes when users sign in to your application through an IdP. If an attribute is immutable, Amazon Cognito throws an error when it attempts to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying Identity Provider Attribute Mappings for Your User Pool</a>.</p>
-    pub mutable: ::std::option::Option<bool>,
+    pub mutable: bool,
     /// <p>Specifies whether a user pool attribute is required. If the attribute is required and the user doesn't provide a value, registration or sign-in will fail.</p>
-    pub required: ::std::option::Option<bool>,
+    pub required: bool,
     /// <p>Specifies the constraints for an attribute of the number type.</p>
     pub number_attribute_constraints: ::std::option::Option<super::super::types::NumberAttributeConstraintsType>,
     /// <p>Specifies the constraints for an attribute of the string type.</p>
@@ -37,16 +37,16 @@
     /// <p>You should use <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_UserPoolClientType.html#CognitoUserPools-Type-UserPoolClientType-WriteAttributes">WriteAttributes</a> in the user pool client to control how attributes can be mutated for new use cases instead of using <code>DeveloperOnlyAttribute</code>.</p>
     /// </note>
     /// <p>Specifies whether the attribute type is developer only. This attribute can only be modified by an administrator. Users won't be able to modify this attribute using their access token. For example, <code>DeveloperOnlyAttribute</code> can be modified using AdminUpdateUserAttributes but can't be updated using UpdateUserAttributes.</p>
-    pub fn developer_only_attribute(&self) -> ::std::option::Option<bool> {
+    pub fn developer_only_attribute(&self) -> bool {
         self.developer_only_attribute
     }
     /// <p>Specifies whether the value of the attribute can be changed.</p>
     /// <p>Any user pool attribute whose value you map from an IdP attribute must be mutable, with a parameter value of <code>true</code>. Amazon Cognito updates mapped attributes when users sign in to your application through an IdP. If an attribute is immutable, Amazon Cognito throws an error when it attempts to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying Identity Provider Attribute Mappings for Your User Pool</a>.</p>
-    pub fn mutable(&self) -> ::std::option::Option<bool> {
+    pub fn mutable(&self) -> bool {
         self.mutable
     }
     /// <p>Specifies whether a user pool attribute is required. If the attribute is required and the user doesn't provide a value, registration or sign-in will fail.</p>
-    pub fn required(&self) -> ::std::option::Option<bool> {
+    pub fn required(&self) -> bool {
         self.required
     }
     /// <p>Specifies the constraints for an attribute of the number type.</p>
@@ -193,9 +193,9 @@
         super::super::types::SchemaAttributeType {
             name: self.name,
             attribute_data_type: self.attribute_data_type,
-            developer_only_attribute: self.developer_only_attribute,
-            mutable: self.mutable,
-            required: self.required,
+            developer_only_attribute: self.developer_only_attribute.unwrap_or_default(),
+            mutable: self.mutable.unwrap_or_default(),
+            required: self.required.unwrap_or_default(),
             number_attribute_constraints: self.number_attribute_constraints,
             string_attribute_constraints: self.string_attribute_constraints,
         }
```

### `src/types/_user_pool_client_type.rs`

```diff
--- reference/src/types/_user_pool_client_type.rs
+++ generated/src/types/_user_pool_client_type.rs
@@ -77,9 +77,9 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as myapp://example are also supported.</p>
-    pub callback_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub callback_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>A list of allowed logout URLs for the IdPs.</p>
-    pub logout_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub logout_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p>
     /// <p>A redirect URI must:</p>
     /// <ul>
@@ -131,7 +131,7 @@
     /// <p><code>AllowedOAuthFlows</code>: Support for authorization code, implicit, and client credentials OAuth 2.0 grants.</p></li>
     /// </ul>
     /// <p>To use authorization server features, configure one of these features in the Amazon Cognito console or set <code>AllowedOAuthFlowsUserPoolClient</code> to <code>true</code> in a <code>CreateUserPoolClient</code> or <code>UpdateUserPoolClient</code> API request. If you don't set a value for <code>AllowedOAuthFlowsUserPoolClient</code> in a request with the CLI or SDKs, it defaults to <code>false</code>. When <code>false</code>, only SDK-based API sign-in is permitted.</p>
-    pub allowed_o_auth_flows_user_pool_client: ::std::option::Option<bool>,
+    pub allowed_o_auth_flows_user_pool_client: bool,
     /// <p>The user pool analytics configuration for collecting metrics and sending them to your Amazon Pinpoint campaign.</p><note>
     /// <p>In Amazon Web Services Regions where Amazon Pinpoint isn't available, user pools only support sending events to Amazon Pinpoint projects in Amazon Web Services Region us-east-1. In Regions where Amazon Pinpoint is available, user pools support sending events to Amazon Pinpoint projects within that same Region.</p>
     /// </note>
@@ -261,15 +261,15 @@
     /// <p>Amazon Cognito requires HTTPS over HTTP for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as myapp://example are also supported.</p>
     ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.callback_urls.is_none()`.
-    pub fn callback_urls(&self) -> &[::std::string::String] {
-        self.callback_urls.as_deref().unwrap_or_default()
+    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.callback_ur_ls.is_none()`.
+    pub fn callback_ur_ls(&self) -> &[::std::string::String] {
+        self.callback_ur_ls.as_deref().unwrap_or_default()
     }
     /// <p>A list of allowed logout URLs for the IdPs.</p>
     ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.logout_urls.is_none()`.
-    pub fn logout_urls(&self) -> &[::std::string::String] {
-        self.logout_urls.as_deref().unwrap_or_default()
+    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.logout_ur_ls.is_none()`.
+    pub fn logout_ur_ls(&self) -> &[::std::string::String] {
+        self.logout_ur_ls.as_deref().unwrap_or_default()
     }
     /// <p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p>
     /// <p>A redirect URI must:</p>
@@ -332,7 +332,7 @@
     /// <p><code>AllowedOAuthFlows</code>: Support for authorization code, implicit, and client credentials OAuth 2.0 grants.</p></li>
     /// </ul>
     /// <p>To use authorization server features, configure one of these features in the Amazon Cognito console or set <code>AllowedOAuthFlowsUserPoolClient</code> to <code>true</code> in a <code>CreateUserPoolClient</code> or <code>UpdateUserPoolClient</code> API request. If you don't set a value for <code>AllowedOAuthFlowsUserPoolClient</code> in a request with the CLI or SDKs, it defaults to <code>false</code>. When <code>false</code>, only SDK-based API sign-in is permitted.</p>
-    pub fn allowed_o_auth_flows_user_pool_client(&self) -> ::std::option::Option<bool> {
+    pub fn allowed_o_auth_flows_user_pool_client(&self) -> bool {
         self.allowed_o_auth_flows_user_pool_client
     }
     /// <p>The user pool analytics configuration for collecting metrics and sending them to your Amazon Pinpoint campaign.</p><note>
@@ -382,8 +382,8 @@
         formatter.field("write_attributes", &self.write_attributes);
         formatter.field("explicit_auth_flows", &self.explicit_auth_flows);
         formatter.field("supported_identity_providers", &self.supported_identity_providers);
-        formatter.field("callback_urls", &self.callback_urls);
-        formatter.field("logout_urls", &self.logout_urls);
+        formatter.field("callback_ur_ls", &self.callback_ur_ls);
+        formatter.field("logout_ur_ls", &self.logout_ur_ls);
         formatter.field("default_redirect_uri", &self.default_redirect_uri);
         formatter.field("allowed_o_auth_flows", &self.allowed_o_auth_flows);
         formatter.field("allowed_o_auth_scopes", &self.allowed_o_auth_scopes);
@@ -425,8 +425,8 @@
     pub(crate) write_attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) explicit_auth_flows: ::std::option::Option<::std::vec::Vec<super::super::types::ExplicitAuthFlowsType>>,
     pub(crate) supported_identity_providers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) callback_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) logout_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub(crate) callback_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub(crate) logout_ur_ls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) default_redirect_uri: ::std::option::Option<::std::string::String>,
     pub(crate) allowed_o_auth_flows: ::std::option::Option<::std::vec::Vec<super::super::types::OAuthFlowType>>,
     pub(crate) allowed_o_auth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -755,9 +755,9 @@
     pub fn get_supported_identity_providers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
         &self.supported_identity_providers
     }
-    /// Appends an item to `callback_urls`.
+    /// Appends an item to `callback_ur_ls`.
     ///
-    /// To override the contents of this collection use [`set_callback_urls`](Self::set_callback_urls).
+    /// To override the contents of this collection use [`set_callback_ur_ls`](Self::set_callback_ur_ls).
     ///
     /// <p>A list of allowed redirect (callback) URLs for the IdPs.</p>
     /// <p>A redirect URI must:</p>
@@ -772,10 +772,10 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as myapp://example are also supported.</p>
-    pub fn callback_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        let mut v = self.callback_urls.unwrap_or_default();
+    pub fn callback_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        let mut v = self.callback_ur_ls.unwrap_or_default();
         v.push(input.into());
-        self.callback_urls = ::std::option::Option::Some(v);
+        self.callback_ur_ls = ::std::option::Option::Some(v);
         self
     }
     /// <p>A list of allowed redirect (callback) URLs for the IdPs.</p>
@@ -791,8 +791,8 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as myapp://example are also supported.</p>
-    pub fn set_callback_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.callback_urls = input;
+    pub fn set_callback_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.callback_ur_ls = input;
         self
     }
     /// <p>A list of allowed redirect (callback) URLs for the IdPs.</p>
@@ -808,28 +808,28 @@
     /// <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p>
     /// <p>Amazon Cognito requires HTTPS over HTTP for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p>
     /// <p>App callback URLs such as myapp://example are also supported.</p>
-    pub fn get_callback_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        &self.callback_urls
+    pub fn get_callback_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        &self.callback_ur_ls
     }
-    /// Appends an item to `logout_urls`.
+    /// Appends an item to `logout_ur_ls`.
     ///
-    /// To override the contents of this collection use [`set_logout_urls`](Self::set_logout_urls).
+    /// To override the contents of this collection use [`set_logout_ur_ls`](Self::set_logout_ur_ls).
     ///
     /// <p>A list of allowed logout URLs for the IdPs.</p>
-    pub fn logout_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
-        let mut v = self.logout_urls.unwrap_or_default();
+    pub fn logout_ur_ls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+        let mut v = self.logout_ur_ls.unwrap_or_default();
         v.push(input.into());
-        self.logout_urls = ::std::option::Option::Some(v);
+        self.logout_ur_ls = ::std::option::Option::Some(v);
         self
     }
     /// <p>A list of allowed logout URLs for the IdPs.</p>
-    pub fn set_logout_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
-        self.logout_urls = input;
+    pub fn set_logout_ur_ls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+        self.logout_ur_ls = input;
         self
     }
     /// <p>A list of allowed logout URLs for the IdPs.</p>
-    pub fn get_logout_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
-        &self.logout_urls
+    pub fn get_logout_ur_ls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
+        &self.logout_ur_ls
     }
     /// <p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p>
     /// <p>A redirect URI must:</p>
@@ -1147,12 +1147,12 @@
             write_attributes: self.write_attributes,
             explicit_auth_flows: self.explicit_auth_flows,
             supported_identity_providers: self.supported_identity_providers,
-            callback_urls: self.callback_urls,
-            logout_urls: self.logout_urls,
+            callback_ur_ls: self.callback_ur_ls,
+            logout_ur_ls: self.logout_ur_ls,
             default_redirect_uri: self.default_redirect_uri,
             allowed_o_auth_flows: self.allowed_o_auth_flows,
             allowed_o_auth_scopes: self.allowed_o_auth_scopes,
-            allowed_o_auth_flows_user_pool_client: self.allowed_o_auth_flows_user_pool_client,
+            allowed_o_auth_flows_user_pool_client: self.allowed_o_auth_flows_user_pool_client.unwrap_or_default(),
             analytics_configuration: self.analytics_configuration,
             prevent_user_existence_errors: self.prevent_user_existence_errors,
             enable_token_revocation: self.enable_token_revocation,
@@ -1179,8 +1179,8 @@
         formatter.field("write_attributes", &self.write_attributes);
         formatter.field("explicit_auth_flows", &self.explicit_auth_flows);
         formatter.field("supported_identity_providers", &self.supported_identity_providers);
-        formatter.field("callback_urls", &self.callback_urls);
-        formatter.field("logout_urls", &self.logout_urls);
+        formatter.field("callback_ur_ls", &self.callback_ur_ls);
+        formatter.field("logout_ur_ls", &self.logout_ur_ls);
         formatter.field("default_redirect_uri", &self.default_redirect_uri);
         formatter.field("allowed_o_auth_flows", &self.allowed_o_auth_flows);
         formatter.field("allowed_o_auth_scopes", &self.allowed_o_auth_scopes);
```

### `src/types/_user_status_type.rs`

```diff
--- reference/src/types/_user_status_type.rs
+++ generated/src/types/_user_status_type.rs
@@ -19,7 +19,7 @@
 ///     UserStatusType::ForceChangePassword => { /* ... */ },
 ///     UserStatusType::ResetRequired => { /* ... */ },
 ///     UserStatusType::Unconfirmed => { /* ... */ },
-///     UserStatusType::UnknownValue => { /* ... */ },
+///     UserStatusType::Unknown => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -42,8 +42,7 @@
 /// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
 /// - It might inadvertently shadow other intended match arms.
 ///
-///
-/// _Note: `UserStatusType::Unknown` has been renamed to `::UnknownValue`._
+#[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
@@ -63,9 +62,8 @@
     ResetRequired,
     #[allow(missing_docs)] // documentation missing in model
     Unconfirmed,
-    ///
-    /// _Note: `::Unknown` has been renamed to `::UnknownValue`._
-    UnknownValue,
+    #[allow(missing_docs)] // documentation missing in model
+    Unknown,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -80,7 +78,7 @@
             "FORCE_CHANGE_PASSWORD" => UserStatusType::ForceChangePassword,
             "RESET_REQUIRED" => UserStatusType::ResetRequired,
             "UNCONFIRMED" => UserStatusType::Unconfirmed,
-            "UNKNOWN" => UserStatusType::UnknownValue,
+            "UNKNOWN" => UserStatusType::Unknown,
             other => UserStatusType::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -103,7 +101,7 @@
             UserStatusType::ForceChangePassword => "FORCE_CHANGE_PASSWORD",
             UserStatusType::ResetRequired => "RESET_REQUIRED",
             UserStatusType::Unconfirmed => "UNCONFIRMED",
-            UserStatusType::UnknownValue => "UNKNOWN",
+            UserStatusType::Unknown => "UNKNOWN",
             UserStatusType::Unknown(value) => value.as_str(),
         }
     }
@@ -148,7 +146,7 @@
             UserStatusType::ForceChangePassword => write!(f, "FORCE_CHANGE_PASSWORD"),
             UserStatusType::ResetRequired => write!(f, "RESET_REQUIRED"),
             UserStatusType::Unconfirmed => write!(f, "UNCONFIRMED"),
-            UserStatusType::UnknownValue => write!(f, "UNKNOWN"),
+            UserStatusType::Unknown => write!(f, "UNKNOWN"),
             UserStatusType::Unknown(value) => write!(f, "{value}"),
         }
     }
```
