# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/cognitoidentityprovider`
- model: `models/cognitoidentityprovider.json`
- package: `aws-sdk-cognitoidentityprovider-modular`
- library: `aws_sdk_cognitoidentityprovider_modular`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_add_custom_attributes`: `AddCustomAttributes`
- `op_add_user_pool_client_secret`: `AddUserPoolClientSecret`
- `op_admin_add_user_to_group`: `AdminAddUserToGroup`
- `op_admin_confirm_sign_up`: `AdminConfirmSignUp`
- `op_admin_create_user`: `AdminCreateUser`
- `op_admin_delete_user`: `AdminDeleteUser`
- `op_admin_delete_user_attributes`: `AdminDeleteUserAttributes`
- `op_admin_disable_provider_for_user`: `AdminDisableProviderForUser`
- `op_admin_disable_user`: `AdminDisableUser`
- `op_admin_enable_user`: `AdminEnableUser`
- `op_admin_forget_device`: `AdminForgetDevice`
- `op_admin_get_device`: `AdminGetDevice`
- `op_admin_get_user`: `AdminGetUser`
- `op_admin_get_user_auth_factors`: `AdminGetUserAuthFactors`
- `op_admin_initiate_auth`: `AdminInitiateAuth`
- `op_admin_link_provider_for_user`: `AdminLinkProviderForUser`
- `op_admin_list_devices`: `AdminListDevices`
- `op_admin_list_groups_for_user`: `AdminListGroupsForUser`
- `op_admin_list_user_auth_events`: `AdminListUserAuthEvents`
- `op_admin_remove_user_from_group`: `AdminRemoveUserFromGroup`
- `op_admin_reset_user_password`: `AdminResetUserPassword`
- `op_admin_respond_to_auth_challenge`: `AdminRespondToAuthChallenge`
- `op_admin_set_user_mfa_preference`: `AdminSetUserMFAPreference`
- `op_admin_set_user_password`: `AdminSetUserPassword`
- `op_admin_set_user_settings`: `AdminSetUserSettings`
- `op_admin_update_auth_event_feedback`: `AdminUpdateAuthEventFeedback`
- `op_admin_update_device_status`: `AdminUpdateDeviceStatus`
- `op_admin_update_user_attributes`: `AdminUpdateUserAttributes`
- `op_admin_user_global_sign_out`: `AdminUserGlobalSignOut`
- `op_associate_software_token`: `AssociateSoftwareToken`
- `op_change_password`: `ChangePassword`
- `op_complete_web_authn_registration`: `CompleteWebAuthnRegistration`
- `op_confirm_device`: `ConfirmDevice`
- `op_confirm_forgot_password`: `ConfirmForgotPassword`
- `op_confirm_sign_up`: `ConfirmSignUp`
- `op_create_group`: `CreateGroup`
- `op_create_identity_provider`: `CreateIdentityProvider`
- `op_create_managed_login_branding`: `CreateManagedLoginBranding`
- `op_create_resource_server`: `CreateResourceServer`
- `op_create_terms`: `CreateTerms`
- `op_create_user_import_job`: `CreateUserImportJob`
- `op_create_user_pool`: `CreateUserPool`
- `op_create_user_pool_client`: `CreateUserPoolClient`
- `op_create_user_pool_domain`: `CreateUserPoolDomain`
- `op_create_user_pool_replica`: `CreateUserPoolReplica`
- `op_delete_group`: `DeleteGroup`
- `op_delete_identity_provider`: `DeleteIdentityProvider`
- `op_delete_managed_login_branding`: `DeleteManagedLoginBranding`
- `op_delete_resource_server`: `DeleteResourceServer`
- `op_delete_terms`: `DeleteTerms`
- `op_delete_user`: `DeleteUser`
- `op_delete_user_attributes`: `DeleteUserAttributes`
- `op_delete_user_pool`: `DeleteUserPool`
- `op_delete_user_pool_client`: `DeleteUserPoolClient`
- `op_delete_user_pool_client_secret`: `DeleteUserPoolClientSecret`
- `op_delete_user_pool_domain`: `DeleteUserPoolDomain`
- `op_delete_user_pool_replica`: `DeleteUserPoolReplica`
- `op_delete_web_authn_credential`: `DeleteWebAuthnCredential`
- `op_describe_identity_provider`: `DescribeIdentityProvider`
- `op_describe_managed_login_branding`: `DescribeManagedLoginBranding`
- `op_describe_managed_login_branding_by_client`: `DescribeManagedLoginBrandingByClient`
- `op_describe_resource_server`: `DescribeResourceServer`
- `op_describe_risk_configuration`: `DescribeRiskConfiguration`
- `op_describe_terms`: `DescribeTerms`
- `op_describe_user_import_job`: `DescribeUserImportJob`
- `op_describe_user_pool`: `DescribeUserPool`
- `op_describe_user_pool_client`: `DescribeUserPoolClient`
- `op_describe_user_pool_domain`: `DescribeUserPoolDomain`
- `op_forget_device`: `ForgetDevice`
- `op_forgot_password`: `ForgotPassword`
- `op_get_csv_header`: `GetCSVHeader`
- `op_get_device`: `GetDevice`
- `op_get_group`: `GetGroup`
- `op_get_identity_provider_by_identifier`: `GetIdentityProviderByIdentifier`
- `op_get_log_delivery_configuration`: `GetLogDeliveryConfiguration`
- `op_get_provisioned_limit`: `GetProvisionedLimit`
- `op_get_signing_certificate`: `GetSigningCertificate`
- `op_get_tokens_from_refresh_token`: `GetTokensFromRefreshToken`
- `op_get_ui_customization`: `GetUICustomization`
- `op_get_user`: `GetUser`
- `op_get_user_attribute_verification_code`: `GetUserAttributeVerificationCode`
- `op_get_user_auth_factors`: `GetUserAuthFactors`
- `op_get_user_pool_mfa_config`: `GetUserPoolMfaConfig`
- `op_global_sign_out`: `GlobalSignOut`
- `op_initiate_auth`: `InitiateAuth`
- `op_list_devices`: `ListDevices`
- `op_list_groups`: `ListGroups`
- `op_list_identity_providers`: `ListIdentityProviders`
- `op_list_resource_servers`: `ListResourceServers`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_list_terms`: `ListTerms`
- `op_list_user_import_jobs`: `ListUserImportJobs`
- `op_list_user_pool_clients`: `ListUserPoolClients`
- `op_list_user_pool_client_secrets`: `ListUserPoolClientSecrets`
- `op_list_user_pool_replicas`: `ListUserPoolReplicas`
- `op_list_user_pools`: `ListUserPools`
- `op_list_users`: `ListUsers`
- `op_list_users_in_group`: `ListUsersInGroup`
- `op_list_web_authn_credentials`: `ListWebAuthnCredentials`
- `op_resend_confirmation_code`: `ResendConfirmationCode`
- `op_respond_to_auth_challenge`: `RespondToAuthChallenge`
- `op_revoke_token`: `RevokeToken`
- `op_set_log_delivery_configuration`: `SetLogDeliveryConfiguration`
- `op_set_risk_configuration`: `SetRiskConfiguration`
- `op_set_ui_customization`: `SetUICustomization`
- `op_set_user_mfa_preference`: `SetUserMFAPreference`
- `op_set_user_pool_mfa_config`: `SetUserPoolMfaConfig`
- `op_set_user_settings`: `SetUserSettings`
- `op_sign_up`: `SignUp`
- `op_start_user_import_job`: `StartUserImportJob`
- `op_start_web_authn_registration`: `StartWebAuthnRegistration`
- `op_stop_user_import_job`: `StopUserImportJob`
- `op_tag_resource`: `TagResource`
- `op_untag_resource`: `UntagResource`
- `op_update_auth_event_feedback`: `UpdateAuthEventFeedback`
- `op_update_device_status`: `UpdateDeviceStatus`
- `op_update_group`: `UpdateGroup`
- `op_update_identity_provider`: `UpdateIdentityProvider`
- `op_update_managed_login_branding`: `UpdateManagedLoginBranding`
- `op_update_provisioned_limit`: `UpdateProvisionedLimit`
- `op_update_resource_server`: `UpdateResourceServer`
- `op_update_terms`: `UpdateTerms`
- `op_update_user_attributes`: `UpdateUserAttributes`
- `op_update_user_pool`: `UpdateUserPool`
- `op_update_user_pool_client`: `UpdateUserPoolClient`
- `op_update_user_pool_domain`: `UpdateUserPoolDomain`
- `op_update_user_pool_replica`: `UpdateUserPoolReplica`
- `op_verify_software_token`: `VerifySoftwareToken`
- `op_verify_user_attribute`: `VerifyUserAttribute`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/config.rs`
- `src/error_meta.rs`
- `src/lens.rs`
- `src/lib.rs`
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
