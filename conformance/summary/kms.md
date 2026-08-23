# AWS SDK Conformance Report: kms

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## kms
**Progress:** `600/600` files compared · `280` matched · `115` mismatches · `204` missing · `1` extra · `46.67%` match (100.00% means fully matched)

### `src/client/cancel_key_deletion.rs`

```diff
--- reference/src/client/cancel_key_deletion.rs
+++ generated/src/client/cancel_key_deletion.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`CancelKeyDeletion`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key whose deletion is being canceled.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key whose deletion is being canceled.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`CancelKeyDeletionOutput`](crate::operation::cancel_key_deletion::CancelKeyDeletionOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::cancel_key_deletion::CancelKeyDeletionOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key whose deletion is canceled.</p>
     /// - On failure, responds with [`SdkError<CancelKeyDeletionError>`](crate::operation::cancel_key_deletion::CancelKeyDeletionError)
```

### `src/client/connect_custom_key_store.rs`

```diff
--- reference/src/client/connect_custom_key_store.rs
+++ generated/src/client/connect_custom_key_store.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`ConnectCustomKeyStore`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the key store ID of the custom key store that you want to connect. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the key store ID of the custom key store that you want to connect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p><br>
     /// - On success, responds with [`ConnectCustomKeyStoreOutput`](crate::operation::connect_custom_key_store::ConnectCustomKeyStoreOutput)
     /// - On failure, responds with [`SdkError<ConnectCustomKeyStoreError>`](crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError)
     pub fn connect_custom_key_store(&self) -> crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder {
```

### `src/client/create_alias.rs`

```diff
--- reference/src/client/create_alias.rs
+++ generated/src/client/create_alias.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`alias_name(impl Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::alias_name) / [`set_alias_name(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_alias_name):<br>required: **true**<br><p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed keys</a>.</p><br>
-    ///   - [`target_key_id(impl Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_target_key_id):<br>required: **true**<br><p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-mgn-key">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region.</p> <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p> <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/find-cmk-id-arn.html">Find the key ID and key ARN</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`target_key_id(impl Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_target_key_id):<br>required: **true**<br><p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-mgn-key">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region.</p> <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p> <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/find-cmk-id-arn.html">Find the key ID and key ARN</a> in the <i> <i>Key Management Service Developer Guide</i></i>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`CreateAliasOutput`](crate::operation::create_alias::CreateAliasOutput)
     /// - On failure, responds with [`SdkError<CreateAliasError>`](crate::operation::create_alias::CreateAliasError)
     pub fn create_alias(&self) -> crate::operation::create_alias::builders::CreateAliasFluentBuilder {
```

### `src/client/create_custom_key_store.rs`

```diff
--- reference/src/client/create_custom_key_store.rs
+++ generated/src/client/create_custom_key_store.rs
@@ -8,11 +8,11 @@
     ///   - [`trust_anchor_certificate(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::trust_anchor_certificate) / [`set_trust_anchor_certificate(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_trust_anchor_certificate):<br>required: **false**<br><p>Specifies the certificate for an CloudHSM key store. This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p> <p>Enter the content of the trust anchor certificate for the CloudHSM cluster. This is the content of the <code>customerCA.crt</code> file that you created when you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html">initialized the cluster</a>.</p><br>
     ///   - [`key_store_password(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::key_store_password) / [`set_key_store_password(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_key_store_password):<br>required: **false**<br><p>Specifies the <code>kmsuser</code> password for an CloudHSM key store. This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p> <p>Enter the password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-cloudhsm.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU) account</a> in the specified CloudHSM cluster. KMS logs into the cluster as this user to manage key material on your behalf.</p> <p>The password must be a string of 7 to 32 characters. Its value is case sensitive.</p> <p>This parameter tells KMS the <code>kmsuser</code> account password; it does not change the password in the CloudHSM cluster.</p><br>
     ///   - [`custom_key_store_type(CustomKeyStoreType)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::custom_key_store_type) / [`set_custom_key_store_type(Option<CustomKeyStoreType>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_custom_key_store_type):<br>required: **false**<br><p>Specifies the type of custom key store. The default value is <code>AWS_CLOUDHSM</code>.</p> <p>For a custom key store backed by an CloudHSM cluster, omit the parameter or enter <code>AWS_CLOUDHSM</code>. For a custom key store backed by an external key manager outside of Amazon Web Services, enter <code>EXTERNAL_KEY_STORE</code>. You cannot change this property after the key store is created.</p><br>
-    ///   - [`xks_proxy_uri_endpoint(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_uri_endpoint) / [`set_xks_proxy_uri_endpoint(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_uri_endpoint):<br>required: **false**<br><p>Specifies the endpoint that KMS uses to send requests to the external key store proxy (XKS proxy). This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p> <p>The protocol must be HTTPS. KMS communicates on port 443. Do not specify the port in the <code>XksProxyUriEndpoint</code> value.</p> <p>For external key stores with <code>XksProxyConnectivity</code> value of <code>VPC_ENDPOINT_SERVICE</code>, specify <code>https://</code> followed by the private DNS name of the VPC endpoint service.</p> <p>For external key stores with <code>PUBLIC_ENDPOINT</code> connectivity, this endpoint must be reachable before you create the custom key store. KMS connects to the external key store proxy while creating the custom key store. For external key stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity, KMS connects when you call the <code>ConnectCustomKeyStore</code> operation.</p> <p>The value of this parameter must begin with <code>https://</code>. The remainder can contain upper and lower case letters (A-Z and a-z), numbers (0-9), dots (<code>.</code>), and hyphens (<code>-</code>). Additional slashes (<code>/</code> and <code>\</code>) are not permitted.</p> <p><b>Uniqueness requirements: </b></p> <ul>  <li>   <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values must be unique in the Amazon Web Services account and Region.</p></li>  <li>   <p>An external key store with <code>PUBLIC_ENDPOINT</code> connectivity cannot use the same <code>XksProxyUriEndpoint</code> value as an external key store with <code>VPC_ENDPOINT_SERVICE</code> connectivity in this Amazon Web Services Region.</p></li>  <li>   <p>Each external key store with <code>VPC_ENDPOINT_SERVICE</code> connectivity must have its own private DNS name. The <code>XksProxyUriEndpoint</code> value for external key stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity (private DNS name) must be unique in the Amazon Web Services account and Region.</p></li> </ul><br>
+    ///   - [`xks_proxy_uri_endpoint(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_uri_endpoint) / [`set_xks_proxy_uri_endpoint(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_uri_endpoint):<br>required: **false**<br><p>Specifies the endpoint that KMS uses to send requests to the external key store proxy (XKS proxy). This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p> <p>The protocol must be HTTPS. KMS communicates on port 443. Do not specify the port in the <code>XksProxyUriEndpoint</code> value.</p> <p>For external key stores with <code>XksProxyConnectivity</code> value of <code>VPC_ENDPOINT_SERVICE</code>, specify <code>https://</code> followed by the private DNS name of the VPC endpoint service.</p> <p>For external key stores with <code>PUBLIC_ENDPOINT</code> connectivity, this endpoint must be reachable before you create the custom key store. KMS connects to the external key store proxy while creating the custom key store. For external key stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity, KMS connects when you call the <a>ConnectCustomKeyStore</a> operation.</p> <p>The value of this parameter must begin with <code>https://</code>. The remainder can contain upper and lower case letters (A-Z and a-z), numbers (0-9), dots (<code>.</code>), and hyphens (<code>-</code>). Additional slashes (<code>/</code> and <code>\</code>) are not permitted.</p> <p><b>Uniqueness requirements: </b></p> <ul>  <li>   <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values must be unique in the Amazon Web Services account and Region.</p></li>  <li>   <p>An external key store with <code>PUBLIC_ENDPOINT</code> connectivity cannot use the same <code>XksProxyUriEndpoint</code> value as an external key store with <code>VPC_ENDPOINT_SERVICE</code> connectivity in this Amazon Web Services Region.</p></li>  <li>   <p>Each external key store with <code>VPC_ENDPOINT_SERVICE</code> connectivity must have its own private DNS name. The <code>XksProxyUriEndpoint</code> value for external key stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity (private DNS name) must be unique in the Amazon Web Services account and Region.</p></li> </ul><br>
     ///   - [`xks_proxy_uri_path(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_uri_path) / [`set_xks_proxy_uri_path(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_uri_path):<br>required: **false**<br><p>Specifies the base path to the proxy APIs for this external key store. To find this value, see the documentation for your external key store proxy. This parameter is required for all custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p> <p>The value must start with <code>/</code> and must end with <code>/kms/xks/v1</code> where <code>v1</code> represents the version of the KMS external key store proxy API. This path can include an optional prefix between the required elements such as <code>/<i>prefix</i>/kms/xks/v1</code>.</p> <p><b>Uniqueness requirements: </b></p> <ul>  <li>   <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values must be unique in the Amazon Web Services account and Region.</p></li> </ul><br>
     ///   - [`xks_proxy_vpc_endpoint_service_name(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_vpc_endpoint_service_name) / [`set_xks_proxy_vpc_endpoint_service_name(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_vpc_endpoint_service_name):<br>required: **false**<br><p>Specifies the name of the Amazon VPC endpoint service for interface endpoints that is used to communicate with your external key store proxy (XKS proxy). This parameter is required when the value of <code>CustomKeyStoreType</code> is <code>EXTERNAL_KEY_STORE</code> and the value of <code>XksProxyConnectivity</code> is <code>VPC_ENDPOINT_SERVICE</code>.</p> <p>The Amazon VPC endpoint service must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keystore.html#xks-requirements">fulfill all requirements</a> for use with an external key store.</p> <p><b>Uniqueness requirements:</b></p> <ul>  <li>   <p>External key stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity can share an Amazon VPC, but each external key store must have its own VPC endpoint service and private DNS name.</p></li> </ul><br>
     ///   - [`xks_proxy_vpc_endpoint_service_owner(impl Into<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_vpc_endpoint_service_owner) / [`set_xks_proxy_vpc_endpoint_service_owner(Option<String>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_vpc_endpoint_service_owner):<br>required: **false**<br><p>Specifies the Amazon Web Services account ID that owns the Amazon VPC service endpoint for the interface that is used to communicate with your external key store proxy (XKS proxy). This parameter is optional. If not provided, the Amazon Web Services account ID calling the action will be used.</p><br>
-    ///   - [`xks_proxy_authentication_credential(XksProxyAuthenticationCredentialType)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_authentication_credential) / [`set_xks_proxy_authentication_credential(Option<XksProxyAuthenticationCredentialType>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_authentication_credential):<br>required: **false**<br><p>Specifies an authentication credential for the external key store proxy (XKS proxy). This parameter is required for all custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p> <p>The <code>XksProxyAuthenticationCredential</code> has two required elements: <code>RawSecretAccessKey</code>, a secret key, and <code>AccessKeyId</code>, a unique identifier for the <code>RawSecretAccessKey</code>. For character requirements, see <a href="API_XksProxyAuthenticationCredentialType.html">XksProxyAuthenticationCredentialType</a>.</p> <p>KMS uses this authentication credential to sign requests to the external key store proxy on your behalf. This credential is unrelated to Identity and Access Management (IAM) and Amazon Web Services credentials.</p> <p>This parameter doesn't set or change the authentication credentials on the XKS proxy. It just tells KMS the credential that you established on your external key store proxy. If you rotate your proxy authentication credential, use the <code>UpdateCustomKeyStore</code> operation to provide the new credential to KMS.</p><br>
+    ///   - [`xks_proxy_authentication_credential(XksProxyAuthenticationCredentialType)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_authentication_credential) / [`set_xks_proxy_authentication_credential(Option<XksProxyAuthenticationCredentialType>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_authentication_credential):<br>required: **false**<br><p>Specifies an authentication credential for the external key store proxy (XKS proxy). This parameter is required for all custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p> <p>The <code>XksProxyAuthenticationCredential</code> has two required elements: <code>RawSecretAccessKey</code>, a secret key, and <code>AccessKeyId</code>, a unique identifier for the <code>RawSecretAccessKey</code>. For character requirements, see <a href="API_XksProxyAuthenticationCredentialType.html">XksProxyAuthenticationCredentialType</a>.</p> <p>KMS uses this authentication credential to sign requests to the external key store proxy on your behalf. This credential is unrelated to Identity and Access Management (IAM) and Amazon Web Services credentials.</p> <p>This parameter doesn't set or change the authentication credentials on the XKS proxy. It just tells KMS the credential that you established on your external key store proxy. If you rotate your proxy authentication credential, use the <a>UpdateCustomKeyStore</a> operation to provide the new credential to KMS.</p><br>
     ///   - [`xks_proxy_connectivity(XksProxyConnectivityType)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::xks_proxy_connectivity) / [`set_xks_proxy_connectivity(Option<XksProxyConnectivityType>)`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreFluentBuilder::set_xks_proxy_connectivity):<br>required: **false**<br><p>Indicates how KMS communicates with the external key store proxy. This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p> <p>If the external key store proxy uses a public endpoint, specify <code>PUBLIC_ENDPOINT</code>. If the external key store proxy uses a Amazon VPC endpoint service for communication with KMS, specify <code>VPC_ENDPOINT_SERVICE</code>. For help making this choice, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/choose-xks-connectivity.html">Choosing a connectivity option</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>An Amazon VPC endpoint service keeps your communication with KMS in a private address space entirely within Amazon Web Services, but it requires more configuration, including establishing a Amazon VPC with multiple subnets, a VPC endpoint service, a network load balancer, and a verified private DNS name. A public endpoint is simpler to set up, but it might be slower and might not fulfill your security requirements. You might consider testing with a public endpoint, and then establishing a VPC endpoint service for production tasks. Note that this choice does not determine the location of the external key store proxy. Even if you choose a VPC endpoint service, the proxy can be hosted within the VPC or outside of Amazon Web Services such as in your corporate data center.</p><br>
     /// - On success, responds with [`CreateCustomKeyStoreOutput`](crate::operation::create_custom_key_store::CreateCustomKeyStoreOutput) with field(s):
     ///   - [`custom_key_store_id(Option<String>)`](crate::operation::create_custom_key_store::CreateCustomKeyStoreOutput::custom_key_store_id): <p>A unique identifier for the new custom key store.</p>
```

### `src/client/create_grant.rs`

```diff
--- reference/src/client/create_grant.rs
+++ generated/src/client/create_grant.rs
@@ -3,19 +3,19 @@
     /// Constructs a fluent builder for the [`CreateGrant`](crate::operation::create_grant::builders::CreateGrantFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key for the grant. The grant gives principals permission to use this KMS key.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`grantee_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::grantee_principal) / [`set_grantee_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_grantee_principal):<br>required: **false**<br><p>The identity that gets the permissions specified in the grant.</p> <p>To specify the grantee principal, use the Amazon Resource Name (ARN) of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p> <p>You must specify either <code>GranteePrincipal</code> or <code>GranteeServicePrincipal</code>, but not both.</p><br>
-    ///   - [`retiring_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::retiring_principal) / [`set_retiring_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_retiring_principal):<br>required: **false**<br><p>The principal that has permission to use the <code>RetireGrant</code> operation to retire the grant.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p> <p>The grant determines the retiring principal. Other principals might have permission to retire the grant or revoke the grant. For details, see <code>RevokeGrant</code> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-delete.html">Retiring and revoking grants</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>You can specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
-    ///   - [`operations(GrantOperation)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::operations) / [`set_operations(Option<Vec::<GrantOperation>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_operations):<br>required: **true**<br><p>A list of operations that the grant permits.</p> <p>This list must include only operations that are permitted in a grant. Also, the operation must be supported on the KMS key. For example, you cannot create a grant for a symmetric encryption KMS key that allows the <code>Sign</code> operation, or a grant for an asymmetric KMS key that allows the <code>GenerateDataKey</code> operation. If you try, KMS returns a <code>ValidationError</code> exception. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">Grant operations</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
-    ///   - [`constraints(GrantConstraints)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::constraints) / [`set_constraints(Option<GrantConstraints>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_constraints):<br>required: **false**<br><p>Specifies a grant constraint.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>KMS supports the following grant constraints.</p> <ul>  <li>   <p><code>EncryptionContextEquals</code> and <code>EncryptionContextSubset</code> — These encryption context grant constraints allow the permissions in the grant only when the encryption context in the request matches (<code>EncryptionContextEquals</code>) or includes (<code>EncryptionContextSubset</code>) the encryption context specified in the constraint.</p>   <p>Encryption context grant constraints are supported only on <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">grant operations</a> that include an <code>EncryptionContext</code> parameter, such as cryptographic operations on symmetric encryption KMS keys. You cannot use an encryption context grant constraint for cryptographic operations with asymmetric KMS keys or HMAC KMS keys. Operations with these keys don't support an encryption context. Grants with encryption context grant constraints can include the <code>DescribeKey</code> and <code>RetireGrant</code> operations, but the constraint doesn't apply to these operations. If a grant with an encryption context grant constraint includes the <code>CreateGrant</code> operation, the constraint requires that any grants created with the <code>CreateGrant</code> permission have an equally strict or stricter encryption context constraint.</p>   <p>Each constraint value can include up to 8 encryption context pairs. The encryption context value in each constraint cannot exceed 384 characters. For more information about encryption context, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p></li>  <li>   <p><code>SourceArn</code> — This grant constraint allows the permissions in the grant only when the request is made on behalf of a specific Amazon Web Services resource, identified by its <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a>. This is effectively the same as having the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> global condition key in the grant. The SourceArn constraint is supported on grants for all types of KMS keys and can also be applied to the <code>DescribeKey</code> operation when specified in the request. However, it does not apply to <code>RetireGrant</code> operation.</p></li> </ul> <p>For information about grant constraints, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-grant-overview.html#grant-constraints">Using grant constraints</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key for the grant. The grant gives principals permission to use this KMS key.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`grantee_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::grantee_principal) / [`set_grantee_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_grantee_principal):<br>required: **false**<br><p>The identity that gets the permissions specified in the grant.</p> <p>To specify the grantee principal, use the Amazon Resource Name (ARN) of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p> <p>You must specify either <code>GranteePrincipal</code> or <code>GranteeServicePrincipal</code>, but not both.</p><br>
+    ///   - [`retiring_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::retiring_principal) / [`set_retiring_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_retiring_principal):<br>required: **false**<br><p>The principal that has permission to use the <a>RetireGrant</a> operation to retire the grant.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p> <p>The grant determines the retiring principal. Other principals might have permission to retire the grant or revoke the grant. For details, see <a>RevokeGrant</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-delete.html">Retiring and revoking grants</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>You can specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
+    ///   - [`operations(GrantOperation)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::operations) / [`set_operations(Option<Vec::<GrantOperation>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_operations):<br>required: **true**<br><p>A list of operations that the grant permits.</p> <p>This list must include only operations that are permitted in a grant. Also, the operation must be supported on the KMS key. For example, you cannot create a grant for a symmetric encryption KMS key that allows the <a>Sign</a> operation, or a grant for an asymmetric KMS key that allows the <a>GenerateDataKey</a> operation. If you try, KMS returns a <code>ValidationError</code> exception. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">Grant operations</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
+    ///   - [`constraints(GrantConstraints)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::constraints) / [`set_constraints(Option<GrantConstraints>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_constraints):<br>required: **false**<br><p>Specifies a grant constraint.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>KMS supports the following grant constraints.</p> <ul>  <li>   <p><code>EncryptionContextEquals</code> and <code>EncryptionContextSubset</code> — These encryption context grant constraints allow the permissions in the grant only when the encryption context in the request matches (<code>EncryptionContextEquals</code>) or includes (<code>EncryptionContextSubset</code>) the encryption context specified in the constraint.</p>   <p>Encryption context grant constraints are supported only on <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">grant operations</a> that include an <code>EncryptionContext</code> parameter, such as cryptographic operations on symmetric encryption KMS keys. You cannot use an encryption context grant constraint for cryptographic operations with asymmetric KMS keys or HMAC KMS keys. Operations with these keys don't support an encryption context. Grants with encryption context grant constraints can include the <a>DescribeKey</a> and <a>RetireGrant</a> operations, but the constraint doesn't apply to these operations. If a grant with an encryption context grant constraint includes the <code>CreateGrant</code> operation, the constraint requires that any grants created with the <code>CreateGrant</code> permission have an equally strict or stricter encryption context constraint.</p>   <p>Each constraint value can include up to 8 encryption context pairs. The encryption context value in each constraint cannot exceed 384 characters. For more information about encryption context, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i> <i>Key Management Service Developer Guide</i></i>.</p></li>  <li>   <p><code>SourceArn</code> — This grant constraint allows the permissions in the grant only when the request is made on behalf of a specific Amazon Web Services resource, identified by its <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a>. This is effectively the same as having the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> global condition key in the grant. The SourceArn constraint is supported on grants for all types of KMS keys and can also be applied to the <a>DescribeKey</a> operation when specified in the request. However, it does not apply to <a>RetireGrant</a> operation.</p></li> </ul> <p>For information about grant constraints, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-grant-overview.html#grant-constraints">Using grant constraints</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`name(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_name):<br>required: **false**<br><p>A friendly name for the grant. Use this value to prevent the unintended creation of duplicate grants when retrying this request.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p> <p>When this value is present, you can retry a <code>CreateGrant</code> request with identical parameters; if the grant already exists, the original <code>GrantId</code> is returned without creating a new grant. Note that the returned grant token is unique with every <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned. All grant tokens for the same grant ID can be used interchangeably.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`grantee_service_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::grantee_service_principal) / [`set_grantee_service_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_grantee_service_principal):<br>required: **false**<br><p>The Amazon Web Services <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a> that gets the permissions specified in the grant.</p> <p>When you specify a <code>GranteeServicePrincipal</code>, you must also specify a <code>SourceArn</code> grant constraint. In addition, you must specify either a <code>RetiringPrincipal</code> or a <code>RetiringServicePrincipal</code>.</p> <p>You must specify either <code>GranteePrincipal</code> or <code>GranteeServicePrincipal</code>, but not both.</p><br>
-    ///   - [`retiring_service_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::retiring_service_principal) / [`set_retiring_service_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_retiring_service_principal):<br>required: **false**<br><p>The Amazon Web Services <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a> that has permission to use the <code>RetireGrant</code> operation to retire the grant.</p> <p>You can specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
+    ///   - [`retiring_service_principal(impl Into<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::retiring_service_principal) / [`set_retiring_service_principal(Option<String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_retiring_service_principal):<br>required: **false**<br><p>The Amazon Web Services <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a> that has permission to use the <a>RetireGrant</a> operation to retire the grant.</p> <p>You can specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
     /// - On success, responds with [`CreateGrantOutput`](crate::operation::create_grant::CreateGrantOutput) with field(s):
     ///   - [`grant_token(Option<String>)`](crate::operation::create_grant::CreateGrantOutput::grant_token): <p>The grant token.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
-    ///   - [`grant_id(Option<String>)`](crate::operation::create_grant::CreateGrantOutput::grant_id): <p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a <code>ListGrants</code>, <code>RetireGrant</code>, or <code>RevokeGrant</code> operation.</p>
+    ///   - [`grant_id(Option<String>)`](crate::operation::create_grant::CreateGrantOutput::grant_id): <p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a <a>ListGrants</a>, <a>RetireGrant</a>, or <a>RevokeGrant</a> operation.</p>
     /// - On failure, responds with [`SdkError<CreateGrantError>`](crate::operation::create_grant::CreateGrantError)
     pub fn create_grant(&self) -> crate::operation::create_grant::builders::CreateGrantFluentBuilder {
         crate::operation::create_grant::builders::CreateGrantFluentBuilder::new(self.handle.clone())
```

### `src/client/create_key.rs`

```diff
--- reference/src/client/create_key.rs
+++ generated/src/client/create_key.rs
@@ -3,16 +3,16 @@
     /// Constructs a fluent builder for the [`CreateKey`](crate::operation::create_key::builders::CreateKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`policy(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_policy):<br>required: **false**<br><p>The key policy to attach to the KMS key.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul><note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>CreateKey</code> and <code>PutKeyPolicy</code> API requests succeed, even though the policy statement is ineffective.</p>  <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>If you do not provide a key policy, KMS attaches a default key policy to the KMS key. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p><note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p><br>
-    ///   - [`description(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_description):<br>required: **false**<br><p>A description of the KMS key. Use a description that helps you decide whether the KMS key is appropriate for a task. The default value is an empty string (no description).</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>To set or change the description after the key is created, use <code>UpdateKeyDescription</code>.</p><br>
-    ///   - [`key_usage(KeyUsageType)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_usage) / [`set_key_usage(Option<KeyUsageType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_usage):<br>required: **false**<br><p>Determines the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a> for which you can use the KMS key. The default value is <code>ENCRYPT_DECRYPT</code>. This parameter is optional when you are creating a symmetric encryption KMS key; otherwise, it is required. You can't change the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#key-usage"> <code>KeyUsage</code> </a> value after the KMS key is created. Each KMS key can have only one key usage. This follows key usage best practices according to <a href="https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final">NIST SP 800-57 Recommendations for Key Management</a>, section 5.2, Key usage.</p> <p>Select only one valid value.</p> <ul>  <li>   <p>For symmetric encryption KMS keys, omit the parameter or specify <code>ENCRYPT_DECRYPT</code>.</p></li>  <li>   <p>For HMAC KMS keys (symmetric), specify <code>GENERATE_VERIFY_MAC</code>.</p></li>  <li>   <p>For asymmetric KMS keys with RSA key pairs, specify <code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with NIST-standard elliptic curve key pairs, specify <code>SIGN_VERIFY</code> or <code>KEY_AGREEMENT</code>.</p></li>  <li>   <p>For asymmetric KMS keys with <code>ECC_SECG_P256K1</code> key pairs, specify <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with ML-DSA key pairs, specify <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with SM2 key pairs (China Regions only), specify <code>ENCRYPT_DECRYPT</code>, <code>SIGN_VERIFY</code>, or <code>KEY_AGREEMENT</code>.</p></li> </ul><br>
+    ///   - [`policy(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_policy):<br>required: **false**<br><p>The key policy to attach to the KMS key.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>CreateKey</code> and <code>PutKeyPolicy</code> API requests succeed, even though the policy statement is ineffective.</p> <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>If you do not provide a key policy, KMS attaches a default key policy to the KMS key. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p><note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p><br>
+    ///   - [`description(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_description):<br>required: **false**<br><p>A description of the KMS key. Use a description that helps you decide whether the KMS key is appropriate for a task. The default value is an empty string (no description).</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>To set or change the description after the key is created, use <a>UpdateKeyDescription</a>.</p><br>
+    ///   - [`key_usage(KeyUsageType)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_usage) / [`set_key_usage(Option<KeyUsageType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_usage):<br>required: **false**<br><p>Determines the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a> for which you can use the KMS key. The default value is <code>ENCRYPT_DECRYPT</code>. This parameter is optional when you are creating a symmetric encryption KMS key; otherwise, it is required. You can't change the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#key-usage"> <code>KeyUsage</code></a> value after the KMS key is created. Each KMS key can have only one key usage. This follows key usage best practices according to <a href="https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final">NIST SP 800-57 Recommendations for Key Management</a>, section 5.2, Key usage.</p> <p>Select only one valid value.</p> <ul>  <li>   <p>For symmetric encryption KMS keys, omit the parameter or specify <code>ENCRYPT_DECRYPT</code>.</p></li>  <li>   <p>For HMAC KMS keys (symmetric), specify <code>GENERATE_VERIFY_MAC</code>.</p></li>  <li>   <p>For asymmetric KMS keys with RSA key pairs, specify <code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with NIST-standard elliptic curve key pairs, specify <code>SIGN_VERIFY</code> or <code>KEY_AGREEMENT</code>.</p></li>  <li>   <p>For asymmetric KMS keys with <code>ECC_SECG_P256K1</code> key pairs, specify <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with ML-DSA key pairs, specify <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with SM2 key pairs (China Regions only), specify <code>ENCRYPT_DECRYPT</code>, <code>SIGN_VERIFY</code>, or <code>KEY_AGREEMENT</code>.</p></li> </ul><br>
     ///   - [`customer_master_key_spec(CustomerMasterKeySpec)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::customer_master_key_spec) / [`set_customer_master_key_spec(Option<CustomerMasterKeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_customer_master_key_spec):<br>required: **false**<br><p>Instead, use the <code>KeySpec</code> parameter.</p> <p>The <code>KeySpec</code> and <code>CustomerMasterKeySpec</code> parameters work the same way. Only the names differ. We recommend that you use <code>KeySpec</code> parameter in your code. However, to avoid breaking changes, KMS supports both parameters.</p><br>
-    ///   - [`key_spec(KeySpec)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_spec) / [`set_key_spec(Option<KeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_spec):<br>required: **false**<br><p>Specifies the type of KMS key to create. The default value, <code>SYMMETRIC_DEFAULT</code>, creates a KMS key with a 256-bit AES-GCM key that is used for encryption and decryption, except in China Regions, where it creates a 128-bit symmetric key that uses SM4 encryption. For a detailed description of all supported key specs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose-key-spec.html">Key spec reference</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p> <p>The <code>KeySpec</code> determines whether the KMS key contains a symmetric key or an asymmetric key pair. It also determines the algorithms that the KMS key supports. You can't change the <code>KeySpec</code> after the KMS key is created. To further restrict the algorithms that can be used with the KMS key, use a condition key in its key policy or IAM policy. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-encryption-algorithm">kms:EncryptionAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-mac-algorithm">kms:MacAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-key-agreement-algorithm">kms:KeyAgreementAlgorithm</a>, or <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-signing-algorithm">kms:SigningAlgorithm</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p><important>  <p><a href="http://aws.amazon.com/kms/features/#AWS_Service_Integration">Amazon Web Services services that are integrated with KMS</a> use symmetric encryption KMS keys to protect your data. These services do not support asymmetric KMS keys or HMAC KMS keys.</p> </important> <p>KMS supports the following key specs for KMS keys:</p> <ul>  <li>   <p>Symmetric encryption key (default)</p>   <ul>    <li>     <p><code>SYMMETRIC_DEFAULT</code></p></li>   </ul></li>  <li>   <p>HMAC keys (symmetric)</p>   <ul>    <li>     <p><code>HMAC_224</code></p></li>    <li>     <p><code>HMAC_256</code></p></li>    <li>     <p><code>HMAC_384</code></p></li>    <li>     <p><code>HMAC_512</code></p></li>   </ul></li>  <li>   <p>Asymmetric RSA key pairs (encryption and decryption -or- signing and verification)</p>   <ul>    <li>     <p><code>RSA_2048</code></p></li>    <li>     <p><code>RSA_3072</code></p></li>    <li>     <p><code>RSA_4096</code></p></li>   </ul></li>  <li>   <p>Asymmetric NIST-standard elliptic curve key pairs (signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>ECC_NIST_P256</code> (secp256r1)</p></li>    <li>     <p><code>ECC_NIST_P384</code> (secp384r1)</p></li>    <li>     <p><code>ECC_NIST_P521</code> (secp521r1)</p></li>    <li>     <p><code>ECC_NIST_EDWARDS25519</code> (ed25519) - signing and verification only</p>     <ul>      <li>       <p><b>Note:</b> For ECC_NIST_EDWARDS25519 KMS keys, the ED25519_SHA_512 signing algorithm requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:RAW</code> </a>, while ED25519_PH_SHA_512 requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:DIGEST</code> </a>. These message types cannot be used interchangeably.</p></li>     </ul></li>   </ul></li>  <li>   <p>Other asymmetric elliptic curve key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ECC_SECG_P256K1</code> (secp256k1), commonly used for cryptocurrencies.</p></li>   </ul></li>  <li>   <p>Asymmetric ML-DSA key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ML_DSA_44</code></p></li>    <li>     <p><code>ML_DSA_65</code></p></li>    <li>     <p><code>ML_DSA_87</code></p></li>   </ul></li>  <li>   <p>SM2 key pairs (encryption and decryption -or- signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>SM2</code> (China Regions only)</p></li>   </ul></li> </ul><br>
+    ///   - [`key_spec(KeySpec)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_spec) / [`set_key_spec(Option<KeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_spec):<br>required: **false**<br><p>Specifies the type of KMS key to create. The default value, <code>SYMMETRIC_DEFAULT</code>, creates a KMS key with a 256-bit AES-GCM key that is used for encryption and decryption, except in China Regions, where it creates a 128-bit symmetric key that uses SM4 encryption. For a detailed description of all supported key specs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose-key-spec.html">Key spec reference</a> in the <i> <i>Key Management Service Developer Guide</i></i>.</p> <p>The <code>KeySpec</code> determines whether the KMS key contains a symmetric key or an asymmetric key pair. It also determines the algorithms that the KMS key supports. You can't change the <code>KeySpec</code> after the KMS key is created. To further restrict the algorithms that can be used with the KMS key, use a condition key in its key policy or IAM policy. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-encryption-algorithm">kms:EncryptionAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-mac-algorithm">kms:MacAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-key-agreement-algorithm">kms:KeyAgreementAlgorithm</a>, or <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-signing-algorithm">kms:SigningAlgorithm</a> in the <i> <i>Key Management Service Developer Guide</i></i>.</p><important>  <p><a href="http://aws.amazon.com/kms/features/#AWS_Service_Integration">Amazon Web Services services that are integrated with KMS</a> use symmetric encryption KMS keys to protect your data. These services do not support asymmetric KMS keys or HMAC KMS keys.</p> </important> <p>KMS supports the following key specs for KMS keys:</p> <ul>  <li>   <p>Symmetric encryption key (default)</p>   <ul>    <li>     <p><code>SYMMETRIC_DEFAULT</code></p></li>   </ul></li>  <li>   <p>HMAC keys (symmetric)</p>   <ul>    <li>     <p><code>HMAC_224</code></p></li>    <li>     <p><code>HMAC_256</code></p></li>    <li>     <p><code>HMAC_384</code></p></li>    <li>     <p><code>HMAC_512</code></p></li>   </ul></li>  <li>   <p>Asymmetric RSA key pairs (encryption and decryption -or- signing and verification)</p>   <ul>    <li>     <p><code>RSA_2048</code></p></li>    <li>     <p><code>RSA_3072</code></p></li>    <li>     <p><code>RSA_4096</code></p></li>   </ul></li>  <li>   <p>Asymmetric NIST-standard elliptic curve key pairs (signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>ECC_NIST_P256</code> (secp256r1)</p></li>    <li>     <p><code>ECC_NIST_P384</code> (secp384r1)</p></li>    <li>     <p><code>ECC_NIST_P521</code> (secp521r1)</p></li>    <li>     <p><code>ECC_NIST_EDWARDS25519</code> (ed25519) - signing and verification only</p>     <ul>      <li>       <p><b>Note:</b> For ECC_NIST_EDWARDS25519 KMS keys, the ED25519_SHA_512 signing algorithm requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:RAW</code></a>, while ED25519_PH_SHA_512 requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:DIGEST</code></a>. These message types cannot be used interchangeably.</p></li>     </ul></li>   </ul></li>  <li>   <p>Other asymmetric elliptic curve key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ECC_SECG_P256K1</code> (secp256k1), commonly used for cryptocurrencies.</p></li>   </ul></li>  <li>   <p>Asymmetric ML-DSA key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ML_DSA_44</code></p></li>    <li>     <p><code>ML_DSA_65</code></p></li>    <li>     <p><code>ML_DSA_87</code></p></li>   </ul></li>  <li>   <p>SM2 key pairs (encryption and decryption -or- signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>SM2</code> (China Regions only)</p></li>   </ul></li> </ul><br>
     ///   - [`origin(OriginType)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::origin) / [`set_origin(Option<OriginType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_origin):<br>required: **false**<br><p>The source of the key material for the KMS key. You cannot change the origin after you create the KMS key. The default is <code>AWS_KMS</code>, which means that KMS creates the key material.</p> <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-create-cmk.html">create a KMS key with no key material</a> (for imported key material), set this value to <code>EXTERNAL</code>. For more information about importing key material into KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>Key Management Service Developer Guide</i>. The <code>EXTERNAL</code> origin value is valid only for symmetric KMS keys.</p> <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html">create a KMS key in an CloudHSM key store</a> and create its key material in the associated CloudHSM cluster, set this value to <code>AWS_CLOUDHSM</code>. You must also use the <code>CustomKeyStoreId</code> parameter to identify the CloudHSM key store. The <code>KeySpec</code> value must be <code>SYMMETRIC_DEFAULT</code>.</p> <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keys.html">create a KMS key in an external key store</a>, set this value to <code>EXTERNAL_KEY_STORE</code>. You must also use the <code>CustomKeyStoreId</code> parameter to identify the external key store and the <code>XksKeyId</code> parameter to identify the associated external key. The <code>KeySpec</code> value must be <code>SYMMETRIC_DEFAULT</code>.</p><br>
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_custom_key_store_id):<br>required: **false**<br><p>Creates the KMS key in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. The <code>ConnectionState</code> of the custom key store must be <code>CONNECTED</code>. To find the CustomKeyStoreID and ConnectionState use the <code>DescribeCustomKeyStores</code> operation.</p> <p>This parameter is valid only for symmetric encryption KMS keys in a single Region. You cannot create any other type of KMS key in a custom key store.</p> <p>When you create a KMS key in an CloudHSM key store, KMS generates a non-exportable 256-bit symmetric key in its associated CloudHSM cluster and associates it with the KMS key. When you create a KMS key in an external key store, you must use the <code>XksKeyId</code> parameter to specify an external key that serves as key material for the KMS key.</p><br>
-    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
-    ///   - [`tags(Tag)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_tags):<br>required: **false**<br><p>Assigns one or more tags to the KMS key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <code>TagResource</code> operation.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <note>  <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p> <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tags in KMS</a>.</p><br>
-    ///   - [`multi_region(bool)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::multi_region) / [`set_multi_region(Option<bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_multi_region):<br>required: **false**<br><p>Creates a multi-Region primary key that you can replicate into other Amazon Web Services Regions. You cannot change this value after you create the KMS key.</p> <p>For a multi-Region key, set this parameter to <code>True</code>. For a single-Region KMS key, omit this parameter or set it to <code>False</code>. The default value is <code>False</code>.</p> <p>This operation supports <i>multi-Region keys</i>, an KMS feature that lets you create multiple interoperable KMS keys in different Amazon Web Services Regions. Because these KMS keys have the same key ID, key material, and other metadata, you can use them interchangeably to encrypt data in one Amazon Web Services Region and decrypt it in a different Amazon Web Services Region without re-encrypting the data or making a cross-Region call. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Multi-Region keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>This value creates a <i>primary key</i>, not a replica. To create a <i>replica key</i>, use the <code>ReplicateKey</code> operation.</p> <p>You can create a symmetric or asymmetric multi-Region key, and you can create a multi-Region key with imported key material. However, you cannot create a multi-Region key in a custom key store.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_custom_key_store_id):<br>required: **false**<br><p>Creates the KMS key in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. The <code>ConnectionState</code> of the custom key store must be <code>CONNECTED</code>. To find the CustomKeyStoreID and ConnectionState use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This parameter is valid only for symmetric encryption KMS keys in a single Region. You cannot create any other type of KMS key in a custom key store.</p> <p>When you create a KMS key in an CloudHSM key store, KMS generates a non-exportable 256-bit symmetric key in its associated CloudHSM cluster and associates it with the KMS key. When you create a KMS key in an external key store, you must use the <code>XksKeyId</code> parameter to specify an external key that serves as key material for the KMS key.</p><br>
+    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
+    ///   - [`tags(Tag)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_tags):<br>required: **false**<br><p>Assigns one or more tags to the KMS key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <a>TagResource</a> operation.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <note>  <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p> <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tags in KMS</a>.</p><br>
+    ///   - [`multi_region(bool)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::multi_region) / [`set_multi_region(Option<bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_multi_region):<br>required: **false**<br><p>Creates a multi-Region primary key that you can replicate into other Amazon Web Services Regions. You cannot change this value after you create the KMS key.</p> <p>For a multi-Region key, set this parameter to <code>True</code>. For a single-Region KMS key, omit this parameter or set it to <code>False</code>. The default value is <code>False</code>.</p> <p>This operation supports <i>multi-Region keys</i>, an KMS feature that lets you create multiple interoperable KMS keys in different Amazon Web Services Regions. Because these KMS keys have the same key ID, key material, and other metadata, you can use them interchangeably to encrypt data in one Amazon Web Services Region and decrypt it in a different Amazon Web Services Region without re-encrypting the data or making a cross-Region call. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Multi-Region keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>This value creates a <i>primary key</i>, not a replica. To create a <i>replica key</i>, use the <a>ReplicateKey</a> operation.</p> <p>You can create a symmetric or asymmetric multi-Region key, and you can create a multi-Region key with imported key material. However, you cannot create a multi-Region key in a custom key store.</p><br>
     ///   - [`xks_key_id(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::xks_key_id) / [`set_xks_key_id(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_xks_key_id):<br>required: **false**<br><p>Identifies the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html#concept-external-key">external key</a> that serves as key material for the KMS key in an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html">external key store</a>. Specify the ID that the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html#concept-xks-proxy">external key store proxy</a> uses to refer to the external key. For help, see the documentation for your external key store proxy.</p> <p>This parameter is required for a KMS key with an <code>Origin</code> value of <code>EXTERNAL_KEY_STORE</code>. It is not valid for KMS keys with any other <code>Origin</code> value.</p> <p>The external key must be an existing 256-bit AES symmetric encryption key hosted outside of Amazon Web Services in an external key manager associated with the external key store specified by the <code>CustomKeyStoreId</code> parameter. This key must be enabled and configured to perform encryption and decryption. Each KMS key in an external key store must use a different external key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keys.html#xks-key-requirements">Requirements for a KMS key in an external key store</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>Each KMS key in an external key store is associated two backing keys. One is key material that KMS generates. The other is the external key specified by this parameter. When you use the KMS key in an external key store to encrypt data, the encryption operation is performed first by KMS using the KMS key material, and then by the external key manager using the specified external key, a process known as <i>double encryption</i>. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html#concept-double-encryption">Double encryption</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`CreateKeyOutput`](crate::operation::create_key::CreateKeyOutput) with field(s):
     ///   - [`key_metadata(Option<KeyMetadata>)`](crate::operation::create_key::CreateKeyOutput::key_metadata): <p>Metadata associated with the KMS key.</p>
```

### `src/client/decrypt.rs`

```diff
--- reference/src/client/decrypt.rs
+++ generated/src/client/decrypt.rs
@@ -6,7 +6,7 @@
     ///   - [`ciphertext_blob(Blob)`](crate::operation::decrypt::builders::DecryptFluentBuilder::ciphertext_blob) / [`set_ciphertext_blob(Option<Blob>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_ciphertext_blob):<br>required: **false**<br><p>Ciphertext to be decrypted. The blob includes metadata.</p> <p>This parameter is required in all cases except when <code>DryRun</code> is <code>true</code> and <code>DryRunModifiers</code> is set to <code>IGNORE_CIPHERTEXT</code>.</p><br>
     ///   - [`encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_encryption_context):<br>required: **false**<br><p>Specifies the encryption context to use when decrypting the data. An encryption context is valid only for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a> with a symmetric encryption KMS key. The standard asymmetric encryption algorithms and HMAC algorithms that KMS uses do not support an encryption context.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
-    ///   - [`key_id(impl Into<String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_key_id):<br>required: **false**<br><p>Specifies the KMS key that KMS uses to decrypt the ciphertext.</p> <p>Enter a key ID of the KMS key that was used to encrypt the ciphertext. If you identify a different KMS key, the <code>Decrypt</code> operation throws an <code>IncorrectKeyException</code>.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key or when <code>DryRun</code> is <code>true</code> and <code>DryRunModifiers</code> is set to <code>IGNORE_CIPHERTEXT</code>. If you used a symmetric encryption KMS key, KMS can get the KMS key from metadata that it adds to the symmetric ciphertext blob. However, it is always recommended as a best practice. This practice ensures that you use the KMS key that you intend.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you should use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_key_id):<br>required: **false**<br><p>Specifies the KMS key that KMS uses to decrypt the ciphertext.</p> <p>Enter a key ID of the KMS key that was used to encrypt the ciphertext. If you identify a different KMS key, the <code>Decrypt</code> operation throws an <code>IncorrectKeyException</code>.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key or when <code>DryRun</code> is <code>true</code> and <code>DryRunModifiers</code> is set to <code>IGNORE_CIPHERTEXT</code>. If you used a symmetric encryption KMS key, KMS can get the KMS key from metadata that it adds to the symmetric ciphertext blob. However, it is always recommended as a best practice. This practice ensures that you use the KMS key that you intend.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you should use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`encryption_algorithm(EncryptionAlgorithmSpec)`](crate::operation::decrypt::builders::DecryptFluentBuilder::encryption_algorithm) / [`set_encryption_algorithm(Option<EncryptionAlgorithmSpec>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_encryption_algorithm):<br>required: **false**<br><p>Specifies the encryption algorithm that will be used to decrypt the ciphertext. Specify the same algorithm that was used to encrypt the data. If you specify a different algorithm, the <code>Decrypt</code> operation fails.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the only supported algorithm that is valid for symmetric encryption KMS keys.</p><br>
     ///   - [`recipient(RecipientInfo)`](crate::operation::decrypt::builders::DecryptFluentBuilder::recipient) / [`set_recipient(Option<RecipientInfo>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_recipient):<br>required: **false**<br><p>A signed <a href="https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave-concepts.html#term-attestdoc">attestation document</a> from an Amazon Web Services Nitro enclave or NitroTPM, and the encryption algorithm to use with the public key in the attestation document. The only valid encryption algorithm is <code>RSAES_OAEP_SHA_256</code>.</p> <p>This parameter supports the <a href="https://docs.aws.amazon.com/enclaves/latest/user/developing-applications.html#sdk">Amazon Web Services Nitro Enclaves SDK</a> or any Amazon Web Services SDK for Amazon Web Services Nitro Enclaves. It supports any Amazon Web Services SDK for Amazon Web Services NitroTPM.</p> <p>When you use this parameter, instead of returning the plaintext data, KMS encrypts the plaintext data with the public key in the attestation document, and returns the resulting ciphertext in the <code>CiphertextForRecipient</code> field in the response. This ciphertext can be decrypted only with the private key in the attested environment. The <code>Plaintext</code> field in the response is null or empty.</p> <p>For information about the interaction between KMS and Amazon Web Services Nitro Enclaves or Amazon Web Services NitroTPM, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/cryptographic-attestation.html">Cryptographic attestation support in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::decrypt::builders::DecryptFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/delete_custom_key_store.rs`

```diff
--- reference/src/client/delete_custom_key_store.rs
+++ generated/src/client/delete_custom_key_store.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`DeleteCustomKeyStore`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the ID of the custom key store you want to delete. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the ID of the custom key store you want to delete. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p><br>
     /// - On success, responds with [`DeleteCustomKeyStoreOutput`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreOutput)
     /// - On failure, responds with [`SdkError<DeleteCustomKeyStoreError>`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError)
     pub fn delete_custom_key_store(&self) -> crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder {
```

### `src/client/delete_imported_key_material.rs`

```diff
--- reference/src/client/delete_imported_key_material.rs
+++ generated/src/client/delete_imported_key_material.rs
@@ -3,8 +3,8 @@
     /// Constructs a fluent builder for the [`DeleteImportedKeyMaterial`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key from which you are deleting imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`key_material_id(impl Into<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::key_material_id) / [`set_key_material_id(Option<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::set_key_material_id):<br>required: **false**<br><p>Identifies the imported key material you are deleting.</p><important>  <p>If no KeyMaterialId is specified, KMS deletes the current key material.</p> </important> <p>To get the list of key material IDs associated with a KMS key, use <code>ListKeyRotations</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key from which you are deleting imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`key_material_id(impl Into<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::key_material_id) / [`set_key_material_id(Option<String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::set_key_material_id):<br>required: **false**<br><p>Identifies the imported key material you are deleting.</p><important>  <p>If no KeyMaterialId is specified, KMS deletes the current key material.</p> </important> <p>To get the list of key material IDs associated with a KMS key, use <a>ListKeyRotations</a>.</p><br>
     /// - On success, responds with [`DeleteImportedKeyMaterialOutput`](crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key from which the key material was deleted.</p>
     ///   - [`key_material_id(Option<String>)`](crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialOutput::key_material_id): <p>Identifies the deleted key material.</p>
```

### `src/client/derive_shared_secret.rs`

```diff
--- reference/src/client/derive_shared_secret.rs
+++ generated/src/client/derive_shared_secret.rs
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`DeriveSharedSecret`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies an asymmetric NIST-standard ECC or SM2 (China Regions only) KMS key. KMS uses the private key in the specified key pair to derive the shared secret. The key usage of the KMS key must be <code>KEY_AGREEMENT</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies an asymmetric NIST-standard ECC or SM2 (China Regions only) KMS key. KMS uses the private key in the specified key pair to derive the shared secret. The key usage of the KMS key must be <code>KEY_AGREEMENT</code>. To find the <code>KeyUsage</code> of a KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`key_agreement_algorithm(KeyAgreementAlgorithmSpec)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::key_agreement_algorithm) / [`set_key_agreement_algorithm(Option<KeyAgreementAlgorithmSpec>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_key_agreement_algorithm):<br>required: **true**<br><p>Specifies the key agreement algorithm used to derive the shared secret. The only valid value is <code>ECDH</code>.</p><br>
-    ///   - [`public_key(Blob)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::public_key) / [`set_public_key(Option<Blob>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_public_key):<br>required: **true**<br><p>Specifies the public key in your peer's NIST-standard elliptic curve (ECC) or SM2 (China Regions only) key pair.</p> <p>The public key must be a DER-encoded X.509 public key, also known as <code>SubjectPublicKeyInfo</code> (SPKI), as defined in <a href="https://tools.ietf.org/html/rfc5280">RFC 5280</a>.</p> <p><code>GetPublicKey</code> returns the public key of an asymmetric KMS key pair in the required DER-encoded format.</p><note>  <p>If you use <a href="https://docs.aws.amazon.com/cli/v1/userguide/cli-chap-welcome.html">Amazon Web Services CLI version 1</a>, you must provide the DER-encoded X.509 public key in a file. Otherwise, the Amazon Web Services CLI Base64-encodes the public key a second time, resulting in a <code>ValidationException</code>.</p> </note> <p>You can specify the public key as binary data in a file using fileb (<code>fileb://<path-to-file></path-to-file></code>) or in-line using a Base64 encoded string.</p><br>
+    ///   - [`public_key(Blob)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::public_key) / [`set_public_key(Option<Blob>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_public_key):<br>required: **true**<br><p>Specifies the public key in your peer's NIST-standard elliptic curve (ECC) or SM2 (China Regions only) key pair.</p> <p>The public key must be a DER-encoded X.509 public key, also known as <code>SubjectPublicKeyInfo</code> (SPKI), as defined in <a href="https://tools.ietf.org/html/rfc5280">RFC 5280</a>.</p> <p><a>GetPublicKey</a> returns the public key of an asymmetric KMS key pair in the required DER-encoded format.</p><note>  <p>If you use <a href="https://docs.aws.amazon.com/cli/v1/userguide/cli-chap-welcome.html">Amazon Web Services CLI version 1</a>, you must provide the DER-encoded X.509 public key in a file. Otherwise, the Amazon Web Services CLI Base64-encodes the public key a second time, resulting in a <code>ValidationException</code>.</p> </note> <p>You can specify the public key as binary data in a file using fileb (<code>fileb://<path-to-file></code>) or in-line using a Base64 encoded string.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`recipient(RecipientInfo)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::recipient) / [`set_recipient(Option<RecipientInfo>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_recipient):<br>required: **false**<br><p>A signed <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitro-enclave-how.html#term-attestdoc">attestation document</a> from an Amazon Web Services Nitro enclave or NitroTPM, and the encryption algorithm to use with the public key in the attestation document. The only valid encryption algorithm is <code>RSAES_OAEP_SHA_256</code>.</p> <p>This parameter only supports attestation documents for Amazon Web Services Nitro Enclaves or Amazon Web Services NitroTPM. To call DeriveSharedSecret generate an attestation document use either <a href="https://docs.aws.amazon.com/enclaves/latest/user/developing-applications.html#sdk">Amazon Web Services Nitro Enclaves SDK</a> for an Amazon Web Services Nitro Enclaves or <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/attestation-get-doc.html">Amazon Web Services NitroTPM tools</a> for Amazon Web Services NitroTPM. Then use the Recipient parameter from any Amazon Web Services SDK to provide the attestation document for the attested environment.</p> <p>When you use this parameter, instead of returning a plaintext copy of the shared secret, KMS encrypts the plaintext shared secret under the public key in the attestation document, and returns the resulting ciphertext in the <code>CiphertextForRecipient</code> field in the response. This ciphertext can be decrypted only with the private key in the attested environment. The <code>CiphertextBlob</code> field in the response contains the encrypted shared secret derived from the KMS key specified by the <code>KeyId</code> parameter and public key specified by the <code>PublicKey</code> parameter. The <code>SharedSecret</code> field in the response is null or empty.</p> <p>For information about the interaction between KMS and Amazon Web Services Nitro Enclaves or Amazon Web Services NitroTPM, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/cryptographic-attestation.html">Cryptographic attestation support in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/describe_key.rs`

```diff
--- reference/src/client/describe_key.rs
+++ generated/src/client/describe_key.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`DescribeKey`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Describes the specified KMS key.</p> <p>If you specify a predefined Amazon Web Services alias (an Amazon Web Services alias with no key ID), KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed key</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Describes the specified KMS key.</p> <p>If you specify a predefined Amazon Web Services alias (an Amazon Web Services alias with no key ID), KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed key</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`DescribeKeyOutput`](crate::operation::describe_key::DescribeKeyOutput) with field(s):
     ///   - [`key_metadata(Option<KeyMetadata>)`](crate::operation::describe_key::DescribeKeyOutput::key_metadata): <p>Metadata associated with the key.</p>
```

### `src/client/disable_key.rs`

```diff
--- reference/src/client/disable_key.rs
+++ generated/src/client/disable_key.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`DisableKey`](crate::operation::disable_key::builders::DisableKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to disable.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to disable.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`DisableKeyOutput`](crate::operation::disable_key::DisableKeyOutput)
     /// - On failure, responds with [`SdkError<DisableKeyError>`](crate::operation::disable_key::DisableKeyError)
     pub fn disable_key(&self) -> crate::operation::disable_key::builders::DisableKeyFluentBuilder {
```

### `src/client/disable_key_rotation.rs`

```diff
--- reference/src/client/disable_key_rotation.rs
+++ generated/src/client/disable_key_rotation.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`DisableKeyRotation`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable or disable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable or disable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`DisableKeyRotationOutput`](crate::operation::disable_key_rotation::DisableKeyRotationOutput)
     /// - On failure, responds with [`SdkError<DisableKeyRotationError>`](crate::operation::disable_key_rotation::DisableKeyRotationError)
     pub fn disable_key_rotation(&self) -> crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder {
```

### `src/client/disconnect_custom_key_store.rs`

```diff
--- reference/src/client/disconnect_custom_key_store.rs
+++ generated/src/client/disconnect_custom_key_store.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`DisconnectCustomKeyStore`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p><br>
     /// - On success, responds with [`DisconnectCustomKeyStoreOutput`](crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreOutput)
     /// - On failure, responds with [`SdkError<DisconnectCustomKeyStoreError>`](crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError)
     pub fn disconnect_custom_key_store(&self) -> crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder {
```

### `src/client/enable_key.rs`

```diff
--- reference/src/client/enable_key.rs
+++ generated/src/client/enable_key.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`EnableKey`](crate::operation::enable_key::builders::EnableKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::enable_key::builders::EnableKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::enable_key::builders::EnableKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to enable.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::enable_key::builders::EnableKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::enable_key::builders::EnableKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to enable.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`EnableKeyOutput`](crate::operation::enable_key::EnableKeyOutput)
     /// - On failure, responds with [`SdkError<EnableKeyError>`](crate::operation::enable_key::EnableKeyError)
     pub fn enable_key(&self) -> crate::operation::enable_key::builders::EnableKeyFluentBuilder {
```

### `src/client/enable_key_rotation.rs`

```diff
--- reference/src/client/enable_key_rotation.rs
+++ generated/src/client/enable_key_rotation.rs
@@ -3,8 +3,8 @@
     /// Constructs a fluent builder for the [`EnableKeyRotation`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#multi-region-rotate">multi-Region keys</a>, set the property on the primary key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`rotation_period_in_days(i32)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::rotation_period_in_days) / [`set_rotation_period_in_days(Option<i32>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_rotation_period_in_days):<br>required: **false**<br><p>Use this parameter to specify a custom period of time between each rotation date. If no value is specified, the default value is 365 days.</p> <p>The rotation period defines the number of days after you enable automatic key rotation that KMS will rotate your key material, and the number of days between each automatic rotation thereafter.</p> <p>You can use the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-rotation-period-in-days"> <code>kms:RotationPeriodInDays</code> </a> condition key to further constrain the values that principals can specify in the <code>RotationPeriodInDays</code> parameter.</p> <p></p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#multi-region-rotate">multi-Region keys</a>, set the property on the primary key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`rotation_period_in_days(i32)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::rotation_period_in_days) / [`set_rotation_period_in_days(Option<i32>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_rotation_period_in_days):<br>required: **false**<br><p>Use this parameter to specify a custom period of time between each rotation date. If no value is specified, the default value is 365 days.</p> <p>The rotation period defines the number of days after you enable automatic key rotation that KMS will rotate your key material, and the number of days between each automatic rotation thereafter.</p> <p>You can use the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-rotation-period-in-days"> <code>kms:RotationPeriodInDays</code></a> condition key to further constrain the values that principals can specify in the <code>RotationPeriodInDays</code> parameter.</p> <p></p><br>
     /// - On success, responds with [`EnableKeyRotationOutput`](crate::operation::enable_key_rotation::EnableKeyRotationOutput)
     /// - On failure, responds with [`SdkError<EnableKeyRotationError>`](crate::operation::enable_key_rotation::EnableKeyRotationError)
     pub fn enable_key_rotation(&self) -> crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder {
```

### `src/client/encrypt.rs`

```diff
--- reference/src/client/encrypt.rs
+++ generated/src/client/encrypt.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`Encrypt`](crate::operation::encrypt::builders::EncryptFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to use in the encryption operation. The KMS key must have a <code>KeyUsage</code> of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to use in the encryption operation. The KMS key must have a <code>KeyUsage</code> of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> of a KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`plaintext(Blob)`](crate::operation::encrypt::builders::EncryptFluentBuilder::plaintext) / [`set_plaintext(Option<Blob>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_plaintext):<br>required: **true**<br><p>Data to be encrypted.</p><br>
     ///   - [`encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_encryption_context):<br>required: **false**<br><p>Specifies the encryption context that will be used to encrypt the data. An encryption context is valid only for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a> with a symmetric encryption KMS key. The standard asymmetric encryption algorithms and HMAC algorithms that KMS uses do not support an encryption context.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/generate_data_key.rs`

```diff
--- reference/src/client/generate_data_key.rs
+++ generated/src/client/generate_data_key.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`GenerateDataKey`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_encryption_context):<br>required: **false**<br><p>Specifies the encryption context that will be used when encrypting the data key.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`number_of_bytes(i32)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<i32>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_number_of_bytes):<br>required: **false**<br><p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p> <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p><br>
     ///   - [`key_spec(DataKeySpec)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_spec) / [`set_key_spec(Option<DataKeySpec>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_spec):<br>required: **false**<br><p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p> <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p><br>
```

### `src/client/generate_data_key_pair.rs`

```diff
--- reference/src/client/generate_data_key_pair.rs
+++ generated/src/client/generate_data_key_pair.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_encryption_context):<br>required: **false**<br><p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
-    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the private key in the data key pair. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the private key in the data key pair. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`key_pair_spec(DataKeyPairSpec)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_pair_spec) / [`set_key_pair_spec(Option<DataKeyPairSpec>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_pair_spec):<br>required: **true**<br><p>Determines the type of data key pair that is generated.</p> <p>The KMS rule that restricts the use of asymmetric RSA and SM2 KMS keys to encrypt and decrypt or to sign and verify (but not both), the rule that permits you to use ECC KMS keys only to sign and verify, and the rule that permits you to use ML-DSA key pairs to sign and verify only are not effective on data key pairs, which are used outside of KMS. The SM2 key spec is only available in China Regions.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`recipient(RecipientInfo)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::recipient) / [`set_recipient(Option<RecipientInfo>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_recipient):<br>required: **false**<br><p>A signed <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitro-enclave-how.html#term-attestdoc">attestation document</a> from an Amazon Web Services Nitro enclave or NitroTPM, and the encryption algorithm to use with the public key in the attestation document. The only valid encryption algorithm is <code>RSAES_OAEP_SHA_256</code>.</p> <p>This parameter only supports attestation documents for Amazon Web Services Nitro Enclaves or Amazon Web Services NitroTPM. To call GenerateDataKeyPair generate an attestation document use either <a href="https://docs.aws.amazon.com/enclaves/latest/user/developing-applications.html#sdk">Amazon Web Services Nitro Enclaves SDK</a> for an Amazon Web Services Nitro Enclaves or <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/attestation-get-doc.html">Amazon Web Services NitroTPM tools</a> for Amazon Web Services NitroTPM. Then use the Recipient parameter from any Amazon Web Services SDK to provide the attestation document for the attested environment.</p> <p>When you use this parameter, instead of returning a plaintext copy of the private data key, KMS encrypts the plaintext private data key under the public key in the attestation document, and returns the resulting ciphertext in the <code>CiphertextForRecipient</code> field in the response. This ciphertext can be decrypted only with the private key in the attested environment. The <code>CiphertextBlob</code> field in the response contains a copy of the private data key encrypted under the KMS key specified by the <code>KeyId</code> parameter. The <code>PrivateKeyPlaintext</code> field in the response is null or empty.</p> <p>For information about the interaction between KMS and Amazon Web Services Nitro Enclaves or Amazon Web Services NitroTPM, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/cryptographic-attestation.html">Cryptographic attestation support in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/generate_data_key_pair_without_plaintext.rs`

```diff
--- reference/src/client/generate_data_key_pair_without_plaintext.rs
+++ generated/src/client/generate_data_key_pair_without_plaintext.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_encryption_context):<br>required: **false**<br><p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
-    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the private key in the data key pair. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the private key in the data key pair. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`key_pair_spec(DataKeyPairSpec)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::key_pair_spec) / [`set_key_pair_spec(Option<DataKeyPairSpec>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_key_pair_spec):<br>required: **true**<br><p>Determines the type of data key pair that is generated.</p> <p>The KMS rule that restricts the use of asymmetric RSA and SM2 KMS keys to encrypt and decrypt or to sign and verify (but not both), the rule that permits you to use ECC KMS keys only to sign and verify, and the rule that permits you to use ML-DSA key pairs to sign and verify only are not effective on data key pairs, which are used outside of KMS. The SM2 key spec is only available in China Regions.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/generate_data_key_without_plaintext.rs`

```diff
--- reference/src/client/generate_data_key_without_plaintext.rs
+++ generated/src/client/generate_data_key_without_plaintext.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`GenerateDataKeyWithoutPlaintext`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_key_id):<br>required: **true**<br><p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_encryption_context):<br>required: **false**<br><p>Specifies the encryption context that will be used when encrypting the data key.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`key_spec(DataKeySpec)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::key_spec) / [`set_key_spec(Option<DataKeySpec>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_key_spec):<br>required: **false**<br><p>The length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p><br>
     ///   - [`number_of_bytes(i32)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<i32>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_number_of_bytes):<br>required: **false**<br><p>The length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p><br>
```

### `src/client/generate_mac.rs`

```diff
--- reference/src/client/generate_mac.rs
+++ generated/src/client/generate_mac.rs
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`GenerateMac`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`message(Blob)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::message) / [`set_message(Option<Blob>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_message):<br>required: **true**<br><p>The message to be hashed. Specify a message of up to 4,096 bytes.</p> <p><code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generate an HMAC for a hash digest of a message, you must verify the HMAC of the same hash digest.</p><br>
-    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_key_id):<br>required: **true**<br><p>The HMAC KMS key to use in the operation. The MAC algorithm computes the HMAC for the message and the key as described in <a href="https://datatracker.ietf.org/doc/html/rfc2104">RFC 2104</a>.</p> <p>To identify an HMAC KMS key, use the <code>DescribeKey</code> operation and see the <code>KeySpec</code> field in the response.</p><br>
-    ///   - [`mac_algorithm(MacAlgorithmSpec)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::mac_algorithm) / [`set_mac_algorithm(Option<MacAlgorithmSpec>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_mac_algorithm):<br>required: **true**<br><p>The MAC algorithm used in the operation.</p> <p>The algorithm must be compatible with the HMAC KMS key that you specify. To find the MAC algorithms that your HMAC KMS key supports, use the <code>DescribeKey</code> operation and see the <code>MacAlgorithms</code> field in the <code>DescribeKey</code> response.</p><br>
+    ///   - [`message(Blob)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::message) / [`set_message(Option<Blob>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_message):<br>required: **true**<br><p>The message to be hashed. Specify a message of up to 4,096 bytes.</p> <p><code>GenerateMac</code> and <a>VerifyMac</a> do not provide special handling for message digests. If you generate an HMAC for a hash digest of a message, you must verify the HMAC of the same hash digest.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_key_id):<br>required: **true**<br><p>The HMAC KMS key to use in the operation. The MAC algorithm computes the HMAC for the message and the key as described in <a href="https://datatracker.ietf.org/doc/html/rfc2104">RFC 2104</a>.</p> <p>To identify an HMAC KMS key, use the <a>DescribeKey</a> operation and see the <code>KeySpec</code> field in the response.</p><br>
+    ///   - [`mac_algorithm(MacAlgorithmSpec)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::mac_algorithm) / [`set_mac_algorithm(Option<MacAlgorithmSpec>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_mac_algorithm):<br>required: **true**<br><p>The MAC algorithm used in the operation.</p> <p>The algorithm must be compatible with the HMAC KMS key that you specify. To find the MAC algorithms that your HMAC KMS key supports, use the <a>DescribeKey</a> operation and see the <code>MacAlgorithms</code> field in the <code>DescribeKey</code> response.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`GenerateMacOutput`](crate::operation::generate_mac::GenerateMacOutput) with field(s):
```

### `src/client/generate_random.rs`

```diff
--- reference/src/client/generate_random.rs
+++ generated/src/client/generate_random.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`number_of_bytes(i32)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<i32>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_number_of_bytes):<br>required: **false**<br><p>The length of the random byte string. This parameter is required.</p><br>
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_custom_key_store_id):<br>required: **false**<br><p>Generates the random byte string in the CloudHSM cluster that is associated with the specified CloudHSM key store. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p> <p>External key store IDs are not valid for this parameter. If you specify the ID of an external key store, <code>GenerateRandom</code> throws an <code>UnsupportedOperationException</code>.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_custom_key_store_id):<br>required: **false**<br><p>Generates the random byte string in the CloudHSM cluster that is associated with the specified CloudHSM key store. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>External key store IDs are not valid for this parameter. If you specify the ID of an external key store, <code>GenerateRandom</code> throws an <code>UnsupportedOperationException</code>.</p><br>
     ///   - [`recipient(RecipientInfo)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::recipient) / [`set_recipient(Option<RecipientInfo>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_recipient):<br>required: **false**<br><p>A signed <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitro-enclave-how.html#term-attestdoc">attestation document</a> from an Amazon Web Services Nitro enclave or NitroTPM, and the encryption algorithm to use with the public key in the attestation document. The only valid encryption algorithm is <code>RSAES_OAEP_SHA_256</code>.</p> <p>This parameter supports the <a href="https://docs.aws.amazon.com/enclaves/latest/user/developing-applications.html#sdk">Amazon Web Services Nitro Enclaves SDK</a> or any Amazon Web Services SDK for Amazon Web Services Nitro Enclaves. It supports any Amazon Web Services SDK for Amazon Web Services NitroTPM.</p> <p>When you use this parameter, instead of returning plaintext bytes, KMS encrypts the plaintext bytes under the public key in the attestation document, and returns the resulting ciphertext in the <code>CiphertextForRecipient</code> field in the response. This ciphertext can be decrypted only with the private key in the attested environment. The <code>Plaintext</code> field in the response is null or empty.</p> <p>For information about the interaction between KMS and Amazon Web Services Nitro Enclaves or Amazon Web Services NitroTPM, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/cryptographic-attestation.html">Cryptographic attestation support in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`GenerateRandomOutput`](crate::operation::generate_random::GenerateRandomOutput) with field(s):
     ///   - [`plaintext(Option<Blob>)`](crate::operation::generate_random::GenerateRandomOutput::plaintext): <p>The random byte string. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p> <p>If the response includes the <code>CiphertextForRecipient</code> field, the <code>Plaintext</code> field is null or empty.</p>
```

### `src/client/get_key_last_usage.rs`

```diff
--- reference/src/client/get_key_last_usage.rs
+++ generated/src/client/get_key_last_usage.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`GetKeyLastUsage`](crate::operation::get_key_last_usage::builders::GetKeyLastUsageFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_last_usage::builders::GetKeyLastUsageFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_last_usage::builders::GetKeyLastUsageFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to get usage information for. To specify a KMS key, use its key ID or key ARN. Alias names are not supported.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_last_usage::builders::GetKeyLastUsageFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_last_usage::builders::GetKeyLastUsageFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key to get usage information for. To specify a KMS key, use its key ID or key ARN. Alias names are not supported.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`GetKeyLastUsageOutput`](crate::operation::get_key_last_usage::GetKeyLastUsageOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::get_key_last_usage::GetKeyLastUsageOutput::key_id): <p>The globally unique identifier for the KMS key.</p>
     ///   - [`key_last_usage(Option<KeyLastUsageData>)`](crate::operation::get_key_last_usage::GetKeyLastUsageOutput::key_last_usage): <p>Contains usage information about the last time the KMS key was used for a successful cryptographic operation. If the key has not been used since tracking began, this response element is empty.</p>
```

### `src/client/get_key_policy.rs`

```diff
--- reference/src/client/get_key_policy.rs
+++ generated/src/client/get_key_policy.rs
@@ -3,8 +3,8 @@
     /// Constructs a fluent builder for the [`GetKeyPolicy`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the key policy for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`policy_name(impl Into<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::set_policy_name):<br>required: **false**<br><p>Specifies the name of the key policy. If no policy name is specified, the default value is <code>default</code>. The only valid name is <code>default</code>. To get the names of key policies, use <code>ListKeyPolicies</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the key policy for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`policy_name(impl Into<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::set_policy_name):<br>required: **false**<br><p>Specifies the name of the key policy. If no policy name is specified, the default value is <code>default</code>. The only valid name is <code>default</code>. To get the names of key policies, use <a>ListKeyPolicies</a>.</p><br>
     /// - On success, responds with [`GetKeyPolicyOutput`](crate::operation::get_key_policy::GetKeyPolicyOutput) with field(s):
     ///   - [`policy(Option<String>)`](crate::operation::get_key_policy::GetKeyPolicyOutput::policy): <p>A key policy document in JSON format.</p>
     ///   - [`policy_name(Option<String>)`](crate::operation::get_key_policy::GetKeyPolicyOutput::policy_name): <p>The name of the key policy. The only valid value is <code>default</code>.</p>
```

### `src/client/get_key_rotation_status.rs`

```diff
--- reference/src/client/get_key_rotation_status.rs
+++ generated/src/client/get_key_rotation_status.rs
@@ -3,13 +3,13 @@
     /// Constructs a fluent builder for the [`GetKeyRotationStatus`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the rotation status for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the rotation status for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`GetKeyRotationStatusOutput`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput) with field(s):
     ///   - [`key_rotation_enabled(bool)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::key_rotation_enabled): <p>A Boolean value that specifies whether key rotation is enabled.</p>
     ///   - [`key_id(Option<String>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::key_id): <p>Identifies the specified symmetric encryption KMS key.</p>
     ///   - [`rotation_period_in_days(Option<i32>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::rotation_period_in_days): <p>The number of days between each automatic rotation. The default value is 365 days.</p>
     ///   - [`next_rotation_date(Option<DateTime>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::next_rotation_date): <p>The next date that KMS will automatically rotate the key material.</p>
-    ///   - [`on_demand_rotation_start_date(Option<DateTime>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::on_demand_rotation_start_date): <p>Identifies the date and time that an in progress on-demand rotation was initiated.</p> <p>KMS uses a background process to perform rotations. As a result, there might be a slight delay between initiating on-demand key rotation and the rotation's completion. Once the on-demand rotation is complete, KMS removes this field from the response. You can use <code>ListKeyRotations</code> to view the details of the completed on-demand rotation.</p>
+    ///   - [`on_demand_rotation_start_date(Option<DateTime>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::on_demand_rotation_start_date): <p>Identifies the date and time that an in progress on-demand rotation was initiated.</p> <p>KMS uses a background process to perform rotations. As a result, there might be a slight delay between initiating on-demand key rotation and the rotation's completion. Once the on-demand rotation is complete, KMS removes this field from the response. You can use <a>ListKeyRotations</a> to view the details of the completed on-demand rotation.</p>
     /// - On failure, responds with [`SdkError<GetKeyRotationStatusError>`](crate::operation::get_key_rotation_status::GetKeyRotationStatusError)
     pub fn get_key_rotation_status(&self) -> crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder {
         crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::new(self.handle.clone())
```

### `src/client/get_parameters_for_import.rs`

```diff
--- reference/src/client/get_parameters_for_import.rs
+++ generated/src/client/get_parameters_for_import.rs
@@ -3,14 +3,14 @@
     /// Constructs a fluent builder for the [`GetParametersForImport`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_key_id):<br>required: **true**<br><p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p> <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_key_id):<br>required: **true**<br><p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p> <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`wrapping_algorithm(AlgorithmSpec)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::wrapping_algorithm) / [`set_wrapping_algorithm(Option<AlgorithmSpec>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_wrapping_algorithm):<br>required: **true**<br><p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p> <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p> <ul>  <li>   <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>  <li>   <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>  <li>   <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>   <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>  <li>   <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>   <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>  <li>   <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li> </ul><br>
     ///   - [`wrapping_key_spec(WrappingKeySpec)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::wrapping_key_spec) / [`set_wrapping_key_spec(Option<WrappingKeySpec>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_wrapping_key_spec):<br>required: **true**<br><p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p> <p>Use the longest RSA wrapping key that is practical.</p> <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p><br>
     /// - On success, responds with [`GetParametersForImportOutput`](crate::operation::get_parameters_for_import::GetParametersForImportOutput) with field(s):
-    ///   - [`key_id(Option<String>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key to use in a subsequent <code>ImportKeyMaterial</code> request. This is the same KMS key specified in the <code>GetParametersForImport</code> request.</p>
-    ///   - [`import_token(Option<Blob>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::import_token): <p>The import token to send in a subsequent <code>ImportKeyMaterial</code> request.</p>
-    ///   - [`public_key(Option<Blob>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::public_key): <p>The public key to use to encrypt the key material before importing it with <code>ImportKeyMaterial</code>.</p>
-    ///   - [`parameters_valid_to(Option<DateTime>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::parameters_valid_to): <p>The time at which the import token and public key are no longer valid. After this time, you cannot use them to make an <code>ImportKeyMaterial</code> request and you must send another <code>GetParametersForImport</code> request to get new ones.</p>
+    ///   - [`key_id(Option<String>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key to use in a subsequent <a>ImportKeyMaterial</a> request. This is the same KMS key specified in the <code>GetParametersForImport</code> request.</p>
+    ///   - [`import_token(Option<Blob>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::import_token): <p>The import token to send in a subsequent <a>ImportKeyMaterial</a> request.</p>
+    ///   - [`public_key(Option<Blob>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::public_key): <p>The public key to use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>.</p>
+    ///   - [`parameters_valid_to(Option<DateTime>)`](crate::operation::get_parameters_for_import::GetParametersForImportOutput::parameters_valid_to): <p>The time at which the import token and public key are no longer valid. After this time, you cannot use them to make an <a>ImportKeyMaterial</a> request and you must send another <code>GetParametersForImport</code> request to get new ones.</p>
     /// - On failure, responds with [`SdkError<GetParametersForImportError>`](crate::operation::get_parameters_for_import::GetParametersForImportError)
     pub fn get_parameters_for_import(&self) -> crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder {
         crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::new(self.handle.clone())
```

### `src/client/get_public_key.rs`

```diff
--- reference/src/client/get_public_key.rs
+++ generated/src/client/get_public_key.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`GetPublicKey`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the asymmetric KMS key that includes the public key.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the asymmetric KMS key that includes the public key.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`GetPublicKeyOutput`](crate::operation::get_public_key::GetPublicKeyOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::get_public_key::GetPublicKeyOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the asymmetric KMS key from which the public key was downloaded.</p>
```

### `src/client/import_key_material.rs`

```diff
--- reference/src/client/import_key_material.rs
+++ generated/src/client/import_key_material.rs
@@ -3,14 +3,14 @@
     /// Constructs a fluent builder for the [`ImportKeyMaterial`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_id):<br>required: **true**<br><p>The identifier of the KMS key that will be associated with the imported key material. This must be the same KMS key specified in the <code>KeyID</code> parameter of the corresponding <code>GetParametersForImport</code> request. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code> and its <code>KeyState</code> must be <code>PendingImport</code>.</p> <p>The KMS key can be a symmetric encryption KMS key, HMAC KMS key, asymmetric encryption KMS key, or asymmetric signing KMS key, including a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">multi-Region key</a> of any supported type. You cannot perform this operation on a KMS key in a custom key store, or on a KMS key in a different Amazon Web Services account.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`import_token(Blob)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::import_token) / [`set_import_token(Option<Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_import_token):<br>required: **true**<br><p>The import token that you received in the response to a previous <code>GetParametersForImport</code> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p><br>
-    ///   - [`encrypted_key_material(Blob)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::encrypted_key_material) / [`set_encrypted_key_material(Option<Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_encrypted_key_material):<br>required: **true**<br><p>The encrypted key material to import. The key material must be encrypted under the public wrapping key that <code>GetParametersForImport</code> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p><br>
-    ///   - [`valid_to(DateTime)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::valid_to) / [`set_valid_to(Option<DateTime>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_valid_to):<br>required: **false**<br><p>The date and time when the imported key material expires. This parameter is required when the value of the <code>ExpirationModel</code> parameter is <code>KEY_MATERIAL_EXPIRES</code>. Otherwise it is not valid.</p> <p>The value of this parameter must be a future date and time. The maximum value is 365 days from the request date.</p> <p>When the key material expires, KMS deletes the key material from the KMS key. Without its key material, the KMS key is unusable. To use the KMS key in cryptographic operations, you must reimport the same key material.</p> <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_id):<br>required: **true**<br><p>The identifier of the KMS key that will be associated with the imported key material. This must be the same KMS key specified in the <code>KeyID</code> parameter of the corresponding <a>GetParametersForImport</a> request. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code> and its <code>KeyState</code> must be <code>PendingImport</code>.</p> <p>The KMS key can be a symmetric encryption KMS key, HMAC KMS key, asymmetric encryption KMS key, or asymmetric signing KMS key, including a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">multi-Region key</a> of any supported type. You cannot perform this operation on a KMS key in a custom key store, or on a KMS key in a different Amazon Web Services account.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`import_token(Blob)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::import_token) / [`set_import_token(Option<Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_import_token):<br>required: **true**<br><p>The import token that you received in the response to a previous <a>GetParametersForImport</a> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p><br>
+    ///   - [`encrypted_key_material(Blob)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::encrypted_key_material) / [`set_encrypted_key_material(Option<Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_encrypted_key_material):<br>required: **true**<br><p>The encrypted key material to import. The key material must be encrypted under the public wrapping key that <a>GetParametersForImport</a> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p><br>
+    ///   - [`valid_to(DateTime)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::valid_to) / [`set_valid_to(Option<DateTime>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_valid_to):<br>required: **false**<br><p>The date and time when the imported key material expires. This parameter is required when the value of the <code>ExpirationModel</code> parameter is <code>KEY_MATERIAL_EXPIRES</code>. Otherwise it is not valid.</p> <p>The value of this parameter must be a future date and time. The maximum value is 365 days from the request date.</p> <p>When the key material expires, KMS deletes the key material from the KMS key. Without its key material, the KMS key is unusable. To use the KMS key in cryptographic operations, you must reimport the same key material.</p> <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<a>DeleteImportedKeyMaterial</a>) and reimport the key material.</p><br>
     ///   - [`expiration_model(ExpirationModelType)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::expiration_model) / [`set_expiration_model(Option<ExpirationModelType>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_expiration_model):<br>required: **false**<br><p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>. For help with this choice, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-import-key-material.html#importing-keys-expiration">Setting an expiration time</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>When the value of <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, you must specify a value for the <code>ValidTo</code> parameter. When value is <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p> <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must reimport the key material.</p><br>
     ///   - [`import_type(ImportType)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::import_type) / [`set_import_type(Option<ImportType>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_import_type):<br>required: **false**<br><p>Indicates whether the key material being imported is previously associated with this KMS key or not. This parameter is optional and only usable with symmetric encryption keys. If no key material has ever been imported into the KMS key, and this parameter is omitted, the parameter defaults to <code>NEW_KEY_MATERIAL</code>. After the first key material is imported, if this parameter is omitted then the parameter defaults to <code>EXISTING_KEY_MATERIAL</code>.</p> <p>For multi-Region keys, you must first import new key material into the primary Region key. You should use the <code>NEW_KEY_MATERIAL</code> import type when importing key material into the primary Region key. Then, you can import the same key material into the replica Region key. The import type for the replica Region key should be <code>EXISTING_KEY_MATERIAL</code>.</p><br>
     ///   - [`key_material_description(impl Into<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_material_description) / [`set_key_material_description(Option<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_material_description):<br>required: **false**<br><p>Description for the key material being imported. This parameter is optional and only usable with symmetric encryption keys. If you do not specify a key material description, KMS retains the value you specified when you last imported the same key material into this KMS key.</p><br>
-    ///   - [`key_material_id(impl Into<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_material_id) / [`set_key_material_id(Option<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_material_id):<br>required: **false**<br><p>Identifies the key material being imported. This parameter is optional and only usable with symmetric encryption keys. You cannot specify a key material ID with <code>ImportType</code> set to <code>NEW_KEY_MATERIAL</code>. Whenever you import key material into a symmetric encryption key, KMS assigns a unique identifier to the key material based on the KMS key ID and the imported key material. When you re-import key material with a specified key material ID, KMS:</p> <ul>  <li>   <p>Computes the identifier for the key material</p></li>  <li>   <p>Matches the computed identifier against the specified key material ID</p></li>  <li>   <p>Verifies that the key material ID is already associated with the KMS key</p></li> </ul> <p>To get the list of key material IDs associated with a KMS key, use <code>ListKeyRotations</code>.</p><br>
+    ///   - [`key_material_id(impl Into<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_material_id) / [`set_key_material_id(Option<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_material_id):<br>required: **false**<br><p>Identifies the key material being imported. This parameter is optional and only usable with symmetric encryption keys. You cannot specify a key material ID with <code>ImportType</code> set to <code>NEW_KEY_MATERIAL</code>. Whenever you import key material into a symmetric encryption key, KMS assigns a unique identifier to the key material based on the KMS key ID and the imported key material. When you re-import key material with a specified key material ID, KMS:</p> <ul>  <li>   <p>Computes the identifier for the key material</p></li>  <li>   <p>Matches the computed identifier against the specified key material ID</p></li>  <li>   <p>Verifies that the key material ID is already associated with the KMS key</p></li> </ul> <p>To get the list of key material IDs associated with a KMS key, use <a>ListKeyRotations</a>.</p><br>
     /// - On success, responds with [`ImportKeyMaterialOutput`](crate::operation::import_key_material::ImportKeyMaterialOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::import_key_material::ImportKeyMaterialOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key into which key material was imported.</p>
     ///   - [`key_material_id(Option<String>)`](crate::operation::import_key_material::ImportKeyMaterialOutput::key_material_id): <p>Identifies the imported key material.</p>
```

### `src/client/list_aliases.rs`

```diff
--- reference/src/client/list_aliases.rs
+++ generated/src/client/list_aliases.rs
@@ -4,7 +4,7 @@
     /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::into_paginator).
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_key_id):<br>required: **false**<br><p>Lists only aliases that are associated with the specified KMS key. Enter a KMS key in your Amazon Web Services account.</p> <p>This parameter is optional. If you omit it, <code>ListAliases</code> returns all aliases in the account and Region.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_key_id):<br>required: **false**<br><p>Lists only aliases that are associated with the specified KMS key. Enter a KMS key in your Amazon Web Services account.</p> <p>This parameter is optional. If you omit it, <code>ListAliases</code> returns all aliases in the account and Region.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`limit(i32)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p><br>
     ///   - [`marker(impl Into<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
     /// - On success, responds with [`ListAliasesOutput`](crate::operation::list_aliases::ListAliasesOutput) with field(s):
```

### `src/client/list_grants.rs`

```diff
--- reference/src/client/list_grants.rs
+++ generated/src/client/list_grants.rs
@@ -6,7 +6,7 @@
     /// - The fluent builder is configurable:
     ///   - [`limit(i32)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p><br>
     ///   - [`marker(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
-    ///   - [`key_id(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_key_id):<br>required: **true**<br><p>Returns only grants for the specified KMS key. This parameter is required.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_key_id):<br>required: **true**<br><p>Returns only grants for the specified KMS key. This parameter is required.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`grant_id(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grant_id) / [`set_grant_id(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grant_id):<br>required: **false**<br><p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant.</p><br>
     ///   - [`grantee_principal(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grantee_principal) / [`set_grantee_principal(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grantee_principal):<br>required: **false**<br><p>Returns only grants where the specified principal is the grantee principal for the grant.</p> <p>You can specify either <code>GranteePrincipal</code> or <code>GranteeServicePrincipal</code>, but not both.</p><br>
     ///   - [`grantee_service_principal(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grantee_service_principal) / [`set_grantee_service_principal(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grantee_service_principal):<br>required: **false**<br><p>Returns only grants where the specified Amazon Web Services service principal is the grantee service principal for the grant. This filter is only usable by callers in a service principal.</p> <p>You can specify either <code>GranteePrincipal</code> or <code>GranteeServicePrincipal</code>, but not both.</p><br>
```

### `src/client/list_key_policies.rs`

```diff
--- reference/src/client/list_key_policies.rs
+++ generated/src/client/list_key_policies.rs
@@ -4,7 +4,7 @@
     /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::into_paginator).
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the names of key policies for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the names of key policies for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`limit(i32)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p> <p>Only one policy can be attached to a key.</p><br>
     ///   - [`marker(impl Into<String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
     /// - On success, responds with [`ListKeyPoliciesOutput`](crate::operation::list_key_policies::ListKeyPoliciesOutput) with field(s):
```

### `src/client/list_key_rotations.rs`

```diff
--- reference/src/client/list_key_rotations.rs
+++ generated/src/client/list_key_rotations.rs
@@ -4,7 +4,7 @@
     /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::into_paginator).
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the key rotations for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets the key rotations for the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`include_key_material(IncludeKeyMaterial)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::include_key_material) / [`set_include_key_material(Option<IncludeKeyMaterial>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_include_key_material):<br>required: **false**<br><p>Use this optional parameter to control which key materials associated with this key are listed in the response. The default value of this parameter is <code>ROTATIONS_ONLY</code>. If you omit this parameter, KMS returns information on the key materials created by automatic or on-demand key rotation. When you specify a value of <code>ALL_KEY_MATERIAL</code>, KMS adds the first key material and any imported key material pending rotation to the response. This parameter can only be used with KMS keys that support automatic or on-demand key rotation.</p><br>
     ///   - [`limit(i32)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p><br>
     ///   - [`marker(impl Into<String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
```

### `src/client/list_resource_tags.rs`

```diff
--- reference/src/client/list_resource_tags.rs
+++ generated/src/client/list_resource_tags.rs
@@ -4,7 +4,7 @@
     /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::into_paginator).
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets tags on the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_key_id):<br>required: **true**<br><p>Gets tags on the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`limit(i32)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 50, inclusive. If you do not include a value, it defaults to 50.</p><br>
     ///   - [`marker(impl Into<String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p> <p>Do not attempt to construct this value. Use only the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
     /// - On success, responds with [`ListResourceTagsOutput`](crate::operation::list_resource_tags::ListResourceTagsOutput) with field(s):
```

### `src/client/list_retirable_grants.rs`

```diff
--- reference/src/client/list_retirable_grants.rs
+++ generated/src/client/list_retirable_grants.rs
@@ -6,7 +6,7 @@
     /// - The fluent builder is configurable:
     ///   - [`limit(i32)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p><br>
     ///   - [`marker(impl Into<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
-    ///   - [`retiring_principal(impl Into<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::retiring_principal) / [`set_retiring_principal(Option<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_retiring_principal):<br>required: **false**<br><p>The retiring principal for which to list grants. Enter a principal in your Amazon Web Services account.</p> <p>To specify the retiring principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p> <p>You must specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
+    ///   - [`retiring_principal(impl Into<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::retiring_principal) / [`set_retiring_principal(Option<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_retiring_principal):<br>required: **false**<br><p>The retiring principal for which to list grants. Enter a principal in your Amazon Web Services account.</p> <p>To specify the retiring principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p> <p>You must specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
     ///   - [`retiring_service_principal(impl Into<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::retiring_service_principal) / [`set_retiring_service_principal(Option<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_retiring_service_principal):<br>required: **false**<br><p>The retiring service principal for which to list grants. This filter is only usable by callers in a service principal.</p> <p>You must specify either <code>RetiringPrincipal</code> or <code>RetiringServicePrincipal</code>, but not both.</p><br>
     /// - On success, responds with [`ListRetirableGrantsOutput`](crate::operation::list_retirable_grants::ListRetirableGrantsOutput) with field(s):
     ///   - [`grants(Option<Vec::<GrantListEntry>>)`](crate::operation::list_retirable_grants::ListRetirableGrantsOutput::grants): <p>A list of grants.</p>
```

### `src/client/put_key_policy.rs`

```diff
--- reference/src/client/put_key_policy.rs
+++ generated/src/client/put_key_policy.rs
@@ -3,10 +3,10 @@
     /// Constructs a fluent builder for the [`PutKeyPolicy`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_key_id):<br>required: **true**<br><p>Sets the key policy on the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_key_id):<br>required: **true**<br><p>Sets the key policy on the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`policy_name(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy_name):<br>required: **false**<br><p>The name of the key policy. If no policy name is specified, the default value is <code>default</code>. The only valid value is <code>default</code>.</p><br>
-    ///   - [`policy(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The key policy to attach to the KMS key.</p> <p>The key policy must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul><note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>PutKeyPolicy</code> API request succeeds, even though the policy statement is ineffective.</p>  <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul><note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>.For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p><br>
-    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
+    ///   - [`policy(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The key policy to attach to the KMS key.</p> <p>The key policy must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>PutKeyPolicy</code> API request succeeds, even though the policy statement is ineffective.</p> <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul> <note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>.For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p><br>
+    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
     /// - On success, responds with [`PutKeyPolicyOutput`](crate::operation::put_key_policy::PutKeyPolicyOutput)
     /// - On failure, responds with [`SdkError<PutKeyPolicyError>`](crate::operation::put_key_policy::PutKeyPolicyError)
     pub fn put_key_policy(&self) -> crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder {
```

### `src/client/re_encrypt.rs`

```diff
--- reference/src/client/re_encrypt.rs
+++ generated/src/client/re_encrypt.rs
@@ -5,8 +5,8 @@
     /// - The fluent builder is configurable:
     ///   - [`ciphertext_blob(Blob)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::ciphertext_blob) / [`set_ciphertext_blob(Option<Blob>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_ciphertext_blob):<br>required: **false**<br><p>Ciphertext of the data to reencrypt.</p> <p>This parameter is required in all cases except when <code>DryRun</code> is <code>true</code> and <code>DryRunModifiers</code> is set to <code>IGNORE_CIPHERTEXT</code>.</p><br>
     ///   - [`source_encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_encryption_context) / [`set_source_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_encryption_context):<br>required: **false**<br><p>Specifies the encryption context to use to decrypt the ciphertext. Enter the same encryption context that was used to encrypt the ciphertext.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
-    ///   - [`source_key_id(impl Into<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_key_id) / [`set_source_key_id(Option<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_key_id):<br>required: **false**<br><p>Specifies the KMS key that KMS will use to decrypt the ciphertext before it is re-encrypted.</p> <p>Enter a key ID of the KMS key that was used to encrypt the ciphertext. If you identify a different KMS key, the <code>ReEncrypt</code> operation throws an <code>IncorrectKeyException</code>.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key or when <code>DryRun</code> is <code>true</code> and <code>DryRunModifiers</code> is set to <code>IGNORE_CIPHERTEXT</code>. If you used a symmetric encryption KMS key, KMS can get the KMS key from metadata that it adds to the symmetric ciphertext blob. However, it is always recommended as a best practice. This practice ensures that you use the KMS key that you intend.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you should use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
-    ///   - [`destination_key_id(impl Into<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_key_id) / [`set_destination_key_id(Option<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_key_id):<br>required: **true**<br><p>A unique identifier for the KMS key that is used to reencrypt the data. Specify a symmetric encryption KMS key or an asymmetric KMS key with a <code>KeyUsage</code> value of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> value of a KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`source_key_id(impl Into<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_key_id) / [`set_source_key_id(Option<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_key_id):<br>required: **false**<br><p>Specifies the KMS key that KMS will use to decrypt the ciphertext before it is re-encrypted.</p> <p>Enter a key ID of the KMS key that was used to encrypt the ciphertext. If you identify a different KMS key, the <code>ReEncrypt</code> operation throws an <code>IncorrectKeyException</code>.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key or when <code>DryRun</code> is <code>true</code> and <code>DryRunModifiers</code> is set to <code>IGNORE_CIPHERTEXT</code>. If you used a symmetric encryption KMS key, KMS can get the KMS key from metadata that it adds to the symmetric ciphertext blob. However, it is always recommended as a best practice. This practice ensures that you use the KMS key that you intend.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you should use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
+    ///   - [`destination_key_id(impl Into<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_key_id) / [`set_destination_key_id(Option<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_key_id):<br>required: **true**<br><p>A unique identifier for the KMS key that is used to reencrypt the data. Specify a symmetric encryption KMS key or an asymmetric KMS key with a <code>KeyUsage</code> value of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> value of a KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`destination_encryption_context(impl Into<String>, impl Into<String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_encryption_context) / [`set_destination_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_encryption_context):<br>required: **false**<br><p>Specifies that encryption context to use when the reencrypting the data.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>A destination encryption context is valid only when the destination KMS key is a symmetric encryption KMS key. The standard ciphertext format for asymmetric KMS keys does not include fields for metadata.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`source_encryption_algorithm(EncryptionAlgorithmSpec)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_encryption_algorithm) / [`set_source_encryption_algorithm(Option<EncryptionAlgorithmSpec>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_encryption_algorithm):<br>required: **false**<br><p>Specifies the encryption algorithm that KMS will use to decrypt the ciphertext before it is reencrypted. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the algorithm used for symmetric encryption KMS keys.</p> <p>Specify the same algorithm that was used to encrypt the ciphertext. If you specify a different algorithm, the decrypt attempt fails.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key.</p><br>
     ///   - [`destination_encryption_algorithm(EncryptionAlgorithmSpec)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_encryption_algorithm) / [`set_destination_encryption_algorithm(Option<EncryptionAlgorithmSpec>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_encryption_algorithm):<br>required: **false**<br><p>Specifies the encryption algorithm that KMS will use to reecrypt the data after it has decrypted it. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the encryption algorithm used for symmetric encryption KMS keys.</p> <p>This parameter is required only when the destination KMS key is an asymmetric KMS key.</p><br>
```

### `src/client/replicate_key.rs`

```diff
--- reference/src/client/replicate_key.rs
+++ generated/src/client/replicate_key.rs
@@ -3,12 +3,12 @@
     /// Constructs a fluent builder for the [`ReplicateKey`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the multi-Region primary key that is being replicated. To determine whether a KMS key is a multi-Region primary key, use the <code>DescribeKey</code> operation to check the value of the <code>MultiRegionKeyType</code> property.</p> <p>Specify the key ID or key ARN of a multi-Region primary key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the multi-Region primary key that is being replicated. To determine whether a KMS key is a multi-Region primary key, use the <a>DescribeKey</a> operation to check the value of the <code>MultiRegionKeyType</code> property.</p> <p>Specify the key ID or key ARN of a multi-Region primary key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`replica_region(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::replica_region) / [`set_replica_region(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_replica_region):<br>required: **true**<br><p>The Region ID of the Amazon Web Services Region for this replica key.</p> <p>Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. For a list of Amazon Web Services Regions in which KMS is supported, see <a href="https://docs.aws.amazon.com/general/latest/gr/kms.html#kms_region">KMS service endpoints</a> in the <i>Amazon Web Services General Reference</i>.</p> <p>The replica must be in a different Amazon Web Services Region than its primary key and other replicas of that primary key, but in the same Amazon Web Services partition. KMS must be available in the replica Region. If the Region is not enabled by default, the Amazon Web Services account must be enabled in the Region. For information about Amazon Web Services partitions, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. For information about enabling and disabling Regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">Enabling a Region</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-disable">Disabling a Region</a> in the <i>Amazon Web Services General Reference</i>.</p><br>
-    ///   - [`policy(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_policy):<br>required: **false**<br><p>The key policy to attach to the KMS key. This parameter is optional. If you do not provide a key policy, KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">default key policy</a> to the KMS key.</p> <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p><br>
-    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
+    ///   - [`policy(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_policy):<br>required: **false**<br><p>The key policy to attach to the KMS key. This parameter is optional. If you do not provide a key policy, KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">default key policy</a> to the KMS key.</p> <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p><br>
+    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
     ///   - [`description(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_description):<br>required: **false**<br><p>A description of the KMS key. The default value is an empty string (no description).</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>The description is not a shared property of multi-Region keys. You can specify the same description or a different description for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p><br>
-    ///   - [`tags(Tag)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_tags):<br>required: **false**<br><p>Assigns one or more tags to the replica key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <code>TagResource</code> operation.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <note>  <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Tags are not a shared property of multi-Region keys. You can specify the same tags or different tags for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p> <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tags in KMS</a>.</p><br>
+    ///   - [`tags(Tag)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_tags):<br>required: **false**<br><p>Assigns one or more tags to the replica key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <a>TagResource</a> operation.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <note>  <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Tags are not a shared property of multi-Region keys. You can specify the same tags or different tags for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p> <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tags in KMS</a>.</p><br>
     /// - On success, responds with [`ReplicateKeyOutput`](crate::operation::replicate_key::ReplicateKeyOutput) with field(s):
     ///   - [`replica_key_metadata(Option<KeyMetadata>)`](crate::operation::replicate_key::ReplicateKeyOutput::replica_key_metadata): <p>Displays details about the new replica key, including its Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a>. It also includes the ARN and Amazon Web Services Region of its primary key and other replica keys.</p>
     ///   - [`replica_policy(Option<String>)`](crate::operation::replicate_key::ReplicateKeyOutput::replica_policy): <p>The key policy of the new replica key. The value is a key policy document in JSON format.</p>
```

### `src/client/retire_grant.rs`

```diff
--- reference/src/client/retire_grant.rs
+++ generated/src/client/retire_grant.rs
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`RetireGrant`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`grant_token(impl Into<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::grant_token) / [`set_grant_token(Option<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_grant_token):<br>required: **false**<br><p>Identifies the grant to be retired. You can use a grant token to identify a new grant even before it has achieved eventual consistency.</p> <p>Only the <code>CreateGrant</code> operation returns a grant token. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-eventual-consistency">Eventual consistency</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
-    ///   - [`key_id(impl Into<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_key_id):<br>required: **false**<br><p>The key ARN KMS key associated with the grant. To find the key ARN, use the <code>ListKeys</code> operation.</p> <p>For example: <code>arn:aws:kms:us-east-2:444455556666:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p><br>
-    ///   - [`grant_id(impl Into<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::grant_id) / [`set_grant_id(Option<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_grant_id):<br>required: **false**<br><p>Identifies the grant to retire. To get the grant ID, use <code>CreateGrant</code>, <code>ListGrants</code>, or <code>ListRetirableGrants</code>.</p> <ul>  <li>   <p>Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123</p></li> </ul><br>
+    ///   - [`grant_token(impl Into<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::grant_token) / [`set_grant_token(Option<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_grant_token):<br>required: **false**<br><p>Identifies the grant to be retired. You can use a grant token to identify a new grant even before it has achieved eventual consistency.</p> <p>Only the <a>CreateGrant</a> operation returns a grant token. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-eventual-consistency">Eventual consistency</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_key_id):<br>required: **false**<br><p>The key ARN KMS key associated with the grant. To find the key ARN, use the <a>ListKeys</a> operation.</p> <p>For example: <code>arn:aws:kms:us-east-2:444455556666:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p><br>
+    ///   - [`grant_id(impl Into<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::grant_id) / [`set_grant_id(Option<String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_grant_id):<br>required: **false**<br><p>Identifies the grant to retire. To get the grant ID, use <a>CreateGrant</a>, <a>ListGrants</a>, or <a>ListRetirableGrants</a>.</p> <ul>  <li>   <p>Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123</p></li> </ul><br>
     ///   - [`dry_run(bool)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`RetireGrantOutput`](crate::operation::retire_grant::RetireGrantOutput)
     /// - On failure, responds with [`SdkError<RetireGrantError>`](crate::operation::retire_grant::RetireGrantError)
```

### `src/client/revoke_grant.rs`

```diff
--- reference/src/client/revoke_grant.rs
+++ generated/src/client/revoke_grant.rs
@@ -3,8 +3,8 @@
     /// Constructs a fluent builder for the [`RevokeGrant`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_key_id):<br>required: **true**<br><p>A unique identifier for the KMS key associated with the grant. To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`grant_id(impl Into<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::grant_id) / [`set_grant_id(Option<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_grant_id):<br>required: **true**<br><p>Identifies the grant to revoke. To get the grant ID, use <code>CreateGrant</code>, <code>ListGrants</code>, or <code>ListRetirableGrants</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_key_id):<br>required: **true**<br><p>A unique identifier for the KMS key associated with the grant. To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`grant_id(impl Into<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::grant_id) / [`set_grant_id(Option<String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_grant_id):<br>required: **true**<br><p>Identifies the grant to revoke. To get the grant ID, use <a>CreateGrant</a>, <a>ListGrants</a>, or <a>ListRetirableGrants</a>.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`RevokeGrantOutput`](crate::operation::revoke_grant::RevokeGrantOutput)
     /// - On failure, responds with [`SdkError<RevokeGrantError>`](crate::operation::revoke_grant::RevokeGrantError)
```

### `src/client/rotate_key_on_demand.rs`

```diff
--- reference/src/client/rotate_key_on_demand.rs
+++ generated/src/client/rotate_key_on_demand.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`RotateKeyOnDemand`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot perform on-demand rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, multi-Region KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. To perform on-demand rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#multi-region-rotate">multi-Region keys</a>, invoke the on-demand rotation on the primary key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot perform on-demand rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, multi-Region KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. To perform on-demand rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#multi-region-rotate">multi-Region keys</a>, invoke the on-demand rotation on the primary key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     /// - On success, responds with [`RotateKeyOnDemandOutput`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandOutput::key_id): <p>Identifies the symmetric encryption KMS key that you initiated on-demand rotation on.</p>
     /// - On failure, responds with [`SdkError<RotateKeyOnDemandError>`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandError)
```

### `src/client/schedule_key_deletion.rs`

```diff
--- reference/src/client/schedule_key_deletion.rs
+++ generated/src/client/schedule_key_deletion.rs
@@ -3,8 +3,8 @@
     /// Constructs a fluent builder for the [`ScheduleKeyDeletion`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_key_id):<br>required: **true**<br><p>The unique identifier of the KMS key to delete.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
-    ///   - [`pending_window_in_days(i32)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::pending_window_in_days) / [`set_pending_window_in_days(Option<i32>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_pending_window_in_days):<br>required: **false**<br><p>The waiting period, specified in number of days. After the waiting period ends, KMS deletes the KMS key.</p> <p>If the KMS key is a multi-Region primary key with replica keys, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30. You can use the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-schedule-key-deletion-pending-window-in-days"> <code>kms:ScheduleKeyDeletionPendingWindowInDays</code> </a> condition key to further constrain the values that principals can specify in the <code>PendingWindowInDays</code> parameter.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_key_id):<br>required: **true**<br><p>The unique identifier of the KMS key to delete.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
+    ///   - [`pending_window_in_days(i32)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::pending_window_in_days) / [`set_pending_window_in_days(Option<i32>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_pending_window_in_days):<br>required: **false**<br><p>The waiting period, specified in number of days. After the waiting period ends, KMS deletes the KMS key.</p> <p>If the KMS key is a multi-Region primary key with replica keys, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30. You can use the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-schedule-key-deletion-pending-window-in-days"> <code>kms:ScheduleKeyDeletionPendingWindowInDays</code></a> condition key to further constrain the values that principals can specify in the <code>PendingWindowInDays</code> parameter.</p><br>
     /// - On success, responds with [`ScheduleKeyDeletionOutput`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key whose deletion is scheduled.</p>
     ///   - [`deletion_date(Option<DateTime>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionOutput::deletion_date): <p>The date and time after which KMS deletes the KMS key.</p> <p>If the KMS key is a multi-Region primary key with replica keys, this field does not appear. The deletion date for the primary key isn't known until its last replica key is deleted.</p>
```

### `src/client/sign.rs`

```diff
--- reference/src/client/sign.rs
+++ generated/src/client/sign.rs
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`Sign`](crate::operation::sign::builders::SignFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::sign::builders::SignFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::sign::builders::SignFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies an asymmetric KMS key. KMS uses the private key in the asymmetric KMS key to sign the message. The <code>KeyUsage</code> type of the KMS key must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::sign::builders::SignFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::sign::builders::SignFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies an asymmetric KMS key. KMS uses the private key in the asymmetric KMS key to sign the message. The <code>KeyUsage</code> type of the KMS key must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a KMS key, use the <a>DescribeKey</a> operation.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`message(Blob)`](crate::operation::sign::builders::SignFluentBuilder::message) / [`set_message(Option<Blob>)`](crate::operation::sign::builders::SignFluentBuilder::set_message):<br>required: **true**<br><p>Specifies the message or message digest to sign. Messages can be 0-4096 bytes. To sign a larger message, provide a message digest.</p> <p>If you provide a message digest, use the <code>DIGEST</code> value of <code>MessageType</code> to prevent the digest from being hashed again while signing.</p><br>
-    ///   - [`message_type(MessageType)`](crate::operation::sign::builders::SignFluentBuilder::message_type) / [`set_message_type(Option<MessageType>)`](crate::operation::sign::builders::SignFluentBuilder::set_message_type):<br>required: **false**<br><p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p> <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>  <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p> </important> <p>When using ECC_NIST_EDWARDS25519 KMS keys:</p> <ul>  <li>   <p>ED25519_SHA_512 signing algorithm requires KMS <code>MessageType:RAW</code></p></li>  <li>   <p>ED25519_PH_SHA_512 signing algorithm requires KMS <code>MessageType:DIGEST</code></p></li> </ul><important>  <p>When you specify the ED25519_PH_SHA_512 signing algorithm with <code>MessageType:DIGEST</code>, KMS still performs the SHA-512 prehash described in <a href="https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf#page=39">Step 1 of Section 7.8.1 in FIPS 186-5</a>. This means the input is hashed twice: once by you and once by KMS.</p> </important> <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p> <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p> <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, this can cause verification failures when verifying with a system that assumes a single hash.</p> <p>The hashing algorithm that <code>Sign</code> uses is based on the <code>SigningAlgorithm</code> value.</p> <ul>  <li>   <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>  <li>   <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li> </ul><br>
+    ///   - [`message_type(MessageType)`](crate::operation::sign::builders::SignFluentBuilder::message_type) / [`set_message_type(Option<MessageType>)`](crate::operation::sign::builders::SignFluentBuilder::set_message_type):<br>required: **false**<br><p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p> <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>  <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p> </important> <p>When using ECC_NIST_EDWARDS25519 KMS keys:</p> <ul>  <li>   <p>ED25519_SHA_512 signing algorithm requires KMS <code>MessageType:RAW</code></p></li>  <li>   <p>ED25519_PH_SHA_512 signing algorithm requires KMS <code>MessageType:DIGEST</code></p></li> </ul> <important>  <p>When you specify the ED25519_PH_SHA_512 signing algorithm with <code>MessageType:DIGEST</code>, KMS still performs the SHA-512 prehash described in <a href="https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf#page=39">Step 1 of Section 7.8.1 in FIPS 186-5</a>. This means the input is hashed twice: once by you and once by KMS.</p> </important> <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p> <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p> <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, this can cause verification failures when verifying with a system that assumes a single hash.</p> <p>The hashing algorithm that <code>Sign</code> uses is based on the <code>SigningAlgorithm</code> value.</p> <ul>  <li>   <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>  <li>   <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li> </ul><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::sign::builders::SignFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::sign::builders::SignFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`signing_algorithm(SigningAlgorithmSpec)`](crate::operation::sign::builders::SignFluentBuilder::signing_algorithm) / [`set_signing_algorithm(Option<SigningAlgorithmSpec>)`](crate::operation::sign::builders::SignFluentBuilder::set_signing_algorithm):<br>required: **true**<br><p>Specifies the signing algorithm to use when signing the message.</p> <p>Choose an algorithm that is compatible with the type and size of the specified asymmetric KMS key. When signing with RSA key pairs, RSASSA-PSS algorithms are preferred. We include RSASSA-PKCS1-v1_5 algorithms for compatibility with existing applications.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::sign::builders::SignFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::sign::builders::SignFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/tag_resource.rs`

```diff
--- reference/src/client/tag_resource.rs
+++ generated/src/client/tag_resource.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a customer managed key in the account and Region.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a customer managed key in the account and Region.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`tags(Tag)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tags):<br>required: **true**<br><p>One or more tags. Each tag consists of a tag key and a tag value. The tag value can be an empty (null) string.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p><br>
     /// - On success, responds with [`TagResourceOutput`](crate::operation::tag_resource::TagResourceOutput)
     /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
```

### `src/client/untag_resource.rs`

```diff
--- reference/src/client/untag_resource.rs
+++ generated/src/client/untag_resource.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`UntagResource`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key from which you are removing tags.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the KMS key from which you are removing tags.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`tag_keys(impl Into<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec::<String>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_tag_keys):<br>required: **true**<br><p>One or more tag keys. Specify only the tag keys, not the tag values.</p><br>
     /// - On success, responds with [`UntagResourceOutput`](crate::operation::untag_resource::UntagResourceOutput)
     /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::operation::untag_resource::UntagResourceError)
```

### `src/client/update_alias.rs`

```diff
--- reference/src/client/update_alias.rs
+++ generated/src/client/update_alias.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`alias_name(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::alias_name) / [`set_alias_name(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_alias_name):<br>required: **true**<br><p>Identifies the alias that is changing its KMS key. This value must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>. You cannot use <code>UpdateAlias</code> to change the alias name.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important><br>
-    ///   - [`target_key_id(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_target_key_id):<br>required: **true**<br><p>Identifies the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-mgn-key">customer managed key</a> to associate with the alias. You don't have permission to associate an alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed key</a>.</p> <p>The KMS key must be in the same Amazon Web Services account and Region as the alias. Also, the new target KMS key must be the same type as the current target KMS key (both symmetric or both asymmetric or both HMAC) and they must have the same key usage.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p> <p>To verify that the alias is mapped to the correct KMS key, use <code>ListAliases</code>.</p><br>
+    ///   - [`target_key_id(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_target_key_id):<br>required: **true**<br><p>Identifies the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-mgn-key">customer managed key</a> to associate with the alias. You don't have permission to associate an alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed key</a>.</p> <p>The KMS key must be in the same Amazon Web Services account and Region as the alias. Also, the new target KMS key must be the same type as the current target KMS key (both symmetric or both asymmetric or both HMAC) and they must have the same key usage.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>To verify that the alias is mapped to the correct KMS key, use <a>ListAliases</a>.</p><br>
     /// - On success, responds with [`UpdateAliasOutput`](crate::operation::update_alias::UpdateAliasOutput)
     /// - On failure, responds with [`SdkError<UpdateAliasError>`](crate::operation::update_alias::UpdateAliasError)
     pub fn update_alias(&self) -> crate::operation::update_alias::builders::UpdateAliasFluentBuilder {
```

### `src/client/update_custom_key_store.rs`

```diff
--- reference/src/client/update_custom_key_store.rs
+++ generated/src/client/update_custom_key_store.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`UpdateCustomKeyStore`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Identifies the custom key store that you want to update. Enter the ID of the custom key store. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Identifies the custom key store that you want to update. Enter the ID of the custom key store. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p><br>
     ///   - [`new_custom_key_store_name(impl Into<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::new_custom_key_store_name) / [`set_new_custom_key_store_name(Option<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::set_new_custom_key_store_name):<br>required: **false**<br><p>Changes the friendly name of the custom key store to the value that you specify. The custom key store name must be unique in the Amazon Web Services account.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>To change this value, the custom key store can be connected or disconnected.</p><br>
     ///   - [`key_store_password(impl Into<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::key_store_password) / [`set_key_store_password(Option<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::set_key_store_password):<br>required: **false**<br><p>Enter the current password of the <code>kmsuser</code> crypto user (CU) in the CloudHSM cluster that is associated with the custom key store. This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p> <p>This parameter tells KMS the current password of the <code>kmsuser</code> crypto user (CU). It does not set or change the password of any users in the CloudHSM cluster.</p> <p>To change this value, the CloudHSM key store must be disconnected.</p><br>
     ///   - [`cloud_hsm_cluster_id(impl Into<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::cloud_hsm_cluster_id) / [`set_cloud_hsm_cluster_id(Option<String>)`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreFluentBuilder::set_cloud_hsm_cluster_id):<br>required: **false**<br><p>Associates the custom key store with a related CloudHSM cluster. This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p> <p>Enter the cluster ID of the cluster that you used to create the custom key store or a cluster that shares a backup history and has the same cluster certificate as the original cluster. You cannot use this parameter to associate a custom key store with an unrelated cluster. In addition, the replacement cluster must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">fulfill the requirements</a> for a cluster associated with a custom key store. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p> <p>To change this value, the CloudHSM key store must be disconnected.</p><br>
```

### `src/client/update_key_description.rs`

```diff
--- reference/src/client/update_key_description.rs
+++ generated/src/client/update_key_description.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`UpdateKeyDescription`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::set_key_id):<br>required: **true**<br><p>Updates the description of the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::set_key_id):<br>required: **true**<br><p>Updates the description of the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`description(impl Into<String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::set_description):<br>required: **true**<br><p>New description for the KMS key.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important><br>
     /// - On success, responds with [`UpdateKeyDescriptionOutput`](crate::operation::update_key_description::UpdateKeyDescriptionOutput)
     /// - On failure, responds with [`SdkError<UpdateKeyDescriptionError>`](crate::operation::update_key_description::UpdateKeyDescriptionError)
```

### `src/client/update_primary_region.rs`

```diff
--- reference/src/client/update_primary_region.rs
+++ generated/src/client/update_primary_region.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`UpdatePrimaryRegion`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the current primary key. When the operation completes, this KMS key will be a replica key.</p> <p>Specify the key ID or key ARN of a multi-Region primary key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the current primary key. When the operation completes, this KMS key will be a replica key.</p> <p>Specify the key ID or key ARN of a multi-Region primary key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`primary_region(impl Into<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::primary_region) / [`set_primary_region(Option<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_primary_region):<br>required: **true**<br><p>The Amazon Web Services Region of the new primary key. Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. There must be an existing replica key in this Region.</p> <p>When the operation completes, the multi-Region key in this Region will be the primary key.</p><br>
     /// - On success, responds with [`UpdatePrimaryRegionOutput`](crate::operation::update_primary_region::UpdatePrimaryRegionOutput)
     /// - On failure, responds with [`SdkError<UpdatePrimaryRegionError>`](crate::operation::update_primary_region::UpdatePrimaryRegionError)
```

### `src/client/verify.rs`

```diff
--- reference/src/client/verify.rs
+++ generated/src/client/verify.rs
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`Verify`](crate::operation::verify::builders::VerifyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::verify::builders::VerifyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the asymmetric KMS key that will be used to verify the signature. This must be the same KMS key that was used to generate the signature. If you specify a different KMS key, the signature verification fails.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::verify::builders::VerifyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the asymmetric KMS key that will be used to verify the signature. This must be the same KMS key that was used to generate the signature. If you specify a different KMS key, the signature verification fails.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p><br>
     ///   - [`message(Blob)`](crate::operation::verify::builders::VerifyFluentBuilder::message) / [`set_message(Option<Blob>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_message):<br>required: **true**<br><p>Specifies the message that was signed. You can submit a raw message of up to 4096 bytes, or a hash digest of the message. If you submit a digest, use the <code>MessageType</code> parameter with a value of <code>DIGEST</code>.</p> <p>If the message specified here is different from the message that was signed, the signature verification fails. A message and its hash digest are considered to be the same message.</p><br>
-    ///   - [`message_type(MessageType)`](crate::operation::verify::builders::VerifyFluentBuilder::message_type) / [`set_message_type(Option<MessageType>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_message_type):<br>required: **false**<br><p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p> <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>  <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p> </important> <p>When using ECC_NIST_EDWARDS25519 KMS keys:</p> <ul>  <li>   <p>ED25519_SHA_512 signing algorithm requires KMS <code>MessageType:RAW</code></p></li>  <li>   <p>ED25519_PH_SHA_512 signing algorithm requires KMS <code>MessageType:DIGEST</code></p></li> </ul><important>  <p>When you specify the ED25519_PH_SHA_512 signing algorithm with <code>MessageType:DIGEST</code>, KMS still performs the SHA-512 prehash described in <a href="https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf#page=39">Step 1 of Section 7.8.1 in FIPS 186-5</a>. This means the input is hashed twice: once by you and once by KMS.</p> </important> <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p> <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p> <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, if the signed message is hashed once while signing, but twice while verifying, verification fails, even when the message hasn't changed.</p> <p>The hashing algorithm that <code>Verify</code> uses is based on the <code>SigningAlgorithm</code> value.</p> <ul>  <li>   <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>  <li>   <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li> </ul><br>
+    ///   - [`message_type(MessageType)`](crate::operation::verify::builders::VerifyFluentBuilder::message_type) / [`set_message_type(Option<MessageType>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_message_type):<br>required: **false**<br><p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p> <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>  <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p> </important> <p>When using ECC_NIST_EDWARDS25519 KMS keys:</p> <ul>  <li>   <p>ED25519_SHA_512 signing algorithm requires KMS <code>MessageType:RAW</code></p></li>  <li>   <p>ED25519_PH_SHA_512 signing algorithm requires KMS <code>MessageType:DIGEST</code></p></li> </ul> <important>  <p>When you specify the ED25519_PH_SHA_512 signing algorithm with <code>MessageType:DIGEST</code>, KMS still performs the SHA-512 prehash described in <a href="https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf#page=39">Step 1 of Section 7.8.1 in FIPS 186-5</a>. This means the input is hashed twice: once by you and once by KMS.</p> </important> <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p> <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p> <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, if the signed message is hashed once while signing, but twice while verifying, verification fails, even when the message hasn't changed.</p> <p>The hashing algorithm that <code>Verify</code> uses is based on the <code>SigningAlgorithm</code> value.</p> <ul>  <li>   <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>  <li>   <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>  <li>   <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li> </ul><br>
     ///   - [`signature(Blob)`](crate::operation::verify::builders::VerifyFluentBuilder::signature) / [`set_signature(Option<Blob>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_signature):<br>required: **true**<br><p>The signature that the <code>Sign</code> operation generated.</p><br>
     ///   - [`signing_algorithm(SigningAlgorithmSpec)`](crate::operation::verify::builders::VerifyFluentBuilder::signing_algorithm) / [`set_signing_algorithm(Option<SigningAlgorithmSpec>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_signing_algorithm):<br>required: **true**<br><p>The signing algorithm that was used to sign the message. If you submit a different algorithm, the signature verification fails.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::verify::builders::VerifyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
```

### `src/client/verify_mac.rs`

```diff
--- reference/src/client/verify_mac.rs
+++ generated/src/client/verify_mac.rs
@@ -3,10 +3,10 @@
     /// Constructs a fluent builder for the [`VerifyMac`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`message(Blob)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::message) / [`set_message(Option<Blob>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_message):<br>required: **true**<br><p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p> <p><code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p><br>
+    ///   - [`message(Blob)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::message) / [`set_message(Option<Blob>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_message):<br>required: **true**<br><p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p> <p><a>GenerateMac</a> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p><br>
     ///   - [`key_id(impl Into<String>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_key_id):<br>required: **true**<br><p>The KMS key that will be used in the verification.</p> <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p><br>
     ///   - [`mac_algorithm(MacAlgorithmSpec)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::mac_algorithm) / [`set_mac_algorithm(Option<MacAlgorithmSpec>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_mac_algorithm):<br>required: **true**<br><p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p><br>
-    ///   - [`mac(Blob)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::mac) / [`set_mac(Option<Blob>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_mac):<br>required: **true**<br><p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p><br>
+    ///   - [`mac(Blob)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::mac) / [`set_mac(Option<Blob>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_mac):<br>required: **true**<br><p>The HMAC to verify. Enter the HMAC that was generated by the <a>GenerateMac</a> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p><br>
     ///   - [`grant_tokens(impl Into<String>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p> <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
     /// - On success, responds with [`VerifyMacOutput`](crate::operation::verify_mac::VerifyMacOutput) with field(s):
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -1,1719 +1,45 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#![allow(clippy::empty_line_after_doc_comments)]
-/// Configuration for a aws_sdk_kms service client.
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
-        "kms"
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
-    /// use aws_sdk_kms::config::Config;
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
-    /// use aws_sdk_kms::config::{Builder, Config};
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
-    /// let mut builder = aws_sdk_kms::Config::builder();
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
-    /// impl aws_sdk_kms::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_kms::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         AuthSchemeOptionsFuture::ready(Ok(vec![AuthSchemeOption::from(AuthSchemeId::new(
-    ///             "custom",
-    ///         ))]))
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
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
-    /// impl aws_sdk_kms::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_kms::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
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
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-    /// rules for `aws_sdk_kms`.
-    ///
-    ///
-    /// Note: setting an endpoint resolver will replace any endpoint URL that has been set.
-    /// This method accepts an endpoint resolver [specific to this service](crate::config::endpoint::ResolveEndpoint). If you want to
-    /// provide a shared endpoint resolver, use [`Self::set_endpoint_resolver`].
-    ///
-    /// # Examples
-    /// Create a custom endpoint resolver that resolves a different endpoing per-stage, e.g. staging vs. production.
-    /// ```no_run
-    /// use aws_sdk_kms::config::endpoint::{ResolveEndpoint, EndpointFuture, Params, Endpoint};
-    /// #[derive(Debug)]
-    /// struct StageResolver { stage: String }
-    /// impl ResolveEndpoint for StageResolver {
-    ///     fn resolve_endpoint(&self, params: &Params) -> EndpointFuture<'_> {
-    ///         let stage = &self.stage;
-    ///         EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{stage}.myservice.com")).build()))
-    ///     }
-    /// }
-    /// let resolver = StageResolver { stage: std::env::var("STAGE").unwrap() };
-    /// let config = aws_sdk_kms::Config::builder().endpoint_resolver(resolver).build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-    /// rules for `aws_sdk_kms`.
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
-    /// use aws_sdk_kms::config::Config;
-    /// use aws_sdk_kms::config::retry::RetryConfig;
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
-    /// use aws_sdk_kms::config::{Builder, Config};
-    /// use aws_sdk_kms::config::retry::RetryConfig;
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
-    /// use aws_sdk_kms::config::{AsyncSleep, Config, SharedAsyncSleep, Sleep};
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
-    /// use aws_sdk_kms::config::{AsyncSleep, Builder, Config, SharedAsyncSleep, Sleep};
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
-    /// use aws_sdk_kms::config::Config;
-    /// use aws_sdk_kms::config::timeout::TimeoutConfig;
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
-    /// use aws_sdk_kms::config::{Builder, Config};
-    /// use aws_sdk_kms::config::timeout::TimeoutConfig;
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
-    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `kms`
-    /// (or `kms-<region>` if a region is configured).
-    /// All KMS clients without an explicit retry partition will share this default partition.
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
-    /// use aws_sdk_kms::config::Config;
-    /// use aws_sdk_kms::config::retry::{RetryPartition, TokenBucket};
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
-    /// use aws_sdk_kms::config::Config;
-    /// use aws_sdk_kms::config::retry::{RetryPartition, TokenBucket};
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
-    /// use aws_sdk_kms::config::Config;
-    /// use aws_sdk_kms::config::retry::{ClientRateLimiter, RetryConfig, RetryPartition};
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
-    /// use aws_sdk_kms::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_kms::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-    /// use aws_sdk_kms::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_kms::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-    /// use aws_sdk_kms::config::Config;
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
-    /// use aws_sdk_kms::config::Config;
-    /// # #[derive(Debug)]
-    /// # struct SomeOperationError {}
-    /// # impl StdError for SomeOperationError {}
-    /// # impl fmt::Display for SomeOperationError {
-    /// #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
-    /// # }
-    /// # impl ProvideErrorMetadata for SomeOperationError {
-    /// #    fn meta(&self) -> &aws_sdk_kms::error::ErrorMetadata { todo!() }
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
-    /// use aws_sdk_kms::config::{Builder, Config};
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
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
-    /// use aws_sdk_kms::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_kms::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-    /// use aws_sdk_kms::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_kms::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_kms::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_kms::Client::from_conf(config);
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
-        layer.store_put(::aws_types::SigningName::from_static("kms"));
-        layer
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| layer.store_put(::aws_types::region::SigningRegion::from(r)));
-        Config {
-            config: crate::config::Layer::from(layer.clone())
-                .with_name("aws_sdk_kms::config::Config")
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
-            let mut cfg = ::aws_smithy_types::config_bag::Layer::new("TrentService");
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
-                .with_name("aws_sdk_kms::config::ConfigOverrideRuntimePlugin")
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
-                        conf.load_config(service_config_key("KMS", "AWS_ENDPOINT_URL", "endpoint_url"))
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
-    let default_retry_partition = "kms";
-    let default_retry_partition = match config.region() {
-        Some(region) => ::std::borrow::Cow::from(format!("{default_retry_partition}-{region}")),
-        None => ::std::borrow::Cow::from(default_retry_partition),
-    };
-
-    let scope = "aws-sdk-kms";
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

### `src/operation/cancel_key_deletion.rs`

```diff
--- reference/src/operation/cancel_key_deletion.rs
+++ generated/src/operation/cancel_key_deletion.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CancelKeyDeletion", "KMS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CancelKeyDeletion")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CancelKeyDeletionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CancelKeyDeletionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::cancel_key_deletion::CancelKeyDeletionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::cancel_key_deletion::CancelKeyDeletionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::cancel_key_deletion::CancelKeyDeletionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CancelKeyDeletion")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(CancelKeyDeletionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(CancelKeyDeletionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::cancel_key_deletion::CancelKeyDeletionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::cancel_key_deletion::CancelKeyDeletionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::cancel_key_deletion::CancelKeyDeletionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.CancelKeyDeletion",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_cancel_key_deletion::ser_cancel_key_deletion_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_cancel_key_deletion_input::ser_cancel_key_deletion_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +439,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::cancel_key_deletion::CancelKeyDeletionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::cancel_key_deletion::CancelKeyDeletionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/connect_custom_key_store.rs`

```diff
--- reference/src/operation/connect_custom_key_store.rs
+++ generated/src/operation/connect_custom_key_store.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ConnectCustomKeyStore")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ConnectCustomKeyStoreTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ConnectCustomKeyStoreEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ConnectCustomKeyStore")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ConnectCustomKeyStoreTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ConnectCustomKeyStoreEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError,
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
@@ -250,16 +259,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ConnectCustomKeyStore",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_connect_custom_key_store::ser_connect_custom_key_store_input(&input)?,
+            crate::protocol_serde::shape_connect_custom_key_store_input::ser_connect_custom_key_store_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -294,8 +298,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +474,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_alias.rs`

```diff
--- reference/src/operation/create_alias.rs
+++ generated/src/operation/create_alias.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateAlias", "KMS"));
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
                 crate::operation::create_alias::CreateAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_alias::CreateAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_alias::CreateAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.CreateAlias",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_alias::ser_create_alias_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_alias_input::ser_create_alias_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -475,6 +476,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_alias::CreateAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_alias::CreateAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_custom_key_store.rs`

```diff
--- reference/src/operation/create_custom_key_store.rs
+++ generated/src/operation/create_custom_key_store.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateCustomKeyStore")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateCustomKeyStoreTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateCustomKeyStoreEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::create_custom_key_store::CreateCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::create_custom_key_store::CreateCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_custom_key_store::CreateCustomKeyStoreError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateCustomKeyStore")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(CreateCustomKeyStoreTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(CreateCustomKeyStoreEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::create_custom_key_store::CreateCustomKeyStoreError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::create_custom_key_store::CreateCustomKeyStoreError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_custom_key_store::CreateCustomKeyStoreError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -280,17 +272,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.CreateCustomKeyStore",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_custom_key_store::ser_create_custom_key_store_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_create_custom_key_store_input::ser_create_custom_key_store_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -324,8 +311,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -615,6 +602,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_custom_key_store::CreateCustomKeyStoreError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_custom_key_store::CreateCustomKeyStoreError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_grant.rs`

```diff
--- reference/src/operation/create_grant.rs
+++ generated/src/operation/create_grant.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateGrant", "KMS"));
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
                 crate::operation::create_grant::CreateGrantError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_grant::CreateGrantError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_grant::CreateGrantError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -272,15 +278,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.CreateGrant",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_grant::ser_create_grant_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_grant_input::ser_create_grant_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -314,8 +315,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -515,6 +516,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_grant::CreateGrantError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_grant::CreateGrantError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_key/_create_key_input.rs`

```diff
--- reference/src/operation/create_key/_create_key_input.rs
+++ generated/src/operation/create_key/_create_key_input.rs
@@ -923,7 +923,7 @@
             key_spec: self.key_spec,
             origin: self.origin,
             custom_key_store_id: self.custom_key_store_id,
-            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
+            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check.unwrap_or_default(),
             tags: self.tags,
             multi_region: self.multi_region,
             xks_key_id: self.xks_key_id,
```

### `src/operation/create_key.rs`

```diff
--- reference/src/operation/create_key.rs
+++ generated/src/operation/create_key.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateKey", "KMS"));
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
                 crate::operation::create_key::CreateKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_key::CreateKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_key::CreateKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -260,15 +266,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.CreateKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_key::ser_create_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_key_input::ser_create_key_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -302,8 +303,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -560,6 +561,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_key::CreateKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_key::CreateKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/decrypt.rs`

```diff
--- reference/src/operation/decrypt.rs
+++ generated/src/operation/decrypt.rs
@@ -101,9 +101,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Decrypt", "KMS"));
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
                 crate::operation::decrypt::DecryptError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::decrypt::DecryptError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::decrypt::DecryptError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -242,15 +248,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.Decrypt",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_decrypt::ser_decrypt_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_decrypt_input::ser_decrypt_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -284,8 +285,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -514,6 +515,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::decrypt::DecryptError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::decrypt::DecryptError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_alias.rs`

```diff
--- reference/src/operation/delete_alias.rs
+++ generated/src/operation/delete_alias.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteAlias", "KMS"));
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
                 crate::operation::delete_alias::DeleteAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_alias::DeleteAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_alias::DeleteAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DeleteAlias",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_alias::ser_delete_alias_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_alias_input::ser_delete_alias_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -440,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_alias::DeleteAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_alias::DeleteAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_custom_key_store.rs`

```diff
--- reference/src/operation/delete_custom_key_store.rs
+++ generated/src/operation/delete_custom_key_store.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteCustomKeyStore")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteCustomKeyStoreTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteCustomKeyStoreEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteCustomKeyStore")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteCustomKeyStoreTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteCustomKeyStoreEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,17 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DeleteCustomKeyStore",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_custom_key_store::ser_delete_custom_key_store_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_delete_custom_key_store_input::ser_delete_custom_key_store_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +281,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +437,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_imported_key_material.rs`

```diff
--- reference/src/operation/delete_imported_key_material.rs
+++ generated/src/operation/delete_imported_key_material.rs
@@ -107,9 +107,9 @@
             "KMS",
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
                 crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError,
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
@@ -255,16 +263,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DeleteImportedKeyMaterial",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_delete_imported_key_material::ser_delete_imported_key_material_input(&input)?,
+            crate::protocol_serde::shape_delete_imported_key_material_input::ser_delete_imported_key_material_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -299,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +473,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/derive_shared_secret.rs`

```diff
--- reference/src/operation/derive_shared_secret.rs
+++ generated/src/operation/derive_shared_secret.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeriveSharedSecret", "KMS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -123,25 +123,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeriveSharedSecret")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeriveSharedSecretTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeriveSharedSecretEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::derive_shared_secret::DeriveSharedSecretError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::derive_shared_secret::DeriveSharedSecretError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::derive_shared_secret::DeriveSharedSecretError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeriveSharedSecret")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeriveSharedSecretTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeriveSharedSecretEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::derive_shared_secret::DeriveSharedSecretError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::derive_shared_secret::DeriveSharedSecretError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::derive_shared_secret::DeriveSharedSecretError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -248,16 +240,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DeriveSharedSecret",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_derive_shared_secret::ser_derive_shared_secret_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_derive_shared_secret_input::ser_derive_shared_secret_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -291,8 +279,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -500,6 +488,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::derive_shared_secret::DeriveSharedSecretError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::derive_shared_secret::DeriveSharedSecretError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_custom_key_stores.rs`

```diff
--- reference/src/operation/describe_custom_key_stores.rs
+++ generated/src/operation/describe_custom_key_stores.rs
@@ -108,9 +108,9 @@
             "KMS",
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
                 crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError,
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
@@ -261,16 +269,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DescribeCustomKeyStores",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_describe_custom_key_stores::ser_describe_custom_key_stores_input(&input)?,
+            crate::protocol_serde::shape_describe_custom_key_stores_input::ser_describe_custom_key_stores_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -305,8 +308,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -438,6 +441,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_key.rs`

```diff
--- reference/src/operation/describe_key.rs
+++ generated/src/operation/describe_key.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DescribeKey", "KMS"));
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
                 crate::operation::describe_key::DescribeKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_key::DescribeKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_key::DescribeKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DescribeKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_key::ser_describe_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_key_input::ser_describe_key_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -432,6 +433,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_key::DescribeKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_key::DescribeKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/disable_key.rs`

```diff
--- reference/src/operation/disable_key.rs
+++ generated/src/operation/disable_key.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DisableKey", "KMS"));
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
                 crate::operation::disable_key::DisableKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::disable_key::DisableKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::disable_key::DisableKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,15 +251,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DisableKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_disable_key::ser_disable_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_disable_key_input::ser_disable_key_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -287,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -448,6 +449,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::disable_key::DisableKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::disable_key::DisableKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/disable_key_rotation.rs`

```diff
--- reference/src/operation/disable_key_rotation.rs
+++ generated/src/operation/disable_key_rotation.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DisableKeyRotation", "KMS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DisableKeyRotation")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DisableKeyRotationTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DisableKeyRotationEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::disable_key_rotation::DisableKeyRotationError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::disable_key_rotation::DisableKeyRotationError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::disable_key_rotation::DisableKeyRotationError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DisableKeyRotation")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DisableKeyRotationTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DisableKeyRotationEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::disable_key_rotation::DisableKeyRotationError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::disable_key_rotation::DisableKeyRotationError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::disable_key_rotation::DisableKeyRotationError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,16 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DisableKeyRotation",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_disable_key_rotation::ser_disable_key_rotation_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_disable_key_rotation_input::ser_disable_key_rotation_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -290,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -471,6 +459,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::disable_key_rotation::DisableKeyRotationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::disable_key_rotation::DisableKeyRotationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/disconnect_custom_key_store.rs`

```diff
--- reference/src/operation/disconnect_custom_key_store.rs
+++ generated/src/operation/disconnect_custom_key_store.rs
@@ -107,9 +107,9 @@
             "KMS",
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
                 crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError,
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
@@ -250,16 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.DisconnectCustomKeyStore",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_disconnect_custom_key_store::ser_disconnect_custom_key_store_input(&input)?,
+            crate::protocol_serde::shape_disconnect_custom_key_store_input::ser_disconnect_custom_key_store_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -294,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -440,6 +443,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/enable_key.rs`

```diff
--- reference/src/operation/enable_key.rs
+++ generated/src/operation/enable_key.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("EnableKey", "KMS"));
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
                 crate::operation::enable_key::EnableKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::enable_key::EnableKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::enable_key::EnableKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,15 +251,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.EnableKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_enable_key::ser_enable_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_enable_key_input::ser_enable_key_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -287,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -458,6 +459,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::enable_key::EnableKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::enable_key::EnableKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/enable_key_rotation.rs`

```diff
--- reference/src/operation/enable_key_rotation.rs
+++ generated/src/operation/enable_key_rotation.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("EnableKeyRotation", "KMS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("EnableKeyRotation")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                EnableKeyRotationTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                EnableKeyRotationEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::enable_key_rotation::EnableKeyRotationError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::enable_key_rotation::EnableKeyRotationError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::enable_key_rotation::EnableKeyRotationError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("EnableKeyRotation")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(EnableKeyRotationTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(EnableKeyRotationEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::enable_key_rotation::EnableKeyRotationError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::enable_key_rotation::EnableKeyRotationError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::enable_key_rotation::EnableKeyRotationError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.EnableKeyRotation",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_enable_key_rotation::ser_enable_key_rotation_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_enable_key_rotation_input::ser_enable_key_rotation_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +459,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::enable_key_rotation::EnableKeyRotationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::enable_key_rotation::EnableKeyRotationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/encrypt.rs`

```diff
--- reference/src/operation/encrypt.rs
+++ generated/src/operation/encrypt.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Encrypt", "KMS"));
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
                 crate::operation::encrypt::EncryptError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::encrypt::EncryptError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::encrypt::EncryptError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -241,15 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.Encrypt",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_encrypt::ser_encrypt_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_encrypt_input::ser_encrypt_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -283,8 +284,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -492,6 +493,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::encrypt::EncryptError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::encrypt::EncryptError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/generate_data_key.rs`

```diff
--- reference/src/operation/generate_data_key.rs
+++ generated/src/operation/generate_data_key.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GenerateDataKey", "KMS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::generate_data_key::GenerateDataKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::generate_data_key::GenerateDataKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::generate_data_key::GenerateDataKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -248,15 +254,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GenerateDataKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_data_key::ser_generate_data_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_data_key_input::ser_generate_data_key_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -290,8 +293,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -499,6 +502,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::generate_data_key::GenerateDataKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::generate_data_key::GenerateDataKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/generate_data_key_pair.rs`

```diff
--- reference/src/operation/generate_data_key_pair.rs
+++ generated/src/operation/generate_data_key_pair.rs
@@ -108,9 +108,9 @@
             "KMS",
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
@@ -126,25 +126,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GenerateDataKeyPair")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GenerateDataKeyPairTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GenerateDataKeyPairEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::generate_data_key_pair::GenerateDataKeyPairError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::generate_data_key_pair::GenerateDataKeyPairError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::generate_data_key_pair::GenerateDataKeyPairError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GenerateDataKeyPair")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GenerateDataKeyPairTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GenerateDataKeyPairEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::generate_data_key_pair::GenerateDataKeyPairError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::generate_data_key_pair::GenerateDataKeyPairError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::generate_data_key_pair::GenerateDataKeyPairError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -251,17 +243,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GenerateDataKeyPair",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_data_key_pair::ser_generate_data_key_pair_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_generate_data_key_pair_input::ser_generate_data_key_pair_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -295,8 +282,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -514,6 +501,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::generate_data_key_pair::GenerateDataKeyPairError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::generate_data_key_pair::GenerateDataKeyPairError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/generate_data_key_pair_without_plaintext.rs`

```diff
--- reference/src/operation/generate_data_key_pair_without_plaintext.rs
+++ generated/src/operation/generate_data_key_pair_without_plaintext.rs
@@ -113,9 +113,9 @@
             "KMS",
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
                 crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError,
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
@@ -260,16 +268,13 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GenerateDataKeyPairWithoutPlaintext",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_generate_data_key_pair_without_plaintext::ser_generate_data_key_pair_without_plaintext_input(&input)?,
+            crate::protocol_serde::shape_generate_data_key_pair_without_plaintext_input::ser_generate_data_key_pair_without_plaintext_op_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -304,8 +309,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -523,6 +528,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/generate_data_key_without_plaintext.rs`

```diff
--- reference/src/operation/generate_data_key_without_plaintext.rs
+++ generated/src/operation/generate_data_key_without_plaintext.rs
@@ -113,9 +113,9 @@
             "KMS",
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
                 crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError,
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
@@ -258,16 +266,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GenerateDataKeyWithoutPlaintext",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_generate_data_key_without_plaintext::ser_generate_data_key_without_plaintext_input(&input)?,
+            crate::protocol_serde::shape_generate_data_key_without_plaintext_input::ser_generate_data_key_without_plaintext_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -302,8 +305,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -511,6 +514,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/generate_mac.rs`

```diff
--- reference/src/operation/generate_mac.rs
+++ generated/src/operation/generate_mac.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GenerateMac", "KMS"));
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
                 crate::operation::generate_mac::GenerateMacError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::generate_mac::GenerateMacError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::generate_mac::GenerateMacError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GenerateMac",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_mac::ser_generate_mac_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_mac_input::ser_generate_mac_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -488,6 +489,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::generate_mac::GenerateMacError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::generate_mac::GenerateMacError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/generate_random.rs`

```diff
--- reference/src/operation/generate_random.rs
+++ generated/src/operation/generate_random.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GenerateRandom", "KMS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::generate_random::GenerateRandomError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::generate_random::GenerateRandomError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::generate_random::GenerateRandomError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -248,15 +254,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GenerateRandom",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_random::ser_generate_random_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_generate_random_input::ser_generate_random_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -290,8 +291,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -456,6 +457,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::generate_random::GenerateRandomError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::generate_random::GenerateRandomError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_key_last_usage.rs`

```diff
--- reference/src/operation/get_key_last_usage.rs
+++ generated/src/operation/get_key_last_usage.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetKeyLastUsage", "KMS"));
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
                 crate::operation::get_key_last_usage::GetKeyLastUsageError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_key_last_usage::GetKeyLastUsageError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_key_last_usage::GetKeyLastUsageError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GetKeyLastUsage",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_key_last_usage::ser_get_key_last_usage_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_key_last_usage_input::ser_get_key_last_usage_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +292,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -432,6 +435,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_key_last_usage::GetKeyLastUsageError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_key_last_usage::GetKeyLastUsageError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_key_policy.rs`

```diff
--- reference/src/operation/get_key_policy.rs
+++ generated/src/operation/get_key_policy.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetKeyPolicy", "KMS"));
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
                 crate::operation::get_key_policy::GetKeyPolicyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_key_policy::GetKeyPolicyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_key_policy::GetKeyPolicyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GetKeyPolicy",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_key_policy::ser_get_key_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_key_policy_input::ser_get_key_policy_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_key_policy::GetKeyPolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_key_policy::GetKeyPolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_key_rotation_status.rs`

```diff
--- reference/src/operation/get_key_rotation_status.rs
+++ generated/src/operation/get_key_rotation_status.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetKeyRotationStatus")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetKeyRotationStatusTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetKeyRotationStatusEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetKeyRotationStatus")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetKeyRotationStatusTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetKeyRotationStatusEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_key_rotation_status::GetKeyRotationStatusError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_key_rotation_status::GetKeyRotationStatusError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_key_rotation_status::GetKeyRotationStatusError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,17 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GetKeyRotationStatus",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_key_rotation_status::ser_get_key_rotation_status_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_get_key_rotation_status_input::ser_get_key_rotation_status_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +281,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -465,6 +452,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_key_rotation_status::GetKeyRotationStatusError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_key_rotation_status::GetKeyRotationStatusError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_parameters_for_import.rs`

```diff
--- reference/src/operation/get_parameters_for_import.rs
+++ generated/src/operation/get_parameters_for_import.rs
@@ -108,9 +108,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetParametersForImport")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetParametersForImportTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetParametersForImportEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_parameters_for_import::GetParametersForImportError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_parameters_for_import::GetParametersForImportError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_parameters_for_import::GetParametersForImportError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetParametersForImport")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetParametersForImportTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetParametersForImportEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::get_parameters_for_import::GetParametersForImportError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::get_parameters_for_import::GetParametersForImportError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::get_parameters_for_import::GetParametersForImportError,
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
@@ -251,16 +260,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GetParametersForImport",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_parameters_for_import::ser_get_parameters_for_import_input(&input)?,
+            crate::protocol_serde::shape_get_parameters_for_import_input::ser_get_parameters_for_import_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -295,8 +299,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -466,6 +470,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_parameters_for_import::GetParametersForImportError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_parameters_for_import::GetParametersForImportError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_public_key.rs`

```diff
--- reference/src/operation/get_public_key.rs
+++ generated/src/operation/get_public_key.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetPublicKey", "KMS"));
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
                 crate::operation::get_public_key::GetPublicKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_public_key::GetPublicKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_public_key::GetPublicKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.GetPublicKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_public_key::ser_get_public_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_public_key_input::ser_get_public_key_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -508,6 +509,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_public_key::GetPublicKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_public_key::GetPublicKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/import_key_material.rs`

```diff
--- reference/src/operation/import_key_material.rs
+++ generated/src/operation/import_key_material.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ImportKeyMaterial", "KMS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ImportKeyMaterial")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ImportKeyMaterialTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ImportKeyMaterialEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::import_key_material::ImportKeyMaterialError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::import_key_material::ImportKeyMaterialError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::import_key_material::ImportKeyMaterialError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ImportKeyMaterial")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ImportKeyMaterialTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ImportKeyMaterialEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::import_key_material::ImportKeyMaterialError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::import_key_material::ImportKeyMaterialError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::import_key_material::ImportKeyMaterialError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,15 +249,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ImportKeyMaterial",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_import_key_material::ser_import_key_material_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_import_key_material_input::ser_import_key_material_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -511,6 +500,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::import_key_material::ImportKeyMaterialError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::import_key_material::ImportKeyMaterialError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_aliases.rs`

```diff
--- reference/src/operation/list_aliases.rs
+++ generated/src/operation/list_aliases.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAliases", "KMS"));
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
                 crate::operation::list_aliases::ListAliasesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_aliases::ListAliasesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_aliases::ListAliasesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListAliases",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_aliases::ser_list_aliases_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_aliases_input::ser_list_aliases_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -447,6 +448,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_aliases::ListAliasesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_aliases::ListAliasesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_grants.rs`

```diff
--- reference/src/operation/list_grants.rs
+++ generated/src/operation/list_grants.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListGrants", "KMS"));
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
                 crate::operation::list_grants::ListGrantsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_grants::ListGrantsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_grants::ListGrantsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -265,15 +271,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListGrants",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_grants::ser_list_grants_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_grants_input::ser_list_grants_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -307,8 +308,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -488,6 +489,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_grants::ListGrantsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_grants::ListGrantsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_key_policies.rs`

```diff
--- reference/src/operation/list_key_policies.rs
+++ generated/src/operation/list_key_policies.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListKeyPolicies", "KMS"));
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
                 crate::operation::list_key_policies::ListKeyPoliciesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_key_policies::ListKeyPoliciesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_key_policies::ListKeyPoliciesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListKeyPolicies",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_key_policies::ser_list_key_policies_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_key_policies_input::ser_list_key_policies_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +458,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_key_policies::ListKeyPoliciesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_key_policies::ListKeyPoliciesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_key_rotations.rs`

```diff
--- reference/src/operation/list_key_rotations.rs
+++ generated/src/operation/list_key_rotations.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListKeyRotations", "KMS"));
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
                 crate::operation::list_key_rotations::ListKeyRotationsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_key_rotations::ListKeyRotationsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_key_rotations::ListKeyRotationsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListKeyRotations",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_key_rotations::ser_list_key_rotations_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_key_rotations_input::ser_list_key_rotations_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -465,6 +468,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_key_rotations::ListKeyRotationsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_key_rotations::ListKeyRotationsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_keys.rs`

```diff
--- reference/src/operation/list_keys.rs
+++ generated/src/operation/list_keys.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListKeys", "KMS"));
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
                 crate::operation::list_keys::ListKeysError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_keys::ListKeysError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_keys::ListKeysError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -241,15 +247,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListKeys",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_keys::ser_list_keys_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_keys_input::ser_list_keys_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -283,8 +284,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -416,6 +417,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_keys::ListKeysError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_keys::ListKeysError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_resource_tags.rs`

```diff
--- reference/src/operation/list_resource_tags.rs
+++ generated/src/operation/list_resource_tags.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListResourceTags", "KMS"));
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
                 crate::operation::list_resource_tags::ListResourceTagsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_resource_tags::ListResourceTagsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_resource_tags::ListResourceTagsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListResourceTags",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_resource_tags::ser_list_resource_tags_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_resource_tags_input::ser_list_resource_tags_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -437,6 +440,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_resource_tags::ListResourceTagsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_resource_tags::ListResourceTagsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_retirable_grants.rs`

```diff
--- reference/src/operation/list_retirable_grants.rs
+++ generated/src/operation/list_retirable_grants.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListRetirableGrants")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListRetirableGrantsTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListRetirableGrantsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_retirable_grants::ListRetirableGrantsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_retirable_grants::ListRetirableGrantsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_retirable_grants::ListRetirableGrantsError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListRetirableGrants")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListRetirableGrantsTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListRetirableGrantsEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::list_retirable_grants::ListRetirableGrantsError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::list_retirable_grants::ListRetirableGrantsError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_retirable_grants::ListRetirableGrantsError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -260,17 +252,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ListRetirableGrants",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_retirable_grants::ser_list_retirable_grants_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_list_retirable_grants_input::ser_list_retirable_grants_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -304,8 +291,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -457,6 +444,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_retirable_grants::ListRetirableGrantsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_retirable_grants::ListRetirableGrantsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_key_policy/_put_key_policy_input.rs`

```diff
--- reference/src/operation/put_key_policy/_put_key_policy_input.rs
+++ generated/src/operation/put_key_policy/_put_key_policy_input.rs
@@ -288,7 +288,7 @@
             key_id: self.key_id,
             policy_name: self.policy_name,
             policy: self.policy,
-            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
+            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/put_key_policy.rs`

```diff
--- reference/src/operation/put_key_policy.rs
+++ generated/src/operation/put_key_policy.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("PutKeyPolicy", "KMS"));
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
                 crate::operation::put_key_policy::PutKeyPolicyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_key_policy::PutKeyPolicyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::put_key_policy::PutKeyPolicyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,15 +263,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.PutKeyPolicy",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_key_policy::ser_put_key_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_key_policy_input::ser_put_key_policy_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +300,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -490,6 +491,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_key_policy::PutKeyPolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_key_policy::PutKeyPolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/re_encrypt.rs`

```diff
--- reference/src/operation/re_encrypt.rs
+++ generated/src/operation/re_encrypt.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ReEncrypt", "KMS"));
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
                 crate::operation::re_encrypt::ReEncryptError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::re_encrypt::ReEncryptError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::re_encrypt::ReEncryptError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +256,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ReEncrypt",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_re_encrypt::ser_re_encrypt_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_re_encrypt_input::ser_re_encrypt_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,8 +293,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -522,6 +523,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::re_encrypt::ReEncryptError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::re_encrypt::ReEncryptError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/replicate_key/_replicate_key_input.rs`

```diff
--- reference/src/operation/replicate_key/_replicate_key_input.rs
+++ generated/src/operation/replicate_key/_replicate_key_input.rs
@@ -383,7 +383,7 @@
             key_id: self.key_id,
             replica_region: self.replica_region,
             policy: self.policy,
-            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
+            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check.unwrap_or_default(),
             description: self.description,
             tags: self.tags,
         })
```

### `src/operation/replicate_key.rs`

```diff
--- reference/src/operation/replicate_key.rs
+++ generated/src/operation/replicate_key.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ReplicateKey", "KMS"));
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
                 crate::operation::replicate_key::ReplicateKeyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::replicate_key::ReplicateKeyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::replicate_key::ReplicateKeyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -262,15 +268,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ReplicateKey",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_replicate_key::ser_replicate_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_replicate_key_input::ser_replicate_key_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -304,8 +305,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -515,6 +516,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::replicate_key::ReplicateKeyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::replicate_key::ReplicateKeyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/retire_grant.rs`

```diff
--- reference/src/operation/retire_grant.rs
+++ generated/src/operation/retire_grant.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("RetireGrant", "KMS"));
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
                 crate::operation::retire_grant::RetireGrantError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::retire_grant::RetireGrantError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::retire_grant::RetireGrantError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,15 +263,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.RetireGrant",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_retire_grant::ser_retire_grant_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_retire_grant_input::ser_retire_grant_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +300,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -490,6 +491,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::retire_grant::RetireGrantError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::retire_grant::RetireGrantError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/revoke_grant.rs`

```diff
--- reference/src/operation/revoke_grant.rs
+++ generated/src/operation/revoke_grant.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("RevokeGrant", "KMS"));
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
                 crate::operation::revoke_grant::RevokeGrantError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::revoke_grant::RevokeGrantError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::revoke_grant::RevokeGrantError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.RevokeGrant",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_revoke_grant::ser_revoke_grant_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_revoke_grant_input::ser_revoke_grant_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -475,6 +476,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::revoke_grant::RevokeGrantError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::revoke_grant::RevokeGrantError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/rotate_key_on_demand.rs`

```diff
--- reference/src/operation/rotate_key_on_demand.rs
+++ generated/src/operation/rotate_key_on_demand.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("RotateKeyOnDemand", "KMS"));
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RotateKeyOnDemand")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                RotateKeyOnDemandTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                RotateKeyOnDemandEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::rotate_key_on_demand::RotateKeyOnDemandError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::rotate_key_on_demand::RotateKeyOnDemandError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::rotate_key_on_demand::RotateKeyOnDemandError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RotateKeyOnDemand")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(RotateKeyOnDemandTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(RotateKeyOnDemandEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::rotate_key_on_demand::RotateKeyOnDemandError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::rotate_key_on_demand::RotateKeyOnDemandError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::rotate_key_on_demand::RotateKeyOnDemandError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,16 +239,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.RotateKeyOnDemand",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_rotate_key_on_demand::ser_rotate_key_on_demand_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_rotate_key_on_demand_input::ser_rotate_key_on_demand_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -290,8 +278,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -491,6 +479,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::rotate_key_on_demand::RotateKeyOnDemandError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::rotate_key_on_demand::RotateKeyOnDemandError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/schedule_key_deletion.rs`

```diff
--- reference/src/operation/schedule_key_deletion.rs
+++ generated/src/operation/schedule_key_deletion.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ScheduleKeyDeletion")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ScheduleKeyDeletionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ScheduleKeyDeletionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::schedule_key_deletion::ScheduleKeyDeletionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::schedule_key_deletion::ScheduleKeyDeletionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::schedule_key_deletion::ScheduleKeyDeletionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ScheduleKeyDeletion")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ScheduleKeyDeletionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ScheduleKeyDeletionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::schedule_key_deletion::ScheduleKeyDeletionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::schedule_key_deletion::ScheduleKeyDeletionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::schedule_key_deletion::ScheduleKeyDeletionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,17 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.ScheduleKeyDeletion",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_schedule_key_deletion::ser_schedule_key_deletion_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_schedule_key_deletion_input::ser_schedule_key_deletion_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +281,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +442,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::schedule_key_deletion::ScheduleKeyDeletionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::schedule_key_deletion::ScheduleKeyDeletionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/sign.rs`

```diff
--- reference/src/operation/sign.rs
+++ generated/src/operation/sign.rs
@@ -95,9 +95,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Sign", "KMS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -129,9 +129,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::sign::SignError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::sign::SignError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::sign::SignError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -236,15 +242,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.Sign",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_sign::ser_sign_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_sign_input::ser_sign_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -275,8 +276,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -484,6 +485,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::sign::SignError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::sign::SignError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/tag_resource.rs`

```diff
--- reference/src/operation/tag_resource.rs
+++ generated/src/operation/tag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("TagResource", "KMS"));
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
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.TagResource",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource::ser_tag_resource_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +461,11 @@
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

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UntagResource", "KMS"));
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
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.UntagResource",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_resource::ser_untag_resource_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_resource_input::ser_untag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,8 +290,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +451,11 @@
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

### `src/operation/update_alias.rs`

```diff
--- reference/src/operation/update_alias.rs
+++ generated/src/operation/update_alias.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UpdateAlias", "KMS"));
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
                 crate::operation::update_alias::UpdateAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_alias::UpdateAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_alias::UpdateAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.UpdateAlias",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_alias::ser_update_alias_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_alias_input::ser_update_alias_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,8 +295,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_alias::UpdateAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_alias::UpdateAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_custom_key_store.rs`

```diff
--- reference/src/operation/update_custom_key_store.rs
+++ generated/src/operation/update_custom_key_store.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateCustomKeyStore")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateCustomKeyStoreTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateCustomKeyStoreEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::update_custom_key_store::UpdateCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::update_custom_key_store::UpdateCustomKeyStoreError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_custom_key_store::UpdateCustomKeyStoreError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateCustomKeyStore")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateCustomKeyStoreTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateCustomKeyStoreEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::update_custom_key_store::UpdateCustomKeyStoreError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::update_custom_key_store::UpdateCustomKeyStoreError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_custom_key_store::UpdateCustomKeyStoreError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -280,17 +272,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.UpdateCustomKeyStore",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_custom_key_store::ser_update_custom_key_store_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_update_custom_key_store_input::ser_update_custom_key_store_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -324,8 +311,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -628,6 +615,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_custom_key_store::UpdateCustomKeyStoreError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_custom_key_store::UpdateCustomKeyStoreError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_key_description.rs`

```diff
--- reference/src/operation/update_key_description.rs
+++ generated/src/operation/update_key_description.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateKeyDescription")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateKeyDescriptionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateKeyDescriptionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::update_key_description::UpdateKeyDescriptionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::update_key_description::UpdateKeyDescriptionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_key_description::UpdateKeyDescriptionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateKeyDescription")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateKeyDescriptionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateKeyDescriptionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::update_key_description::UpdateKeyDescriptionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::update_key_description::UpdateKeyDescriptionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_key_description::UpdateKeyDescriptionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,17 +247,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.UpdateKeyDescription",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_key_description::ser_update_key_description_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_update_key_description_input::ser_update_key_description_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +286,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +447,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_key_description::UpdateKeyDescriptionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_key_description::UpdateKeyDescriptionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_primary_region.rs`

```diff
--- reference/src/operation/update_primary_region.rs
+++ generated/src/operation/update_primary_region.rs
@@ -107,9 +107,9 @@
             "KMS",
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
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdatePrimaryRegion")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdatePrimaryRegionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdatePrimaryRegionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::update_primary_region::UpdatePrimaryRegionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::update_primary_region::UpdatePrimaryRegionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_primary_region::UpdatePrimaryRegionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdatePrimaryRegion")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdatePrimaryRegionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdatePrimaryRegionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::update_primary_region::UpdatePrimaryRegionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::update_primary_region::UpdatePrimaryRegionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_primary_region::UpdatePrimaryRegionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,17 +247,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.UpdatePrimaryRegion",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_primary_region::ser_update_primary_region_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_update_primary_region_input::ser_update_primary_region_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,8 +286,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +457,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_primary_region::UpdatePrimaryRegionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_primary_region::UpdatePrimaryRegionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/verify.rs`

```diff
--- reference/src/operation/verify.rs
+++ generated/src/operation/verify.rs
@@ -96,9 +96,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Verify", "KMS"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -130,9 +130,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::verify::VerifyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::verify::VerifyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::verify::VerifyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -237,15 +243,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.Verify",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_verify::ser_verify_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_verify_input::ser_verify_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -276,8 +277,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -495,6 +496,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::verify::VerifyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::verify::VerifyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/verify_mac.rs`

```diff
--- reference/src/operation/verify_mac.rs
+++ generated/src/operation/verify_mac.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("VerifyMac", "KMS"));
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
                 crate::operation::verify_mac::VerifyMacError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::verify_mac::VerifyMacError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::verify_mac::VerifyMacError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,15 +251,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.1");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "TrentService.VerifyMac",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_verify_mac::ser_verify_mac_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_verify_mac_input::ser_verify_mac_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -287,8 +288,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -496,6 +497,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::verify_mac::VerifyMacError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::verify_mac::VerifyMacError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/types/error/_kms_internal_exception.rs`

```diff
--- reference/src/types/error/_kms_internal_exception.rs
+++ generated/src/types/error/_kms_internal_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsInternalException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsInternalException [KMSInternalException]")?;
+        ::std::write!(f, "KmsInternalException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_invalid_mac_exception.rs`

```diff
--- reference/src/types/error/_kms_invalid_mac_exception.rs
+++ generated/src/types/error/_kms_invalid_mac_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsInvalidMacException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsInvalidMacException [KMSInvalidMacException]")?;
+        ::std::write!(f, "KmsInvalidMacException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_invalid_signature_exception.rs`

```diff
--- reference/src/types/error/_kms_invalid_signature_exception.rs
+++ generated/src/types/error/_kms_invalid_signature_exception.rs
@@ -16,7 +16,7 @@
 }
 impl ::std::fmt::Display for KmsInvalidSignatureException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsInvalidSignatureException [KMSInvalidSignatureException]")?;
+        ::std::write!(f, "KmsInvalidSignatureException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_invalid_state_exception.rs`

```diff
--- reference/src/types/error/_kms_invalid_state_exception.rs
+++ generated/src/types/error/_kms_invalid_state_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for KmsInvalidStateException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsInvalidStateException [KMSInvalidStateException]")?;
+        ::std::write!(f, "KmsInvalidStateException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -75,7 +75,7 @@

 pub use crate::types::error::_key_unavailable_exception::KeyUnavailableExceptionBuilder;

-pub use crate::types::error::_custom_key_store_has_cmks_exception::CustomKeyStoreHasCmKsExceptionBuilder;
+pub use crate::types::error::_custom_key_store_has_cm_ks_exception::CustomKeyStoreHasCmKsExceptionBuilder;

 pub use crate::types::error::_invalid_marker_exception::InvalidMarkerExceptionBuilder;

```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -75,7 +75,7 @@

 pub use crate::types::error::_key_unavailable_exception::KeyUnavailableException;

-pub use crate::types::error::_custom_key_store_has_cmks_exception::CustomKeyStoreHasCmKsException;
+pub use crate::types::error::_custom_key_store_has_cm_ks_exception::CustomKeyStoreHasCmKsException;

 pub use crate::types::error::_invalid_marker_exception::InvalidMarkerException;

@@ -109,7 +109,7 @@

 mod _conflict_exception;

-mod _custom_key_store_has_cmks_exception;
+mod _custom_key_store_has_cm_ks_exception;

 mod _custom_key_store_invalid_state_exception;

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
- `src/json_errors.rs`
- `src/protocol_serde/shape_alias_list.rs`
- `src/protocol_serde/shape_alias_list_entry.rs`
- `src/protocol_serde/shape_already_exists_exception.rs`
- `src/protocol_serde/shape_cancel_key_deletion.rs`
- `src/protocol_serde/shape_cancel_key_deletion_input.rs`
- `src/protocol_serde/shape_cloud_hsm_cluster_in_use_exception.rs`
- `src/protocol_serde/shape_cloud_hsm_cluster_invalid_configuration_exception.rs`
- `src/protocol_serde/shape_cloud_hsm_cluster_not_active_exception.rs`
- `src/protocol_serde/shape_cloud_hsm_cluster_not_found_exception.rs`
- `src/protocol_serde/shape_cloud_hsm_cluster_not_related_exception.rs`
- `src/protocol_serde/shape_conflict_exception.rs`
- `src/protocol_serde/shape_connect_custom_key_store.rs`
- `src/protocol_serde/shape_connect_custom_key_store_input.rs`
- `src/protocol_serde/shape_create_alias.rs`
- `src/protocol_serde/shape_create_alias_input.rs`
- `src/protocol_serde/shape_create_custom_key_store.rs`
- `src/protocol_serde/shape_create_custom_key_store_input.rs`
- `src/protocol_serde/shape_create_grant.rs`
- `src/protocol_serde/shape_create_grant_input.rs`
- `src/protocol_serde/shape_create_key.rs`
- `src/protocol_serde/shape_create_key_input.rs`
- `src/protocol_serde/shape_custom_key_store_has_cmks_exception.rs`
- `src/protocol_serde/shape_custom_key_store_invalid_state_exception.rs`
- `src/protocol_serde/shape_custom_key_store_name_in_use_exception.rs`
- `src/protocol_serde/shape_custom_key_store_not_found_exception.rs`
- `src/protocol_serde/shape_custom_key_stores_list.rs`
- `src/protocol_serde/shape_custom_key_stores_list_entry.rs`
- `src/protocol_serde/shape_decrypt.rs`
- `src/protocol_serde/shape_decrypt_input.rs`
- `src/protocol_serde/shape_delete_alias.rs`
- `src/protocol_serde/shape_delete_alias_input.rs`
- `src/protocol_serde/shape_delete_custom_key_store.rs`
- `src/protocol_serde/shape_delete_custom_key_store_input.rs`
- `src/protocol_serde/shape_delete_imported_key_material.rs`
- `src/protocol_serde/shape_delete_imported_key_material_input.rs`
- `src/protocol_serde/shape_dependency_timeout_exception.rs`
- `src/protocol_serde/shape_derive_shared_secret.rs`
- `src/protocol_serde/shape_derive_shared_secret_input.rs`
- `src/protocol_serde/shape_describe_custom_key_stores.rs`
- `src/protocol_serde/shape_describe_custom_key_stores_input.rs`
- `src/protocol_serde/shape_describe_key.rs`
- `src/protocol_serde/shape_describe_key_input.rs`
- `src/protocol_serde/shape_disable_key.rs`
- `src/protocol_serde/shape_disable_key_input.rs`
- `src/protocol_serde/shape_disable_key_rotation.rs`
- `src/protocol_serde/shape_disable_key_rotation_input.rs`
- `src/protocol_serde/shape_disabled_exception.rs`
- `src/protocol_serde/shape_disconnect_custom_key_store.rs`
- `src/protocol_serde/shape_disconnect_custom_key_store_input.rs`
- `src/protocol_serde/shape_dry_run_operation_exception.rs`
- `src/protocol_serde/shape_enable_key.rs`
- `src/protocol_serde/shape_enable_key_input.rs`
- `src/protocol_serde/shape_enable_key_rotation.rs`
- `src/protocol_serde/shape_enable_key_rotation_input.rs`
- `src/protocol_serde/shape_encrypt.rs`
- `src/protocol_serde/shape_encrypt_input.rs`
- `src/protocol_serde/shape_encryption_algorithm_spec_list.rs`
- `src/protocol_serde/shape_encryption_context_type.rs`
- `src/protocol_serde/shape_expired_import_token_exception.rs`
- `src/protocol_serde/shape_generate_data_key.rs`
- `src/protocol_serde/shape_generate_data_key_input.rs`
- `src/protocol_serde/shape_generate_data_key_pair.rs`
- `src/protocol_serde/shape_generate_data_key_pair_input.rs`
- `src/protocol_serde/shape_generate_data_key_pair_without_plaintext.rs`
- `src/protocol_serde/shape_generate_data_key_pair_without_plaintext_input.rs`
- `src/protocol_serde/shape_generate_data_key_without_plaintext.rs`
- `src/protocol_serde/shape_generate_data_key_without_plaintext_input.rs`
- `src/protocol_serde/shape_generate_mac.rs`
- `src/protocol_serde/shape_generate_mac_input.rs`
- `src/protocol_serde/shape_generate_random.rs`
- `src/protocol_serde/shape_generate_random_input.rs`
- `src/protocol_serde/shape_get_key_last_usage.rs`
- `src/protocol_serde/shape_get_key_last_usage_input.rs`
- `src/protocol_serde/shape_get_key_policy.rs`
- `src/protocol_serde/shape_get_key_policy_input.rs`
- `src/protocol_serde/shape_get_key_rotation_status.rs`
- `src/protocol_serde/shape_get_key_rotation_status_input.rs`
- `src/protocol_serde/shape_get_parameters_for_import.rs`
- `src/protocol_serde/shape_get_parameters_for_import_input.rs`
- `src/protocol_serde/shape_get_public_key.rs`
- `src/protocol_serde/shape_get_public_key_input.rs`
- `src/protocol_serde/shape_grant_constraints.rs`
- `src/protocol_serde/shape_grant_list.rs`
- `src/protocol_serde/shape_grant_list_entry.rs`
- `src/protocol_serde/shape_grant_operation_list.rs`
- `src/protocol_serde/shape_import_key_material.rs`
- `src/protocol_serde/shape_import_key_material_input.rs`
- `src/protocol_serde/shape_incorrect_key_exception.rs`
- `src/protocol_serde/shape_incorrect_key_material_exception.rs`
- `src/protocol_serde/shape_incorrect_trust_anchor_exception.rs`
- `src/protocol_serde/shape_invalid_alias_name_exception.rs`
- `src/protocol_serde/shape_invalid_arn_exception.rs`
- `src/protocol_serde/shape_invalid_ciphertext_exception.rs`
- `src/protocol_serde/shape_invalid_grant_id_exception.rs`
- `src/protocol_serde/shape_invalid_grant_token_exception.rs`
- `src/protocol_serde/shape_invalid_import_token_exception.rs`
- `src/protocol_serde/shape_invalid_key_usage_exception.rs`
- `src/protocol_serde/shape_invalid_marker_exception.rs`
- `src/protocol_serde/shape_key_agreement_algorithm_spec_list.rs`
- `src/protocol_serde/shape_key_last_usage_data.rs`
- `src/protocol_serde/shape_key_list.rs`
- `src/protocol_serde/shape_key_list_entry.rs`
- `src/protocol_serde/shape_key_metadata.rs`
- `src/protocol_serde/shape_key_unavailable_exception.rs`
- `src/protocol_serde/shape_kms_internal_exception.rs`
- `src/protocol_serde/shape_kms_invalid_mac_exception.rs`
- `src/protocol_serde/shape_kms_invalid_signature_exception.rs`
- `src/protocol_serde/shape_kms_invalid_state_exception.rs`
- `src/protocol_serde/shape_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_list_aliases.rs`
- `src/protocol_serde/shape_list_aliases_input.rs`
- `src/protocol_serde/shape_list_grants.rs`
- `src/protocol_serde/shape_list_grants_input.rs`
- `src/protocol_serde/shape_list_key_policies.rs`
- `src/protocol_serde/shape_list_key_policies_input.rs`
- `src/protocol_serde/shape_list_key_rotations.rs`
- `src/protocol_serde/shape_list_key_rotations_input.rs`
- `src/protocol_serde/shape_list_keys.rs`
- `src/protocol_serde/shape_list_keys_input.rs`
- `src/protocol_serde/shape_list_resource_tags.rs`
- `src/protocol_serde/shape_list_resource_tags_input.rs`
- `src/protocol_serde/shape_list_retirable_grants.rs`
- `src/protocol_serde/shape_list_retirable_grants_input.rs`
- `src/protocol_serde/shape_mac_algorithm_spec_list.rs`
- `src/protocol_serde/shape_malformed_policy_document_exception.rs`
- `src/protocol_serde/shape_multi_region_configuration.rs`
- `src/protocol_serde/shape_multi_region_key.rs`
- `src/protocol_serde/shape_multi_region_key_list.rs`
- `src/protocol_serde/shape_not_found_exception.rs`
- `src/protocol_serde/shape_policy_name_list.rs`
- `src/protocol_serde/shape_put_key_policy.rs`
- `src/protocol_serde/shape_put_key_policy_input.rs`
- `src/protocol_serde/shape_re_encrypt.rs`
- `src/protocol_serde/shape_re_encrypt_input.rs`
- `src/protocol_serde/shape_recipient_info.rs`
- `src/protocol_serde/shape_replicate_key.rs`
- `src/protocol_serde/shape_replicate_key_input.rs`
- `src/protocol_serde/shape_retire_grant.rs`
- `src/protocol_serde/shape_retire_grant_input.rs`
- `src/protocol_serde/shape_revoke_grant.rs`
- `src/protocol_serde/shape_revoke_grant_input.rs`
- `src/protocol_serde/shape_rotate_key_on_demand.rs`
- `src/protocol_serde/shape_rotate_key_on_demand_input.rs`
- `src/protocol_serde/shape_rotations_list.rs`
- `src/protocol_serde/shape_rotations_list_entry.rs`
- `src/protocol_serde/shape_schedule_key_deletion.rs`
- `src/protocol_serde/shape_schedule_key_deletion_input.rs`
- `src/protocol_serde/shape_sign.rs`
- `src/protocol_serde/shape_sign_input.rs`
- `src/protocol_serde/shape_signing_algorithm_spec_list.rs`
- `src/protocol_serde/shape_tag.rs`
- `src/protocol_serde/shape_tag_exception.rs`
- `src/protocol_serde/shape_tag_list.rs`
- `src/protocol_serde/shape_tag_resource.rs`
- `src/protocol_serde/shape_tag_resource_input.rs`
- `src/protocol_serde/shape_unsupported_operation_exception.rs`
- `src/protocol_serde/shape_untag_resource.rs`
- `src/protocol_serde/shape_untag_resource_input.rs`
- `src/protocol_serde/shape_update_alias.rs`
- `src/protocol_serde/shape_update_alias_input.rs`
- `src/protocol_serde/shape_update_custom_key_store.rs`
- `src/protocol_serde/shape_update_custom_key_store_input.rs`
- `src/protocol_serde/shape_update_key_description.rs`
- `src/protocol_serde/shape_update_key_description_input.rs`
- `src/protocol_serde/shape_update_primary_region.rs`
- `src/protocol_serde/shape_update_primary_region_input.rs`
- `src/protocol_serde/shape_verify.rs`
- `src/protocol_serde/shape_verify_input.rs`
- `src/protocol_serde/shape_verify_mac.rs`
- `src/protocol_serde/shape_verify_mac_input.rs`
- `src/protocol_serde/shape_xks_key_already_in_use_exception.rs`
- `src/protocol_serde/shape_xks_key_configuration_type.rs`
- `src/protocol_serde/shape_xks_key_invalid_configuration_exception.rs`
- `src/protocol_serde/shape_xks_key_not_found_exception.rs`
- `src/protocol_serde/shape_xks_proxy_authentication_credential_type.rs`
- `src/protocol_serde/shape_xks_proxy_configuration_type.rs`
- `src/protocol_serde/shape_xks_proxy_incorrect_authentication_credential_exception.rs`
- `src/protocol_serde/shape_xks_proxy_invalid_configuration_exception.rs`
- `src/protocol_serde/shape_xks_proxy_invalid_response_exception.rs`
- `src/protocol_serde/shape_xks_proxy_uri_endpoint_in_use_exception.rs`
- `src/protocol_serde/shape_xks_proxy_uri_in_use_exception.rs`
- `src/protocol_serde/shape_xks_proxy_uri_unreachable_exception.rs`
- `src/protocol_serde/shape_xks_proxy_vpc_endpoint_service_in_use_exception.rs`
- `src/protocol_serde/shape_xks_proxy_vpc_endpoint_service_invalid_configuration_exception.rs`
- `src/protocol_serde/shape_xks_proxy_vpc_endpoint_service_not_found_exception.rs`
- `src/protocol_serde.rs`
- `src/serialization_settings.rs`
- `src/types/error/_custom_key_store_has_cmks_exception.rs`
- `tests/endpoint_tests.rs`
- `tests/integration.rs`
- `tests/retryable_errors.rs`
- `tests/sensitive-it.rs`
- `tests/traits.rs`

### Unexpected generated files

- `src/types/error/_custom_key_store_has_cm_ks_exception.rs`

### Rust token differences

- `src/client/cancel_key_deletion.rs`
- `src/client/connect_custom_key_store.rs`
- `src/client/create_alias.rs`
- `src/client/create_custom_key_store.rs`
- `src/client/create_grant.rs`
- `src/client/create_key.rs`
- `src/client/decrypt.rs`
- `src/client/delete_custom_key_store.rs`
- `src/client/delete_imported_key_material.rs`
- `src/client/derive_shared_secret.rs`
- `src/client/describe_key.rs`
- `src/client/disable_key.rs`
- `src/client/disable_key_rotation.rs`
- `src/client/disconnect_custom_key_store.rs`
- `src/client/enable_key.rs`
- `src/client/enable_key_rotation.rs`
- `src/client/encrypt.rs`
- `src/client/generate_data_key.rs`
- `src/client/generate_data_key_pair.rs`
- `src/client/generate_data_key_pair_without_plaintext.rs`
- `src/client/generate_data_key_without_plaintext.rs`
- `src/client/generate_mac.rs`
- `src/client/generate_random.rs`
- `src/client/get_key_last_usage.rs`
- `src/client/get_key_policy.rs`
- `src/client/get_key_rotation_status.rs`
- `src/client/get_parameters_for_import.rs`
- `src/client/get_public_key.rs`
- `src/client/import_key_material.rs`
- `src/client/list_aliases.rs`
- `src/client/list_grants.rs`
- `src/client/list_key_policies.rs`
- `src/client/list_key_rotations.rs`
- `src/client/list_resource_tags.rs`
- `src/client/list_retirable_grants.rs`
- `src/client/put_key_policy.rs`
- `src/client/re_encrypt.rs`
- `src/client/replicate_key.rs`
- `src/client/retire_grant.rs`
- `src/client/revoke_grant.rs`
- `src/client/rotate_key_on_demand.rs`
- `src/client/schedule_key_deletion.rs`
- `src/client/sign.rs`
- `src/client/tag_resource.rs`
- `src/client/untag_resource.rs`
- `src/client/update_alias.rs`
- `src/client/update_custom_key_store.rs`
- `src/client/update_key_description.rs`
- `src/client/update_primary_region.rs`
- `src/client/verify.rs`
- `src/client/verify_mac.rs`
- `src/config.rs`
- `src/operation/cancel_key_deletion.rs`
- `src/operation/connect_custom_key_store.rs`
- `src/operation/create_alias.rs`
- `src/operation/create_custom_key_store.rs`
- `src/operation/create_grant.rs`
- `src/operation/create_key/_create_key_input.rs`
- `src/operation/create_key.rs`
- `src/operation/decrypt.rs`
- `src/operation/delete_alias.rs`
- `src/operation/delete_custom_key_store.rs`
- `src/operation/delete_imported_key_material.rs`
- `src/operation/derive_shared_secret.rs`
- `src/operation/describe_custom_key_stores.rs`
- `src/operation/describe_key.rs`
- `src/operation/disable_key.rs`
- `src/operation/disable_key_rotation.rs`
- `src/operation/disconnect_custom_key_store.rs`
- `src/operation/enable_key.rs`
- `src/operation/enable_key_rotation.rs`
- `src/operation/encrypt.rs`
- `src/operation/generate_data_key.rs`
- `src/operation/generate_data_key_pair.rs`
- `src/operation/generate_data_key_pair_without_plaintext.rs`
- `src/operation/generate_data_key_without_plaintext.rs`
- `src/operation/generate_mac.rs`
- `src/operation/generate_random.rs`
- `src/operation/get_key_last_usage.rs`
- `src/operation/get_key_policy.rs`
- `src/operation/get_key_rotation_status.rs`
- `src/operation/get_parameters_for_import.rs`
- `src/operation/get_public_key.rs`
- `src/operation/import_key_material.rs`
- `src/operation/list_aliases.rs`
- `src/operation/list_grants.rs`
- `src/operation/list_key_policies.rs`
- `src/operation/list_key_rotations.rs`
- `src/operation/list_keys.rs`
- `src/operation/list_resource_tags.rs`
- `src/operation/list_retirable_grants.rs`
- `src/operation/put_key_policy/_put_key_policy_input.rs`
- `src/operation/put_key_policy.rs`
- `src/operation/re_encrypt.rs`
- `src/operation/replicate_key/_replicate_key_input.rs`
- `src/operation/replicate_key.rs`
- `src/operation/retire_grant.rs`
- `src/operation/revoke_grant.rs`
- `src/operation/rotate_key_on_demand.rs`
- `src/operation/schedule_key_deletion.rs`
- `src/operation/sign.rs`
- `src/operation/tag_resource.rs`
- `src/operation/untag_resource.rs`
- `src/operation/update_alias.rs`
- `src/operation/update_custom_key_store.rs`
- `src/operation/update_key_description.rs`
- `src/operation/update_primary_region.rs`
- `src/operation/verify.rs`
- `src/operation/verify_mac.rs`
- `src/types/error/_kms_internal_exception.rs`
- `src/types/error/_kms_invalid_mac_exception.rs`
- `src/types/error/_kms_invalid_signature_exception.rs`
- `src/types/error/_kms_invalid_state_exception.rs`
- `src/types/error/builders.rs`
- `src/types/error.rs`
