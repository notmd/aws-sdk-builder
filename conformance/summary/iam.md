# AWS SDK Conformance Report: iam

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## iam
**Progress:** `1626/1626` files compared · `1602` matched · `24` mismatches · `0` missing · `0` extra · `98.52%` match (100.00% means fully matched)

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
```

### `src/operation/list_mfa_device_tags/builders.rs`

```diff
--- reference/src/operation/list_mfa_device_tags/builders.rs
+++ generated/src/operation/list_mfa_device_tags/builders.rs
@@ -110,9 +110,9 @@
     }
     /// Create a paginator for this request
     ///
-    /// Paginators are used by calling [`send().await`](crate::operation::list_mfa_device_tags::paginator::ListMfaDeviceTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_mfa_device_tags::paginator::ListMfaDeviceTagsPaginator {
-        super::super::super::operation::list_mfa_device_tags::paginator::ListMfaDeviceTagsPaginator::new(self.handle, self.inner)
+    /// Paginators are used by calling [`send().await`](crate::operation::list_mfa_device_tags::paginator::ListMFADeviceTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
+    pub fn into_paginator(self) -> super::super::super::operation::list_mfa_device_tags::paginator::ListMFADeviceTagsPaginator {
+        super::super::super::operation::list_mfa_device_tags::paginator::ListMFADeviceTagsPaginator::new(self.handle, self.inner)
     }
     /// <p>The unique identifier for the IAM virtual MFA device whose tags you want to see. For virtual MFA devices, the serial number is the same as the ARN.</p>
     /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
```

### `src/operation/list_mfa_device_tags/paginator.rs`

```diff
--- reference/src/operation/list_mfa_device_tags/paginator.rs
+++ generated/src/operation/list_mfa_device_tags/paginator.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 /// Paginator for [`ListMFADeviceTags`](crate::operation::list_mfa_device_tags::ListMFADeviceTags)
-pub struct ListMfaDeviceTagsPaginator {
+pub struct ListMFADeviceTagsPaginator {
     handle: std::sync::Arc<super::super::super::client::Handle>,
     builder: super::super::super::operation::list_mfa_device_tags::builders::ListMfaDeviceTagsInputBuilder,
     stop_on_duplicate_token: bool,
 }

-impl ListMfaDeviceTagsPaginator {
+impl ListMFADeviceTagsPaginator {
     /// Create a new paginator-wrapper
     pub(crate) fn new(
         handle: std::sync::Arc<super::super::super::client::Handle>,
@@ -31,8 +31,8 @@
     ///
     /// This paginator automatically flattens results using `tags`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> super::super::super::operation::list_mfa_device_tags::paginator::ListMfaDeviceTagsPaginatorItems {
-        super::super::super::operation::list_mfa_device_tags::paginator::ListMfaDeviceTagsPaginatorItems(self)
+    pub fn items(self) -> super::super::super::operation::list_mfa_device_tags::paginator::ListMFADeviceTagsPaginatorItems {
+        super::super::super::operation::list_mfa_device_tags::paginator::ListMFADeviceTagsPaginatorItems(self)
     }

     /// Stop paginating when the service returns the same pagination token twice in a row.
@@ -116,12 +116,12 @@
     }
 }

-/// Flattened paginator for `ListMfaDeviceTagsPaginator`
+/// Flattened paginator for `ListMFADeviceTagsPaginator`
 ///
-/// This is created with [`.items()`](ListMfaDeviceTagsPaginator::items)
-pub struct ListMfaDeviceTagsPaginatorItems(ListMfaDeviceTagsPaginator);
+/// This is created with [`.items()`](ListMFADeviceTagsPaginator::items)
+pub struct ListMFADeviceTagsPaginatorItems(ListMFADeviceTagsPaginator);

-impl ListMfaDeviceTagsPaginatorItems {
+impl ListMFADeviceTagsPaginatorItems {
     /// Create the pagination stream
     ///
     /// _Note_: No requests will be dispatched until the stream is used
```

### `src/operation/list_mfa_devices/builders.rs`

```diff
--- reference/src/operation/list_mfa_devices/builders.rs
+++ generated/src/operation/list_mfa_devices/builders.rs
@@ -111,9 +111,9 @@
     }
     /// Create a paginator for this request
     ///
-    /// Paginators are used by calling [`send().await`](crate::operation::list_mfa_devices::paginator::ListMfaDevicesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_mfa_devices::paginator::ListMfaDevicesPaginator {
-        super::super::super::operation::list_mfa_devices::paginator::ListMfaDevicesPaginator::new(self.handle, self.inner)
+    /// Paginators are used by calling [`send().await`](crate::operation::list_mfa_devices::paginator::ListMFADevicesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
+    pub fn into_paginator(self) -> super::super::super::operation::list_mfa_devices::paginator::ListMFADevicesPaginator {
+        super::super::super::operation::list_mfa_devices::paginator::ListMFADevicesPaginator::new(self.handle, self.inner)
     }
     /// <p>The name of the user whose MFA devices you want to list.</p>
     /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
```

### `src/operation/list_mfa_devices/paginator.rs`

```diff
--- reference/src/operation/list_mfa_devices/paginator.rs
+++ generated/src/operation/list_mfa_devices/paginator.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 /// Paginator for [`ListMFADevices`](crate::operation::list_mfa_devices::ListMFADevices)
-pub struct ListMfaDevicesPaginator {
+pub struct ListMFADevicesPaginator {
     handle: std::sync::Arc<super::super::super::client::Handle>,
     builder: super::super::super::operation::list_mfa_devices::builders::ListMfaDevicesInputBuilder,
     stop_on_duplicate_token: bool,
 }

-impl ListMfaDevicesPaginator {
+impl ListMFADevicesPaginator {
     /// Create a new paginator-wrapper
     pub(crate) fn new(
         handle: std::sync::Arc<super::super::super::client::Handle>,
@@ -31,8 +31,8 @@
     ///
     /// This paginator automatically flattens results using `mfa_devices`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> super::super::super::operation::list_mfa_devices::paginator::ListMfaDevicesPaginatorItems {
-        super::super::super::operation::list_mfa_devices::paginator::ListMfaDevicesPaginatorItems(self)
+    pub fn items(self) -> super::super::super::operation::list_mfa_devices::paginator::ListMFADevicesPaginatorItems {
+        super::super::super::operation::list_mfa_devices::paginator::ListMFADevicesPaginatorItems(self)
     }

     /// Stop paginating when the service returns the same pagination token twice in a row.
@@ -116,12 +116,12 @@
     }
 }

-/// Flattened paginator for `ListMfaDevicesPaginator`
+/// Flattened paginator for `ListMFADevicesPaginator`
 ///
-/// This is created with [`.items()`](ListMfaDevicesPaginator::items)
-pub struct ListMfaDevicesPaginatorItems(ListMfaDevicesPaginator);
+/// This is created with [`.items()`](ListMFADevicesPaginator::items)
+pub struct ListMFADevicesPaginatorItems(ListMFADevicesPaginator);

-impl ListMfaDevicesPaginatorItems {
+impl ListMFADevicesPaginatorItems {
     /// Create the pagination stream
     ///
     /// _Note_: No requests will be dispatched until the stream is used
```

### `src/operation/list_open_id_connect_provider_tags/builders.rs`

```diff
--- reference/src/operation/list_open_id_connect_provider_tags/builders.rs
+++ generated/src/operation/list_open_id_connect_provider_tags/builders.rs
@@ -111,9 +111,9 @@
     }
     /// Create a paginator for this request
     ///
-    /// Paginators are used by calling [`send().await`](crate::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator {
-        super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginator::new(self.handle, self.inner)
+    /// Paginators are used by calling [`send().await`](crate::operation::list_open_id_connect_provider_tags::paginator::ListOpenIDConnectProviderTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
+    pub fn into_paginator(self) -> super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIDConnectProviderTagsPaginator {
+        super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIDConnectProviderTagsPaginator::new(self.handle, self.inner)
     }
     /// <p>The ARN of the OpenID Connect (OIDC) identity provider whose tags you want to see.</p>
     /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
```

### `src/operation/list_open_id_connect_provider_tags/paginator.rs`

```diff
--- reference/src/operation/list_open_id_connect_provider_tags/paginator.rs
+++ generated/src/operation/list_open_id_connect_provider_tags/paginator.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 /// Paginator for [`ListOpenIDConnectProviderTags`](crate::operation::list_open_id_connect_provider_tags::ListOpenIDConnectProviderTags)
-pub struct ListOpenIdConnectProviderTagsPaginator {
+pub struct ListOpenIDConnectProviderTagsPaginator {
     handle: std::sync::Arc<super::super::super::client::Handle>,
     builder: super::super::super::operation::list_open_id_connect_provider_tags::builders::ListOpenIdConnectProviderTagsInputBuilder,
     stop_on_duplicate_token: bool,
 }

-impl ListOpenIdConnectProviderTagsPaginator {
+impl ListOpenIDConnectProviderTagsPaginator {
     /// Create a new paginator-wrapper
     pub(crate) fn new(
         handle: std::sync::Arc<super::super::super::client::Handle>,
@@ -31,8 +31,8 @@
     ///
     /// This paginator automatically flattens results using `tags`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginatorItems {
-        super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIdConnectProviderTagsPaginatorItems(self)
+    pub fn items(self) -> super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIDConnectProviderTagsPaginatorItems {
+        super::super::super::operation::list_open_id_connect_provider_tags::paginator::ListOpenIDConnectProviderTagsPaginatorItems(self)
     }

     /// Stop paginating when the service returns the same pagination token twice in a row.
@@ -120,12 +120,12 @@
     }
 }

-/// Flattened paginator for `ListOpenIdConnectProviderTagsPaginator`
+/// Flattened paginator for `ListOpenIDConnectProviderTagsPaginator`
 ///
-/// This is created with [`.items()`](ListOpenIdConnectProviderTagsPaginator::items)
-pub struct ListOpenIdConnectProviderTagsPaginatorItems(ListOpenIdConnectProviderTagsPaginator);
+/// This is created with [`.items()`](ListOpenIDConnectProviderTagsPaginator::items)
+pub struct ListOpenIDConnectProviderTagsPaginatorItems(ListOpenIDConnectProviderTagsPaginator);

-impl ListOpenIdConnectProviderTagsPaginatorItems {
+impl ListOpenIDConnectProviderTagsPaginatorItems {
     /// Create the pagination stream
     ///
     /// _Note_: No requests will be dispatched until the stream is used
```

### `src/operation/list_saml_provider_tags/builders.rs`

```diff
--- reference/src/operation/list_saml_provider_tags/builders.rs
+++ generated/src/operation/list_saml_provider_tags/builders.rs
@@ -111,9 +111,9 @@
     }
     /// Create a paginator for this request
     ///
-    /// Paginators are used by calling [`send().await`](crate::operation::list_saml_provider_tags::paginator::ListSamlProviderTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_saml_provider_tags::paginator::ListSamlProviderTagsPaginator {
-        super::super::super::operation::list_saml_provider_tags::paginator::ListSamlProviderTagsPaginator::new(self.handle, self.inner)
+    /// Paginators are used by calling [`send().await`](crate::operation::list_saml_provider_tags::paginator::ListSAMLProviderTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
+    pub fn into_paginator(self) -> super::super::super::operation::list_saml_provider_tags::paginator::ListSAMLProviderTagsPaginator {
+        super::super::super::operation::list_saml_provider_tags::paginator::ListSAMLProviderTagsPaginator::new(self.handle, self.inner)
     }
     /// <p>The ARN of the Security Assertion Markup Language (SAML) identity provider whose tags you want to see.</p>
     /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
```

### `src/operation/list_saml_provider_tags/paginator.rs`

```diff
--- reference/src/operation/list_saml_provider_tags/paginator.rs
+++ generated/src/operation/list_saml_provider_tags/paginator.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 /// Paginator for [`ListSAMLProviderTags`](crate::operation::list_saml_provider_tags::ListSAMLProviderTags)
-pub struct ListSamlProviderTagsPaginator {
+pub struct ListSAMLProviderTagsPaginator {
     handle: std::sync::Arc<super::super::super::client::Handle>,
     builder: super::super::super::operation::list_saml_provider_tags::builders::ListSamlProviderTagsInputBuilder,
     stop_on_duplicate_token: bool,
 }

-impl ListSamlProviderTagsPaginator {
+impl ListSAMLProviderTagsPaginator {
     /// Create a new paginator-wrapper
     pub(crate) fn new(
         handle: std::sync::Arc<super::super::super::client::Handle>,
@@ -31,8 +31,8 @@
     ///
     /// This paginator automatically flattens results using `tags`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> super::super::super::operation::list_saml_provider_tags::paginator::ListSamlProviderTagsPaginatorItems {
-        super::super::super::operation::list_saml_provider_tags::paginator::ListSamlProviderTagsPaginatorItems(self)
+    pub fn items(self) -> super::super::super::operation::list_saml_provider_tags::paginator::ListSAMLProviderTagsPaginatorItems {
+        super::super::super::operation::list_saml_provider_tags::paginator::ListSAMLProviderTagsPaginatorItems(self)
     }

     /// Stop paginating when the service returns the same pagination token twice in a row.
@@ -117,12 +117,12 @@
     }
 }

-/// Flattened paginator for `ListSamlProviderTagsPaginator`
+/// Flattened paginator for `ListSAMLProviderTagsPaginator`
 ///
-/// This is created with [`.items()`](ListSamlProviderTagsPaginator::items)
-pub struct ListSamlProviderTagsPaginatorItems(ListSamlProviderTagsPaginator);
+/// This is created with [`.items()`](ListSAMLProviderTagsPaginator::items)
+pub struct ListSAMLProviderTagsPaginatorItems(ListSAMLProviderTagsPaginator);

-impl ListSamlProviderTagsPaginatorItems {
+impl ListSAMLProviderTagsPaginatorItems {
     /// Create the pagination stream
     ///
     /// _Note_: No requests will be dispatched until the stream is used
```

### `src/operation/list_ssh_public_keys/builders.rs`

```diff
--- reference/src/operation/list_ssh_public_keys/builders.rs
+++ generated/src/operation/list_ssh_public_keys/builders.rs
@@ -112,9 +112,9 @@
     }
     /// Create a paginator for this request
     ///
-    /// Paginators are used by calling [`send().await`](crate::operation::list_ssh_public_keys::paginator::ListSshPublicKeysPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_ssh_public_keys::paginator::ListSshPublicKeysPaginator {
-        super::super::super::operation::list_ssh_public_keys::paginator::ListSshPublicKeysPaginator::new(self.handle, self.inner)
+    /// Paginators are used by calling [`send().await`](crate::operation::list_ssh_public_keys::paginator::ListSSHPublicKeysPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
+    pub fn into_paginator(self) -> super::super::super::operation::list_ssh_public_keys::paginator::ListSSHPublicKeysPaginator {
+        super::super::super::operation::list_ssh_public_keys::paginator::ListSSHPublicKeysPaginator::new(self.handle, self.inner)
     }
     /// <p>The name of the IAM user to list SSH public keys for. If none is specified, the <code>UserName</code> field is determined implicitly based on the Amazon Web Services access key used to sign the request.</p>
     /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
```

### `src/operation/list_ssh_public_keys/paginator.rs`

```diff
--- reference/src/operation/list_ssh_public_keys/paginator.rs
+++ generated/src/operation/list_ssh_public_keys/paginator.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 /// Paginator for [`ListSSHPublicKeys`](crate::operation::list_ssh_public_keys::ListSSHPublicKeys)
-pub struct ListSshPublicKeysPaginator {
+pub struct ListSSHPublicKeysPaginator {
     handle: std::sync::Arc<super::super::super::client::Handle>,
     builder: super::super::super::operation::list_ssh_public_keys::builders::ListSshPublicKeysInputBuilder,
     stop_on_duplicate_token: bool,
 }

-impl ListSshPublicKeysPaginator {
+impl ListSSHPublicKeysPaginator {
     /// Create a new paginator-wrapper
     pub(crate) fn new(
         handle: std::sync::Arc<super::super::super::client::Handle>,
@@ -31,8 +31,8 @@
     ///
     /// This paginator automatically flattens results using `ssh_public_keys`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> super::super::super::operation::list_ssh_public_keys::paginator::ListSshPublicKeysPaginatorItems {
-        super::super::super::operation::list_ssh_public_keys::paginator::ListSshPublicKeysPaginatorItems(self)
+    pub fn items(self) -> super::super::super::operation::list_ssh_public_keys::paginator::ListSSHPublicKeysPaginatorItems {
+        super::super::super::operation::list_ssh_public_keys::paginator::ListSSHPublicKeysPaginatorItems(self)
     }

     /// Stop paginating when the service returns the same pagination token twice in a row.
@@ -116,12 +116,12 @@
     }
 }

-/// Flattened paginator for `ListSshPublicKeysPaginator`
+/// Flattened paginator for `ListSSHPublicKeysPaginator`
 ///
-/// This is created with [`.items()`](ListSshPublicKeysPaginator::items)
-pub struct ListSshPublicKeysPaginatorItems(ListSshPublicKeysPaginator);
+/// This is created with [`.items()`](ListSSHPublicKeysPaginator::items)
+pub struct ListSSHPublicKeysPaginatorItems(ListSSHPublicKeysPaginator);

-impl ListSshPublicKeysPaginatorItems {
+impl ListSSHPublicKeysPaginatorItems {
     /// Create the pagination stream
     ///
     /// _Note_: No requests will be dispatched until the stream is used
```

### `src/operation/list_virtual_mfa_devices/builders.rs`

```diff
--- reference/src/operation/list_virtual_mfa_devices/builders.rs
+++ generated/src/operation/list_virtual_mfa_devices/builders.rs
@@ -113,9 +113,9 @@
     }
     /// Create a paginator for this request
     ///
-    /// Paginators are used by calling [`send().await`](crate::operation::list_virtual_mfa_devices::paginator::ListVirtualMfaDevicesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMfaDevicesPaginator {
-        super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMfaDevicesPaginator::new(self.handle, self.inner)
+    /// Paginators are used by calling [`send().await`](crate::operation::list_virtual_mfa_devices::paginator::ListVirtualMFADevicesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
+    pub fn into_paginator(self) -> super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMFADevicesPaginator {
+        super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMFADevicesPaginator::new(self.handle, self.inner)
     }
     /// <p>The status (<code>Unassigned</code> or <code>Assigned</code>) of the devices to list. If you do not specify an <code>AssignmentStatus</code>, the operation defaults to <code>Any</code>, which lists both assigned and unassigned virtual MFA devices.,</p>
     pub fn assignment_status(mut self, input: super::super::super::types::AssignmentStatusType) -> Self {
```

### `src/operation/list_virtual_mfa_devices/paginator.rs`

```diff
--- reference/src/operation/list_virtual_mfa_devices/paginator.rs
+++ generated/src/operation/list_virtual_mfa_devices/paginator.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 /// Paginator for [`ListVirtualMFADevices`](crate::operation::list_virtual_mfa_devices::ListVirtualMFADevices)
-pub struct ListVirtualMfaDevicesPaginator {
+pub struct ListVirtualMFADevicesPaginator {
     handle: std::sync::Arc<super::super::super::client::Handle>,
     builder: super::super::super::operation::list_virtual_mfa_devices::builders::ListVirtualMfaDevicesInputBuilder,
     stop_on_duplicate_token: bool,
 }

-impl ListVirtualMfaDevicesPaginator {
+impl ListVirtualMFADevicesPaginator {
     /// Create a new paginator-wrapper
     pub(crate) fn new(
         handle: std::sync::Arc<super::super::super::client::Handle>,
@@ -31,8 +31,8 @@
     ///
     /// This paginator automatically flattens results using `virtual_mfa_devices`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMfaDevicesPaginatorItems {
-        super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMfaDevicesPaginatorItems(self)
+    pub fn items(self) -> super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMFADevicesPaginatorItems {
+        super::super::super::operation::list_virtual_mfa_devices::paginator::ListVirtualMFADevicesPaginatorItems(self)
     }

     /// Stop paginating when the service returns the same pagination token twice in a row.
@@ -117,12 +117,12 @@
     }
 }

-/// Flattened paginator for `ListVirtualMfaDevicesPaginator`
+/// Flattened paginator for `ListVirtualMFADevicesPaginator`
 ///
-/// This is created with [`.items()`](ListVirtualMfaDevicesPaginator::items)
-pub struct ListVirtualMfaDevicesPaginatorItems(ListVirtualMfaDevicesPaginator);
+/// This is created with [`.items()`](ListVirtualMFADevicesPaginator::items)
+pub struct ListVirtualMFADevicesPaginatorItems(ListVirtualMFADevicesPaginator);

-impl ListVirtualMfaDevicesPaginatorItems {
+impl ListVirtualMFADevicesPaginatorItems {
     /// Create the pagination stream
     ///
     /// _Note_: No requests will be dispatched until the stream is used
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

### `src/protocol_serde/shape_get_credential_report.rs`

```diff
--- reference/src/protocol_serde/shape_get_credential_report.rs
+++ generated/src/protocol_serde/shape_get_credential_report.rs
@@ -143,10 +143,10 @@
             s if s.matches("Content") /* Content com.amazonaws.iam.synthetic#GetCredentialReportOutput$Content */ =>  {
                 let var_1 =
                     Some(
-                        ::aws_smithy_types::base64::decode(
+                        Result::<::aws_smithy_types::Blob, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                             ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
+                            .into()
                         )
-                        .map_err(|err|::aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid base64: {err:?}"))).map(::aws_smithy_types::Blob::new)
                         ?
                     )
                 ;
```

### `src/protocol_serde/shape_inline_policy_identifier_type.rs`

```diff
--- reference/src/protocol_serde/shape_inline_policy_identifier_type.rs
+++ generated/src/protocol_serde/shape_inline_policy_identifier_type.rs
@@ -12,7 +12,7 @@
     #[allow(unused_mut)]
     let mut scope_2 = writer.prefix("AttachmentType");
     {
-        scope_2.string(input.attachment_type.as_str());
+        scope_2.string(&input.attachment_type.as_str());
     }
     #[allow(unused_mut)]
     let mut scope_3 = writer.prefix("AttachmentName");
```

### `src/protocol_serde/shape_policy_identifier.rs`

```diff
--- reference/src/protocol_serde/shape_policy_identifier.rs
+++ generated/src/protocol_serde/shape_policy_identifier.rs
@@ -6,13 +6,25 @@
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::PolicyIdentifier::PolicyType(inner) => {
-            writer.string(inner.as_str());
+            #[allow(unused_mut)]
+            let mut scope_1 = writer.prefix("PolicyType");
+            {
+                scope_1.string(inner.as_str());
+            }
         }
         super::super::types::PolicyIdentifier::PolicyArn(inner) => {
-            writer.string(inner);
+            #[allow(unused_mut)]
+            let mut scope_2 = writer.prefix("PolicyArn");
+            {
+                scope_2.string(inner);
+            }
         }
         super::super::types::PolicyIdentifier::InlinePolicyIdentifier(inner) => {
-            super::super::protocol_serde::shape_inline_policy_identifier_type::ser_inline_policy_identifier_type(writer, inner)?;
+            #[allow(unused_mut)]
+            let mut scope_3 = writer.prefix("InlinePolicyIdentifier");
+            {
+                super::super::protocol_serde::shape_inline_policy_identifier_type::ser_inline_policy_identifier_type(scope_3, inner)?;
+            }
         }
         super::super::types::PolicyIdentifier::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_simulate_principal_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_simulate_principal_policy_input.rs
+++ generated/src/protocol_serde/shape_simulate_principal_policy_input.rs
@@ -39,77 +39,75 @@
         for item_15 in var_14 {
             #[allow(unused_mut)]
             let mut entry_17 = list_16.entry();
-            #[allow(unused_mut)]
-            let mut scope_18 = entry_17.prefix("member");
-            super::super::protocol_serde::shape_policy_identifier::ser_policy_identifier(scope_18, item_15)?;
+            super::super::protocol_serde::shape_policy_identifier::ser_policy_identifier(entry_17, item_15)?;
         }
         list_16.finish();
     }
     #[allow(unused_mut)]
-    let mut scope_19 = writer.prefix("ActionNames");
-    if let Some(var_20) = &input.action_names {
-        let mut list_22 = scope_19.start_list(false, None);
-        for item_21 in var_20 {
+    let mut scope_18 = writer.prefix("ActionNames");
+    if let Some(var_19) = &input.action_names {
+        let mut list_21 = scope_18.start_list(false, None);
+        for item_20 in var_19 {
             #[allow(unused_mut)]
-            let mut entry_23 = list_22.entry();
-            entry_23.string(item_21);
+            let mut entry_22 = list_21.entry();
+            entry_22.string(item_20);
         }
-        list_22.finish();
+        list_21.finish();
     }
     #[allow(unused_mut)]
-    let mut scope_24 = writer.prefix("ResourceArns");
-    if let Some(var_25) = &input.resource_arns {
-        let mut list_27 = scope_24.start_list(false, None);
-        for item_26 in var_25 {
+    let mut scope_23 = writer.prefix("ResourceArns");
+    if let Some(var_24) = &input.resource_arns {
+        let mut list_26 = scope_23.start_list(false, None);
+        for item_25 in var_24 {
             #[allow(unused_mut)]
-            let mut entry_28 = list_27.entry();
-            entry_28.string(item_26);
+            let mut entry_27 = list_26.entry();
+            entry_27.string(item_25);
         }
-        list_27.finish();
+        list_26.finish();
     }
     #[allow(unused_mut)]
-    let mut scope_29 = writer.prefix("ResourcePolicy");
-    if let Some(var_30) = &input.resource_policy {
-        scope_29.string(var_30);
+    let mut scope_28 = writer.prefix("ResourcePolicy");
+    if let Some(var_29) = &input.resource_policy {
+        scope_28.string(var_29);
     }
     #[allow(unused_mut)]
-    let mut scope_31 = writer.prefix("ResourceOwner");
-    if let Some(var_32) = &input.resource_owner {
-        scope_31.string(var_32);
+    let mut scope_30 = writer.prefix("ResourceOwner");
+    if let Some(var_31) = &input.resource_owner {
+        scope_30.string(var_31);
     }
     #[allow(unused_mut)]
-    let mut scope_33 = writer.prefix("CallerArn");
-    if let Some(var_34) = &input.caller_arn {
-        scope_33.string(var_34);
+    let mut scope_32 = writer.prefix("CallerArn");
+    if let Some(var_33) = &input.caller_arn {
+        scope_32.string(var_33);
     }
     #[allow(unused_mut)]
-    let mut scope_35 = writer.prefix("ContextEntries");
-    if let Some(var_36) = &input.context_entries {
-        let mut list_38 = scope_35.start_list(false, None);
-        for item_37 in var_36 {
+    let mut scope_34 = writer.prefix("ContextEntries");
+    if let Some(var_35) = &input.context_entries {
+        let mut list_37 = scope_34.start_list(false, None);
+        for item_36 in var_35 {
             #[allow(unused_mut)]
-            let mut entry_39 = list_38.entry();
-            super::super::protocol_serde::shape_context_entry::ser_context_entry(entry_39, item_37)?;
+            let mut entry_38 = list_37.entry();
+            super::super::protocol_serde::shape_context_entry::ser_context_entry(entry_38, item_36)?;
         }
-        list_38.finish();
+        list_37.finish();
     }
     #[allow(unused_mut)]
-    let mut scope_40 = writer.prefix("ResourceHandlingOption");
-    if let Some(var_41) = &input.resource_handling_option {
-        scope_40.string(var_41);
+    let mut scope_39 = writer.prefix("ResourceHandlingOption");
+    if let Some(var_40) = &input.resource_handling_option {
+        scope_39.string(var_40);
     }
     #[allow(unused_mut)]
-    let mut scope_42 = writer.prefix("MaxItems");
-    if let Some(var_43) = &input.max_items {
-        scope_42.number(
+    let mut scope_41 = writer.prefix("MaxItems");
+    if let Some(var_42) = &input.max_items {
+        scope_41.number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_43).into()),
+            ::aws_smithy_types::Number::NegInt((*var_42).into()),
         );
     }
     #[allow(unused_mut)]
-    let mut scope_44 = writer.prefix("Marker");
-    if let Some(var_45) = &input.marker {
-        scope_44.string(var_45);
+    let mut scope_43 = writer.prefix("Marker");
+    if let Some(var_44) = &input.marker {
+        scope_43.string(var_44);
     }
     writer.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
```

### `src/protocol_serde/shape_virtual_mfa_device.rs`

```diff
--- reference/src/protocol_serde/shape_virtual_mfa_device.rs
+++ generated/src/protocol_serde/shape_virtual_mfa_device.rs
@@ -27,10 +27,10 @@
             s if s.matches("Base32StringSeed") /* Base32StringSeed com.amazonaws.iam#VirtualMFADevice$Base32StringSeed */ =>  {
                 let var_2 =
                     Some(
-                        ::aws_smithy_types::base64::decode(
+                        Result::<::aws_smithy_types::Blob, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                             ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
+                            .into()
                         )
-                        .map_err(|err|::aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid base64: {err:?}"))).map(::aws_smithy_types::Blob::new)
                         ?
                     )
                 ;
@@ -40,10 +40,10 @@
             s if s.matches("QRCodePNG") /* QRCodePNG com.amazonaws.iam#VirtualMFADevice$QRCodePNG */ =>  {
                 let var_3 =
                     Some(
-                        ::aws_smithy_types::base64::decode(
+                        Result::<::aws_smithy_types::Blob, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                             ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
+                            .into()
                         )
-                        .map_err(|err|::aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid base64: {err:?}"))).map(::aws_smithy_types::Blob::new)
                         ?
                     )
                 ;
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -773,6 +773,8 @@

 pub(crate) mod shape_feature_enabled_exception;

+pub(crate) mod shape_inline_policy_identifier_type;
+
 pub(crate) mod shape_invalid_authentication_code_exception;

 pub(crate) mod shape_invalid_certificate_exception;
@@ -811,6 +813,8 @@

 pub(crate) mod shape_policy_not_attachable_exception;

+pub(crate) mod shape_policy_parameter;
+
 pub(crate) mod shape_replacement_value_entry;

 pub(crate) mod shape_report_generation_limit_exceeded_exception;
@@ -873,8 +877,6 @@

 pub(crate) mod shape_group_list_type;

-pub(crate) mod shape_inline_policy_identifier_type;
-
 pub(crate) mod shape_instance_profile;

 pub(crate) mod shape_instance_profile_list_type;
@@ -901,8 +903,6 @@

 pub(crate) mod shape_policy_name_list_type;

-pub(crate) mod shape_policy_parameter;
-
 pub(crate) mod shape_policy_role_list_type;

 pub(crate) mod shape_policy_user_list_type;
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
