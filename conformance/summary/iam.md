# AWS SDK Conformance Report: iam

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## iam
**Progress:** `1626/1626` files compared · `654` matched · `452` mismatches · `520` missing · `0` extra · `40.22%` match (100.00% means fully matched)

### `src/client/add_role_to_instance_profile.rs`

```diff
--- reference/src/client/add_role_to_instance_profile.rs
+++ generated/src/client/add_role_to_instance_profile.rs
@@ -7,7 +7,9 @@
     ///   - [`role_name(impl Into<String>)`](crate::operation::add_role_to_instance_profile::builders::AddRoleToInstanceProfileFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::add_role_to_instance_profile::builders::AddRoleToInstanceProfileFluentBuilder::set_role_name):<br>required: **true**<br><p>The name of the role to add.</p> <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p><br>
     /// - On success, responds with [`AddRoleToInstanceProfileOutput`](crate::operation::add_role_to_instance_profile::AddRoleToInstanceProfileOutput)
     /// - On failure, responds with [`SdkError<AddRoleToInstanceProfileError>`](crate::operation::add_role_to_instance_profile::AddRoleToInstanceProfileError)
-    pub fn add_role_to_instance_profile(&self) -> super::super::operation::add_role_to_instance_profile::builders::AddRoleToInstanceProfileFluentBuilder {
+    pub fn add_role_to_instance_profile(
+        &self,
+    ) -> super::super::operation::add_role_to_instance_profile::builders::AddRoleToInstanceProfileFluentBuilder {
         super::super::operation::add_role_to_instance_profile::builders::AddRoleToInstanceProfileFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/associate_delegation_request.rs`

```diff
--- reference/src/client/associate_delegation_request.rs
+++ generated/src/client/associate_delegation_request.rs
@@ -6,7 +6,9 @@
     ///   - [`delegation_request_id(impl Into<String>)`](crate::operation::associate_delegation_request::builders::AssociateDelegationRequestFluentBuilder::delegation_request_id) / [`set_delegation_request_id(Option<String>)`](crate::operation::associate_delegation_request::builders::AssociateDelegationRequestFluentBuilder::set_delegation_request_id):<br>required: **true**<br><p>The unique identifier of the delegation request to associate.</p><br>
     /// - On success, responds with [`AssociateDelegationRequestOutput`](crate::operation::associate_delegation_request::AssociateDelegationRequestOutput)
     /// - On failure, responds with [`SdkError<AssociateDelegationRequestError>`](crate::operation::associate_delegation_request::AssociateDelegationRequestError)
-    pub fn associate_delegation_request(&self) -> super::super::operation::associate_delegation_request::builders::AssociateDelegationRequestFluentBuilder {
+    pub fn associate_delegation_request(
+        &self,
+    ) -> super::super::operation::associate_delegation_request::builders::AssociateDelegationRequestFluentBuilder {
         super::super::operation::associate_delegation_request::builders::AssociateDelegationRequestFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/delete_service_linked_role.rs`

```diff
--- reference/src/client/delete_service_linked_role.rs
+++ generated/src/client/delete_service_linked_role.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`role_name(impl Into<String>)`](crate::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleFluentBuilder::set_role_name):<br>required: **true**<br><p>The name of the service-linked role to be deleted.</p><br>
     /// - On success, responds with [`DeleteServiceLinkedRoleOutput`](crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput) with field(s):
-    ///   - [`deletion_task_id(String)`](crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput::deletion_task_id): <p>The deletion task identifier that you can use to check the status of the deletion. This identifier is returned in the format <code>task/aws-service-role/<service-principal-name>    /    <role-name>     /     <task-uuid></task-uuid>    </role-name>   </service-principal-name></code>.</p>
+    ///   - [`deletion_task_id(String)`](crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput::deletion_task_id): <p>The deletion task identifier that you can use to check the status of the deletion. This identifier is returned in the format <code>task/aws-service-role/<service-principal-name>    /    <role-name>     /     <task-uuid></task-uuid>      </role-name>     </service-principal-name></code>.</p>
     /// - On failure, responds with [`SdkError<DeleteServiceLinkedRoleError>`](crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleError)
     pub fn delete_service_linked_role(&self) -> super::super::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleFluentBuilder {
         super::super::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleFluentBuilder::new(self.handle.clone())
```

### `src/client/disable_organizations_root_credentials_management.rs`

```diff
--- reference/src/client/disable_organizations_root_credentials_management.rs
+++ generated/src/client/disable_organizations_root_credentials_management.rs
@@ -7,12 +7,7 @@
     ///   - [`organization_id(Option<String>)`](crate::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput::organization_id): <p>The unique identifier (ID) of an organization.</p>
     ///   - [`enabled_features(Option<Vec::<FeatureType>>)`](crate::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput::enabled_features): <p>The features enabled for centralized root access for member accounts in your organization.</p>
     /// - On failure, responds with [`SdkError<DisableOrganizationsRootCredentialsManagementError>`](crate::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementError)
-    pub fn disable_organizations_root_credentials_management(
-        &self,
-    ) -> super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementFluentBuilder
-    {
-        super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementFluentBuilder::new(
-            self.handle.clone(),
-        )
+    pub fn disable_organizations_root_credentials_management(&self) -> super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementFluentBuilder{
+        super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/disable_organizations_root_sessions.rs`

```diff
--- reference/src/client/disable_organizations_root_sessions.rs
+++ generated/src/client/disable_organizations_root_sessions.rs
@@ -10,6 +10,8 @@
     pub fn disable_organizations_root_sessions(
         &self,
     ) -> super::super::operation::disable_organizations_root_sessions::builders::DisableOrganizationsRootSessionsFluentBuilder {
-        super::super::operation::disable_organizations_root_sessions::builders::DisableOrganizationsRootSessionsFluentBuilder::new(self.handle.clone())
+        super::super::operation::disable_organizations_root_sessions::builders::DisableOrganizationsRootSessionsFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/enable_organizations_root_credentials_management.rs`

```diff
--- reference/src/client/enable_organizations_root_credentials_management.rs
+++ generated/src/client/enable_organizations_root_credentials_management.rs
@@ -9,9 +9,8 @@
     /// - On failure, responds with [`SdkError<EnableOrganizationsRootCredentialsManagementError>`](crate::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementError)
     pub fn enable_organizations_root_credentials_management(
         &self,
-    ) -> super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementFluentBuilder {
-        super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementFluentBuilder::new(
-            self.handle.clone(),
-        )
+    ) -> super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementFluentBuilder
+    {
+        super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/generate_organizations_access_report.rs`

```diff
--- reference/src/client/generate_organizations_access_report.rs
+++ generated/src/client/generate_organizations_access_report.rs
@@ -11,6 +11,8 @@
     pub fn generate_organizations_access_report(
         &self,
     ) -> super::super::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder {
-        super::super::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::new(self.handle.clone())
+        super::super::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/generate_service_last_accessed_details.rs`

```diff
--- reference/src/client/generate_service_last_accessed_details.rs
+++ generated/src/client/generate_service_last_accessed_details.rs
@@ -11,6 +11,8 @@
     pub fn generate_service_last_accessed_details(
         &self,
     ) -> super::super::operation::generate_service_last_accessed_details::builders::GenerateServiceLastAccessedDetailsFluentBuilder {
-        super::super::operation::generate_service_last_accessed_details::builders::GenerateServiceLastAccessedDetailsFluentBuilder::new(self.handle.clone())
+        super::super::operation::generate_service_last_accessed_details::builders::GenerateServiceLastAccessedDetailsFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/get_account_password_policy.rs`

```diff
--- reference/src/client/get_account_password_policy.rs
+++ generated/src/client/get_account_password_policy.rs
@@ -6,7 +6,9 @@
     /// - On success, responds with [`GetAccountPasswordPolicyOutput`](crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput) with field(s):
     ///   - [`password_policy(Option<PasswordPolicy>)`](crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput::password_policy): <p>A structure that contains details about the account's password policy.</p>
     /// - On failure, responds with [`SdkError<GetAccountPasswordPolicyError>`](crate::operation::get_account_password_policy::GetAccountPasswordPolicyError)
-    pub fn get_account_password_policy(&self) -> super::super::operation::get_account_password_policy::builders::GetAccountPasswordPolicyFluentBuilder {
+    pub fn get_account_password_policy(
+        &self,
+    ) -> super::super::operation::get_account_password_policy::builders::GetAccountPasswordPolicyFluentBuilder {
         super::super::operation::get_account_password_policy::builders::GetAccountPasswordPolicyFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_context_keys_for_principal_policy.rs`

```diff
--- reference/src/client/get_context_keys_for_principal_policy.rs
+++ generated/src/client/get_context_keys_for_principal_policy.rs
@@ -11,6 +11,8 @@
     pub fn get_context_keys_for_principal_policy(
         &self,
     ) -> super::super::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder {
-        super::super::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::new(self.handle.clone())
+        super::super::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/get_open_id_connect_provider.rs`

```diff
--- reference/src/client/get_open_id_connect_provider.rs
+++ generated/src/client/get_open_id_connect_provider.rs
@@ -11,7 +11,9 @@
     ///   - [`create_date(Option<DateTime>)`](crate::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput::create_date): <p>The date and time when the IAM OIDC provider resource object was created in the Amazon Web Services account.</p>
     ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput::tags): <p>A list of tags that are attached to the specified IAM OIDC provider. The returned list of tags is sorted by tag key. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
     /// - On failure, responds with [`SdkError<GetOpenIDConnectProviderError>`](crate::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError)
-    pub fn get_open_id_connect_provider(&self) -> super::super::operation::get_open_id_connect_provider::builders::GetOpenIDConnectProviderFluentBuilder {
+    pub fn get_open_id_connect_provider(
+        &self,
+    ) -> super::super::operation::get_open_id_connect_provider::builders::GetOpenIDConnectProviderFluentBuilder {
         super::super::operation::get_open_id_connect_provider::builders::GetOpenIDConnectProviderFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_service_last_accessed_details_with_entities.rs`

```diff
--- reference/src/client/get_service_last_accessed_details_with_entities.rs
+++ generated/src/client/get_service_last_accessed_details_with_entities.rs
@@ -18,9 +18,8 @@
     /// - On failure, responds with [`SdkError<GetServiceLastAccessedDetailsWithEntitiesError>`](crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesError)
     pub fn get_service_last_accessed_details_with_entities(
         &self,
-    ) -> super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesFluentBuilder {
-        super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesFluentBuilder::new(
-            self.handle.clone(),
-        )
+    ) -> super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesFluentBuilder
+    {
+        super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_service_linked_role_deletion_status.rs`

```diff
--- reference/src/client/get_service_linked_role_deletion_status.rs
+++ generated/src/client/get_service_linked_role_deletion_status.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`GetServiceLinkedRoleDeletionStatus`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`deletion_task_id(impl Into<String>)`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::deletion_task_id) / [`set_deletion_task_id(Option<String>)`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::set_deletion_task_id):<br>required: **true**<br><p>The deletion task identifier. This identifier is returned by the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_DeleteServiceLinkedRole.html">DeleteServiceLinkedRole</a> operation in the format <code>task/aws-service-role/<service-principal-name>    /    <role-name>     /     <task-uuid></task-uuid>    </role-name>   </service-principal-name></code>.</p><br>
+    ///   - [`deletion_task_id(impl Into<String>)`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::deletion_task_id) / [`set_deletion_task_id(Option<String>)`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::set_deletion_task_id):<br>required: **true**<br><p>The deletion task identifier. This identifier is returned by the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_DeleteServiceLinkedRole.html">DeleteServiceLinkedRole</a> operation in the format <code>task/aws-service-role/<service-principal-name>    /    <role-name>     /     <task-uuid></task-uuid>      </role-name>     </service-principal-name></code>.</p><br>
     /// - On success, responds with [`GetServiceLinkedRoleDeletionStatusOutput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput) with field(s):
     ///   - [`status(DeletionTaskStatusType)`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput::status): <p>The status of the deletion.</p>
     ///   - [`reason(Option<DeletionTaskFailureReasonType>)`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput::reason): <p>An object that contains details about the reason the deletion failed.</p>
@@ -11,6 +11,8 @@
     pub fn get_service_linked_role_deletion_status(
         &self,
     ) -> super::super::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder {
-        super::super::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::new(self.handle.clone())
+        super::super::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/list_attached_group_policies.rs`

```diff
--- reference/src/client/list_attached_group_policies.rs
+++ generated/src/client/list_attached_group_policies.rs
@@ -13,7 +13,9 @@
     ///   - [`is_truncated(bool)`](crate::operation::list_attached_group_policies::ListAttachedGroupPoliciesOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
     ///   - [`marker(Option<String>)`](crate::operation::list_attached_group_policies::ListAttachedGroupPoliciesOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
     /// - On failure, responds with [`SdkError<ListAttachedGroupPoliciesError>`](crate::operation::list_attached_group_policies::ListAttachedGroupPoliciesError)
-    pub fn list_attached_group_policies(&self) -> super::super::operation::list_attached_group_policies::builders::ListAttachedGroupPoliciesFluentBuilder {
+    pub fn list_attached_group_policies(
+        &self,
+    ) -> super::super::operation::list_attached_group_policies::builders::ListAttachedGroupPoliciesFluentBuilder {
         super::super::operation::list_attached_group_policies::builders::ListAttachedGroupPoliciesFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/list_attached_role_policies.rs`

```diff
--- reference/src/client/list_attached_role_policies.rs
+++ generated/src/client/list_attached_role_policies.rs
@@ -13,7 +13,9 @@
     ///   - [`is_truncated(bool)`](crate::operation::list_attached_role_policies::ListAttachedRolePoliciesOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
     ///   - [`marker(Option<String>)`](crate::operation::list_attached_role_policies::ListAttachedRolePoliciesOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
     /// - On failure, responds with [`SdkError<ListAttachedRolePoliciesError>`](crate::operation::list_attached_role_policies::ListAttachedRolePoliciesError)
-    pub fn list_attached_role_policies(&self) -> super::super::operation::list_attached_role_policies::builders::ListAttachedRolePoliciesFluentBuilder {
+    pub fn list_attached_role_policies(
+        &self,
+    ) -> super::super::operation::list_attached_role_policies::builders::ListAttachedRolePoliciesFluentBuilder {
         super::super::operation::list_attached_role_policies::builders::ListAttachedRolePoliciesFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/list_attached_user_policies.rs`

```diff
--- reference/src/client/list_attached_user_policies.rs
+++ generated/src/client/list_attached_user_policies.rs
@@ -13,7 +13,9 @@
     ///   - [`is_truncated(bool)`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
     ///   - [`marker(Option<String>)`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
     /// - On failure, responds with [`SdkError<ListAttachedUserPoliciesError>`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesError)
-    pub fn list_attached_user_policies(&self) -> super::super::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder {
+    pub fn list_attached_user_policies(
+        &self,
+    ) -> super::super::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder {
         super::super::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/list_organizations_features.rs`

```diff
--- reference/src/client/list_organizations_features.rs
+++ generated/src/client/list_organizations_features.rs
@@ -7,7 +7,9 @@
     ///   - [`organization_id(Option<String>)`](crate::operation::list_organizations_features::ListOrganizationsFeaturesOutput::organization_id): <p>The unique identifier (ID) of an organization.</p>
     ///   - [`enabled_features(Option<Vec::<FeatureType>>)`](crate::operation::list_organizations_features::ListOrganizationsFeaturesOutput::enabled_features): <p>Specifies the features that are currently available in your organization.</p>
     /// - On failure, responds with [`SdkError<ListOrganizationsFeaturesError>`](crate::operation::list_organizations_features::ListOrganizationsFeaturesError)
-    pub fn list_organizations_features(&self) -> super::super::operation::list_organizations_features::builders::ListOrganizationsFeaturesFluentBuilder {
+    pub fn list_organizations_features(
+        &self,
+    ) -> super::super::operation::list_organizations_features::builders::ListOrganizationsFeaturesFluentBuilder {
         super::super::operation::list_organizations_features::builders::ListOrganizationsFeaturesFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/list_policies_granting_service_access.rs`

```diff
--- reference/src/client/list_policies_granting_service_access.rs
+++ generated/src/client/list_policies_granting_service_access.rs
@@ -14,6 +14,8 @@
     pub fn list_policies_granting_service_access(
         &self,
     ) -> super::super::operation::list_policies_granting_service_access::builders::ListPoliciesGrantingServiceAccessFluentBuilder {
-        super::super::operation::list_policies_granting_service_access::builders::ListPoliciesGrantingServiceAccessFluentBuilder::new(self.handle.clone())
+        super::super::operation::list_policies_granting_service_access::builders::ListPoliciesGrantingServiceAccessFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/list_server_certificate_tags.rs`

```diff
--- reference/src/client/list_server_certificate_tags.rs
+++ generated/src/client/list_server_certificate_tags.rs
@@ -12,7 +12,9 @@
     ///   - [`is_truncated(bool)`](crate::operation::list_server_certificate_tags::ListServerCertificateTagsOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
     ///   - [`marker(Option<String>)`](crate::operation::list_server_certificate_tags::ListServerCertificateTagsOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
     /// - On failure, responds with [`SdkError<ListServerCertificateTagsError>`](crate::operation::list_server_certificate_tags::ListServerCertificateTagsError)
-    pub fn list_server_certificate_tags(&self) -> super::super::operation::list_server_certificate_tags::builders::ListServerCertificateTagsFluentBuilder {
+    pub fn list_server_certificate_tags(
+        &self,
+    ) -> super::super::operation::list_server_certificate_tags::builders::ListServerCertificateTagsFluentBuilder {
         super::super::operation::list_server_certificate_tags::builders::ListServerCertificateTagsFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/set_security_token_service_preferences.rs`

```diff
--- reference/src/client/set_security_token_service_preferences.rs
+++ generated/src/client/set_security_token_service_preferences.rs
@@ -9,6 +9,8 @@
     pub fn set_security_token_service_preferences(
         &self,
     ) -> super::super::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder {
-        super::super::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder::new(self.handle.clone())
+        super::super::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/simulate_custom_policy.rs`

```diff
--- reference/src/client/simulate_custom_policy.rs
+++ generated/src/client/simulate_custom_policy.rs
@@ -9,7 +9,7 @@
     ///   - [`ordered_organization_policy_input_list(OrderedOrganizationPolicyType)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::ordered_organization_policy_input_list) / [`set_ordered_organization_policy_input_list(Option<Vec::<OrderedOrganizationPolicyType>>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_ordered_organization_policy_input_list):<br>required: **false**<br><p>An ordered list of service control policies (SCPs) to include in the simulation. Each element represents one level of an Organizations hierarchy, from the organization root to the account.</p> <p>The simulator evaluates SCPs in the order that you provide, consistent with how Organizations enforces SCPs. The first element must represent the organization root, and the last element must represent the account. Any elements between them represent organizational units (OUs) in descending order.</p> <p>Use this parameter to simulate the effect of an SCP hierarchy without calling <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_SimulatePrincipalPolicy.html">SimulatePrincipalPolicy</a>.</p><br>
     ///   - [`action_names(impl Into<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::action_names) / [`set_action_names(Option<Vec::<String>>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_action_names):<br>required: **true**<br><p>A list of names of API operations to evaluate in the simulation. Each operation is evaluated against each resource. Each operation must include the service identifier, such as <code>iam:CreateUser</code>. This operation does not support using wildcards (*) in an action name.</p><br>
     ///   - [`resource_arns(impl Into<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::resource_arns) / [`set_resource_arns(Option<Vec::<String>>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_resource_arns):<br>required: **false**<br><p>A list of ARNs of Amazon Web Services resources to include in the simulation. If this parameter is not provided, then the value defaults to <code>*</code> (all resources). Each API in the <code>ActionNames</code> parameter is evaluated for each resource in this list. The simulation determines the access result (allowed or denied) of each combination and reports it in the response. You can simulate resources that don't exist in your account.</p> <p>The simulation does not automatically retrieve policies for the specified resources. If you want to include a resource policy in the simulation, then you must include the policy as a string in the <code>ResourcePolicy</code> parameter.</p> <p>If you include a <code>ResourcePolicy</code>, then it must be applicable to all of the resources included in the simulation or you receive an invalid input error.</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p><note>  <p>Simulation of resource-based policies isn't supported for IAM roles.</p> </note><br>
-    ///   - [`resource_policy(impl Into<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::resource_policy) / [`set_resource_policy(Option<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_resource_policy):<br>required: **false**<br><p>A resource-based policy to include in the simulation provided as a string. Each resource in the simulation is treated as if it had this policy attached. You can include only one resource-based policy in a simulation.</p> <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p> <ul>  <li>   <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p></li>  <li>   <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p></li>  <li>   <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p></li> </ul><note>  <p>Simulation of resource-based policies isn't supported for IAM roles.</p> </note><br>
+    ///   - [`resource_policy(impl Into<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::resource_policy) / [`set_resource_policy(Option<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_resource_policy):<br>required: **false**<br><p>A resource-based policy to include in the simulation provided as a string. Each resource in the simulation is treated as if it had this policy attached. You can include only one resource-based policy in a simulation.</p> <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p> <ul>  <li>   <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p></li>  <li>   <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p></li>  <li>   <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p></li> </ul> <note>  <p>Simulation of resource-based policies isn't supported for IAM roles.</p> </note><br>
     ///   - [`resource_owner(impl Into<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::resource_owner) / [`set_resource_owner(Option<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_resource_owner):<br>required: **false**<br><p>An ARN representing the Amazon Web Services account ID that specifies the owner of any simulated resource that does not identify its owner in the resource ARN. Examples of resource ARNs include an S3 bucket or object. If <code>ResourceOwner</code> is specified, it is also used as the account owner of any <code>ResourcePolicy</code> included in the simulation. If the <code>ResourceOwner</code> parameter is not specified, then the owner of the resources and the resource policy defaults to the account of the identity provided in <code>CallerArn</code>. This parameter is required only if you specify a resource-based policy and account that owns the resource is different from the account that owns the simulated calling user <code>CallerArn</code>.</p> <p>The ARN for an account uses the following syntax: <code>arn:aws:iam::<i>AWS-account-ID</i>:root</code>. For example, to represent the account with the 112233445566 ID, use the following ARN: <code>arn:aws:iam::112233445566-ID:root</code>.</p><br>
     ///   - [`caller_arn(impl Into<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::caller_arn) / [`set_caller_arn(Option<String>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_caller_arn):<br>required: **false**<br><p>The ARN of the IAM user, group, or role that you want to use as the simulated caller of the API operations. <code>CallerArn</code> is required if you include a <code>ResourcePolicy</code> so that the policy's <code>Principal</code> element has a value to use in evaluating the policy.</p> <p>You cannot specify the ARN of an assumed role, federated user, or a service principal.</p><br>
     ///   - [`context_entries(ContextEntry)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::context_entries) / [`set_context_entries(Option<Vec::<ContextEntry>>)`](crate::operation::simulate_custom_policy::builders::SimulateCustomPolicyFluentBuilder::set_context_entries):<br>required: **false**<br><p>A list of context keys and corresponding values for the simulation to use. Whenever a context key is evaluated in one of the simulated IAM permissions policies, the corresponding value is supplied.</p><br>
```

### `src/client/simulate_principal_policy.rs`

```diff
--- reference/src/client/simulate_principal_policy.rs
+++ generated/src/client/simulate_principal_policy.rs
@@ -10,7 +10,7 @@
     ///   - [`policy_exclusion_list(PolicyIdentifier)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::policy_exclusion_list) / [`set_policy_exclusion_list(Option<Vec::<PolicyIdentifier>>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_policy_exclusion_list):<br>required: **false**<br><p>A list of policies to exclude from the simulation. Use this parameter to test what the simulation result would be if a policy were removed, without changing which policies are actually attached to the principal identified by <code>PolicySourceArn</code>.</p> <p>Each entry is a <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_PolicyIdentifier.html">PolicyIdentifier</a> that identifies one or more policies to exclude by policy type, by Amazon Resource Name (ARN), or by the name of an inline policy and the entity it is attached to.</p> <p>Syntactically invalid identifiers, such as malformed ARNs or wildcards in disallowed positions, cause the request to fail with an <code>InvalidInput</code> error. Syntactically valid identifiers that don't match any attached policy are ignored. Resource control policies (RCPs) are not supported in this release; identifiers that target RCPs are also ignored.</p><br>
     ///   - [`action_names(impl Into<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::action_names) / [`set_action_names(Option<Vec::<String>>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_action_names):<br>required: **true**<br><p>A list of names of API operations to evaluate in the simulation. Each operation is evaluated for each resource. Each operation must include the service identifier, such as <code>iam:CreateUser</code>.</p><br>
     ///   - [`resource_arns(impl Into<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::resource_arns) / [`set_resource_arns(Option<Vec::<String>>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_resource_arns):<br>required: **false**<br><p>A list of ARNs of Amazon Web Services resources to include in the simulation. If this parameter is not provided, then the value defaults to <code>*</code> (all resources). Each API in the <code>ActionNames</code> parameter is evaluated for each resource in this list. The simulation determines the access result (allowed or denied) of each combination and reports it in the response. You can simulate resources that don't exist in your account.</p> <p>The simulation does not automatically retrieve policies for the specified resources. If you want to include a resource policy in the simulation, then you must include the policy as a string in the <code>ResourcePolicy</code> parameter.</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p><note>  <p>Simulation of resource-based policies isn't supported for IAM roles.</p> </note><br>
-    ///   - [`resource_policy(impl Into<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::resource_policy) / [`set_resource_policy(Option<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_resource_policy):<br>required: **false**<br><p>A resource-based policy to include in the simulation provided as a string. Each resource in the simulation is treated as if it had this policy attached. You can include only one resource-based policy in a simulation.</p> <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p> <ul>  <li>   <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p></li>  <li>   <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p></li>  <li>   <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p></li> </ul><note>  <p>Simulation of resource-based policies isn't supported for IAM roles.</p> </note><br>
+    ///   - [`resource_policy(impl Into<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::resource_policy) / [`set_resource_policy(Option<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_resource_policy):<br>required: **false**<br><p>A resource-based policy to include in the simulation provided as a string. Each resource in the simulation is treated as if it had this policy attached. You can include only one resource-based policy in a simulation.</p> <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p> <ul>  <li>   <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p></li>  <li>   <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p></li>  <li>   <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p></li> </ul> <note>  <p>Simulation of resource-based policies isn't supported for IAM roles.</p> </note><br>
     ///   - [`resource_owner(impl Into<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::resource_owner) / [`set_resource_owner(Option<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_resource_owner):<br>required: **false**<br><p>An Amazon Web Services account ID that specifies the owner of any simulated resource that does not identify its owner in the resource ARN. Examples of resource ARNs include an S3 bucket or object. If <code>ResourceOwner</code> is specified, it is also used as the account owner of any <code>ResourcePolicy</code> included in the simulation. If the <code>ResourceOwner</code> parameter is not specified, then the owner of the resources and the resource policy defaults to the account of the identity provided in <code>CallerArn</code>. This parameter is required only if you specify a resource-based policy and account that owns the resource is different from the account that owns the simulated calling user <code>CallerArn</code>.</p><br>
     ///   - [`caller_arn(impl Into<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::caller_arn) / [`set_caller_arn(Option<String>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_caller_arn):<br>required: **false**<br><p>The ARN of the IAM user, group, or role that you want to specify as the simulated caller of the API operations. If you do not specify a <code>CallerArn</code>, it defaults to the ARN of the user, group, or role that you specify in <code>PolicySourceArn</code>. If you include both a <code>PolicySourceArn</code> (for example, <code>arn:aws:iam::123456789012:user/David</code>) and a <code>CallerArn</code> (for example, <code>arn:aws:iam::123456789012:user/Bob</code>), the result is that you simulate calling the API operations as Bob, as if Bob had David's policies.</p> <p>You can specify the ARN of an IAM user, group, or role. You cannot specify the ARN of an assumed role, federated user, or a service principal.</p> <p><code>CallerArn</code> is required if you include a <code>ResourcePolicy</code> and the <code>PolicySourceArn</code> is not the ARN for an IAM user, group, or role. This is required so that the resource-based policy's <code>Principal</code> element has a value to use in evaluating the policy.</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p><br>
     ///   - [`context_entries(ContextEntry)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::context_entries) / [`set_context_entries(Option<Vec::<ContextEntry>>)`](crate::operation::simulate_principal_policy::builders::SimulatePrincipalPolicyFluentBuilder::set_context_entries):<br>required: **false**<br><p>A list of context keys and corresponding values for the simulation to use. Whenever a context key is evaluated in one of the simulated IAM permissions policies, the corresponding value is supplied.</p><br>
```

### `src/client/tag_open_id_connect_provider.rs`

```diff
--- reference/src/client/tag_open_id_connect_provider.rs
+++ generated/src/client/tag_open_id_connect_provider.rs
@@ -7,7 +7,9 @@
     ///   - [`tags(Tag)`](crate::operation::tag_open_id_connect_provider::builders::TagOpenIDConnectProviderFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::tag_open_id_connect_provider::builders::TagOpenIDConnectProviderFluentBuilder::set_tags):<br>required: **true**<br><p>The list of tags that you want to attach to the OIDC identity provider in IAM. Each tag consists of a key name and an associated value.</p><br>
     /// - On success, responds with [`TagOpenIdConnectProviderOutput`](crate::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput)
     /// - On failure, responds with [`SdkError<TagOpenIDConnectProviderError>`](crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError)
-    pub fn tag_open_id_connect_provider(&self) -> super::super::operation::tag_open_id_connect_provider::builders::TagOpenIDConnectProviderFluentBuilder {
+    pub fn tag_open_id_connect_provider(
+        &self,
+    ) -> super::super::operation::tag_open_id_connect_provider::builders::TagOpenIDConnectProviderFluentBuilder {
         super::super::operation::tag_open_id_connect_provider::builders::TagOpenIDConnectProviderFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -29,7 +29,10 @@
 /// Endpoint resolver trait specific to AWS Identity and Access Management
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -622,7 +625,10 @@
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

### `src/operation/accept_delegation_request.rs`

```diff
--- reference/src/operation/accept_delegation_request.rs
+++ generated/src/operation/accept_delegation_request.rs
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
-            super::super::protocol_serde::shape_accept_delegation_request_input::ser_accept_delegation_request_input_input_input(&input)?,
+            super::super::protocol_serde::shape_accept_delegation_request_input::ser_accept_delegation_request_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/acquire_role/_acquire_role_input.rs`

```diff
--- reference/src/operation/acquire_role/_acquire_role_input.rs
+++ generated/src/operation/acquire_role/_acquire_role_input.rs
@@ -9,7 +9,8 @@
     /// <p>The minor version of the role template to use. If you do not specify a minor version, the service uses the template's default minor version.</p>
     pub template_minor_version: ::std::option::Option<i32>,
     /// <p>A map of values to substitute for the parameters that are defined in the role template version. Each key is a parameter name from the template, and each value is a structure that contains the replacement values for that parameter.</p>
-    pub replacement_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ReplacementValueEntry>>,
+    pub replacement_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ReplacementValueEntry>>,
 }
 impl AcquireRoleInput {
     /// <p>The Amazon Resource Name (ARN) of the role template to create the role from.</p>
@@ -41,7 +42,8 @@
 pub struct AcquireRoleInputBuilder {
     pub(crate) template_arn: ::std::option::Option<::std::string::String>,
     pub(crate) template_minor_version: ::std::option::Option<i32>,
-    pub(crate) replacement_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ReplacementValueEntry>>,
+    pub(crate) replacement_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ReplacementValueEntry>>,
 }
 impl AcquireRoleInputBuilder {
     /// <p>The Amazon Resource Name (ARN) of the role template to create the role from.</p>
@@ -81,7 +83,11 @@
     /// To override the contents of this collection use [`set_replacement_values`](Self::set_replacement_values).
     ///
     /// <p>A map of values to substitute for the parameters that are defined in the role template version. Each key is a parameter name from the template, and each value is a structure that contains the replacement values for that parameter.</p>
-    pub fn replacement_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::ReplacementValueEntry) -> Self {
+    pub fn replacement_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::ReplacementValueEntry,
+    ) -> Self {
         let mut hash_map = self.replacement_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.replacement_values = ::std::option::Option::Some(hash_map);
@@ -102,7 +108,9 @@
         &self.replacement_values
     }
     /// Consumes the builder and constructs a [`AcquireRoleInput`](crate::operation::acquire_role::AcquireRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::acquire_role::AcquireRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::acquire_role::AcquireRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::acquire_role::AcquireRoleInput {
             template_arn: self.template_arn,
             template_minor_version: self.template_minor_version,
```

### `src/operation/acquire_role/builders.rs`

```diff
--- reference/src/operation/acquire_role/builders.rs
+++ generated/src/operation/acquire_role/builders.rs
@@ -146,7 +146,11 @@
     /// To override the contents of this collection use [`set_replacement_values`](Self::set_replacement_values).
     ///
     /// <p>A map of values to substitute for the parameters that are defined in the role template version. Each key is a parameter name from the template, and each value is a structure that contains the replacement values for that parameter.</p>
-    pub fn replacement_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::ReplacementValueEntry) -> Self {
+    pub fn replacement_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::ReplacementValueEntry,
+    ) -> Self {
         self.inner = self.inner.replacement_values(k.into(), v);
         self
     }
```

### `src/operation/acquire_role.rs`

```diff
--- reference/src/operation/acquire_role.rs
+++ generated/src/operation/acquire_role.rs
@@ -247,13 +247,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_acquire_role_input::ser_acquire_role_input_input_input(
-            &input,
-        )?);
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_acquire_role_input::ser_acquire_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/add_client_id_to_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/add_client_id_to_open_id_connect_provider/builders.rs
+++ generated/src/operation/add_client_id_to_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError,
+            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `AddClientIDToOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `AddClientIdToOpenIdConnectProvider`.
 ///
 /// <p>Adds a new client ID (also known as audience) to the list of client IDs already registered for the specified IAM OpenID Connect (OIDC) provider resource.</p>
 /// <p>This operation is idempotent; it does not fail or return an error if you add an existing client ID to the provider.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct AddClientIDToOpenIDConnectProviderFluentBuilder {
+pub struct AddClientIdToOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIdToOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput,
-        super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError,
-    > for AddClientIDToOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderError,
+    > for AddClientIdToOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput,
-            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError,
+            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl AddClientIDToOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `AddClientIDToOpenIDConnectProviderFluentBuilder`.
+impl AddClientIdToOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `AddClientIdToOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,8 +57,10 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the AddClientIDToOpenIDConnectProvider as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIdToOpenIdConnectProviderInputBuilder {
+    /// Access the AddClientIdToOpenIdConnectProvider as a reference.
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIdToOpenIdConnectProviderInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -74,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError,
+            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +85,16 @@
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
         let runtime_plugins =
-            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProvider::operation_runtime_plugins(
+            super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProvider::operation_runtime_plugins(
                 self.handle.runtime_plugins.clone(),
                 &self.handle.conf,
                 self.config_override,
             );
-        super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProvider::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +102,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput,
-        super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError,
+        super::super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/add_client_id_to_open_id_connect_provider.rs`

```diff
--- reference/src/operation/add_client_id_to_open_id_connect_provider.rs
+++ generated/src/operation/add_client_id_to_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `AddClientIDToOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `AddClientIdToOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct AddClientIDToOpenIDConnectProvider;
-impl AddClientIDToOpenIDConnectProvider {
-    /// Creates a new `AddClientIDToOpenIDConnectProvider`
+pub struct AddClientIdToOpenIdConnectProvider;
+impl AddClientIdToOpenIdConnectProvider {
+    /// Creates a new `AddClientIdToOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AddClientIDToOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for AddClientIdToOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AddClientIDToOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            AddClientIDToOpenIDConnectProviderRequestSerializer,
+            AddClientIdToOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AddClientIDToOpenIDConnectProviderResponseDeserializer,
+            AddClientIdToOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -133,13 +133,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AddClientIDToOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                AddClientIDToOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                AddClientIdToOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                AddClientIDToOpenIDConnectProviderEndpointParamsInterceptor,
+                AddClientIdToOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError,
@@ -156,12 +156,12 @@
 }

 #[derive(Debug)]
-struct AddClientIDToOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct AddClientIdToOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AddClientIDToOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AddClientIdToOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "AddClientIDToOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "AddClientIdToOpenIdConnectProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -265,11 +265,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_add_client_id_to_open_id_connect_provider_input::ser_add_client_id_to_open_id_connect_provider_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_add_client_id_to_open_id_connect_provider_input::ser_add_client_id_to_open_id_connect_provider_op_input(& input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -278,12 +277,12 @@
     }
 }
 #[derive(Debug)]
-struct AddClientIDToOpenIDConnectProviderEndpointParamsInterceptor;
+struct AddClientIdToOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AddClientIDToOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AddClientIdToOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "AddClientIDToOpenIDConnectProviderEndpointParamsInterceptor"
+        "AddClientIdToOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
@@ -456,7 +455,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError {
+impl ::aws_types::request_id::RequestId
+    for super::super::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/add_role_to_instance_profile/_add_role_to_instance_profile_input.rs`

```diff
--- reference/src/operation/add_role_to_instance_profile/_add_role_to_instance_profile_input.rs
+++ generated/src/operation/add_role_to_instance_profile/_add_role_to_instance_profile_input.rs
@@ -80,9 +80,11 @@
         super::super::super::operation::add_role_to_instance_profile::AddRoleToInstanceProfileInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::add_role_to_instance_profile::AddRoleToInstanceProfileInput {
-            instance_profile_name: self.instance_profile_name,
-            role_name: self.role_name,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::add_role_to_instance_profile::AddRoleToInstanceProfileInput {
+                instance_profile_name: self.instance_profile_name,
+                role_name: self.role_name,
+            },
+        )
     }
 }
```

### `src/operation/add_role_to_instance_profile.rs`

```diff
--- reference/src/operation/add_role_to_instance_profile.rs
+++ generated/src/operation/add_role_to_instance_profile.rs
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
-            super::super::protocol_serde::shape_add_role_to_instance_profile_input::ser_add_role_to_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_add_role_to_instance_profile_input::ser_add_role_to_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/add_user_to_group.rs`

```diff
--- reference/src/operation/add_user_to_group.rs
+++ generated/src/operation/add_user_to_group.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_add_user_to_group_input::ser_add_user_to_group_input_input_input(&input)?,
+            super::super::protocol_serde::shape_add_user_to_group_input::ser_add_user_to_group_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/associate_delegation_request/_associate_delegation_request_input.rs`

```diff
--- reference/src/operation/associate_delegation_request/_associate_delegation_request_input.rs
+++ generated/src/operation/associate_delegation_request/_associate_delegation_request_input.rs
@@ -48,8 +48,10 @@
         super::super::super::operation::associate_delegation_request::AssociateDelegationRequestInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::associate_delegation_request::AssociateDelegationRequestInput {
-            delegation_request_id: self.delegation_request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::associate_delegation_request::AssociateDelegationRequestInput {
+                delegation_request_id: self.delegation_request_id,
+            },
+        )
     }
 }
```

### `src/operation/associate_delegation_request.rs`

```diff
--- reference/src/operation/associate_delegation_request.rs
+++ generated/src/operation/associate_delegation_request.rs
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
-            super::super::protocol_serde::shape_associate_delegation_request_input::ser_associate_delegation_request_input_input_input(&input)?,
+            super::super::protocol_serde::shape_associate_delegation_request_input::ser_associate_delegation_request_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/attach_group_policy/_attach_group_policy_input.rs`

```diff
--- reference/src/operation/attach_group_policy/_attach_group_policy_input.rs
+++ generated/src/operation/attach_group_policy/_attach_group_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`AttachGroupPolicyInput`](crate::operation::attach_group_policy::AttachGroupPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::attach_group_policy::AttachGroupPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::attach_group_policy::AttachGroupPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::attach_group_policy::AttachGroupPolicyInput {
             group_name: self.group_name,
             policy_arn: self.policy_arn,
```

### `src/operation/attach_group_policy.rs`

```diff
--- reference/src/operation/attach_group_policy.rs
+++ generated/src/operation/attach_group_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_attach_group_policy_input::ser_attach_group_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_attach_group_policy_input::ser_attach_group_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/attach_role_policy/_attach_role_policy_input.rs`

```diff
--- reference/src/operation/attach_role_policy/_attach_role_policy_input.rs
+++ generated/src/operation/attach_role_policy/_attach_role_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`AttachRolePolicyInput`](crate::operation::attach_role_policy::AttachRolePolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::attach_role_policy::AttachRolePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::attach_role_policy::AttachRolePolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::attach_role_policy::AttachRolePolicyInput {
             role_name: self.role_name,
             policy_arn: self.policy_arn,
```

### `src/operation/attach_role_policy.rs`

```diff
--- reference/src/operation/attach_role_policy.rs
+++ generated/src/operation/attach_role_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_attach_role_policy_input::ser_attach_role_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_attach_role_policy_input::ser_attach_role_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/attach_user_policy/_attach_user_policy_input.rs`

```diff
--- reference/src/operation/attach_user_policy/_attach_user_policy_input.rs
+++ generated/src/operation/attach_user_policy/_attach_user_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`AttachUserPolicyInput`](crate::operation::attach_user_policy::AttachUserPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::attach_user_policy::AttachUserPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::attach_user_policy::AttachUserPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::attach_user_policy::AttachUserPolicyInput {
             user_name: self.user_name,
             policy_arn: self.policy_arn,
```

### `src/operation/attach_user_policy.rs`

```diff
--- reference/src/operation/attach_user_policy.rs
+++ generated/src/operation/attach_user_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_attach_user_policy_input::ser_attach_user_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_attach_user_policy_input::ser_attach_user_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/change_password.rs`

```diff
--- reference/src/operation/change_password.rs
+++ generated/src/operation/change_password.rs
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
-            super::super::protocol_serde::shape_change_password_input::ser_change_password_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_change_password_input::ser_change_password_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/create_access_key/_create_access_key_input.rs`

```diff
--- reference/src/operation/create_access_key/_create_access_key_input.rs
+++ generated/src/operation/create_access_key/_create_access_key_input.rs
@@ -48,7 +48,10 @@
     /// Consumes the builder and constructs a [`CreateAccessKeyInput`](crate::operation::create_access_key::CreateAccessKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_access_key::CreateAccessKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_access_key::CreateAccessKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_access_key::CreateAccessKeyInput { user_name: self.user_name })
     }
 }
```

### `src/operation/create_access_key.rs`

```diff
--- reference/src/operation/create_access_key.rs
+++ generated/src/operation/create_access_key.rs
@@ -248,12 +248,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_create_access_key_input::ser_create_access_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_access_key_input::ser_create_access_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_account_alias/_create_account_alias_input.rs`

```diff
--- reference/src/operation/create_account_alias/_create_account_alias_input.rs
+++ generated/src/operation/create_account_alias/_create_account_alias_input.rs
@@ -49,8 +49,10 @@
     /// Consumes the builder and constructs a [`CreateAccountAliasInput`](crate::operation::create_account_alias::CreateAccountAliasInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_account_alias::CreateAccountAliasInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_account_alias::CreateAccountAliasInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_account_alias::CreateAccountAliasInput {
             account_alias: self.account_alias,
         })
```

### `src/operation/create_account_alias.rs`

```diff
--- reference/src/operation/create_account_alias.rs
+++ generated/src/operation/create_account_alias.rs
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
-            super::super::protocol_serde::shape_create_account_alias_input::ser_create_account_alias_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_account_alias_input::ser_create_account_alias_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_delegation_request/_create_delegation_request_input.rs`

```diff
--- reference/src/operation/create_delegation_request/_create_delegation_request_input.rs
+++ generated/src/operation/create_delegation_request/_create_delegation_request_input.rs
@@ -274,7 +274,7 @@
             redirect_url: self.redirect_url,
             notification_channel: self.notification_channel,
             session_duration: self.session_duration,
-            only_send_by_owner: self.only_send_by_owner,
+            only_send_by_owner: self.only_send_by_owner.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/create_delegation_request.rs`

```diff
--- reference/src/operation/create_delegation_request.rs
+++ generated/src/operation/create_delegation_request.rs
@@ -275,12 +275,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_create_delegation_request_input::ser_create_delegation_request_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_delegation_request_input::ser_create_delegation_request_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_group/_create_group_input.rs`

```diff
--- reference/src/operation/create_group/_create_group_input.rs
+++ generated/src/operation/create_group/_create_group_input.rs
@@ -78,7 +78,9 @@
         &self.group_name
     }
     /// Consumes the builder and constructs a [`CreateGroupInput`](crate::operation::create_group::CreateGroupInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_group::CreateGroupInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_group::CreateGroupInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_group::CreateGroupInput {
             path: self.path,
             group_name: self.group_name,
```

### `src/operation/create_group.rs`

```diff
--- reference/src/operation/create_group.rs
+++ generated/src/operation/create_group.rs
@@ -252,13 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_group_input::ser_create_group_input_input_input(
-            &input,
-        )?);
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_group_input::ser_create_group_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/create_instance_profile/_create_instance_profile_input.rs`

```diff
--- reference/src/operation/create_instance_profile/_create_instance_profile_input.rs
+++ generated/src/operation/create_instance_profile/_create_instance_profile_input.rs
@@ -119,8 +119,10 @@
     /// Consumes the builder and constructs a [`CreateInstanceProfileInput`](crate::operation::create_instance_profile::CreateInstanceProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_instance_profile::CreateInstanceProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_instance_profile::CreateInstanceProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_instance_profile::CreateInstanceProfileInput {
             instance_profile_name: self.instance_profile_name,
             path: self.path,
```

### `src/operation/create_instance_profile.rs`

```diff
--- reference/src/operation/create_instance_profile.rs
+++ generated/src/operation/create_instance_profile.rs
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
-            super::super::protocol_serde::shape_create_instance_profile_input::ser_create_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_instance_profile_input::ser_create_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_login_profile/_create_login_profile_input.rs`

```diff
--- reference/src/operation/create_login_profile/_create_login_profile_input.rs
+++ generated/src/operation/create_login_profile/_create_login_profile_input.rs
@@ -114,12 +114,14 @@
     /// Consumes the builder and constructs a [`CreateLoginProfileInput`](crate::operation::create_login_profile::CreateLoginProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_login_profile::CreateLoginProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_login_profile::CreateLoginProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_login_profile::CreateLoginProfileInput {
             user_name: self.user_name,
             password: self.password,
-            password_reset_required: self.password_reset_required,
+            password_reset_required: self.password_reset_required.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/create_login_profile.rs`

```diff
--- reference/src/operation/create_login_profile.rs
+++ generated/src/operation/create_login_profile.rs
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
-            super::super::protocol_serde::shape_create_login_profile_input::ser_create_login_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_login_profile_input::ser_create_login_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_open_id_connect_provider/_create_open_id_connect_provider_input.rs`

```diff
--- reference/src/operation/create_open_id_connect_provider/_create_open_id_connect_provider_input.rs
+++ generated/src/operation/create_open_id_connect_provider/_create_open_id_connect_provider_input.rs
@@ -190,11 +190,13 @@
         super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderInput {
-            url: self.url,
-            client_id_list: self.client_id_list,
-            thumbprint_list: self.thumbprint_list,
-            tags: self.tags,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderInput {
+                url: self.url,
+                client_id_list: self.client_id_list,
+                thumbprint_list: self.thumbprint_list,
+                tags: self.tags,
+            },
+        )
     }
 }
```

### `src/operation/create_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/create_open_id_connect_provider/builders.rs
+++ generated/src/operation/create_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProviderError,
+            super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `CreateOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `CreateOpenIdConnectProvider`.
 ///
 /// <p>Creates an IAM entity to describe an identity provider (IdP) that supports <a href="http://openid.net/connect/">OpenID Connect (OIDC)</a>.</p>
 /// <p>The OIDC provider that you create with this operation can be used as a principal in a role's trust policy. Such a policy establishes a trust relationship between Amazon Web Services and the OIDC provider.</p>
@@ -42,7 +42,7 @@
 /// <p>The trust for the OIDC provider is derived from the IAM provider that this operation creates. Therefore, it is best to limit access to the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateOpenIDConnectProvider.html">CreateOpenIDConnectProvider</a> operation to highly privileged users.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct CreateOpenIDConnectProviderFluentBuilder {
+pub struct CreateOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::create_open_id_connect_provider::builders::CreateOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -50,8 +50,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput,
-        super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProviderError,
-    > for CreateOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderError,
+    > for CreateOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -59,14 +59,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput,
-            super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProviderError,
+            super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl CreateOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `CreateOpenIDConnectProviderFluentBuilder`.
+impl CreateOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `CreateOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -74,7 +74,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the CreateOpenIDConnectProvider as a reference.
+    /// Access the CreateOpenIdConnectProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::create_open_id_connect_provider::builders::CreateOpenIdConnectProviderInputBuilder {
         &self.inner
     }
@@ -91,7 +91,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProviderError,
+            super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -99,12 +99,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -112,7 +112,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput,
-        super::super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProviderError,
+        super::super::super::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/create_open_id_connect_provider.rs`

```diff
--- reference/src/operation/create_open_id_connect_provider.rs
+++ generated/src/operation/create_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `CreateOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `CreateOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct CreateOpenIDConnectProvider;
-impl CreateOpenIDConnectProvider {
-    /// Creates a new `CreateOpenIDConnectProvider`
+pub struct CreateOpenIdConnectProvider;
+impl CreateOpenIdConnectProvider {
+    /// Creates a new `CreateOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            CreateOpenIDConnectProviderRequestSerializer,
+            CreateOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateOpenIDConnectProviderResponseDeserializer,
+            CreateOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                CreateOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateOpenIDConnectProviderEndpointParamsInterceptor,
+                CreateOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::create_open_id_connect_provider::CreateOpenIDConnectProviderError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct CreateOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct CreateOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "CreateOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "CreateOpenIdConnectProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -209,7 +209,9 @@
         let parse_result = if !success && status != 200 || force_error {
             super::super::protocol_serde::shape_create_open_id_connect_provider::de_create_open_id_connect_provider_http_error(status, headers, body)
         } else {
-            super::super::protocol_serde::shape_create_open_id_connect_provider::de_create_open_id_connect_provider_http_response(status, headers, body)
+            super::super::protocol_serde::shape_create_open_id_connect_provider::de_create_open_id_connect_provider_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -250,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_create_open_id_connect_provider_input::ser_create_open_id_connect_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_open_id_connect_provider_input::ser_create_open_id_connect_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct CreateOpenIDConnectProviderEndpointParamsInterceptor;
+struct CreateOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "CreateOpenIDConnectProviderEndpointParamsInterceptor"
+        "CreateOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/create_policy.rs`

```diff
--- reference/src/operation/create_policy.rs
+++ generated/src/operation/create_policy.rs
@@ -262,11 +262,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_policy_input::ser_create_policy_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_policy_input::ser_create_policy_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
```

### `src/operation/create_policy_version/_create_policy_version_input.rs`

```diff
--- reference/src/operation/create_policy_version/_create_policy_version_input.rs
+++ generated/src/operation/create_policy_version/_create_policy_version_input.rs
@@ -157,12 +157,14 @@
     /// Consumes the builder and constructs a [`CreatePolicyVersionInput`](crate::operation::create_policy_version::CreatePolicyVersionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_policy_version::CreatePolicyVersionInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_policy_version::CreatePolicyVersionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_policy_version::CreatePolicyVersionInput {
             policy_arn: self.policy_arn,
             policy_document: self.policy_document,
-            set_as_default: self.set_as_default,
+            set_as_default: self.set_as_default.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/create_policy_version.rs`

```diff
--- reference/src/operation/create_policy_version.rs
+++ generated/src/operation/create_policy_version.rs
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
-            super::super::protocol_serde::shape_create_policy_version_input::ser_create_policy_version_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_policy_version_input::ser_create_policy_version_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_role/_create_role_input.rs`

```diff
--- reference/src/operation/create_role/_create_role_input.rs
+++ generated/src/operation/create_role/_create_role_input.rs
@@ -277,7 +277,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`CreateRoleInput`](crate::operation::create_role::CreateRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_role::CreateRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_role::CreateRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_role::CreateRoleInput {
             path: self.path,
             role_name: self.role_name,
```

### `src/operation/create_role.rs`

```diff
--- reference/src/operation/create_role.rs
+++ generated/src/operation/create_role.rs
@@ -240,7 +240,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::create_role::CreateRoleInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::create_role::CreateRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -265,12 +267,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_role_input::ser_create_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_role_input::ser_create_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/create_saml_provider/_create_saml_provider_input.rs`

```diff
--- reference/src/operation/create_saml_provider/_create_saml_provider_input.rs
+++ generated/src/operation/create_saml_provider/_create_saml_provider_input.rs
@@ -168,8 +168,10 @@
     /// Consumes the builder and constructs a [`CreateSamlProviderInput`](crate::operation::create_saml_provider::CreateSamlProviderInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_saml_provider::CreateSamlProviderInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_saml_provider::CreateSamlProviderInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_saml_provider::CreateSamlProviderInput {
             saml_metadata_document: self.saml_metadata_document,
             name: self.name,
```

### `src/operation/create_saml_provider/builders.rs`

```diff
--- reference/src/operation/create_saml_provider/builders.rs
+++ generated/src/operation/create_saml_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_saml_provider::CreateSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_saml_provider::CreateSAMLProviderError,
+            super::super::super::operation::create_saml_provider::CreateSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `CreateSAMLProvider`.
+/// Fluent builder constructing a request to `CreateSamlProvider`.
 ///
 /// <p>Creates an IAM resource that describes an identity provider (IdP) that supports SAML 2.0.</p>
 /// <p>The SAML provider resource that you create with this operation can be used as a principal in an IAM role's trust policy. Such a policy can enable federated users who sign in using the SAML IdP to assume the role. You can create an IAM role that supports Web-based single sign-on (SSO) to the Amazon Web Services Management Console or one that supports API access to Amazon Web Services.</p>
@@ -29,7 +29,7 @@
 /// </note>
 /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-saml.html">Enabling SAML 2.0 federated users to access the Amazon Web Services Management Console</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_saml.html">About SAML 2.0-based federation</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct CreateSAMLProviderFluentBuilder {
+pub struct CreateSamlProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::create_saml_provider::builders::CreateSamlProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -37,8 +37,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::create_saml_provider::CreateSamlProviderOutput,
-        super::super::super::operation::create_saml_provider::CreateSAMLProviderError,
-    > for CreateSAMLProviderFluentBuilder
+        super::super::super::operation::create_saml_provider::CreateSamlProviderError,
+    > for CreateSamlProviderFluentBuilder
 {
     fn send(
         self,
@@ -46,14 +46,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::create_saml_provider::CreateSamlProviderOutput,
-            super::super::super::operation::create_saml_provider::CreateSAMLProviderError,
+            super::super::super::operation::create_saml_provider::CreateSamlProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl CreateSAMLProviderFluentBuilder {
-    /// Creates a new `CreateSAMLProviderFluentBuilder`.
+impl CreateSamlProviderFluentBuilder {
+    /// Creates a new `CreateSamlProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -61,7 +61,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the CreateSAMLProvider as a reference.
+    /// Access the CreateSamlProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::create_saml_provider::builders::CreateSamlProviderInputBuilder {
         &self.inner
     }
@@ -78,7 +78,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_saml_provider::CreateSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_saml_provider::CreateSAMLProviderError,
+            super::super::super::operation::create_saml_provider::CreateSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -86,12 +86,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::create_saml_provider::CreateSAMLProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::create_saml_provider::CreateSamlProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::create_saml_provider::CreateSAMLProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::create_saml_provider::CreateSamlProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -99,7 +99,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::create_saml_provider::CreateSamlProviderOutput,
-        super::super::super::operation::create_saml_provider::CreateSAMLProviderError,
+        super::super::super::operation::create_saml_provider::CreateSamlProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/create_saml_provider.rs`

```diff
--- reference/src/operation/create_saml_provider.rs
+++ generated/src/operation/create_saml_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `CreateSAMLProvider`.
+/// Orchestration and serialization glue logic for `CreateSamlProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct CreateSAMLProvider;
-impl CreateSAMLProvider {
-    /// Creates a new `CreateSAMLProvider`
+pub struct CreateSamlProvider;
+impl CreateSamlProvider {
+    /// Creates a new `CreateSamlProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateSAMLProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateSamlProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateSAMLProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            CreateSAMLProviderRequestSerializer,
+            CreateSamlProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateSAMLProviderResponseDeserializer,
+            CreateSamlProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateSAMLProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateSAMLProviderTelemetryInputCaptureInterceptor,
+                CreateSamlProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateSAMLProviderEndpointParamsInterceptor,
+                CreateSamlProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::create_saml_provider::CreateSAMLProviderError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct CreateSAMLProviderTelemetryInputCaptureInterceptor;
+struct CreateSamlProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSAMLProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSamlProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "CreateSAMLProviderTelemetryInputCaptureInterceptor"
+        "CreateSamlProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_create_saml_provider_input::ser_create_saml_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_saml_provider_input::ser_create_saml_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct CreateSAMLProviderEndpointParamsInterceptor;
+struct CreateSamlProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSAMLProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateSamlProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "CreateSAMLProviderEndpointParamsInterceptor"
+        "CreateSamlProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/create_service_linked_role.rs`

```diff
--- reference/src/operation/create_service_linked_role.rs
+++ generated/src/operation/create_service_linked_role.rs
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
-            super::super::protocol_serde::shape_create_service_linked_role_input::ser_create_service_linked_role_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_service_linked_role_input::ser_create_service_linked_role_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_service_specific_credential/builders.rs`

```diff
--- reference/src/operation/create_service_specific_credential/builders.rs
+++ generated/src/operation/create_service_specific_credential/builders.rs
@@ -68,7 +68,9 @@
         }
     }
     /// Access the CreateServiceSpecificCredential as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::create_service_specific_credential::builders::CreateServiceSpecificCredentialInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::create_service_specific_credential::builders::CreateServiceSpecificCredentialInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -92,12 +94,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::create_service_specific_credential::CreateServiceSpecificCredential::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::create_service_specific_credential::CreateServiceSpecificCredential::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::create_service_specific_credential::CreateServiceSpecificCredential::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::create_service_specific_credential::CreateServiceSpecificCredential::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/create_service_specific_credential.rs`

```diff
--- reference/src/operation/create_service_specific_credential.rs
+++ generated/src/operation/create_service_specific_credential.rs
@@ -219,7 +219,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_create_service_specific_credential::de_create_service_specific_credential_http_error(status, headers, body)
+            super::super::protocol_serde::shape_create_service_specific_credential::de_create_service_specific_credential_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_create_service_specific_credential::de_create_service_specific_credential_http_response(
                 status, headers, body,
@@ -264,12 +266,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_create_service_specific_credential_input::ser_create_service_specific_credential_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_service_specific_credential_input::ser_create_service_specific_credential_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/create_user/_create_user_input.rs`

```diff
--- reference/src/operation/create_user/_create_user_input.rs
+++ generated/src/operation/create_user/_create_user_input.rs
@@ -148,7 +148,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`CreateUserInput`](crate::operation::create_user::CreateUserInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_user::CreateUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_user::CreateUserInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_user::CreateUserInput {
             path: self.path,
             user_name: self.user_name,
```

### `src/operation/create_user.rs`

```diff
--- reference/src/operation/create_user.rs
+++ generated/src/operation/create_user.rs
@@ -230,7 +230,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::create_user::CreateUserInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::create_user::CreateUserInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -255,12 +257,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_user_input::ser_create_user_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_user_input::ser_create_user_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/create_virtual_mfa_device/builders.rs`

```diff
--- reference/src/operation/create_virtual_mfa_device/builders.rs
+++ generated/src/operation/create_virtual_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
+            super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `CreateVirtualMFADevice`.
+/// Fluent builder constructing a request to `CreateVirtualMfaDevice`.
 ///
 /// <p>Creates a new virtual MFA device for the Amazon Web Services account. After creating the virtual MFA, use <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_EnableMFADevice.html">EnableMFADevice</a> to attach the MFA device to an IAM user. For more information about creating and working with virtual MFA devices, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_VirtualMFA.html">Using a virtual MFA device</a> in the <i>IAM User Guide</i>.</p>
 /// <p>For information about the maximum number of MFA devices you can create, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html">IAM and STS quotas</a> in the <i>IAM User Guide</i>.</p><important>
@@ -27,7 +27,7 @@
 /// <p>The seed information contained in the QR code and the Base32 string should be treated like any other secret access information. In other words, protect the seed information as you would your Amazon Web Services access keys or your passwords. After you provision your virtual device, you should ensure that the information is destroyed following secure procedures.</p>
 /// </important>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct CreateVirtualMFADeviceFluentBuilder {
+pub struct CreateVirtualMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::create_virtual_mfa_device::builders::CreateVirtualMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -35,8 +35,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput,
-        super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
-    > for CreateVirtualMFADeviceFluentBuilder
+        super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceError,
+    > for CreateVirtualMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -44,14 +44,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput,
-            super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
+            super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl CreateVirtualMFADeviceFluentBuilder {
-    /// Creates a new `CreateVirtualMFADeviceFluentBuilder`.
+impl CreateVirtualMfaDeviceFluentBuilder {
+    /// Creates a new `CreateVirtualMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -59,7 +59,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the CreateVirtualMFADevice as a reference.
+    /// Access the CreateVirtualMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::create_virtual_mfa_device::builders::CreateVirtualMfaDeviceInputBuilder {
         &self.inner
     }
@@ -76,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
+            super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,12 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -97,7 +97,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput,
-        super::super::super::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
+        super::super::super::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/create_virtual_mfa_device.rs`

```diff
--- reference/src/operation/create_virtual_mfa_device.rs
+++ generated/src/operation/create_virtual_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `CreateVirtualMFADevice`.
+/// Orchestration and serialization glue logic for `CreateVirtualMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct CreateVirtualMFADevice;
-impl CreateVirtualMFADevice {
-    /// Creates a new `CreateVirtualMFADevice`
+pub struct CreateVirtualMfaDevice;
+impl CreateVirtualMfaDevice {
+    /// Creates a new `CreateVirtualMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateVirtualMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateVirtualMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateVirtualMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            CreateVirtualMFADeviceRequestSerializer,
+            CreateVirtualMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            CreateVirtualMFADeviceResponseDeserializer,
+            CreateVirtualMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -128,13 +128,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateVirtualMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateVirtualMFADeviceTelemetryInputCaptureInterceptor,
+                CreateVirtualMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateVirtualMFADeviceEndpointParamsInterceptor,
+                CreateVirtualMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
@@ -151,12 +151,12 @@
 }

 #[derive(Debug)]
-struct CreateVirtualMFADeviceTelemetryInputCaptureInterceptor;
+struct CreateVirtualMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateVirtualMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateVirtualMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "CreateVirtualMFADeviceTelemetryInputCaptureInterceptor"
+        "CreateVirtualMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -256,12 +256,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_create_virtual_mfa_device_input::ser_create_virtual_mfa_device_input_input_input(&input)?,
+            super::super::protocol_serde::shape_create_virtual_mfa_device_input::ser_create_virtual_mfa_device_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -271,12 +270,12 @@
     }
 }
 #[derive(Debug)]
-struct CreateVirtualMFADeviceEndpointParamsInterceptor;
+struct CreateVirtualMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateVirtualMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateVirtualMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "CreateVirtualMFADeviceEndpointParamsInterceptor"
+        "CreateVirtualMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/deactivate_mfa_device/_deactivate_mfa_device_input.rs`

```diff
--- reference/src/operation/deactivate_mfa_device/_deactivate_mfa_device_input.rs
+++ generated/src/operation/deactivate_mfa_device/_deactivate_mfa_device_input.rs
@@ -80,8 +80,10 @@
     /// Consumes the builder and constructs a [`DeactivateMfaDeviceInput`](crate::operation::deactivate_mfa_device::DeactivateMfaDeviceInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceInput {
             user_name: self.user_name,
             serial_number: self.serial_number,
```

### `src/operation/deactivate_mfa_device/builders.rs`

```diff
--- reference/src/operation/deactivate_mfa_device/builders.rs
+++ generated/src/operation/deactivate_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::deactivate_mfa_device::DeactivateMFADeviceError,
+            super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `DeactivateMFADevice`.
+/// Fluent builder constructing a request to `DeactivateMfaDevice`.
 ///
 /// <p>Deactivates the specified MFA device and removes it from association with the user name for which it was originally enabled.</p>
 /// <p>For more information about creating and working with virtual MFA devices, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_VirtualMFA.html">Enabling a virtual multi-factor authentication (MFA) device</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct DeactivateMFADeviceFluentBuilder {
+pub struct DeactivateMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::deactivate_mfa_device::builders::DeactivateMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput,
-        super::super::super::operation::deactivate_mfa_device::DeactivateMFADeviceError,
-    > for DeactivateMFADeviceFluentBuilder
+        super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceError,
+    > for DeactivateMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput,
-            super::super::super::operation::deactivate_mfa_device::DeactivateMFADeviceError,
+            super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeactivateMFADeviceFluentBuilder {
-    /// Creates a new `DeactivateMFADeviceFluentBuilder`.
+impl DeactivateMfaDeviceFluentBuilder {
+    /// Creates a new `DeactivateMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeactivateMFADevice as a reference.
+    /// Access the DeactivateMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::deactivate_mfa_device::builders::DeactivateMfaDeviceInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::deactivate_mfa_device::DeactivateMFADeviceError,
+            super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::deactivate_mfa_device::DeactivateMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::deactivate_mfa_device::DeactivateMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::deactivate_mfa_device::DeactivateMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::deactivate_mfa_device::DeactivateMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput,
-        super::super::super::operation::deactivate_mfa_device::DeactivateMFADeviceError,
+        super::super::super::operation::deactivate_mfa_device::DeactivateMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/deactivate_mfa_device.rs`

```diff
--- reference/src/operation/deactivate_mfa_device.rs
+++ generated/src/operation/deactivate_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `DeactivateMFADevice`.
+/// Orchestration and serialization glue logic for `DeactivateMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct DeactivateMFADevice;
-impl DeactivateMFADevice {
-    /// Creates a new `DeactivateMFADevice`
+pub struct DeactivateMfaDevice;
+impl DeactivateMfaDevice {
+    /// Creates a new `DeactivateMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeactivateMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeactivateMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeactivateMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            DeactivateMFADeviceRequestSerializer,
+            DeactivateMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DeactivateMFADeviceResponseDeserializer,
+            DeactivateMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeactivateMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeactivateMFADeviceTelemetryInputCaptureInterceptor,
+                DeactivateMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeactivateMFADeviceEndpointParamsInterceptor,
+                DeactivateMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::deactivate_mfa_device::DeactivateMFADeviceError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct DeactivateMFADeviceTelemetryInputCaptureInterceptor;
+struct DeactivateMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeactivateMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeactivateMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "DeactivateMFADeviceTelemetryInputCaptureInterceptor"
+        "DeactivateMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_deactivate_mfa_device_input::ser_deactivate_mfa_device_input_input_input(&input)?,
+            super::super::protocol_serde::shape_deactivate_mfa_device_input::ser_deactivate_mfa_device_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -270,12 +269,12 @@
     }
 }
 #[derive(Debug)]
-struct DeactivateMFADeviceEndpointParamsInterceptor;
+struct DeactivateMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeactivateMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeactivateMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "DeactivateMFADeviceEndpointParamsInterceptor"
+        "DeactivateMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/delete_access_key/_delete_access_key_input.rs`

```diff
--- reference/src/operation/delete_access_key/_delete_access_key_input.rs
+++ generated/src/operation/delete_access_key/_delete_access_key_input.rs
@@ -75,7 +75,10 @@
     /// Consumes the builder and constructs a [`DeleteAccessKeyInput`](crate::operation::delete_access_key::DeleteAccessKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_access_key::DeleteAccessKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_access_key::DeleteAccessKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_access_key::DeleteAccessKeyInput {
             user_name: self.user_name,
             access_key_id: self.access_key_id,
```

### `src/operation/delete_access_key.rs`

```diff
--- reference/src/operation/delete_access_key.rs
+++ generated/src/operation/delete_access_key.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_access_key_input::ser_delete_access_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_access_key_input::ser_delete_access_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_account_alias/_delete_account_alias_input.rs`

```diff
--- reference/src/operation/delete_account_alias/_delete_account_alias_input.rs
+++ generated/src/operation/delete_account_alias/_delete_account_alias_input.rs
@@ -49,8 +49,10 @@
     /// Consumes the builder and constructs a [`DeleteAccountAliasInput`](crate::operation::delete_account_alias::DeleteAccountAliasInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_account_alias::DeleteAccountAliasInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_account_alias::DeleteAccountAliasInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_account_alias::DeleteAccountAliasInput {
             account_alias: self.account_alias,
         })
```

### `src/operation/delete_account_alias.rs`

```diff
--- reference/src/operation/delete_account_alias.rs
+++ generated/src/operation/delete_account_alias.rs
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
-            super::super::protocol_serde::shape_delete_account_alias_input::ser_delete_account_alias_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_account_alias_input::ser_delete_account_alias_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_account_password_policy.rs`

```diff
--- reference/src/operation/delete_account_password_policy.rs
+++ generated/src/operation/delete_account_password_policy.rs
@@ -204,13 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_account_password_policy_input::ser_delete_account_password_policy_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/delete_group/_delete_group_input.rs`

```diff
--- reference/src/operation/delete_group/_delete_group_input.rs
+++ generated/src/operation/delete_group/_delete_group_input.rs
@@ -47,7 +47,9 @@
         &self.group_name
     }
     /// Consumes the builder and constructs a [`DeleteGroupInput`](crate::operation::delete_group::DeleteGroupInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_group::DeleteGroupInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_group::DeleteGroupInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_group::DeleteGroupInput { group_name: self.group_name })
     }
 }
```

### `src/operation/delete_group.rs`

```diff
--- reference/src/operation/delete_group.rs
+++ generated/src/operation/delete_group.rs
@@ -247,13 +247,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_group_input::ser_delete_group_input_input_input(
-            &input,
-        )?);
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_group_input::ser_delete_group_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_group_policy/_delete_group_policy_input.rs`

```diff
--- reference/src/operation/delete_group_policy/_delete_group_policy_input.rs
+++ generated/src/operation/delete_group_policy/_delete_group_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`DeleteGroupPolicyInput`](crate::operation::delete_group_policy::DeleteGroupPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_group_policy::DeleteGroupPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_group_policy::DeleteGroupPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_group_policy::DeleteGroupPolicyInput {
             group_name: self.group_name,
             policy_name: self.policy_name,
```

### `src/operation/delete_group_policy.rs`

```diff
--- reference/src/operation/delete_group_policy.rs
+++ generated/src/operation/delete_group_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_group_policy_input::ser_delete_group_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_group_policy_input::ser_delete_group_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_instance_profile/_delete_instance_profile_input.rs`

```diff
--- reference/src/operation/delete_instance_profile/_delete_instance_profile_input.rs
+++ generated/src/operation/delete_instance_profile/_delete_instance_profile_input.rs
@@ -49,8 +49,10 @@
     /// Consumes the builder and constructs a [`DeleteInstanceProfileInput`](crate::operation::delete_instance_profile::DeleteInstanceProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_instance_profile::DeleteInstanceProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_instance_profile::DeleteInstanceProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_instance_profile::DeleteInstanceProfileInput {
             instance_profile_name: self.instance_profile_name,
         })
```

### `src/operation/delete_instance_profile.rs`

```diff
--- reference/src/operation/delete_instance_profile.rs
+++ generated/src/operation/delete_instance_profile.rs
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
-            super::super::protocol_serde::shape_delete_instance_profile_input::ser_delete_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_instance_profile_input::ser_delete_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_login_profile/_delete_login_profile_input.rs`

```diff
--- reference/src/operation/delete_login_profile/_delete_login_profile_input.rs
+++ generated/src/operation/delete_login_profile/_delete_login_profile_input.rs
@@ -53,8 +53,10 @@
     /// Consumes the builder and constructs a [`DeleteLoginProfileInput`](crate::operation::delete_login_profile::DeleteLoginProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_login_profile::DeleteLoginProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_login_profile::DeleteLoginProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_login_profile::DeleteLoginProfileInput { user_name: self.user_name })
     }
 }
```

### `src/operation/delete_login_profile.rs`

```diff
--- reference/src/operation/delete_login_profile.rs
+++ generated/src/operation/delete_login_profile.rs
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
-            super::super::protocol_serde::shape_delete_login_profile_input::ser_delete_login_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_login_profile_input::ser_delete_login_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_open_id_connect_provider/_delete_open_id_connect_provider_input.rs`

```diff
--- reference/src/operation/delete_open_id_connect_provider/_delete_open_id_connect_provider_input.rs
+++ generated/src/operation/delete_open_id_connect_provider/_delete_open_id_connect_provider_input.rs
@@ -48,8 +48,10 @@
         super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderInput {
-            open_id_connect_provider_arn: self.open_id_connect_provider_arn,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderInput {
+                open_id_connect_provider_arn: self.open_id_connect_provider_arn,
+            },
+        )
     }
 }
```

### `src/operation/delete_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/delete_open_id_connect_provider/builders.rs
+++ generated/src/operation/delete_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProviderError,
+            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,13 +20,13 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `DeleteOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `DeleteOpenIdConnectProvider`.
 ///
 /// <p>Deletes an OpenID Connect identity provider (IdP) resource object in IAM.</p>
 /// <p>Deleting an IAM OIDC provider resource does not update any roles that reference the provider as a principal in their trust policies. Any attempt to assume a role that references a deleted provider fails.</p>
 /// <p>This operation is idempotent; it does not fail or return an error if you call the operation for a provider that does not exist.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct DeleteOpenIDConnectProviderFluentBuilder {
+pub struct DeleteOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::delete_open_id_connect_provider::builders::DeleteOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -34,8 +34,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput,
-        super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProviderError,
-    > for DeleteOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderError,
+    > for DeleteOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -43,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput,
-            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProviderError,
+            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeleteOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `DeleteOpenIDConnectProviderFluentBuilder`.
+impl DeleteOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `DeleteOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -58,7 +58,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeleteOpenIDConnectProvider as a reference.
+    /// Access the DeleteOpenIdConnectProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::delete_open_id_connect_provider::builders::DeleteOpenIdConnectProviderInputBuilder {
         &self.inner
     }
@@ -75,7 +75,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProviderError,
+            super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +96,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput,
-        super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProviderError,
+        super::super::super::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/delete_open_id_connect_provider.rs`

```diff
--- reference/src/operation/delete_open_id_connect_provider.rs
+++ generated/src/operation/delete_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `DeleteOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `DeleteOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct DeleteOpenIDConnectProvider;
-impl DeleteOpenIDConnectProvider {
-    /// Creates a new `DeleteOpenIDConnectProvider`
+pub struct DeleteOpenIdConnectProvider;
+impl DeleteOpenIdConnectProvider {
+    /// Creates a new `DeleteOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            DeleteOpenIDConnectProviderRequestSerializer,
+            DeleteOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DeleteOpenIDConnectProviderResponseDeserializer,
+            DeleteOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                DeleteOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteOpenIDConnectProviderEndpointParamsInterceptor,
+                DeleteOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::delete_open_id_connect_provider::DeleteOpenIDConnectProviderError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct DeleteOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct DeleteOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "DeleteOpenIdConnectProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -209,7 +209,9 @@
         let parse_result = if !success && status != 200 || force_error {
             super::super::protocol_serde::shape_delete_open_id_connect_provider::de_delete_open_id_connect_provider_http_error(status, headers, body)
         } else {
-            super::super::protocol_serde::shape_delete_open_id_connect_provider::de_delete_open_id_connect_provider_http_response(status, headers, body)
+            super::super::protocol_serde::shape_delete_open_id_connect_provider::de_delete_open_id_connect_provider_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -250,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_open_id_connect_provider_input::ser_delete_open_id_connect_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_open_id_connect_provider_input::ser_delete_open_id_connect_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct DeleteOpenIDConnectProviderEndpointParamsInterceptor;
+struct DeleteOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteOpenIDConnectProviderEndpointParamsInterceptor"
+        "DeleteOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/delete_policy.rs`

```diff
--- reference/src/operation/delete_policy.rs
+++ generated/src/operation/delete_policy.rs
@@ -247,11 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_policy_input::ser_delete_policy_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_policy_input::ser_delete_policy_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
```

### `src/operation/delete_policy_version/_delete_policy_version_input.rs`

```diff
--- reference/src/operation/delete_policy_version/_delete_policy_version_input.rs
+++ generated/src/operation/delete_policy_version/_delete_policy_version_input.rs
@@ -81,8 +81,10 @@
     /// Consumes the builder and constructs a [`DeletePolicyVersionInput`](crate::operation::delete_policy_version::DeletePolicyVersionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_policy_version::DeletePolicyVersionInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_policy_version::DeletePolicyVersionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_policy_version::DeletePolicyVersionInput {
             policy_arn: self.policy_arn,
             version_id: self.version_id,
```

### `src/operation/delete_policy_version.rs`

```diff
--- reference/src/operation/delete_policy_version.rs
+++ generated/src/operation/delete_policy_version.rs
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
-            super::super::protocol_serde::shape_delete_policy_version_input::ser_delete_policy_version_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_policy_version_input::ser_delete_policy_version_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_role/_delete_role_input.rs`

```diff
--- reference/src/operation/delete_role/_delete_role_input.rs
+++ generated/src/operation/delete_role/_delete_role_input.rs
@@ -47,7 +47,9 @@
         &self.role_name
     }
     /// Consumes the builder and constructs a [`DeleteRoleInput`](crate::operation::delete_role::DeleteRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_role::DeleteRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_role::DeleteRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_role::DeleteRoleInput { role_name: self.role_name })
     }
 }
```

### `src/operation/delete_role.rs`

```diff
--- reference/src/operation/delete_role.rs
+++ generated/src/operation/delete_role.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::delete_role::DeleteRoleInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::delete_role::DeleteRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -245,12 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_role_input::ser_delete_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_role_input::ser_delete_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_role_permissions_boundary/_delete_role_permissions_boundary_input.rs`

```diff
--- reference/src/operation/delete_role_permissions_boundary/_delete_role_permissions_boundary_input.rs
+++ generated/src/operation/delete_role_permissions_boundary/_delete_role_permissions_boundary_input.rs
@@ -48,8 +48,8 @@
         super::super::super::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput {
-            role_name: self.role_name,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput { role_name: self.role_name },
+        )
     }
 }
```

### `src/operation/delete_role_permissions_boundary.rs`

```diff
--- reference/src/operation/delete_role_permissions_boundary.rs
+++ generated/src/operation/delete_role_permissions_boundary.rs
@@ -207,9 +207,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_delete_role_permissions_boundary::de_delete_role_permissions_boundary_http_error(status, headers, body)
+            super::super::protocol_serde::shape_delete_role_permissions_boundary::de_delete_role_permissions_boundary_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_delete_role_permissions_boundary::de_delete_role_permissions_boundary_http_response(status, headers, body)
+            super::super::protocol_serde::shape_delete_role_permissions_boundary::de_delete_role_permissions_boundary_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -250,12 +254,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_role_permissions_boundary_input::ser_delete_role_permissions_boundary_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_role_permissions_boundary_input::ser_delete_role_permissions_boundary_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_role_policy/_delete_role_policy_input.rs`

```diff
--- reference/src/operation/delete_role_policy/_delete_role_policy_input.rs
+++ generated/src/operation/delete_role_policy/_delete_role_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`DeleteRolePolicyInput`](crate::operation::delete_role_policy::DeleteRolePolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_role_policy::DeleteRolePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_role_policy::DeleteRolePolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_role_policy::DeleteRolePolicyInput {
             role_name: self.role_name,
             policy_name: self.policy_name,
```

### `src/operation/delete_role_policy.rs`

```diff
--- reference/src/operation/delete_role_policy.rs
+++ generated/src/operation/delete_role_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_role_policy_input::ser_delete_role_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_role_policy_input::ser_delete_role_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_saml_provider/_delete_saml_provider_input.rs`

```diff
--- reference/src/operation/delete_saml_provider/_delete_saml_provider_input.rs
+++ generated/src/operation/delete_saml_provider/_delete_saml_provider_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`DeleteSamlProviderInput`](crate::operation::delete_saml_provider::DeleteSamlProviderInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_saml_provider::DeleteSamlProviderInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_saml_provider::DeleteSamlProviderInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_saml_provider::DeleteSamlProviderInput {
             saml_provider_arn: self.saml_provider_arn,
         })
```

### `src/operation/delete_saml_provider/builders.rs`

```diff
--- reference/src/operation/delete_saml_provider/builders.rs
+++ generated/src/operation/delete_saml_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_saml_provider::DeleteSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_saml_provider::DeleteSAMLProviderError,
+            super::super::super::operation::delete_saml_provider::DeleteSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `DeleteSAMLProvider`.
+/// Fluent builder constructing a request to `DeleteSamlProvider`.
 ///
 /// <p>Deletes a SAML provider resource in IAM.</p>
 /// <p>Deleting the provider resource from IAM does not update any roles that reference the SAML provider resource's ARN as a principal in their trust policies. Any attempt to assume a role that references a non-existent provider resource ARN fails.</p><note>
@@ -27,7 +27,7 @@
 /// <p>This operation requires <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4</a>.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct DeleteSAMLProviderFluentBuilder {
+pub struct DeleteSamlProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::delete_saml_provider::builders::DeleteSamlProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -35,8 +35,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::delete_saml_provider::DeleteSamlProviderOutput,
-        super::super::super::operation::delete_saml_provider::DeleteSAMLProviderError,
-    > for DeleteSAMLProviderFluentBuilder
+        super::super::super::operation::delete_saml_provider::DeleteSamlProviderError,
+    > for DeleteSamlProviderFluentBuilder
 {
     fn send(
         self,
@@ -44,14 +44,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::delete_saml_provider::DeleteSamlProviderOutput,
-            super::super::super::operation::delete_saml_provider::DeleteSAMLProviderError,
+            super::super::super::operation::delete_saml_provider::DeleteSamlProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeleteSAMLProviderFluentBuilder {
-    /// Creates a new `DeleteSAMLProviderFluentBuilder`.
+impl DeleteSamlProviderFluentBuilder {
+    /// Creates a new `DeleteSamlProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -59,7 +59,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeleteSAMLProvider as a reference.
+    /// Access the DeleteSamlProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::delete_saml_provider::builders::DeleteSamlProviderInputBuilder {
         &self.inner
     }
@@ -76,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_saml_provider::DeleteSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_saml_provider::DeleteSAMLProviderError,
+            super::super::super::operation::delete_saml_provider::DeleteSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,12 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::delete_saml_provider::DeleteSAMLProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::delete_saml_provider::DeleteSamlProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::delete_saml_provider::DeleteSAMLProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::delete_saml_provider::DeleteSamlProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -97,7 +97,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::delete_saml_provider::DeleteSamlProviderOutput,
-        super::super::super::operation::delete_saml_provider::DeleteSAMLProviderError,
+        super::super::super::operation::delete_saml_provider::DeleteSamlProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/delete_saml_provider.rs`

```diff
--- reference/src/operation/delete_saml_provider.rs
+++ generated/src/operation/delete_saml_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `DeleteSAMLProvider`.
+/// Orchestration and serialization glue logic for `DeleteSamlProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct DeleteSAMLProvider;
-impl DeleteSAMLProvider {
-    /// Creates a new `DeleteSAMLProvider`
+pub struct DeleteSamlProvider;
+impl DeleteSamlProvider {
+    /// Creates a new `DeleteSamlProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteSAMLProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteSamlProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteSAMLProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            DeleteSAMLProviderRequestSerializer,
+            DeleteSamlProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DeleteSAMLProviderResponseDeserializer,
+            DeleteSamlProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteSAMLProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteSAMLProviderTelemetryInputCaptureInterceptor,
+                DeleteSamlProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteSAMLProviderEndpointParamsInterceptor,
+                DeleteSamlProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::delete_saml_provider::DeleteSAMLProviderError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct DeleteSAMLProviderTelemetryInputCaptureInterceptor;
+struct DeleteSamlProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSAMLProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSamlProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteSAMLProviderTelemetryInputCaptureInterceptor"
+        "DeleteSamlProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_delete_saml_provider_input::ser_delete_saml_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_saml_provider_input::ser_delete_saml_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -262,12 +261,12 @@
     }
 }
 #[derive(Debug)]
-struct DeleteSAMLProviderEndpointParamsInterceptor;
+struct DeleteSamlProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSAMLProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSamlProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteSAMLProviderEndpointParamsInterceptor"
+        "DeleteSamlProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/delete_server_certificate.rs`

```diff
--- reference/src/operation/delete_server_certificate.rs
+++ generated/src/operation/delete_server_certificate.rs
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
-            super::super::protocol_serde::shape_delete_server_certificate_input::ser_delete_server_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_server_certificate_input::ser_delete_server_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_service_linked_role/_delete_service_linked_role_input.rs`

```diff
--- reference/src/operation/delete_service_linked_role/_delete_service_linked_role_input.rs
+++ generated/src/operation/delete_service_linked_role/_delete_service_linked_role_input.rs
@@ -48,6 +48,8 @@
         super::super::super::operation::delete_service_linked_role::DeleteServiceLinkedRoleInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_service_linked_role::DeleteServiceLinkedRoleInput { role_name: self.role_name })
+        ::std::result::Result::Ok(super::super::super::operation::delete_service_linked_role::DeleteServiceLinkedRoleInput {
+            role_name: self.role_name,
+        })
     }
 }
```

### `src/operation/delete_service_linked_role/_delete_service_linked_role_output.rs`

```diff
--- reference/src/operation/delete_service_linked_role/_delete_service_linked_role_output.rs
+++ generated/src/operation/delete_service_linked_role/_delete_service_linked_role_output.rs
@@ -97,14 +97,16 @@
         super::super::super::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput {
-            deletion_task_id: self.deletion_task_id.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "deletion_task_id",
-                    "deletion_task_id was not specified but it is required when building DeleteServiceLinkedRoleOutput",
-                )
-            })?,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput {
+                deletion_task_id: self.deletion_task_id.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "deletion_task_id",
+                        "deletion_task_id was not specified but it is required when building DeleteServiceLinkedRoleOutput",
+                    )
+                })?,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/delete_service_linked_role.rs`

```diff
--- reference/src/operation/delete_service_linked_role.rs
+++ generated/src/operation/delete_service_linked_role.rs
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
-            super::super::protocol_serde::shape_delete_service_linked_role_input::ser_delete_service_linked_role_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_service_linked_role_input::ser_delete_service_linked_role_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_service_specific_credential/builders.rs`

```diff
--- reference/src/operation/delete_service_specific_credential/builders.rs
+++ generated/src/operation/delete_service_specific_credential/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the DeleteServiceSpecificCredential as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::delete_service_specific_credential::builders::DeleteServiceSpecificCredentialInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::delete_service_specific_credential::builders::DeleteServiceSpecificCredentialInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::delete_service_specific_credential::DeleteServiceSpecificCredential::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::delete_service_specific_credential::DeleteServiceSpecificCredential::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::delete_service_specific_credential::DeleteServiceSpecificCredential::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::delete_service_specific_credential::DeleteServiceSpecificCredential::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/delete_service_specific_credential.rs`

```diff
--- reference/src/operation/delete_service_specific_credential.rs
+++ generated/src/operation/delete_service_specific_credential.rs
@@ -218,7 +218,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_delete_service_specific_credential::de_delete_service_specific_credential_http_error(status, headers, body)
+            super::super::protocol_serde::shape_delete_service_specific_credential::de_delete_service_specific_credential_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_delete_service_specific_credential::de_delete_service_specific_credential_http_response(
                 status, headers, body,
@@ -263,12 +265,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_service_specific_credential_input::ser_delete_service_specific_credential_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_service_specific_credential_input::ser_delete_service_specific_credential_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_signing_certificate/_delete_signing_certificate_input.rs`

```diff
--- reference/src/operation/delete_signing_certificate/_delete_signing_certificate_input.rs
+++ generated/src/operation/delete_signing_certificate/_delete_signing_certificate_input.rs
@@ -79,9 +79,11 @@
         super::super::super::operation::delete_signing_certificate::DeleteSigningCertificateInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_signing_certificate::DeleteSigningCertificateInput {
-            user_name: self.user_name,
-            certificate_id: self.certificate_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::delete_signing_certificate::DeleteSigningCertificateInput {
+                user_name: self.user_name,
+                certificate_id: self.certificate_id,
+            },
+        )
     }
 }
```

### `src/operation/delete_signing_certificate.rs`

```diff
--- reference/src/operation/delete_signing_certificate.rs
+++ generated/src/operation/delete_signing_certificate.rs
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
-            super::super::protocol_serde::shape_delete_signing_certificate_input::ser_delete_signing_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_signing_certificate_input::ser_delete_signing_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_ssh_public_key/_delete_ssh_public_key_input.rs`

```diff
--- reference/src/operation/delete_ssh_public_key/_delete_ssh_public_key_input.rs
+++ generated/src/operation/delete_ssh_public_key/_delete_ssh_public_key_input.rs
@@ -76,8 +76,10 @@
     /// Consumes the builder and constructs a [`DeleteSshPublicKeyInput`](crate::operation::delete_ssh_public_key::DeleteSshPublicKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyInput {
             user_name: self.user_name,
             ssh_public_key_id: self.ssh_public_key_id,
```

### `src/operation/delete_ssh_public_key/builders.rs`

```diff
--- reference/src/operation/delete_ssh_public_key/builders.rs
+++ generated/src/operation/delete_ssh_public_key/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKeyError,
+            super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `DeleteSSHPublicKey`.
+/// Fluent builder constructing a request to `DeleteSshPublicKey`.
 ///
 /// <p>Deletes the specified SSH public key.</p>
 /// <p>The SSH public key deleted by this operation is used only for authenticating the associated IAM user to an CodeCommit repository. For more information about using SSH keys to authenticate to an CodeCommit repository, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/setting-up-credentials-ssh.html">Set up CodeCommit for SSH connections</a> in the <i>CodeCommit User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct DeleteSSHPublicKeyFluentBuilder {
+pub struct DeleteSshPublicKeyFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::delete_ssh_public_key::builders::DeleteSshPublicKeyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput,
-        super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKeyError,
-    > for DeleteSSHPublicKeyFluentBuilder
+        super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyError,
+    > for DeleteSshPublicKeyFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput,
-            super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKeyError,
+            super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeleteSSHPublicKeyFluentBuilder {
-    /// Creates a new `DeleteSSHPublicKeyFluentBuilder`.
+impl DeleteSshPublicKeyFluentBuilder {
+    /// Creates a new `DeleteSshPublicKeyFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeleteSSHPublicKey as a reference.
+    /// Access the DeleteSshPublicKey as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::delete_ssh_public_key::builders::DeleteSshPublicKeyInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKeyError,
+            super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKey::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKey::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKey::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKey::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput,
-        super::super::super::operation::delete_ssh_public_key::DeleteSSHPublicKeyError,
+        super::super::super::operation::delete_ssh_public_key::DeleteSshPublicKeyError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/delete_ssh_public_key.rs`

```diff
--- reference/src/operation/delete_ssh_public_key.rs
+++ generated/src/operation/delete_ssh_public_key.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `DeleteSSHPublicKey`.
+/// Orchestration and serialization glue logic for `DeleteSshPublicKey`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct DeleteSSHPublicKey;
-impl DeleteSSHPublicKey {
-    /// Creates a new `DeleteSSHPublicKey`
+pub struct DeleteSshPublicKey;
+impl DeleteSshPublicKey {
+    /// Creates a new `DeleteSshPublicKey`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteSSHPublicKey {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteSshPublicKey {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteSSHPublicKey");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            DeleteSSHPublicKeyRequestSerializer,
+            DeleteSshPublicKeyRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DeleteSSHPublicKeyResponseDeserializer,
+            DeleteSshPublicKeyResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteSSHPublicKey")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteSSHPublicKeyTelemetryInputCaptureInterceptor,
+                DeleteSshPublicKeyTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteSSHPublicKeyEndpointParamsInterceptor,
+                DeleteSshPublicKeyEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::delete_ssh_public_key::DeleteSSHPublicKeyError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct DeleteSSHPublicKeyTelemetryInputCaptureInterceptor;
+struct DeleteSshPublicKeyTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSSHPublicKeyTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSshPublicKeyTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteSSHPublicKeyTelemetryInputCaptureInterceptor"
+        "DeleteSshPublicKeyTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_ssh_public_key_input::ser_delete_ssh_public_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_ssh_public_key_input::ser_delete_ssh_public_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct DeleteSSHPublicKeyEndpointParamsInterceptor;
+struct DeleteSshPublicKeyEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSSHPublicKeyEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteSshPublicKeyEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteSSHPublicKeyEndpointParamsInterceptor"
+        "DeleteSshPublicKeyEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/delete_user/_delete_user_input.rs`

```diff
--- reference/src/operation/delete_user/_delete_user_input.rs
+++ generated/src/operation/delete_user/_delete_user_input.rs
@@ -47,7 +47,9 @@
         &self.user_name
     }
     /// Consumes the builder and constructs a [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_user::DeleteUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_user::DeleteUserInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_user::DeleteUserInput { user_name: self.user_name })
     }
 }
```

### `src/operation/delete_user.rs`

```diff
--- reference/src/operation/delete_user.rs
+++ generated/src/operation/delete_user.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::delete_user::DeleteUserInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::delete_user::DeleteUserInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -245,12 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_user_input::ser_delete_user_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_user_input::ser_delete_user_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_user_permissions_boundary/_delete_user_permissions_boundary_input.rs`

```diff
--- reference/src/operation/delete_user_permissions_boundary/_delete_user_permissions_boundary_input.rs
+++ generated/src/operation/delete_user_permissions_boundary/_delete_user_permissions_boundary_input.rs
@@ -48,8 +48,8 @@
         super::super::super::operation::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryInput {
-            user_name: self.user_name,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryInput { user_name: self.user_name },
+        )
     }
 }
```

### `src/operation/delete_user_permissions_boundary.rs`

```diff
--- reference/src/operation/delete_user_permissions_boundary.rs
+++ generated/src/operation/delete_user_permissions_boundary.rs
@@ -207,9 +207,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_delete_user_permissions_boundary::de_delete_user_permissions_boundary_http_error(status, headers, body)
+            super::super::protocol_serde::shape_delete_user_permissions_boundary::de_delete_user_permissions_boundary_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_delete_user_permissions_boundary::de_delete_user_permissions_boundary_http_response(status, headers, body)
+            super::super::protocol_serde::shape_delete_user_permissions_boundary::de_delete_user_permissions_boundary_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -250,12 +254,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_user_permissions_boundary_input::ser_delete_user_permissions_boundary_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_user_permissions_boundary_input::ser_delete_user_permissions_boundary_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_user_policy/_delete_user_policy_input.rs`

```diff
--- reference/src/operation/delete_user_policy/_delete_user_policy_input.rs
+++ generated/src/operation/delete_user_policy/_delete_user_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`DeleteUserPolicyInput`](crate::operation::delete_user_policy::DeleteUserPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_user_policy::DeleteUserPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_user_policy::DeleteUserPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_user_policy::DeleteUserPolicyInput {
             user_name: self.user_name,
             policy_name: self.policy_name,
```

### `src/operation/delete_user_policy.rs`

```diff
--- reference/src/operation/delete_user_policy.rs
+++ generated/src/operation/delete_user_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_delete_user_policy_input::ser_delete_user_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_user_policy_input::ser_delete_user_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/delete_virtual_mfa_device/builders.rs`

```diff
--- reference/src/operation/delete_virtual_mfa_device/builders.rs
+++ generated/src/operation/delete_virtual_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADeviceError,
+            super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,13 +20,13 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `DeleteVirtualMFADevice`.
+/// Fluent builder constructing a request to `DeleteVirtualMfaDevice`.
 ///
 /// <p>Deletes a virtual MFA device.</p><note>
 /// <p>You must deactivate a user's virtual MFA device before you can delete it. For information about deactivating MFA devices, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_DeactivateMFADevice.html">DeactivateMFADevice</a>.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct DeleteVirtualMFADeviceFluentBuilder {
+pub struct DeleteVirtualMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::delete_virtual_mfa_device::builders::DeleteVirtualMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -34,8 +34,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput,
-        super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADeviceError,
-    > for DeleteVirtualMFADeviceFluentBuilder
+        super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceError,
+    > for DeleteVirtualMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -43,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput,
-            super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADeviceError,
+            super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl DeleteVirtualMFADeviceFluentBuilder {
-    /// Creates a new `DeleteVirtualMFADeviceFluentBuilder`.
+impl DeleteVirtualMfaDeviceFluentBuilder {
+    /// Creates a new `DeleteVirtualMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -58,7 +58,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the DeleteVirtualMFADevice as a reference.
+    /// Access the DeleteVirtualMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::delete_virtual_mfa_device::builders::DeleteVirtualMfaDeviceInputBuilder {
         &self.inner
     }
@@ -75,7 +75,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADeviceError,
+            super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +96,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput,
-        super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADeviceError,
+        super::super::super::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/delete_virtual_mfa_device.rs`

```diff
--- reference/src/operation/delete_virtual_mfa_device.rs
+++ generated/src/operation/delete_virtual_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `DeleteVirtualMFADevice`.
+/// Orchestration and serialization glue logic for `DeleteVirtualMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct DeleteVirtualMFADevice;
-impl DeleteVirtualMFADevice {
-    /// Creates a new `DeleteVirtualMFADevice`
+pub struct DeleteVirtualMfaDevice;
+impl DeleteVirtualMfaDevice {
+    /// Creates a new `DeleteVirtualMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteVirtualMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteVirtualMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteVirtualMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            DeleteVirtualMFADeviceRequestSerializer,
+            DeleteVirtualMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DeleteVirtualMFADeviceResponseDeserializer,
+            DeleteVirtualMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteVirtualMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteVirtualMFADeviceTelemetryInputCaptureInterceptor,
+                DeleteVirtualMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteVirtualMFADeviceEndpointParamsInterceptor,
+                DeleteVirtualMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::delete_virtual_mfa_device::DeleteVirtualMFADeviceError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct DeleteVirtualMFADeviceTelemetryInputCaptureInterceptor;
+struct DeleteVirtualMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteVirtualMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteVirtualMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteVirtualMFADeviceTelemetryInputCaptureInterceptor"
+        "DeleteVirtualMfaDeviceTelemetryInputCaptureInterceptor"
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
-            super::super::protocol_serde::shape_delete_virtual_mfa_device_input::ser_delete_virtual_mfa_device_input_input_input(&input)?,
+            super::super::protocol_serde::shape_delete_virtual_mfa_device_input::ser_delete_virtual_mfa_device_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +264,12 @@
     }
 }
 #[derive(Debug)]
-struct DeleteVirtualMFADeviceEndpointParamsInterceptor;
+struct DeleteVirtualMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteVirtualMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteVirtualMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "DeleteVirtualMFADeviceEndpointParamsInterceptor"
+        "DeleteVirtualMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/detach_group_policy/_detach_group_policy_input.rs`

```diff
--- reference/src/operation/detach_group_policy/_detach_group_policy_input.rs
+++ generated/src/operation/detach_group_policy/_detach_group_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`DetachGroupPolicyInput`](crate::operation::detach_group_policy::DetachGroupPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::detach_group_policy::DetachGroupPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::detach_group_policy::DetachGroupPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::detach_group_policy::DetachGroupPolicyInput {
             group_name: self.group_name,
             policy_arn: self.policy_arn,
```

### `src/operation/detach_group_policy.rs`

```diff
--- reference/src/operation/detach_group_policy.rs
+++ generated/src/operation/detach_group_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_detach_group_policy_input::ser_detach_group_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_detach_group_policy_input::ser_detach_group_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/detach_role_policy/_detach_role_policy_input.rs`

```diff
--- reference/src/operation/detach_role_policy/_detach_role_policy_input.rs
+++ generated/src/operation/detach_role_policy/_detach_role_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`DetachRolePolicyInput`](crate::operation::detach_role_policy::DetachRolePolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::detach_role_policy::DetachRolePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::detach_role_policy::DetachRolePolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::detach_role_policy::DetachRolePolicyInput {
             role_name: self.role_name,
             policy_arn: self.policy_arn,
```

### `src/operation/detach_role_policy.rs`

```diff
--- reference/src/operation/detach_role_policy.rs
+++ generated/src/operation/detach_role_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_detach_role_policy_input::ser_detach_role_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_detach_role_policy_input::ser_detach_role_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/detach_user_policy/_detach_user_policy_input.rs`

```diff
--- reference/src/operation/detach_user_policy/_detach_user_policy_input.rs
+++ generated/src/operation/detach_user_policy/_detach_user_policy_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`DetachUserPolicyInput`](crate::operation::detach_user_policy::DetachUserPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::detach_user_policy::DetachUserPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::detach_user_policy::DetachUserPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::detach_user_policy::DetachUserPolicyInput {
             user_name: self.user_name,
             policy_arn: self.policy_arn,
```

### `src/operation/detach_user_policy.rs`

```diff
--- reference/src/operation/detach_user_policy.rs
+++ generated/src/operation/detach_user_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_detach_user_policy_input::ser_detach_user_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_detach_user_policy_input::ser_detach_user_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/disable_organizations_root_credentials_management/_disable_organizations_root_credentials_management_output.rs`

```diff
--- reference/src/operation/disable_organizations_root_credentials_management/_disable_organizations_root_credentials_management_output.rs
+++ generated/src/operation/disable_organizations_root_credentials_management/_disable_organizations_root_credentials_management_output.rs
@@ -28,9 +28,7 @@
 }
 impl DisableOrganizationsRootCredentialsManagementOutput {
     /// Creates a new builder-style object to manufacture [`DisableOrganizationsRootCredentialsManagementOutput`](crate::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput).
-    pub fn builder(
-    ) -> super::super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementOutputBuilder
-    {
+    pub fn builder() -> super::super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementOutputBuilder{
         super::super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementOutputBuilder::default()
     }
 }
@@ -88,7 +86,9 @@
         self
     }
     /// Consumes the builder and constructs a [`DisableOrganizationsRootCredentialsManagementOutput`](crate::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput).
-    pub fn build(self) -> super::super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput {
+    pub fn build(
+        self,
+    ) -> super::super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput {
         super::super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput {
             organization_id: self.organization_id,
             enabled_features: self.enabled_features,
```

### `src/operation/disable_organizations_root_credentials_management/builders.rs`

```diff
--- reference/src/operation/disable_organizations_root_credentials_management/builders.rs
+++ generated/src/operation/disable_organizations_root_credentials_management/builders.rs
@@ -57,10 +57,7 @@
         }
     }
     /// Access the DisableOrganizationsRootCredentialsManagement as a reference.
-    pub fn as_input(
-        &self,
-    ) -> &super::super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementInputBuilder
-    {
+    pub fn as_input(&self) -> &super::super::super::operation::disable_organizations_root_credentials_management::builders::DisableOrganizationsRootCredentialsManagementInputBuilder{
         &self.inner
     }
     /// Sends the request and returns the response.
```

### `src/operation/disable_organizations_root_credentials_management.rs`

```diff
--- reference/src/operation/disable_organizations_root_credentials_management.rs
+++ generated/src/operation/disable_organizations_root_credentials_management.rs
@@ -30,12 +30,7 @@
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(
-            output
-                .downcast::<super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput>(
-                )
-                .expect("correct output type"),
-        )
+        ::std::result::Result::Ok(output.downcast::<super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementOutput>().expect("correct output type"))
     }

     pub(crate) async fn orchestrate_with_stop_point(
@@ -184,9 +179,7 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input
-            .downcast::<super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementInput>()
-            .expect("correct type");
+        let input = input.downcast::<super::super::operation::disable_organizations_root_credentials_management::DisableOrganizationsRootCredentialsManagementInput>().expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -211,11 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_disable_organizations_root_credentials_management_input::ser_disable_organizations_root_credentials_management_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/disable_organizations_root_sessions/builders.rs`

```diff
--- reference/src/operation/disable_organizations_root_sessions/builders.rs
+++ generated/src/operation/disable_organizations_root_sessions/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the DisableOrganizationsRootSessions as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::disable_organizations_root_sessions::builders::DisableOrganizationsRootSessionsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::disable_organizations_root_sessions::builders::DisableOrganizationsRootSessionsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::disable_organizations_root_sessions::DisableOrganizationsRootSessions::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::disable_organizations_root_sessions::DisableOrganizationsRootSessions::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::disable_organizations_root_sessions::DisableOrganizationsRootSessions::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::disable_organizations_root_sessions::DisableOrganizationsRootSessions::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/disable_organizations_root_sessions.rs`

```diff
--- reference/src/operation/disable_organizations_root_sessions.rs
+++ generated/src/operation/disable_organizations_root_sessions.rs
@@ -167,7 +167,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_disable_organizations_root_sessions::de_disable_organizations_root_sessions_http_error(status, headers, body)
+            super::super::protocol_serde::shape_disable_organizations_root_sessions::de_disable_organizations_root_sessions_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_disable_organizations_root_sessions::de_disable_organizations_root_sessions_http_response(
                 status, headers, body,
@@ -212,15 +214,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_disable_organizations_root_sessions_input::ser_disable_organizations_root_sessions_input_input_input(
-                &input,
-            )?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/disable_outbound_web_identity_federation/_disable_outbound_web_identity_federation_input.rs`

```diff
--- reference/src/operation/disable_outbound_web_identity_federation/_disable_outbound_web_identity_federation_input.rs
+++ generated/src/operation/disable_outbound_web_identity_federation/_disable_outbound_web_identity_federation_input.rs
@@ -5,8 +5,10 @@
 pub struct DisableOutboundWebIdentityFederationInput {}
 impl DisableOutboundWebIdentityFederationInput {
     /// Creates a new builder-style object to manufacture [`DisableOutboundWebIdentityFederationInput`](crate::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederationInput).
-    pub fn builder() -> super::super::super::operation::disable_outbound_web_identity_federation::builders::DisableOutboundWebIdentityFederationInputBuilder {
-        super::super::super::operation::disable_outbound_web_identity_federation::builders::DisableOutboundWebIdentityFederationInputBuilder::default()
+    pub fn builder(
+    ) -> super::super::super::operation::disable_outbound_web_identity_federation::builders::DisableOutboundWebIdentityFederationInputBuilder {
+        super::super::super::operation::disable_outbound_web_identity_federation::builders::DisableOutboundWebIdentityFederationInputBuilder::default(
+        )
     }
 }

@@ -22,6 +24,8 @@
         super::super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederationInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederationInput {})
+        ::std::result::Result::Ok(
+            super::super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederationInput {},
+        )
     }
 }
```

### `src/operation/disable_outbound_web_identity_federation/builders.rs`

```diff
--- reference/src/operation/disable_outbound_web_identity_federation/builders.rs
+++ generated/src/operation/disable_outbound_web_identity_federation/builders.rs
@@ -89,7 +89,11 @@
                 &self.handle.conf,
                 self.config_override,
             );
-        super::super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederation::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederation::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/disable_outbound_web_identity_federation.rs`

```diff
--- reference/src/operation/disable_outbound_web_identity_federation.rs
+++ generated/src/operation/disable_outbound_web_identity_federation.rs
@@ -214,11 +214,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_disable_outbound_web_identity_federation_input::ser_disable_outbound_web_identity_federation_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -362,7 +360,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederationError {
+impl ::aws_types::request_id::RequestId
+    for super::super::operation::disable_outbound_web_identity_federation::DisableOutboundWebIdentityFederationError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/enable_mfa_device/_enable_mfa_device_input.rs`

```diff
--- reference/src/operation/enable_mfa_device/_enable_mfa_device_input.rs
+++ generated/src/operation/enable_mfa_device/_enable_mfa_device_input.rs
@@ -150,7 +150,10 @@
     /// Consumes the builder and constructs a [`EnableMfaDeviceInput`](crate::operation::enable_mfa_device::EnableMfaDeviceInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::enable_mfa_device::EnableMfaDeviceInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::enable_mfa_device::EnableMfaDeviceInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::enable_mfa_device::EnableMfaDeviceInput {
             user_name: self.user_name,
             serial_number: self.serial_number,
```

### `src/operation/enable_mfa_device/builders.rs`

```diff
--- reference/src/operation/enable_mfa_device/builders.rs
+++ generated/src/operation/enable_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::enable_mfa_device::EnableMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::enable_mfa_device::EnableMFADeviceError,
+            super::super::super::operation::enable_mfa_device::EnableMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `EnableMFADevice`.
+/// Fluent builder constructing a request to `EnableMfaDevice`.
 ///
 /// <p>Enables the specified MFA device and associates it with the specified IAM user. When enabled, the MFA device is required for every subsequent login by the IAM user associated with the device.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct EnableMFADeviceFluentBuilder {
+pub struct EnableMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::enable_mfa_device::builders::EnableMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::enable_mfa_device::EnableMfaDeviceOutput,
-        super::super::super::operation::enable_mfa_device::EnableMFADeviceError,
-    > for EnableMFADeviceFluentBuilder
+        super::super::super::operation::enable_mfa_device::EnableMfaDeviceError,
+    > for EnableMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::enable_mfa_device::EnableMfaDeviceOutput,
-            super::super::super::operation::enable_mfa_device::EnableMFADeviceError,
+            super::super::super::operation::enable_mfa_device::EnableMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl EnableMFADeviceFluentBuilder {
-    /// Creates a new `EnableMFADeviceFluentBuilder`.
+impl EnableMfaDeviceFluentBuilder {
+    /// Creates a new `EnableMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the EnableMFADevice as a reference.
+    /// Access the EnableMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::enable_mfa_device::builders::EnableMfaDeviceInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::enable_mfa_device::EnableMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::enable_mfa_device::EnableMFADeviceError,
+            super::super::super::operation::enable_mfa_device::EnableMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::enable_mfa_device::EnableMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::enable_mfa_device::EnableMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::enable_mfa_device::EnableMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::enable_mfa_device::EnableMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::enable_mfa_device::EnableMfaDeviceOutput,
-        super::super::super::operation::enable_mfa_device::EnableMFADeviceError,
+        super::super::super::operation::enable_mfa_device::EnableMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/enable_mfa_device.rs`

```diff
--- reference/src/operation/enable_mfa_device.rs
+++ generated/src/operation/enable_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `EnableMFADevice`.
+/// Orchestration and serialization glue logic for `EnableMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct EnableMFADevice;
-impl EnableMFADevice {
-    /// Creates a new `EnableMFADevice`
+pub struct EnableMfaDevice;
+impl EnableMfaDevice {
+    /// Creates a new `EnableMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for EnableMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for EnableMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("EnableMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            EnableMFADeviceRequestSerializer,
+            EnableMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            EnableMFADeviceResponseDeserializer,
+            EnableMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("EnableMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                EnableMFADeviceTelemetryInputCaptureInterceptor,
+                EnableMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                EnableMFADeviceEndpointParamsInterceptor,
+                EnableMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::enable_mfa_device::EnableMFADeviceError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct EnableMFADeviceTelemetryInputCaptureInterceptor;
+struct EnableMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for EnableMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for EnableMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "EnableMFADeviceTelemetryInputCaptureInterceptor"
+        "EnableMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -262,12 +262,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_enable_mfa_device_input::ser_enable_mfa_device_input_input_input(&input)?,
+            super::super::protocol_serde::shape_enable_mfa_device_input::ser_enable_mfa_device_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -277,12 +276,12 @@
     }
 }
 #[derive(Debug)]
-struct EnableMFADeviceEndpointParamsInterceptor;
+struct EnableMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for EnableMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for EnableMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "EnableMFADeviceEndpointParamsInterceptor"
+        "EnableMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/enable_organizations_root_credentials_management/_enable_organizations_root_credentials_management_output.rs`

```diff
--- reference/src/operation/enable_organizations_root_credentials_management/_enable_organizations_root_credentials_management_output.rs
+++ generated/src/operation/enable_organizations_root_credentials_management/_enable_organizations_root_credentials_management_output.rs
@@ -28,8 +28,7 @@
 }
 impl EnableOrganizationsRootCredentialsManagementOutput {
     /// Creates a new builder-style object to manufacture [`EnableOrganizationsRootCredentialsManagementOutput`](crate::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput).
-    pub fn builder(
-    ) -> super::super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementOutputBuilder {
+    pub fn builder() -> super::super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementOutputBuilder{
         super::super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementOutputBuilder::default()
     }
 }
@@ -87,7 +86,9 @@
         self
     }
     /// Consumes the builder and constructs a [`EnableOrganizationsRootCredentialsManagementOutput`](crate::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput).
-    pub fn build(self) -> super::super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput {
+    pub fn build(
+        self,
+    ) -> super::super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput {
         super::super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput {
             organization_id: self.organization_id,
             enabled_features: self.enabled_features,
```

### `src/operation/enable_organizations_root_credentials_management/builders.rs`

```diff
--- reference/src/operation/enable_organizations_root_credentials_management/builders.rs
+++ generated/src/operation/enable_organizations_root_credentials_management/builders.rs
@@ -64,9 +64,7 @@
         }
     }
     /// Access the EnableOrganizationsRootCredentialsManagement as a reference.
-    pub fn as_input(
-        &self,
-    ) -> &super::super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementInputBuilder {
+    pub fn as_input(&self) -> &super::super::super::operation::enable_organizations_root_credentials_management::builders::EnableOrganizationsRootCredentialsManagementInputBuilder{
         &self.inner
     }
     /// Sends the request and returns the response.
```

### `src/operation/enable_organizations_root_credentials_management.rs`

```diff
--- reference/src/operation/enable_organizations_root_credentials_management.rs
+++ generated/src/operation/enable_organizations_root_credentials_management.rs
@@ -23,20 +23,14 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >| {
             err.map_service_error(|err| {
-                err.downcast::<super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementError>(
-                )
-                .expect("correct error type")
-            })
+                                err.downcast::<super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementError>().expect("correct error type")
+                            })
         };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(
-            output
-                .downcast::<super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput>()
-                .expect("correct output type"),
-        )
+        ::std::result::Result::Ok(output.downcast::<super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementOutput>().expect("correct output type"))
     }

     pub(crate) async fn orchestrate_with_stop_point(
@@ -186,7 +180,8 @@
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
         let input = input
-            .downcast::<super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementInput>()
+            .downcast::<super::super::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementInput>(
+            )
             .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
@@ -212,11 +207,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_enable_organizations_root_credentials_management_input::ser_enable_organizations_root_credentials_management_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/enable_organizations_root_sessions/builders.rs`

```diff
--- reference/src/operation/enable_organizations_root_sessions/builders.rs
+++ generated/src/operation/enable_organizations_root_sessions/builders.rs
@@ -64,7 +64,9 @@
         }
     }
     /// Access the EnableOrganizationsRootSessions as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::enable_organizations_root_sessions::builders::EnableOrganizationsRootSessionsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::enable_organizations_root_sessions::builders::EnableOrganizationsRootSessionsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -88,12 +90,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::enable_organizations_root_sessions::EnableOrganizationsRootSessions::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::enable_organizations_root_sessions::EnableOrganizationsRootSessions::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::enable_organizations_root_sessions::EnableOrganizationsRootSessions::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::enable_organizations_root_sessions::EnableOrganizationsRootSessions::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/enable_organizations_root_sessions.rs`

```diff
--- reference/src/operation/enable_organizations_root_sessions.rs
+++ generated/src/operation/enable_organizations_root_sessions.rs
@@ -167,7 +167,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_enable_organizations_root_sessions::de_enable_organizations_root_sessions_http_error(status, headers, body)
+            super::super::protocol_serde::shape_enable_organizations_root_sessions::de_enable_organizations_root_sessions_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_enable_organizations_root_sessions::de_enable_organizations_root_sessions_http_response(
                 status, headers, body,
@@ -212,13 +214,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_enable_organizations_root_sessions_input::ser_enable_organizations_root_sessions_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/enable_outbound_web_identity_federation/_enable_outbound_web_identity_federation_input.rs`

```diff
--- reference/src/operation/enable_outbound_web_identity_federation/_enable_outbound_web_identity_federation_input.rs
+++ generated/src/operation/enable_outbound_web_identity_federation/_enable_outbound_web_identity_federation_input.rs
@@ -5,7 +5,8 @@
 pub struct EnableOutboundWebIdentityFederationInput {}
 impl EnableOutboundWebIdentityFederationInput {
     /// Creates a new builder-style object to manufacture [`EnableOutboundWebIdentityFederationInput`](crate::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederationInput).
-    pub fn builder() -> super::super::super::operation::enable_outbound_web_identity_federation::builders::EnableOutboundWebIdentityFederationInputBuilder {
+    pub fn builder(
+    ) -> super::super::super::operation::enable_outbound_web_identity_federation::builders::EnableOutboundWebIdentityFederationInputBuilder {
         super::super::super::operation::enable_outbound_web_identity_federation::builders::EnableOutboundWebIdentityFederationInputBuilder::default()
     }
 }
@@ -22,6 +23,8 @@
         super::super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederationInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederationInput {})
+        ::std::result::Result::Ok(
+            super::super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederationInput {},
+        )
     }
 }
```

### `src/operation/enable_outbound_web_identity_federation/builders.rs`

```diff
--- reference/src/operation/enable_outbound_web_identity_federation/builders.rs
+++ generated/src/operation/enable_outbound_web_identity_federation/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the EnableOutboundWebIdentityFederation as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::enable_outbound_web_identity_federation::builders::EnableOutboundWebIdentityFederationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::enable_outbound_web_identity_federation::builders::EnableOutboundWebIdentityFederationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -87,7 +89,11 @@
                 &self.handle.conf,
                 self.config_override,
             );
-        super::super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederation::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederation::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/enable_outbound_web_identity_federation.rs`

```diff
--- reference/src/operation/enable_outbound_web_identity_federation.rs
+++ generated/src/operation/enable_outbound_web_identity_federation.rs
@@ -214,11 +214,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_enable_outbound_web_identity_federation_input::ser_enable_outbound_web_identity_federation_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -362,7 +360,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederationError {
+impl ::aws_types::request_id::RequestId
+    for super::super::operation::enable_outbound_web_identity_federation::EnableOutboundWebIdentityFederationError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/generate_credential_report.rs`

```diff
--- reference/src/operation/generate_credential_report.rs
+++ generated/src/operation/generate_credential_report.rs
@@ -204,13 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_generate_credential_report_input::ser_generate_credential_report_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/generate_organizations_access_report/builders.rs`

```diff
--- reference/src/operation/generate_organizations_access_report/builders.rs
+++ generated/src/operation/generate_organizations_access_report/builders.rs
@@ -89,7 +89,9 @@
         }
     }
     /// Access the GenerateOrganizationsAccessReport as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -113,12 +115,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::generate_organizations_access_report::GenerateOrganizationsAccessReport::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::generate_organizations_access_report::GenerateOrganizationsAccessReport::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::generate_organizations_access_report::GenerateOrganizationsAccessReport::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::generate_organizations_access_report::GenerateOrganizationsAccessReport::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/generate_organizations_access_report.rs`

```diff
--- reference/src/operation/generate_organizations_access_report.rs
+++ generated/src/operation/generate_organizations_access_report.rs
@@ -265,12 +265,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_generate_organizations_access_report_input::ser_generate_organizations_access_report_input_input_input(
+            super::super::protocol_serde::shape_generate_organizations_access_report_input::ser_generate_organizations_access_report_op_input(
                 &input,
             )?,
         );
```

### `src/operation/generate_service_last_accessed_details/builders.rs`

```diff
--- reference/src/operation/generate_service_last_accessed_details/builders.rs
+++ generated/src/operation/generate_service_last_accessed_details/builders.rs
@@ -72,7 +72,9 @@
         }
     }
     /// Access the GenerateServiceLastAccessedDetails as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::generate_service_last_accessed_details::builders::GenerateServiceLastAccessedDetailsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::generate_service_last_accessed_details::builders::GenerateServiceLastAccessedDetailsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -96,12 +98,17 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetails::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetails::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetails::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetails::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/generate_service_last_accessed_details.rs`

```diff
--- reference/src/operation/generate_service_last_accessed_details.rs
+++ generated/src/operation/generate_service_last_accessed_details.rs
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
-            super::super::protocol_serde::shape_generate_service_last_accessed_details_input::ser_generate_service_last_accessed_details_input_input_input(
+            super::super::protocol_serde::shape_generate_service_last_accessed_details_input::ser_generate_service_last_accessed_details_op_input(
                 &input,
             )?,
         );
```

### `src/operation/get_access_key_last_used/_get_access_key_last_used_input.rs`

```diff
--- reference/src/operation/get_access_key_last_used/_get_access_key_last_used_input.rs
+++ generated/src/operation/get_access_key_last_used/_get_access_key_last_used_input.rs
@@ -49,8 +49,10 @@
     /// Consumes the builder and constructs a [`GetAccessKeyLastUsedInput`](crate::operation::get_access_key_last_used::GetAccessKeyLastUsedInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_access_key_last_used::GetAccessKeyLastUsedInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_access_key_last_used::GetAccessKeyLastUsedInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_access_key_last_used::GetAccessKeyLastUsedInput {
             access_key_id: self.access_key_id,
         })
```

### `src/operation/get_access_key_last_used.rs`

```diff
--- reference/src/operation/get_access_key_last_used.rs
+++ generated/src/operation/get_access_key_last_used.rs
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
-            super::super::protocol_serde::shape_get_access_key_last_used_input::ser_get_access_key_last_used_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_access_key_last_used_input::ser_get_access_key_last_used_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_account_authorization_details/_get_account_authorization_details_input.rs`

```diff
--- reference/src/operation/get_account_authorization_details/_get_account_authorization_details_input.rs
+++ generated/src/operation/get_account_authorization_details/_get_account_authorization_details_input.rs
@@ -107,10 +107,12 @@
         super::super::super::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput {
-            filter: self.filter,
-            max_items: self.max_items,
-            marker: self.marker,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput {
+                filter: self.filter,
+                max_items: self.max_items,
+                marker: self.marker,
+            },
+        )
     }
 }
```

### `src/operation/get_account_authorization_details/builders.rs`

```diff
--- reference/src/operation/get_account_authorization_details/builders.rs
+++ generated/src/operation/get_account_authorization_details/builders.rs
@@ -60,7 +60,9 @@
         }
     }
     /// Access the GetAccountAuthorizationDetails as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::get_account_authorization_details::builders::GetAccountAuthorizationDetailsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::get_account_authorization_details::builders::GetAccountAuthorizationDetailsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -84,11 +86,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_account_authorization_details::GetAccountAuthorizationDetails::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::get_account_authorization_details::GetAccountAuthorizationDetails::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::get_account_authorization_details::GetAccountAuthorizationDetails::orchestrate(&runtime_plugins, input).await
     }

@@ -114,8 +117,13 @@
     /// Create a paginator for this request
     ///
     /// Paginators are used by calling [`send().await`](crate::operation::get_account_authorization_details::paginator::GetAccountAuthorizationDetailsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::get_account_authorization_details::paginator::GetAccountAuthorizationDetailsPaginator {
-        super::super::super::operation::get_account_authorization_details::paginator::GetAccountAuthorizationDetailsPaginator::new(self.handle, self.inner)
+    pub fn into_paginator(
+        self,
+    ) -> super::super::super::operation::get_account_authorization_details::paginator::GetAccountAuthorizationDetailsPaginator {
+        super::super::super::operation::get_account_authorization_details::paginator::GetAccountAuthorizationDetailsPaginator::new(
+            self.handle,
+            self.inner,
+        )
     }
     ///
     /// Appends an item to `Filter`.
```

### `src/operation/get_account_authorization_details.rs`

```diff
--- reference/src/operation/get_account_authorization_details.rs
+++ generated/src/operation/get_account_authorization_details.rs
@@ -213,9 +213,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_get_account_authorization_details::de_get_account_authorization_details_http_error(status, headers, body)
+            super::super::protocol_serde::shape_get_account_authorization_details::de_get_account_authorization_details_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_get_account_authorization_details::de_get_account_authorization_details_http_response(status, headers, body)
+            super::super::protocol_serde::shape_get_account_authorization_details::de_get_account_authorization_details_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -256,12 +260,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_account_authorization_details_input::ser_get_account_authorization_details_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_account_authorization_details_input::ser_get_account_authorization_details_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_account_password_policy.rs`

```diff
--- reference/src/operation/get_account_password_policy.rs
+++ generated/src/operation/get_account_password_policy.rs
@@ -204,13 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_account_password_policy_input::ser_get_account_password_policy_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/get_account_properties/_get_account_properties_input.rs`

```diff
--- reference/src/operation/get_account_properties/_get_account_properties_input.rs
+++ generated/src/operation/get_account_properties/_get_account_properties_input.rs
@@ -18,8 +18,10 @@
     /// Consumes the builder and constructs a [`GetAccountPropertiesInput`](crate::operation::get_account_properties::GetAccountPropertiesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_account_properties::GetAccountPropertiesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_account_properties::GetAccountPropertiesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_account_properties::GetAccountPropertiesInput {})
     }
 }
```

### `src/operation/get_account_properties.rs`

```diff
--- reference/src/operation/get_account_properties.rs
+++ generated/src/operation/get_account_properties.rs
@@ -204,13 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_account_properties_input::ser_get_account_properties_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/get_account_summary/_get_account_summary_input.rs`

```diff
--- reference/src/operation/get_account_summary/_get_account_summary_input.rs
+++ generated/src/operation/get_account_summary/_get_account_summary_input.rs
@@ -18,7 +18,10 @@
     /// Consumes the builder and constructs a [`GetAccountSummaryInput`](crate::operation::get_account_summary::GetAccountSummaryInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_account_summary::GetAccountSummaryInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_account_summary::GetAccountSummaryInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_account_summary::GetAccountSummaryInput {})
     }
 }
```

### `src/operation/get_account_summary/_get_account_summary_output.rs`

```diff
--- reference/src/operation/get_account_summary/_get_account_summary_output.rs
+++ generated/src/operation/get_account_summary/_get_account_summary_output.rs
@@ -46,7 +46,10 @@
         self
     }
     /// <p>A set of key–value pairs containing information about IAM entity usage and IAM quotas.</p>
-    pub fn set_summary_map(mut self, input: ::std::option::Option<::std::collections::HashMap<super::super::super::types::SummaryKeyType, i32>>) -> Self {
+    pub fn set_summary_map(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<super::super::super::types::SummaryKeyType, i32>>,
+    ) -> Self {
         self.summary_map = input;
         self
     }
```

### `src/operation/get_account_summary.rs`

```diff
--- reference/src/operation/get_account_summary.rs
+++ generated/src/operation/get_account_summary.rs
@@ -201,13 +201,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_account_summary_input::ser_get_account_summary_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/get_context_keys_for_custom_policy/_get_context_keys_for_custom_policy_input.rs`

```diff
--- reference/src/operation/get_context_keys_for_custom_policy/_get_context_keys_for_custom_policy_input.rs
+++ generated/src/operation/get_context_keys_for_custom_policy/_get_context_keys_for_custom_policy_input.rs
@@ -100,8 +100,10 @@
         super::super::super::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyInput {
-            policy_input_list: self.policy_input_list,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyInput {
+                policy_input_list: self.policy_input_list,
+            },
+        )
     }
 }
```

### `src/operation/get_context_keys_for_custom_policy/builders.rs`

```diff
--- reference/src/operation/get_context_keys_for_custom_policy/builders.rs
+++ generated/src/operation/get_context_keys_for_custom_policy/builders.rs
@@ -58,7 +58,9 @@
         }
     }
     /// Access the GetContextKeysForCustomPolicy as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::get_context_keys_for_custom_policy::builders::GetContextKeysForCustomPolicyInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::get_context_keys_for_custom_policy::builders::GetContextKeysForCustomPolicyInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -82,11 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicy::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicy::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicy::orchestrate(&runtime_plugins, input).await
     }

```

### `src/operation/get_context_keys_for_custom_policy.rs`

```diff
--- reference/src/operation/get_context_keys_for_custom_policy.rs
+++ generated/src/operation/get_context_keys_for_custom_policy.rs
@@ -161,7 +161,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_get_context_keys_for_custom_policy::de_get_context_keys_for_custom_policy_http_error(status, headers, body)
+            super::super::protocol_serde::shape_get_context_keys_for_custom_policy::de_get_context_keys_for_custom_policy_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_get_context_keys_for_custom_policy::de_get_context_keys_for_custom_policy_http_response(
                 status, headers, body,
@@ -206,12 +208,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_context_keys_for_custom_policy_input::ser_get_context_keys_for_custom_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_context_keys_for_custom_policy_input::ser_get_context_keys_for_custom_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_context_keys_for_principal_policy/builders.rs`

```diff
--- reference/src/operation/get_context_keys_for_principal_policy/builders.rs
+++ generated/src/operation/get_context_keys_for_principal_policy/builders.rs
@@ -60,7 +60,9 @@
         }
     }
     /// Access the GetContextKeysForPrincipalPolicy as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -84,12 +86,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicy::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicy::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicy::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicy::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/get_context_keys_for_principal_policy.rs`

```diff
--- reference/src/operation/get_context_keys_for_principal_policy.rs
+++ generated/src/operation/get_context_keys_for_principal_policy.rs
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
-            super::super::protocol_serde::shape_get_context_keys_for_principal_policy_input::ser_get_context_keys_for_principal_policy_input_input_input(
+            super::super::protocol_serde::shape_get_context_keys_for_principal_policy_input::ser_get_context_keys_for_principal_policy_op_input(
                 &input,
             )?,
         );
```

### `src/operation/get_credential_report/_get_credential_report_input.rs`

```diff
--- reference/src/operation/get_credential_report/_get_credential_report_input.rs
+++ generated/src/operation/get_credential_report/_get_credential_report_input.rs
@@ -18,8 +18,10 @@
     /// Consumes the builder and constructs a [`GetCredentialReportInput`](crate::operation::get_credential_report::GetCredentialReportInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_credential_report::GetCredentialReportInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_credential_report::GetCredentialReportInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_credential_report::GetCredentialReportInput {})
     }
 }
```

### `src/operation/get_credential_report.rs`

```diff
--- reference/src/operation/get_credential_report.rs
+++ generated/src/operation/get_credential_report.rs
@@ -204,13 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_credential_report_input::ser_get_credential_report_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/get_delegation_request/_get_delegation_request_input.rs`

```diff
--- reference/src/operation/get_delegation_request/_get_delegation_request_input.rs
+++ generated/src/operation/get_delegation_request/_get_delegation_request_input.rs
@@ -75,11 +75,13 @@
     /// Consumes the builder and constructs a [`GetDelegationRequestInput`](crate::operation::get_delegation_request::GetDelegationRequestInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_delegation_request::GetDelegationRequestInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_delegation_request::GetDelegationRequestInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_delegation_request::GetDelegationRequestInput {
             delegation_request_id: self.delegation_request_id,
-            delegation_permission_check: self.delegation_permission_check,
+            delegation_permission_check: self.delegation_permission_check.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/get_delegation_request.rs`

```diff
--- reference/src/operation/get_delegation_request.rs
+++ generated/src/operation/get_delegation_request.rs
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
-            super::super::protocol_serde::shape_get_delegation_request_input::ser_get_delegation_request_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_delegation_request_input::ser_get_delegation_request_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_group/_get_group_input.rs`

```diff
--- reference/src/operation/get_group/_get_group_input.rs
+++ generated/src/operation/get_group/_get_group_input.rs
@@ -94,7 +94,9 @@
         &self.max_items
     }
     /// Consumes the builder and constructs a [`GetGroupInput`](crate::operation::get_group::GetGroupInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_group::GetGroupInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_group::GetGroupInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::get_group::GetGroupInput {
             group_name: self.group_name,
             marker: self.marker,
```

### `src/operation/get_group/_get_group_output.rs`

```diff
--- reference/src/operation/get_group/_get_group_output.rs
+++ generated/src/operation/get_group/_get_group_output.rs
@@ -131,7 +131,9 @@
     /// Consumes the builder and constructs a [`GetGroupOutput`](crate::operation::get_group::GetGroupOutput).
     /// This method will fail if any of the following fields are not set:
     /// - [`users`](crate::operation::get_group::builders::GetGroupOutputBuilder::users)
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_group::GetGroupOutput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_group::GetGroupOutput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::get_group::GetGroupOutput {
             group: self.group,
             users: self.users.ok_or_else(|| {
```

### `src/operation/get_group/builders.rs`

```diff
--- reference/src/operation/get_group/builders.rs
+++ generated/src/operation/get_group/builders.rs
@@ -29,14 +29,20 @@
     inner: super::super::super::operation::get_group::builders::GetGroupInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::get_group::GetGroupOutput, super::super::super::operation::get_group::GetGroupError>
-    for GetGroupFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::get_group::GetGroupOutput,
+        super::super::super::operation::get_group::GetGroupError,
+    > for GetGroupFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::get_group::GetGroupOutput, super::super::super::operation::get_group::GetGroupError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::get_group::GetGroupOutput,
+            super::super::super::operation::get_group::GetGroupError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -86,8 +92,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::get_group::GetGroupOutput, super::super::super::operation::get_group::GetGroupError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::get_group::GetGroupOutput,
+        super::super::super::operation::get_group::GetGroupError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/get_group/paginator.rs`

```diff
--- reference/src/operation/get_group/paginator.rs
+++ generated/src/operation/get_group/paginator.rs
@@ -8,7 +8,10 @@

 impl GetGroupPaginator {
     /// Create a new paginator-wrapper
-    pub(crate) fn new(handle: std::sync::Arc<super::super::super::client::Handle>, builder: super::super::super::operation::get_group::builders::GetGroupInputBuilder) -> Self {
+    pub(crate) fn new(
+        handle: std::sync::Arc<super::super::super::client::Handle>,
+        builder: super::super::super::operation::get_group::builders::GetGroupInputBuilder,
+    ) -> Self {
         Self {
             handle,
             builder,
@@ -136,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_get_group_output_output_users(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_get_group_output_output_users(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/get_group.rs`

```diff
--- reference/src/operation/get_group.rs
+++ generated/src/operation/get_group.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::get_group::GetGroupError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::get_group::GetGroupError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -221,7 +225,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::get_group::GetGroupInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::get_group::GetGroupInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -246,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_group_input::ser_get_group_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_group_input::ser_get_group_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_group_policy.rs`

```diff
--- reference/src/operation/get_group_policy.rs
+++ generated/src/operation/get_group_policy.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_group_policy_input::ser_get_group_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_group_policy_input::ser_get_group_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_human_readable_summary/builders.rs`

```diff
--- reference/src/operation/get_human_readable_summary/builders.rs
+++ generated/src/operation/get_human_readable_summary/builders.rs
@@ -24,7 +24,7 @@
 ///
 /// <p>Retrieves a human readable summary for a given entity. At this time, the only supported entity type is <code>delegation-request</code></p>
 /// <p>This method uses a Large Language Model (LLM) to generate the summary.</p>
-/// <p>If a delegation request has no owner or owner account, <code>GetHumanReadableSummary</code> for that delegation request can be called by any account. If the owner account is assigned but there is no owner id, only identities within that owner account can call <code>GetHumanReadableSummary</code> for the delegation request to retrieve a summary of that request. Once the delegation request is fully owned, the owner of the request gets a default permission to get that delegation request. For more details, read <code>default permissions granted to delegation requests</code>. These rules are identical to <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetDelegationRequest.html">GetDelegationRequest</a> API behavior, such that a party who has permissions to call <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetDelegationRequest.html">GetDelegationRequest</a> for a given delegation request will always be able to retrieve the human readable summary for that request.</p>
+/// <p>If a delegation request has no owner or owner account, <code>GetHumanReadableSummary</code> for that delegation request can be called by any account. If the owner account is assigned but there is no owner id, only identities within that owner account can call <code>GetHumanReadableSummary</code> for the delegation request to retrieve a summary of that request. Once the delegation request is fully owned, the owner of the request gets a default permission to get that delegation request. For more details, read <a href="">default permissions granted to delegation requests</a>. These rules are identical to <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetDelegationRequest.html">GetDelegationRequest</a> API behavior, such that a party who has permissions to call <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetDelegationRequest.html">GetDelegationRequest</a> for a given delegation request will always be able to retrieve the human readable summary for that request.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
 pub struct GetHumanReadableSummaryFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
```

### `src/operation/get_human_readable_summary.rs`

```diff
--- reference/src/operation/get_human_readable_summary.rs
+++ generated/src/operation/get_human_readable_summary.rs
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
-            super::super::protocol_serde::shape_get_human_readable_summary_input::ser_get_human_readable_summary_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_human_readable_summary_input::ser_get_human_readable_summary_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_instance_profile/_get_instance_profile_input.rs`

```diff
--- reference/src/operation/get_instance_profile/_get_instance_profile_input.rs
+++ generated/src/operation/get_instance_profile/_get_instance_profile_input.rs
@@ -49,8 +49,10 @@
     /// Consumes the builder and constructs a [`GetInstanceProfileInput`](crate::operation::get_instance_profile::GetInstanceProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_instance_profile::GetInstanceProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_instance_profile::GetInstanceProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_instance_profile::GetInstanceProfileInput {
             instance_profile_name: self.instance_profile_name,
         })
```

### `src/operation/get_instance_profile.rs`

```diff
--- reference/src/operation/get_instance_profile.rs
+++ generated/src/operation/get_instance_profile.rs
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
-            super::super::protocol_serde::shape_get_instance_profile_input::ser_get_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_instance_profile_input::ser_get_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_login_profile/_get_login_profile_input.rs`

```diff
--- reference/src/operation/get_login_profile/_get_login_profile_input.rs
+++ generated/src/operation/get_login_profile/_get_login_profile_input.rs
@@ -53,7 +53,10 @@
     /// Consumes the builder and constructs a [`GetLoginProfileInput`](crate::operation::get_login_profile::GetLoginProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_login_profile::GetLoginProfileInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_login_profile::GetLoginProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_login_profile::GetLoginProfileInput { user_name: self.user_name })
     }
 }
```

### `src/operation/get_login_profile.rs`

```diff
--- reference/src/operation/get_login_profile.rs
+++ generated/src/operation/get_login_profile.rs
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
-            super::super::protocol_serde::shape_get_login_profile_input::ser_get_login_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_login_profile_input::ser_get_login_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_mfa_device/builders.rs`

```diff
--- reference/src/operation/get_mfa_device/builders.rs
+++ generated/src/operation/get_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_mfa_device::GetMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_mfa_device::GetMFADeviceError,
+            super::super::super::operation::get_mfa_device::GetMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetMFADevice`.
+/// Fluent builder constructing a request to `GetMfaDevice`.
 ///
 /// <p>Retrieves information about an MFA device for a specified user.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetMFADeviceFluentBuilder {
+pub struct GetMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_mfa_device::builders::GetMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_mfa_device::GetMfaDeviceOutput,
-        super::super::super::operation::get_mfa_device::GetMFADeviceError,
-    > for GetMFADeviceFluentBuilder
+        super::super::super::operation::get_mfa_device::GetMfaDeviceError,
+    > for GetMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_mfa_device::GetMfaDeviceOutput,
-            super::super::super::operation::get_mfa_device::GetMFADeviceError,
+            super::super::super::operation::get_mfa_device::GetMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetMFADeviceFluentBuilder {
-    /// Creates a new `GetMFADeviceFluentBuilder`.
+impl GetMfaDeviceFluentBuilder {
+    /// Creates a new `GetMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetMFADevice as a reference.
+    /// Access the GetMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_mfa_device::builders::GetMfaDeviceInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_mfa_device::GetMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_mfa_device::GetMFADeviceError,
+            super::super::super::operation::get_mfa_device::GetMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_mfa_device::GetMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_mfa_device::GetMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_mfa_device::GetMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_mfa_device::GetMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_mfa_device::GetMfaDeviceOutput,
-        super::super::super::operation::get_mfa_device::GetMFADeviceError,
+        super::super::super::operation::get_mfa_device::GetMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_mfa_device.rs`

```diff
--- reference/src/operation/get_mfa_device.rs
+++ generated/src/operation/get_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetMFADevice`.
+/// Orchestration and serialization glue logic for `GetMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetMFADevice;
-impl GetMFADevice {
-    /// Creates a new `GetMFADevice`
+pub struct GetMfaDevice;
+impl GetMfaDevice {
+    /// Creates a new `GetMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetMFADeviceRequestSerializer,
+            GetMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetMFADeviceResponseDeserializer,
+            GetMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetMFADeviceTelemetryInputCaptureInterceptor,
+                GetMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetMFADeviceEndpointParamsInterceptor,
+                GetMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_mfa_device::GetMFADeviceError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct GetMFADeviceTelemetryInputCaptureInterceptor;
+struct GetMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "GetMFADeviceTelemetryInputCaptureInterceptor"
+        "GetMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_mfa_device_input::ser_get_mfa_device_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_mfa_device_input::ser_get_mfa_device_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct GetMFADeviceEndpointParamsInterceptor;
+struct GetMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetMFADeviceEndpointParamsInterceptor"
+        "GetMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_open_id_connect_provider/_get_open_id_connect_provider_input.rs`

```diff
--- reference/src/operation/get_open_id_connect_provider/_get_open_id_connect_provider_input.rs
+++ generated/src/operation/get_open_id_connect_provider/_get_open_id_connect_provider_input.rs
@@ -53,8 +53,10 @@
         super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderInput {
-            open_id_connect_provider_arn: self.open_id_connect_provider_arn,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderInput {
+                open_id_connect_provider_arn: self.open_id_connect_provider_arn,
+            },
+        )
     }
 }
```

### `src/operation/get_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/get_open_id_connect_provider/builders.rs
+++ generated/src/operation/get_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError,
+            super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `GetOpenIdConnectProvider`.
 ///
 /// <p>Returns information about the specified OpenID Connect (OIDC) provider resource object in IAM.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetOpenIDConnectProviderFluentBuilder {
+pub struct GetOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_open_id_connect_provider::builders::GetOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput,
-        super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError,
-    > for GetOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderError,
+    > for GetOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput,
-            super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError,
+            super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `GetOpenIDConnectProviderFluentBuilder`.
+impl GetOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `GetOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetOpenIDConnectProvider as a reference.
+    /// Access the GetOpenIdConnectProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_open_id_connect_provider::builders::GetOpenIdConnectProviderInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError,
+            super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput,
-        super::super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError,
+        super::super::super::operation::get_open_id_connect_provider::GetOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_open_id_connect_provider.rs`

```diff
--- reference/src/operation/get_open_id_connect_provider.rs
+++ generated/src/operation/get_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `GetOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetOpenIDConnectProvider;
-impl GetOpenIDConnectProvider {
-    /// Creates a new `GetOpenIDConnectProvider`
+pub struct GetOpenIdConnectProvider;
+impl GetOpenIdConnectProvider {
+    /// Creates a new `GetOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetOpenIDConnectProviderRequestSerializer,
+            GetOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetOpenIDConnectProviderResponseDeserializer,
+            GetOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                GetOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetOpenIDConnectProviderEndpointParamsInterceptor,
+                GetOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_open_id_connect_provider::GetOpenIDConnectProviderError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct GetOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct GetOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "GetOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "GetOpenIdConnectProviderTelemetryInputCaptureInterceptor"
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
-            super::super::protocol_serde::shape_get_open_id_connect_provider_input::ser_get_open_id_connect_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_open_id_connect_provider_input::ser_get_open_id_connect_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +264,12 @@
     }
 }
 #[derive(Debug)]
-struct GetOpenIDConnectProviderEndpointParamsInterceptor;
+struct GetOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetOpenIDConnectProviderEndpointParamsInterceptor"
+        "GetOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_organizations_access_report/_get_organizations_access_report_input.rs`

```diff
--- reference/src/operation/get_organizations_access_report/_get_organizations_access_report_input.rs
+++ generated/src/operation/get_organizations_access_report/_get_organizations_access_report_input.rs
@@ -116,11 +116,13 @@
         super::super::super::operation::get_organizations_access_report::GetOrganizationsAccessReportInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_organizations_access_report::GetOrganizationsAccessReportInput {
-            job_id: self.job_id,
-            max_items: self.max_items,
-            marker: self.marker,
-            sort_key: self.sort_key,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_organizations_access_report::GetOrganizationsAccessReportInput {
+                job_id: self.job_id,
+                max_items: self.max_items,
+                marker: self.marker,
+                sort_key: self.sort_key,
+            },
+        )
     }
 }
```

### `src/operation/get_organizations_access_report/_get_organizations_access_report_output.rs`

```diff
--- reference/src/operation/get_organizations_access_report/_get_organizations_access_report_output.rs
+++ generated/src/operation/get_organizations_access_report/_get_organizations_access_report_output.rs
@@ -254,27 +254,29 @@
         super::super::super::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput {
-            job_status: self.job_status.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "job_status",
-                    "job_status was not specified but it is required when building GetOrganizationsAccessReportOutput",
-                )
-            })?,
-            job_creation_date: self.job_creation_date.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "job_creation_date",
-                    "job_creation_date was not specified but it is required when building GetOrganizationsAccessReportOutput",
-                )
-            })?,
-            job_completion_date: self.job_completion_date,
-            number_of_services_accessible: self.number_of_services_accessible,
-            number_of_services_not_accessed: self.number_of_services_not_accessed,
-            access_details: self.access_details,
-            is_truncated: self.is_truncated.unwrap_or_default(),
-            marker: self.marker,
-            error_details: self.error_details,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput {
+                job_status: self.job_status.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "job_status",
+                        "job_status was not specified but it is required when building GetOrganizationsAccessReportOutput",
+                    )
+                })?,
+                job_creation_date: self.job_creation_date.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "job_creation_date",
+                        "job_creation_date was not specified but it is required when building GetOrganizationsAccessReportOutput",
+                    )
+                })?,
+                job_completion_date: self.job_completion_date,
+                number_of_services_accessible: self.number_of_services_accessible,
+                number_of_services_not_accessed: self.number_of_services_not_accessed,
+                access_details: self.access_details,
+                is_truncated: self.is_truncated.unwrap_or_default(),
+                marker: self.marker,
+                error_details: self.error_details,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/get_organizations_access_report.rs`

```diff
--- reference/src/operation/get_organizations_access_report.rs
+++ generated/src/operation/get_organizations_access_report.rs
@@ -214,7 +214,9 @@
         let parse_result = if !success && status != 200 || force_error {
             super::super::protocol_serde::shape_get_organizations_access_report::de_get_organizations_access_report_http_error(status, headers, body)
         } else {
-            super::super::protocol_serde::shape_get_organizations_access_report::de_get_organizations_access_report_http_response(status, headers, body)
+            super::super::protocol_serde::shape_get_organizations_access_report::de_get_organizations_access_report_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -255,12 +257,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_organizations_access_report_input::ser_get_organizations_access_report_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_organizations_access_report_input::ser_get_organizations_access_report_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_outbound_web_identity_federation_info/_get_outbound_web_identity_federation_info_input.rs`

```diff
--- reference/src/operation/get_outbound_web_identity_federation_info/_get_outbound_web_identity_federation_info_input.rs
+++ generated/src/operation/get_outbound_web_identity_federation_info/_get_outbound_web_identity_federation_info_input.rs
@@ -5,8 +5,10 @@
 pub struct GetOutboundWebIdentityFederationInfoInput {}
 impl GetOutboundWebIdentityFederationInfoInput {
     /// Creates a new builder-style object to manufacture [`GetOutboundWebIdentityFederationInfoInput`](crate::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfoInput).
-    pub fn builder() -> super::super::super::operation::get_outbound_web_identity_federation_info::builders::GetOutboundWebIdentityFederationInfoInputBuilder {
-        super::super::super::operation::get_outbound_web_identity_federation_info::builders::GetOutboundWebIdentityFederationInfoInputBuilder::default()
+    pub fn builder(
+    ) -> super::super::super::operation::get_outbound_web_identity_federation_info::builders::GetOutboundWebIdentityFederationInfoInputBuilder {
+        super::super::super::operation::get_outbound_web_identity_federation_info::builders::GetOutboundWebIdentityFederationInfoInputBuilder::default(
+        )
     }
 }

@@ -22,6 +24,8 @@
         super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfoInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfoInput {})
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfoInput {},
+        )
     }
 }
```

### `src/operation/get_outbound_web_identity_federation_info/builders.rs`

```diff
--- reference/src/operation/get_outbound_web_identity_federation_info/builders.rs
+++ generated/src/operation/get_outbound_web_identity_federation_info/builders.rs
@@ -83,13 +83,16 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfo::operation_runtime_plugins(
-                self.handle.runtime_plugins.clone(),
-                &self.handle.conf,
-                self.config_override,
-            );
-        super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfo::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins = super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfo::operation_runtime_plugins(
+                            self.handle.runtime_plugins.clone(),
+                            &self.handle.conf,
+                            self.config_override,
+                        );
+        super::super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfo::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/get_outbound_web_identity_federation_info.rs`

```diff
--- reference/src/operation/get_outbound_web_identity_federation_info.rs
+++ generated/src/operation/get_outbound_web_identity_federation_info.rs
@@ -214,11 +214,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_outbound_web_identity_federation_info_input::ser_get_outbound_web_identity_federation_info_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -362,7 +360,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfoError {
+impl ::aws_types::request_id::RequestId
+    for super::super::operation::get_outbound_web_identity_federation_info::GetOutboundWebIdentityFederationInfoError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/get_policy/_get_policy_input.rs`

```diff
--- reference/src/operation/get_policy/_get_policy_input.rs
+++ generated/src/operation/get_policy/_get_policy_input.rs
@@ -47,7 +47,9 @@
         &self.policy_arn
     }
     /// Consumes the builder and constructs a [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_policy::GetPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_policy::GetPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::get_policy::GetPolicyInput { policy_arn: self.policy_arn })
     }
 }
```

### `src/operation/get_policy/builders.rs`

```diff
--- reference/src/operation/get_policy/builders.rs
+++ generated/src/operation/get_policy/builders.rs
@@ -31,14 +31,20 @@
     inner: super::super::super::operation::get_policy::builders::GetPolicyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::get_policy::GetPolicyOutput, super::super::super::operation::get_policy::GetPolicyError>
-    for GetPolicyFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::get_policy::GetPolicyOutput,
+        super::super::super::operation::get_policy::GetPolicyError,
+    > for GetPolicyFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::get_policy::GetPolicyOutput, super::super::super::operation::get_policy::GetPolicyError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::get_policy::GetPolicyOutput,
+            super::super::super::operation::get_policy::GetPolicyError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/get_policy.rs`

```diff
--- reference/src/operation/get_policy.rs
+++ generated/src/operation/get_policy.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::get_policy::GetPolicyInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::get_policy::GetPolicyInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -245,11 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_policy_input::ser_get_policy_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_policy_input::ser_get_policy_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_policy_version/_get_policy_version_input.rs`

```diff
--- reference/src/operation/get_policy_version/_get_policy_version_input.rs
+++ generated/src/operation/get_policy_version/_get_policy_version_input.rs
@@ -76,7 +76,10 @@
     /// Consumes the builder and constructs a [`GetPolicyVersionInput`](crate::operation::get_policy_version::GetPolicyVersionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_policy_version::GetPolicyVersionInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_policy_version::GetPolicyVersionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_policy_version::GetPolicyVersionInput {
             policy_arn: self.policy_arn,
             version_id: self.version_id,
```

### `src/operation/get_policy_version.rs`

```diff
--- reference/src/operation/get_policy_version.rs
+++ generated/src/operation/get_policy_version.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_policy_version_input::ser_get_policy_version_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_policy_version_input::ser_get_policy_version_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_role/_get_role_input.rs`

```diff
--- reference/src/operation/get_role/_get_role_input.rs
+++ generated/src/operation/get_role/_get_role_input.rs
@@ -47,7 +47,9 @@
         &self.role_name
     }
     /// Consumes the builder and constructs a [`GetRoleInput`](crate::operation::get_role::GetRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_role::GetRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_role::GetRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::get_role::GetRoleInput { role_name: self.role_name })
     }
 }
```

### `src/operation/get_role/builders.rs`

```diff
--- reference/src/operation/get_role/builders.rs
+++ generated/src/operation/get_role/builders.rs
@@ -31,14 +31,20 @@
     inner: super::super::super::operation::get_role::builders::GetRoleInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::get_role::GetRoleOutput, super::super::super::operation::get_role::GetRoleError>
-    for GetRoleFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::get_role::GetRoleOutput,
+        super::super::super::operation::get_role::GetRoleError,
+    > for GetRoleFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::get_role::GetRoleOutput, super::super::super::operation::get_role::GetRoleError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::get_role::GetRoleOutput,
+            super::super::super::operation::get_role::GetRoleError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -88,8 +94,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::get_role::GetRoleOutput, super::super::super::operation::get_role::GetRoleError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::get_role::GetRoleOutput,
+        super::super::super::operation::get_role::GetRoleError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/get_role.rs`

```diff
--- reference/src/operation/get_role.rs
+++ generated/src/operation/get_role.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::get_role::GetRoleError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::get_role::GetRoleError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -241,11 +245,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_role_input::ser_get_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_role_input::ser_get_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_role_policy.rs`

```diff
--- reference/src/operation/get_role_policy.rs
+++ generated/src/operation/get_role_policy.rs
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
-            super::super::protocol_serde::shape_get_role_policy_input::ser_get_role_policy_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_role_policy_input::ser_get_role_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_role_template_version.rs`

```diff
--- reference/src/operation/get_role_template_version.rs
+++ generated/src/operation/get_role_template_version.rs
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
-            super::super::protocol_serde::shape_get_role_template_version_input::ser_get_role_template_version_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_role_template_version_input::ser_get_role_template_version_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_saml_provider/_get_saml_provider_input.rs`

```diff
--- reference/src/operation/get_saml_provider/_get_saml_provider_input.rs
+++ generated/src/operation/get_saml_provider/_get_saml_provider_input.rs
@@ -49,7 +49,10 @@
     /// Consumes the builder and constructs a [`GetSamlProviderInput`](crate::operation::get_saml_provider::GetSamlProviderInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_saml_provider::GetSamlProviderInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_saml_provider::GetSamlProviderInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_saml_provider::GetSamlProviderInput {
             saml_provider_arn: self.saml_provider_arn,
         })
```

### `src/operation/get_saml_provider/builders.rs`

```diff
--- reference/src/operation/get_saml_provider/builders.rs
+++ generated/src/operation/get_saml_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_saml_provider::GetSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_saml_provider::GetSAMLProviderError,
+            super::super::super::operation::get_saml_provider::GetSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,13 +20,13 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetSAMLProvider`.
+/// Fluent builder constructing a request to `GetSamlProvider`.
 ///
 /// <p>Returns the SAML provider metadocument that was uploaded when the IAM SAML provider resource object was created or updated.</p><note>
 /// <p>This operation requires <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4</a>.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetSAMLProviderFluentBuilder {
+pub struct GetSamlProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_saml_provider::builders::GetSamlProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -34,8 +34,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_saml_provider::GetSamlProviderOutput,
-        super::super::super::operation::get_saml_provider::GetSAMLProviderError,
-    > for GetSAMLProviderFluentBuilder
+        super::super::super::operation::get_saml_provider::GetSamlProviderError,
+    > for GetSamlProviderFluentBuilder
 {
     fn send(
         self,
@@ -43,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_saml_provider::GetSamlProviderOutput,
-            super::super::super::operation::get_saml_provider::GetSAMLProviderError,
+            super::super::super::operation::get_saml_provider::GetSamlProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetSAMLProviderFluentBuilder {
-    /// Creates a new `GetSAMLProviderFluentBuilder`.
+impl GetSamlProviderFluentBuilder {
+    /// Creates a new `GetSamlProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -58,7 +58,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetSAMLProvider as a reference.
+    /// Access the GetSamlProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_saml_provider::builders::GetSamlProviderInputBuilder {
         &self.inner
     }
@@ -75,7 +75,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_saml_provider::GetSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_saml_provider::GetSAMLProviderError,
+            super::super::super::operation::get_saml_provider::GetSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_saml_provider::GetSAMLProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_saml_provider::GetSamlProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_saml_provider::GetSAMLProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_saml_provider::GetSamlProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +96,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_saml_provider::GetSamlProviderOutput,
-        super::super::super::operation::get_saml_provider::GetSAMLProviderError,
+        super::super::super::operation::get_saml_provider::GetSamlProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_saml_provider.rs`

```diff
--- reference/src/operation/get_saml_provider.rs
+++ generated/src/operation/get_saml_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetSAMLProvider`.
+/// Orchestration and serialization glue logic for `GetSamlProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetSAMLProvider;
-impl GetSAMLProvider {
-    /// Creates a new `GetSAMLProvider`
+pub struct GetSamlProvider;
+impl GetSamlProvider {
+    /// Creates a new `GetSamlProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSAMLProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSamlProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetSAMLProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetSAMLProviderRequestSerializer,
+            GetSamlProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetSAMLProviderResponseDeserializer,
+            GetSamlProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetSAMLProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSAMLProviderTelemetryInputCaptureInterceptor,
+                GetSamlProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSAMLProviderEndpointParamsInterceptor,
+                GetSamlProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_saml_provider::GetSAMLProviderError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct GetSAMLProviderTelemetryInputCaptureInterceptor;
+struct GetSamlProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSAMLProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSamlProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "GetSAMLProviderTelemetryInputCaptureInterceptor"
+        "GetSamlProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_get_saml_provider_input::ser_get_saml_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_saml_provider_input::ser_get_saml_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -262,12 +261,12 @@
     }
 }
 #[derive(Debug)]
-struct GetSAMLProviderEndpointParamsInterceptor;
+struct GetSamlProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSAMLProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSamlProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetSAMLProviderEndpointParamsInterceptor"
+        "GetSamlProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_server_certificate/_get_server_certificate_input.rs`

```diff
--- reference/src/operation/get_server_certificate/_get_server_certificate_input.rs
+++ generated/src/operation/get_server_certificate/_get_server_certificate_input.rs
@@ -49,8 +49,10 @@
     /// Consumes the builder and constructs a [`GetServerCertificateInput`](crate::operation::get_server_certificate::GetServerCertificateInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_server_certificate::GetServerCertificateInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_server_certificate::GetServerCertificateInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_server_certificate::GetServerCertificateInput {
             server_certificate_name: self.server_certificate_name,
         })
```

### `src/operation/get_server_certificate.rs`

```diff
--- reference/src/operation/get_server_certificate.rs
+++ generated/src/operation/get_server_certificate.rs
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
-            super::super::protocol_serde::shape_get_server_certificate_input::ser_get_server_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_server_certificate_input::ser_get_server_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_service_last_accessed_details/_get_service_last_accessed_details_input.rs`

```diff
--- reference/src/operation/get_service_last_accessed_details/_get_service_last_accessed_details_input.rs
+++ generated/src/operation/get_service_last_accessed_details/_get_service_last_accessed_details_input.rs
@@ -95,10 +95,12 @@
         super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsInput {
-            job_id: self.job_id,
-            max_items: self.max_items,
-            marker: self.marker,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsInput {
+                job_id: self.job_id,
+                max_items: self.max_items,
+                marker: self.marker,
+            },
+        )
     }
 }
```

### `src/operation/get_service_last_accessed_details/_get_service_last_accessed_details_output.rs`

```diff
--- reference/src/operation/get_service_last_accessed_details/_get_service_last_accessed_details_output.rs
+++ generated/src/operation/get_service_last_accessed_details/_get_service_last_accessed_details_output.rs
@@ -141,7 +141,10 @@
         self
     }
     /// <p>A <code>ServiceLastAccessed</code> object that contains details about the most recent attempt to access the service.</p>
-    pub fn set_services_last_accessed(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ServiceLastAccessed>>) -> Self {
+    pub fn set_services_last_accessed(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ServiceLastAccessed>>,
+    ) -> Self {
         self.services_last_accessed = input;
         self
     }
@@ -230,36 +233,38 @@
         super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsOutput {
-            job_status: self.job_status.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "job_status",
-                    "job_status was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
-                )
-            })?,
-            job_type: self.job_type,
-            job_creation_date: self.job_creation_date.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "job_creation_date",
-                    "job_creation_date was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
-                )
-            })?,
-            services_last_accessed: self.services_last_accessed.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "services_last_accessed",
-                    "services_last_accessed was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
-                )
-            })?,
-            job_completion_date: self.job_completion_date.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "job_completion_date",
-                    "job_completion_date was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
-                )
-            })?,
-            is_truncated: self.is_truncated.unwrap_or_default(),
-            marker: self.marker,
-            error: self.error,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsOutput {
+                job_status: self.job_status.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "job_status",
+                        "job_status was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
+                    )
+                })?,
+                job_type: self.job_type,
+                job_creation_date: self.job_creation_date.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "job_creation_date",
+                        "job_creation_date was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
+                    )
+                })?,
+                services_last_accessed: self.services_last_accessed.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "services_last_accessed",
+                        "services_last_accessed was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
+                    )
+                })?,
+                job_completion_date: self.job_completion_date.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "job_completion_date",
+                        "job_completion_date was not specified but it is required when building GetServiceLastAccessedDetailsOutput",
+                    )
+                })?,
+                is_truncated: self.is_truncated.unwrap_or_default(),
+                marker: self.marker,
+                error: self.error,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/get_service_last_accessed_details/builders.rs`

```diff
--- reference/src/operation/get_service_last_accessed_details/builders.rs
+++ generated/src/operation/get_service_last_accessed_details/builders.rs
@@ -74,7 +74,9 @@
         }
     }
     /// Access the GetServiceLastAccessedDetails as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::get_service_last_accessed_details::builders::GetServiceLastAccessedDetailsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::get_service_last_accessed_details::builders::GetServiceLastAccessedDetailsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -98,11 +100,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetails::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetails::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::get_service_last_accessed_details::GetServiceLastAccessedDetails::orchestrate(&runtime_plugins, input).await
     }

```

### `src/operation/get_service_last_accessed_details.rs`

```diff
--- reference/src/operation/get_service_last_accessed_details.rs
+++ generated/src/operation/get_service_last_accessed_details.rs
@@ -212,9 +212,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_get_service_last_accessed_details::de_get_service_last_accessed_details_http_error(status, headers, body)
+            super::super::protocol_serde::shape_get_service_last_accessed_details::de_get_service_last_accessed_details_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_get_service_last_accessed_details::de_get_service_last_accessed_details_http_response(status, headers, body)
+            super::super::protocol_serde::shape_get_service_last_accessed_details::de_get_service_last_accessed_details_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -255,12 +259,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_service_last_accessed_details_input::ser_get_service_last_accessed_details_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_service_last_accessed_details_input::ser_get_service_last_accessed_details_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/get_service_last_accessed_details_with_entities/builders.rs`

```diff
--- reference/src/operation/get_service_last_accessed_details_with_entities/builders.rs
+++ generated/src/operation/get_service_last_accessed_details_with_entities/builders.rs
@@ -3,7 +3,9 @@

 pub use super::super::super::operation::get_service_last_accessed_details_with_entities::_get_service_last_accessed_details_with_entities_output::GetServiceLastAccessedDetailsWithEntitiesOutputBuilder;

-impl super::super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesInputBuilder {
+impl
+    super::super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesInputBuilder
+{
     /// Sends a request with this input using the given client.
     pub async fn send_with(
         self,
@@ -66,9 +68,7 @@
         }
     }
     /// Access the GetServiceLastAccessedDetailsWithEntities as a reference.
-    pub fn as_input(
-        &self,
-    ) -> &super::super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesInputBuilder {
+    pub fn as_input(&self) -> &super::super::super::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesInputBuilder{
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -92,12 +92,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntities::operation_runtime_plugins(
-                self.handle.runtime_plugins.clone(),
-                &self.handle.conf,
-                self.config_override,
-            );
+        let runtime_plugins = super::super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntities::operation_runtime_plugins(
+                            self.handle.runtime_plugins.clone(),
+                            &self.handle.conf,
+                            self.config_override,
+                        );
         super::super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntities::orchestrate(
             &runtime_plugins,
             input,
```

### `src/operation/get_service_last_accessed_details_with_entities.rs`

```diff
--- reference/src/operation/get_service_last_accessed_details_with_entities.rs
+++ generated/src/operation/get_service_last_accessed_details_with_entities.rs
@@ -23,19 +23,14 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >| {
             err.map_service_error(|err| {
-                err.downcast::<super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesError>()
-                    .expect("correct error type")
-            })
+                                err.downcast::<super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesError>().expect("correct error type")
+                            })
         };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(
-            output
-                .downcast::<super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesOutput>()
-                .expect("correct output type"),
-        )
+        ::std::result::Result::Ok(output.downcast::<super::super::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesOutput>().expect("correct output type"))
     }

     pub(crate) async fn orchestrate_with_stop_point(
@@ -267,11 +262,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_service_last_accessed_details_with_entities_input::ser_get_service_last_accessed_details_with_entities_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_service_last_accessed_details_with_entities_input::ser_get_service_last_accessed_details_with_entities_op_input(& input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_service_linked_role_deletion_status/builders.rs`

```diff
--- reference/src/operation/get_service_linked_role_deletion_status/builders.rs
+++ generated/src/operation/get_service_linked_role_deletion_status/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the GetServiceLinkedRoleDeletionStatus as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -87,7 +89,11 @@
                 &self.handle.conf,
                 self.config_override,
             );
-        super::super::super::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatus::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatus::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/get_service_linked_role_deletion_status.rs`

```diff
--- reference/src/operation/get_service_linked_role_deletion_status.rs
+++ generated/src/operation/get_service_linked_role_deletion_status.rs
@@ -260,11 +260,14 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_service_linked_role_deletion_status_input::ser_get_service_linked_role_deletion_status_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_get_service_linked_role_deletion_status_input::ser_get_service_linked_role_deletion_status_op_input(
+                &input,
+            )?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -431,7 +434,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for super::super::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusError {
+impl ::aws_types::request_id::RequestId
+    for super::super::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/get_ssh_public_key/_get_ssh_public_key_input.rs`

```diff
--- reference/src/operation/get_ssh_public_key/_get_ssh_public_key_input.rs
+++ generated/src/operation/get_ssh_public_key/_get_ssh_public_key_input.rs
@@ -98,7 +98,10 @@
     /// Consumes the builder and constructs a [`GetSshPublicKeyInput`](crate::operation::get_ssh_public_key::GetSshPublicKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_ssh_public_key::GetSshPublicKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_ssh_public_key::GetSshPublicKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_ssh_public_key::GetSshPublicKeyInput {
             user_name: self.user_name,
             ssh_public_key_id: self.ssh_public_key_id,
```

### `src/operation/get_ssh_public_key/builders.rs`

```diff
--- reference/src/operation/get_ssh_public_key/builders.rs
+++ generated/src/operation/get_ssh_public_key/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_ssh_public_key::GetSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_ssh_public_key::GetSSHPublicKeyError,
+            super::super::super::operation::get_ssh_public_key::GetSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `GetSSHPublicKey`.
+/// Fluent builder constructing a request to `GetSshPublicKey`.
 ///
 /// <p>Retrieves the specified SSH public key, including metadata about the key.</p>
 /// <p>The SSH public key retrieved by this operation is used only for authenticating the associated IAM user to an CodeCommit repository. For more information about using SSH keys to authenticate to an CodeCommit repository, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/setting-up-credentials-ssh.html">Set up CodeCommit for SSH connections</a> in the <i>CodeCommit User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct GetSSHPublicKeyFluentBuilder {
+pub struct GetSshPublicKeyFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::get_ssh_public_key::builders::GetSshPublicKeyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::get_ssh_public_key::GetSshPublicKeyOutput,
-        super::super::super::operation::get_ssh_public_key::GetSSHPublicKeyError,
-    > for GetSSHPublicKeyFluentBuilder
+        super::super::super::operation::get_ssh_public_key::GetSshPublicKeyError,
+    > for GetSshPublicKeyFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::get_ssh_public_key::GetSshPublicKeyOutput,
-            super::super::super::operation::get_ssh_public_key::GetSSHPublicKeyError,
+            super::super::super::operation::get_ssh_public_key::GetSshPublicKeyError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl GetSSHPublicKeyFluentBuilder {
-    /// Creates a new `GetSSHPublicKeyFluentBuilder`.
+impl GetSshPublicKeyFluentBuilder {
+    /// Creates a new `GetSshPublicKeyFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the GetSSHPublicKey as a reference.
+    /// Access the GetSshPublicKey as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::get_ssh_public_key::builders::GetSshPublicKeyInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::get_ssh_public_key::GetSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::get_ssh_public_key::GetSSHPublicKeyError,
+            super::super::super::operation::get_ssh_public_key::GetSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::get_ssh_public_key::GetSSHPublicKey::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::get_ssh_public_key::GetSshPublicKey::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::get_ssh_public_key::GetSSHPublicKey::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::get_ssh_public_key::GetSshPublicKey::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::get_ssh_public_key::GetSshPublicKeyOutput,
-        super::super::super::operation::get_ssh_public_key::GetSSHPublicKeyError,
+        super::super::super::operation::get_ssh_public_key::GetSshPublicKeyError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/get_ssh_public_key.rs`

```diff
--- reference/src/operation/get_ssh_public_key.rs
+++ generated/src/operation/get_ssh_public_key.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `GetSSHPublicKey`.
+/// Orchestration and serialization glue logic for `GetSshPublicKey`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct GetSSHPublicKey;
-impl GetSSHPublicKey {
-    /// Creates a new `GetSSHPublicKey`
+pub struct GetSshPublicKey;
+impl GetSshPublicKey {
+    /// Creates a new `GetSshPublicKey`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSSHPublicKey {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetSshPublicKey {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetSSHPublicKey");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetSSHPublicKeyRequestSerializer,
+            GetSshPublicKeyRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetSSHPublicKeyResponseDeserializer,
+            GetSshPublicKeyResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetSSHPublicKey")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSSHPublicKeyTelemetryInputCaptureInterceptor,
+                GetSshPublicKeyTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetSSHPublicKeyEndpointParamsInterceptor,
+                GetSshPublicKeyEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::get_ssh_public_key::GetSSHPublicKeyError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct GetSSHPublicKeyTelemetryInputCaptureInterceptor;
+struct GetSshPublicKeyTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSSHPublicKeyTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSshPublicKeyTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "GetSSHPublicKeyTelemetryInputCaptureInterceptor"
+        "GetSshPublicKeyTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_get_ssh_public_key_input::ser_get_ssh_public_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_get_ssh_public_key_input::ser_get_ssh_public_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct GetSSHPublicKeyEndpointParamsInterceptor;
+struct GetSshPublicKeyEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSSHPublicKeyEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetSshPublicKeyEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "GetSSHPublicKeyEndpointParamsInterceptor"
+        "GetSshPublicKeyEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/get_user/_get_user_input.rs`

```diff
--- reference/src/operation/get_user/_get_user_input.rs
+++ generated/src/operation/get_user/_get_user_input.rs
@@ -46,7 +46,9 @@
         &self.user_name
     }
     /// Consumes the builder and constructs a [`GetUserInput`](crate::operation::get_user::GetUserInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_user::GetUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_user::GetUserInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::get_user::GetUserInput { user_name: self.user_name })
     }
 }
```

### `src/operation/get_user/builders.rs`

```diff
--- reference/src/operation/get_user/builders.rs
+++ generated/src/operation/get_user/builders.rs
@@ -30,14 +30,20 @@
     inner: super::super::super::operation::get_user::builders::GetUserInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::get_user::GetUserOutput, super::super::super::operation::get_user::GetUserError>
-    for GetUserFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::get_user::GetUserOutput,
+        super::super::super::operation::get_user::GetUserError,
+    > for GetUserFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::get_user::GetUserOutput, super::super::super::operation::get_user::GetUserError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::get_user::GetUserOutput,
+            super::super::super::operation::get_user::GetUserError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -87,8 +93,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::get_user::GetUserOutput, super::super::super::operation::get_user::GetUserError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::get_user::GetUserOutput,
+        super::super::super::operation::get_user::GetUserError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/get_user.rs`

```diff
--- reference/src/operation/get_user.rs
+++ generated/src/operation/get_user.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::get_user::GetUserError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::get_user::GetUserError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -241,11 +245,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_user_input::ser_get_user_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_user_input::ser_get_user_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_user_policy.rs`

```diff
--- reference/src/operation/get_user_policy.rs
+++ generated/src/operation/get_user_policy.rs
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
-            super::super::protocol_serde::shape_get_user_policy_input::ser_get_user_policy_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_user_policy_input::ser_get_user_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_access_keys.rs`

```diff
--- reference/src/operation/list_access_keys.rs
+++ generated/src/operation/list_access_keys.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_access_keys_input::ser_list_access_keys_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_access_keys_input::ser_list_access_keys_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_account_aliases/_list_account_aliases_input.rs`

```diff
--- reference/src/operation/list_account_aliases/_list_account_aliases_input.rs
+++ generated/src/operation/list_account_aliases/_list_account_aliases_input.rs
@@ -69,8 +69,10 @@
     /// Consumes the builder and constructs a [`ListAccountAliasesInput`](crate::operation::list_account_aliases::ListAccountAliasesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_account_aliases::ListAccountAliasesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_account_aliases::ListAccountAliasesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_account_aliases::ListAccountAliasesInput {
             marker: self.marker,
             max_items: self.max_items,
```

### `src/operation/list_account_aliases/_list_account_aliases_output.rs`

```diff
--- reference/src/operation/list_account_aliases/_list_account_aliases_output.rs
+++ generated/src/operation/list_account_aliases/_list_account_aliases_output.rs
@@ -111,8 +111,10 @@
     /// - [`account_aliases`](crate::operation::list_account_aliases::builders::ListAccountAliasesOutputBuilder::account_aliases)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_account_aliases::ListAccountAliasesOutput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_account_aliases::ListAccountAliasesOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_account_aliases::ListAccountAliasesOutput {
             account_aliases: self.account_aliases.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_account_aliases.rs`

```diff
--- reference/src/operation/list_account_aliases.rs
+++ generated/src/operation/list_account_aliases.rs
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
-            super::super::protocol_serde::shape_list_account_aliases_input::ser_list_account_aliases_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_account_aliases_input::ser_list_account_aliases_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_attached_group_policies/_list_attached_group_policies_input.rs`

```diff
--- reference/src/operation/list_attached_group_policies/_list_attached_group_policies_input.rs
+++ generated/src/operation/list_attached_group_policies/_list_attached_group_policies_input.rs
@@ -126,11 +126,13 @@
         super::super::super::operation::list_attached_group_policies::ListAttachedGroupPoliciesInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_attached_group_policies::ListAttachedGroupPoliciesInput {
-            group_name: self.group_name,
-            path_prefix: self.path_prefix,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_attached_group_policies::ListAttachedGroupPoliciesInput {
+                group_name: self.group_name,
+                path_prefix: self.path_prefix,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_attached_group_policies/paginator.rs`

```diff
--- reference/src/operation/list_attached_group_policies/paginator.rs
+++ generated/src/operation/list_attached_group_policies/paginator.rs
@@ -86,9 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_attached_group_policies::ListAttachedGroupPolicies::orchestrate(&runtime_plugins, input.clone())
-                                .await;
+                        let resp = super::super::super::operation::list_attached_group_policies::ListAttachedGroupPolicies::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_attached_group_policies.rs`

```diff
--- reference/src/operation/list_attached_group_policies.rs
+++ generated/src/operation/list_attached_group_policies.rs
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
-            super::super::protocol_serde::shape_list_attached_group_policies_input::ser_list_attached_group_policies_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_attached_group_policies_input::ser_list_attached_group_policies_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_attached_role_policies/_list_attached_role_policies_input.rs`

```diff
--- reference/src/operation/list_attached_role_policies/_list_attached_role_policies_input.rs
+++ generated/src/operation/list_attached_role_policies/_list_attached_role_policies_input.rs
@@ -126,11 +126,13 @@
         super::super::super::operation::list_attached_role_policies::ListAttachedRolePoliciesInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_attached_role_policies::ListAttachedRolePoliciesInput {
-            role_name: self.role_name,
-            path_prefix: self.path_prefix,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_attached_role_policies::ListAttachedRolePoliciesInput {
+                role_name: self.role_name,
+                path_prefix: self.path_prefix,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_attached_role_policies/paginator.rs`

```diff
--- reference/src/operation/list_attached_role_policies/paginator.rs
+++ generated/src/operation/list_attached_role_policies/paginator.rs
@@ -86,9 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_attached_role_policies::ListAttachedRolePolicies::orchestrate(&runtime_plugins, input.clone())
-                                .await;
+                        let resp = super::super::super::operation::list_attached_role_policies::ListAttachedRolePolicies::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_attached_role_policies.rs`

```diff
--- reference/src/operation/list_attached_role_policies.rs
+++ generated/src/operation/list_attached_role_policies.rs
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
-            super::super::protocol_serde::shape_list_attached_role_policies_input::ser_list_attached_role_policies_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_attached_role_policies_input::ser_list_attached_role_policies_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_attached_user_policies/_list_attached_user_policies_input.rs`

```diff
--- reference/src/operation/list_attached_user_policies/_list_attached_user_policies_input.rs
+++ generated/src/operation/list_attached_user_policies/_list_attached_user_policies_input.rs
@@ -126,11 +126,13 @@
         super::super::super::operation::list_attached_user_policies::ListAttachedUserPoliciesInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_attached_user_policies::ListAttachedUserPoliciesInput {
-            user_name: self.user_name,
-            path_prefix: self.path_prefix,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_attached_user_policies::ListAttachedUserPoliciesInput {
+                user_name: self.user_name,
+                path_prefix: self.path_prefix,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_attached_user_policies/paginator.rs`

```diff
--- reference/src/operation/list_attached_user_policies/paginator.rs
+++ generated/src/operation/list_attached_user_policies/paginator.rs
@@ -86,9 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_attached_user_policies::ListAttachedUserPolicies::orchestrate(&runtime_plugins, input.clone())
-                                .await;
+                        let resp = super::super::super::operation::list_attached_user_policies::ListAttachedUserPolicies::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_attached_user_policies.rs`

```diff
--- reference/src/operation/list_attached_user_policies.rs
+++ generated/src/operation/list_attached_user_policies.rs
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
-            super::super::protocol_serde::shape_list_attached_user_policies_input::ser_list_attached_user_policies_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_attached_user_policies_input::ser_list_attached_user_policies_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_delegation_requests.rs`

```diff
--- reference/src/operation/list_delegation_requests.rs
+++ generated/src/operation/list_delegation_requests.rs
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
-            super::super::protocol_serde::shape_list_delegation_requests_input::ser_list_delegation_requests_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_delegation_requests_input::ser_list_delegation_requests_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_entities_for_policy/_list_entities_for_policy_input.rs`

```diff
--- reference/src/operation/list_entities_for_policy/_list_entities_for_policy_input.rs
+++ generated/src/operation/list_entities_for_policy/_list_entities_for_policy_input.rs
@@ -179,8 +179,10 @@
     /// Consumes the builder and constructs a [`ListEntitiesForPolicyInput`](crate::operation::list_entities_for_policy::ListEntitiesForPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_entities_for_policy::ListEntitiesForPolicyInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_entities_for_policy::ListEntitiesForPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_entities_for_policy::ListEntitiesForPolicyInput {
             policy_arn: self.policy_arn,
             entity_filter: self.entity_filter,
```

### `src/operation/list_entities_for_policy/paginator.rs`

```diff
--- reference/src/operation/list_entities_for_policy/paginator.rs
+++ generated/src/operation/list_entities_for_policy/paginator.rs
@@ -78,8 +78,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_entities_for_policy::ListEntitiesForPolicy::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_entities_for_policy::ListEntitiesForPolicy::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_entities_for_policy.rs`

```diff
--- reference/src/operation/list_entities_for_policy.rs
+++ generated/src/operation/list_entities_for_policy.rs
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
-            super::super::protocol_serde::shape_list_entities_for_policy_input::ser_list_entities_for_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_entities_for_policy_input::ser_list_entities_for_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_group_policies/_list_group_policies_input.rs`

```diff
--- reference/src/operation/list_group_policies/_list_group_policies_input.rs
+++ generated/src/operation/list_group_policies/_list_group_policies_input.rs
@@ -96,7 +96,10 @@
     /// Consumes the builder and constructs a [`ListGroupPoliciesInput`](crate::operation::list_group_policies::ListGroupPoliciesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_group_policies::ListGroupPoliciesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_group_policies::ListGroupPoliciesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_group_policies::ListGroupPoliciesInput {
             group_name: self.group_name,
             marker: self.marker,
```

### `src/operation/list_group_policies/_list_group_policies_output.rs`

```diff
--- reference/src/operation/list_group_policies/_list_group_policies_output.rs
+++ generated/src/operation/list_group_policies/_list_group_policies_output.rs
@@ -116,7 +116,10 @@
     /// - [`policy_names`](crate::operation::list_group_policies::builders::ListGroupPoliciesOutputBuilder::policy_names)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_group_policies::ListGroupPoliciesOutput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_group_policies::ListGroupPoliciesOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_group_policies::ListGroupPoliciesOutput {
             policy_names: self.policy_names.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_group_policies.rs`

```diff
--- reference/src/operation/list_group_policies.rs
+++ generated/src/operation/list_group_policies.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_group_policies_input::ser_list_group_policies_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_group_policies_input::ser_list_group_policies_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_groups/_list_groups_input.rs`

```diff
--- reference/src/operation/list_groups/_list_groups_input.rs
+++ generated/src/operation/list_groups/_list_groups_input.rs
@@ -93,7 +93,9 @@
         &self.max_items
     }
     /// Consumes the builder and constructs a [`ListGroupsInput`](crate::operation::list_groups::ListGroupsInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_groups::ListGroupsInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_groups::ListGroupsInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_groups::ListGroupsInput {
             path_prefix: self.path_prefix,
             marker: self.marker,
```

### `src/operation/list_groups/_list_groups_output.rs`

```diff
--- reference/src/operation/list_groups/_list_groups_output.rs
+++ generated/src/operation/list_groups/_list_groups_output.rs
@@ -109,7 +109,9 @@
     /// Consumes the builder and constructs a [`ListGroupsOutput`](crate::operation::list_groups::ListGroupsOutput).
     /// This method will fail if any of the following fields are not set:
     /// - [`groups`](crate::operation::list_groups::builders::ListGroupsOutputBuilder::groups)
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_groups::ListGroupsOutput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_groups::ListGroupsOutput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_groups::ListGroupsOutput {
             groups: self.groups.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_groups/paginator.rs`

```diff
--- reference/src/operation/list_groups/paginator.rs
+++ generated/src/operation/list_groups/paginator.rs
@@ -139,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_groups_output_output_groups(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_groups_output_output_groups(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_groups.rs`

```diff
--- reference/src/operation/list_groups.rs
+++ generated/src/operation/list_groups.rs
@@ -225,7 +225,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::list_groups::ListGroupsInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::list_groups::ListGroupsInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -250,12 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_groups_input::ser_list_groups_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_groups_input::ser_list_groups_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_groups_for_user/_list_groups_for_user_input.rs`

```diff
--- reference/src/operation/list_groups_for_user/_list_groups_for_user_input.rs
+++ generated/src/operation/list_groups_for_user/_list_groups_for_user_input.rs
@@ -96,7 +96,10 @@
     /// Consumes the builder and constructs a [`ListGroupsForUserInput`](crate::operation::list_groups_for_user::ListGroupsForUserInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_groups_for_user::ListGroupsForUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_groups_for_user::ListGroupsForUserInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_groups_for_user::ListGroupsForUserInput {
             user_name: self.user_name,
             marker: self.marker,
```

### `src/operation/list_groups_for_user/_list_groups_for_user_output.rs`

```diff
--- reference/src/operation/list_groups_for_user/_list_groups_for_user_output.rs
+++ generated/src/operation/list_groups_for_user/_list_groups_for_user_output.rs
@@ -111,8 +111,10 @@
     /// - [`groups`](crate::operation::list_groups_for_user::builders::ListGroupsForUserOutputBuilder::groups)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_groups_for_user::ListGroupsForUserOutput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_groups_for_user::ListGroupsForUserOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_groups_for_user::ListGroupsForUserOutput {
             groups: self.groups.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_groups_for_user.rs`

```diff
--- reference/src/operation/list_groups_for_user.rs
+++ generated/src/operation/list_groups_for_user.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_groups_for_user_input::ser_list_groups_for_user_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_groups_for_user_input::ser_list_groups_for_user_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_instance_profile_tags/_list_instance_profile_tags_output.rs`

```diff
--- reference/src/operation/list_instance_profile_tags/_list_instance_profile_tags_output.rs
+++ generated/src/operation/list_instance_profile_tags/_list_instance_profile_tags_output.rs
@@ -114,16 +114,18 @@
         super::super::super::operation::list_instance_profile_tags::ListInstanceProfileTagsOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_instance_profile_tags::ListInstanceProfileTagsOutput {
-            tags: self.tags.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "tags",
-                    "tags was not specified but it is required when building ListInstanceProfileTagsOutput",
-                )
-            })?,
-            is_truncated: self.is_truncated.unwrap_or_default(),
-            marker: self.marker,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_instance_profile_tags::ListInstanceProfileTagsOutput {
+                tags: self.tags.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "tags",
+                        "tags was not specified but it is required when building ListInstanceProfileTagsOutput",
+                    )
+                })?,
+                is_truncated: self.is_truncated.unwrap_or_default(),
+                marker: self.marker,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/list_instance_profile_tags/paginator.rs`

```diff
--- reference/src/operation/list_instance_profile_tags/paginator.rs
+++ generated/src/operation/list_instance_profile_tags/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_instance_profile_tags::ListInstanceProfileTags::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_instance_profile_tags::ListInstanceProfileTags::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_instance_profile_tags.rs`

```diff
--- reference/src/operation/list_instance_profile_tags.rs
+++ generated/src/operation/list_instance_profile_tags.rs
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
-            super::super::protocol_serde::shape_list_instance_profile_tags_input::ser_list_instance_profile_tags_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_instance_profile_tags_input::ser_list_instance_profile_tags_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_instance_profiles/_list_instance_profiles_input.rs`

```diff
--- reference/src/operation/list_instance_profiles/_list_instance_profiles_input.rs
+++ generated/src/operation/list_instance_profiles/_list_instance_profiles_input.rs
@@ -95,8 +95,10 @@
     /// Consumes the builder and constructs a [`ListInstanceProfilesInput`](crate::operation::list_instance_profiles::ListInstanceProfilesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_instance_profiles::ListInstanceProfilesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_instance_profiles::ListInstanceProfilesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_instance_profiles::ListInstanceProfilesInput {
             path_prefix: self.path_prefix,
             marker: self.marker,
```

### `src/operation/list_instance_profiles/_list_instance_profiles_output.rs`

```diff
--- reference/src/operation/list_instance_profiles/_list_instance_profiles_output.rs
+++ generated/src/operation/list_instance_profiles/_list_instance_profiles_output.rs
@@ -111,8 +111,10 @@
     /// - [`instance_profiles`](crate::operation::list_instance_profiles::builders::ListInstanceProfilesOutputBuilder::instance_profiles)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_instance_profiles::ListInstanceProfilesOutput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_instance_profiles::ListInstanceProfilesOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_instance_profiles::ListInstanceProfilesOutput {
             instance_profiles: self.instance_profiles.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_instance_profiles/paginator.rs`

```diff
--- reference/src/operation/list_instance_profiles/paginator.rs
+++ generated/src/operation/list_instance_profiles/paginator.rs
@@ -86,7 +86,11 @@
                         }
                     };
                     loop {
-                        let resp = super::super::super::operation::list_instance_profiles::ListInstanceProfiles::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_instance_profiles::ListInstanceProfiles::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_instance_profiles.rs`

```diff
--- reference/src/operation/list_instance_profiles.rs
+++ generated/src/operation/list_instance_profiles.rs
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
-            super::super::protocol_serde::shape_list_instance_profiles_input::ser_list_instance_profiles_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_instance_profiles_input::ser_list_instance_profiles_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_instance_profiles_for_role/_list_instance_profiles_for_role_input.rs`

```diff
--- reference/src/operation/list_instance_profiles_for_role/_list_instance_profiles_for_role_input.rs
+++ generated/src/operation/list_instance_profiles_for_role/_list_instance_profiles_for_role_input.rs
@@ -100,10 +100,12 @@
         super::super::super::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleInput {
-            role_name: self.role_name,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleInput {
+                role_name: self.role_name,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_instance_profiles_for_role/_list_instance_profiles_for_role_output.rs`

```diff
--- reference/src/operation/list_instance_profiles_for_role/_list_instance_profiles_for_role_output.rs
+++ generated/src/operation/list_instance_profiles_for_role/_list_instance_profiles_for_role_output.rs
@@ -115,16 +115,18 @@
         super::super::super::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleOutput {
-            instance_profiles: self.instance_profiles.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "instance_profiles",
-                    "instance_profiles was not specified but it is required when building ListInstanceProfilesForRoleOutput",
-                )
-            })?,
-            is_truncated: self.is_truncated.unwrap_or_default(),
-            marker: self.marker,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleOutput {
+                instance_profiles: self.instance_profiles.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "instance_profiles",
+                        "instance_profiles was not specified but it is required when building ListInstanceProfilesForRoleOutput",
+                    )
+                })?,
+                is_truncated: self.is_truncated.unwrap_or_default(),
+                marker: self.marker,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/list_instance_profiles_for_role.rs`

```diff
--- reference/src/operation/list_instance_profiles_for_role.rs
+++ generated/src/operation/list_instance_profiles_for_role.rs
@@ -214,7 +214,9 @@
         let parse_result = if !success && status != 200 || force_error {
             super::super::protocol_serde::shape_list_instance_profiles_for_role::de_list_instance_profiles_for_role_http_error(status, headers, body)
         } else {
-            super::super::protocol_serde::shape_list_instance_profiles_for_role::de_list_instance_profiles_for_role_http_response(status, headers, body)
+            super::super::protocol_serde::shape_list_instance_profiles_for_role::de_list_instance_profiles_for_role_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -255,12 +257,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_instance_profiles_for_role_input::ser_list_instance_profiles_for_role_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_instance_profiles_for_role_input::ser_list_instance_profiles_for_role_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_mfa_device_tags/_list_mfa_device_tags_input.rs`

```diff
--- reference/src/operation/list_mfa_device_tags/_list_mfa_device_tags_input.rs
+++ generated/src/operation/list_mfa_device_tags/_list_mfa_device_tags_input.rs
@@ -96,7 +96,10 @@
     /// Consumes the builder and constructs a [`ListMfaDeviceTagsInput`](crate::operation::list_mfa_device_tags::ListMfaDeviceTagsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsInput {
             serial_number: self.serial_number,
             marker: self.marker,
```

### `src/operation/list_mfa_device_tags/_list_mfa_device_tags_output.rs`

```diff
--- reference/src/operation/list_mfa_device_tags/_list_mfa_device_tags_output.rs
+++ generated/src/operation/list_mfa_device_tags/_list_mfa_device_tags_output.rs
@@ -110,8 +110,10 @@
     /// - [`tags`](crate::operation::list_mfa_device_tags::builders::ListMfaDeviceTagsOutputBuilder::tags)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput {
             tags: self.tags.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_mfa_device_tags/builders.rs`

```diff
--- reference/src/operation/list_mfa_device_tags/builders.rs
+++ generated/src/operation/list_mfa_device_tags/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_mfa_device_tags::ListMFADeviceTagsError,
+            super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListMFADeviceTags`.
+/// Fluent builder constructing a request to `ListMfaDeviceTags`.
 ///
 /// <p>Lists the tags that are attached to the specified IAM virtual multi-factor authentication (MFA) device. The returned list of tags is sorted by tag key. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListMFADeviceTagsFluentBuilder {
+pub struct ListMfaDeviceTagsFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_mfa_device_tags::builders::ListMfaDeviceTagsInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput,
-        super::super::super::operation::list_mfa_device_tags::ListMFADeviceTagsError,
-    > for ListMFADeviceTagsFluentBuilder
+        super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsError,
+    > for ListMfaDeviceTagsFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput,
-            super::super::super::operation::list_mfa_device_tags::ListMFADeviceTagsError,
+            super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListMFADeviceTagsFluentBuilder {
-    /// Creates a new `ListMFADeviceTagsFluentBuilder`.
+impl ListMfaDeviceTagsFluentBuilder {
+    /// Creates a new `ListMfaDeviceTagsFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListMFADeviceTags as a reference.
+    /// Access the ListMfaDeviceTags as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_mfa_device_tags::builders::ListMfaDeviceTagsInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_mfa_device_tags::ListMFADeviceTagsError,
+            super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_mfa_device_tags::ListMFADeviceTags::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTags::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_mfa_device_tags::ListMFADeviceTags::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTags::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput,
-        super::super::super::operation::list_mfa_device_tags::ListMFADeviceTagsError,
+        super::super::super::operation::list_mfa_device_tags::ListMfaDeviceTagsError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_mfa_device_tags.rs`

```diff
--- reference/src/operation/list_mfa_device_tags.rs
+++ generated/src/operation/list_mfa_device_tags.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListMFADeviceTags`.
+/// Orchestration and serialization glue logic for `ListMfaDeviceTags`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListMFADeviceTags;
-impl ListMFADeviceTags {
-    /// Creates a new `ListMFADeviceTags`
+pub struct ListMfaDeviceTags;
+impl ListMfaDeviceTags {
+    /// Creates a new `ListMfaDeviceTags`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListMFADeviceTags {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListMfaDeviceTags {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListMFADeviceTags");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListMFADeviceTagsRequestSerializer,
+            ListMfaDeviceTagsRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListMFADeviceTagsResponseDeserializer,
+            ListMfaDeviceTagsResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListMFADeviceTags")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListMFADeviceTagsTelemetryInputCaptureInterceptor,
+                ListMfaDeviceTagsTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListMFADeviceTagsEndpointParamsInterceptor,
+                ListMfaDeviceTagsEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_mfa_device_tags::ListMFADeviceTagsError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct ListMFADeviceTagsTelemetryInputCaptureInterceptor;
+struct ListMfaDeviceTagsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMFADeviceTagsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMfaDeviceTagsTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListMFADeviceTagsTelemetryInputCaptureInterceptor"
+        "ListMfaDeviceTagsTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_mfa_device_tags_input::ser_list_mfa_device_tags_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_mfa_device_tags_input::ser_list_mfa_device_tags_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct ListMFADeviceTagsEndpointParamsInterceptor;
+struct ListMfaDeviceTagsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMFADeviceTagsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMfaDeviceTagsEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListMFADeviceTagsEndpointParamsInterceptor"
+        "ListMfaDeviceTagsEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_mfa_devices/builders.rs`

```diff
--- reference/src/operation/list_mfa_devices/builders.rs
+++ generated/src/operation/list_mfa_devices/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_mfa_devices::ListMfaDevicesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_mfa_devices::ListMFADevicesError,
+            super::super::super::operation::list_mfa_devices::ListMfaDevicesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListMFADevices`.
+/// Fluent builder constructing a request to `ListMfaDevices`.
 ///
 /// <p>Lists the MFA devices for an IAM user. If the request includes a IAM user name, then this operation lists all the MFA devices associated with the specified user. If you do not specify a user name, IAM determines the user name implicitly based on the Amazon Web Services access key ID signing the request for this operation.</p>
 /// <p>You can paginate the results using the <code>MaxItems</code> and <code>Marker</code> parameters.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListMFADevicesFluentBuilder {
+pub struct ListMfaDevicesFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_mfa_devices::builders::ListMfaDevicesInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_mfa_devices::ListMfaDevicesOutput,
-        super::super::super::operation::list_mfa_devices::ListMFADevicesError,
-    > for ListMFADevicesFluentBuilder
+        super::super::super::operation::list_mfa_devices::ListMfaDevicesError,
+    > for ListMfaDevicesFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_mfa_devices::ListMfaDevicesOutput,
-            super::super::super::operation::list_mfa_devices::ListMFADevicesError,
+            super::super::super::operation::list_mfa_devices::ListMfaDevicesError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListMFADevicesFluentBuilder {
-    /// Creates a new `ListMFADevicesFluentBuilder`.
+impl ListMfaDevicesFluentBuilder {
+    /// Creates a new `ListMfaDevicesFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListMFADevices as a reference.
+    /// Access the ListMfaDevices as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_mfa_devices::builders::ListMfaDevicesInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_mfa_devices::ListMfaDevicesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_mfa_devices::ListMFADevicesError,
+            super::super::super::operation::list_mfa_devices::ListMfaDevicesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_mfa_devices::ListMFADevices::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_mfa_devices::ListMfaDevices::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_mfa_devices::ListMFADevices::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_mfa_devices::ListMfaDevices::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_mfa_devices::ListMfaDevicesOutput,
-        super::super::super::operation::list_mfa_devices::ListMFADevicesError,
+        super::super::super::operation::list_mfa_devices::ListMfaDevicesError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_mfa_devices.rs`

```diff
--- reference/src/operation/list_mfa_devices.rs
+++ generated/src/operation/list_mfa_devices.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListMFADevices`.
+/// Orchestration and serialization glue logic for `ListMfaDevices`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListMFADevices;
-impl ListMFADevices {
-    /// Creates a new `ListMFADevices`
+pub struct ListMfaDevices;
+impl ListMfaDevices {
+    /// Creates a new `ListMfaDevices`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListMFADevices {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListMfaDevices {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListMFADevices");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListMFADevicesRequestSerializer,
+            ListMfaDevicesRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListMFADevicesResponseDeserializer,
+            ListMfaDevicesResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListMFADevices")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListMFADevicesTelemetryInputCaptureInterceptor,
+                ListMfaDevicesTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListMFADevicesEndpointParamsInterceptor,
+                ListMfaDevicesEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_mfa_devices::ListMFADevicesError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct ListMFADevicesTelemetryInputCaptureInterceptor;
+struct ListMfaDevicesTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMFADevicesTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMfaDevicesTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListMFADevicesTelemetryInputCaptureInterceptor"
+        "ListMfaDevicesTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_mfa_devices_input::ser_list_mfa_devices_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_mfa_devices_input::ser_list_mfa_devices_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct ListMFADevicesEndpointParamsInterceptor;
+struct ListMfaDevicesEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMFADevicesEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListMfaDevicesEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListMFADevicesEndpointParamsInterceptor"
+        "ListMfaDevicesEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_open_id_connect_provider_tags/_list_open_id_connect_provider_tags_input.rs`

```diff
--- reference/src/operation/list_open_id_connect_provider_tags/_list_open_id_connect_provider_tags_input.rs
+++ generated/src/operation/list_open_id_connect_provider_tags/_list_open_id_connect_provider_tags_input.rs
@@ -100,10 +100,12 @@
         super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsInput {
-            open_id_connect_provider_arn: self.open_id_connect_provider_arn,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsInput {
+                open_id_connect_provider_arn: self.open_id_connect_provider_arn,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_open_id_connect_provider_tags/builders.rs`

```diff
--- reference/src/operation/list_open_id_connect_provider_tags/builders.rs
+++ generated/src/operation/list_open_id_connect_provider_tags/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTagsError,
+            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListOpenIDConnectProviderTags`.
+/// Fluent builder constructing a request to `ListOpenIdConnectProviderTags`.
 ///
 /// <p>Lists the tags that are attached to the specified OpenID Connect (OIDC)-compatible identity provider. The returned list of tags is sorted by tag key. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_oidc.html">About web identity federation</a>.</p>
 /// <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListOpenIDConnectProviderTagsFluentBuilder {
+pub struct ListOpenIdConnectProviderTagsFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_open_id_connect_provider_tags::builders::ListOpenIdConnectProviderTagsInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput,
-        super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTagsError,
-    > for ListOpenIDConnectProviderTagsFluentBuilder
+        super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsError,
+    > for ListOpenIdConnectProviderTagsFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput,
-            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTagsError,
+            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListOpenIDConnectProviderTagsFluentBuilder {
-    /// Creates a new `ListOpenIDConnectProviderTagsFluentBuilder`.
+impl ListOpenIdConnectProviderTagsFluentBuilder {
+    /// Creates a new `ListOpenIdConnectProviderTagsFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,8 +57,10 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListOpenIDConnectProviderTags as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::list_open_id_connect_provider_tags::builders::ListOpenIdConnectProviderTagsInputBuilder {
+    /// Access the ListOpenIdConnectProviderTags as a reference.
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::list_open_id_connect_provider_tags::builders::ListOpenIdConnectProviderTagsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -74,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTagsError,
+            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +84,13 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTags::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTags::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTags::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTags::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +98,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput,
-        super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTagsError,
+        super::super::super::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
@@ -112,8 +115,13 @@
     /// Create a paginator for this request
     ///
     /// Paginators are used by calling [`send().await`](crate::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator {
-        super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator::new(self.handle, self.inner)
+    pub fn into_paginator(
+        self,
+    ) -> super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator {
+        super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator::new(
+            self.handle,
+            self.inner,
+        )
     }
     /// <p>The ARN of the OpenID Connect (OIDC) identity provider whose tags you want to see.</p>
     /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
```

### `src/operation/list_open_id_connect_provider_tags.rs`

```diff
--- reference/src/operation/list_open_id_connect_provider_tags.rs
+++ generated/src/operation/list_open_id_connect_provider_tags.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListOpenIDConnectProviderTags`.
+/// Orchestration and serialization glue logic for `ListOpenIdConnectProviderTags`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListOpenIDConnectProviderTags;
-impl ListOpenIDConnectProviderTags {
-    /// Creates a new `ListOpenIDConnectProviderTags`
+pub struct ListOpenIdConnectProviderTags;
+impl ListOpenIdConnectProviderTags {
+    /// Creates a new `ListOpenIdConnectProviderTags`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListOpenIDConnectProviderTags {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListOpenIdConnectProviderTags {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListOpenIDConnectProviderTags");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListOpenIDConnectProviderTagsRequestSerializer,
+            ListOpenIdConnectProviderTagsRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListOpenIDConnectProviderTagsResponseDeserializer,
+            ListOpenIdConnectProviderTagsResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListOpenIDConnectProviderTags")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListOpenIDConnectProviderTagsTelemetryInputCaptureInterceptor,
+                ListOpenIdConnectProviderTagsTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListOpenIDConnectProviderTagsEndpointParamsInterceptor,
+                ListOpenIdConnectProviderTagsEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTagsError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct ListOpenIDConnectProviderTagsTelemetryInputCaptureInterceptor;
+struct ListOpenIdConnectProviderTagsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListOpenIDConnectProviderTagsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListOpenIdConnectProviderTagsTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListOpenIDConnectProviderTagsTelemetryInputCaptureInterceptor"
+        "ListOpenIdConnectProviderTagsTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -212,7 +212,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_list_open_id_connect_provider_tags::de_list_open_id_connect_provider_tags_http_error(status, headers, body)
+            super::super::protocol_serde::shape_list_open_id_connect_provider_tags::de_list_open_id_connect_provider_tags_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_list_open_id_connect_provider_tags::de_list_open_id_connect_provider_tags_http_response(
                 status, headers, body,
@@ -257,12 +259,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_open_id_connect_provider_tags_input::ser_list_open_id_connect_provider_tags_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_open_id_connect_provider_tags_input::ser_list_open_id_connect_provider_tags_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -272,12 +273,12 @@
     }
 }
 #[derive(Debug)]
-struct ListOpenIDConnectProviderTagsEndpointParamsInterceptor;
+struct ListOpenIdConnectProviderTagsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListOpenIDConnectProviderTagsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListOpenIdConnectProviderTagsEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListOpenIDConnectProviderTagsEndpointParamsInterceptor"
+        "ListOpenIdConnectProviderTagsEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_open_id_connect_providers/_list_open_id_connect_providers_output.rs`

```diff
--- reference/src/operation/list_open_id_connect_providers/_list_open_id_connect_providers_output.rs
+++ generated/src/operation/list_open_id_connect_providers/_list_open_id_connect_providers_output.rs
@@ -56,7 +56,9 @@
         self
     }
     /// <p>The list of IAM OIDC provider resource objects defined in the Amazon Web Services account.</p>
-    pub fn get_open_id_connect_provider_list(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::OpenIdConnectProviderListEntry>> {
+    pub fn get_open_id_connect_provider_list(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::OpenIdConnectProviderListEntry>> {
         &self.open_id_connect_provider_list
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/list_open_id_connect_providers/builders.rs`

```diff
--- reference/src/operation/list_open_id_connect_providers/builders.rs
+++ generated/src/operation/list_open_id_connect_providers/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProvidersError,
+            super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,13 +20,13 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListOpenIDConnectProviders`.
+/// Fluent builder constructing a request to `ListOpenIdConnectProviders`.
 ///
 /// <p>Lists information about the IAM OpenID Connect (OIDC) provider resource objects defined in the Amazon Web Services account.</p><note>
 /// <p>IAM resource-listing operations return a subset of the available attributes for the resource. For example, this operation does not return tags, even though they are an attribute of the returned object. To view all of the information for an OIDC provider, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetOpenIDConnectProvider.html">GetOpenIDConnectProvider</a>.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListOpenIDConnectProvidersFluentBuilder {
+pub struct ListOpenIdConnectProvidersFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_open_id_connect_providers::builders::ListOpenIdConnectProvidersInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -34,8 +34,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput,
-        super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProvidersError,
-    > for ListOpenIDConnectProvidersFluentBuilder
+        super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersError,
+    > for ListOpenIdConnectProvidersFluentBuilder
 {
     fn send(
         self,
@@ -43,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput,
-            super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProvidersError,
+            super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListOpenIDConnectProvidersFluentBuilder {
-    /// Creates a new `ListOpenIDConnectProvidersFluentBuilder`.
+impl ListOpenIdConnectProvidersFluentBuilder {
+    /// Creates a new `ListOpenIdConnectProvidersFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -58,7 +58,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListOpenIDConnectProviders as a reference.
+    /// Access the ListOpenIdConnectProviders as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_open_id_connect_providers::builders::ListOpenIdConnectProvidersInputBuilder {
         &self.inner
     }
@@ -75,7 +75,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProvidersError,
+            super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProviders::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProviders::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProviders::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProviders::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +96,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput,
-        super::super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProvidersError,
+        super::super::super::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_open_id_connect_providers.rs`

```diff
--- reference/src/operation/list_open_id_connect_providers.rs
+++ generated/src/operation/list_open_id_connect_providers.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListOpenIDConnectProviders`.
+/// Orchestration and serialization glue logic for `ListOpenIdConnectProviders`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListOpenIDConnectProviders;
-impl ListOpenIDConnectProviders {
-    /// Creates a new `ListOpenIDConnectProviders`
+pub struct ListOpenIdConnectProviders;
+impl ListOpenIdConnectProviders {
+    /// Creates a new `ListOpenIdConnectProviders`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListOpenIDConnectProviders {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListOpenIdConnectProviders {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListOpenIDConnectProviders");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListOpenIDConnectProvidersRequestSerializer,
+            ListOpenIdConnectProvidersRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListOpenIDConnectProvidersResponseDeserializer,
+            ListOpenIdConnectProvidersResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -130,7 +130,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListOpenIDConnectProvidersEndpointParamsInterceptor,
+                ListOpenIdConnectProvidersEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_open_id_connect_providers::ListOpenIDConnectProvidersError,
@@ -204,24 +204,20 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_open_id_connect_providers_input::ser_list_open_id_connect_providers_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
 #[derive(Debug)]
-struct ListOpenIDConnectProvidersEndpointParamsInterceptor;
+struct ListOpenIdConnectProvidersEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListOpenIDConnectProvidersEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListOpenIdConnectProvidersEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListOpenIDConnectProvidersEndpointParamsInterceptor"
+        "ListOpenIdConnectProvidersEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_organizations_features.rs`

```diff
--- reference/src/operation/list_organizations_features.rs
+++ generated/src/operation/list_organizations_features.rs
@@ -204,13 +204,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_organizations_features_input::ser_list_organizations_features_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/list_policies/_list_policies_input.rs`

```diff
--- reference/src/operation/list_policies/_list_policies_input.rs
+++ generated/src/operation/list_policies/_list_policies_input.rs
@@ -178,10 +178,11 @@
     /// Consumes the builder and constructs a [`ListPoliciesInput`](crate::operation::list_policies::ListPoliciesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_policies::ListPoliciesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::list_policies::ListPoliciesInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::list_policies::ListPoliciesInput {
             scope: self.scope,
-            only_attached: self.only_attached,
+            only_attached: self.only_attached.unwrap_or_default(),
             path_prefix: self.path_prefix,
             policy_usage_filter: self.policy_usage_filter,
             marker: self.marker,
```

### `src/operation/list_policies.rs`

```diff
--- reference/src/operation/list_policies.rs
+++ generated/src/operation/list_policies.rs
@@ -252,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_policies_input::ser_list_policies_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_policies_input::ser_list_policies_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
```

### `src/operation/list_policies_granting_service_access/builders.rs`

```diff
--- reference/src/operation/list_policies_granting_service_access/builders.rs
+++ generated/src/operation/list_policies_granting_service_access/builders.rs
@@ -70,7 +70,9 @@
         }
     }
     /// Access the ListPoliciesGrantingServiceAccess as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::list_policies_granting_service_access::builders::ListPoliciesGrantingServiceAccessInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::list_policies_granting_service_access::builders::ListPoliciesGrantingServiceAccessInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -94,12 +96,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_policies_granting_service_access::ListPoliciesGrantingServiceAccess::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::list_policies_granting_service_access::ListPoliciesGrantingServiceAccess::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::list_policies_granting_service_access::ListPoliciesGrantingServiceAccess::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::list_policies_granting_service_access::ListPoliciesGrantingServiceAccess::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/list_policies_granting_service_access.rs`

```diff
--- reference/src/operation/list_policies_granting_service_access.rs
+++ generated/src/operation/list_policies_granting_service_access.rs
@@ -265,12 +265,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_policies_granting_service_access_input::ser_list_policies_granting_service_access_input_input_input(
+            super::super::protocol_serde::shape_list_policies_granting_service_access_input::ser_list_policies_granting_service_access_op_input(
                 &input,
             )?,
         );
```

### `src/operation/list_policy_tags.rs`

```diff
--- reference/src/operation/list_policy_tags.rs
+++ generated/src/operation/list_policy_tags.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_policy_tags_input::ser_list_policy_tags_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_policy_tags_input::ser_list_policy_tags_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_policy_versions/_list_policy_versions_input.rs`

```diff
--- reference/src/operation/list_policy_versions/_list_policy_versions_input.rs
+++ generated/src/operation/list_policy_versions/_list_policy_versions_input.rs
@@ -96,8 +96,10 @@
     /// Consumes the builder and constructs a [`ListPolicyVersionsInput`](crate::operation::list_policy_versions::ListPolicyVersionsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_policy_versions::ListPolicyVersionsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_policy_versions::ListPolicyVersionsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_policy_versions::ListPolicyVersionsInput {
             policy_arn: self.policy_arn,
             marker: self.marker,
```

### `src/operation/list_policy_versions.rs`

```diff
--- reference/src/operation/list_policy_versions.rs
+++ generated/src/operation/list_policy_versions.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_policy_versions_input::ser_list_policy_versions_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_policy_versions_input::ser_list_policy_versions_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_role_policies/_list_role_policies_input.rs`

```diff
--- reference/src/operation/list_role_policies/_list_role_policies_input.rs
+++ generated/src/operation/list_role_policies/_list_role_policies_input.rs
@@ -96,7 +96,10 @@
     /// Consumes the builder and constructs a [`ListRolePoliciesInput`](crate::operation::list_role_policies::ListRolePoliciesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_role_policies::ListRolePoliciesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_role_policies::ListRolePoliciesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_role_policies::ListRolePoliciesInput {
             role_name: self.role_name,
             marker: self.marker,
```

### `src/operation/list_role_policies/_list_role_policies_output.rs`

```diff
--- reference/src/operation/list_role_policies/_list_role_policies_output.rs
+++ generated/src/operation/list_role_policies/_list_role_policies_output.rs
@@ -111,7 +111,10 @@
     /// - [`policy_names`](crate::operation::list_role_policies::builders::ListRolePoliciesOutputBuilder::policy_names)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_role_policies::ListRolePoliciesOutput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_role_policies::ListRolePoliciesOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_role_policies::ListRolePoliciesOutput {
             policy_names: self.policy_names.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_role_policies.rs`

```diff
--- reference/src/operation/list_role_policies.rs
+++ generated/src/operation/list_role_policies.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_role_policies_input::ser_list_role_policies_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_role_policies_input::ser_list_role_policies_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_role_tags/paginator.rs`

```diff
--- reference/src/operation/list_role_tags/paginator.rs
+++ generated/src/operation/list_role_tags/paginator.rs
@@ -139,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_role_tags_output_output_tags(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_role_tags_output_output_tags(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_role_tags.rs`

```diff
--- reference/src/operation/list_role_tags.rs
+++ generated/src/operation/list_role_tags.rs
@@ -252,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_role_tags_input::ser_list_role_tags_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_role_tags_input::ser_list_role_tags_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
```

### `src/operation/list_roles/_list_roles_input.rs`

```diff
--- reference/src/operation/list_roles/_list_roles_input.rs
+++ generated/src/operation/list_roles/_list_roles_input.rs
@@ -93,7 +93,9 @@
         &self.max_items
     }
     /// Consumes the builder and constructs a [`ListRolesInput`](crate::operation::list_roles::ListRolesInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_roles::ListRolesInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_roles::ListRolesInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_roles::ListRolesInput {
             path_prefix: self.path_prefix,
             marker: self.marker,
```

### `src/operation/list_roles/_list_roles_output.rs`

```diff
--- reference/src/operation/list_roles/_list_roles_output.rs
+++ generated/src/operation/list_roles/_list_roles_output.rs
@@ -109,7 +109,9 @@
     /// Consumes the builder and constructs a [`ListRolesOutput`](crate::operation::list_roles::ListRolesOutput).
     /// This method will fail if any of the following fields are not set:
     /// - [`roles`](crate::operation::list_roles::builders::ListRolesOutputBuilder::roles)
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_roles::ListRolesOutput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_roles::ListRolesOutput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_roles::ListRolesOutput {
             roles: self.roles.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_roles/builders.rs`

```diff
--- reference/src/operation/list_roles/builders.rs
+++ generated/src/operation/list_roles/builders.rs
@@ -41,14 +41,20 @@
     inner: super::super::super::operation::list_roles::builders::ListRolesInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::list_roles::ListRolesOutput, super::super::super::operation::list_roles::ListRolesError>
-    for ListRolesFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::list_roles::ListRolesOutput,
+        super::super::super::operation::list_roles::ListRolesError,
+    > for ListRolesFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::list_roles::ListRolesOutput, super::super::super::operation::list_roles::ListRolesError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::list_roles::ListRolesOutput,
+            super::super::super::operation::list_roles::ListRolesError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/list_roles/paginator.rs`

```diff
--- reference/src/operation/list_roles/paginator.rs
+++ generated/src/operation/list_roles/paginator.rs
@@ -8,7 +8,10 @@

 impl ListRolesPaginator {
     /// Create a new paginator-wrapper
-    pub(crate) fn new(handle: std::sync::Arc<super::super::super::client::Handle>, builder: super::super::super::operation::list_roles::builders::ListRolesInputBuilder) -> Self {
+    pub(crate) fn new(
+        handle: std::sync::Arc<super::super::super::client::Handle>,
+        builder: super::super::super::operation::list_roles::builders::ListRolesInputBuilder,
+    ) -> Self {
         Self {
             handle,
             builder,
@@ -136,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_roles_output_output_roles(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_roles_output_output_roles(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_roles.rs`

```diff
--- reference/src/operation/list_roles.rs
+++ generated/src/operation/list_roles.rs
@@ -225,7 +225,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::list_roles::ListRolesInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::list_roles::ListRolesInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -250,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_roles_input::ser_list_roles_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_roles_input::ser_list_roles_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_saml_provider_tags/_list_saml_provider_tags_input.rs`

```diff
--- reference/src/operation/list_saml_provider_tags/_list_saml_provider_tags_input.rs
+++ generated/src/operation/list_saml_provider_tags/_list_saml_provider_tags_input.rs
@@ -96,8 +96,10 @@
     /// Consumes the builder and constructs a [`ListSamlProviderTagsInput`](crate::operation::list_saml_provider_tags::ListSamlProviderTagsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsInput {
             saml_provider_arn: self.saml_provider_arn,
             marker: self.marker,
```

### `src/operation/list_saml_provider_tags/_list_saml_provider_tags_output.rs`

```diff
--- reference/src/operation/list_saml_provider_tags/_list_saml_provider_tags_output.rs
+++ generated/src/operation/list_saml_provider_tags/_list_saml_provider_tags_output.rs
@@ -110,8 +110,10 @@
     /// - [`tags`](crate::operation::list_saml_provider_tags::builders::ListSamlProviderTagsOutputBuilder::tags)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput {
             tags: self.tags.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_saml_provider_tags/builders.rs`

```diff
--- reference/src/operation/list_saml_provider_tags/builders.rs
+++ generated/src/operation/list_saml_provider_tags/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTagsError,
+            super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListSAMLProviderTags`.
+/// Fluent builder constructing a request to `ListSamlProviderTags`.
 ///
 /// <p>Lists the tags that are attached to the specified Security Assertion Markup Language (SAML) identity provider. The returned list of tags is sorted by tag key. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_saml.html">About SAML 2.0-based federation</a>.</p>
 /// <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListSAMLProviderTagsFluentBuilder {
+pub struct ListSamlProviderTagsFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_saml_provider_tags::builders::ListSamlProviderTagsInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput,
-        super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTagsError,
-    > for ListSAMLProviderTagsFluentBuilder
+        super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsError,
+    > for ListSamlProviderTagsFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput,
-            super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTagsError,
+            super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListSAMLProviderTagsFluentBuilder {
-    /// Creates a new `ListSAMLProviderTagsFluentBuilder`.
+impl ListSamlProviderTagsFluentBuilder {
+    /// Creates a new `ListSamlProviderTagsFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListSAMLProviderTags as a reference.
+    /// Access the ListSamlProviderTags as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_saml_provider_tags::builders::ListSamlProviderTagsInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTagsError,
+            super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTags::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_saml_provider_tags::ListSamlProviderTags::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTags::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_saml_provider_tags::ListSamlProviderTags::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsOutput,
-        super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTagsError,
+        super::super::super::operation::list_saml_provider_tags::ListSamlProviderTagsError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_saml_provider_tags/paginator.rs`

```diff
--- reference/src/operation/list_saml_provider_tags/paginator.rs
+++ generated/src/operation/list_saml_provider_tags/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTags::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_saml_provider_tags::ListSAMLProviderTags::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_saml_provider_tags.rs`

```diff
--- reference/src/operation/list_saml_provider_tags.rs
+++ generated/src/operation/list_saml_provider_tags.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListSAMLProviderTags`.
+/// Orchestration and serialization glue logic for `ListSamlProviderTags`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListSAMLProviderTags;
-impl ListSAMLProviderTags {
-    /// Creates a new `ListSAMLProviderTags`
+pub struct ListSamlProviderTags;
+impl ListSamlProviderTags {
+    /// Creates a new `ListSamlProviderTags`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSAMLProviderTags {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSamlProviderTags {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListSAMLProviderTags");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListSAMLProviderTagsRequestSerializer,
+            ListSamlProviderTagsRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListSAMLProviderTagsResponseDeserializer,
+            ListSamlProviderTagsResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListSAMLProviderTags")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSAMLProviderTagsTelemetryInputCaptureInterceptor,
+                ListSamlProviderTagsTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSAMLProviderTagsEndpointParamsInterceptor,
+                ListSamlProviderTagsEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_saml_provider_tags::ListSAMLProviderTagsError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct ListSAMLProviderTagsTelemetryInputCaptureInterceptor;
+struct ListSamlProviderTagsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSAMLProviderTagsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSamlProviderTagsTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListSAMLProviderTagsTelemetryInputCaptureInterceptor"
+        "ListSamlProviderTagsTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_list_saml_provider_tags_input::ser_list_saml_provider_tags_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_saml_provider_tags_input::ser_list_saml_provider_tags_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -270,12 +269,12 @@
     }
 }
 #[derive(Debug)]
-struct ListSAMLProviderTagsEndpointParamsInterceptor;
+struct ListSamlProviderTagsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSAMLProviderTagsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSamlProviderTagsEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListSAMLProviderTagsEndpointParamsInterceptor"
+        "ListSamlProviderTagsEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_saml_providers/_list_saml_providers_input.rs`

```diff
--- reference/src/operation/list_saml_providers/_list_saml_providers_input.rs
+++ generated/src/operation/list_saml_providers/_list_saml_providers_input.rs
@@ -18,7 +18,10 @@
     /// Consumes the builder and constructs a [`ListSamlProvidersInput`](crate::operation::list_saml_providers::ListSamlProvidersInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_saml_providers::ListSamlProvidersInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_saml_providers::ListSamlProvidersInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_saml_providers::ListSamlProvidersInput {})
     }
 }
```

### `src/operation/list_saml_providers/_list_saml_providers_output.rs`

```diff
--- reference/src/operation/list_saml_providers/_list_saml_providers_output.rs
+++ generated/src/operation/list_saml_providers/_list_saml_providers_output.rs
@@ -48,7 +48,10 @@
         self
     }
     /// <p>The list of SAML provider resource objects defined in IAM for this Amazon Web Services account.</p>
-    pub fn set_saml_provider_list(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::SamlProviderListEntry>>) -> Self {
+    pub fn set_saml_provider_list(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::SamlProviderListEntry>>,
+    ) -> Self {
         self.saml_provider_list = input;
         self
     }
```

### `src/operation/list_saml_providers/builders.rs`

```diff
--- reference/src/operation/list_saml_providers/builders.rs
+++ generated/src/operation/list_saml_providers/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_saml_providers::ListSamlProvidersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_saml_providers::ListSAMLProvidersError,
+            super::super::super::operation::list_saml_providers::ListSamlProvidersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,13 +20,13 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListSAMLProviders`.
+/// Fluent builder constructing a request to `ListSamlProviders`.
 ///
 /// <p>Lists the SAML provider resource objects defined in IAM in the account. IAM resource-listing operations return a subset of the available attributes for the resource. For example, this operation does not return tags, even though they are an attribute of the returned object. To view all of the information for a SAML provider, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetSAMLProvider.html">GetSAMLProvider</a>.</p><important>
 /// <p>This operation requires <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4</a>.</p>
 /// </important>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListSAMLProvidersFluentBuilder {
+pub struct ListSamlProvidersFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_saml_providers::builders::ListSamlProvidersInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -34,8 +34,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_saml_providers::ListSamlProvidersOutput,
-        super::super::super::operation::list_saml_providers::ListSAMLProvidersError,
-    > for ListSAMLProvidersFluentBuilder
+        super::super::super::operation::list_saml_providers::ListSamlProvidersError,
+    > for ListSamlProvidersFluentBuilder
 {
     fn send(
         self,
@@ -43,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_saml_providers::ListSamlProvidersOutput,
-            super::super::super::operation::list_saml_providers::ListSAMLProvidersError,
+            super::super::super::operation::list_saml_providers::ListSamlProvidersError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListSAMLProvidersFluentBuilder {
-    /// Creates a new `ListSAMLProvidersFluentBuilder`.
+impl ListSamlProvidersFluentBuilder {
+    /// Creates a new `ListSamlProvidersFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -58,7 +58,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListSAMLProviders as a reference.
+    /// Access the ListSamlProviders as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_saml_providers::builders::ListSamlProvidersInputBuilder {
         &self.inner
     }
@@ -75,7 +75,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_saml_providers::ListSamlProvidersOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_saml_providers::ListSAMLProvidersError,
+            super::super::super::operation::list_saml_providers::ListSamlProvidersError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_saml_providers::ListSAMLProviders::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_saml_providers::ListSamlProviders::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_saml_providers::ListSAMLProviders::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_saml_providers::ListSamlProviders::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +96,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_saml_providers::ListSamlProvidersOutput,
-        super::super::super::operation::list_saml_providers::ListSAMLProvidersError,
+        super::super::super::operation::list_saml_providers::ListSamlProvidersError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_saml_providers.rs`

```diff
--- reference/src/operation/list_saml_providers.rs
+++ generated/src/operation/list_saml_providers.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListSAMLProviders`.
+/// Orchestration and serialization glue logic for `ListSamlProviders`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListSAMLProviders;
-impl ListSAMLProviders {
-    /// Creates a new `ListSAMLProviders`
+pub struct ListSamlProviders;
+impl ListSamlProviders {
+    /// Creates a new `ListSamlProviders`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSAMLProviders {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSamlProviders {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListSAMLProviders");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListSAMLProvidersRequestSerializer,
+            ListSamlProvidersRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListSAMLProvidersResponseDeserializer,
+            ListSamlProvidersResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,7 +127,7 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSAMLProvidersEndpointParamsInterceptor,
+                ListSamlProvidersEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_saml_providers::ListSAMLProvidersError,
@@ -201,24 +201,20 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_saml_providers_input::ser_list_saml_providers_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
 #[derive(Debug)]
-struct ListSAMLProvidersEndpointParamsInterceptor;
+struct ListSamlProvidersEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSAMLProvidersEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSamlProvidersEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListSAMLProvidersEndpointParamsInterceptor"
+        "ListSamlProvidersEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_server_certificate_tags/_list_server_certificate_tags_input.rs`

```diff
--- reference/src/operation/list_server_certificate_tags/_list_server_certificate_tags_input.rs
+++ generated/src/operation/list_server_certificate_tags/_list_server_certificate_tags_input.rs
@@ -100,10 +100,12 @@
         super::super::super::operation::list_server_certificate_tags::ListServerCertificateTagsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_server_certificate_tags::ListServerCertificateTagsInput {
-            server_certificate_name: self.server_certificate_name,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_server_certificate_tags::ListServerCertificateTagsInput {
+                server_certificate_name: self.server_certificate_name,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_server_certificate_tags/_list_server_certificate_tags_output.rs`

```diff
--- reference/src/operation/list_server_certificate_tags/_list_server_certificate_tags_output.rs
+++ generated/src/operation/list_server_certificate_tags/_list_server_certificate_tags_output.rs
@@ -114,16 +114,18 @@
         super::super::super::operation::list_server_certificate_tags::ListServerCertificateTagsOutput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_server_certificate_tags::ListServerCertificateTagsOutput {
-            tags: self.tags.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "tags",
-                    "tags was not specified but it is required when building ListServerCertificateTagsOutput",
-                )
-            })?,
-            is_truncated: self.is_truncated.unwrap_or_default(),
-            marker: self.marker,
-            _request_id: self._request_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_server_certificate_tags::ListServerCertificateTagsOutput {
+                tags: self.tags.ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
+                        "tags",
+                        "tags was not specified but it is required when building ListServerCertificateTagsOutput",
+                    )
+                })?,
+                is_truncated: self.is_truncated.unwrap_or_default(),
+                marker: self.marker,
+                _request_id: self._request_id,
+            },
+        )
     }
 }
```

### `src/operation/list_server_certificate_tags/paginator.rs`

```diff
--- reference/src/operation/list_server_certificate_tags/paginator.rs
+++ generated/src/operation/list_server_certificate_tags/paginator.rs
@@ -86,9 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_server_certificate_tags::ListServerCertificateTags::orchestrate(&runtime_plugins, input.clone())
-                                .await;
+                        let resp = super::super::super::operation::list_server_certificate_tags::ListServerCertificateTags::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_server_certificate_tags.rs`

```diff
--- reference/src/operation/list_server_certificate_tags.rs
+++ generated/src/operation/list_server_certificate_tags.rs
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
-            super::super::protocol_serde::shape_list_server_certificate_tags_input::ser_list_server_certificate_tags_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_server_certificate_tags_input::ser_list_server_certificate_tags_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_server_certificates/_list_server_certificates_output.rs`

```diff
--- reference/src/operation/list_server_certificates/_list_server_certificates_output.rs
+++ generated/src/operation/list_server_certificates/_list_server_certificates_output.rs
@@ -69,7 +69,9 @@
         self
     }
     /// <p>A list of server certificates.</p>
-    pub fn get_server_certificate_metadata_list(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::ServerCertificateMetadata>> {
+    pub fn get_server_certificate_metadata_list(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::ServerCertificateMetadata>> {
         &self.server_certificate_metadata_list
     }
     /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
```

### `src/operation/list_server_certificates/paginator.rs`

```diff
--- reference/src/operation/list_server_certificates/paginator.rs
+++ generated/src/operation/list_server_certificates/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_server_certificates::ListServerCertificates::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_server_certificates::ListServerCertificates::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_server_certificates.rs`

```diff
--- reference/src/operation/list_server_certificates.rs
+++ generated/src/operation/list_server_certificates.rs
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
-            super::super::protocol_serde::shape_list_server_certificates_input::ser_list_server_certificates_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_server_certificates_input::ser_list_server_certificates_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_service_specific_credentials/_list_service_specific_credentials_input.rs`

```diff
--- reference/src/operation/list_service_specific_credentials/_list_service_specific_credentials_input.rs
+++ generated/src/operation/list_service_specific_credentials/_list_service_specific_credentials_input.rs
@@ -136,12 +136,14 @@
         super::super::super::operation::list_service_specific_credentials::ListServiceSpecificCredentialsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::list_service_specific_credentials::ListServiceSpecificCredentialsInput {
-            user_name: self.user_name,
-            service_name: self.service_name,
-            all_users: self.all_users,
-            marker: self.marker,
-            max_items: self.max_items,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::list_service_specific_credentials::ListServiceSpecificCredentialsInput {
+                user_name: self.user_name,
+                service_name: self.service_name,
+                all_users: self.all_users,
+                marker: self.marker,
+                max_items: self.max_items,
+            },
+        )
     }
 }
```

### `src/operation/list_service_specific_credentials/_list_service_specific_credentials_output.rs`

```diff
--- reference/src/operation/list_service_specific_credentials/_list_service_specific_credentials_output.rs
+++ generated/src/operation/list_service_specific_credentials/_list_service_specific_credentials_output.rs
@@ -69,7 +69,9 @@
         self
     }
     /// <p>A list of structures that each contain details about a service-specific credential.</p>
-    pub fn get_service_specific_credentials(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::ServiceSpecificCredentialMetadata>> {
+    pub fn get_service_specific_credentials(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::ServiceSpecificCredentialMetadata>> {
         &self.service_specific_credentials
     }
     /// <p>When IsTruncated is true, this element is present and contains the value to use for the Marker parameter in a subsequent pagination request.</p>
```

### `src/operation/list_service_specific_credentials/builders.rs`

```diff
--- reference/src/operation/list_service_specific_credentials/builders.rs
+++ generated/src/operation/list_service_specific_credentials/builders.rs
@@ -65,7 +65,9 @@
         }
     }
     /// Access the ListServiceSpecificCredentials as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::list_service_specific_credentials::builders::ListServiceSpecificCredentialsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::list_service_specific_credentials::builders::ListServiceSpecificCredentialsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -89,11 +91,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_service_specific_credentials::ListServiceSpecificCredentials::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::list_service_specific_credentials::ListServiceSpecificCredentials::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::list_service_specific_credentials::ListServiceSpecificCredentials::orchestrate(&runtime_plugins, input).await
     }

```

### `src/operation/list_service_specific_credentials.rs`

```diff
--- reference/src/operation/list_service_specific_credentials.rs
+++ generated/src/operation/list_service_specific_credentials.rs
@@ -223,9 +223,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_list_service_specific_credentials::de_list_service_specific_credentials_http_error(status, headers, body)
+            super::super::protocol_serde::shape_list_service_specific_credentials::de_list_service_specific_credentials_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_list_service_specific_credentials::de_list_service_specific_credentials_http_response(status, headers, body)
+            super::super::protocol_serde::shape_list_service_specific_credentials::de_list_service_specific_credentials_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -266,12 +270,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_service_specific_credentials_input::ser_list_service_specific_credentials_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_service_specific_credentials_input::ser_list_service_specific_credentials_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_signing_certificates/paginator.rs`

```diff
--- reference/src/operation/list_signing_certificates/paginator.rs
+++ generated/src/operation/list_signing_certificates/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_signing_certificates::ListSigningCertificates::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_signing_certificates::ListSigningCertificates::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_signing_certificates.rs`

```diff
--- reference/src/operation/list_signing_certificates.rs
+++ generated/src/operation/list_signing_certificates.rs
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
-            super::super::protocol_serde::shape_list_signing_certificates_input::ser_list_signing_certificates_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_signing_certificates_input::ser_list_signing_certificates_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_ssh_public_keys/_list_ssh_public_keys_input.rs`

```diff
--- reference/src/operation/list_ssh_public_keys/_list_ssh_public_keys_input.rs
+++ generated/src/operation/list_ssh_public_keys/_list_ssh_public_keys_input.rs
@@ -95,7 +95,10 @@
     /// Consumes the builder and constructs a [`ListSshPublicKeysInput`](crate::operation::list_ssh_public_keys::ListSshPublicKeysInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysInput {
             user_name: self.user_name,
             marker: self.marker,
```

### `src/operation/list_ssh_public_keys/builders.rs`

```diff
--- reference/src/operation/list_ssh_public_keys/builders.rs
+++ generated/src/operation/list_ssh_public_keys/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeysError,
+            super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,13 +20,13 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListSSHPublicKeys`.
+/// Fluent builder constructing a request to `ListSshPublicKeys`.
 ///
 /// <p>Returns information about the SSH public keys associated with the specified IAM user. If none exists, the operation returns an empty list.</p>
 /// <p>The SSH public keys returned by this operation are used only for authenticating the IAM user to an CodeCommit repository. For more information about using SSH keys to authenticate to an CodeCommit repository, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/setting-up-credentials-ssh.html">Set up CodeCommit for SSH connections</a> in the <i>CodeCommit User Guide</i>.</p>
 /// <p>Although each user is limited to a small number of keys, you can still paginate the results using the <code>MaxItems</code> and <code>Marker</code> parameters.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListSSHPublicKeysFluentBuilder {
+pub struct ListSshPublicKeysFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_ssh_public_keys::builders::ListSshPublicKeysInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -34,8 +34,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysOutput,
-        super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeysError,
-    > for ListSSHPublicKeysFluentBuilder
+        super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysError,
+    > for ListSshPublicKeysFluentBuilder
 {
     fn send(
         self,
@@ -43,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysOutput,
-            super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeysError,
+            super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListSSHPublicKeysFluentBuilder {
-    /// Creates a new `ListSSHPublicKeysFluentBuilder`.
+impl ListSshPublicKeysFluentBuilder {
+    /// Creates a new `ListSshPublicKeysFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -58,7 +58,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListSSHPublicKeys as a reference.
+    /// Access the ListSshPublicKeys as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_ssh_public_keys::builders::ListSshPublicKeysInputBuilder {
         &self.inner
     }
@@ -75,7 +75,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeysError,
+            super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -83,12 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeys::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_ssh_public_keys::ListSshPublicKeys::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeys::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_ssh_public_keys::ListSshPublicKeys::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -96,7 +96,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysOutput,
-        super::super::super::operation::list_ssh_public_keys::ListSSHPublicKeysError,
+        super::super::super::operation::list_ssh_public_keys::ListSshPublicKeysError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_ssh_public_keys.rs`

```diff
--- reference/src/operation/list_ssh_public_keys.rs
+++ generated/src/operation/list_ssh_public_keys.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListSSHPublicKeys`.
+/// Orchestration and serialization glue logic for `ListSshPublicKeys`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListSSHPublicKeys;
-impl ListSSHPublicKeys {
-    /// Creates a new `ListSSHPublicKeys`
+pub struct ListSshPublicKeys;
+impl ListSshPublicKeys {
+    /// Creates a new `ListSshPublicKeys`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSSHPublicKeys {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSshPublicKeys {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListSSHPublicKeys");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListSSHPublicKeysRequestSerializer,
+            ListSshPublicKeysRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListSSHPublicKeysResponseDeserializer,
+            ListSshPublicKeysResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListSSHPublicKeys")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSSHPublicKeysTelemetryInputCaptureInterceptor,
+                ListSshPublicKeysTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListSSHPublicKeysEndpointParamsInterceptor,
+                ListSshPublicKeysEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_ssh_public_keys::ListSSHPublicKeysError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct ListSSHPublicKeysTelemetryInputCaptureInterceptor;
+struct ListSshPublicKeysTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSSHPublicKeysTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSshPublicKeysTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListSSHPublicKeysTelemetryInputCaptureInterceptor"
+        "ListSshPublicKeysTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_ssh_public_keys_input::ser_list_ssh_public_keys_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_ssh_public_keys_input::ser_list_ssh_public_keys_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct ListSSHPublicKeysEndpointParamsInterceptor;
+struct ListSshPublicKeysEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSSHPublicKeysEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListSshPublicKeysEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListSSHPublicKeysEndpointParamsInterceptor"
+        "ListSshPublicKeysEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/list_user_policies/_list_user_policies_input.rs`

```diff
--- reference/src/operation/list_user_policies/_list_user_policies_input.rs
+++ generated/src/operation/list_user_policies/_list_user_policies_input.rs
@@ -96,7 +96,10 @@
     /// Consumes the builder and constructs a [`ListUserPoliciesInput`](crate::operation::list_user_policies::ListUserPoliciesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_user_policies::ListUserPoliciesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_user_policies::ListUserPoliciesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_user_policies::ListUserPoliciesInput {
             user_name: self.user_name,
             marker: self.marker,
```

### `src/operation/list_user_policies/_list_user_policies_output.rs`

```diff
--- reference/src/operation/list_user_policies/_list_user_policies_output.rs
+++ generated/src/operation/list_user_policies/_list_user_policies_output.rs
@@ -111,7 +111,10 @@
     /// - [`policy_names`](crate::operation::list_user_policies::builders::ListUserPoliciesOutputBuilder::policy_names)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_user_policies::ListUserPoliciesOutput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_user_policies::ListUserPoliciesOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_user_policies::ListUserPoliciesOutput {
             policy_names: self.policy_names.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_user_policies.rs`

```diff
--- reference/src/operation/list_user_policies.rs
+++ generated/src/operation/list_user_policies.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_list_user_policies_input::ser_list_user_policies_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_user_policies_input::ser_list_user_policies_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/list_user_tags/paginator.rs`

```diff
--- reference/src/operation/list_user_tags/paginator.rs
+++ generated/src/operation/list_user_tags/paginator.rs
@@ -139,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_user_tags_output_output_tags(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_user_tags_output_output_tags(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_user_tags.rs`

```diff
--- reference/src/operation/list_user_tags.rs
+++ generated/src/operation/list_user_tags.rs
@@ -252,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_user_tags_input::ser_list_user_tags_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_user_tags_input::ser_list_user_tags_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
```

### `src/operation/list_users/_list_users_input.rs`

```diff
--- reference/src/operation/list_users/_list_users_input.rs
+++ generated/src/operation/list_users/_list_users_input.rs
@@ -93,7 +93,9 @@
         &self.max_items
     }
     /// Consumes the builder and constructs a [`ListUsersInput`](crate::operation::list_users::ListUsersInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_users::ListUsersInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_users::ListUsersInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_users::ListUsersInput {
             path_prefix: self.path_prefix,
             marker: self.marker,
```

### `src/operation/list_users/_list_users_output.rs`

```diff
--- reference/src/operation/list_users/_list_users_output.rs
+++ generated/src/operation/list_users/_list_users_output.rs
@@ -109,7 +109,9 @@
     /// Consumes the builder and constructs a [`ListUsersOutput`](crate::operation::list_users::ListUsersOutput).
     /// This method will fail if any of the following fields are not set:
     /// - [`users`](crate::operation::list_users::builders::ListUsersOutputBuilder::users)
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_users::ListUsersOutput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_users::ListUsersOutput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_users::ListUsersOutput {
             users: self.users.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/list_users/builders.rs`

```diff
--- reference/src/operation/list_users/builders.rs
+++ generated/src/operation/list_users/builders.rs
@@ -39,14 +39,20 @@
     inner: super::super::super::operation::list_users::builders::ListUsersInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::list_users::ListUsersOutput, super::super::super::operation::list_users::ListUsersError>
-    for ListUsersFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::list_users::ListUsersOutput,
+        super::super::super::operation::list_users::ListUsersError,
+    > for ListUsersFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::list_users::ListUsersOutput, super::super::super::operation::list_users::ListUsersError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::list_users::ListUsersOutput,
+            super::super::super::operation::list_users::ListUsersError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/list_users/paginator.rs`

```diff
--- reference/src/operation/list_users/paginator.rs
+++ generated/src/operation/list_users/paginator.rs
@@ -8,7 +8,10 @@

 impl ListUsersPaginator {
     /// Create a new paginator-wrapper
-    pub(crate) fn new(handle: std::sync::Arc<super::super::super::client::Handle>, builder: super::super::super::operation::list_users::builders::ListUsersInputBuilder) -> Self {
+    pub(crate) fn new(
+        handle: std::sync::Arc<super::super::super::client::Handle>,
+        builder: super::super::super::operation::list_users::builders::ListUsersInputBuilder,
+    ) -> Self {
         Self {
             handle,
             builder,
@@ -136,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_users_output_output_users(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_users_output_output_users(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_users.rs`

```diff
--- reference/src/operation/list_users.rs
+++ generated/src/operation/list_users.rs
@@ -225,7 +225,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::list_users::ListUsersInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::list_users::ListUsersInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -250,11 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_users_input::ser_list_users_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_users_input::ser_list_users_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_virtual_mfa_devices/_list_virtual_mfa_devices_input.rs`

```diff
--- reference/src/operation/list_virtual_mfa_devices/_list_virtual_mfa_devices_input.rs
+++ generated/src/operation/list_virtual_mfa_devices/_list_virtual_mfa_devices_input.rs
@@ -90,8 +90,10 @@
     /// Consumes the builder and constructs a [`ListVirtualMfaDevicesInput`](crate::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesInput {
             assignment_status: self.assignment_status,
             marker: self.marker,
```

### `src/operation/list_virtual_mfa_devices/builders.rs`

```diff
--- reference/src/operation/list_virtual_mfa_devices/builders.rs
+++ generated/src/operation/list_virtual_mfa_devices/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevicesError,
+            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ListVirtualMFADevices`.
+/// Fluent builder constructing a request to `ListVirtualMfaDevices`.
 ///
 /// <p>Lists the virtual MFA devices defined in the Amazon Web Services account by assignment status. If you do not specify an assignment status, the operation returns a list of all virtual MFA devices. Assignment status can be <code>Assigned</code>, <code>Unassigned</code>, or <code>Any</code>.</p><note>
 /// <p>IAM resource-listing operations return a subset of the available attributes for the resource. For example, this operation does not return tags, even though they are an attribute of the returned object. To view tag information for a virtual MFA device, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_ListMFADeviceTags.html">ListMFADeviceTags</a>.</p>
@@ -27,7 +27,7 @@
 /// </note>
 /// <p>You can paginate the results using the <code>MaxItems</code> and <code>Marker</code> parameters.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ListVirtualMFADevicesFluentBuilder {
+pub struct ListVirtualMfaDevicesFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::list_virtual_mfa_devices::builders::ListVirtualMfaDevicesInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -35,8 +35,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput,
-        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevicesError,
-    > for ListVirtualMFADevicesFluentBuilder
+        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesError,
+    > for ListVirtualMfaDevicesFluentBuilder
 {
     fn send(
         self,
@@ -44,14 +44,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput,
-            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevicesError,
+            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ListVirtualMFADevicesFluentBuilder {
-    /// Creates a new `ListVirtualMFADevicesFluentBuilder`.
+impl ListVirtualMfaDevicesFluentBuilder {
+    /// Creates a new `ListVirtualMfaDevicesFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -59,7 +59,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ListVirtualMFADevices as a reference.
+    /// Access the ListVirtualMfaDevices as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::list_virtual_mfa_devices::builders::ListVirtualMfaDevicesInputBuilder {
         &self.inner
     }
@@ -76,7 +76,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevicesError,
+            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,12 +84,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevices::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevices::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevices::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevices::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -97,7 +97,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput,
-        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevicesError,
+        super::super::super::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/list_virtual_mfa_devices/paginator.rs`

```diff
--- reference/src/operation/list_virtual_mfa_devices/paginator.rs
+++ generated/src/operation/list_virtual_mfa_devices/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevices::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevices::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_virtual_mfa_devices.rs`

```diff
--- reference/src/operation/list_virtual_mfa_devices.rs
+++ generated/src/operation/list_virtual_mfa_devices.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ListVirtualMFADevices`.
+/// Orchestration and serialization glue logic for `ListVirtualMfaDevices`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ListVirtualMFADevices;
-impl ListVirtualMFADevices {
-    /// Creates a new `ListVirtualMFADevices`
+pub struct ListVirtualMfaDevices;
+impl ListVirtualMfaDevices {
+    /// Creates a new `ListVirtualMfaDevices`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListVirtualMFADevices {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListVirtualMfaDevices {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListVirtualMFADevices");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListVirtualMFADevicesRequestSerializer,
+            ListVirtualMfaDevicesRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListVirtualMFADevicesResponseDeserializer,
+            ListVirtualMfaDevicesResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -128,13 +128,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListVirtualMFADevices")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListVirtualMFADevicesTelemetryInputCaptureInterceptor,
+                ListVirtualMfaDevicesTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListVirtualMFADevicesEndpointParamsInterceptor,
+                ListVirtualMfaDevicesEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::list_virtual_mfa_devices::ListVirtualMFADevicesError,
@@ -151,12 +151,12 @@
 }

 #[derive(Debug)]
-struct ListVirtualMFADevicesTelemetryInputCaptureInterceptor;
+struct ListVirtualMfaDevicesTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListVirtualMFADevicesTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListVirtualMfaDevicesTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ListVirtualMFADevicesTelemetryInputCaptureInterceptor"
+        "ListVirtualMfaDevicesTelemetryInputCaptureInterceptor"
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
-            super::super::protocol_serde::shape_list_virtual_mfa_devices_input::ser_list_virtual_mfa_devices_input_input_input(&input)?,
+            super::super::protocol_serde::shape_list_virtual_mfa_devices_input::ser_list_virtual_mfa_devices_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -266,12 +265,12 @@
     }
 }
 #[derive(Debug)]
-struct ListVirtualMFADevicesEndpointParamsInterceptor;
+struct ListVirtualMfaDevicesEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListVirtualMFADevicesEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListVirtualMfaDevicesEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ListVirtualMFADevicesEndpointParamsInterceptor"
+        "ListVirtualMfaDevicesEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/put_account_properties/_put_account_properties_input.rs`

```diff
--- reference/src/operation/put_account_properties/_put_account_properties_input.rs
+++ generated/src/operation/put_account_properties/_put_account_properties_input.rs
@@ -59,8 +59,10 @@
     /// Consumes the builder and constructs a [`PutAccountPropertiesInput`](crate::operation::put_account_properties::PutAccountPropertiesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::put_account_properties::PutAccountPropertiesInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::put_account_properties::PutAccountPropertiesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::put_account_properties::PutAccountPropertiesInput { properties: self.properties })
     }
 }
```

### `src/operation/put_account_properties.rs`

```diff
--- reference/src/operation/put_account_properties.rs
+++ generated/src/operation/put_account_properties.rs
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
-            super::super::protocol_serde::shape_put_account_properties_input::ser_put_account_properties_input_input_input(&input)?,
+            super::super::protocol_serde::shape_put_account_properties_input::ser_put_account_properties_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/put_group_policy.rs`

```diff
--- reference/src/operation/put_group_policy.rs
+++ generated/src/operation/put_group_policy.rs
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
-            super::super::protocol_serde::shape_put_group_policy_input::ser_put_group_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_put_group_policy_input::ser_put_group_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/put_role_permissions_boundary/_put_role_permissions_boundary_input.rs`

```diff
--- reference/src/operation/put_role_permissions_boundary/_put_role_permissions_boundary_input.rs
+++ generated/src/operation/put_role_permissions_boundary/_put_role_permissions_boundary_input.rs
@@ -80,9 +80,11 @@
         super::super::super::operation::put_role_permissions_boundary::PutRolePermissionsBoundaryInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::put_role_permissions_boundary::PutRolePermissionsBoundaryInput {
-            role_name: self.role_name,
-            permissions_boundary: self.permissions_boundary,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::put_role_permissions_boundary::PutRolePermissionsBoundaryInput {
+                role_name: self.role_name,
+                permissions_boundary: self.permissions_boundary,
+            },
+        )
     }
 }
```

### `src/operation/put_role_permissions_boundary.rs`

```diff
--- reference/src/operation/put_role_permissions_boundary.rs
+++ generated/src/operation/put_role_permissions_boundary.rs
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
-            super::super::protocol_serde::shape_put_role_permissions_boundary_input::ser_put_role_permissions_boundary_input_input_input(&input)?,
+            super::super::protocol_serde::shape_put_role_permissions_boundary_input::ser_put_role_permissions_boundary_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/put_role_policy.rs`

```diff
--- reference/src/operation/put_role_policy.rs
+++ generated/src/operation/put_role_policy.rs
@@ -257,13 +257,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_put_role_policy_input::ser_put_role_policy_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_put_role_policy_input::ser_put_role_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/put_user_permissions_boundary/_put_user_permissions_boundary_input.rs`

```diff
--- reference/src/operation/put_user_permissions_boundary/_put_user_permissions_boundary_input.rs
+++ generated/src/operation/put_user_permissions_boundary/_put_user_permissions_boundary_input.rs
@@ -80,9 +80,11 @@
         super::super::super::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput {
-            user_name: self.user_name,
-            permissions_boundary: self.permissions_boundary,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput {
+                user_name: self.user_name,
+                permissions_boundary: self.permissions_boundary,
+            },
+        )
     }
 }
```

### `src/operation/put_user_permissions_boundary.rs`

```diff
--- reference/src/operation/put_user_permissions_boundary.rs
+++ generated/src/operation/put_user_permissions_boundary.rs
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
-            super::super::protocol_serde::shape_put_user_permissions_boundary_input::ser_put_user_permissions_boundary_input_input_input(&input)?,
+            super::super::protocol_serde::shape_put_user_permissions_boundary_input::ser_put_user_permissions_boundary_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/put_user_policy.rs`

```diff
--- reference/src/operation/put_user_policy.rs
+++ generated/src/operation/put_user_policy.rs
@@ -257,13 +257,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_put_user_policy_input::ser_put_user_policy_input_input_input(&input)?,
-        );
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_put_user_policy_input::ser_put_user_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/reject_delegation_request.rs`

```diff
--- reference/src/operation/reject_delegation_request.rs
+++ generated/src/operation/reject_delegation_request.rs
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
-            super::super::protocol_serde::shape_reject_delegation_request_input::ser_reject_delegation_request_input_input_input(&input)?,
+            super::super::protocol_serde::shape_reject_delegation_request_input::ser_reject_delegation_request_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/remove_client_id_from_open_id_connect_provider/_remove_client_id_from_open_id_connect_provider_output.rs`

```diff
--- reference/src/operation/remove_client_id_from_open_id_connect_provider/_remove_client_id_from_open_id_connect_provider_output.rs
+++ generated/src/operation/remove_client_id_from_open_id_connect_provider/_remove_client_id_from_open_id_connect_provider_output.rs
@@ -13,7 +13,8 @@
 impl RemoveClientIdFromOpenIdConnectProviderOutput {
     /// Creates a new builder-style object to manufacture [`RemoveClientIdFromOpenIdConnectProviderOutput`](crate::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput).
     pub fn builder(
-    ) -> super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderOutputBuilder {
+    ) -> super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderOutputBuilder
+    {
         super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderOutputBuilder::default()
     }
 }
@@ -35,7 +36,9 @@
         self
     }
     /// Consumes the builder and constructs a [`RemoveClientIdFromOpenIdConnectProviderOutput`](crate::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput).
-    pub fn build(self) -> super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput {
+    pub fn build(
+        self,
+    ) -> super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput {
         super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput {
             _request_id: self._request_id,
         }
```

### `src/operation/remove_client_id_from_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/remove_client_id_from_open_id_connect_provider/builders.rs
+++ generated/src/operation/remove_client_id_from_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError,
+            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,21 +20,22 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `RemoveClientIDFromOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `RemoveClientIdFromOpenIdConnectProvider`.
 ///
 /// <p>Removes the specified client ID (also known as audience) from the list of client IDs registered for the specified IAM OpenID Connect (OIDC) provider resource object.</p>
 /// <p>This operation is idempotent; it does not fail or return an error if you try to remove a client ID that does not exist.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct RemoveClientIDFromOpenIDConnectProviderFluentBuilder {
+pub struct RemoveClientIdFromOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
-    inner: super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderInputBuilder,
+    inner:
+        super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput,
-        super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError,
-    > for RemoveClientIDFromOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderError,
+    > for RemoveClientIdFromOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +43,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput,
-            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError,
+            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl RemoveClientIDFromOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `RemoveClientIDFromOpenIDConnectProviderFluentBuilder`.
+impl RemoveClientIdFromOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `RemoveClientIdFromOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,10 +58,11 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the RemoveClientIDFromOpenIDConnectProvider as a reference.
+    /// Access the RemoveClientIdFromOpenIdConnectProvider as a reference.
     pub fn as_input(
         &self,
-    ) -> &super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderInputBuilder {
+    ) -> &super::super::super::operation::remove_client_id_from_open_id_connect_provider::builders::RemoveClientIdFromOpenIdConnectProviderInputBuilder
+    {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -76,7 +78,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError,
+            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -84,13 +86,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProvider::operation_runtime_plugins(
-                self.handle.runtime_plugins.clone(),
-                &self.handle.conf,
-                self.config_override,
-            );
-        super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProvider::orchestrate(
+        let runtime_plugins = super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProvider::operation_runtime_plugins(
+                            self.handle.runtime_plugins.clone(),
+                            &self.handle.conf,
+                            self.config_override,
+                        );
+        super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProvider::orchestrate(
             &runtime_plugins,
             input,
         )
@@ -102,7 +103,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput,
-        super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError,
+        super::super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/remove_client_id_from_open_id_connect_provider.rs`

```diff
--- reference/src/operation/remove_client_id_from_open_id_connect_provider.rs
+++ generated/src/operation/remove_client_id_from_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `RemoveClientIDFromOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `RemoveClientIdFromOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct RemoveClientIDFromOpenIDConnectProvider;
-impl RemoveClientIDFromOpenIDConnectProvider {
-    /// Creates a new `RemoveClientIDFromOpenIDConnectProvider`
+pub struct RemoveClientIdFromOpenIdConnectProvider;
+impl RemoveClientIdFromOpenIdConnectProvider {
+    /// Creates a new `RemoveClientIdFromOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -23,8 +23,9 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >| {
             err.map_service_error(|err| {
-                err.downcast::<super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError>()
-                    .expect("correct error type")
+                err.downcast::<super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError>(
+                )
+                .expect("correct error type")
             })
         };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
@@ -90,15 +91,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for RemoveClientIDFromOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for RemoveClientIdFromOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("RemoveClientIDFromOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            RemoveClientIDFromOpenIDConnectProviderRequestSerializer,
+            RemoveClientIdFromOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            RemoveClientIDFromOpenIDConnectProviderResponseDeserializer,
+            RemoveClientIdFromOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -133,13 +134,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RemoveClientIDFromOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                RemoveClientIDFromOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                RemoveClientIdFromOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                RemoveClientIDFromOpenIDConnectProviderEndpointParamsInterceptor,
+                RemoveClientIdFromOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIDFromOpenIDConnectProviderError,
@@ -156,12 +157,12 @@
 }

 #[derive(Debug)]
-struct RemoveClientIDFromOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct RemoveClientIdFromOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RemoveClientIDFromOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RemoveClientIdFromOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "RemoveClientIDFromOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "RemoveClientIdFromOpenIdConnectProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -218,9 +219,7 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_remove_client_id_from_open_id_connect_provider::de_remove_client_id_from_open_id_connect_provider_http_error(
-                status, headers, body,
-            )
+            super::super::protocol_serde::shape_remove_client_id_from_open_id_connect_provider::de_remove_client_id_from_open_id_connect_provider_http_error(status, headers, body)
         } else {
             super::super::protocol_serde::shape_remove_client_id_from_open_id_connect_provider::de_remove_client_id_from_open_id_connect_provider_http_response(status, headers, body)
         };
@@ -263,11 +262,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_remove_client_id_from_open_id_connect_provider_input::ser_remove_client_id_from_open_id_connect_provider_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_remove_client_id_from_open_id_connect_provider_input::ser_remove_client_id_from_open_id_connect_provider_op_input(& input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -276,12 +274,12 @@
     }
 }
 #[derive(Debug)]
-struct RemoveClientIDFromOpenIDConnectProviderEndpointParamsInterceptor;
+struct RemoveClientIdFromOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RemoveClientIDFromOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RemoveClientIdFromOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "RemoveClientIDFromOpenIDConnectProviderEndpointParamsInterceptor"
+        "RemoveClientIdFromOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/remove_role_from_instance_profile/_remove_role_from_instance_profile_input.rs`

```diff
--- reference/src/operation/remove_role_from_instance_profile/_remove_role_from_instance_profile_input.rs
+++ generated/src/operation/remove_role_from_instance_profile/_remove_role_from_instance_profile_input.rs
@@ -80,9 +80,11 @@
         super::super::super::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileInput {
-            instance_profile_name: self.instance_profile_name,
-            role_name: self.role_name,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileInput {
+                instance_profile_name: self.instance_profile_name,
+                role_name: self.role_name,
+            },
+        )
     }
 }
```

### `src/operation/remove_role_from_instance_profile/builders.rs`

```diff
--- reference/src/operation/remove_role_from_instance_profile/builders.rs
+++ generated/src/operation/remove_role_from_instance_profile/builders.rs
@@ -60,7 +60,9 @@
         }
     }
     /// Access the RemoveRoleFromInstanceProfile as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -84,11 +86,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfile::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfile::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfile::orchestrate(&runtime_plugins, input).await
     }

```

### `src/operation/remove_role_from_instance_profile.rs`

```diff
--- reference/src/operation/remove_role_from_instance_profile.rs
+++ generated/src/operation/remove_role_from_instance_profile.rs
@@ -212,9 +212,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_remove_role_from_instance_profile::de_remove_role_from_instance_profile_http_error(status, headers, body)
+            super::super::protocol_serde::shape_remove_role_from_instance_profile::de_remove_role_from_instance_profile_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_remove_role_from_instance_profile::de_remove_role_from_instance_profile_http_response(status, headers, body)
+            super::super::protocol_serde::shape_remove_role_from_instance_profile::de_remove_role_from_instance_profile_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -255,12 +259,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_remove_role_from_instance_profile_input::ser_remove_role_from_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_remove_role_from_instance_profile_input::ser_remove_role_from_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/remove_user_from_group/_remove_user_from_group_input.rs`

```diff
--- reference/src/operation/remove_user_from_group/_remove_user_from_group_input.rs
+++ generated/src/operation/remove_user_from_group/_remove_user_from_group_input.rs
@@ -76,8 +76,10 @@
     /// Consumes the builder and constructs a [`RemoveUserFromGroupInput`](crate::operation::remove_user_from_group::RemoveUserFromGroupInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::remove_user_from_group::RemoveUserFromGroupInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::remove_user_from_group::RemoveUserFromGroupInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::remove_user_from_group::RemoveUserFromGroupInput {
             group_name: self.group_name,
             user_name: self.user_name,
```

### `src/operation/remove_user_from_group.rs`

```diff
--- reference/src/operation/remove_user_from_group.rs
+++ generated/src/operation/remove_user_from_group.rs
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
-            super::super::protocol_serde::shape_remove_user_from_group_input::ser_remove_user_from_group_input_input_input(&input)?,
+            super::super::protocol_serde::shape_remove_user_from_group_input::ser_remove_user_from_group_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/reset_service_specific_credential/_reset_service_specific_credential_input.rs`

```diff
--- reference/src/operation/reset_service_specific_credential/_reset_service_specific_credential_input.rs
+++ generated/src/operation/reset_service_specific_credential/_reset_service_specific_credential_input.rs
@@ -79,9 +79,11 @@
         super::super::super::operation::reset_service_specific_credential::ResetServiceSpecificCredentialInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::reset_service_specific_credential::ResetServiceSpecificCredentialInput {
-            user_name: self.user_name,
-            service_specific_credential_id: self.service_specific_credential_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::reset_service_specific_credential::ResetServiceSpecificCredentialInput {
+                user_name: self.user_name,
+                service_specific_credential_id: self.service_specific_credential_id,
+            },
+        )
     }
 }
```

### `src/operation/reset_service_specific_credential/builders.rs`

```diff
--- reference/src/operation/reset_service_specific_credential/builders.rs
+++ generated/src/operation/reset_service_specific_credential/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the ResetServiceSpecificCredential as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,11 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::reset_service_specific_credential::ResetServiceSpecificCredential::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::reset_service_specific_credential::ResetServiceSpecificCredential::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::reset_service_specific_credential::ResetServiceSpecificCredential::orchestrate(&runtime_plugins, input).await
     }

```

### `src/operation/reset_service_specific_credential.rs`

```diff
--- reference/src/operation/reset_service_specific_credential.rs
+++ generated/src/operation/reset_service_specific_credential.rs
@@ -219,9 +219,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_reset_service_specific_credential::de_reset_service_specific_credential_http_error(status, headers, body)
+            super::super::protocol_serde::shape_reset_service_specific_credential::de_reset_service_specific_credential_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_reset_service_specific_credential::de_reset_service_specific_credential_http_response(status, headers, body)
+            super::super::protocol_serde::shape_reset_service_specific_credential::de_reset_service_specific_credential_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -262,12 +266,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_reset_service_specific_credential_input::ser_reset_service_specific_credential_input_input_input(&input)?,
+            super::super::protocol_serde::shape_reset_service_specific_credential_input::ser_reset_service_specific_credential_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/resync_mfa_device/_resync_mfa_device_input.rs`

```diff
--- reference/src/operation/resync_mfa_device/_resync_mfa_device_input.rs
+++ generated/src/operation/resync_mfa_device/_resync_mfa_device_input.rs
@@ -130,7 +130,10 @@
     /// Consumes the builder and constructs a [`ResyncMfaDeviceInput`](crate::operation::resync_mfa_device::ResyncMfaDeviceInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::resync_mfa_device::ResyncMfaDeviceInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::resync_mfa_device::ResyncMfaDeviceInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::resync_mfa_device::ResyncMfaDeviceInput {
             user_name: self.user_name,
             serial_number: self.serial_number,
```

### `src/operation/resync_mfa_device/builders.rs`

```diff
--- reference/src/operation/resync_mfa_device/builders.rs
+++ generated/src/operation/resync_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::resync_mfa_device::ResyncMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::resync_mfa_device::ResyncMFADeviceError,
+            super::super::super::operation::resync_mfa_device::ResyncMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `ResyncMFADevice`.
+/// Fluent builder constructing a request to `ResyncMfaDevice`.
 ///
 /// <p>Synchronizes the specified MFA device with its IAM resource object on the Amazon Web Services servers.</p>
 /// <p>For more information about creating and working with virtual MFA devices, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_VirtualMFA.html">Using a virtual MFA device</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct ResyncMFADeviceFluentBuilder {
+pub struct ResyncMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::resync_mfa_device::builders::ResyncMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::resync_mfa_device::ResyncMfaDeviceOutput,
-        super::super::super::operation::resync_mfa_device::ResyncMFADeviceError,
-    > for ResyncMFADeviceFluentBuilder
+        super::super::super::operation::resync_mfa_device::ResyncMfaDeviceError,
+    > for ResyncMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::resync_mfa_device::ResyncMfaDeviceOutput,
-            super::super::super::operation::resync_mfa_device::ResyncMFADeviceError,
+            super::super::super::operation::resync_mfa_device::ResyncMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl ResyncMFADeviceFluentBuilder {
-    /// Creates a new `ResyncMFADeviceFluentBuilder`.
+impl ResyncMfaDeviceFluentBuilder {
+    /// Creates a new `ResyncMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the ResyncMFADevice as a reference.
+    /// Access the ResyncMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::resync_mfa_device::builders::ResyncMfaDeviceInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::resync_mfa_device::ResyncMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::resync_mfa_device::ResyncMFADeviceError,
+            super::super::super::operation::resync_mfa_device::ResyncMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::resync_mfa_device::ResyncMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::resync_mfa_device::ResyncMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::resync_mfa_device::ResyncMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::resync_mfa_device::ResyncMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::resync_mfa_device::ResyncMfaDeviceOutput,
-        super::super::super::operation::resync_mfa_device::ResyncMFADeviceError,
+        super::super::super::operation::resync_mfa_device::ResyncMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/resync_mfa_device.rs`

```diff
--- reference/src/operation/resync_mfa_device.rs
+++ generated/src/operation/resync_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `ResyncMFADevice`.
+/// Orchestration and serialization glue logic for `ResyncMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct ResyncMFADevice;
-impl ResyncMFADevice {
-    /// Creates a new `ResyncMFADevice`
+pub struct ResyncMfaDevice;
+impl ResyncMfaDevice {
+    /// Creates a new `ResyncMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ResyncMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ResyncMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ResyncMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ResyncMFADeviceRequestSerializer,
+            ResyncMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ResyncMFADeviceResponseDeserializer,
+            ResyncMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ResyncMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ResyncMFADeviceTelemetryInputCaptureInterceptor,
+                ResyncMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ResyncMFADeviceEndpointParamsInterceptor,
+                ResyncMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::resync_mfa_device::ResyncMFADeviceError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct ResyncMFADeviceTelemetryInputCaptureInterceptor;
+struct ResyncMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ResyncMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ResyncMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "ResyncMFADeviceTelemetryInputCaptureInterceptor"
+        "ResyncMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -262,12 +262,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_resync_mfa_device_input::ser_resync_mfa_device_input_input_input(&input)?,
+            super::super::protocol_serde::shape_resync_mfa_device_input::ser_resync_mfa_device_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -277,12 +276,12 @@
     }
 }
 #[derive(Debug)]
-struct ResyncMFADeviceEndpointParamsInterceptor;
+struct ResyncMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ResyncMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ResyncMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "ResyncMFADeviceEndpointParamsInterceptor"
+        "ResyncMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/send_delegation_token/_send_delegation_token_input.rs`

```diff
--- reference/src/operation/send_delegation_token/_send_delegation_token_input.rs
+++ generated/src/operation/send_delegation_token/_send_delegation_token_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`SendDelegationTokenInput`](crate::operation::send_delegation_token::SendDelegationTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::send_delegation_token::SendDelegationTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::send_delegation_token::SendDelegationTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::send_delegation_token::SendDelegationTokenInput {
             delegation_request_id: self.delegation_request_id,
         })
```

### `src/operation/send_delegation_token.rs`

```diff
--- reference/src/operation/send_delegation_token.rs
+++ generated/src/operation/send_delegation_token.rs
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
-            super::super::protocol_serde::shape_send_delegation_token_input::ser_send_delegation_token_input_input_input(&input)?,
+            super::super::protocol_serde::shape_send_delegation_token_input::ser_send_delegation_token_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/set_default_policy_version.rs`

```diff
--- reference/src/operation/set_default_policy_version.rs
+++ generated/src/operation/set_default_policy_version.rs
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
-            super::super::protocol_serde::shape_set_default_policy_version_input::ser_set_default_policy_version_input_input_input(&input)?,
+            super::super::protocol_serde::shape_set_default_policy_version_input::ser_set_default_policy_version_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/set_security_token_service_preferences/builders.rs`

```diff
--- reference/src/operation/set_security_token_service_preferences/builders.rs
+++ generated/src/operation/set_security_token_service_preferences/builders.rs
@@ -60,7 +60,9 @@
         }
     }
     /// Access the SetSecurityTokenServicePreferences as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -84,12 +86,17 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferences::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferences::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferences::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferences::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/set_security_token_service_preferences.rs`

```diff
--- reference/src/operation/set_security_token_service_preferences.rs
+++ generated/src/operation/set_security_token_service_preferences.rs
@@ -214,12 +214,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_set_security_token_service_preferences_input::ser_set_security_token_service_preferences_input_input_input(
+            super::super::protocol_serde::shape_set_security_token_service_preferences_input::ser_set_security_token_service_preferences_op_input(
                 &input,
             )?,
         );
```

### `src/operation/simulate_custom_policy/_simulate_custom_policy_input.rs`

```diff
--- reference/src/operation/simulate_custom_policy/_simulate_custom_policy_input.rs
+++ generated/src/operation/simulate_custom_policy/_simulate_custom_policy_input.rs
@@ -217,7 +217,8 @@
 pub struct SimulateCustomPolicyInputBuilder {
     pub(crate) policy_input_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) permissions_boundary_policy_input_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
-    pub(crate) ordered_organization_policy_input_list: ::std::option::Option<::std::vec::Vec<super::super::super::types::OrderedOrganizationPolicyType>>,
+    pub(crate) ordered_organization_policy_input_list:
+        ::std::option::Option<::std::vec::Vec<super::super::super::types::OrderedOrganizationPolicyType>>,
     pub(crate) action_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) resource_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     pub(crate) resource_policy: ::std::option::Option<::std::string::String>,
@@ -355,7 +356,9 @@
     /// <p>An ordered list of service control policies (SCPs) to include in the simulation. Each element represents one level of an Organizations hierarchy, from the organization root to the account.</p>
     /// <p>The simulator evaluates SCPs in the order that you provide, consistent with how Organizations enforces SCPs. The first element must represent the organization root, and the last element must represent the account. Any elements between them represent organizational units (OUs) in descending order.</p>
     /// <p>Use this parameter to simulate the effect of an SCP hierarchy without calling <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_SimulatePrincipalPolicy.html">SimulatePrincipalPolicy</a>.</p>
-    pub fn get_ordered_organization_policy_input_list(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::OrderedOrganizationPolicyType>> {
+    pub fn get_ordered_organization_policy_input_list(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::OrderedOrganizationPolicyType>> {
         &self.ordered_organization_policy_input_list
     }
     /// Appends an item to `action_names`.
@@ -610,8 +613,10 @@
     /// Consumes the builder and constructs a [`SimulateCustomPolicyInput`](crate::operation::simulate_custom_policy::SimulateCustomPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::simulate_custom_policy::SimulateCustomPolicyInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::simulate_custom_policy::SimulateCustomPolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::simulate_custom_policy::SimulateCustomPolicyInput {
             policy_input_list: self.policy_input_list,
             permissions_boundary_policy_input_list: self.permissions_boundary_policy_input_list,
```

### `src/operation/simulate_custom_policy/builders.rs`

```diff
--- reference/src/operation/simulate_custom_policy/builders.rs
+++ generated/src/operation/simulate_custom_policy/builders.rs
@@ -243,7 +243,9 @@
     /// <p>An ordered list of service control policies (SCPs) to include in the simulation. Each element represents one level of an Organizations hierarchy, from the organization root to the account.</p>
     /// <p>The simulator evaluates SCPs in the order that you provide, consistent with how Organizations enforces SCPs. The first element must represent the organization root, and the last element must represent the account. Any elements between them represent organizational units (OUs) in descending order.</p>
     /// <p>Use this parameter to simulate the effect of an SCP hierarchy without calling <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_SimulatePrincipalPolicy.html">SimulatePrincipalPolicy</a>.</p>
-    pub fn get_ordered_organization_policy_input_list(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::OrderedOrganizationPolicyType>> {
+    pub fn get_ordered_organization_policy_input_list(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::OrderedOrganizationPolicyType>> {
         self.inner.get_ordered_organization_policy_input_list()
     }
     ///
```

### `src/operation/simulate_custom_policy/paginator.rs`

```diff
--- reference/src/operation/simulate_custom_policy/paginator.rs
+++ generated/src/operation/simulate_custom_policy/paginator.rs
@@ -86,7 +86,11 @@
                         }
                     };
                     loop {
-                        let resp = super::super::super::operation::simulate_custom_policy::SimulateCustomPolicy::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::simulate_custom_policy::SimulateCustomPolicy::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/simulate_custom_policy.rs`

```diff
--- reference/src/operation/simulate_custom_policy.rs
+++ generated/src/operation/simulate_custom_policy.rs
@@ -270,12 +270,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_simulate_custom_policy_input::ser_simulate_custom_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_simulate_custom_policy_input::ser_simulate_custom_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/simulate_principal_policy/paginator.rs`

```diff
--- reference/src/operation/simulate_principal_policy/paginator.rs
+++ generated/src/operation/simulate_principal_policy/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::simulate_principal_policy::SimulatePrincipalPolicy::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::simulate_principal_policy::SimulatePrincipalPolicy::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/simulate_principal_policy.rs`

```diff
--- reference/src/operation/simulate_principal_policy.rs
+++ generated/src/operation/simulate_principal_policy.rs
@@ -275,12 +275,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_simulate_principal_policy_input::ser_simulate_principal_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_simulate_principal_policy_input::ser_simulate_principal_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/tag_instance_profile/_tag_instance_profile_input.rs`

```diff
--- reference/src/operation/tag_instance_profile/_tag_instance_profile_input.rs
+++ generated/src/operation/tag_instance_profile/_tag_instance_profile_input.rs
@@ -78,8 +78,10 @@
     /// Consumes the builder and constructs a [`TagInstanceProfileInput`](crate::operation::tag_instance_profile::TagInstanceProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::tag_instance_profile::TagInstanceProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::tag_instance_profile::TagInstanceProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::tag_instance_profile::TagInstanceProfileInput {
             instance_profile_name: self.instance_profile_name,
             tags: self.tags,
```

### `src/operation/tag_instance_profile.rs`

```diff
--- reference/src/operation/tag_instance_profile.rs
+++ generated/src/operation/tag_instance_profile.rs
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
-            super::super::protocol_serde::shape_tag_instance_profile_input::ser_tag_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_tag_instance_profile_input::ser_tag_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/tag_mfa_device/builders.rs`

```diff
--- reference/src/operation/tag_mfa_device/builders.rs
+++ generated/src/operation/tag_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::tag_mfa_device::TagMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::tag_mfa_device::TagMFADeviceError,
+            super::super::super::operation::tag_mfa_device::TagMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `TagMFADevice`.
+/// Fluent builder constructing a request to `TagMfaDevice`.
 ///
 /// <p>Adds one or more tags to an IAM virtual multi-factor authentication (MFA) device. If a tag with the same key name already exists, then that tag is overwritten with the new value.</p>
 /// <p>A tag consists of a key name and an associated value. By assigning tags to your resources, you can do the following:</p>
@@ -38,7 +38,7 @@
 /// </ul>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct TagMFADeviceFluentBuilder {
+pub struct TagMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::tag_mfa_device::builders::TagMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -46,8 +46,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::tag_mfa_device::TagMfaDeviceOutput,
-        super::super::super::operation::tag_mfa_device::TagMFADeviceError,
-    > for TagMFADeviceFluentBuilder
+        super::super::super::operation::tag_mfa_device::TagMfaDeviceError,
+    > for TagMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -55,14 +55,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::tag_mfa_device::TagMfaDeviceOutput,
-            super::super::super::operation::tag_mfa_device::TagMFADeviceError,
+            super::super::super::operation::tag_mfa_device::TagMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl TagMFADeviceFluentBuilder {
-    /// Creates a new `TagMFADeviceFluentBuilder`.
+impl TagMfaDeviceFluentBuilder {
+    /// Creates a new `TagMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -70,7 +70,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the TagMFADevice as a reference.
+    /// Access the TagMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::tag_mfa_device::builders::TagMfaDeviceInputBuilder {
         &self.inner
     }
@@ -87,7 +87,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::tag_mfa_device::TagMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::tag_mfa_device::TagMFADeviceError,
+            super::super::super::operation::tag_mfa_device::TagMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -95,12 +95,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::tag_mfa_device::TagMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::tag_mfa_device::TagMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::tag_mfa_device::TagMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::tag_mfa_device::TagMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -108,7 +108,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::tag_mfa_device::TagMfaDeviceOutput,
-        super::super::super::operation::tag_mfa_device::TagMFADeviceError,
+        super::super::super::operation::tag_mfa_device::TagMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/tag_mfa_device.rs`

```diff
--- reference/src/operation/tag_mfa_device.rs
+++ generated/src/operation/tag_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `TagMFADevice`.
+/// Orchestration and serialization glue logic for `TagMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct TagMFADevice;
-impl TagMFADevice {
-    /// Creates a new `TagMFADevice`
+pub struct TagMfaDevice;
+impl TagMfaDevice {
+    /// Creates a new `TagMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TagMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TagMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("TagMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            TagMFADeviceRequestSerializer,
+            TagMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            TagMFADeviceResponseDeserializer,
+            TagMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TagMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TagMFADeviceTelemetryInputCaptureInterceptor,
+                TagMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TagMFADeviceEndpointParamsInterceptor,
+                TagMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::tag_mfa_device::TagMFADeviceError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct TagMFADeviceTelemetryInputCaptureInterceptor;
+struct TagMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "TagMFADeviceTelemetryInputCaptureInterceptor"
+        "TagMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -247,11 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_mfa_device_input::ser_tag_mfa_device_input_input_input(
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_mfa_device_input::ser_tag_mfa_device_op_input(
             &input,
         )?);
         if let Some(content_length) = body.content_length() {
@@ -262,12 +261,12 @@
     }
 }
 #[derive(Debug)]
-struct TagMFADeviceEndpointParamsInterceptor;
+struct TagMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "TagMFADeviceEndpointParamsInterceptor"
+        "TagMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/tag_open_id_connect_provider/_tag_open_id_connect_provider_input.rs`

```diff
--- reference/src/operation/tag_open_id_connect_provider/_tag_open_id_connect_provider_input.rs
+++ generated/src/operation/tag_open_id_connect_provider/_tag_open_id_connect_provider_input.rs
@@ -82,9 +82,11 @@
         super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderInput {
-            open_id_connect_provider_arn: self.open_id_connect_provider_arn,
-            tags: self.tags,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderInput {
+                open_id_connect_provider_arn: self.open_id_connect_provider_arn,
+                tags: self.tags,
+            },
+        )
     }
 }
```

### `src/operation/tag_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/tag_open_id_connect_provider/builders.rs
+++ generated/src/operation/tag_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
+            super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `TagOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `TagOpenIdConnectProvider`.
 ///
 /// <p>Adds one or more tags to an OpenID Connect (OIDC)-compatible identity provider. For more information about these providers, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_oidc.html">About web identity federation</a>. If a tag with the same key name already exists, then that tag is overwritten with the new value.</p>
 /// <p>A tag consists of a key name and an associated value. By assigning tags to your resources, you can do the following:</p>
@@ -38,7 +38,7 @@
 /// </ul>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct TagOpenIDConnectProviderFluentBuilder {
+pub struct TagOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::tag_open_id_connect_provider::builders::TagOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -46,8 +46,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
-        super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
-    > for TagOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderError,
+    > for TagOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -55,14 +55,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
-            super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
+            super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl TagOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `TagOpenIDConnectProviderFluentBuilder`.
+impl TagOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `TagOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -70,7 +70,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the TagOpenIDConnectProvider as a reference.
+    /// Access the TagOpenIdConnectProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::tag_open_id_connect_provider::builders::TagOpenIdConnectProviderInputBuilder {
         &self.inner
     }
@@ -87,7 +87,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
+            super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -95,12 +95,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -108,7 +108,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
-        super::super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
+        super::super::super::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/tag_open_id_connect_provider.rs`

```diff
--- reference/src/operation/tag_open_id_connect_provider.rs
+++ generated/src/operation/tag_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `TagOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `TagOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct TagOpenIDConnectProvider;
-impl TagOpenIDConnectProvider {
-    /// Creates a new `TagOpenIDConnectProvider`
+pub struct TagOpenIdConnectProvider;
+impl TagOpenIdConnectProvider {
+    /// Creates a new `TagOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TagOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TagOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("TagOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            TagOpenIDConnectProviderRequestSerializer,
+            TagOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            TagOpenIDConnectProviderResponseDeserializer,
+            TagOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TagOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TagOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                TagOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TagOpenIDConnectProviderEndpointParamsInterceptor,
+                TagOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct TagOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct TagOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "TagOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "TagOpenIdConnectProviderTelemetryInputCaptureInterceptor"
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
-            super::super::protocol_serde::shape_tag_open_id_connect_provider_input::ser_tag_open_id_connect_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_tag_open_id_connect_provider_input::ser_tag_open_id_connect_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +264,12 @@
     }
 }
 #[derive(Debug)]
-struct TagOpenIDConnectProviderEndpointParamsInterceptor;
+struct TagOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "TagOpenIDConnectProviderEndpointParamsInterceptor"
+        "TagOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/tag_policy/_tag_policy_input.rs`

```diff
--- reference/src/operation/tag_policy/_tag_policy_input.rs
+++ generated/src/operation/tag_policy/_tag_policy_input.rs
@@ -76,7 +76,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`TagPolicyInput`](crate::operation::tag_policy::TagPolicyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::tag_policy::TagPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::tag_policy::TagPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::tag_policy::TagPolicyInput {
             policy_arn: self.policy_arn,
             tags: self.tags,
```

### `src/operation/tag_policy/builders.rs`

```diff
--- reference/src/operation/tag_policy/builders.rs
+++ generated/src/operation/tag_policy/builders.rs
@@ -43,14 +43,20 @@
     inner: super::super::super::operation::tag_policy::builders::TagPolicyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::tag_policy::TagPolicyOutput, super::super::super::operation::tag_policy::TagPolicyError>
-    for TagPolicyFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::tag_policy::TagPolicyOutput,
+        super::super::super::operation::tag_policy::TagPolicyError,
+    > for TagPolicyFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::tag_policy::TagPolicyOutput, super::super::super::operation::tag_policy::TagPolicyError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::tag_policy::TagPolicyOutput,
+            super::super::super::operation::tag_policy::TagPolicyError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/tag_policy.rs`

```diff
--- reference/src/operation/tag_policy.rs
+++ generated/src/operation/tag_policy.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::tag_policy::TagPolicyInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::tag_policy::TagPolicyInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -245,11 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_policy_input::ser_tag_policy_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_policy_input::ser_tag_policy_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/tag_role/_tag_role_input.rs`

```diff
--- reference/src/operation/tag_role/_tag_role_input.rs
+++ generated/src/operation/tag_role/_tag_role_input.rs
@@ -76,7 +76,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`TagRoleInput`](crate::operation::tag_role::TagRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::tag_role::TagRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::tag_role::TagRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::tag_role::TagRoleInput {
             role_name: self.role_name,
             tags: self.tags,
```

### `src/operation/tag_role/builders.rs`

```diff
--- reference/src/operation/tag_role/builders.rs
+++ generated/src/operation/tag_role/builders.rs
@@ -46,14 +46,20 @@
     inner: super::super::super::operation::tag_role::builders::TagRoleInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::tag_role::TagRoleOutput, super::super::super::operation::tag_role::TagRoleError>
-    for TagRoleFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::tag_role::TagRoleOutput,
+        super::super::super::operation::tag_role::TagRoleError,
+    > for TagRoleFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::tag_role::TagRoleOutput, super::super::super::operation::tag_role::TagRoleError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::tag_role::TagRoleOutput,
+            super::super::super::operation::tag_role::TagRoleError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -103,8 +109,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::tag_role::TagRoleOutput, super::super::super::operation::tag_role::TagRoleError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::tag_role::TagRoleOutput,
+        super::super::super::operation::tag_role::TagRoleError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/tag_role.rs`

```diff
--- reference/src/operation/tag_role.rs
+++ generated/src/operation/tag_role.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::tag_role::TagRoleError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::tag_role::TagRoleError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -241,11 +245,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_role_input::ser_tag_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_role_input::ser_tag_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/tag_saml_provider/_tag_saml_provider_input.rs`

```diff
--- reference/src/operation/tag_saml_provider/_tag_saml_provider_input.rs
+++ generated/src/operation/tag_saml_provider/_tag_saml_provider_input.rs
@@ -78,7 +78,10 @@
     /// Consumes the builder and constructs a [`TagSamlProviderInput`](crate::operation::tag_saml_provider::TagSamlProviderInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::tag_saml_provider::TagSamlProviderInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::tag_saml_provider::TagSamlProviderInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::tag_saml_provider::TagSamlProviderInput {
             saml_provider_arn: self.saml_provider_arn,
             tags: self.tags,
```

### `src/operation/tag_saml_provider/builders.rs`

```diff
--- reference/src/operation/tag_saml_provider/builders.rs
+++ generated/src/operation/tag_saml_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::tag_saml_provider::TagSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::tag_saml_provider::TagSAMLProviderError,
+            super::super::super::operation::tag_saml_provider::TagSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `TagSAMLProvider`.
+/// Fluent builder constructing a request to `TagSamlProvider`.
 ///
 /// <p>Adds one or more tags to a Security Assertion Markup Language (SAML) identity provider. For more information about these providers, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_saml.html">About SAML 2.0-based federation </a>. If a tag with the same key name already exists, then that tag is overwritten with the new value.</p>
 /// <p>A tag consists of a key name and an associated value. By assigning tags to your resources, you can do the following:</p>
@@ -38,7 +38,7 @@
 /// </ul>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct TagSAMLProviderFluentBuilder {
+pub struct TagSamlProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::tag_saml_provider::builders::TagSamlProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -46,8 +46,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::tag_saml_provider::TagSamlProviderOutput,
-        super::super::super::operation::tag_saml_provider::TagSAMLProviderError,
-    > for TagSAMLProviderFluentBuilder
+        super::super::super::operation::tag_saml_provider::TagSamlProviderError,
+    > for TagSamlProviderFluentBuilder
 {
     fn send(
         self,
@@ -55,14 +55,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::tag_saml_provider::TagSamlProviderOutput,
-            super::super::super::operation::tag_saml_provider::TagSAMLProviderError,
+            super::super::super::operation::tag_saml_provider::TagSamlProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl TagSAMLProviderFluentBuilder {
-    /// Creates a new `TagSAMLProviderFluentBuilder`.
+impl TagSamlProviderFluentBuilder {
+    /// Creates a new `TagSamlProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -70,7 +70,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the TagSAMLProvider as a reference.
+    /// Access the TagSamlProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::tag_saml_provider::builders::TagSamlProviderInputBuilder {
         &self.inner
     }
@@ -87,7 +87,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::tag_saml_provider::TagSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::tag_saml_provider::TagSAMLProviderError,
+            super::super::super::operation::tag_saml_provider::TagSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -95,12 +95,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::tag_saml_provider::TagSAMLProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::tag_saml_provider::TagSamlProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::tag_saml_provider::TagSAMLProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::tag_saml_provider::TagSamlProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -108,7 +108,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::tag_saml_provider::TagSamlProviderOutput,
-        super::super::super::operation::tag_saml_provider::TagSAMLProviderError,
+        super::super::super::operation::tag_saml_provider::TagSamlProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/tag_saml_provider.rs`

```diff
--- reference/src/operation/tag_saml_provider.rs
+++ generated/src/operation/tag_saml_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `TagSAMLProvider`.
+/// Orchestration and serialization glue logic for `TagSamlProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct TagSAMLProvider;
-impl TagSAMLProvider {
-    /// Creates a new `TagSAMLProvider`
+pub struct TagSamlProvider;
+impl TagSamlProvider {
+    /// Creates a new `TagSamlProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TagSAMLProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TagSamlProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("TagSAMLProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            TagSAMLProviderRequestSerializer,
+            TagSamlProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            TagSAMLProviderResponseDeserializer,
+            TagSamlProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TagSAMLProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TagSAMLProviderTelemetryInputCaptureInterceptor,
+                TagSamlProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TagSAMLProviderEndpointParamsInterceptor,
+                TagSamlProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::tag_saml_provider::TagSAMLProviderError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct TagSAMLProviderTelemetryInputCaptureInterceptor;
+struct TagSamlProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagSAMLProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagSamlProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "TagSAMLProviderTelemetryInputCaptureInterceptor"
+        "TagSamlProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_tag_saml_provider_input::ser_tag_saml_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_tag_saml_provider_input::ser_tag_saml_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -262,12 +261,12 @@
     }
 }
 #[derive(Debug)]
-struct TagSAMLProviderEndpointParamsInterceptor;
+struct TagSamlProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagSAMLProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TagSamlProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "TagSAMLProviderEndpointParamsInterceptor"
+        "TagSamlProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/tag_server_certificate/_tag_server_certificate_input.rs`

```diff
--- reference/src/operation/tag_server_certificate/_tag_server_certificate_input.rs
+++ generated/src/operation/tag_server_certificate/_tag_server_certificate_input.rs
@@ -78,8 +78,10 @@
     /// Consumes the builder and constructs a [`TagServerCertificateInput`](crate::operation::tag_server_certificate::TagServerCertificateInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::tag_server_certificate::TagServerCertificateInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::tag_server_certificate::TagServerCertificateInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::tag_server_certificate::TagServerCertificateInput {
             server_certificate_name: self.server_certificate_name,
             tags: self.tags,
```

### `src/operation/tag_server_certificate.rs`

```diff
--- reference/src/operation/tag_server_certificate.rs
+++ generated/src/operation/tag_server_certificate.rs
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
-            super::super::protocol_serde::shape_tag_server_certificate_input::ser_tag_server_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_tag_server_certificate_input::ser_tag_server_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/tag_user/_tag_user_input.rs`

```diff
--- reference/src/operation/tag_user/_tag_user_input.rs
+++ generated/src/operation/tag_user/_tag_user_input.rs
@@ -76,7 +76,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`TagUserInput`](crate::operation::tag_user::TagUserInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::tag_user::TagUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::tag_user::TagUserInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::tag_user::TagUserInput {
             user_name: self.user_name,
             tags: self.tags,
```

### `src/operation/tag_user/builders.rs`

```diff
--- reference/src/operation/tag_user/builders.rs
+++ generated/src/operation/tag_user/builders.rs
@@ -46,14 +46,20 @@
     inner: super::super::super::operation::tag_user::builders::TagUserInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::tag_user::TagUserOutput, super::super::super::operation::tag_user::TagUserError>
-    for TagUserFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::tag_user::TagUserOutput,
+        super::super::super::operation::tag_user::TagUserError,
+    > for TagUserFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::tag_user::TagUserOutput, super::super::super::operation::tag_user::TagUserError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::tag_user::TagUserOutput,
+            super::super::super::operation::tag_user::TagUserError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -103,8 +109,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::tag_user::TagUserOutput, super::super::super::operation::tag_user::TagUserError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::tag_user::TagUserOutput,
+        super::super::super::operation::tag_user::TagUserError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/tag_user.rs`

```diff
--- reference/src/operation/tag_user.rs
+++ generated/src/operation/tag_user.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::tag_user::TagUserError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::tag_user::TagUserError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -241,11 +245,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_user_input::ser_tag_user_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_tag_user_input::ser_tag_user_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/untag_instance_profile/_untag_instance_profile_input.rs`

```diff
--- reference/src/operation/untag_instance_profile/_untag_instance_profile_input.rs
+++ generated/src/operation/untag_instance_profile/_untag_instance_profile_input.rs
@@ -78,8 +78,10 @@
     /// Consumes the builder and constructs a [`UntagInstanceProfileInput`](crate::operation::untag_instance_profile::UntagInstanceProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::untag_instance_profile::UntagInstanceProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::untag_instance_profile::UntagInstanceProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::untag_instance_profile::UntagInstanceProfileInput {
             instance_profile_name: self.instance_profile_name,
             tag_keys: self.tag_keys,
```

### `src/operation/untag_instance_profile.rs`

```diff
--- reference/src/operation/untag_instance_profile.rs
+++ generated/src/operation/untag_instance_profile.rs
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
-            super::super::protocol_serde::shape_untag_instance_profile_input::ser_untag_instance_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_untag_instance_profile_input::ser_untag_instance_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/untag_mfa_device/builders.rs`

```diff
--- reference/src/operation/untag_mfa_device/builders.rs
+++ generated/src/operation/untag_mfa_device/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::untag_mfa_device::UntagMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::untag_mfa_device::UntagMFADeviceError,
+            super::super::super::operation::untag_mfa_device::UntagMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UntagMFADevice`.
+/// Fluent builder constructing a request to `UntagMfaDevice`.
 ///
 /// <p>Removes the specified tags from the IAM virtual multi-factor authentication (MFA) device. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UntagMFADeviceFluentBuilder {
+pub struct UntagMfaDeviceFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::untag_mfa_device::builders::UntagMfaDeviceInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::untag_mfa_device::UntagMfaDeviceOutput,
-        super::super::super::operation::untag_mfa_device::UntagMFADeviceError,
-    > for UntagMFADeviceFluentBuilder
+        super::super::super::operation::untag_mfa_device::UntagMfaDeviceError,
+    > for UntagMfaDeviceFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::untag_mfa_device::UntagMfaDeviceOutput,
-            super::super::super::operation::untag_mfa_device::UntagMFADeviceError,
+            super::super::super::operation::untag_mfa_device::UntagMfaDeviceError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UntagMFADeviceFluentBuilder {
-    /// Creates a new `UntagMFADeviceFluentBuilder`.
+impl UntagMfaDeviceFluentBuilder {
+    /// Creates a new `UntagMfaDeviceFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UntagMFADevice as a reference.
+    /// Access the UntagMfaDevice as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::untag_mfa_device::builders::UntagMfaDeviceInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::untag_mfa_device::UntagMfaDeviceOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::untag_mfa_device::UntagMFADeviceError,
+            super::super::super::operation::untag_mfa_device::UntagMfaDeviceError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::untag_mfa_device::UntagMFADevice::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::untag_mfa_device::UntagMfaDevice::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::untag_mfa_device::UntagMFADevice::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::untag_mfa_device::UntagMfaDevice::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::untag_mfa_device::UntagMfaDeviceOutput,
-        super::super::super::operation::untag_mfa_device::UntagMFADeviceError,
+        super::super::super::operation::untag_mfa_device::UntagMfaDeviceError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/untag_mfa_device.rs`

```diff
--- reference/src/operation/untag_mfa_device.rs
+++ generated/src/operation/untag_mfa_device.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UntagMFADevice`.
+/// Orchestration and serialization glue logic for `UntagMfaDevice`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UntagMFADevice;
-impl UntagMFADevice {
-    /// Creates a new `UntagMFADevice`
+pub struct UntagMfaDevice;
+impl UntagMfaDevice {
+    /// Creates a new `UntagMfaDevice`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UntagMFADevice {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UntagMfaDevice {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UntagMFADevice");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UntagMFADeviceRequestSerializer,
+            UntagMfaDeviceRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UntagMFADeviceResponseDeserializer,
+            UntagMfaDeviceResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UntagMFADevice")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UntagMFADeviceTelemetryInputCaptureInterceptor,
+                UntagMfaDeviceTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UntagMFADeviceEndpointParamsInterceptor,
+                UntagMfaDeviceEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::untag_mfa_device::UntagMFADeviceError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct UntagMFADeviceTelemetryInputCaptureInterceptor;
+struct UntagMfaDeviceTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagMFADeviceTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagMfaDeviceTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UntagMFADeviceTelemetryInputCaptureInterceptor"
+        "UntagMfaDeviceTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_untag_mfa_device_input::ser_untag_mfa_device_input_input_input(&input)?,
+            super::super::protocol_serde::shape_untag_mfa_device_input::ser_untag_mfa_device_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -262,12 +261,12 @@
     }
 }
 #[derive(Debug)]
-struct UntagMFADeviceEndpointParamsInterceptor;
+struct UntagMfaDeviceEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagMFADeviceEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagMfaDeviceEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UntagMFADeviceEndpointParamsInterceptor"
+        "UntagMfaDeviceEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/untag_open_id_connect_provider/_untag_open_id_connect_provider_input.rs`

```diff
--- reference/src/operation/untag_open_id_connect_provider/_untag_open_id_connect_provider_input.rs
+++ generated/src/operation/untag_open_id_connect_provider/_untag_open_id_connect_provider_input.rs
@@ -82,9 +82,11 @@
         super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderInput {
-            open_id_connect_provider_arn: self.open_id_connect_provider_arn,
-            tag_keys: self.tag_keys,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderInput {
+                open_id_connect_provider_arn: self.open_id_connect_provider_arn,
+                tag_keys: self.tag_keys,
+            },
+        )
     }
 }
```

### `src/operation/untag_open_id_connect_provider/builders.rs`

```diff
--- reference/src/operation/untag_open_id_connect_provider/builders.rs
+++ generated/src/operation/untag_open_id_connect_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProviderError,
+            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UntagOpenIDConnectProvider`.
+/// Fluent builder constructing a request to `UntagOpenIdConnectProvider`.
 ///
 /// <p>Removes the specified tags from the specified OpenID Connect (OIDC)-compatible identity provider in IAM. For more information about OIDC providers, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_oidc.html">About web identity federation</a>. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UntagOpenIDConnectProviderFluentBuilder {
+pub struct UntagOpenIdConnectProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::untag_open_id_connect_provider::builders::UntagOpenIdConnectProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput,
-        super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProviderError,
-    > for UntagOpenIDConnectProviderFluentBuilder
+        super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderError,
+    > for UntagOpenIdConnectProviderFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput,
-            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProviderError,
+            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UntagOpenIDConnectProviderFluentBuilder {
-    /// Creates a new `UntagOpenIDConnectProviderFluentBuilder`.
+impl UntagOpenIdConnectProviderFluentBuilder {
+    /// Creates a new `UntagOpenIdConnectProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UntagOpenIDConnectProvider as a reference.
+    /// Access the UntagOpenIdConnectProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::untag_open_id_connect_provider::builders::UntagOpenIdConnectProviderInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProviderError,
+            super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput,
-        super::super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProviderError,
+        super::super::super::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/untag_open_id_connect_provider.rs`

```diff
--- reference/src/operation/untag_open_id_connect_provider.rs
+++ generated/src/operation/untag_open_id_connect_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UntagOpenIDConnectProvider`.
+/// Orchestration and serialization glue logic for `UntagOpenIdConnectProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UntagOpenIDConnectProvider;
-impl UntagOpenIDConnectProvider {
-    /// Creates a new `UntagOpenIDConnectProvider`
+pub struct UntagOpenIdConnectProvider;
+impl UntagOpenIdConnectProvider {
+    /// Creates a new `UntagOpenIdConnectProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UntagOpenIDConnectProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UntagOpenIdConnectProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UntagOpenIDConnectProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UntagOpenIDConnectProviderRequestSerializer,
+            UntagOpenIdConnectProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UntagOpenIDConnectProviderResponseDeserializer,
+            UntagOpenIdConnectProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -127,13 +127,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UntagOpenIDConnectProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UntagOpenIDConnectProviderTelemetryInputCaptureInterceptor,
+                UntagOpenIdConnectProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UntagOpenIDConnectProviderEndpointParamsInterceptor,
+                UntagOpenIdConnectProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::untag_open_id_connect_provider::UntagOpenIDConnectProviderError,
@@ -150,12 +150,12 @@
 }

 #[derive(Debug)]
-struct UntagOpenIDConnectProviderTelemetryInputCaptureInterceptor;
+struct UntagOpenIdConnectProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagOpenIDConnectProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagOpenIdConnectProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UntagOpenIDConnectProviderTelemetryInputCaptureInterceptor"
+        "UntagOpenIdConnectProviderTelemetryInputCaptureInterceptor"
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
-            super::super::protocol_serde::shape_untag_open_id_connect_provider_input::ser_untag_open_id_connect_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_untag_open_id_connect_provider_input::ser_untag_open_id_connect_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -265,12 +264,12 @@
     }
 }
 #[derive(Debug)]
-struct UntagOpenIDConnectProviderEndpointParamsInterceptor;
+struct UntagOpenIdConnectProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagOpenIDConnectProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagOpenIdConnectProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UntagOpenIDConnectProviderEndpointParamsInterceptor"
+        "UntagOpenIdConnectProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/untag_policy/_untag_policy_input.rs`

```diff
--- reference/src/operation/untag_policy/_untag_policy_input.rs
+++ generated/src/operation/untag_policy/_untag_policy_input.rs
@@ -76,7 +76,9 @@
         &self.tag_keys
     }
     /// Consumes the builder and constructs a [`UntagPolicyInput`](crate::operation::untag_policy::UntagPolicyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::untag_policy::UntagPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::untag_policy::UntagPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::untag_policy::UntagPolicyInput {
             policy_arn: self.policy_arn,
             tag_keys: self.tag_keys,
```

### `src/operation/untag_policy.rs`

```diff
--- reference/src/operation/untag_policy.rs
+++ generated/src/operation/untag_policy.rs
@@ -247,13 +247,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_policy_input::ser_untag_policy_input_input_input(
-            &input,
-        )?);
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_policy_input::ser_untag_policy_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/untag_role/_untag_role_input.rs`

```diff
--- reference/src/operation/untag_role/_untag_role_input.rs
+++ generated/src/operation/untag_role/_untag_role_input.rs
@@ -76,7 +76,9 @@
         &self.tag_keys
     }
     /// Consumes the builder and constructs a [`UntagRoleInput`](crate::operation::untag_role::UntagRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::untag_role::UntagRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::untag_role::UntagRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::untag_role::UntagRoleInput {
             role_name: self.role_name,
             tag_keys: self.tag_keys,
```

### `src/operation/untag_role/builders.rs`

```diff
--- reference/src/operation/untag_role/builders.rs
+++ generated/src/operation/untag_role/builders.rs
@@ -29,14 +29,20 @@
     inner: super::super::super::operation::untag_role::builders::UntagRoleInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::untag_role::UntagRoleOutput, super::super::super::operation::untag_role::UntagRoleError>
-    for UntagRoleFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::untag_role::UntagRoleOutput,
+        super::super::super::operation::untag_role::UntagRoleError,
+    > for UntagRoleFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::untag_role::UntagRoleOutput, super::super::super::operation::untag_role::UntagRoleError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::untag_role::UntagRoleOutput,
+            super::super::super::operation::untag_role::UntagRoleError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/untag_role.rs`

```diff
--- reference/src/operation/untag_role.rs
+++ generated/src/operation/untag_role.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::untag_role::UntagRoleInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::untag_role::UntagRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -245,11 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_role_input::ser_untag_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_role_input::ser_untag_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/untag_saml_provider/_untag_saml_provider_input.rs`

```diff
--- reference/src/operation/untag_saml_provider/_untag_saml_provider_input.rs
+++ generated/src/operation/untag_saml_provider/_untag_saml_provider_input.rs
@@ -78,7 +78,10 @@
     /// Consumes the builder and constructs a [`UntagSamlProviderInput`](crate::operation::untag_saml_provider::UntagSamlProviderInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::untag_saml_provider::UntagSamlProviderInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::untag_saml_provider::UntagSamlProviderInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::untag_saml_provider::UntagSamlProviderInput {
             saml_provider_arn: self.saml_provider_arn,
             tag_keys: self.tag_keys,
```

### `src/operation/untag_saml_provider/builders.rs`

```diff
--- reference/src/operation/untag_saml_provider/builders.rs
+++ generated/src/operation/untag_saml_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::untag_saml_provider::UntagSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::untag_saml_provider::UntagSAMLProviderError,
+            super::super::super::operation::untag_saml_provider::UntagSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UntagSAMLProvider`.
+/// Fluent builder constructing a request to `UntagSamlProvider`.
 ///
 /// <p>Removes the specified tags from the specified Security Assertion Markup Language (SAML) identity provider in IAM. For more information about these providers, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_oidc.html">About web identity federation</a>. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UntagSAMLProviderFluentBuilder {
+pub struct UntagSamlProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::untag_saml_provider::builders::UntagSamlProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::untag_saml_provider::UntagSamlProviderOutput,
-        super::super::super::operation::untag_saml_provider::UntagSAMLProviderError,
-    > for UntagSAMLProviderFluentBuilder
+        super::super::super::operation::untag_saml_provider::UntagSamlProviderError,
+    > for UntagSamlProviderFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::untag_saml_provider::UntagSamlProviderOutput,
-            super::super::super::operation::untag_saml_provider::UntagSAMLProviderError,
+            super::super::super::operation::untag_saml_provider::UntagSamlProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UntagSAMLProviderFluentBuilder {
-    /// Creates a new `UntagSAMLProviderFluentBuilder`.
+impl UntagSamlProviderFluentBuilder {
+    /// Creates a new `UntagSamlProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UntagSAMLProvider as a reference.
+    /// Access the UntagSamlProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::untag_saml_provider::builders::UntagSamlProviderInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::untag_saml_provider::UntagSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::untag_saml_provider::UntagSAMLProviderError,
+            super::super::super::operation::untag_saml_provider::UntagSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::untag_saml_provider::UntagSAMLProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::untag_saml_provider::UntagSamlProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::untag_saml_provider::UntagSAMLProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::untag_saml_provider::UntagSamlProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::untag_saml_provider::UntagSamlProviderOutput,
-        super::super::super::operation::untag_saml_provider::UntagSAMLProviderError,
+        super::super::super::operation::untag_saml_provider::UntagSamlProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/untag_saml_provider.rs`

```diff
--- reference/src/operation/untag_saml_provider.rs
+++ generated/src/operation/untag_saml_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UntagSAMLProvider`.
+/// Orchestration and serialization glue logic for `UntagSamlProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UntagSAMLProvider;
-impl UntagSAMLProvider {
-    /// Creates a new `UntagSAMLProvider`
+pub struct UntagSamlProvider;
+impl UntagSamlProvider {
+    /// Creates a new `UntagSamlProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UntagSAMLProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UntagSamlProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UntagSAMLProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UntagSAMLProviderRequestSerializer,
+            UntagSamlProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UntagSAMLProviderResponseDeserializer,
+            UntagSamlProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UntagSAMLProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UntagSAMLProviderTelemetryInputCaptureInterceptor,
+                UntagSamlProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UntagSAMLProviderEndpointParamsInterceptor,
+                UntagSamlProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::untag_saml_provider::UntagSAMLProviderError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct UntagSAMLProviderTelemetryInputCaptureInterceptor;
+struct UntagSamlProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagSAMLProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagSamlProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UntagSAMLProviderTelemetryInputCaptureInterceptor"
+        "UntagSamlProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_untag_saml_provider_input::ser_untag_saml_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_untag_saml_provider_input::ser_untag_saml_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -262,12 +261,12 @@
     }
 }
 #[derive(Debug)]
-struct UntagSAMLProviderEndpointParamsInterceptor;
+struct UntagSamlProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagSAMLProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UntagSamlProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UntagSAMLProviderEndpointParamsInterceptor"
+        "UntagSamlProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/untag_server_certificate.rs`

```diff
--- reference/src/operation/untag_server_certificate.rs
+++ generated/src/operation/untag_server_certificate.rs
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
-            super::super::protocol_serde::shape_untag_server_certificate_input::ser_untag_server_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_untag_server_certificate_input::ser_untag_server_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/untag_user/_untag_user_input.rs`

```diff
--- reference/src/operation/untag_user/_untag_user_input.rs
+++ generated/src/operation/untag_user/_untag_user_input.rs
@@ -76,7 +76,9 @@
         &self.tag_keys
     }
     /// Consumes the builder and constructs a [`UntagUserInput`](crate::operation::untag_user::UntagUserInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::untag_user::UntagUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::untag_user::UntagUserInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::untag_user::UntagUserInput {
             user_name: self.user_name,
             tag_keys: self.tag_keys,
```

### `src/operation/untag_user/builders.rs`

```diff
--- reference/src/operation/untag_user/builders.rs
+++ generated/src/operation/untag_user/builders.rs
@@ -29,14 +29,20 @@
     inner: super::super::super::operation::untag_user::builders::UntagUserInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::untag_user::UntagUserOutput, super::super::super::operation::untag_user::UntagUserError>
-    for UntagUserFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::untag_user::UntagUserOutput,
+        super::super::super::operation::untag_user::UntagUserError,
+    > for UntagUserFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::untag_user::UntagUserOutput, super::super::super::operation::untag_user::UntagUserError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::untag_user::UntagUserOutput,
+            super::super::super::operation::untag_user::UntagUserError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/untag_user.rs`

```diff
--- reference/src/operation/untag_user.rs
+++ generated/src/operation/untag_user.rs
@@ -220,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::untag_user::UntagUserInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::untag_user::UntagUserInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -245,11 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_user_input::ser_untag_user_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_user_input::ser_untag_user_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/update_access_key/_update_access_key_input.rs`

```diff
--- reference/src/operation/update_access_key/_update_access_key_input.rs
+++ generated/src/operation/update_access_key/_update_access_key_input.rs
@@ -97,7 +97,10 @@
     /// Consumes the builder and constructs a [`UpdateAccessKeyInput`](crate::operation::update_access_key::UpdateAccessKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_access_key::UpdateAccessKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_access_key::UpdateAccessKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_access_key::UpdateAccessKeyInput {
             user_name: self.user_name,
             access_key_id: self.access_key_id,
```

### `src/operation/update_access_key.rs`

```diff
--- reference/src/operation/update_access_key.rs
+++ generated/src/operation/update_access_key.rs
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_update_access_key_input::ser_update_access_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_access_key_input::ser_update_access_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_account_password_policy/_update_account_password_policy_input.rs`

```diff
--- reference/src/operation/update_account_password_policy/_update_account_password_policy_input.rs
+++ generated/src/operation/update_account_password_policy/_update_account_password_policy_input.rs
@@ -275,16 +275,18 @@
         super::super::super::operation::update_account_password_policy::UpdateAccountPasswordPolicyInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::update_account_password_policy::UpdateAccountPasswordPolicyInput {
-            minimum_password_length: self.minimum_password_length,
-            require_symbols: self.require_symbols,
-            require_numbers: self.require_numbers,
-            require_uppercase_characters: self.require_uppercase_characters,
-            require_lowercase_characters: self.require_lowercase_characters,
-            allow_users_to_change_password: self.allow_users_to_change_password,
-            max_password_age: self.max_password_age,
-            password_reuse_prevention: self.password_reuse_prevention,
-            hard_expiry: self.hard_expiry,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::update_account_password_policy::UpdateAccountPasswordPolicyInput {
+                minimum_password_length: self.minimum_password_length,
+                require_symbols: self.require_symbols.unwrap_or_default(),
+                require_numbers: self.require_numbers.unwrap_or_default(),
+                require_uppercase_characters: self.require_uppercase_characters.unwrap_or_default(),
+                require_lowercase_characters: self.require_lowercase_characters.unwrap_or_default(),
+                allow_users_to_change_password: self.allow_users_to_change_password.unwrap_or_default(),
+                max_password_age: self.max_password_age,
+                password_reuse_prevention: self.password_reuse_prevention,
+                hard_expiry: self.hard_expiry,
+            },
+        )
     }
 }
```

### `src/operation/update_account_password_policy.rs`

```diff
--- reference/src/operation/update_account_password_policy.rs
+++ generated/src/operation/update_account_password_policy.rs
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
-            super::super::protocol_serde::shape_update_account_password_policy_input::ser_update_account_password_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_account_password_policy_input::ser_update_account_password_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_assume_role_policy.rs`

```diff
--- reference/src/operation/update_assume_role_policy.rs
+++ generated/src/operation/update_assume_role_policy.rs
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
-            super::super::protocol_serde::shape_update_assume_role_policy_input::ser_update_assume_role_policy_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_assume_role_policy_input::ser_update_assume_role_policy_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_delegation_request.rs`

```diff
--- reference/src/operation/update_delegation_request.rs
+++ generated/src/operation/update_delegation_request.rs
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
-            super::super::protocol_serde::shape_update_delegation_request_input::ser_update_delegation_request_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_delegation_request_input::ser_update_delegation_request_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_group/_update_group_input.rs`

```diff
--- reference/src/operation/update_group/_update_group_input.rs
+++ generated/src/operation/update_group/_update_group_input.rs
@@ -99,7 +99,9 @@
         &self.new_group_name
     }
     /// Consumes the builder and constructs a [`UpdateGroupInput`](crate::operation::update_group::UpdateGroupInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::update_group::UpdateGroupInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::update_group::UpdateGroupInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::update_group::UpdateGroupInput {
             group_name: self.group_name,
             new_path: self.new_path,
```

### `src/operation/update_group.rs`

```diff
--- reference/src/operation/update_group.rs
+++ generated/src/operation/update_group.rs
@@ -257,13 +257,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_group_input::ser_update_group_input_input_input(
-            &input,
-        )?);
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_group_input::ser_update_group_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/update_login_profile/_update_login_profile_input.rs`

```diff
--- reference/src/operation/update_login_profile/_update_login_profile_input.rs
+++ generated/src/operation/update_login_profile/_update_login_profile_input.rs
@@ -150,8 +150,10 @@
     /// Consumes the builder and constructs a [`UpdateLoginProfileInput`](crate::operation::update_login_profile::UpdateLoginProfileInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_login_profile::UpdateLoginProfileInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_login_profile::UpdateLoginProfileInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_login_profile::UpdateLoginProfileInput {
             user_name: self.user_name,
             password: self.password,
```

### `src/operation/update_login_profile.rs`

```diff
--- reference/src/operation/update_login_profile.rs
+++ generated/src/operation/update_login_profile.rs
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
-            super::super::protocol_serde::shape_update_login_profile_input::ser_update_login_profile_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_login_profile_input::ser_update_login_profile_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_open_id_connect_provider_thumbprint/builders.rs`

```diff
--- reference/src/operation/update_open_id_connect_provider_thumbprint/builders.rs
+++ generated/src/operation/update_open_id_connect_provider_thumbprint/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError,
+            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,7 +20,7 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UpdateOpenIDConnectProviderThumbprint`.
+/// Fluent builder constructing a request to `UpdateOpenIdConnectProviderThumbprint`.
 ///
 /// <p>Replaces the existing list of server certificate thumbprints associated with an OpenID Connect (OIDC) provider resource object with a new list of thumbprints.</p>
 /// <p>The list that you pass with this operation completely replaces the existing list of thumbprints. (The lists are not merged.)</p>
@@ -30,7 +30,7 @@
 /// <p>Trust for the OIDC provider is derived from the provider certificate and is validated by the thumbprint. Therefore, it is best to limit access to the <code>UpdateOpenIDConnectProviderThumbprint</code> operation to highly privileged users.</p>
 /// </note>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UpdateOpenIDConnectProviderThumbprintFluentBuilder {
+pub struct UpdateOpenIdConnectProviderThumbprintFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::update_open_id_connect_provider_thumbprint::builders::UpdateOpenIdConnectProviderThumbprintInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -38,8 +38,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput,
-        super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError,
-    > for UpdateOpenIDConnectProviderThumbprintFluentBuilder
+        super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintError,
+    > for UpdateOpenIdConnectProviderThumbprintFluentBuilder
 {
     fn send(
         self,
@@ -47,14 +47,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput,
-            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError,
+            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UpdateOpenIDConnectProviderThumbprintFluentBuilder {
-    /// Creates a new `UpdateOpenIDConnectProviderThumbprintFluentBuilder`.
+impl UpdateOpenIdConnectProviderThumbprintFluentBuilder {
+    /// Creates a new `UpdateOpenIdConnectProviderThumbprintFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -62,10 +62,11 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UpdateOpenIDConnectProviderThumbprint as a reference.
+    /// Access the UpdateOpenIdConnectProviderThumbprint as a reference.
     pub fn as_input(
         &self,
-    ) -> &super::super::super::operation::update_open_id_connect_provider_thumbprint::builders::UpdateOpenIdConnectProviderThumbprintInputBuilder {
+    ) -> &super::super::super::operation::update_open_id_connect_provider_thumbprint::builders::UpdateOpenIdConnectProviderThumbprintInputBuilder
+    {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,7 +82,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError,
+            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -89,14 +90,16 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprint::operation_runtime_plugins(
-                self.handle.runtime_plugins.clone(),
-                &self.handle.conf,
-                self.config_override,
-            );
-        super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprint::orchestrate(&runtime_plugins, input)
-            .await
+        let runtime_plugins = super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprint::operation_runtime_plugins(
+                            self.handle.runtime_plugins.clone(),
+                            &self.handle.conf,
+                            self.config_override,
+                        );
+        super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprint::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -104,7 +107,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput,
-        super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError,
+        super::super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/update_open_id_connect_provider_thumbprint.rs`

```diff
--- reference/src/operation/update_open_id_connect_provider_thumbprint.rs
+++ generated/src/operation/update_open_id_connect_provider_thumbprint.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UpdateOpenIDConnectProviderThumbprint`.
+/// Orchestration and serialization glue logic for `UpdateOpenIdConnectProviderThumbprint`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UpdateOpenIDConnectProviderThumbprint;
-impl UpdateOpenIDConnectProviderThumbprint {
-    /// Creates a new `UpdateOpenIDConnectProviderThumbprint`
+pub struct UpdateOpenIdConnectProviderThumbprint;
+impl UpdateOpenIdConnectProviderThumbprint {
+    /// Creates a new `UpdateOpenIdConnectProviderThumbprint`
     pub fn new() -> Self {
         Self
     }
@@ -90,15 +90,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateOpenIDConnectProviderThumbprint {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateOpenIdConnectProviderThumbprint {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UpdateOpenIDConnectProviderThumbprint");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UpdateOpenIDConnectProviderThumbprintRequestSerializer,
+            UpdateOpenIdConnectProviderThumbprintRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UpdateOpenIDConnectProviderThumbprintResponseDeserializer,
+            UpdateOpenIdConnectProviderThumbprintResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -133,13 +133,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateOpenIDConnectProviderThumbprint")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateOpenIDConnectProviderThumbprintTelemetryInputCaptureInterceptor,
+                UpdateOpenIdConnectProviderThumbprintTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateOpenIDConnectProviderThumbprintEndpointParamsInterceptor,
+                UpdateOpenIdConnectProviderThumbprintEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError,
@@ -156,12 +156,12 @@
 }

 #[derive(Debug)]
-struct UpdateOpenIDConnectProviderThumbprintTelemetryInputCaptureInterceptor;
+struct UpdateOpenIdConnectProviderThumbprintTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateOpenIDConnectProviderThumbprintTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateOpenIdConnectProviderThumbprintTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UpdateOpenIDConnectProviderThumbprintTelemetryInputCaptureInterceptor"
+        "UpdateOpenIdConnectProviderThumbprintTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -217,9 +217,7 @@
                 status, headers, body,
             )
         } else {
-            super::super::protocol_serde::shape_update_open_id_connect_provider_thumbprint::de_update_open_id_connect_provider_thumbprint_http_response(
-                status, headers, body,
-            )
+            super::super::protocol_serde::shape_update_open_id_connect_provider_thumbprint::de_update_open_id_connect_provider_thumbprint_http_response(status, headers, body)
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
@@ -260,11 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_open_id_connect_provider_thumbprint_input::ser_update_open_id_connect_provider_thumbprint_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_open_id_connect_provider_thumbprint_input::ser_update_open_id_connect_provider_thumbprint_op_input(& input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -273,12 +270,12 @@
     }
 }
 #[derive(Debug)]
-struct UpdateOpenIDConnectProviderThumbprintEndpointParamsInterceptor;
+struct UpdateOpenIdConnectProviderThumbprintEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateOpenIDConnectProviderThumbprintEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateOpenIdConnectProviderThumbprintEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UpdateOpenIDConnectProviderThumbprintEndpointParamsInterceptor"
+        "UpdateOpenIdConnectProviderThumbprintEndpointParamsInterceptor"
     }

     fn read_before_execution(
@@ -441,7 +438,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError {
+impl ::aws_types::request_id::RequestId
+    for super::super::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIDConnectProviderThumbprintError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/update_role/_update_role_input.rs`

```diff
--- reference/src/operation/update_role/_update_role_input.rs
+++ generated/src/operation/update_role/_update_role_input.rs
@@ -99,7 +99,9 @@
         &self.max_session_duration
     }
     /// Consumes the builder and constructs a [`UpdateRoleInput`](crate::operation::update_role::UpdateRoleInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::update_role::UpdateRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::update_role::UpdateRoleInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::update_role::UpdateRoleInput {
             role_name: self.role_name,
             description: self.description,
```

### `src/operation/update_role.rs`

```diff
--- reference/src/operation/update_role.rs
+++ generated/src/operation/update_role.rs
@@ -225,7 +225,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::update_role::UpdateRoleInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::update_role::UpdateRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -250,12 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_role_input::ser_update_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_role_input::ser_update_role_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/update_role_description/_update_role_description_input.rs`

```diff
--- reference/src/operation/update_role_description/_update_role_description_input.rs
+++ generated/src/operation/update_role_description/_update_role_description_input.rs
@@ -66,8 +66,10 @@
     /// Consumes the builder and constructs a [`UpdateRoleDescriptionInput`](crate::operation::update_role_description::UpdateRoleDescriptionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_role_description::UpdateRoleDescriptionInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_role_description::UpdateRoleDescriptionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_role_description::UpdateRoleDescriptionInput {
             role_name: self.role_name,
             description: self.description,
```

### `src/operation/update_role_description.rs`

```diff
--- reference/src/operation/update_role_description.rs
+++ generated/src/operation/update_role_description.rs
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
-            super::super::protocol_serde::shape_update_role_description_input::ser_update_role_description_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_role_description_input::ser_update_role_description_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_saml_provider/_update_saml_provider_input.rs`

```diff
--- reference/src/operation/update_saml_provider/_update_saml_provider_input.rs
+++ generated/src/operation/update_saml_provider/_update_saml_provider_input.rs
@@ -144,8 +144,10 @@
     /// Consumes the builder and constructs a [`UpdateSamlProviderInput`](crate::operation::update_saml_provider::UpdateSamlProviderInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_saml_provider::UpdateSamlProviderInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_saml_provider::UpdateSamlProviderInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_saml_provider::UpdateSamlProviderInput {
             saml_metadata_document: self.saml_metadata_document,
             saml_provider_arn: self.saml_provider_arn,
```

### `src/operation/update_saml_provider/builders.rs`

```diff
--- reference/src/operation/update_saml_provider/builders.rs
+++ generated/src/operation/update_saml_provider/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::update_saml_provider::UpdateSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::update_saml_provider::UpdateSAMLProviderError,
+            super::super::super::operation::update_saml_provider::UpdateSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,11 +20,11 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UpdateSAMLProvider`.
+/// Fluent builder constructing a request to `UpdateSamlProvider`.
 ///
 /// <p>Updates the metadata document, SAML encryption settings, and private keys for an existing SAML provider. To rotate private keys, add your new private key and then remove the old key in a separate request.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UpdateSAMLProviderFluentBuilder {
+pub struct UpdateSamlProviderFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::update_saml_provider::builders::UpdateSamlProviderInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -32,8 +32,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::update_saml_provider::UpdateSamlProviderOutput,
-        super::super::super::operation::update_saml_provider::UpdateSAMLProviderError,
-    > for UpdateSAMLProviderFluentBuilder
+        super::super::super::operation::update_saml_provider::UpdateSamlProviderError,
+    > for UpdateSamlProviderFluentBuilder
 {
     fn send(
         self,
@@ -41,14 +41,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::update_saml_provider::UpdateSamlProviderOutput,
-            super::super::super::operation::update_saml_provider::UpdateSAMLProviderError,
+            super::super::super::operation::update_saml_provider::UpdateSamlProviderError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UpdateSAMLProviderFluentBuilder {
-    /// Creates a new `UpdateSAMLProviderFluentBuilder`.
+impl UpdateSamlProviderFluentBuilder {
+    /// Creates a new `UpdateSamlProviderFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -56,7 +56,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UpdateSAMLProvider as a reference.
+    /// Access the UpdateSamlProvider as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::update_saml_provider::builders::UpdateSamlProviderInputBuilder {
         &self.inner
     }
@@ -73,7 +73,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::update_saml_provider::UpdateSamlProviderOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::update_saml_provider::UpdateSAMLProviderError,
+            super::super::super::operation::update_saml_provider::UpdateSamlProviderError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -81,12 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::update_saml_provider::UpdateSAMLProvider::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::update_saml_provider::UpdateSamlProvider::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::update_saml_provider::UpdateSAMLProvider::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::update_saml_provider::UpdateSamlProvider::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -94,7 +94,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::update_saml_provider::UpdateSamlProviderOutput,
-        super::super::super::operation::update_saml_provider::UpdateSAMLProviderError,
+        super::super::super::operation::update_saml_provider::UpdateSamlProviderError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/update_saml_provider.rs`

```diff
--- reference/src/operation/update_saml_provider.rs
+++ generated/src/operation/update_saml_provider.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UpdateSAMLProvider`.
+/// Orchestration and serialization glue logic for `UpdateSamlProvider`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UpdateSAMLProvider;
-impl UpdateSAMLProvider {
-    /// Creates a new `UpdateSAMLProvider`
+pub struct UpdateSamlProvider;
+impl UpdateSamlProvider {
+    /// Creates a new `UpdateSamlProvider`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateSAMLProvider {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateSamlProvider {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UpdateSAMLProvider");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UpdateSAMLProviderRequestSerializer,
+            UpdateSamlProviderRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UpdateSAMLProviderResponseDeserializer,
+            UpdateSamlProviderResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateSAMLProvider")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateSAMLProviderTelemetryInputCaptureInterceptor,
+                UpdateSamlProviderTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateSAMLProviderEndpointParamsInterceptor,
+                UpdateSamlProviderEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::update_saml_provider::UpdateSAMLProviderError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct UpdateSAMLProviderTelemetryInputCaptureInterceptor;
+struct UpdateSamlProviderTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSAMLProviderTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSamlProviderTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UpdateSAMLProviderTelemetryInputCaptureInterceptor"
+        "UpdateSamlProviderTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
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
-            super::super::protocol_serde::shape_update_saml_provider_input::ser_update_saml_provider_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_saml_provider_input::ser_update_saml_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -272,12 +271,12 @@
     }
 }
 #[derive(Debug)]
-struct UpdateSAMLProviderEndpointParamsInterceptor;
+struct UpdateSamlProviderEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSAMLProviderEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSamlProviderEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UpdateSAMLProviderEndpointParamsInterceptor"
+        "UpdateSamlProviderEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/update_server_certificate.rs`

```diff
--- reference/src/operation/update_server_certificate.rs
+++ generated/src/operation/update_server_certificate.rs
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
-            super::super::protocol_serde::shape_update_server_certificate_input::ser_update_server_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_server_certificate_input::ser_update_server_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_service_specific_credential/builders.rs`

```diff
--- reference/src/operation/update_service_specific_credential/builders.rs
+++ generated/src/operation/update_service_specific_credential/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the UpdateServiceSpecificCredential as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::update_service_specific_credential::builders::UpdateServiceSpecificCredentialInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::update_service_specific_credential::builders::UpdateServiceSpecificCredentialInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::update_service_specific_credential::UpdateServiceSpecificCredential::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::update_service_specific_credential::UpdateServiceSpecificCredential::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::update_service_specific_credential::UpdateServiceSpecificCredential::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::update_service_specific_credential::UpdateServiceSpecificCredential::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/update_service_specific_credential.rs`

```diff
--- reference/src/operation/update_service_specific_credential.rs
+++ generated/src/operation/update_service_specific_credential.rs
@@ -218,7 +218,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_update_service_specific_credential::de_update_service_specific_credential_http_error(status, headers, body)
+            super::super::protocol_serde::shape_update_service_specific_credential::de_update_service_specific_credential_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_update_service_specific_credential::de_update_service_specific_credential_http_response(
                 status, headers, body,
@@ -263,12 +265,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_update_service_specific_credential_input::ser_update_service_specific_credential_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_service_specific_credential_input::ser_update_service_specific_credential_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_signing_certificate/_update_signing_certificate_input.rs`

```diff
--- reference/src/operation/update_signing_certificate/_update_signing_certificate_input.rs
+++ generated/src/operation/update_signing_certificate/_update_signing_certificate_input.rs
@@ -101,10 +101,12 @@
         super::super::super::operation::update_signing_certificate::UpdateSigningCertificateInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::update_signing_certificate::UpdateSigningCertificateInput {
-            user_name: self.user_name,
-            certificate_id: self.certificate_id,
-            status: self.status,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::update_signing_certificate::UpdateSigningCertificateInput {
+                user_name: self.user_name,
+                certificate_id: self.certificate_id,
+                status: self.status,
+            },
+        )
     }
 }
```

### `src/operation/update_signing_certificate.rs`

```diff
--- reference/src/operation/update_signing_certificate.rs
+++ generated/src/operation/update_signing_certificate.rs
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
-            super::super::protocol_serde::shape_update_signing_certificate_input::ser_update_signing_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_signing_certificate_input::ser_update_signing_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/update_ssh_public_key/_update_ssh_public_key_input.rs`

```diff
--- reference/src/operation/update_ssh_public_key/_update_ssh_public_key_input.rs
+++ generated/src/operation/update_ssh_public_key/_update_ssh_public_key_input.rs
@@ -98,8 +98,10 @@
     /// Consumes the builder and constructs a [`UpdateSshPublicKeyInput`](crate::operation::update_ssh_public_key::UpdateSshPublicKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyInput {
             user_name: self.user_name,
             ssh_public_key_id: self.ssh_public_key_id,
```

### `src/operation/update_ssh_public_key/builders.rs`

```diff
--- reference/src/operation/update_ssh_public_key/builders.rs
+++ generated/src/operation/update_ssh_public_key/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKeyError,
+            super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UpdateSSHPublicKey`.
+/// Fluent builder constructing a request to `UpdateSshPublicKey`.
 ///
 /// <p>Sets the status of an IAM user's SSH public key to active or inactive. SSH public keys that are inactive cannot be used for authentication. This operation can be used to disable a user's SSH public key as part of a key rotation work flow.</p>
 /// <p>The SSH public key affected by this operation is used only for authenticating the associated IAM user to an CodeCommit repository. For more information about using SSH keys to authenticate to an CodeCommit repository, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/setting-up-credentials-ssh.html">Set up CodeCommit for SSH connections</a> in the <i>CodeCommit User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UpdateSSHPublicKeyFluentBuilder {
+pub struct UpdateSshPublicKeyFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::update_ssh_public_key::builders::UpdateSshPublicKeyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyOutput,
-        super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKeyError,
-    > for UpdateSSHPublicKeyFluentBuilder
+        super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyError,
+    > for UpdateSshPublicKeyFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyOutput,
-            super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKeyError,
+            super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UpdateSSHPublicKeyFluentBuilder {
-    /// Creates a new `UpdateSSHPublicKeyFluentBuilder`.
+impl UpdateSshPublicKeyFluentBuilder {
+    /// Creates a new `UpdateSshPublicKeyFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UpdateSSHPublicKey as a reference.
+    /// Access the UpdateSshPublicKey as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::update_ssh_public_key::builders::UpdateSshPublicKeyInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKeyError,
+            super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKey::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::update_ssh_public_key::UpdateSshPublicKey::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKey::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::update_ssh_public_key::UpdateSshPublicKey::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyOutput,
-        super::super::super::operation::update_ssh_public_key::UpdateSSHPublicKeyError,
+        super::super::super::operation::update_ssh_public_key::UpdateSshPublicKeyError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/update_ssh_public_key.rs`

```diff
--- reference/src/operation/update_ssh_public_key.rs
+++ generated/src/operation/update_ssh_public_key.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UpdateSSHPublicKey`.
+/// Orchestration and serialization glue logic for `UpdateSshPublicKey`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UpdateSSHPublicKey;
-impl UpdateSSHPublicKey {
-    /// Creates a new `UpdateSSHPublicKey`
+pub struct UpdateSshPublicKey;
+impl UpdateSshPublicKey {
+    /// Creates a new `UpdateSshPublicKey`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateSSHPublicKey {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateSshPublicKey {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UpdateSSHPublicKey");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UpdateSSHPublicKeyRequestSerializer,
+            UpdateSshPublicKeyRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UpdateSSHPublicKeyResponseDeserializer,
+            UpdateSshPublicKeyResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateSSHPublicKey")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateSSHPublicKeyTelemetryInputCaptureInterceptor,
+                UpdateSshPublicKeyTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateSSHPublicKeyEndpointParamsInterceptor,
+                UpdateSshPublicKeyEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::update_ssh_public_key::UpdateSSHPublicKeyError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct UpdateSSHPublicKeyTelemetryInputCaptureInterceptor;
+struct UpdateSshPublicKeyTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSSHPublicKeyTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSshPublicKeyTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UpdateSSHPublicKeyTelemetryInputCaptureInterceptor"
+        "UpdateSshPublicKeyTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_update_ssh_public_key_input::ser_update_ssh_public_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_update_ssh_public_key_input::ser_update_ssh_public_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct UpdateSSHPublicKeyEndpointParamsInterceptor;
+struct UpdateSshPublicKeyEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSSHPublicKeyEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateSshPublicKeyEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UpdateSSHPublicKeyEndpointParamsInterceptor"
+        "UpdateSshPublicKeyEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation/update_user/_update_user_input.rs`

```diff
--- reference/src/operation/update_user/_update_user_input.rs
+++ generated/src/operation/update_user/_update_user_input.rs
@@ -99,7 +99,9 @@
         &self.new_user_name
     }
     /// Consumes the builder and constructs a [`UpdateUserInput`](crate::operation::update_user::UpdateUserInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::update_user::UpdateUserInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::update_user::UpdateUserInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::update_user::UpdateUserInput {
             user_name: self.user_name,
             new_path: self.new_path,
```

### `src/operation/update_user.rs`

```diff
--- reference/src/operation/update_user.rs
+++ generated/src/operation/update_user.rs
@@ -230,7 +230,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::update_user::UpdateUserInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::update_user::UpdateUserInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -255,12 +257,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_user_input::ser_update_user_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_user_input::ser_update_user_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/upload_server_certificate.rs`

```diff
--- reference/src/operation/upload_server_certificate.rs
+++ generated/src/operation/upload_server_certificate.rs
@@ -265,12 +265,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_upload_server_certificate_input::ser_upload_server_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_upload_server_certificate_input::ser_upload_server_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/upload_signing_certificate/_upload_signing_certificate_input.rs`

```diff
--- reference/src/operation/upload_signing_certificate/_upload_signing_certificate_input.rs
+++ generated/src/operation/upload_signing_certificate/_upload_signing_certificate_input.rs
@@ -119,9 +119,11 @@
         super::super::super::operation::upload_signing_certificate::UploadSigningCertificateInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::upload_signing_certificate::UploadSigningCertificateInput {
-            user_name: self.user_name,
-            certificate_body: self.certificate_body,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::upload_signing_certificate::UploadSigningCertificateInput {
+                user_name: self.user_name,
+                certificate_body: self.certificate_body,
+            },
+        )
     }
 }
```

### `src/operation/upload_signing_certificate.rs`

```diff
--- reference/src/operation/upload_signing_certificate.rs
+++ generated/src/operation/upload_signing_certificate.rs
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
-            super::super::protocol_serde::shape_upload_signing_certificate_input::ser_upload_signing_certificate_input_input_input(&input)?,
+            super::super::protocol_serde::shape_upload_signing_certificate_input::ser_upload_signing_certificate_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/upload_ssh_public_key/_upload_ssh_public_key_input.rs`

```diff
--- reference/src/operation/upload_ssh_public_key/_upload_ssh_public_key_input.rs
+++ generated/src/operation/upload_ssh_public_key/_upload_ssh_public_key_input.rs
@@ -116,8 +116,10 @@
     /// Consumes the builder and constructs a [`UploadSshPublicKeyInput`](crate::operation::upload_ssh_public_key::UploadSshPublicKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyInput {
             user_name: self.user_name,
             ssh_public_key_body: self.ssh_public_key_body,
```

### `src/operation/upload_ssh_public_key/builders.rs`

```diff
--- reference/src/operation/upload_ssh_public_key/builders.rs
+++ generated/src/operation/upload_ssh_public_key/builders.rs
@@ -11,7 +11,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKeyError,
+            super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -20,12 +20,12 @@
         fluent_builder.send().await
     }
 }
-/// Fluent builder constructing a request to `UploadSSHPublicKey`.
+/// Fluent builder constructing a request to `UploadSshPublicKey`.
 ///
 /// <p>Uploads an SSH public key and associates it with the specified IAM user.</p>
 /// <p>The SSH public key uploaded by this operation can be used only for authenticating the associated IAM user to an CodeCommit repository. For more information about using SSH keys to authenticate to an CodeCommit repository, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/setting-up-credentials-ssh.html">Set up CodeCommit for SSH connections</a> in the <i>CodeCommit User Guide</i>.</p>
 #[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct UploadSSHPublicKeyFluentBuilder {
+pub struct UploadSshPublicKeyFluentBuilder {
     handle: ::std::sync::Arc<super::super::super::client::Handle>,
     inner: super::super::super::operation::upload_ssh_public_key::builders::UploadSshPublicKeyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
@@ -33,8 +33,8 @@
 impl
     super::super::super::client::customize::internal::CustomizableSend<
         super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyOutput,
-        super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKeyError,
-    > for UploadSSHPublicKeyFluentBuilder
+        super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyError,
+    > for UploadSshPublicKeyFluentBuilder
 {
     fn send(
         self,
@@ -42,14 +42,14 @@
     ) -> super::super::super::client::customize::internal::BoxFuture<
         super::super::super::client::customize::internal::SendResult<
             super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyOutput,
-            super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKeyError,
+            super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyError,
         >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
 }
-impl UploadSSHPublicKeyFluentBuilder {
-    /// Creates a new `UploadSSHPublicKeyFluentBuilder`.
+impl UploadSshPublicKeyFluentBuilder {
+    /// Creates a new `UploadSshPublicKeyFluentBuilder`.
     pub(crate) fn new(handle: ::std::sync::Arc<super::super::super::client::Handle>) -> Self {
         Self {
             handle,
@@ -57,7 +57,7 @@
             config_override: ::std::option::Option::None,
         }
     }
-    /// Access the UploadSSHPublicKey as a reference.
+    /// Access the UploadSshPublicKey as a reference.
     pub fn as_input(&self) -> &super::super::super::operation::upload_ssh_public_key::builders::UploadSshPublicKeyInputBuilder {
         &self.inner
     }
@@ -74,7 +74,7 @@
     ) -> ::std::result::Result<
         super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyOutput,
         ::aws_smithy_runtime_api::client::result::SdkError<
-            super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKeyError,
+            super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyError,
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
@@ -82,12 +82,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKey::operation_runtime_plugins(
+        let runtime_plugins = super::super::super::operation::upload_ssh_public_key::UploadSshPublicKey::operation_runtime_plugins(
             self.handle.runtime_plugins.clone(),
             &self.handle.conf,
             self.config_override,
         );
-        super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKey::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::upload_ssh_public_key::UploadSshPublicKey::orchestrate(&runtime_plugins, input).await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -95,7 +95,7 @@
         self,
     ) -> super::super::super::client::customize::CustomizableOperation<
         super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyOutput,
-        super::super::super::operation::upload_ssh_public_key::UploadSSHPublicKeyError,
+        super::super::super::operation::upload_ssh_public_key::UploadSshPublicKeyError,
         Self,
     > {
         super::super::super::client::customize::CustomizableOperation::new(self)
```

### `src/operation/upload_ssh_public_key.rs`

```diff
--- reference/src/operation/upload_ssh_public_key.rs
+++ generated/src/operation/upload_ssh_public_key.rs
@@ -1,10 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-/// Orchestration and serialization glue logic for `UploadSSHPublicKey`.
+/// Orchestration and serialization glue logic for `UploadSshPublicKey`.
 #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
-pub struct UploadSSHPublicKey;
-impl UploadSSHPublicKey {
-    /// Creates a new `UploadSSHPublicKey`
+pub struct UploadSshPublicKey;
+impl UploadSshPublicKey {
+    /// Creates a new `UploadSshPublicKey`
     pub fn new() -> Self {
         Self
     }
@@ -84,15 +84,15 @@
         runtime_plugins
     }
 }
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UploadSSHPublicKey {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UploadSshPublicKey {
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UploadSSHPublicKey");

         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            UploadSSHPublicKeyRequestSerializer,
+            UploadSshPublicKeyRequestSerializer,
         ));
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            UploadSSHPublicKeyResponseDeserializer,
+            UploadSshPublicKeyResponseDeserializer,
         ));

         cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
@@ -124,13 +124,13 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UploadSSHPublicKey")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UploadSSHPublicKeyTelemetryInputCaptureInterceptor,
+                UploadSshPublicKeyTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UploadSSHPublicKeyEndpointParamsInterceptor,
+                UploadSshPublicKeyEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 super::super::operation::upload_ssh_public_key::UploadSSHPublicKeyError,
@@ -147,12 +147,12 @@
 }

 #[derive(Debug)]
-struct UploadSSHPublicKeyTelemetryInputCaptureInterceptor;
+struct UploadSshPublicKeyTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UploadSSHPublicKeyTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UploadSshPublicKeyTelemetryInputCaptureInterceptor {
     fn name(&self) -> &'static str {
-        "UploadSSHPublicKeyTelemetryInputCaptureInterceptor"
+        "UploadSshPublicKeyTelemetryInputCaptureInterceptor"
     }

     fn read_before_execution(
@@ -252,12 +252,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::super::protocol_serde::shape_upload_ssh_public_key_input::ser_upload_ssh_public_key_input_input_input(&input)?,
+            super::super::protocol_serde::shape_upload_ssh_public_key_input::ser_upload_ssh_public_key_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -267,12 +266,12 @@
     }
 }
 #[derive(Debug)]
-struct UploadSSHPublicKeyEndpointParamsInterceptor;
+struct UploadSshPublicKeyEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UploadSSHPublicKeyEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UploadSshPublicKeyEndpointParamsInterceptor {
     fn name(&self) -> &'static str {
-        "UploadSSHPublicKeyEndpointParamsInterceptor"
+        "UploadSshPublicKeyEndpointParamsInterceptor"
     }

     fn read_before_execution(
```

### `src/operation.rs`

```diff
--- reference/src/operation.rs
+++ generated/src/operation.rs
@@ -121,6 +121,9 @@
 /// Types for the `DeleteSAMLProvider` operation.
 pub mod delete_saml_provider;

+/// Types for the `DeleteSSHPublicKey` operation.
+pub mod delete_ssh_public_key;
+
 /// Types for the `DeleteServerCertificate` operation.
 pub mod delete_server_certificate;

@@ -133,9 +136,6 @@
 /// Types for the `DeleteSigningCertificate` operation.
 pub mod delete_signing_certificate;

-/// Types for the `DeleteSSHPublicKey` operation.
-pub mod delete_ssh_public_key;
-
 /// Types for the `DeleteUser` operation.
 pub mod delete_user;

@@ -259,6 +259,9 @@
 /// Types for the `GetSAMLProvider` operation.
 pub mod get_saml_provider;

+/// Types for the `GetSSHPublicKey` operation.
+pub mod get_ssh_public_key;
+
 /// Types for the `GetServerCertificate` operation.
 pub mod get_server_certificate;

@@ -271,9 +274,6 @@
 /// Types for the `GetServiceLinkedRoleDeletionStatus` operation.
 pub mod get_service_linked_role_deletion_status;

-/// Types for the `GetSSHPublicKey` operation.
-pub mod get_ssh_public_key;
-
 /// Types for the `GetUser` operation.
 pub mod get_user;

@@ -361,6 +361,9 @@
 /// Types for the `ListSAMLProviders` operation.
 pub mod list_saml_providers;

+/// Types for the `ListSSHPublicKeys` operation.
+pub mod list_ssh_public_keys;
+
 /// Types for the `ListServerCertificateTags` operation.
 pub mod list_server_certificate_tags;

@@ -373,9 +376,6 @@
 /// Types for the `ListSigningCertificates` operation.
 pub mod list_signing_certificates;

-/// Types for the `ListSSHPublicKeys` operation.
-pub mod list_ssh_public_keys;
-
 /// Types for the `ListUserPolicies` operation.
 pub mod list_user_policies;

@@ -517,6 +517,9 @@
 /// Types for the `UpdateSAMLProvider` operation.
 pub mod update_saml_provider;

+/// Types for the `UpdateSSHPublicKey` operation.
+pub mod update_ssh_public_key;
+
 /// Types for the `UpdateServerCertificate` operation.
 pub mod update_server_certificate;

@@ -526,17 +529,14 @@
 /// Types for the `UpdateSigningCertificate` operation.
 pub mod update_signing_certificate;

-/// Types for the `UpdateSSHPublicKey` operation.
-pub mod update_ssh_public_key;
-
 /// Types for the `UpdateUser` operation.
 pub mod update_user;

+/// Types for the `UploadSSHPublicKey` operation.
+pub mod upload_ssh_public_key;
+
 /// Types for the `UploadServerCertificate` operation.
 pub mod upload_server_certificate;

 /// Types for the `UploadSigningCertificate` operation.
 pub mod upload_signing_certificate;
-
-/// Types for the `UploadSSHPublicKey` operation.
-pub mod upload_ssh_public_key;
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -98,7 +98,7 @@
     if builder.password_policy.is_none() {
         builder.password_policy = {
             let builder = super::types::builders::PasswordPolicyBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
@@ -212,7 +212,7 @@
     if builder.role_template_version.is_none() {
         builder.role_template_version = {
             let builder = super::types::builders::RoleTemplateVersionBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
@@ -642,6 +642,24 @@
     builder
 }

+pub(crate) fn server_certificate_metadata_correct_errors(
+    mut builder: super::types::builders::ServerCertificateMetadataBuilder,
+) -> super::types::builders::ServerCertificateMetadataBuilder {
+    if builder.path.is_none() {
+        builder.path = Some(Default::default())
+    }
+    if builder.server_certificate_name.is_none() {
+        builder.server_certificate_name = Some(Default::default())
+    }
+    if builder.server_certificate_id.is_none() {
+        builder.server_certificate_id = Some(Default::default())
+    }
+    if builder.arn.is_none() {
+        builder.arn = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn user_correct_errors(mut builder: super::types::builders::UserBuilder) -> super::types::builders::UserBuilder {
     if builder.path.is_none() {
         builder.path = Some(Default::default())
@@ -701,24 +719,6 @@
     builder
 }

-pub(crate) fn server_certificate_metadata_correct_errors(
-    mut builder: super::types::builders::ServerCertificateMetadataBuilder,
-) -> super::types::builders::ServerCertificateMetadataBuilder {
-    if builder.path.is_none() {
-        builder.path = Some(Default::default())
-    }
-    if builder.server_certificate_name.is_none() {
-        builder.server_certificate_name = Some(Default::default())
-    }
-    if builder.server_certificate_id.is_none() {
-        builder.server_certificate_id = Some(Default::default())
-    }
-    if builder.arn.is_none() {
-        builder.arn = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn service_specific_credential_correct_errors(
     mut builder: super::types::builders::ServiceSpecificCredentialBuilder,
 ) -> super::types::builders::ServiceSpecificCredentialBuilder {
```

### `src/types/_delegation_permission.rs`

```diff
--- reference/src/types/_delegation_permission.rs
+++ generated/src/types/_delegation_permission.rs
@@ -4,13 +4,13 @@
 #[non_exhaustive]
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct DelegationPermission {
-    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <code>partner onboarding documentation</code> to understand how to create a delegation template.</p>
+    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <a href="">partner onboarding documentation</a> to understand how to create a delegation template.</p>
     pub policy_template_arn: ::std::option::Option<::std::string::String>,
     /// <p>A list of policy parameters that define the scope and constraints of the delegated permissions.</p>
     pub parameters: ::std::option::Option<::std::vec::Vec<super::super::types::PolicyParameter>>,
 }
 impl DelegationPermission {
-    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <code>partner onboarding documentation</code> to understand how to create a delegation template.</p>
+    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <a href="">partner onboarding documentation</a> to understand how to create a delegation template.</p>
     pub fn policy_template_arn(&self) -> ::std::option::Option<&str> {
         self.policy_template_arn.as_deref()
     }
@@ -36,17 +36,17 @@
     pub(crate) parameters: ::std::option::Option<::std::vec::Vec<super::super::types::PolicyParameter>>,
 }
 impl DelegationPermissionBuilder {
-    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <code>partner onboarding documentation</code> to understand how to create a delegation template.</p>
+    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <a href="">partner onboarding documentation</a> to understand how to create a delegation template.</p>
     pub fn policy_template_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.policy_template_arn = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <code>partner onboarding documentation</code> to understand how to create a delegation template.</p>
+    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <a href="">partner onboarding documentation</a> to understand how to create a delegation template.</p>
     pub fn set_policy_template_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.policy_template_arn = input;
         self
     }
-    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <code>partner onboarding documentation</code> to understand how to create a delegation template.</p>
+    /// <p>This ARN maps to a pre-registered policy content for this partner. See the <a href="">partner onboarding documentation</a> to understand how to create a delegation template.</p>
     pub fn get_policy_template_arn(&self) -> &::std::option::Option<::std::string::String> {
         &self.policy_template_arn
     }
```

### `src/types/_evaluation_result.rs`

```diff
--- reference/src/types/_evaluation_result.rs
+++ generated/src/types/_evaluation_result.rs
@@ -32,7 +32,8 @@
     /// <p>If the simulation evaluates policies within the same account and includes a resource ARN, then the parameter is present but the response is empty. If the simulation evaluates policies within the same account and specifies all resources (<code>*</code>), then the parameter is not returned.</p>
     /// <p>When you make a cross-account request, Amazon Web Services evaluates the request in the trusting account and the trusted account. The request is allowed only if both evaluations return <code>true</code>. For more information about how policies are evaluated, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic.html#policy-eval-basics">Evaluating policies within a single account</a>.</p>
     /// <p>If an Organizations SCP included in the evaluation denies access, the simulation ends. In this case, policy evaluation does not proceed any further and this parameter is not returned.</p>
-    pub eval_decision_details: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::PolicyEvaluationDecisionType>>,
+    pub eval_decision_details:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::PolicyEvaluationDecisionType>>,
     /// <p>The individual results of the simulation of the API operation specified in EvalActionName on each resource.</p>
     pub resource_specific_results: ::std::option::Option<::std::vec::Vec<super::super::types::ResourceSpecificResult>>,
 }
@@ -234,7 +235,10 @@
         self
     }
     /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when the boundary is applied to an IAM entity.</p>
-    pub fn set_permissions_boundary_decision_detail(mut self, input: ::std::option::Option<super::super::types::PermissionsBoundaryDecisionDetail>) -> Self {
+    pub fn set_permissions_boundary_decision_detail(
+        mut self,
+        input: ::std::option::Option<super::super::types::PermissionsBoundaryDecisionDetail>,
+    ) -> Self {
         self.permissions_boundary_decision_detail = input;
         self
     }
@@ -295,7 +299,10 @@
         self
     }
     /// <p>The individual results of the simulation of the API operation specified in EvalActionName on each resource.</p>
-    pub fn set_resource_specific_results(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::ResourceSpecificResult>>) -> Self {
+    pub fn set_resource_specific_results(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::ResourceSpecificResult>>,
+    ) -> Self {
         self.resource_specific_results = input;
         self
     }
```

### `src/types/_resource_specific_result.rs`

```diff
--- reference/src/types/_resource_specific_result.rs
+++ generated/src/types/_resource_specific_result.rs
@@ -14,7 +14,8 @@
     /// <p>A list of context keys that are required by the included input policies but that were not provided by one of the input parameters. This list is used when a list of ARNs is included in the <code>ResourceArns</code> parameter instead of "*". If you do not specify individual resources, by setting <code>ResourceArns</code> to "*" or by not including the <code>ResourceArns</code> parameter, then any missing context values are instead included under the <code>EvaluationResults</code> section. To discover the context keys used by a set of policies, you can call <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetContextKeysForCustomPolicy.html">GetContextKeysForCustomPolicy</a> or <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetContextKeysForPrincipalPolicy.html">GetContextKeysForPrincipalPolicy</a>.</p>
     pub missing_context_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
     /// <p>Additional details about the results of the evaluation decision on a single resource. This parameter is returned only for cross-account simulations. This parameter explains how each policy type contributes to the resource-specific evaluation decision.</p>
-    pub eval_decision_details: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::PolicyEvaluationDecisionType>>,
+    pub eval_decision_details:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::PolicyEvaluationDecisionType>>,
     /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when that boundary is applied to an IAM entity.</p>
     pub permissions_boundary_decision_detail: ::std::option::Option<super::super::types::PermissionsBoundaryDecisionDetail>,
 }
@@ -176,7 +177,10 @@
         self
     }
     /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when that boundary is applied to an IAM entity.</p>
-    pub fn set_permissions_boundary_decision_detail(mut self, input: ::std::option::Option<super::super::types::PermissionsBoundaryDecisionDetail>) -> Self {
+    pub fn set_permissions_boundary_decision_detail(
+        mut self,
+        input: ::std::option::Option<super::super::types::PermissionsBoundaryDecisionDetail>,
+    ) -> Self {
         self.permissions_boundary_decision_detail = input;
         self
     }
```

### `src/types/_service_specific_credential_metadata.rs`

```diff
--- reference/src/types/_service_specific_credential_metadata.rs
+++ generated/src/types/_service_specific_credential_metadata.rs
@@ -204,7 +204,9 @@
     /// - [`create_date`](crate::types::builders::ServiceSpecificCredentialMetadataBuilder::create_date)
     /// - [`service_specific_credential_id`](crate::types::builders::ServiceSpecificCredentialMetadataBuilder::service_specific_credential_id)
     /// - [`service_name`](crate::types::builders::ServiceSpecificCredentialMetadataBuilder::service_name)
-    pub fn build(self) -> ::std::result::Result<super::super::types::ServiceSpecificCredentialMetadata, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::ServiceSpecificCredentialMetadata, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::ServiceSpecificCredentialMetadata {
             user_name: self.user_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -63,6 +63,10 @@

 pub use super::super::super::types::error::_policy_evaluation_exception::PolicyEvaluationExceptionBuilder;

+pub use super::super::super::types::error::_duplicate_ssh_public_key_exception::DuplicateSshPublicKeyExceptionBuilder;
+
+pub use super::super::super::types::error::_invalid_public_key_exception::InvalidPublicKeyExceptionBuilder;
+
 pub use super::super::super::types::error::_key_pair_mismatch_exception::KeyPairMismatchExceptionBuilder;

 pub use super::super::super::types::error::_malformed_certificate_exception::MalformedCertificateExceptionBuilder;
@@ -70,7 +74,3 @@
 pub use super::super::super::types::error::_duplicate_certificate_exception::DuplicateCertificateExceptionBuilder;

 pub use super::super::super::types::error::_invalid_certificate_exception::InvalidCertificateExceptionBuilder;
-
-pub use super::super::super::types::error::_duplicate_ssh_public_key_exception::DuplicateSshPublicKeyExceptionBuilder;
-
-pub use super::super::super::types::error::_invalid_public_key_exception::InvalidPublicKeyExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -63,6 +63,10 @@

 pub use super::super::types::error::_policy_evaluation_exception::PolicyEvaluationException;

+pub use super::super::types::error::_duplicate_ssh_public_key_exception::DuplicateSshPublicKeyException;
+
+pub use super::super::types::error::_invalid_public_key_exception::InvalidPublicKeyException;
+
 pub use super::super::types::error::_key_pair_mismatch_exception::KeyPairMismatchException;

 pub use super::super::types::error::_malformed_certificate_exception::MalformedCertificateException;
@@ -71,10 +75,6 @@

 pub use super::super::types::error::_invalid_certificate_exception::InvalidCertificateException;

-pub use super::super::types::error::_duplicate_ssh_public_key_exception::DuplicateSshPublicKeyException;
-
-pub use super::super::types::error::_invalid_public_key_exception::InvalidPublicKeyException;
-
 mod _account_not_management_or_delegated_administrator_exception;

 mod _caller_is_not_management_account_exception;
```

### `src/waiters/policy_exists.rs`

```diff
--- reference/src/waiters/policy_exists.rs
+++ generated/src/waiters/policy_exists.rs
@@ -32,7 +32,10 @@
     pub async fn wait(
         self,
         max_wait: ::std::time::Duration,
-    ) -> ::std::result::Result<super::super::waiters::policy_exists::PolicyExistsFinalPoll, super::super::waiters::policy_exists::WaitUntilPolicyExistsError> {
+    ) -> ::std::result::Result<
+        super::super::waiters::policy_exists::PolicyExistsFinalPoll,
+        super::super::waiters::policy_exists::WaitUntilPolicyExistsError,
+    > {
         let input = self
             .inner
             .build()
@@ -51,18 +54,20 @@
         let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
         let time_source = time_components.time_source().expect("a time source is required by waiters");

-        let acceptor =
-            move |result: ::std::result::Result<&super::super::operation::get_policy::GetPolicyOutput, &super::super::operation::get_policy::GetPolicyError>| {
-                // Matches: {"success":true}
-                if super::super::waiters::matchers::match_get_policy_c955e57777ec0d736(result) {
-                    return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
-                }
-                // Matches: {"errorType":"NoSuchEntity"}
-                if super::super::waiters::matchers::match_get_policy_606386b4be9df73c9(result) {
-                    return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
-                }
-                ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
-            };
+        let acceptor = move |result: ::std::result::Result<
+            &super::super::operation::get_policy::GetPolicyOutput,
+            &super::super::operation::get_policy::GetPolicyError,
+        >| {
+            // Matches: {"success":true}
+            if super::super::waiters::matchers::match_get_policy_c955e57777ec0d736(result) {
+                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
+            }
+            // Matches: {"errorType":"NoSuchEntity"}
+            if super::super::waiters::matchers::match_get_policy_606386b4be9df73c9(result) {
+                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
+            }
+            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
+        };
         let operation = move || {
             let input = input.clone();
             let runtime_plugins = runtime_plugins.clone();
```

### `src/waiters/role_exists.rs`

```diff
--- reference/src/waiters/role_exists.rs
+++ generated/src/waiters/role_exists.rs
@@ -32,7 +32,8 @@
     pub async fn wait(
         self,
         max_wait: ::std::time::Duration,
-    ) -> ::std::result::Result<super::super::waiters::role_exists::RoleExistsFinalPoll, super::super::waiters::role_exists::WaitUntilRoleExistsError> {
+    ) -> ::std::result::Result<super::super::waiters::role_exists::RoleExistsFinalPoll, super::super::waiters::role_exists::WaitUntilRoleExistsError>
+    {
         let input = self
             .inner
             .build()
@@ -51,7 +52,10 @@
         let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
         let time_source = time_components.time_source().expect("a time source is required by waiters");

-        let acceptor = move |result: ::std::result::Result<&super::super::operation::get_role::GetRoleOutput, &super::super::operation::get_role::GetRoleError>| {
+        let acceptor = move |result: ::std::result::Result<
+            &super::super::operation::get_role::GetRoleOutput,
+            &super::super::operation::get_role::GetRoleError,
+        >| {
             // Matches: {"success":true}
             if super::super::waiters::matchers::match_get_role_c955e57777ec0d736(result) {
                 return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
```

### `src/waiters/user_exists.rs`

```diff
--- reference/src/waiters/user_exists.rs
+++ generated/src/waiters/user_exists.rs
@@ -32,7 +32,8 @@
     pub async fn wait(
         self,
         max_wait: ::std::time::Duration,
-    ) -> ::std::result::Result<super::super::waiters::user_exists::UserExistsFinalPoll, super::super::waiters::user_exists::WaitUntilUserExistsError> {
+    ) -> ::std::result::Result<super::super::waiters::user_exists::UserExistsFinalPoll, super::super::waiters::user_exists::WaitUntilUserExistsError>
+    {
         let input = self
             .inner
             .build()
@@ -51,7 +52,10 @@
         let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
         let time_source = time_components.time_source().expect("a time source is required by waiters");

-        let acceptor = move |result: ::std::result::Result<&super::super::operation::get_user::GetUserOutput, &super::super::operation::get_user::GetUserError>| {
+        let acceptor = move |result: ::std::result::Result<
+            &super::super::operation::get_user::GetUserOutput,
+            &super::super::operation::get_user::GetUserError,
+        >| {
             // Matches: {"success":true}
             if super::super::waiters::matchers::match_get_user_c955e57777ec0d736(result) {
                 return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
```

### Missing reference files

- `src/protocol_serde/shape_accept_delegation_request.rs`
- `src/protocol_serde/shape_accept_delegation_request_input.rs`
- `src/protocol_serde/shape_access_detail.rs`
- `src/protocol_serde/shape_access_details.rs`
- `src/protocol_serde/shape_access_key.rs`
- `src/protocol_serde/shape_access_key_last_used.rs`
- `src/protocol_serde/shape_access_key_metadata.rs`
- `src/protocol_serde/shape_access_key_metadata_list_type.rs`
- `src/protocol_serde/shape_account_alias_list_type.rs`
- `src/protocol_serde/shape_account_not_management_or_delegated_administrator_exception.rs`
- `src/protocol_serde/shape_account_properties_map_type.rs`
- `src/protocol_serde/shape_acquire_role.rs`
- `src/protocol_serde/shape_acquire_role_input.rs`
- `src/protocol_serde/shape_add_client_id_to_open_id_connect_provider.rs`
- `src/protocol_serde/shape_add_client_id_to_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_add_role_to_instance_profile.rs`
- `src/protocol_serde/shape_add_role_to_instance_profile_input.rs`
- `src/protocol_serde/shape_add_user_to_group.rs`
- `src/protocol_serde/shape_add_user_to_group_input.rs`
- `src/protocol_serde/shape_arn_list_type.rs`
- `src/protocol_serde/shape_associate_delegation_request.rs`
- `src/protocol_serde/shape_associate_delegation_request_input.rs`
- `src/protocol_serde/shape_attach_group_policy.rs`
- `src/protocol_serde/shape_attach_group_policy_input.rs`
- `src/protocol_serde/shape_attach_role_policy.rs`
- `src/protocol_serde/shape_attach_role_policy_input.rs`
- `src/protocol_serde/shape_attach_user_policy.rs`
- `src/protocol_serde/shape_attach_user_policy_input.rs`
- `src/protocol_serde/shape_attached_permissions_boundary.rs`
- `src/protocol_serde/shape_attached_policies_list_type.rs`
- `src/protocol_serde/shape_attached_policy.rs`
- `src/protocol_serde/shape_caller_is_not_management_account_exception.rs`
- `src/protocol_serde/shape_certificate_list_type.rs`
- `src/protocol_serde/shape_certification_map_type.rs`
- `src/protocol_serde/shape_change_password.rs`
- `src/protocol_serde/shape_change_password_input.rs`
- `src/protocol_serde/shape_client_id_list_type.rs`
- `src/protocol_serde/shape_concurrent_modification_exception.rs`
- `src/protocol_serde/shape_context_entry.rs`
- `src/protocol_serde/shape_context_key_names_result_list_type.rs`
- `src/protocol_serde/shape_create_access_key.rs`
- `src/protocol_serde/shape_create_access_key_input.rs`
- `src/protocol_serde/shape_create_account_alias.rs`
- `src/protocol_serde/shape_create_account_alias_input.rs`
- `src/protocol_serde/shape_create_delegation_request.rs`
- `src/protocol_serde/shape_create_delegation_request_input.rs`
- `src/protocol_serde/shape_create_group.rs`
- `src/protocol_serde/shape_create_group_input.rs`
- `src/protocol_serde/shape_create_instance_profile.rs`
- `src/protocol_serde/shape_create_instance_profile_input.rs`
- `src/protocol_serde/shape_create_login_profile.rs`
- `src/protocol_serde/shape_create_login_profile_input.rs`
- `src/protocol_serde/shape_create_open_id_connect_provider.rs`
- `src/protocol_serde/shape_create_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_create_policy.rs`
- `src/protocol_serde/shape_create_policy_input.rs`
- `src/protocol_serde/shape_create_policy_version.rs`
- `src/protocol_serde/shape_create_policy_version_input.rs`
- `src/protocol_serde/shape_create_role.rs`
- `src/protocol_serde/shape_create_role_input.rs`
- `src/protocol_serde/shape_create_saml_provider.rs`
- `src/protocol_serde/shape_create_saml_provider_input.rs`
- `src/protocol_serde/shape_create_service_linked_role.rs`
- `src/protocol_serde/shape_create_service_linked_role_input.rs`
- `src/protocol_serde/shape_create_service_specific_credential.rs`
- `src/protocol_serde/shape_create_service_specific_credential_input.rs`
- `src/protocol_serde/shape_create_user.rs`
- `src/protocol_serde/shape_create_user_input.rs`
- `src/protocol_serde/shape_create_virtual_mfa_device.rs`
- `src/protocol_serde/shape_create_virtual_mfa_device_input.rs`
- `src/protocol_serde/shape_credential_report_expired_exception.rs`
- `src/protocol_serde/shape_credential_report_not_present_exception.rs`
- `src/protocol_serde/shape_credential_report_not_ready_exception.rs`
- `src/protocol_serde/shape_deactivate_mfa_device.rs`
- `src/protocol_serde/shape_deactivate_mfa_device_input.rs`
- `src/protocol_serde/shape_delegation_permission.rs`
- `src/protocol_serde/shape_delegation_request.rs`
- `src/protocol_serde/shape_delegation_requests_list_type.rs`
- `src/protocol_serde/shape_delete_access_key.rs`
- `src/protocol_serde/shape_delete_access_key_input.rs`
- `src/protocol_serde/shape_delete_account_alias.rs`
- `src/protocol_serde/shape_delete_account_alias_input.rs`
- `src/protocol_serde/shape_delete_account_password_policy.rs`
- `src/protocol_serde/shape_delete_account_password_policy_input.rs`
- `src/protocol_serde/shape_delete_conflict_exception.rs`
- `src/protocol_serde/shape_delete_group.rs`
- `src/protocol_serde/shape_delete_group_input.rs`
- `src/protocol_serde/shape_delete_group_policy.rs`
- `src/protocol_serde/shape_delete_group_policy_input.rs`
- `src/protocol_serde/shape_delete_instance_profile.rs`
- `src/protocol_serde/shape_delete_instance_profile_input.rs`
- `src/protocol_serde/shape_delete_login_profile.rs`
- `src/protocol_serde/shape_delete_login_profile_input.rs`
- `src/protocol_serde/shape_delete_open_id_connect_provider.rs`
- `src/protocol_serde/shape_delete_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_delete_policy.rs`
- `src/protocol_serde/shape_delete_policy_input.rs`
- `src/protocol_serde/shape_delete_policy_version.rs`
- `src/protocol_serde/shape_delete_policy_version_input.rs`
- `src/protocol_serde/shape_delete_role.rs`
- `src/protocol_serde/shape_delete_role_input.rs`
- `src/protocol_serde/shape_delete_role_permissions_boundary.rs`
- `src/protocol_serde/shape_delete_role_permissions_boundary_input.rs`
- `src/protocol_serde/shape_delete_role_policy.rs`
- `src/protocol_serde/shape_delete_role_policy_input.rs`
- `src/protocol_serde/shape_delete_saml_provider.rs`
- `src/protocol_serde/shape_delete_saml_provider_input.rs`
- `src/protocol_serde/shape_delete_server_certificate.rs`
- `src/protocol_serde/shape_delete_server_certificate_input.rs`
- `src/protocol_serde/shape_delete_service_linked_role.rs`
- `src/protocol_serde/shape_delete_service_linked_role_input.rs`
- `src/protocol_serde/shape_delete_service_specific_credential.rs`
- `src/protocol_serde/shape_delete_service_specific_credential_input.rs`
- `src/protocol_serde/shape_delete_signing_certificate.rs`
- `src/protocol_serde/shape_delete_signing_certificate_input.rs`
- `src/protocol_serde/shape_delete_ssh_public_key.rs`
- `src/protocol_serde/shape_delete_ssh_public_key_input.rs`
- `src/protocol_serde/shape_delete_user.rs`
- `src/protocol_serde/shape_delete_user_input.rs`
- `src/protocol_serde/shape_delete_user_permissions_boundary.rs`
- `src/protocol_serde/shape_delete_user_permissions_boundary_input.rs`
- `src/protocol_serde/shape_delete_user_policy.rs`
- `src/protocol_serde/shape_delete_user_policy_input.rs`
- `src/protocol_serde/shape_delete_virtual_mfa_device.rs`
- `src/protocol_serde/shape_delete_virtual_mfa_device_input.rs`
- `src/protocol_serde/shape_deletion_task_failure_reason_type.rs`
- `src/protocol_serde/shape_detach_group_policy.rs`
- `src/protocol_serde/shape_detach_group_policy_input.rs`
- `src/protocol_serde/shape_detach_role_policy.rs`
- `src/protocol_serde/shape_detach_role_policy_input.rs`
- `src/protocol_serde/shape_detach_user_policy.rs`
- `src/protocol_serde/shape_detach_user_policy_input.rs`
- `src/protocol_serde/shape_disable_organizations_root_credentials_management.rs`
- `src/protocol_serde/shape_disable_organizations_root_credentials_management_input.rs`
- `src/protocol_serde/shape_disable_organizations_root_sessions.rs`
- `src/protocol_serde/shape_disable_organizations_root_sessions_input.rs`
- `src/protocol_serde/shape_disable_outbound_web_identity_federation.rs`
- `src/protocol_serde/shape_disable_outbound_web_identity_federation_input.rs`
- `src/protocol_serde/shape_duplicate_certificate_exception.rs`
- `src/protocol_serde/shape_duplicate_ssh_public_key_exception.rs`
- `src/protocol_serde/shape_enable_mfa_device.rs`
- `src/protocol_serde/shape_enable_mfa_device_input.rs`
- `src/protocol_serde/shape_enable_organizations_root_credentials_management.rs`
- `src/protocol_serde/shape_enable_organizations_root_credentials_management_input.rs`
- `src/protocol_serde/shape_enable_organizations_root_sessions.rs`
- `src/protocol_serde/shape_enable_organizations_root_sessions_input.rs`
- `src/protocol_serde/shape_enable_outbound_web_identity_federation.rs`
- `src/protocol_serde/shape_enable_outbound_web_identity_federation_input.rs`
- `src/protocol_serde/shape_entity_already_exists_exception.rs`
- `src/protocol_serde/shape_entity_details.rs`
- `src/protocol_serde/shape_entity_details_list_type.rs`
- `src/protocol_serde/shape_entity_info.rs`
- `src/protocol_serde/shape_entity_temporarily_unmodifiable_exception.rs`
- `src/protocol_serde/shape_error_details.rs`
- `src/protocol_serde/shape_eval_decision_details_type.rs`
- `src/protocol_serde/shape_evaluation_result.rs`
- `src/protocol_serde/shape_evaluation_results_list_type.rs`
- `src/protocol_serde/shape_feature_disabled_exception.rs`
- `src/protocol_serde/shape_feature_enabled_exception.rs`
- `src/protocol_serde/shape_features_list_type.rs`
- `src/protocol_serde/shape_generate_credential_report.rs`
- `src/protocol_serde/shape_generate_credential_report_input.rs`
- `src/protocol_serde/shape_generate_organizations_access_report.rs`
- `src/protocol_serde/shape_generate_organizations_access_report_input.rs`
- `src/protocol_serde/shape_generate_service_last_accessed_details.rs`
- `src/protocol_serde/shape_generate_service_last_accessed_details_input.rs`
- `src/protocol_serde/shape_get_access_key_last_used.rs`
- `src/protocol_serde/shape_get_access_key_last_used_input.rs`
- `src/protocol_serde/shape_get_account_authorization_details.rs`
- `src/protocol_serde/shape_get_account_authorization_details_input.rs`
- `src/protocol_serde/shape_get_account_password_policy.rs`
- `src/protocol_serde/shape_get_account_password_policy_input.rs`
- `src/protocol_serde/shape_get_account_properties.rs`
- `src/protocol_serde/shape_get_account_properties_input.rs`
- `src/protocol_serde/shape_get_account_summary.rs`
- `src/protocol_serde/shape_get_account_summary_input.rs`
- `src/protocol_serde/shape_get_context_keys_for_custom_policy.rs`
- `src/protocol_serde/shape_get_context_keys_for_custom_policy_input.rs`
- `src/protocol_serde/shape_get_context_keys_for_principal_policy.rs`
- `src/protocol_serde/shape_get_context_keys_for_principal_policy_input.rs`
- `src/protocol_serde/shape_get_credential_report.rs`
- `src/protocol_serde/shape_get_credential_report_input.rs`
- `src/protocol_serde/shape_get_delegation_request.rs`
- `src/protocol_serde/shape_get_delegation_request_input.rs`
- `src/protocol_serde/shape_get_group.rs`
- `src/protocol_serde/shape_get_group_input.rs`
- `src/protocol_serde/shape_get_group_policy.rs`
- `src/protocol_serde/shape_get_group_policy_input.rs`
- `src/protocol_serde/shape_get_human_readable_summary.rs`
- `src/protocol_serde/shape_get_human_readable_summary_input.rs`
- `src/protocol_serde/shape_get_instance_profile.rs`
- `src/protocol_serde/shape_get_instance_profile_input.rs`
- `src/protocol_serde/shape_get_login_profile.rs`
- `src/protocol_serde/shape_get_login_profile_input.rs`
- `src/protocol_serde/shape_get_mfa_device.rs`
- `src/protocol_serde/shape_get_mfa_device_input.rs`
- `src/protocol_serde/shape_get_open_id_connect_provider.rs`
- `src/protocol_serde/shape_get_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_get_organizations_access_report.rs`
- `src/protocol_serde/shape_get_organizations_access_report_input.rs`
- `src/protocol_serde/shape_get_outbound_web_identity_federation_info.rs`
- `src/protocol_serde/shape_get_outbound_web_identity_federation_info_input.rs`
- `src/protocol_serde/shape_get_policy.rs`
- `src/protocol_serde/shape_get_policy_input.rs`
- `src/protocol_serde/shape_get_policy_version.rs`
- `src/protocol_serde/shape_get_policy_version_input.rs`
- `src/protocol_serde/shape_get_role.rs`
- `src/protocol_serde/shape_get_role_input.rs`
- `src/protocol_serde/shape_get_role_policy.rs`
- `src/protocol_serde/shape_get_role_policy_input.rs`
- `src/protocol_serde/shape_get_role_template_version.rs`
- `src/protocol_serde/shape_get_role_template_version_input.rs`
- `src/protocol_serde/shape_get_saml_provider.rs`
- `src/protocol_serde/shape_get_saml_provider_input.rs`
- `src/protocol_serde/shape_get_server_certificate.rs`
- `src/protocol_serde/shape_get_server_certificate_input.rs`
- `src/protocol_serde/shape_get_service_last_accessed_details.rs`
- `src/protocol_serde/shape_get_service_last_accessed_details_input.rs`
- `src/protocol_serde/shape_get_service_last_accessed_details_with_entities.rs`
- `src/protocol_serde/shape_get_service_last_accessed_details_with_entities_input.rs`
- `src/protocol_serde/shape_get_service_linked_role_deletion_status.rs`
- `src/protocol_serde/shape_get_service_linked_role_deletion_status_input.rs`
- `src/protocol_serde/shape_get_ssh_public_key.rs`
- `src/protocol_serde/shape_get_ssh_public_key_input.rs`
- `src/protocol_serde/shape_get_user.rs`
- `src/protocol_serde/shape_get_user_input.rs`
- `src/protocol_serde/shape_get_user_policy.rs`
- `src/protocol_serde/shape_get_user_policy_input.rs`
- `src/protocol_serde/shape_group.rs`
- `src/protocol_serde/shape_group_detail.rs`
- `src/protocol_serde/shape_group_detail_list_type.rs`
- `src/protocol_serde/shape_group_list_type.rs`
- `src/protocol_serde/shape_group_name_list_type.rs`
- `src/protocol_serde/shape_inline_policy.rs`
- `src/protocol_serde/shape_inline_policy_identifier_type.rs`
- `src/protocol_serde/shape_inline_policy_template_list_type.rs`
- `src/protocol_serde/shape_instance_profile.rs`
- `src/protocol_serde/shape_instance_profile_list_type.rs`
- `src/protocol_serde/shape_invalid_authentication_code_exception.rs`
- `src/protocol_serde/shape_invalid_certificate_exception.rs`
- `src/protocol_serde/shape_invalid_input_exception.rs`
- `src/protocol_serde/shape_invalid_public_key_exception.rs`
- `src/protocol_serde/shape_invalid_user_type_exception.rs`
- `src/protocol_serde/shape_key_pair_mismatch_exception.rs`
- `src/protocol_serde/shape_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_list_access_keys.rs`
- `src/protocol_serde/shape_list_access_keys_input.rs`
- `src/protocol_serde/shape_list_account_aliases.rs`
- `src/protocol_serde/shape_list_account_aliases_input.rs`
- `src/protocol_serde/shape_list_attached_group_policies.rs`
- `src/protocol_serde/shape_list_attached_group_policies_input.rs`
- `src/protocol_serde/shape_list_attached_role_policies.rs`
- `src/protocol_serde/shape_list_attached_role_policies_input.rs`
- `src/protocol_serde/shape_list_attached_user_policies.rs`
- `src/protocol_serde/shape_list_attached_user_policies_input.rs`
- `src/protocol_serde/shape_list_delegation_requests.rs`
- `src/protocol_serde/shape_list_delegation_requests_input.rs`
- `src/protocol_serde/shape_list_entities_for_policy.rs`
- `src/protocol_serde/shape_list_entities_for_policy_input.rs`
- `src/protocol_serde/shape_list_group_policies.rs`
- `src/protocol_serde/shape_list_group_policies_input.rs`
- `src/protocol_serde/shape_list_groups.rs`
- `src/protocol_serde/shape_list_groups_for_user.rs`
- `src/protocol_serde/shape_list_groups_for_user_input.rs`
- `src/protocol_serde/shape_list_groups_input.rs`
- `src/protocol_serde/shape_list_instance_profile_tags.rs`
- `src/protocol_serde/shape_list_instance_profile_tags_input.rs`
- `src/protocol_serde/shape_list_instance_profiles.rs`
- `src/protocol_serde/shape_list_instance_profiles_for_role.rs`
- `src/protocol_serde/shape_list_instance_profiles_for_role_input.rs`
- `src/protocol_serde/shape_list_instance_profiles_input.rs`
- `src/protocol_serde/shape_list_mfa_device_tags.rs`
- `src/protocol_serde/shape_list_mfa_device_tags_input.rs`
- `src/protocol_serde/shape_list_mfa_devices.rs`
- `src/protocol_serde/shape_list_mfa_devices_input.rs`
- `src/protocol_serde/shape_list_open_id_connect_provider_tags.rs`
- `src/protocol_serde/shape_list_open_id_connect_provider_tags_input.rs`
- `src/protocol_serde/shape_list_open_id_connect_providers.rs`
- `src/protocol_serde/shape_list_open_id_connect_providers_input.rs`
- `src/protocol_serde/shape_list_organizations_features.rs`
- `src/protocol_serde/shape_list_organizations_features_input.rs`
- `src/protocol_serde/shape_list_policies.rs`
- `src/protocol_serde/shape_list_policies_granting_service_access.rs`
- `src/protocol_serde/shape_list_policies_granting_service_access_entry.rs`
- `src/protocol_serde/shape_list_policies_granting_service_access_input.rs`
- `src/protocol_serde/shape_list_policies_input.rs`
- `src/protocol_serde/shape_list_policy_granting_service_access_response_list_type.rs`
- `src/protocol_serde/shape_list_policy_tags.rs`
- `src/protocol_serde/shape_list_policy_tags_input.rs`
- `src/protocol_serde/shape_list_policy_versions.rs`
- `src/protocol_serde/shape_list_policy_versions_input.rs`
- `src/protocol_serde/shape_list_role_policies.rs`
- `src/protocol_serde/shape_list_role_policies_input.rs`
- `src/protocol_serde/shape_list_role_tags.rs`
- `src/protocol_serde/shape_list_role_tags_input.rs`
- `src/protocol_serde/shape_list_roles.rs`
- `src/protocol_serde/shape_list_roles_input.rs`
- `src/protocol_serde/shape_list_saml_provider_tags.rs`
- `src/protocol_serde/shape_list_saml_provider_tags_input.rs`
- `src/protocol_serde/shape_list_saml_providers.rs`
- `src/protocol_serde/shape_list_saml_providers_input.rs`
- `src/protocol_serde/shape_list_server_certificate_tags.rs`
- `src/protocol_serde/shape_list_server_certificate_tags_input.rs`
- `src/protocol_serde/shape_list_server_certificates.rs`
- `src/protocol_serde/shape_list_server_certificates_input.rs`
- `src/protocol_serde/shape_list_service_specific_credentials.rs`
- `src/protocol_serde/shape_list_service_specific_credentials_input.rs`
- `src/protocol_serde/shape_list_signing_certificates.rs`
- `src/protocol_serde/shape_list_signing_certificates_input.rs`
- `src/protocol_serde/shape_list_ssh_public_keys.rs`
- `src/protocol_serde/shape_list_ssh_public_keys_input.rs`
- `src/protocol_serde/shape_list_user_policies.rs`
- `src/protocol_serde/shape_list_user_policies_input.rs`
- `src/protocol_serde/shape_list_user_tags.rs`
- `src/protocol_serde/shape_list_user_tags_input.rs`
- `src/protocol_serde/shape_list_users.rs`
- `src/protocol_serde/shape_list_users_input.rs`
- `src/protocol_serde/shape_list_virtual_mfa_devices.rs`
- `src/protocol_serde/shape_list_virtual_mfa_devices_input.rs`
- `src/protocol_serde/shape_login_profile.rs`
- `src/protocol_serde/shape_malformed_certificate_exception.rs`
- `src/protocol_serde/shape_malformed_policy_document_exception.rs`
- `src/protocol_serde/shape_managed_policy_arn_list_type.rs`
- `src/protocol_serde/shape_managed_policy_detail.rs`
- `src/protocol_serde/shape_managed_policy_detail_list_type.rs`
- `src/protocol_serde/shape_mfa_device.rs`
- `src/protocol_serde/shape_mfa_device_list_type.rs`
- `src/protocol_serde/shape_name_conflict_exception.rs`
- `src/protocol_serde/shape_no_such_entity_exception.rs`
- `src/protocol_serde/shape_open_id_connect_provider_list_entry.rs`
- `src/protocol_serde/shape_open_id_connect_provider_list_type.rs`
- `src/protocol_serde/shape_open_id_idp_communication_error_exception.rs`
- `src/protocol_serde/shape_ordered_organization_policy_type.rs`
- `src/protocol_serde/shape_organization_not_found_exception.rs`
- `src/protocol_serde/shape_organization_not_in_all_features_mode_exception.rs`
- `src/protocol_serde/shape_organizations_decision_detail.rs`
- `src/protocol_serde/shape_parameter_definition.rs`
- `src/protocol_serde/shape_parameters_definition_list_type.rs`
- `src/protocol_serde/shape_password_policy.rs`
- `src/protocol_serde/shape_password_policy_violation_exception.rs`
- `src/protocol_serde/shape_permissions_boundary_decision_detail.rs`
- `src/protocol_serde/shape_policy.rs`
- `src/protocol_serde/shape_policy_detail.rs`
- `src/protocol_serde/shape_policy_detail_list_type.rs`
- `src/protocol_serde/shape_policy_document_version_list_type.rs`
- `src/protocol_serde/shape_policy_evaluation_exception.rs`
- `src/protocol_serde/shape_policy_granting_service_access.rs`
- `src/protocol_serde/shape_policy_granting_service_access_list_type.rs`
- `src/protocol_serde/shape_policy_group.rs`
- `src/protocol_serde/shape_policy_group_list_type.rs`
- `src/protocol_serde/shape_policy_identifier.rs`
- `src/protocol_serde/shape_policy_list_type.rs`
- `src/protocol_serde/shape_policy_name_list_type.rs`
- `src/protocol_serde/shape_policy_not_attachable_exception.rs`
- `src/protocol_serde/shape_policy_parameter.rs`
- `src/protocol_serde/shape_policy_parameter_list_type.rs`
- `src/protocol_serde/shape_policy_parameter_values_list_type.rs`
- `src/protocol_serde/shape_policy_role.rs`
- `src/protocol_serde/shape_policy_role_list_type.rs`
- `src/protocol_serde/shape_policy_user.rs`
- `src/protocol_serde/shape_policy_user_list_type.rs`
- `src/protocol_serde/shape_policy_version.rs`
- `src/protocol_serde/shape_position.rs`
- `src/protocol_serde/shape_private_key_list.rs`
- `src/protocol_serde/shape_put_account_properties.rs`
- `src/protocol_serde/shape_put_account_properties_input.rs`
- `src/protocol_serde/shape_put_group_policy.rs`
- `src/protocol_serde/shape_put_group_policy_input.rs`
- `src/protocol_serde/shape_put_role_permissions_boundary.rs`
- `src/protocol_serde/shape_put_role_permissions_boundary_input.rs`
- `src/protocol_serde/shape_put_role_policy.rs`
- `src/protocol_serde/shape_put_role_policy_input.rs`
- `src/protocol_serde/shape_put_user_permissions_boundary.rs`
- `src/protocol_serde/shape_put_user_permissions_boundary_input.rs`
- `src/protocol_serde/shape_put_user_policy.rs`
- `src/protocol_serde/shape_put_user_policy_input.rs`
- `src/protocol_serde/shape_reject_delegation_request.rs`
- `src/protocol_serde/shape_reject_delegation_request_input.rs`
- `src/protocol_serde/shape_remove_client_id_from_open_id_connect_provider.rs`
- `src/protocol_serde/shape_remove_client_id_from_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_remove_role_from_instance_profile.rs`
- `src/protocol_serde/shape_remove_role_from_instance_profile_input.rs`
- `src/protocol_serde/shape_remove_user_from_group.rs`
- `src/protocol_serde/shape_remove_user_from_group_input.rs`
- `src/protocol_serde/shape_replacement_value_entry.rs`
- `src/protocol_serde/shape_report_generation_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_reset_service_specific_credential.rs`
- `src/protocol_serde/shape_reset_service_specific_credential_input.rs`
- `src/protocol_serde/shape_resource_specific_result.rs`
- `src/protocol_serde/shape_resource_specific_result_list_type.rs`
- `src/protocol_serde/shape_resync_mfa_device.rs`
- `src/protocol_serde/shape_resync_mfa_device_input.rs`
- `src/protocol_serde/shape_role.rs`
- `src/protocol_serde/shape_role_detail.rs`
- `src/protocol_serde/shape_role_detail_list_type.rs`
- `src/protocol_serde/shape_role_last_used.rs`
- `src/protocol_serde/shape_role_list_type.rs`
- `src/protocol_serde/shape_role_modified_exception.rs`
- `src/protocol_serde/shape_role_permission_restriction_arn_list_type.rs`
- `src/protocol_serde/shape_role_template_disabled_exception.rs`
- `src/protocol_serde/shape_role_template_version.rs`
- `src/protocol_serde/shape_role_usage_list_type.rs`
- `src/protocol_serde/shape_role_usage_type.rs`
- `src/protocol_serde/shape_saml_private_key.rs`
- `src/protocol_serde/shape_saml_provider_list_entry.rs`
- `src/protocol_serde/shape_saml_provider_list_type.rs`
- `src/protocol_serde/shape_send_delegation_token.rs`
- `src/protocol_serde/shape_send_delegation_token_input.rs`
- `src/protocol_serde/shape_server_certificate.rs`
- `src/protocol_serde/shape_server_certificate_metadata.rs`
- `src/protocol_serde/shape_server_certificate_metadata_list_type.rs`
- `src/protocol_serde/shape_service_access_not_enabled_exception.rs`
- `src/protocol_serde/shape_service_failure_exception.rs`
- `src/protocol_serde/shape_service_last_accessed.rs`
- `src/protocol_serde/shape_service_not_supported_exception.rs`
- `src/protocol_serde/shape_service_specific_credential.rs`
- `src/protocol_serde/shape_service_specific_credential_metadata.rs`
- `src/protocol_serde/shape_service_specific_credentials_list_type.rs`
- `src/protocol_serde/shape_services_last_accessed.rs`
- `src/protocol_serde/shape_set_default_policy_version.rs`
- `src/protocol_serde/shape_set_default_policy_version_input.rs`
- `src/protocol_serde/shape_set_security_token_service_preferences.rs`
- `src/protocol_serde/shape_set_security_token_service_preferences_input.rs`
- `src/protocol_serde/shape_signing_certificate.rs`
- `src/protocol_serde/shape_simulate_custom_policy.rs`
- `src/protocol_serde/shape_simulate_custom_policy_input.rs`
- `src/protocol_serde/shape_simulate_principal_policy.rs`
- `src/protocol_serde/shape_simulate_principal_policy_input.rs`
- `src/protocol_serde/shape_source_role_template.rs`
- `src/protocol_serde/shape_ssh_public_key.rs`
- `src/protocol_serde/shape_ssh_public_key_list_type.rs`
- `src/protocol_serde/shape_ssh_public_key_metadata.rs`
- `src/protocol_serde/shape_statement.rs`
- `src/protocol_serde/shape_statement_list_type.rs`
- `src/protocol_serde/shape_summary_map_type.rs`
- `src/protocol_serde/shape_tag.rs`
- `src/protocol_serde/shape_tag_instance_profile.rs`
- `src/protocol_serde/shape_tag_instance_profile_input.rs`
- `src/protocol_serde/shape_tag_list_type.rs`
- `src/protocol_serde/shape_tag_mfa_device.rs`
- `src/protocol_serde/shape_tag_mfa_device_input.rs`
- `src/protocol_serde/shape_tag_open_id_connect_provider.rs`
- `src/protocol_serde/shape_tag_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_tag_policy.rs`
- `src/protocol_serde/shape_tag_policy_input.rs`
- `src/protocol_serde/shape_tag_role.rs`
- `src/protocol_serde/shape_tag_role_input.rs`
- `src/protocol_serde/shape_tag_saml_provider.rs`
- `src/protocol_serde/shape_tag_saml_provider_input.rs`
- `src/protocol_serde/shape_tag_server_certificate.rs`
- `src/protocol_serde/shape_tag_server_certificate_input.rs`
- `src/protocol_serde/shape_tag_template.rs`
- `src/protocol_serde/shape_tag_template_list_type.rs`
- `src/protocol_serde/shape_tag_user.rs`
- `src/protocol_serde/shape_tag_user_input.rs`
- `src/protocol_serde/shape_thumbprint_list_type.rs`
- `src/protocol_serde/shape_tracked_action_last_accessed.rs`
- `src/protocol_serde/shape_tracked_actions_last_accessed.rs`
- `src/protocol_serde/shape_unmodifiable_entity_exception.rs`
- `src/protocol_serde/shape_unrecognized_public_key_encoding_exception.rs`
- `src/protocol_serde/shape_untag_instance_profile.rs`
- `src/protocol_serde/shape_untag_instance_profile_input.rs`
- `src/protocol_serde/shape_untag_mfa_device.rs`
- `src/protocol_serde/shape_untag_mfa_device_input.rs`
- `src/protocol_serde/shape_untag_open_id_connect_provider.rs`
- `src/protocol_serde/shape_untag_open_id_connect_provider_input.rs`
- `src/protocol_serde/shape_untag_policy.rs`
- `src/protocol_serde/shape_untag_policy_input.rs`
- `src/protocol_serde/shape_untag_role.rs`
- `src/protocol_serde/shape_untag_role_input.rs`
- `src/protocol_serde/shape_untag_saml_provider.rs`
- `src/protocol_serde/shape_untag_saml_provider_input.rs`
- `src/protocol_serde/shape_untag_server_certificate.rs`
- `src/protocol_serde/shape_untag_server_certificate_input.rs`
- `src/protocol_serde/shape_untag_user.rs`
- `src/protocol_serde/shape_untag_user_input.rs`
- `src/protocol_serde/shape_update_access_key.rs`
- `src/protocol_serde/shape_update_access_key_input.rs`
- `src/protocol_serde/shape_update_account_password_policy.rs`
- `src/protocol_serde/shape_update_account_password_policy_input.rs`
- `src/protocol_serde/shape_update_assume_role_policy.rs`
- `src/protocol_serde/shape_update_assume_role_policy_input.rs`
- `src/protocol_serde/shape_update_delegation_request.rs`
- `src/protocol_serde/shape_update_delegation_request_input.rs`
- `src/protocol_serde/shape_update_group.rs`
- `src/protocol_serde/shape_update_group_input.rs`
- `src/protocol_serde/shape_update_login_profile.rs`
- `src/protocol_serde/shape_update_login_profile_input.rs`
- `src/protocol_serde/shape_update_open_id_connect_provider_thumbprint.rs`
- `src/protocol_serde/shape_update_open_id_connect_provider_thumbprint_input.rs`
- `src/protocol_serde/shape_update_role.rs`
- `src/protocol_serde/shape_update_role_description.rs`
- `src/protocol_serde/shape_update_role_description_input.rs`
- `src/protocol_serde/shape_update_role_input.rs`
- `src/protocol_serde/shape_update_saml_provider.rs`
- `src/protocol_serde/shape_update_saml_provider_input.rs`
- `src/protocol_serde/shape_update_server_certificate.rs`
- `src/protocol_serde/shape_update_server_certificate_input.rs`
- `src/protocol_serde/shape_update_service_specific_credential.rs`
- `src/protocol_serde/shape_update_service_specific_credential_input.rs`
- `src/protocol_serde/shape_update_signing_certificate.rs`
- `src/protocol_serde/shape_update_signing_certificate_input.rs`
- `src/protocol_serde/shape_update_ssh_public_key.rs`
- `src/protocol_serde/shape_update_ssh_public_key_input.rs`
- `src/protocol_serde/shape_update_user.rs`
- `src/protocol_serde/shape_update_user_input.rs`
- `src/protocol_serde/shape_upload_server_certificate.rs`
- `src/protocol_serde/shape_upload_server_certificate_input.rs`
- `src/protocol_serde/shape_upload_signing_certificate.rs`
- `src/protocol_serde/shape_upload_signing_certificate_input.rs`
- `src/protocol_serde/shape_upload_ssh_public_key.rs`
- `src/protocol_serde/shape_upload_ssh_public_key_input.rs`
- `src/protocol_serde/shape_user.rs`
- `src/protocol_serde/shape_user_detail.rs`
- `src/protocol_serde/shape_user_detail_list_type.rs`
- `src/protocol_serde/shape_user_list_type.rs`
- `src/protocol_serde/shape_virtual_mfa_device.rs`
- `src/protocol_serde/shape_virtual_mfa_device_list_type.rs`
- `src/protocol_serde.rs`
- `src/rest_xml_wrapped_errors.rs`
