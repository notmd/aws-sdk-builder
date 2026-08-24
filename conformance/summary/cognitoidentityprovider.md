# AWS SDK Conformance Report: cognitoidentityprovider

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## cognitoidentityprovider
**Progress:** `1361/1361` files compared · `1340` matched · `21` mismatches · `0` missing · `0` extra · `98.46%` match (100.00% means fully matched)

### `src/client/admin_initiate_auth.rs`

```diff
--- reference/src/client/admin_initiate_auth.rs
+++ generated/src/client/admin_initiate_auth.rs
@@ -6,8 +6,8 @@
     ///   - [`user_pool_id(impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool where the user wants to sign in.</p><br>
     ///   - [`client_id(impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::client_id) / [`set_client_id(Option<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_client_id):<br>required: **true**<br><p>The ID of the app client where the user wants to sign in.</p><br>
     ///   - [`auth_flow(AuthFlowType)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_flow) / [`set_auth_flow(Option<AuthFlowType>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_flow):<br>required: **true**<br><p>The authentication flow that you want to initiate. Each <code>AuthFlow</code> has linked <code>AuthParameters</code> that you must submit. The following are some example flows.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <p>The entry point for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a> with passwords, one-time passwords, and WebAuthn authenticators. Request a preferred authentication type or review available authentication types. From the offered authentication types, select one in a challenge response and then authenticate with that method in an additional challenge response. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <p>Username-password authentication with the Secure Remote Password (SRP) protocol. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow.html#Using-SRP-password-verification-in-custom-authentication-flow">Use SRP password verification in custom authentication flow</a>.</p>  </dd>  <dt>   REFRESH_TOKEN_AUTH and REFRESH_TOKEN  </dt>  <dd>   <p>Receive new ID and access tokens when you pass a <code>REFRESH_TOKEN</code> parameter with a valid refresh token as the value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html">Using the refresh token</a>.</p>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <p>Custom authentication with Lambda triggers. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-challenge.html">Custom authentication challenge Lambda triggers</a>.</p>  </dd>  <dt>   ADMIN_USER_PASSWORD_AUTH  </dt>  <dd>   <p>Server-side username-password authentication with the password sent directly in the request. For more information about client-side and server-side authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-public-server-side.html">SDK authorization models</a>.</p>  </dd> </dl><br>
-    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li>   </ul>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>SRP_A</code> (required)</p></li>   </ul>  </dd>  <dt>   ADMIN_USER_PASSWORD_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PASSWORD</code> (required)</p></li>   </ul>  </dd>  <dt>   REFRESH_TOKEN_AUTH/REFRESH_TOKEN  </dt>  <dd>   <ul>    <li>     <p><code>REFRESH_TOKEN</code>(required)</p></li>   </ul>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>ChallengeName: SRP_A</code> (when preceding custom authentication with SRP authentication)</p></li>    <li>     <p><code>SRP_A: (An SRP_A value)</code> (when preceding custom authentication with SRP authentication)</p></li>   </ul>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
-    ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The <code>ClientMetadata</code> value is passed as input to the functions for only the following triggers:</p> <ul>  <li>   <p>Pre signup</p></li>  <li>   <p>Pre authentication</p></li>  <li>   <p>User migration</p></li> </ul> <p>This request also invokes the functions for the following triggers, but doesn't pass <code>ClientMetadata</code>:</p> <ul>  <li>   <p>Post authentication</p></li>  <li>   <p>Custom message</p></li>  <li>   <p>Pre token generation</p></li>  <li>   <p>Create auth challenge</p></li>  <li>   <p>Define auth challenge</p></li>  <li>   <p>Custom email sender</p></li>  <li>   <p>Custom SMS sender</p></li> </ul><note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
+    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::admin_initiate_auth::builders::AdminInitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li> </ul>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>SRP_A</code> (required)</p></li> </ul>  </dd>  <dt>   ADMIN_USER_PASSWORD_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>PASSWORD</code> (required)</p></li> </ul>  </dd>  <dt>   REFRESH_TOKEN_AUTH/REFRESH_TOKEN  </dt>  <dd>   <ul> <li> <p> <code>REFRESH_TOKEN</code>(required)</p></li> </ul>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>ChallengeName: SRP_A</code> (when preceding custom authentication with SRP authentication)</p></li> <li> <p> <code>SRP_A: (An SRP_A value)</code> (when preceding custom authentication with SRP authentication)</p></li> </ul>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
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
+    ///   - [`challenge_responses(impl Into<String>, impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::challenge_responses) / [`set_challenge_responses(Option<HashMap::<String, String>>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_challenge_responses):<br>required: **false**<br><p>The responses to the challenge that you received in the previous request. Each challenge has its own required response parameters. The following examples are partial JSON request bodies that highlight challenge-response parameters.</p><important>  <p>You must provide a SECRET_HASH parameter in all challenge responses to an app client that has a client secret. Include a <code>DEVICE_KEY</code> for device authentication.</p> </important> <dl>  <dt>   SELECT_CHALLENGE  </dt>  <dd>   <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "USERNAME": "\[username\]", "ANSWER": "\[Challenge name\]"}</code></p>   <p>Available challenges are <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, <code>EMAIL_OTP</code>, <code>SMS_OTP</code>, and <code>WEB_AUTHN</code>.</p>   <p>Complete authentication in the <code>SELECT_CHALLENGE</code> response for <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, and <code>WEB_AUTHN</code>:</p>   <ul> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "WEB_AUTHN", "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p> <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></li> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD", "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></li> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD_SRP", "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></li> </ul>   <p>For <code>SMS_OTP</code> and <code>EMAIL_OTP</code>, respond with the username and answer. Your user pool will send a code for the user to submit in the next challenge response.</p>   <ul> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "SMS_OTP", "USERNAME": "\[username\]"}</code></p></li> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "EMAIL_OTP", "USERNAME": "\[username\]"}</code></p></li> </ul>  </dd>  <dt>   WEB_AUTHN  </dt>  <dd>   <p> <code>"ChallengeName": "WEB_AUTHN", "ChallengeResponses": { "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>   <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p>  </dd>  <dt>   PASSWORD  </dt>  <dd>   <p> <code>"ChallengeName": "PASSWORD", "ChallengeResponses": { "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p>  </dd>  <dt>   PASSWORD_SRP  </dt>  <dd>   <p> <code>"ChallengeName": "PASSWORD_SRP", "ChallengeResponses": { "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p>  </dd>  <dt>   SMS_OTP  </dt>  <dd>   <p> <code>"ChallengeName": "SMS_OTP", "ChallengeResponses": {"SMS_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   EMAIL_OTP  </dt>  <dd>   <p> <code>"ChallengeName": "EMAIL_OTP", "ChallengeResponses": {"EMAIL_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   SMS_MFA  </dt>  <dd>   <p> <code>"ChallengeName": "SMS_MFA", "ChallengeResponses": {"SMS_MFA_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   PASSWORD_VERIFIER  </dt>  <dd>   <p>This challenge response is part of the SRP flow. Amazon Cognito requires that your application respond to this challenge within a few seconds. When the response time exceeds this period, your user pool returns a <code>NotAuthorizedException</code> error.</p>   <p> <code>"ChallengeName": "PASSWORD_VERIFIER", "ChallengeResponses": {"PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   CUSTOM_CHALLENGE  </dt>  <dd>   <p> <code>"ChallengeName": "CUSTOM_CHALLENGE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[challenge_answer\]"}</code></p>  </dd>  <dt>   NEW_PASSWORD_REQUIRED  </dt>  <dd>   <p> <code>"ChallengeName": "NEW_PASSWORD_REQUIRED", "ChallengeResponses": {"NEW_PASSWORD": "\[new_password\]", "USERNAME": "\[username\]"}</code></p>   <p>To set any required attributes that <code>InitiateAuth</code> returned in an <code>requiredAttributes</code> parameter, add <code>"userAttributes.\[attribute_name\]": "\[attribute_value\]"</code>. This parameter can also set values for writable attributes that aren't required by your user pool.</p>   <note> <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p> </note>  </dd>  <dt>   SOFTWARE_TOKEN_MFA  </dt>  <dd>   <p> <code>"ChallengeName": "SOFTWARE_TOKEN_MFA", "ChallengeResponses": {"USERNAME": "\[username\]", "SOFTWARE_TOKEN_MFA_CODE": \[authenticator_code\]}</code></p>  </dd>  <dt>   DEVICE_SRP_AUTH  </dt>  <dd>   <p> <code>"ChallengeName": "DEVICE_SRP_AUTH", "ChallengeResponses": {"USERNAME": "\[username\]", "DEVICE_KEY": "\[device_key\]", "SRP_A": "\[srp_a\]"}</code></p>  </dd>  <dt>   DEVICE_PASSWORD_VERIFIER  </dt>  <dd>   <p> <code>"ChallengeName": "DEVICE_PASSWORD_VERIFIER", "ChallengeResponses": {"DEVICE_KEY": "\[device_key\]", "PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   MFA_SETUP  </dt>  <dd>   <p> <code>"ChallengeName": "MFA_SETUP", "ChallengeResponses": {"USERNAME": "\[username\]"}, "SESSION": "\[Session ID from VerifySoftwareToken\]"</code></p>  </dd>  <dt>   SELECT_MFA_TYPE  </dt>  <dd>   <p> <code>"ChallengeName": "SELECT_MFA_TYPE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[SMS_MFA|EMAIL_MFA|SOFTWARE_TOKEN_MFA\]"}</code></p>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
     ///   - [`session(impl Into<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::session) / [`set_session(Option<String>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_session):<br>required: **false**<br><p>The session identifier that maintains the state of authentication requests and challenge responses. If an <code>AdminInitiateAuth</code> or <code>AdminRespondToAuthChallenge</code> API request results in a determination that your application must pass another challenge, Amazon Cognito returns a session with other challenge parameters. Send this session identifier, unmodified, to the next <code>AdminRespondToAuthChallenge</code> request.</p><br>
     ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_analytics_metadata):<br>required: **false**<br><p>Information that supports analytics outcomes with Amazon Pinpoint, including the user's endpoint ID. The endpoint ID is a destination for Amazon Pinpoint push notifications, for example a device identifier, email address, or phone number.</p><br>
     ///   - [`context_data(ContextDataType)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::context_data) / [`set_context_data(Option<ContextDataType>)`](crate::operation::admin_respond_to_auth_challenge::builders::AdminRespondToAuthChallengeFluentBuilder::set_context_data):<br>required: **false**<br><p>Contextual data about your user session like the device fingerprint, IP address, or location. Amazon Cognito threat protection evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-viewing-threat-protection-app.html">Collecting data for threat protection in applications</a>.</p><br>
```

### `src/client/create_user_pool_client.rs`

```diff
--- reference/src/client/create_user_pool_client.rs
+++ generated/src/client/create_user_pool_client.rs
@@ -15,8 +15,8 @@
     ///   - [`write_attributes(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::write_attributes) / [`set_write_attributes(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_write_attributes):<br>required: **false**<br><p>The list of user attributes that you want your app client to have write access to. After your user authenticates in your app, their access token authorizes them to set or modify their own attribute value for any attribute in this list.</p> <p>When you don't specify the <code>WriteAttributes</code> for your app client, your app can write the values of the Standard attributes of your user pool. When your user pool has write access to these default attributes, <code>WriteAttributes</code> doesn't return any information. Amazon Cognito only populates <code>WriteAttributes</code> in the API response if you have specified your own custom set of write attributes.</p> <p>If your app client allows users to sign in through an IdP, this array must include all attributes that you have mapped to IdP attributes. Amazon Cognito updates mapped attributes when users sign in to your application through an IdP. If your app client does not have write access to a mapped attribute, Amazon Cognito throws an error when it tries to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying IdP Attribute Mappings for Your user pool</a>.</p><br>
     ///   - [`explicit_auth_flows(ExplicitAuthFlowsType)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::explicit_auth_flows) / [`set_explicit_auth_flows(Option<Vec::<ExplicitAuthFlowsType>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_explicit_auth_flows):<br>required: **false**<br><p>The <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow-methods.html">authentication flows</a> that you want your user pool client to support. For each app client in your user pool, you can sign in your users with any combination of one or more flows, including with a user name and Secure Remote Password (SRP), a user name and password, or a custom authentication process that you define with Lambda functions.</p><note>  <p>If you don't specify a value for <code>ExplicitAuthFlows</code>, your app client supports <code>ALLOW_REFRESH_TOKEN_AUTH</code>, <code>ALLOW_USER_SRP_AUTH</code>, and <code>ALLOW_CUSTOM_AUTH</code>.</p> </note> <p>The values for authentication flow options include the following.</p> <ul>  <li>   <p><code>ALLOW_USER_AUTH</code>: Enable selection-based sign-in with <code>USER_AUTH</code>. This setting covers username-password, secure remote password (SRP), passwordless, and passkey authentication. This authentiation flow can do username-password and SRP authentication without other <code>ExplicitAuthFlows</code> permitting them. For example users can complete an SRP challenge through <code>USER_AUTH</code> without the flow <code>USER_SRP_AUTH</code> being active for the app client. This flow doesn't include <code>CUSTOM_AUTH</code>.</p>   <p>To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p></li>  <li>   <p><code>ALLOW_ADMIN_USER_PASSWORD_AUTH</code>: Enable admin based user password authentication flow <code>ADMIN_USER_PASSWORD_AUTH</code>. This setting replaces the <code>ADMIN_NO_SRP_AUTH</code> setting. With this authentication flow, your app passes a user name and password to Amazon Cognito in the request, instead of using the Secure Remote Password (SRP) protocol to securely transmit the password.</p></li>  <li>   <p><code>ALLOW_CUSTOM_AUTH</code>: Enable Lambda trigger based authentication.</p></li>  <li>   <p><code>ALLOW_USER_PASSWORD_AUTH</code>: Enable user password-based authentication. In this flow, Amazon Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p></li>  <li>   <p><code>ALLOW_USER_SRP_AUTH</code>: Enable SRP-based authentication.</p></li>  <li>   <p><code>ALLOW_REFRESH_TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p></li> </ul> <p>In some environments, you will see the values <code>ADMIN_NO_SRP_AUTH</code>, <code>CUSTOM_AUTH_FLOW_ONLY</code>, or <code>USER_PASSWORD_AUTH</code>. You can't assign these legacy <code>ExplicitAuthFlows</code> values to user pool clients at the same time as values that begin with <code>ALLOW_</code>, like <code>ALLOW_USER_SRP_AUTH</code>.</p><br>
     ///   - [`supported_identity_providers(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::supported_identity_providers) / [`set_supported_identity_providers(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_supported_identity_providers):<br>required: **false**<br><p>A list of provider names for the identity providers (IdPs) that are supported on this client. The following are supported: <code>COGNITO</code>, <code>Facebook</code>, <code>Google</code>, <code>SignInWithApple</code>, and <code>LoginWithAmazon</code>. You can also specify the names that you configured for the SAML and OIDC IdPs in your user pool, for example <code>MySAMLIdP</code> or <code>MyOIDCIdP</code>.</p> <p>This parameter sets the IdPs that <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html">managed login</a> will display on the login page for your app client. The removal of <code>COGNITO</code> from this list doesn't prevent authentication operations for local users with the user pools API in an Amazon Web Services SDK. The only way to prevent SDK-based authentication is to block access with a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-waf.html">WAF rule</a>.</p><br>
-    ///   - [`callback_urls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::callback_urls) / [`set_callback_urls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_callback_urls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
-    ///   - [`logout_urls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::logout_urls) / [`set_logout_urls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_logout_urls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
+    ///   - [`callback_ur_ls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::callback_ur_ls) / [`set_callback_ur_ls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_callback_ur_ls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for callback URLs to <code>http://localhost</code>, <code>http://127.0.0.1</code> and <code>http://\[::1\]</code>. These callback URLs are for testing purposes only. You can specify custom TCP ports for your callback URLs.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
+    ///   - [`logout_ur_ls(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::logout_ur_ls) / [`set_logout_ur_ls(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_logout_ur_ls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
     ///   - [`default_redirect_uri(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::default_redirect_uri) / [`set_default_redirect_uri(Option<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_default_redirect_uri):<br>required: **false**<br><p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p><br>
     ///   - [`allowed_o_auth_flows(OAuthFlowType)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::allowed_o_auth_flows) / [`set_allowed_o_auth_flows(Option<Vec::<OAuthFlowType>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_allowed_o_auth_flows):<br>required: **false**<br><p>The OAuth grant types that you want your app client to generate for clients in managed login authentication. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p> <dl>  <dt>   code  </dt>  <dd>   <p>Use a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the <code>/oauth2/token</code> endpoint.</p>  </dd>  <dt>   implicit  </dt>  <dd>   <p>Issue the access token, and the ID token when scopes like <code>openid</code> and <code>profile</code> are requested, directly to your user.</p>  </dd>  <dt>   client_credentials  </dt>  <dd>   <p>Issue the access token from the <code>/oauth2/token</code> endpoint directly to a non-person user, authorized by a combination of the client ID and client secret.</p>  </dd> </dl><br>
     ///   - [`allowed_o_auth_scopes(impl Into<String>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::allowed_o_auth_scopes) / [`set_allowed_o_auth_scopes(Option<Vec::<String>>)`](crate::operation::create_user_pool_client::builders::CreateUserPoolClientFluentBuilder::set_allowed_o_auth_scopes):<br>required: **false**<br><p>The OAuth, OpenID Connect (OIDC), and custom scopes that you want to permit your app client to authorize access with. Scopes govern access control to user pool self-service API operations, user data from the <code>userInfo</code> endpoint, and third-party APIs. Scope values include <code>phone</code>, <code>email</code>, <code>openid</code>, and <code>profile</code>. The <code>aws.cognito.signin.user.admin</code> scope authorizes user self-service operations. Custom scopes with resource servers authorize access to external APIs.</p><br>
```

### `src/client/initiate_auth.rs`

```diff
--- reference/src/client/initiate_auth.rs
+++ generated/src/client/initiate_auth.rs
@@ -4,8 +4,8 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`auth_flow(AuthFlowType)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_flow) / [`set_auth_flow(Option<AuthFlowType>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_flow):<br>required: **true**<br><p>The authentication flow that you want to initiate. Each <code>AuthFlow</code> has linked <code>AuthParameters</code> that you must submit. The following are some example flows.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <p>The entry point for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a> with passwords, one-time passwords, and WebAuthn authenticators. Request a preferred authentication type or review available authentication types. From the offered authentication types, select one in a challenge response and then authenticate with that method in an additional challenge response. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <p>Username-password authentication with the Secure Remote Password (SRP) protocol. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow.html#Using-SRP-password-verification-in-custom-authentication-flow">Use SRP password verification in custom authentication flow</a>.</p>  </dd>  <dt>   REFRESH_TOKEN_AUTH and REFRESH_TOKEN  </dt>  <dd>   <p>Receive new ID and access tokens when you pass a <code>REFRESH_TOKEN</code> parameter with a valid refresh token as the value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html">Using the refresh token</a>.</p>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <p>Custom authentication with Lambda triggers. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-challenge.html">Custom authentication challenge Lambda triggers</a>.</p>  </dd>  <dt>   USER_PASSWORD_AUTH  </dt>  <dd>   <p>Client-side username-password authentication with the password sent directly in the request. For more information about client-side and server-side authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-public-server-side.html">SDK authorization models</a>.</p>  </dd> </dl> <p><code>ADMIN_USER_PASSWORD_AUTH</code> is a flow type of <code>AdminInitiateAuth</code> and isn't valid for InitiateAuth. <code>ADMIN_NO_SRP_AUTH</code> is a legacy server-side username-password flow and isn't valid for InitiateAuth.</p><br>
-    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li>   </ul>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>SRP_A</code> (required)</p></li>   </ul>  </dd>  <dt>   USER_PASSWORD_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>PASSWORD</code> (required)</p></li>   </ul>  </dd>  <dt>   REFRESH_TOKEN_AUTH/REFRESH_TOKEN  </dt>  <dd>   <ul>    <li>     <p><code>REFRESH_TOKEN</code>(required)</p></li>   </ul>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <ul>    <li>     <p><code>USERNAME</code> (required)</p></li>    <li>     <p><code>ChallengeName: SRP_A</code> (when doing SRP authentication before custom challenges)</p></li>    <li>     <p><code>SRP_A: (An SRP_A value)</code> (when doing SRP authentication before custom challenges)</p></li>   </ul>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
-    ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The <code>ClientMetadata</code> value is passed as input to the functions for only the following triggers:</p> <ul>  <li>   <p>Pre signup</p></li>  <li>   <p>Pre authentication</p></li>  <li>   <p>User migration</p></li> </ul> <p>This request also invokes the functions for the following triggers, but doesn't pass <code>ClientMetadata</code>:</p> <ul>  <li>   <p>Post authentication</p></li>  <li>   <p>Custom message</p></li>  <li>   <p>Pre token generation</p></li>  <li>   <p>Create auth challenge</p></li>  <li>   <p>Define auth challenge</p></li>  <li>   <p>Custom email sender</p></li>  <li>   <p>Custom SMS sender</p></li> </ul><note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
+    ///   - [`auth_parameters(impl Into<String>, impl Into<String>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::auth_parameters) / [`set_auth_parameters(Option<HashMap::<String, String>>)`](crate::operation::initiate_auth::builders::InitiateAuthFluentBuilder::set_auth_parameters):<br>required: **false**<br><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking.</p> <p>The following are some authentication flows and their parameters. Add a <code>SECRET_HASH</code> parameter if your app client has a client secret. Add <code>DEVICE_KEY</code> if you want to bypass multi-factor authentication with a remembered device.</p> <dl>  <dt>   USER_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>PREFERRED_CHALLENGE</code>. If you don't provide a value for <code>PREFERRED_CHALLENGE</code>, Amazon Cognito responds with the <code>AvailableChallenges</code> parameter that specifies the available sign-in methods.</p></li> </ul>  </dd>  <dt>   USER_SRP_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>SRP_A</code> (required)</p></li> </ul>  </dd>  <dt>   USER_PASSWORD_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>PASSWORD</code> (required)</p></li> </ul>  </dd>  <dt>   REFRESH_TOKEN_AUTH/REFRESH_TOKEN  </dt>  <dd>   <ul> <li> <p> <code>REFRESH_TOKEN</code>(required)</p></li> </ul>  </dd>  <dt>   CUSTOM_AUTH  </dt>  <dd>   <ul> <li> <p> <code>USERNAME</code> (required)</p></li> <li> <p> <code>ChallengeName: SRP_A</code> (when doing SRP authentication before custom challenges)</p></li> <li> <p> <code>SRP_A: (An SRP_A value)</code> (when doing SRP authentication before custom challenges)</p></li> </ul>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
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
+    ///   - [`challenge_responses(impl Into<String>, impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::challenge_responses) / [`set_challenge_responses(Option<HashMap::<String, String>>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_challenge_responses):<br>required: **false**<br><p>The responses to the challenge that you received in the previous request. Each challenge has its own required response parameters. The following examples are partial JSON request bodies that highlight challenge-response parameters.</p><important>  <p>You must provide a SECRET_HASH parameter in all challenge responses to an app client that has a client secret. Include a <code>DEVICE_KEY</code> for device authentication.</p> </important> <dl>  <dt>   SELECT_CHALLENGE  </dt>  <dd>   <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "USERNAME": "\[username\]", "ANSWER": "\[Challenge name\]"}</code></p>   <p>Available challenges are <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, <code>EMAIL_OTP</code>, <code>SMS_OTP</code>, and <code>WEB_AUTHN</code>.</p>   <p>Complete authentication in the <code>SELECT_CHALLENGE</code> response for <code>PASSWORD</code>, <code>PASSWORD_SRP</code>, and <code>WEB_AUTHN</code>:</p>   <ul> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "WEB_AUTHN", "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p> <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p></li> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD", "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p></li> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "PASSWORD_SRP", "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p></li> </ul>   <p>For <code>SMS_OTP</code> and <code>EMAIL_OTP</code>, respond with the username and answer. Your user pool will send a code for the user to submit in the next challenge response.</p>   <ul> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "SMS_OTP", "USERNAME": "\[username\]"}</code></p></li> <li> <p> <code>"ChallengeName": "SELECT_CHALLENGE", "ChallengeResponses": { "ANSWER": "EMAIL_OTP", "USERNAME": "\[username\]"}</code></p></li> </ul>  </dd>  <dt>   WEB_AUTHN  </dt>  <dd>   <p> <code>"ChallengeName": "WEB_AUTHN", "ChallengeResponses": { "USERNAME": "\[username\]", "CREDENTIAL": "\[AuthenticationResponseJSON\]"}</code></p>   <p>See <a href="https://www.w3.org/TR/WebAuthn-3/#dictdef-authenticationresponsejson"> AuthenticationResponseJSON</a>.</p>  </dd>  <dt>   PASSWORD  </dt>  <dd>   <p> <code>"ChallengeName": "PASSWORD", "ChallengeResponses": { "USERNAME": "\[username\]", "PASSWORD": "\[password\]"}</code></p>  </dd>  <dt>   PASSWORD_SRP  </dt>  <dd>   <p> <code>"ChallengeName": "PASSWORD_SRP", "ChallengeResponses": { "USERNAME": "\[username\]", "SRP_A": "\[SRP_A\]"}</code></p>  </dd>  <dt>   SMS_OTP  </dt>  <dd>   <p> <code>"ChallengeName": "SMS_OTP", "ChallengeResponses": {"SMS_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   EMAIL_OTP  </dt>  <dd>   <p> <code>"ChallengeName": "EMAIL_OTP", "ChallengeResponses": {"EMAIL_OTP_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   SMS_MFA  </dt>  <dd>   <p> <code>"ChallengeName": "SMS_MFA", "ChallengeResponses": {"SMS_MFA_CODE": "\[code\]", "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   PASSWORD_VERIFIER  </dt>  <dd>   <p>This challenge response is part of the SRP flow. Amazon Cognito requires that your application respond to this challenge within a few seconds. When the response time exceeds this period, your user pool returns a <code>NotAuthorizedException</code> error.</p>   <p> <code>"ChallengeName": "PASSWORD_VERIFIER", "ChallengeResponses": {"PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   CUSTOM_CHALLENGE  </dt>  <dd>   <p> <code>"ChallengeName": "CUSTOM_CHALLENGE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[challenge_answer\]"}</code></p>  </dd>  <dt>   NEW_PASSWORD_REQUIRED  </dt>  <dd>   <p> <code>"ChallengeName": "NEW_PASSWORD_REQUIRED", "ChallengeResponses": {"NEW_PASSWORD": "\[new_password\]", "USERNAME": "\[username\]"}</code></p>   <p>To set any required attributes that <code>InitiateAuth</code> returned in an <code>requiredAttributes</code> parameter, add <code>"userAttributes.\[attribute_name\]": "\[attribute_value\]"</code>. This parameter can also set values for writable attributes that aren't required by your user pool.</p>   <note> <p>In a <code>NEW_PASSWORD_REQUIRED</code> challenge response, you can't modify a required attribute that already has a value. In <code>AdminRespondToAuthChallenge</code> or <code>RespondToAuthChallenge</code>, set a value for any keys that Amazon Cognito returned in the <code>requiredAttributes</code> parameter, then use the <code>AdminUpdateUserAttributes</code> or <code>UpdateUserAttributes</code> API operation to modify the value of any additional attributes.</p> </note>  </dd>  <dt>   SOFTWARE_TOKEN_MFA  </dt>  <dd>   <p> <code>"ChallengeName": "SOFTWARE_TOKEN_MFA", "ChallengeResponses": {"USERNAME": "\[username\]", "SOFTWARE_TOKEN_MFA_CODE": \[authenticator_code\]}</code></p>  </dd>  <dt>   DEVICE_SRP_AUTH  </dt>  <dd>   <p> <code>"ChallengeName": "DEVICE_SRP_AUTH", "ChallengeResponses": {"USERNAME": "\[username\]", "DEVICE_KEY": "\[device_key\]", "SRP_A": "\[srp_a\]"}</code></p>  </dd>  <dt>   DEVICE_PASSWORD_VERIFIER  </dt>  <dd>   <p> <code>"ChallengeName": "DEVICE_PASSWORD_VERIFIER", "ChallengeResponses": {"DEVICE_KEY": "\[device_key\]", "PASSWORD_CLAIM_SIGNATURE": "\[claim_signature\]", "PASSWORD_CLAIM_SECRET_BLOCK": "\[secret_block\]", "TIMESTAMP": \[timestamp\], "USERNAME": "\[username\]"}</code></p>  </dd>  <dt>   MFA_SETUP  </dt>  <dd>   <p> <code>"ChallengeName": "MFA_SETUP", "ChallengeResponses": {"USERNAME": "\[username\]"}, "SESSION": "\[Session ID from VerifySoftwareToken\]"</code></p>  </dd>  <dt>   SELECT_MFA_TYPE  </dt>  <dd>   <p> <code>"ChallengeName": "SELECT_MFA_TYPE", "ChallengeResponses": {"USERNAME": "\[username\]", "ANSWER": "\[SMS_MFA|EMAIL_MFA|SOFTWARE_TOKEN_MFA\]"}</code></p>  </dd> </dl> <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p><br>
     ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_analytics_metadata):<br>required: **false**<br><p>Information that supports analytics outcomes with Amazon Pinpoint, including the user's endpoint ID. The endpoint ID is a destination for Amazon Pinpoint push notifications, for example a device identifier, email address, or phone number.</p><br>
     ///   - [`user_context_data(UserContextDataType)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::user_context_data) / [`set_user_context_data(Option<UserContextDataType>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_user_context_data):<br>required: **false**<br><p>Contextual data about your user session like the device fingerprint, IP address, or location. Amazon Cognito threat protection evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-viewing-threat-protection-app.html">Collecting data for threat protection in applications</a>.</p><br>
     ///   - [`client_metadata(impl Into<String>, impl Into<String>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::client_metadata) / [`set_client_metadata(Option<HashMap::<String, String>>)`](crate::operation::respond_to_auth_challenge::builders::RespondToAuthChallengeFluentBuilder::set_client_metadata):<br>required: **false**<br><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. You create custom workflows by assigning Lambda functions to user pool triggers.</p> <p>When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute that provides the data that you assigned to the ClientMetadata parameter in your request. In your function code, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>To review the Lambda trigger types that Amazon Cognito invokes at runtime with API requests, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-working-with-lambda-triggers.html#lambda-triggers-by-event"> Connecting API actions to Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>  <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>  <ul>   <li>    <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>   <li>    <p>Validate the <code>ClientMetadata</code> value.</p></li>   <li>    <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>  </ul> </note><br>
```

### `src/client/update_user_pool_client.rs`

```diff
--- reference/src/client/update_user_pool_client.rs
+++ generated/src/client/update_user_pool_client.rs
@@ -14,8 +14,8 @@
     ///   - [`write_attributes(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::write_attributes) / [`set_write_attributes(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_write_attributes):<br>required: **false**<br><p>The list of user attributes that you want your app client to have write access to. After your user authenticates in your app, their access token authorizes them to set or modify their own attribute value for any attribute in this list.</p> <p>When you don't specify the <code>WriteAttributes</code> for your app client, your app can write the values of the Standard attributes of your user pool. When your user pool has write access to these default attributes, <code>WriteAttributes</code> doesn't return any information. Amazon Cognito only populates <code>WriteAttributes</code> in the API response if you have specified your own custom set of write attributes.</p> <p>If your app client allows users to sign in through an IdP, this array must include all attributes that you have mapped to IdP attributes. Amazon Cognito updates mapped attributes when users sign in to your application through an IdP. If your app client does not have write access to a mapped attribute, Amazon Cognito throws an error when it tries to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying IdP Attribute Mappings for Your user pool</a>.</p><br>
     ///   - [`explicit_auth_flows(ExplicitAuthFlowsType)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::explicit_auth_flows) / [`set_explicit_auth_flows(Option<Vec::<ExplicitAuthFlowsType>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_explicit_auth_flows):<br>required: **false**<br><p>The <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow-methods.html">authentication flows</a> that you want your user pool client to support. For each app client in your user pool, you can sign in your users with any combination of one or more flows, including with a user name and Secure Remote Password (SRP), a user name and password, or a custom authentication process that you define with Lambda functions.</p><note>  <p>If you don't specify a value for <code>ExplicitAuthFlows</code>, your app client supports <code>ALLOW_REFRESH_TOKEN_AUTH</code>, <code>ALLOW_USER_SRP_AUTH</code>, and <code>ALLOW_CUSTOM_AUTH</code>.</p> </note> <p>The values for authentication flow options include the following.</p> <ul>  <li>   <p><code>ALLOW_USER_AUTH</code>: Enable selection-based sign-in with <code>USER_AUTH</code>. This setting covers username-password, secure remote password (SRP), passwordless, and passkey authentication. This authentiation flow can do username-password and SRP authentication without other <code>ExplicitAuthFlows</code> permitting them. For example users can complete an SRP challenge through <code>USER_AUTH</code> without the flow <code>USER_SRP_AUTH</code> being active for the app client. This flow doesn't include <code>CUSTOM_AUTH</code>.</p>   <p>To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html"> Essentials tier</a> or higher.</p></li>  <li>   <p><code>ALLOW_ADMIN_USER_PASSWORD_AUTH</code>: Enable admin based user password authentication flow <code>ADMIN_USER_PASSWORD_AUTH</code>. This setting replaces the <code>ADMIN_NO_SRP_AUTH</code> setting. With this authentication flow, your app passes a user name and password to Amazon Cognito in the request, instead of using the Secure Remote Password (SRP) protocol to securely transmit the password.</p></li>  <li>   <p><code>ALLOW_CUSTOM_AUTH</code>: Enable Lambda trigger based authentication.</p></li>  <li>   <p><code>ALLOW_USER_PASSWORD_AUTH</code>: Enable user password-based authentication. In this flow, Amazon Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p></li>  <li>   <p><code>ALLOW_USER_SRP_AUTH</code>: Enable SRP-based authentication.</p></li>  <li>   <p><code>ALLOW_REFRESH_TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p></li> </ul> <p>In some environments, you will see the values <code>ADMIN_NO_SRP_AUTH</code>, <code>CUSTOM_AUTH_FLOW_ONLY</code>, or <code>USER_PASSWORD_AUTH</code>. You can't assign these legacy <code>ExplicitAuthFlows</code> values to user pool clients at the same time as values that begin with <code>ALLOW_</code>, like <code>ALLOW_USER_SRP_AUTH</code>.</p><br>
     ///   - [`supported_identity_providers(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::supported_identity_providers) / [`set_supported_identity_providers(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_supported_identity_providers):<br>required: **false**<br><p>A list of provider names for the identity providers (IdPs) that are supported on this client. The following are supported: <code>COGNITO</code>, <code>Facebook</code>, <code>Google</code>, <code>SignInWithApple</code>, and <code>LoginWithAmazon</code>. You can also specify the names that you configured for the SAML and OIDC IdPs in your user pool, for example <code>MySAMLIdP</code> or <code>MyOIDCIdP</code>.</p> <p>This parameter sets the IdPs that <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html">managed login</a> will display on the login page for your app client. The removal of <code>COGNITO</code> from this list doesn't prevent authentication operations for local users with the user pools API in an Amazon Web Services SDK. The only way to prevent SDK-based authentication is to block access with a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-waf.html">WAF rule</a>.</p><br>
-    ///   - [`callback_urls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::callback_urls) / [`set_callback_urls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_callback_urls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
-    ///   - [`logout_urls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::logout_urls) / [`set_logout_urls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_logout_urls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
+    ///   - [`callback_ur_ls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::callback_ur_ls) / [`set_callback_ur_ls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_callback_ur_ls):<br>required: **false**<br><p>A list of allowed redirect, or callback, URLs for managed login authentication. These URLs are the paths where you want to send your users' browsers after they complete authentication with managed login or a third-party IdP. Typically, callback URLs are the home of an application that uses OAuth or OIDC libraries to process authentication outcomes.</p> <p>A redirect URI must meet the following requirements:</p> <ul>  <li>   <p>Be an absolute URI.</p></li>  <li>   <p>Be registered with the authorization server. Amazon Cognito doesn't accept authorization requests with <code>redirect_uri</code> values that aren't in the list of <code>CallbackURLs</code> that you provide in this parameter.</p></li>  <li>   <p>Not include a fragment component.</p></li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as <code>myapp://example</code> are also supported.</p><br>
+    ///   - [`logout_ur_ls(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::logout_ur_ls) / [`set_logout_ur_ls(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_logout_ur_ls):<br>required: **false**<br><p>A list of allowed logout URLs for managed login authentication. When you pass <code>logout_uri</code> and <code>client_id</code> parameters to <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout URL. This parameter describes the URLs that you want to be the permitted targets of <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout endpoint</a>.</p><br>
     ///   - [`default_redirect_uri(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::default_redirect_uri) / [`set_default_redirect_uri(Option<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_default_redirect_uri):<br>required: **false**<br><p>The default redirect URI. In app clients with one assigned IdP, replaces <code>redirect_uri</code> in authentication requests. Must be in the <code>CallbackURLs</code> list.</p><br>
     ///   - [`allowed_o_auth_flows(OAuthFlowType)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::allowed_o_auth_flows) / [`set_allowed_o_auth_flows(Option<Vec::<OAuthFlowType>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_allowed_o_auth_flows):<br>required: **false**<br><p>The OAuth grant types that you want your app client to generate. To create an app client that generates client credentials grants, you must add <code>client_credentials</code> as the only allowed OAuth flow.</p> <dl>  <dt>   code  </dt>  <dd>   <p>Use a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the <code>/oauth2/token</code> endpoint.</p>  </dd>  <dt>   implicit  </dt>  <dd>   <p>Issue the access token (and, optionally, ID token, based on scopes) directly to your user.</p>  </dd>  <dt>   client_credentials  </dt>  <dd>   <p>Issue the access token from the <code>/oauth2/token</code> endpoint directly to a non-person user using a combination of the client ID and client secret.</p>  </dd> </dl><br>
     ///   - [`allowed_o_auth_scopes(impl Into<String>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::allowed_o_auth_scopes) / [`set_allowed_o_auth_scopes(Option<Vec::<String>>)`](crate::operation::update_user_pool_client::builders::UpdateUserPoolClientFluentBuilder::set_allowed_o_auth_scopes):<br>required: **false**<br><p>The OAuth, OpenID Connect (OIDC), and custom scopes that you want to permit your app client to authorize access with. Scopes govern access control to user pool self-service API operations, user data from the <code>userInfo</code> endpoint, and third-party APIs. Scope values include <code>phone</code>, <code>email</code>, <code>openid</code>, and <code>profile</code>. The <code>aws.cognito.signin.user.admin</code> scope authorizes user self-service operations. Custom scopes with resource servers authorize access to external APIs.</p><br>
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
@@ -1036,8 +1036,8 @@
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
@@ -1008,8 +1008,8 @@
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
