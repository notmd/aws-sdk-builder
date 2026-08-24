# AWS SDK Conformance Report: kms

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## kms
**Progress:** `591/591` files compared · `367` matched · `224` mismatches · `0` missing · `0` extra · `62.10%` match (100.00% means fully matched)

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
     pub fn connect_custom_key_store(&self) -> super::super::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder {
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
     pub fn create_alias(&self) -> super::super::operation::create_alias::builders::CreateAliasFluentBuilder {
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
     pub fn create_grant(&self) -> super::super::operation::create_grant::builders::CreateGrantFluentBuilder {
         super::super::operation::create_grant::builders::CreateGrantFluentBuilder::new(self.handle.clone())
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
+    ///   - [`policy(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_policy):<br>required: **false**<br><p>The key policy to attach to the KMS key.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>CreateKey</code> and <code>PutKeyPolicy</code> API requests succeed, even though the policy statement is ineffective.</p>  <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>If you do not provide a key policy, KMS attaches a default key policy to the KMS key. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p><note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p><br>
+    ///   - [`description(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_description):<br>required: **false**<br><p>A description of the KMS key. Use a description that helps you decide whether the KMS key is appropriate for a task. The default value is an empty string (no description).</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>To set or change the description after the key is created, use <a>UpdateKeyDescription</a>.</p><br>
     ///   - [`key_usage(KeyUsageType)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_usage) / [`set_key_usage(Option<KeyUsageType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_usage):<br>required: **false**<br><p>Determines the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a> for which you can use the KMS key. The default value is <code>ENCRYPT_DECRYPT</code>. This parameter is optional when you are creating a symmetric encryption KMS key; otherwise, it is required. You can't change the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#key-usage"> <code>KeyUsage</code> </a> value after the KMS key is created. Each KMS key can have only one key usage. This follows key usage best practices according to <a href="https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final">NIST SP 800-57 Recommendations for Key Management</a>, section 5.2, Key usage.</p> <p>Select only one valid value.</p> <ul>  <li>   <p>For symmetric encryption KMS keys, omit the parameter or specify <code>ENCRYPT_DECRYPT</code>.</p></li>  <li>   <p>For HMAC KMS keys (symmetric), specify <code>GENERATE_VERIFY_MAC</code>.</p></li>  <li>   <p>For asymmetric KMS keys with RSA key pairs, specify <code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with NIST-standard elliptic curve key pairs, specify <code>SIGN_VERIFY</code> or <code>KEY_AGREEMENT</code>.</p></li>  <li>   <p>For asymmetric KMS keys with <code>ECC_SECG_P256K1</code> key pairs, specify <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with ML-DSA key pairs, specify <code>SIGN_VERIFY</code>.</p></li>  <li>   <p>For asymmetric KMS keys with SM2 key pairs (China Regions only), specify <code>ENCRYPT_DECRYPT</code>, <code>SIGN_VERIFY</code>, or <code>KEY_AGREEMENT</code>.</p></li> </ul><br>
     ///   - [`customer_master_key_spec(CustomerMasterKeySpec)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::customer_master_key_spec) / [`set_customer_master_key_spec(Option<CustomerMasterKeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_customer_master_key_spec):<br>required: **false**<br><p>Instead, use the <code>KeySpec</code> parameter.</p> <p>The <code>KeySpec</code> and <code>CustomerMasterKeySpec</code> parameters work the same way. Only the names differ. We recommend that you use <code>KeySpec</code> parameter in your code. However, to avoid breaking changes, KMS supports both parameters.</p><br>
-    ///   - [`key_spec(KeySpec)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_spec) / [`set_key_spec(Option<KeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_spec):<br>required: **false**<br><p>Specifies the type of KMS key to create. The default value, <code>SYMMETRIC_DEFAULT</code>, creates a KMS key with a 256-bit AES-GCM key that is used for encryption and decryption, except in China Regions, where it creates a 128-bit symmetric key that uses SM4 encryption. For a detailed description of all supported key specs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose-key-spec.html">Key spec reference</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p> <p>The <code>KeySpec</code> determines whether the KMS key contains a symmetric key or an asymmetric key pair. It also determines the algorithms that the KMS key supports. You can't change the <code>KeySpec</code> after the KMS key is created. To further restrict the algorithms that can be used with the KMS key, use a condition key in its key policy or IAM policy. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-encryption-algorithm">kms:EncryptionAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-mac-algorithm">kms:MacAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-key-agreement-algorithm">kms:KeyAgreementAlgorithm</a>, or <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-signing-algorithm">kms:SigningAlgorithm</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p><important>  <p><a href="http://aws.amazon.com/kms/features/#AWS_Service_Integration">Amazon Web Services services that are integrated with KMS</a> use symmetric encryption KMS keys to protect your data. These services do not support asymmetric KMS keys or HMAC KMS keys.</p> </important> <p>KMS supports the following key specs for KMS keys:</p> <ul>  <li>   <p>Symmetric encryption key (default)</p>   <ul>    <li>     <p><code>SYMMETRIC_DEFAULT</code></p></li>   </ul></li>  <li>   <p>HMAC keys (symmetric)</p>   <ul>    <li>     <p><code>HMAC_224</code></p></li>    <li>     <p><code>HMAC_256</code></p></li>    <li>     <p><code>HMAC_384</code></p></li>    <li>     <p><code>HMAC_512</code></p></li>   </ul></li>  <li>   <p>Asymmetric RSA key pairs (encryption and decryption -or- signing and verification)</p>   <ul>    <li>     <p><code>RSA_2048</code></p></li>    <li>     <p><code>RSA_3072</code></p></li>    <li>     <p><code>RSA_4096</code></p></li>   </ul></li>  <li>   <p>Asymmetric NIST-standard elliptic curve key pairs (signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>ECC_NIST_P256</code> (secp256r1)</p></li>    <li>     <p><code>ECC_NIST_P384</code> (secp384r1)</p></li>    <li>     <p><code>ECC_NIST_P521</code> (secp521r1)</p></li>    <li>     <p><code>ECC_NIST_EDWARDS25519</code> (ed25519) - signing and verification only</p>     <ul>      <li>       <p><b>Note:</b> For ECC_NIST_EDWARDS25519 KMS keys, the ED25519_SHA_512 signing algorithm requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:RAW</code> </a>, while ED25519_PH_SHA_512 requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:DIGEST</code> </a>. These message types cannot be used interchangeably.</p></li>     </ul></li>   </ul></li>  <li>   <p>Other asymmetric elliptic curve key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ECC_SECG_P256K1</code> (secp256k1), commonly used for cryptocurrencies.</p></li>   </ul></li>  <li>   <p>Asymmetric ML-DSA key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ML_DSA_44</code></p></li>    <li>     <p><code>ML_DSA_65</code></p></li>    <li>     <p><code>ML_DSA_87</code></p></li>   </ul></li>  <li>   <p>SM2 key pairs (encryption and decryption -or- signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>SM2</code> (China Regions only)</p></li>   </ul></li> </ul><br>
+    ///   - [`key_spec(KeySpec)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_spec) / [`set_key_spec(Option<KeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_spec):<br>required: **false**<br><p>Specifies the type of KMS key to create. The default value, <code>SYMMETRIC_DEFAULT</code>, creates a KMS key with a 256-bit AES-GCM key that is used for encryption and decryption, except in China Regions, where it creates a 128-bit symmetric key that uses SM4 encryption. For a detailed description of all supported key specs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose-key-spec.html">Key spec reference</a> in the <i> <i>Key Management Service Developer Guide</i></i>.</p> <p>The <code>KeySpec</code> determines whether the KMS key contains a symmetric key or an asymmetric key pair. It also determines the algorithms that the KMS key supports. You can't change the <code>KeySpec</code> after the KMS key is created. To further restrict the algorithms that can be used with the KMS key, use a condition key in its key policy or IAM policy. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-encryption-algorithm">kms:EncryptionAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-mac-algorithm">kms:MacAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-key-agreement-algorithm">kms:KeyAgreementAlgorithm</a>, or <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-signing-algorithm">kms:SigningAlgorithm</a> in the <i> <i>Key Management Service Developer Guide</i></i>.</p><important>  <p><a href="http://aws.amazon.com/kms/features/#AWS_Service_Integration">Amazon Web Services services that are integrated with KMS</a> use symmetric encryption KMS keys to protect your data. These services do not support asymmetric KMS keys or HMAC KMS keys.</p> </important> <p>KMS supports the following key specs for KMS keys:</p> <ul>  <li>   <p>Symmetric encryption key (default)</p>   <ul>    <li>     <p><code>SYMMETRIC_DEFAULT</code></p></li>   </ul></li>  <li>   <p>HMAC keys (symmetric)</p>   <ul>    <li>     <p><code>HMAC_224</code></p></li>    <li>     <p><code>HMAC_256</code></p></li>    <li>     <p><code>HMAC_384</code></p></li>    <li>     <p><code>HMAC_512</code></p></li>   </ul></li>  <li>   <p>Asymmetric RSA key pairs (encryption and decryption -or- signing and verification)</p>   <ul>    <li>     <p><code>RSA_2048</code></p></li>    <li>     <p><code>RSA_3072</code></p></li>    <li>     <p><code>RSA_4096</code></p></li>   </ul></li>  <li>   <p>Asymmetric NIST-standard elliptic curve key pairs (signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>ECC_NIST_P256</code> (secp256r1)</p></li>    <li>     <p><code>ECC_NIST_P384</code> (secp384r1)</p></li>    <li>     <p><code>ECC_NIST_P521</code> (secp521r1)</p></li>    <li>     <p><code>ECC_NIST_EDWARDS25519</code> (ed25519) - signing and verification only</p>     <ul>      <li>       <p><b>Note:</b> For ECC_NIST_EDWARDS25519 KMS keys, the ED25519_SHA_512 signing algorithm requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:RAW</code> </a>, while ED25519_PH_SHA_512 requires <a href="kms/latest/APIReference/API_Sign.html#KMS-Sign-request-MessageType"> <code>MessageType:DIGEST</code> </a>. These message types cannot be used interchangeably.</p></li>     </ul></li>   </ul></li>  <li>   <p>Other asymmetric elliptic curve key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ECC_SECG_P256K1</code> (secp256k1), commonly used for cryptocurrencies.</p></li>   </ul></li>  <li>   <p>Asymmetric ML-DSA key pairs (signing and verification)</p>   <ul>    <li>     <p><code>ML_DSA_44</code></p></li>    <li>     <p><code>ML_DSA_65</code></p></li>    <li>     <p><code>ML_DSA_87</code></p></li>   </ul></li>  <li>   <p>SM2 key pairs (encryption and decryption -or- signing and verification -or- deriving shared secrets)</p>   <ul>    <li>     <p><code>SM2</code> (China Regions only)</p></li>   </ul></li> </ul><br>
     ///   - [`origin(OriginType)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::origin) / [`set_origin(Option<OriginType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_origin):<br>required: **false**<br><p>The source of the key material for the KMS key. You cannot change the origin after you create the KMS key. The default is <code>AWS_KMS</code>, which means that KMS creates the key material.</p> <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-create-cmk.html">create a KMS key with no key material</a> (for imported key material), set this value to <code>EXTERNAL</code>. For more information about importing key material into KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>Key Management Service Developer Guide</i>. The <code>EXTERNAL</code> origin value is valid only for symmetric KMS keys.</p> <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html">create a KMS key in an CloudHSM key store</a> and create its key material in the associated CloudHSM cluster, set this value to <code>AWS_CLOUDHSM</code>. You must also use the <code>CustomKeyStoreId</code> parameter to identify the CloudHSM key store. The <code>KeySpec</code> value must be <code>SYMMETRIC_DEFAULT</code>.</p> <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keys.html">create a KMS key in an external key store</a>, set this value to <code>EXTERNAL_KEY_STORE</code>. You must also use the <code>CustomKeyStoreId</code> parameter to identify the external key store and the <code>XksKeyId</code> parameter to identify the associated external key. The <code>KeySpec</code> value must be <code>SYMMETRIC_DEFAULT</code>.</p><br>
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_custom_key_store_id):<br>required: **false**<br><p>Creates the KMS key in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. The <code>ConnectionState</code> of the custom key store must be <code>CONNECTED</code>. To find the CustomKeyStoreID and ConnectionState use the <code>DescribeCustomKeyStores</code> operation.</p> <p>This parameter is valid only for symmetric encryption KMS keys in a single Region. You cannot create any other type of KMS key in a custom key store.</p> <p>When you create a KMS key in an CloudHSM key store, KMS generates a non-exportable 256-bit symmetric key in its associated CloudHSM cluster and associates it with the KMS key. When you create a KMS key in an external key store, you must use the <code>XksKeyId</code> parameter to specify an external key that serves as key material for the KMS key.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_custom_key_store_id):<br>required: **false**<br><p>Creates the KMS key in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. The <code>ConnectionState</code> of the custom key store must be <code>CONNECTED</code>. To find the CustomKeyStoreID and ConnectionState use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This parameter is valid only for symmetric encryption KMS keys in a single Region. You cannot create any other type of KMS key in a custom key store.</p> <p>When you create a KMS key in an CloudHSM key store, KMS generates a non-exportable 256-bit symmetric key in its associated CloudHSM cluster and associates it with the KMS key. When you create a KMS key in an external key store, you must use the <code>XksKeyId</code> parameter to specify an external key that serves as key material for the KMS key.</p><br>
     ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
-    ///   - [`tags(Tag)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_tags):<br>required: **false**<br><p>Assigns one or more tags to the KMS key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <code>TagResource</code> operation.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <note>  <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p> <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tags in KMS</a>.</p><br>
-    ///   - [`multi_region(bool)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::multi_region) / [`set_multi_region(Option<bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_multi_region):<br>required: **false**<br><p>Creates a multi-Region primary key that you can replicate into other Amazon Web Services Regions. You cannot change this value after you create the KMS key.</p> <p>For a multi-Region key, set this parameter to <code>True</code>. For a single-Region KMS key, omit this parameter or set it to <code>False</code>. The default value is <code>False</code>.</p> <p>This operation supports <i>multi-Region keys</i>, an KMS feature that lets you create multiple interoperable KMS keys in different Amazon Web Services Regions. Because these KMS keys have the same key ID, key material, and other metadata, you can use them interchangeably to encrypt data in one Amazon Web Services Region and decrypt it in a different Amazon Web Services Region without re-encrypting the data or making a cross-Region call. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Multi-Region keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p> <p>This value creates a <i>primary key</i>, not a replica. To create a <i>replica key</i>, use the <code>ReplicateKey</code> operation.</p> <p>You can create a symmetric or asymmetric multi-Region key, and you can create a multi-Region key with imported key material. However, you cannot create a multi-Region key in a custom key store.</p><br>
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
     pub fn delete_custom_key_store(&self) -> super::super::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder {
```

### `src/client/delete_imported_key_material.rs`

```diff
--- reference/src/client/delete_imported_key_material.rs
+++ generated/src/client/delete_imported_key_material.rs
@@ -3,13 +3,15 @@
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
     /// - On failure, responds with [`SdkError<DeleteImportedKeyMaterialError>`](crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError)
-    pub fn delete_imported_key_material(&self) -> super::super::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder {
+    pub fn delete_imported_key_material(
+        &self,
+    ) -> super::super::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder {
         super::super::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::new(self.handle.clone())
     }
 }
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
+    ///   - [`public_key(Blob)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::public_key) / [`set_public_key(Option<Blob>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_public_key):<br>required: **true**<br><p>Specifies the public key in your peer's NIST-standard elliptic curve (ECC) or SM2 (China Regions only) key pair.</p> <p>The public key must be a DER-encoded X.509 public key, also known as <code>SubjectPublicKeyInfo</code> (SPKI), as defined in <a href="https://tools.ietf.org/html/rfc5280">RFC 5280</a>.</p> <p><a>GetPublicKey</a> returns the public key of an asymmetric KMS key pair in the required DER-encoded format.</p><note>  <p>If you use <a href="https://docs.aws.amazon.com/cli/v1/userguide/cli-chap-welcome.html">Amazon Web Services CLI version 1</a>, you must provide the DER-encoded X.509 public key in a file. Otherwise, the Amazon Web Services CLI Base64-encodes the public key a second time, resulting in a <code>ValidationException</code>.</p> </note> <p>You can specify the public key as binary data in a file using fileb (<code>fileb://<path-to-file></path-to-file></code>) or in-line using a Base64 encoded string.</p><br>
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
     pub fn disable_key(&self) -> super::super::operation::disable_key::builders::DisableKeyFluentBuilder {
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
     pub fn disable_key_rotation(&self) -> super::super::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder {
```

### `src/client/disconnect_custom_key_store.rs`

```diff
--- reference/src/client/disconnect_custom_key_store.rs
+++ generated/src/client/disconnect_custom_key_store.rs
@@ -3,10 +3,12 @@
     /// Constructs a fluent builder for the [`DisconnectCustomKeyStore`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p><br>
+    ///   - [`custom_key_store_id(impl Into<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::set_custom_key_store_id):<br>required: **true**<br><p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p><br>
     /// - On success, responds with [`DisconnectCustomKeyStoreOutput`](crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreOutput)
     /// - On failure, responds with [`SdkError<DisconnectCustomKeyStoreError>`](crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError)
-    pub fn disconnect_custom_key_store(&self) -> super::super::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder {
+    pub fn disconnect_custom_key_store(
+        &self,
+    ) -> super::super::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder {
         super::super::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::new(self.handle.clone())
     }
 }
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
     pub fn enable_key(&self) -> super::super::operation::enable_key::builders::EnableKeyFluentBuilder {
```

### `src/client/enable_key_rotation.rs`

```diff
--- reference/src/client/enable_key_rotation.rs
+++ generated/src/client/enable_key_rotation.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`EnableKeyRotation`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#multi-region-rotate">multi-Region keys</a>, set the property on the primary key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#multi-region-rotate">multi-Region keys</a>, set the property on the primary key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`rotation_period_in_days(i32)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::rotation_period_in_days) / [`set_rotation_period_in_days(Option<i32>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_rotation_period_in_days):<br>required: **false**<br><p>Use this parameter to specify a custom period of time between each rotation date. If no value is specified, the default value is 365 days.</p> <p>The rotation period defines the number of days after you enable automatic key rotation that KMS will rotate your key material, and the number of days between each automatic rotation thereafter.</p> <p>You can use the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-rotation-period-in-days"> <code>kms:RotationPeriodInDays</code> </a> condition key to further constrain the values that principals can specify in the <code>RotationPeriodInDays</code> parameter.</p> <p></p><br>
     /// - On success, responds with [`EnableKeyRotationOutput`](crate::operation::enable_key_rotation::EnableKeyRotationOutput)
     /// - On failure, responds with [`SdkError<EnableKeyRotationError>`](crate::operation::enable_key_rotation::EnableKeyRotationError)
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
     pub fn get_key_rotation_status(&self) -> super::super::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder {
         super::super::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::new(self.handle.clone())
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
     pub fn get_parameters_for_import(&self) -> super::super::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder {
         super::super::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::new(self.handle.clone())
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
@@ -3,9 +3,9 @@
     /// Constructs a fluent builder for the [`PutKeyPolicy`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_key_id):<br>required: **true**<br><p>Sets the key policy on the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_key_id):<br>required: **true**<br><p>Sets the key policy on the specified KMS key.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`policy_name(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy_name):<br>required: **false**<br><p>The name of the key policy. If no policy name is specified, the default value is <code>default</code>. The only valid value is <code>default</code>.</p><br>
-    ///   - [`policy(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The key policy to attach to the KMS key.</p> <p>The key policy must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul><note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>PutKeyPolicy</code> API request succeeds, even though the policy statement is ineffective.</p>  <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul><note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>.For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p><br>
+    ///   - [`policy(impl Into<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The key policy to attach to the KMS key.</p> <p>The key policy must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <note>  <p>If either of the required <code>Resource</code> or <code>Action</code> elements are missing from a key policy statement, the policy statement has no effect. When a key policy statement is missing one of these elements, the KMS console correctly reports an error, but the <code>PutKeyPolicy</code> API request succeeds, even though the policy statement is ineffective.</p>  <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </note> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul> <note>  <p>If the key policy exceeds the length constraint, KMS returns a <code>LimitExceededException</code>.</p> </note> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>.For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p><br>
     ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
     /// - On success, responds with [`PutKeyPolicyOutput`](crate::operation::put_key_policy::PutKeyPolicyOutput)
     /// - On failure, responds with [`SdkError<PutKeyPolicyError>`](crate::operation::put_key_policy::PutKeyPolicyError)
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
+    ///   - [`policy(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_policy):<br>required: **false**<br><p>The key policy to attach to the KMS key. This parameter is optional. If you do not provide a key policy, KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">default key policy</a> to the KMS key.</p> <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul>  <li>   <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p></li>  <li>   <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p></li> </ul> <p>A key policy document can include only the following characters:</p> <ul>  <li>   <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p></li>  <li>   <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p></li>  <li>   <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p></li> </ul> <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i></i>.</p><br>
     ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_bypass_policy_lockout_safety_check):<br>required: **false**<br><p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p><important>  <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a> request on the KMS key.</p><br>
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
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`ScheduleKeyDeletion`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`key_id(impl Into<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_key_id):<br>required: **true**<br><p>The unique identifier of the KMS key to delete.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
+    ///   - [`key_id(impl Into<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_key_id):<br>required: **true**<br><p>The unique identifier of the KMS key to delete.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p><br>
     ///   - [`pending_window_in_days(i32)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::pending_window_in_days) / [`set_pending_window_in_days(Option<i32>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_pending_window_in_days):<br>required: **false**<br><p>The waiting period, specified in number of days. After the waiting period ends, KMS deletes the KMS key.</p> <p>If the KMS key is a multi-Region primary key with replica keys, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30. You can use the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-schedule-key-deletion-pending-window-in-days"> <code>kms:ScheduleKeyDeletionPendingWindowInDays</code> </a> condition key to further constrain the values that principals can specify in the <code>PendingWindowInDays</code> parameter.</p><br>
     /// - On success, responds with [`ScheduleKeyDeletionOutput`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionOutput) with field(s):
     ///   - [`key_id(Option<String>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key whose deletion is scheduled.</p>
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
     pub fn update_alias(&self) -> super::super::operation::update_alias::builders::UpdateAliasFluentBuilder {
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

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -29,7 +29,10 @@
 /// Endpoint resolver trait specific to AWS Key Management Service
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -268,7 +271,10 @@
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

### `src/operation/cancel_key_deletion/_cancel_key_deletion_input.rs`

```diff
--- reference/src/operation/cancel_key_deletion/_cancel_key_deletion_input.rs
+++ generated/src/operation/cancel_key_deletion/_cancel_key_deletion_input.rs
@@ -89,7 +89,10 @@
     /// Consumes the builder and constructs a [`CancelKeyDeletionInput`](crate::operation::cancel_key_deletion::CancelKeyDeletionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::cancel_key_deletion::CancelKeyDeletionInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::cancel_key_deletion::CancelKeyDeletionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::cancel_key_deletion::CancelKeyDeletionInput { key_id: self.key_id })
     }
 }
```

### `src/operation/cancel_key_deletion.rs`

```diff
--- reference/src/operation/cancel_key_deletion.rs
+++ generated/src/operation/cancel_key_deletion.rs
@@ -255,7 +255,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_cancel_key_deletion::ser_cancel_key_deletion_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_cancel_key_deletion::ser_cancel_key_deletion_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/connect_custom_key_store/_connect_custom_key_store_input.rs`

```diff
--- reference/src/operation/connect_custom_key_store/_connect_custom_key_store_input.rs
+++ generated/src/operation/connect_custom_key_store/_connect_custom_key_store_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`ConnectCustomKeyStoreInput`](crate::operation::connect_custom_key_store::ConnectCustomKeyStoreInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreInput {
             custom_key_store_id: self.custom_key_store_id,
         })
```

### `src/operation/create_alias/_create_alias_input.rs`

```diff
--- reference/src/operation/create_alias/_create_alias_input.rs
+++ generated/src/operation/create_alias/_create_alias_input.rs
@@ -134,7 +134,9 @@
         &self.target_key_id
     }
     /// Consumes the builder and constructs a [`CreateAliasInput`](crate::operation::create_alias::CreateAliasInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_alias::CreateAliasInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_alias::CreateAliasInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_alias::CreateAliasInput {
             alias_name: self.alias_name,
             target_key_id: self.target_key_id,
```

### `src/operation/create_custom_key_store/_create_custom_key_store_input.rs`

```diff
--- reference/src/operation/create_custom_key_store/_create_custom_key_store_input.rs
+++ generated/src/operation/create_custom_key_store/_create_custom_key_store_input.rs
@@ -438,7 +438,9 @@
     /// <p>The <code>XksProxyAuthenticationCredential</code> has two required elements: <code>RawSecretAccessKey</code>, a secret key, and <code>AccessKeyId</code>, a unique identifier for the <code>RawSecretAccessKey</code>. For character requirements, see <a href="API_XksProxyAuthenticationCredentialType.html">XksProxyAuthenticationCredentialType</a>.</p>
     /// <p>KMS uses this authentication credential to sign requests to the external key store proxy on your behalf. This credential is unrelated to Identity and Access Management (IAM) and Amazon Web Services credentials.</p>
     /// <p>This parameter doesn't set or change the authentication credentials on the XKS proxy. It just tells KMS the credential that you established on your external key store proxy. If you rotate your proxy authentication credential, use the <code>UpdateCustomKeyStore</code> operation to provide the new credential to KMS.</p>
-    pub fn get_xks_proxy_authentication_credential(&self) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
+    pub fn get_xks_proxy_authentication_credential(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
         &self.xks_proxy_authentication_credential
     }
     /// <p>Indicates how KMS communicates with the external key store proxy. This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p>
@@ -464,8 +466,10 @@
     /// Consumes the builder and constructs a [`CreateCustomKeyStoreInput`](crate::operation::create_custom_key_store::CreateCustomKeyStoreInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_custom_key_store::CreateCustomKeyStoreInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_custom_key_store::CreateCustomKeyStoreInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_custom_key_store::CreateCustomKeyStoreInput {
             custom_key_store_name: self.custom_key_store_name,
             cloud_hsm_cluster_id: self.cloud_hsm_cluster_id,
```

### `src/operation/create_custom_key_store/builders.rs`

```diff
--- reference/src/operation/create_custom_key_store/builders.rs
+++ generated/src/operation/create_custom_key_store/builders.rs
@@ -386,7 +386,9 @@
     /// <p>The <code>XksProxyAuthenticationCredential</code> has two required elements: <code>RawSecretAccessKey</code>, a secret key, and <code>AccessKeyId</code>, a unique identifier for the <code>RawSecretAccessKey</code>. For character requirements, see <a href="API_XksProxyAuthenticationCredentialType.html">XksProxyAuthenticationCredentialType</a>.</p>
     /// <p>KMS uses this authentication credential to sign requests to the external key store proxy on your behalf. This credential is unrelated to Identity and Access Management (IAM) and Amazon Web Services credentials.</p>
     /// <p>This parameter doesn't set or change the authentication credentials on the XKS proxy. It just tells KMS the credential that you established on your external key store proxy. If you rotate your proxy authentication credential, use the <code>UpdateCustomKeyStore</code> operation to provide the new credential to KMS.</p>
-    pub fn get_xks_proxy_authentication_credential(&self) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
+    pub fn get_xks_proxy_authentication_credential(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
         self.inner.get_xks_proxy_authentication_credential()
     }
     /// <p>Indicates how KMS communicates with the external key store proxy. This parameter is required for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p>
```

### `src/operation/create_custom_key_store.rs`

```diff
--- reference/src/operation/create_custom_key_store.rs
+++ generated/src/operation/create_custom_key_store.rs
@@ -288,9 +288,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_custom_key_store::ser_create_custom_key_store_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_create_custom_key_store::ser_create_custom_key_store_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/create_grant/_create_grant_input.rs`

```diff
--- reference/src/operation/create_grant/_create_grant_input.rs
+++ generated/src/operation/create_grant/_create_grant_input.rs
@@ -427,7 +427,9 @@
         &self.retiring_service_principal
     }
     /// Consumes the builder and constructs a [`CreateGrantInput`](crate::operation::create_grant::CreateGrantInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_grant::CreateGrantInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_grant::CreateGrantInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_grant::CreateGrantInput {
             key_id: self.key_id,
             grantee_principal: self.grantee_principal,
```

### `src/operation/create_key/_create_key_input.rs`

```diff
--- reference/src/operation/create_key/_create_key_input.rs
+++ generated/src/operation/create_key/_create_key_input.rs
@@ -914,7 +914,9 @@
         &self.xks_key_id
     }
     /// Consumes the builder and constructs a [`CreateKeyInput`](crate::operation::create_key::CreateKeyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_key::CreateKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_key::CreateKeyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_key::CreateKeyInput {
             policy: self.policy,
             description: self.description,
@@ -923,7 +925,7 @@
             key_spec: self.key_spec,
             origin: self.origin,
             custom_key_store_id: self.custom_key_store_id,
-            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
+            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check.unwrap_or_default(),
             tags: self.tags,
             multi_region: self.multi_region,
             xks_key_id: self.xks_key_id,
```

### `src/operation/create_key/builders.rs`

```diff
--- reference/src/operation/create_key/builders.rs
+++ generated/src/operation/create_key/builders.rs
@@ -106,14 +106,20 @@
     inner: super::super::super::operation::create_key::builders::CreateKeyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::create_key::CreateKeyOutput, super::super::super::operation::create_key::CreateKeyError>
-    for CreateKeyFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::create_key::CreateKeyOutput,
+        super::super::super::operation::create_key::CreateKeyError,
+    > for CreateKeyFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::create_key::CreateKeyOutput, super::super::super::operation::create_key::CreateKeyError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::create_key::CreateKeyOutput,
+            super::super::super::operation::create_key::CreateKeyError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/decrypt/_decrypt_input.rs`

```diff
--- reference/src/operation/decrypt/_decrypt_input.rs
+++ generated/src/operation/decrypt/_decrypt_input.rs
@@ -347,7 +347,9 @@
         &self.dry_run_modifiers
     }
     /// Consumes the builder and constructs a [`DecryptInput`](crate::operation::decrypt::DecryptInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::decrypt::DecryptInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::decrypt::DecryptInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::decrypt::DecryptInput {
             ciphertext_blob: self.ciphertext_blob,
             encryption_context: self.encryption_context,
```

### `src/operation/decrypt/builders.rs`

```diff
--- reference/src/operation/decrypt/builders.rs
+++ generated/src/operation/decrypt/builders.rs
@@ -61,14 +61,20 @@
     inner: super::super::super::operation::decrypt::builders::DecryptInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::decrypt::DecryptOutput, super::super::super::operation::decrypt::DecryptError>
-    for DecryptFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::decrypt::DecryptOutput,
+        super::super::super::operation::decrypt::DecryptError,
+    > for DecryptFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::decrypt::DecryptOutput, super::super::super::operation::decrypt::DecryptError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::decrypt::DecryptOutput,
+            super::super::super::operation::decrypt::DecryptError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -118,8 +124,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::decrypt::DecryptOutput, super::super::super::operation::decrypt::DecryptError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::decrypt::DecryptOutput,
+        super::super::super::operation::decrypt::DecryptError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/decrypt.rs`

```diff
--- reference/src/operation/decrypt.rs
+++ generated/src/operation/decrypt.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::decrypt::DecryptError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::decrypt::DecryptError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
```

### `src/operation/delete_alias/_delete_alias_input.rs`

```diff
--- reference/src/operation/delete_alias/_delete_alias_input.rs
+++ generated/src/operation/delete_alias/_delete_alias_input.rs
@@ -42,7 +42,9 @@
         &self.alias_name
     }
     /// Consumes the builder and constructs a [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_alias::DeleteAliasInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_alias::DeleteAliasInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_alias::DeleteAliasInput { alias_name: self.alias_name })
     }
 }
```

### `src/operation/delete_custom_key_store/_delete_custom_key_store_input.rs`

```diff
--- reference/src/operation/delete_custom_key_store/_delete_custom_key_store_input.rs
+++ generated/src/operation/delete_custom_key_store/_delete_custom_key_store_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`DeleteCustomKeyStoreInput`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreInput {
             custom_key_store_id: self.custom_key_store_id,
         })
```

### `src/operation/delete_custom_key_store.rs`

```diff
--- reference/src/operation/delete_custom_key_store.rs
+++ generated/src/operation/delete_custom_key_store.rs
@@ -258,9 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_custom_key_store::ser_delete_custom_key_store_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_delete_custom_key_store::ser_delete_custom_key_store_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_imported_key_material/_delete_imported_key_material_input.rs`

```diff
--- reference/src/operation/delete_imported_key_material/_delete_imported_key_material_input.rs
+++ generated/src/operation/delete_imported_key_material/_delete_imported_key_material_input.rs
@@ -129,9 +129,11 @@
         super::super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialInput {
-            key_id: self.key_id,
-            key_material_id: self.key_material_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialInput {
+                key_id: self.key_id,
+                key_material_id: self.key_material_id,
+            },
+        )
     }
 }
```

### `src/operation/derive_shared_secret/_derive_shared_secret_input.rs`

```diff
--- reference/src/operation/derive_shared_secret/_derive_shared_secret_input.rs
+++ generated/src/operation/derive_shared_secret/_derive_shared_secret_input.rs
@@ -274,8 +274,10 @@
     /// Consumes the builder and constructs a [`DeriveSharedSecretInput`](crate::operation::derive_shared_secret::DeriveSharedSecretInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::derive_shared_secret::DeriveSharedSecretInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::derive_shared_secret::DeriveSharedSecretInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::derive_shared_secret::DeriveSharedSecretInput {
             key_id: self.key_id,
             key_agreement_algorithm: self.key_agreement_algorithm,
```

### `src/operation/derive_shared_secret.rs`

```diff
--- reference/src/operation/derive_shared_secret.rs
+++ generated/src/operation/derive_shared_secret.rs
@@ -256,8 +256,9 @@
             );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_derive_shared_secret::ser_derive_shared_secret_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_derive_shared_secret::ser_derive_shared_secret_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/describe_custom_key_stores/_describe_custom_key_stores_output.rs`

```diff
--- reference/src/operation/describe_custom_key_stores/_describe_custom_key_stores_output.rs
+++ generated/src/operation/describe_custom_key_stores/_describe_custom_key_stores_output.rs
@@ -61,7 +61,10 @@
         self
     }
     /// <p>Contains metadata about each custom key store.</p>
-    pub fn set_custom_key_stores(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::CustomKeyStoresListEntry>>) -> Self {
+    pub fn set_custom_key_stores(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::CustomKeyStoresListEntry>>,
+    ) -> Self {
         self.custom_key_stores = input;
         self
     }
```

### `src/operation/describe_custom_key_stores/paginator.rs`

```diff
--- reference/src/operation/describe_custom_key_stores/paginator.rs
+++ generated/src/operation/describe_custom_key_stores/paginator.rs
@@ -86,8 +86,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::describe_custom_key_stores::DescribeCustomKeyStores::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::describe_custom_key_stores::DescribeCustomKeyStores::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/describe_key/_describe_key_input.rs`

```diff
--- reference/src/operation/describe_key/_describe_key_input.rs
+++ generated/src/operation/describe_key/_describe_key_input.rs
@@ -146,7 +146,9 @@
         &self.grant_tokens
     }
     /// Consumes the builder and constructs a [`DescribeKeyInput`](crate::operation::describe_key::DescribeKeyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::describe_key::DescribeKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::describe_key::DescribeKeyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::describe_key::DescribeKeyInput {
             key_id: self.key_id,
             grant_tokens: self.grant_tokens,
```

### `src/operation/disable_key/_disable_key_input.rs`

```diff
--- reference/src/operation/disable_key/_disable_key_input.rs
+++ generated/src/operation/disable_key/_disable_key_input.rs
@@ -87,7 +87,9 @@
         &self.key_id
     }
     /// Consumes the builder and constructs a [`DisableKeyInput`](crate::operation::disable_key::DisableKeyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::disable_key::DisableKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::disable_key::DisableKeyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::disable_key::DisableKeyInput { key_id: self.key_id })
     }
 }
```

### `src/operation/disable_key_rotation/_disable_key_rotation_input.rs`

```diff
--- reference/src/operation/disable_key_rotation/_disable_key_rotation_input.rs
+++ generated/src/operation/disable_key_rotation/_disable_key_rotation_input.rs
@@ -89,8 +89,10 @@
     /// Consumes the builder and constructs a [`DisableKeyRotationInput`](crate::operation::disable_key_rotation::DisableKeyRotationInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::disable_key_rotation::DisableKeyRotationInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::disable_key_rotation::DisableKeyRotationInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::disable_key_rotation::DisableKeyRotationInput { key_id: self.key_id })
     }
 }
```

### `src/operation/disable_key_rotation.rs`

```diff
--- reference/src/operation/disable_key_rotation.rs
+++ generated/src/operation/disable_key_rotation.rs
@@ -255,8 +255,9 @@
             );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_disable_key_rotation::ser_disable_key_rotation_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_disable_key_rotation::ser_disable_key_rotation_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/disconnect_custom_key_store/_disconnect_custom_key_store_input.rs`

```diff
--- reference/src/operation/disconnect_custom_key_store/_disconnect_custom_key_store_input.rs
+++ generated/src/operation/disconnect_custom_key_store/_disconnect_custom_key_store_input.rs
@@ -48,8 +48,10 @@
         super::super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreInput {
-            custom_key_store_id: self.custom_key_store_id,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreInput {
+                custom_key_store_id: self.custom_key_store_id,
+            },
+        )
     }
 }
```

### `src/operation/enable_key/_enable_key_input.rs`

```diff
--- reference/src/operation/enable_key/_enable_key_input.rs
+++ generated/src/operation/enable_key/_enable_key_input.rs
@@ -87,7 +87,9 @@
         &self.key_id
     }
     /// Consumes the builder and constructs a [`EnableKeyInput`](crate::operation::enable_key::EnableKeyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::enable_key::EnableKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::enable_key::EnableKeyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::enable_key::EnableKeyInput { key_id: self.key_id })
     }
 }
```

### `src/operation/enable_key/builders.rs`

```diff
--- reference/src/operation/enable_key/builders.rs
+++ generated/src/operation/enable_key/builders.rs
@@ -34,14 +34,20 @@
     inner: super::super::super::operation::enable_key::builders::EnableKeyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::enable_key::EnableKeyOutput, super::super::super::operation::enable_key::EnableKeyError>
-    for EnableKeyFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::enable_key::EnableKeyOutput,
+        super::super::super::operation::enable_key::EnableKeyError,
+    > for EnableKeyFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::enable_key::EnableKeyOutput, super::super::super::operation::enable_key::EnableKeyError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::enable_key::EnableKeyOutput,
+            super::super::super::operation::enable_key::EnableKeyError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/enable_key_rotation/_enable_key_rotation_input.rs`

```diff
--- reference/src/operation/enable_key_rotation/_enable_key_rotation_input.rs
+++ generated/src/operation/enable_key_rotation/_enable_key_rotation_input.rs
@@ -125,7 +125,10 @@
     /// Consumes the builder and constructs a [`EnableKeyRotationInput`](crate::operation::enable_key_rotation::EnableKeyRotationInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::enable_key_rotation::EnableKeyRotationInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::enable_key_rotation::EnableKeyRotationInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::enable_key_rotation::EnableKeyRotationInput {
             key_id: self.key_id,
             rotation_period_in_days: self.rotation_period_in_days,
```

### `src/operation/enable_key_rotation.rs`

```diff
--- reference/src/operation/enable_key_rotation.rs
+++ generated/src/operation/enable_key_rotation.rs
@@ -255,7 +255,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_enable_key_rotation::ser_enable_key_rotation_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_enable_key_rotation::ser_enable_key_rotation_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/encrypt/_encrypt_input.rs`

```diff
--- reference/src/operation/encrypt/_encrypt_input.rs
+++ generated/src/operation/encrypt/_encrypt_input.rs
@@ -286,7 +286,9 @@
         &self.dry_run
     }
     /// Consumes the builder and constructs a [`EncryptInput`](crate::operation::encrypt::EncryptInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::encrypt::EncryptInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::encrypt::EncryptInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::encrypt::EncryptInput {
             key_id: self.key_id,
             plaintext: self.plaintext,
```

### `src/operation/encrypt/builders.rs`

```diff
--- reference/src/operation/encrypt/builders.rs
+++ generated/src/operation/encrypt/builders.rs
@@ -83,14 +83,20 @@
     inner: super::super::super::operation::encrypt::builders::EncryptInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::encrypt::EncryptOutput, super::super::super::operation::encrypt::EncryptError>
-    for EncryptFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::encrypt::EncryptOutput,
+        super::super::super::operation::encrypt::EncryptError,
+    > for EncryptFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::encrypt::EncryptOutput, super::super::super::operation::encrypt::EncryptError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::encrypt::EncryptOutput,
+            super::super::super::operation::encrypt::EncryptError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -140,8 +146,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::encrypt::EncryptOutput, super::super::super::operation::encrypt::EncryptError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::encrypt::EncryptOutput,
+        super::super::super::operation::encrypt::EncryptError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/encrypt.rs`

```diff
--- reference/src/operation/encrypt.rs
+++ generated/src/operation/encrypt.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::encrypt::EncryptError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::encrypt::EncryptError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
```

### `src/operation/generate_data_key/_generate_data_key_input.rs`

```diff
--- reference/src/operation/generate_data_key/_generate_data_key_input.rs
+++ generated/src/operation/generate_data_key/_generate_data_key_input.rs
@@ -311,7 +311,10 @@
     /// Consumes the builder and constructs a [`GenerateDataKeyInput`](crate::operation::generate_data_key::GenerateDataKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::generate_data_key::GenerateDataKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::generate_data_key::GenerateDataKeyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::generate_data_key::GenerateDataKeyInput {
             key_id: self.key_id,
             encryption_context: self.encryption_context,
```

### `src/operation/generate_data_key.rs`

```diff
--- reference/src/operation/generate_data_key.rs
+++ generated/src/operation/generate_data_key.rs
@@ -256,7 +256,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_generate_data_key::ser_generate_data_key_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_generate_data_key::ser_generate_data_key_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/generate_data_key_pair/_generate_data_key_pair_input.rs`

```diff
--- reference/src/operation/generate_data_key_pair/_generate_data_key_pair_input.rs
+++ generated/src/operation/generate_data_key_pair/_generate_data_key_pair_input.rs
@@ -286,8 +286,10 @@
     /// Consumes the builder and constructs a [`GenerateDataKeyPairInput`](crate::operation::generate_data_key_pair::GenerateDataKeyPairInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::generate_data_key_pair::GenerateDataKeyPairInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::generate_data_key_pair::GenerateDataKeyPairInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::generate_data_key_pair::GenerateDataKeyPairInput {
             encryption_context: self.encryption_context,
             key_id: self.key_id,
```

### `src/operation/generate_data_key_pair.rs`

```diff
--- reference/src/operation/generate_data_key_pair.rs
+++ generated/src/operation/generate_data_key_pair.rs
@@ -259,9 +259,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_generate_data_key_pair::ser_generate_data_key_pair_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_generate_data_key_pair::ser_generate_data_key_pair_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/generate_data_key_pair_without_plaintext/builders.rs`

```diff
--- reference/src/operation/generate_data_key_pair_without_plaintext/builders.rs
+++ generated/src/operation/generate_data_key_pair_without_plaintext/builders.rs
@@ -79,7 +79,9 @@
         }
     }
     /// Access the GenerateDataKeyPairWithoutPlaintext as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -109,7 +111,11 @@
                 &self.handle.conf,
                 self.config_override,
             );
-        super::super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintext::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintext::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/generate_data_key_without_plaintext/builders.rs`

```diff
--- reference/src/operation/generate_data_key_without_plaintext/builders.rs
+++ generated/src/operation/generate_data_key_without_plaintext/builders.rs
@@ -83,7 +83,9 @@
         }
     }
     /// Access the GenerateDataKeyWithoutPlaintext as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -107,12 +109,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintext::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintext::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintext::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintext::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/generate_data_key_without_plaintext.rs`

```diff
--- reference/src/operation/generate_data_key_without_plaintext.rs
+++ generated/src/operation/generate_data_key_without_plaintext.rs
@@ -213,7 +213,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_generate_data_key_without_plaintext::de_generate_data_key_without_plaintext_http_error(status, headers, body)
+            super::super::protocol_serde::shape_generate_data_key_without_plaintext::de_generate_data_key_without_plaintext_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_generate_data_key_without_plaintext::de_generate_data_key_without_plaintext_http_response(
                 status, headers, body,
```

### `src/operation/generate_mac/_generate_mac_input.rs`

```diff
--- reference/src/operation/generate_mac/_generate_mac_input.rs
+++ generated/src/operation/generate_mac/_generate_mac_input.rs
@@ -172,7 +172,9 @@
         &self.dry_run
     }
     /// Consumes the builder and constructs a [`GenerateMacInput`](crate::operation::generate_mac::GenerateMacInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::generate_mac::GenerateMacInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::generate_mac::GenerateMacInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::generate_mac::GenerateMacInput {
             message: self.message,
             key_id: self.key_id,
```

### `src/operation/get_key_last_usage/_get_key_last_usage_input.rs`

```diff
--- reference/src/operation/get_key_last_usage/_get_key_last_usage_input.rs
+++ generated/src/operation/get_key_last_usage/_get_key_last_usage_input.rs
@@ -89,7 +89,10 @@
     /// Consumes the builder and constructs a [`GetKeyLastUsageInput`](crate::operation::get_key_last_usage::GetKeyLastUsageInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_key_last_usage::GetKeyLastUsageInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_key_last_usage::GetKeyLastUsageInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_key_last_usage::GetKeyLastUsageInput { key_id: self.key_id })
     }
 }
```

### `src/operation/get_key_last_usage.rs`

```diff
--- reference/src/operation/get_key_last_usage.rs
+++ generated/src/operation/get_key_last_usage.rs
@@ -255,7 +255,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_key_last_usage::ser_get_key_last_usage_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_key_last_usage::ser_get_key_last_usage_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_key_rotation_status/_get_key_rotation_status_input.rs`

```diff
--- reference/src/operation/get_key_rotation_status/_get_key_rotation_status_input.rs
+++ generated/src/operation/get_key_rotation_status/_get_key_rotation_status_input.rs
@@ -89,8 +89,10 @@
     /// Consumes the builder and constructs a [`GetKeyRotationStatusInput`](crate::operation::get_key_rotation_status::GetKeyRotationStatusInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_key_rotation_status::GetKeyRotationStatusInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_key_rotation_status::GetKeyRotationStatusInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_key_rotation_status::GetKeyRotationStatusInput { key_id: self.key_id })
     }
 }
```

### `src/operation/get_key_rotation_status.rs`

```diff
--- reference/src/operation/get_key_rotation_status.rs
+++ generated/src/operation/get_key_rotation_status.rs
@@ -258,9 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_key_rotation_status::ser_get_key_rotation_status_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_get_key_rotation_status::ser_get_key_rotation_status_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/get_public_key/_get_public_key_output.rs`

```diff
--- reference/src/operation/get_public_key/_get_public_key_output.rs
+++ generated/src/operation/get_public_key/_get_public_key_output.rs
@@ -205,7 +205,10 @@
     /// <p>The encryption algorithms that KMS supports for this key.</p>
     /// <p>This information is critical. If a public key encrypts data outside of KMS by using an unsupported encryption algorithm, the ciphertext cannot be decrypted.</p>
     /// <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>ENCRYPT_DECRYPT</code>.</p>
-    pub fn set_encryption_algorithms(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::EncryptionAlgorithmSpec>>) -> Self {
+    pub fn set_encryption_algorithms(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::EncryptionAlgorithmSpec>>,
+    ) -> Self {
         self.encryption_algorithms = input;
         self
     }
@@ -250,7 +253,10 @@
         self
     }
     /// <p>The key agreement algorithm used to derive a shared secret. This field is present only when the KMS key has a <code>KeyUsage</code> value of <code>KEY_AGREEMENT</code>.</p>
-    pub fn set_key_agreement_algorithms(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::KeyAgreementAlgorithmSpec>>) -> Self {
+    pub fn set_key_agreement_algorithms(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::KeyAgreementAlgorithmSpec>>,
+    ) -> Self {
         self.key_agreement_algorithms = input;
         self
     }
```

### `src/operation/import_key_material/_import_key_material_input.rs`

```diff
--- reference/src/operation/import_key_material/_import_key_material_input.rs
+++ generated/src/operation/import_key_material/_import_key_material_input.rs
@@ -318,7 +318,10 @@
     /// Consumes the builder and constructs a [`ImportKeyMaterialInput`](crate::operation::import_key_material::ImportKeyMaterialInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::import_key_material::ImportKeyMaterialInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::import_key_material::ImportKeyMaterialInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::import_key_material::ImportKeyMaterialInput {
             key_id: self.key_id,
             import_token: self.import_token,
```

### `src/operation/import_key_material.rs`

```diff
--- reference/src/operation/import_key_material.rs
+++ generated/src/operation/import_key_material.rs
@@ -265,7 +265,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_import_key_material::ser_import_key_material_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_import_key_material::ser_import_key_material_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_aliases/_list_aliases_input.rs`

```diff
--- reference/src/operation/list_aliases/_list_aliases_input.rs
+++ generated/src/operation/list_aliases/_list_aliases_input.rs
@@ -138,7 +138,9 @@
         &self.marker
     }
     /// Consumes the builder and constructs a [`ListAliasesInput`](crate::operation::list_aliases::ListAliasesInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_aliases::ListAliasesInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_aliases::ListAliasesInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_aliases::ListAliasesInput {
             key_id: self.key_id,
             limit: self.limit,
```

### `src/operation/list_aliases/paginator.rs`

```diff
--- reference/src/operation/list_aliases/paginator.rs
+++ generated/src/operation/list_aliases/paginator.rs
@@ -139,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_aliases_output_output_aliases(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_aliases_output_output_aliases(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_grants/_list_grants_input.rs`

```diff
--- reference/src/operation/list_grants/_list_grants_input.rs
+++ generated/src/operation/list_grants/_list_grants_input.rs
@@ -207,7 +207,9 @@
         &self.grantee_service_principal
     }
     /// Consumes the builder and constructs a [`ListGrantsInput`](crate::operation::list_grants::ListGrantsInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_grants::ListGrantsInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_grants::ListGrantsInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_grants::ListGrantsInput {
             limit: self.limit,
             marker: self.marker,
```

### `src/operation/list_grants/paginator.rs`

```diff
--- reference/src/operation/list_grants/paginator.rs
+++ generated/src/operation/list_grants/paginator.rs
@@ -139,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_grants_output_output_grants(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_grants_output_output_grants(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_key_policies/_list_key_policies_input.rs`

```diff
--- reference/src/operation/list_key_policies/_list_key_policies_input.rs
+++ generated/src/operation/list_key_policies/_list_key_policies_input.rs
@@ -141,7 +141,10 @@
     /// Consumes the builder and constructs a [`ListKeyPoliciesInput`](crate::operation::list_key_policies::ListKeyPoliciesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_key_policies::ListKeyPoliciesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_key_policies::ListKeyPoliciesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_key_policies::ListKeyPoliciesInput {
             key_id: self.key_id,
             limit: self.limit,
```

### `src/operation/list_key_policies.rs`

```diff
--- reference/src/operation/list_key_policies.rs
+++ generated/src/operation/list_key_policies.rs
@@ -260,7 +260,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_key_policies::ser_list_key_policies_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_key_policies::ser_list_key_policies_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_key_rotations/_list_key_rotations_input.rs`

```diff
--- reference/src/operation/list_key_rotations/_list_key_rotations_input.rs
+++ generated/src/operation/list_key_rotations/_list_key_rotations_input.rs
@@ -157,7 +157,10 @@
     /// Consumes the builder and constructs a [`ListKeyRotationsInput`](crate::operation::list_key_rotations::ListKeyRotationsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_key_rotations::ListKeyRotationsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_key_rotations::ListKeyRotationsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_key_rotations::ListKeyRotationsInput {
             key_id: self.key_id,
             include_key_material: self.include_key_material,
```

### `src/operation/list_key_rotations.rs`

```diff
--- reference/src/operation/list_key_rotations.rs
+++ generated/src/operation/list_key_rotations.rs
@@ -260,7 +260,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_key_rotations::ser_list_key_rotations_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_key_rotations::ser_list_key_rotations_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_keys/_list_keys_input.rs`

```diff
--- reference/src/operation/list_keys/_list_keys_input.rs
+++ generated/src/operation/list_keys/_list_keys_input.rs
@@ -67,7 +67,9 @@
         &self.marker
     }
     /// Consumes the builder and constructs a [`ListKeysInput`](crate::operation::list_keys::ListKeysInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_keys::ListKeysInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_keys::ListKeysInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_keys::ListKeysInput {
             limit: self.limit,
             marker: self.marker,
```

### `src/operation/list_keys/builders.rs`

```diff
--- reference/src/operation/list_keys/builders.rs
+++ generated/src/operation/list_keys/builders.rs
@@ -43,14 +43,20 @@
     inner: super::super::super::operation::list_keys::builders::ListKeysInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::list_keys::ListKeysOutput, super::super::super::operation::list_keys::ListKeysError>
-    for ListKeysFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::list_keys::ListKeysOutput,
+        super::super::super::operation::list_keys::ListKeysError,
+    > for ListKeysFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::list_keys::ListKeysOutput, super::super::super::operation::list_keys::ListKeysError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::list_keys::ListKeysOutput,
+            super::super::super::operation::list_keys::ListKeysError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -100,8 +106,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::list_keys::ListKeysOutput, super::super::super::operation::list_keys::ListKeysError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::list_keys::ListKeysOutput,
+        super::super::super::operation::list_keys::ListKeysError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/list_keys/paginator.rs`

```diff
--- reference/src/operation/list_keys/paginator.rs
+++ generated/src/operation/list_keys/paginator.rs
@@ -8,7 +8,10 @@

 impl ListKeysPaginator {
     /// Create a new paginator-wrapper
-    pub(crate) fn new(handle: std::sync::Arc<super::super::super::client::Handle>, builder: super::super::super::operation::list_keys::builders::ListKeysInputBuilder) -> Self {
+    pub(crate) fn new(
+        handle: std::sync::Arc<super::super::super::client::Handle>,
+        builder: super::super::super::operation::list_keys::builders::ListKeysInputBuilder,
+    ) -> Self {
         Self {
             handle,
             builder,
@@ -136,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_list_keys_output_output_keys(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_list_keys_output_output_keys(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/list_keys.rs`

```diff
--- reference/src/operation/list_keys.rs
+++ generated/src/operation/list_keys.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::list_keys::ListKeysError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::list_keys::ListKeysError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
@@ -216,7 +220,9 @@
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<super::super::operation::list_keys::ListKeysInput>().expect("correct type");
+        let input = input
+            .downcast::<super::super::operation::list_keys::ListKeysInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<super::super::serialization_settings::HeaderSerializationSettings>()
             .cloned()
```

### `src/operation/list_resource_tags/_list_resource_tags_input.rs`

```diff
--- reference/src/operation/list_resource_tags/_list_resource_tags_input.rs
+++ generated/src/operation/list_resource_tags/_list_resource_tags_input.rs
@@ -141,7 +141,10 @@
     /// Consumes the builder and constructs a [`ListResourceTagsInput`](crate::operation::list_resource_tags::ListResourceTagsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_resource_tags::ListResourceTagsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_resource_tags::ListResourceTagsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_resource_tags::ListResourceTagsInput {
             key_id: self.key_id,
             limit: self.limit,
```

### `src/operation/list_resource_tags.rs`

```diff
--- reference/src/operation/list_resource_tags.rs
+++ generated/src/operation/list_resource_tags.rs
@@ -260,7 +260,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_resource_tags::ser_list_resource_tags_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_resource_tags::ser_list_resource_tags_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_retirable_grants/_list_retirable_grants_input.rs`

```diff
--- reference/src/operation/list_retirable_grants/_list_retirable_grants_input.rs
+++ generated/src/operation/list_retirable_grants/_list_retirable_grants_input.rs
@@ -126,8 +126,10 @@
     /// Consumes the builder and constructs a [`ListRetirableGrantsInput`](crate::operation::list_retirable_grants::ListRetirableGrantsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_retirable_grants::ListRetirableGrantsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_retirable_grants::ListRetirableGrantsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_retirable_grants::ListRetirableGrantsInput {
             limit: self.limit,
             marker: self.marker,
```

### `src/operation/list_retirable_grants.rs`

```diff
--- reference/src/operation/list_retirable_grants.rs
+++ generated/src/operation/list_retirable_grants.rs
@@ -268,9 +268,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_retirable_grants::ser_list_retirable_grants_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_list_retirable_grants::ser_list_retirable_grants_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/put_key_policy/_put_key_policy_input.rs`

```diff
--- reference/src/operation/put_key_policy/_put_key_policy_input.rs
+++ generated/src/operation/put_key_policy/_put_key_policy_input.rs
@@ -283,12 +283,13 @@
     /// Consumes the builder and constructs a [`PutKeyPolicyInput`](crate::operation::put_key_policy::PutKeyPolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::put_key_policy::PutKeyPolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::put_key_policy::PutKeyPolicyInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::put_key_policy::PutKeyPolicyInput {
             key_id: self.key_id,
             policy_name: self.policy_name,
             policy: self.policy,
-            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
+            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/re_encrypt/_re_encrypt_input.rs`

```diff
--- reference/src/operation/re_encrypt/_re_encrypt_input.rs
+++ generated/src/operation/re_encrypt/_re_encrypt_input.rs
@@ -492,7 +492,9 @@
         &self.dry_run_modifiers
     }
     /// Consumes the builder and constructs a [`ReEncryptInput`](crate::operation::re_encrypt::ReEncryptInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::re_encrypt::ReEncryptInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::re_encrypt::ReEncryptInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::re_encrypt::ReEncryptInput {
             ciphertext_blob: self.ciphertext_blob,
             source_encryption_context: self.source_encryption_context,
```

### `src/operation/re_encrypt/builders.rs`

```diff
--- reference/src/operation/re_encrypt/builders.rs
+++ generated/src/operation/re_encrypt/builders.rs
@@ -66,14 +66,20 @@
     inner: super::super::super::operation::re_encrypt::builders::ReEncryptInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::re_encrypt::ReEncryptOutput, super::super::super::operation::re_encrypt::ReEncryptError>
-    for ReEncryptFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::re_encrypt::ReEncryptOutput,
+        super::super::super::operation::re_encrypt::ReEncryptError,
+    > for ReEncryptFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::re_encrypt::ReEncryptOutput, super::super::super::operation::re_encrypt::ReEncryptError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::re_encrypt::ReEncryptOutput,
+            super::super::super::operation::re_encrypt::ReEncryptError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/operation/replicate_key/_replicate_key_input.rs`

```diff
--- reference/src/operation/replicate_key/_replicate_key_input.rs
+++ generated/src/operation/replicate_key/_replicate_key_input.rs
@@ -378,12 +378,13 @@
     /// Consumes the builder and constructs a [`ReplicateKeyInput`](crate::operation::replicate_key::ReplicateKeyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::replicate_key::ReplicateKeyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::replicate_key::ReplicateKeyInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::replicate_key::ReplicateKeyInput {
             key_id: self.key_id,
             replica_region: self.replica_region,
             policy: self.policy,
-            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
+            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check.unwrap_or_default(),
             description: self.description,
             tags: self.tags,
         })
```

### `src/operation/retire_grant/_retire_grant_input.rs`

```diff
--- reference/src/operation/retire_grant/_retire_grant_input.rs
+++ generated/src/operation/retire_grant/_retire_grant_input.rs
@@ -139,7 +139,9 @@
         &self.dry_run
     }
     /// Consumes the builder and constructs a [`RetireGrantInput`](crate::operation::retire_grant::RetireGrantInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::retire_grant::RetireGrantInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::retire_grant::RetireGrantInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::retire_grant::RetireGrantInput {
             grant_token: self.grant_token,
             key_id: self.key_id,
```

### `src/operation/revoke_grant/_revoke_grant_input.rs`

```diff
--- reference/src/operation/revoke_grant/_revoke_grant_input.rs
+++ generated/src/operation/revoke_grant/_revoke_grant_input.rs
@@ -135,7 +135,9 @@
         &self.dry_run
     }
     /// Consumes the builder and constructs a [`RevokeGrantInput`](crate::operation::revoke_grant::RevokeGrantInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::revoke_grant::RevokeGrantInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::revoke_grant::RevokeGrantInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::revoke_grant::RevokeGrantInput {
             key_id: self.key_id,
             grant_id: self.grant_id,
```

### `src/operation/rotate_key_on_demand/_rotate_key_on_demand_input.rs`

```diff
--- reference/src/operation/rotate_key_on_demand/_rotate_key_on_demand_input.rs
+++ generated/src/operation/rotate_key_on_demand/_rotate_key_on_demand_input.rs
@@ -89,7 +89,10 @@
     /// Consumes the builder and constructs a [`RotateKeyOnDemandInput`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::rotate_key_on_demand::RotateKeyOnDemandInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::rotate_key_on_demand::RotateKeyOnDemandInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::rotate_key_on_demand::RotateKeyOnDemandInput { key_id: self.key_id })
     }
 }
```

### `src/operation/rotate_key_on_demand.rs`

```diff
--- reference/src/operation/rotate_key_on_demand.rs
+++ generated/src/operation/rotate_key_on_demand.rs
@@ -255,8 +255,9 @@
             );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_rotate_key_on_demand::ser_rotate_key_on_demand_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_rotate_key_on_demand::ser_rotate_key_on_demand_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/schedule_key_deletion/_schedule_key_deletion_input.rs`

```diff
--- reference/src/operation/schedule_key_deletion/_schedule_key_deletion_input.rs
+++ generated/src/operation/schedule_key_deletion/_schedule_key_deletion_input.rs
@@ -120,8 +120,10 @@
     /// Consumes the builder and constructs a [`ScheduleKeyDeletionInput`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::schedule_key_deletion::ScheduleKeyDeletionInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::schedule_key_deletion::ScheduleKeyDeletionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::schedule_key_deletion::ScheduleKeyDeletionInput {
             key_id: self.key_id,
             pending_window_in_days: self.pending_window_in_days,
```

### `src/operation/schedule_key_deletion.rs`

```diff
--- reference/src/operation/schedule_key_deletion.rs
+++ generated/src/operation/schedule_key_deletion.rs
@@ -258,9 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_schedule_key_deletion::ser_schedule_key_deletion_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_schedule_key_deletion::ser_schedule_key_deletion_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/sign/builders.rs`

```diff
--- reference/src/operation/sign/builders.rs
+++ generated/src/operation/sign/builders.rs
@@ -49,14 +49,20 @@
     inner: super::super::super::operation::sign::builders::SignInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::sign::SignOutput, super::super::super::operation::sign::SignError>
-    for SignFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::sign::SignOutput,
+        super::super::super::operation::sign::SignError,
+    > for SignFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::sign::SignOutput, super::super::super::operation::sign::SignError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::sign::SignOutput,
+            super::super::super::operation::sign::SignError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -95,8 +101,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::sign::Sign::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
+        let runtime_plugins = super::super::super::operation::sign::Sign::operation_runtime_plugins(
+            self.handle.runtime_plugins.clone(),
+            &self.handle.conf,
+            self.config_override,
+        );
         super::super::super::operation::sign::Sign::orchestrate(&runtime_plugins, input).await
     }

@@ -103,7 +112,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::sign::SignOutput, super::super::super::operation::sign::SignError, Self> {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::sign::SignOutput,
+        super::super::super::operation::sign::SignError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/sign.rs`

```diff
--- reference/src/operation/sign.rs
+++ generated/src/operation/sign.rs
@@ -18,15 +18,20 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
-            ::aws_smithy_runtime_api::client::interceptors::context::Error,
-            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-        >| { err.map_service_error(|err| err.downcast::<super::super::operation::sign::SignError>().expect("correct error type")) };
+        let map_err =
+            |err: ::aws_smithy_runtime_api::client::result::SdkError<
+                ::aws_smithy_runtime_api::client::interceptors::context::Error,
+                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+            >| { err.map_service_error(|err| err.downcast::<super::super::operation::sign::SignError>().expect("correct error type")) };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(output.downcast::<super::super::operation::sign::SignOutput>().expect("correct output type"))
+        ::std::result::Result::Ok(
+            output
+                .downcast::<super::super::operation::sign::SignOutput>()
+                .expect("correct output type"),
+        )
     }

     pub(crate) async fn orchestrate_with_stop_point(
```

### `src/operation/tag_resource/_tag_resource_input.rs`

```diff
--- reference/src/operation/tag_resource/_tag_resource_input.rs
+++ generated/src/operation/tag_resource/_tag_resource_input.rs
@@ -131,7 +131,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`TagResourceInput`](crate::operation::tag_resource::TagResourceInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::tag_resource::TagResourceInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::tag_resource::TagResourceInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::tag_resource::TagResourceInput {
             key_id: self.key_id,
             tags: self.tags,
```

### `src/operation/update_alias/_update_alias_input.rs`

```diff
--- reference/src/operation/update_alias/_update_alias_input.rs
+++ generated/src/operation/update_alias/_update_alias_input.rs
@@ -129,7 +129,9 @@
         &self.target_key_id
     }
     /// Consumes the builder and constructs a [`UpdateAliasInput`](crate::operation::update_alias::UpdateAliasInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::update_alias::UpdateAliasInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::update_alias::UpdateAliasInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::update_alias::UpdateAliasInput {
             alias_name: self.alias_name,
             target_key_id: self.target_key_id,
```

### `src/operation/update_custom_key_store/_update_custom_key_store_input.rs`

```diff
--- reference/src/operation/update_custom_key_store/_update_custom_key_store_input.rs
+++ generated/src/operation/update_custom_key_store/_update_custom_key_store_input.rs
@@ -333,7 +333,9 @@
     /// <p>You must specify both the <code>AccessKeyId</code> and <code>SecretAccessKey</code> value in the authentication credential, even if you are only updating one value.</p>
     /// <p>This parameter doesn't establish or change your authentication credentials on the proxy. It just tells KMS the credential that you established with your external key store proxy. For example, if you rotate the credential on your external key store proxy, you can use this parameter to update the credential in KMS.</p>
     /// <p>You can change this value when the external key store is connected or disconnected.</p>
-    pub fn get_xks_proxy_authentication_credential(&self) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
+    pub fn get_xks_proxy_authentication_credential(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
         &self.xks_proxy_authentication_credential
     }
     /// <p>Changes the connectivity setting for the external key store. To indicate that the external key store proxy uses a Amazon VPC endpoint service to communicate with KMS, specify <code>VPC_ENDPOINT_SERVICE</code>. Otherwise, specify <code>PUBLIC_ENDPOINT</code>.</p>
@@ -362,8 +364,10 @@
     /// Consumes the builder and constructs a [`UpdateCustomKeyStoreInput`](crate::operation::update_custom_key_store::UpdateCustomKeyStoreInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_custom_key_store::UpdateCustomKeyStoreInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_custom_key_store::UpdateCustomKeyStoreInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_custom_key_store::UpdateCustomKeyStoreInput {
             custom_key_store_id: self.custom_key_store_id,
             new_custom_key_store_name: self.new_custom_key_store_name,
```

### `src/operation/update_custom_key_store/builders.rs`

```diff
--- reference/src/operation/update_custom_key_store/builders.rs
+++ generated/src/operation/update_custom_key_store/builders.rs
@@ -321,7 +321,9 @@
     /// <p>You must specify both the <code>AccessKeyId</code> and <code>SecretAccessKey</code> value in the authentication credential, even if you are only updating one value.</p>
     /// <p>This parameter doesn't establish or change your authentication credentials on the proxy. It just tells KMS the credential that you established with your external key store proxy. For example, if you rotate the credential on your external key store proxy, you can use this parameter to update the credential in KMS.</p>
     /// <p>You can change this value when the external key store is connected or disconnected.</p>
-    pub fn get_xks_proxy_authentication_credential(&self) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
+    pub fn get_xks_proxy_authentication_credential(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::XksProxyAuthenticationCredentialType> {
         self.inner.get_xks_proxy_authentication_credential()
     }
     /// <p>Changes the connectivity setting for the external key store. To indicate that the external key store proxy uses a Amazon VPC endpoint service to communicate with KMS, specify <code>VPC_ENDPOINT_SERVICE</code>. Otherwise, specify <code>PUBLIC_ENDPOINT</code>.</p>
```

### `src/operation/update_custom_key_store.rs`

```diff
--- reference/src/operation/update_custom_key_store.rs
+++ generated/src/operation/update_custom_key_store.rs
@@ -288,9 +288,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_custom_key_store::ser_update_custom_key_store_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_update_custom_key_store::ser_update_custom_key_store_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/update_key_description/_update_key_description_input.rs`

```diff
--- reference/src/operation/update_key_description/_update_key_description_input.rs
+++ generated/src/operation/update_key_description/_update_key_description_input.rs
@@ -121,8 +121,10 @@
     /// Consumes the builder and constructs a [`UpdateKeyDescriptionInput`](crate::operation::update_key_description::UpdateKeyDescriptionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_key_description::UpdateKeyDescriptionInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_key_description::UpdateKeyDescriptionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_key_description::UpdateKeyDescriptionInput {
             key_id: self.key_id,
             description: self.description,
```

### `src/operation/update_key_description.rs`

```diff
--- reference/src/operation/update_key_description.rs
+++ generated/src/operation/update_key_description.rs
@@ -263,9 +263,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_key_description::ser_update_key_description_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_update_key_description::ser_update_key_description_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/update_primary_region/_update_primary_region_input.rs`

```diff
--- reference/src/operation/update_primary_region/_update_primary_region_input.rs
+++ generated/src/operation/update_primary_region/_update_primary_region_input.rs
@@ -116,8 +116,10 @@
     /// Consumes the builder and constructs a [`UpdatePrimaryRegionInput`](crate::operation::update_primary_region::UpdatePrimaryRegionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_primary_region::UpdatePrimaryRegionInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_primary_region::UpdatePrimaryRegionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_primary_region::UpdatePrimaryRegionInput {
             key_id: self.key_id,
             primary_region: self.primary_region,
```

### `src/operation/update_primary_region.rs`

```diff
--- reference/src/operation/update_primary_region.rs
+++ generated/src/operation/update_primary_region.rs
@@ -263,9 +263,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_primary_region::ser_update_primary_region_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_update_primary_region::ser_update_primary_region_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/verify/_verify_input.rs`

```diff
--- reference/src/operation/verify/_verify_input.rs
+++ generated/src/operation/verify/_verify_input.rs
@@ -412,7 +412,9 @@
         &self.dry_run
     }
     /// Consumes the builder and constructs a [`VerifyInput`](crate::operation::verify::VerifyInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::verify::VerifyInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::verify::VerifyInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::verify::VerifyInput {
             key_id: self.key_id,
             message: self.message,
```

### `src/operation/verify/builders.rs`

```diff
--- reference/src/operation/verify/builders.rs
+++ generated/src/operation/verify/builders.rs
@@ -40,14 +40,20 @@
     inner: super::super::super::operation::verify::builders::VerifyInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::verify::VerifyOutput, super::super::super::operation::verify::VerifyError>
-    for VerifyFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::verify::VerifyOutput,
+        super::super::super::operation::verify::VerifyError,
+    > for VerifyFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::verify::VerifyOutput, super::super::super::operation::verify::VerifyError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::verify::VerifyOutput,
+            super::super::super::operation::verify::VerifyError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -86,8 +92,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::verify::Verify::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
+        let runtime_plugins = super::super::super::operation::verify::Verify::operation_runtime_plugins(
+            self.handle.runtime_plugins.clone(),
+            &self.handle.conf,
+            self.config_override,
+        );
         super::super::super::operation::verify::Verify::orchestrate(&runtime_plugins, input).await
     }

@@ -94,7 +103,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::verify::VerifyOutput, super::super::super::operation::verify::VerifyError, Self> {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::verify::VerifyOutput,
+        super::super::super::operation::verify::VerifyError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
```

### `src/operation/verify.rs`

```diff
--- reference/src/operation/verify.rs
+++ generated/src/operation/verify.rs
@@ -18,16 +18,24 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::verify::VerifyError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::verify::VerifyError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(output.downcast::<super::super::operation::verify::VerifyOutput>().expect("correct output type"))
+        ::std::result::Result::Ok(
+            output
+                .downcast::<super::super::operation::verify::VerifyOutput>()
+                .expect("correct output type"),
+        )
     }

     pub(crate) async fn orchestrate_with_stop_point(
```

### `src/operation/verify_mac/_verify_mac_input.rs`

```diff
--- reference/src/operation/verify_mac/_verify_mac_input.rs
+++ generated/src/operation/verify_mac/_verify_mac_input.rs
@@ -190,7 +190,9 @@
         &self.dry_run
     }
     /// Consumes the builder and constructs a [`VerifyMacInput`](crate::operation::verify_mac::VerifyMacInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::verify_mac::VerifyMacInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::verify_mac::VerifyMacInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::verify_mac::VerifyMacInput {
             message: self.message,
             key_id: self.key_id,
```

### `src/operation/verify_mac/builders.rs`

```diff
--- reference/src/operation/verify_mac/builders.rs
+++ generated/src/operation/verify_mac/builders.rs
@@ -36,14 +36,20 @@
     inner: super::super::super::operation::verify_mac::builders::VerifyMacInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::verify_mac::VerifyMacOutput, super::super::super::operation::verify_mac::VerifyMacError>
-    for VerifyMacFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::verify_mac::VerifyMacOutput,
+        super::super::super::operation::verify_mac::VerifyMacError,
+    > for VerifyMacFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::verify_mac::VerifyMacOutput, super::super::super::operation::verify_mac::VerifyMacError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::verify_mac::VerifyMacOutput,
+            super::super::super::operation::verify_mac::VerifyMacError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
```

### `src/protocol_serde/shape_cancel_key_deletion.rs`

```diff
--- reference/src/protocol_serde/shape_cancel_key_deletion.rs
+++ generated/src/protocol_serde/shape_cancel_key_deletion.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::cancel_key_deletion::CancelKeyDeletionOutput, super::super::operation::cancel_key_deletion::CancelKeyDeletionError>
-{
+) -> std::result::Result<
+    super::super::operation::cancel_key_deletion::CancelKeyDeletionOutput,
+    super::super::operation::cancel_key_deletion::CancelKeyDeletionError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::cancel_key_deletion::CancelKeyDeletionError::unhandled)?;
@@ -23,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::cancel_key_deletion::CancelKeyDeletionError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::cancel_key_deletion::CancelKeyDeletionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +73,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::cancel_key_deletion::CancelKeyDeletionError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::cancel_key_deletion::CancelKeyDeletionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -102,8 +108,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::cancel_key_deletion::CancelKeyDeletionOutput, super::super::operation::cancel_key_deletion::CancelKeyDeletionError>
-{
+) -> std::result::Result<
+    super::super::operation::cancel_key_deletion::CancelKeyDeletionOutput,
+    super::super::operation::cancel_key_deletion::CancelKeyDeletionError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::cancel_key_deletion::builders::CancelKeyDeletionOutputBuilder::default();
```

### `src/protocol_serde/shape_cloud_hsm_cluster_in_use_exception.rs`

```diff
--- reference/src/protocol_serde/shape_cloud_hsm_cluster_in_use_exception.rs
+++ generated/src/protocol_serde/shape_cloud_hsm_cluster_in_use_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_cloud_hsm_cluster_in_use_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::CloudHsmClusterInUseExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::CloudHsmClusterInUseExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::CloudHsmClusterInUseExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_connect_custom_key_store.rs`

```diff
--- reference/src/protocol_serde/shape_connect_custom_key_store.rs
+++ generated/src/protocol_serde/shape_connect_custom_key_store.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -42,11 +46,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CloudHsmClusterNotActiveExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(_response_body, output).map_err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -62,12 +62,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(_response_body, output).map_err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -83,11 +78,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::connect_custom_key_store::ConnectCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -142,3 +138,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_connect_custom_key_store(
+    _value: &[u8],
+    mut builder: super::super::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreOutputBuilder,
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

### `src/protocol_serde/shape_create_alias.rs`

```diff
--- reference/src/protocol_serde/shape_create_alias.rs
+++ generated/src/protocol_serde/shape_create_alias.rs
@@ -37,8 +37,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_alias::CreateAliasError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::create_alias::CreateAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidAliasNameExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_alias_name_exception::de_invalid_alias_name_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_alias::CreateAliasError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_alias_name_exception::de_invalid_alias_name_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::create_alias::CreateAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +88,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_alias::CreateAliasError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_alias::CreateAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -149,3 +156,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_create_alias(
+    _value: &[u8],
+    mut builder: super::super::operation::create_alias::builders::CreateAliasOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::create_alias::builders::CreateAliasOutputBuilder,
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

### `src/protocol_serde/shape_create_custom_key_store.rs`

```diff
--- reference/src/protocol_serde/shape_create_custom_key_store.rs
+++ generated/src/protocol_serde/shape_create_custom_key_store.rs
@@ -15,29 +15,35 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "CloudHsmClusterInUseException" => super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::CloudHsmClusterInUseException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "CloudHsmClusterInUseException" => {
+            super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::CloudHsmClusterInUseException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::CloudHsmClusterInUseExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_cloud_hsm_cluster_in_use_exception::de_cloud_hsm_cluster_in_use_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::CloudHsmClusterInUseExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_in_use_exception::de_cloud_hsm_cluster_in_use_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "CloudHsmClusterInvalidConfigurationException" => {
             super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::CloudHsmClusterInvalidConfigurationException({
                 #[allow(unused_mut)]
@@ -60,11 +66,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CloudHsmClusterNotActiveExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(_response_body, output).map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -80,11 +82,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CloudHsmClusterNotFoundExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_found_exception::de_cloud_hsm_cluster_not_found_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_cloud_hsm_cluster_not_found_exception::de_cloud_hsm_cluster_not_found_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -100,7 +103,23 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreNameInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_custom_key_store_name_in_use_exception::de_custom_key_store_name_in_use_exception_json_err(
+                    output = super::super::protocol_serde::shape_custom_key_store_name_in_use_exception::de_custom_key_store_name_in_use_exception_json_err(_response_body, output).map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "IncorrectTrustAnchorException" => {
+            super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::IncorrectTrustAnchorException({
+                #[allow(unused_mut)]
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::IncorrectTrustAnchorExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_incorrect_trust_anchor_exception::de_incorrect_trust_anchor_exception_json_err(
                         _response_body,
                         output,
                     )
@@ -114,24 +133,6 @@
                 tmp
             })
         }
-        "IncorrectTrustAnchorException" => super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::IncorrectTrustAnchorException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::IncorrectTrustAnchorExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_incorrect_trust_anchor_exception::de_incorrect_trust_anchor_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
         "KMSInternalException" => super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::KmsInternalException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -200,11 +201,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::XksProxyInvalidResponseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_xks_proxy_invalid_response_exception::de_xks_proxy_invalid_response_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_xks_proxy_invalid_response_exception::de_xks_proxy_invalid_response_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -220,12 +222,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::XksProxyUriEndpointInUseExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_xks_proxy_uri_endpoint_in_use_exception::de_xks_proxy_uri_endpoint_in_use_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_xks_proxy_uri_endpoint_in_use_exception::de_xks_proxy_uri_endpoint_in_use_exception_json_err(_response_body, output).map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -240,23 +237,7 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::XksProxyUriInUseExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_xks_proxy_uri_in_use_exception::de_xks_proxy_uri_in_use_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "XksProxyUriUnreachableException" => super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::XksProxyUriUnreachableException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::XksProxyUriUnreachableExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_xks_proxy_uri_unreachable_exception::de_xks_proxy_uri_unreachable_exception_json_err(
+                output = super::super::protocol_serde::shape_xks_proxy_uri_in_use_exception::de_xks_proxy_uri_in_use_exception_json_err(
                     _response_body,
                     output,
                 )
@@ -269,6 +250,27 @@
             }
             tmp
         }),
+        "XksProxyUriUnreachableException" => {
+            super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::XksProxyUriUnreachableException({
+                #[allow(unused_mut)]
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::XksProxyUriUnreachableExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_xks_proxy_uri_unreachable_exception::de_xks_proxy_uri_unreachable_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "XksProxyVpcEndpointServiceInUseException" => {
             super::super::operation::create_custom_key_store::CreateCustomKeyStoreError::XksProxyVpcEndpointServiceInUseException({
                 #[allow(unused_mut)]
```

### `src/protocol_serde/shape_create_custom_key_store_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_custom_key_store_input.rs
+++ generated/src/protocol_serde/shape_create_custom_key_store_input.rs
@@ -33,7 +33,10 @@
     if let Some(var_10) = &input.xks_proxy_authentication_credential {
         #[allow(unused_mut)]
         let mut object_11 = object.key("XksProxyAuthenticationCredential").start_object();
-        super::super::protocol_serde::shape_xks_proxy_authentication_credential_type::ser_xks_proxy_authentication_credential_type(&mut object_11, var_10)?;
+        super::super::protocol_serde::shape_xks_proxy_authentication_credential_type::ser_xks_proxy_authentication_credential_type(
+            &mut object_11,
+            var_10,
+        )?;
         object_11.finish();
     }
     if let Some(var_12) = &input.xks_proxy_connectivity {
```

### `src/protocol_serde/shape_create_grant.rs`

```diff
--- reference/src/protocol_serde/shape_create_grant.rs
+++ generated/src/protocol_serde/shape_create_grant.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,9 +86,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +119,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_grant::CreateGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -186,8 +193,10 @@
 pub(crate) fn de_create_grant(
     _value: &[u8],
     mut builder: super::super::operation::create_grant::builders::CreateGrantOutputBuilder,
-) -> ::std::result::Result<super::super::operation::create_grant::builders::CreateGrantOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::create_grant::builders::CreateGrantOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_create_key.rs`

```diff
--- reference/src/protocol_serde/shape_create_key.rs
+++ generated/src/protocol_serde/shape_create_key.rs
@@ -38,11 +38,7 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(_response_body, output).map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -74,8 +70,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -167,23 +166,7 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "XksKeyAlreadyInUseException" => super::super::operation::create_key::CreateKeyError::XksKeyAlreadyInUseException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::XksKeyAlreadyInUseExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_xks_key_already_in_use_exception::de_xks_key_already_in_use_exception_json_err(
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
                     _response_body,
                     output,
                 )
@@ -196,12 +179,12 @@
             }
             tmp
         }),
-        "XksKeyInvalidConfigurationException" => super::super::operation::create_key::CreateKeyError::XksKeyInvalidConfigurationException({
+        "XksKeyAlreadyInUseException" => super::super::operation::create_key::CreateKeyError::XksKeyAlreadyInUseException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::XksKeyInvalidConfigurationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_xks_key_invalid_configuration_exception::de_xks_key_invalid_configuration_exception_json_err(
+                let mut output = super::super::types::error::builders::XksKeyAlreadyInUseExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_xks_key_already_in_use_exception::de_xks_key_already_in_use_exception_json_err(
                     _response_body,
                     output,
                 )
@@ -214,13 +197,30 @@
             }
             tmp
         }),
+        "XksKeyInvalidConfigurationException" => {
+            super::super::operation::create_key::CreateKeyError::XksKeyInvalidConfigurationException({
+                #[allow(unused_mut)]
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::XksKeyInvalidConfigurationExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_xks_key_invalid_configuration_exception::de_xks_key_invalid_configuration_exception_json_err(_response_body, output).map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "XksKeyNotFoundException" => super::super::operation::create_key::CreateKeyError::XksKeyNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::XksKeyNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_xks_key_not_found_exception::de_xks_key_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_xks_key_not_found_exception::de_xks_key_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_key::CreateKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -262,7 +262,10 @@
 pub(crate) fn de_create_key(
     _value: &[u8],
     mut builder: super::super::operation::create_key::builders::CreateKeyOutputBuilder,
-) -> ::std::result::Result<super::super::operation::create_key::builders::CreateKeyOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::operation::create_key::builders::CreateKeyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -273,7 +276,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "KeyMetadata" => {
-                    builder = builder.set_key_metadata(super::super::protocol_serde::shape_key_metadata::de_key_metadata(tokens, _value, depth + 1)?);
+                    builder = builder.set_key_metadata(super::super::protocol_serde::shape_key_metadata::de_key_metadata(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_custom_key_stores_list.rs`

```diff
--- reference/src/protocol_serde/shape_custom_key_stores_list.rs
+++ generated/src/protocol_serde/shape_custom_key_stores_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::CustomKeyStoresListEntry>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::CustomKeyStoresListEntry>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -23,8 +26,11 @@
                         break;
                     }
                     _ => {
-                        let value =
-                            super::super::protocol_serde::shape_custom_key_stores_list_entry::de_custom_key_stores_list_entry(tokens, _value, depth + 1)?;
+                        let value = super::super::protocol_serde::shape_custom_key_stores_list_entry::de_custom_key_stores_list_entry(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_decrypt.rs`

```diff
--- reference/src/protocol_serde/shape_decrypt.rs
+++ generated/src/protocol_serde/shape_decrypt.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +86,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidCiphertextExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_ciphertext_exception::de_invalid_ciphertext_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_ciphertext_exception::de_invalid_ciphertext_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -97,9 +104,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +122,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -158,8 +168,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -196,8 +207,8 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::decrypt::builders::DecryptOutputBuilder::default();
-        output =
-            super::super::protocol_serde::shape_decrypt::de_decrypt(_response_body, output).map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
+        output = super::super::protocol_serde::shape_decrypt::de_decrypt(_response_body, output)
+            .map_err(super::super::operation::decrypt::DecryptError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -216,7 +227,8 @@
 pub(crate) fn de_decrypt(
     _value: &[u8],
     mut builder: super::super::operation::decrypt::builders::DecryptOutputBuilder,
-) -> ::std::result::Result<super::super::operation::decrypt::builders::DecryptOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::operation::decrypt::builders::DecryptOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_delete_alias.rs`

```diff
--- reference/src/protocol_serde/shape_delete_alias.rs
+++ generated/src/protocol_serde/shape_delete_alias.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_alias::DeleteAliasError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_alias::DeleteAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_alias::DeleteAliasError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::delete_alias::DeleteAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -104,3 +108,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_alias(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_alias::builders::DeleteAliasOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_alias::builders::DeleteAliasOutputBuilder,
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

### `src/protocol_serde/shape_delete_custom_key_store.rs`

```diff
--- reference/src/protocol_serde/shape_delete_custom_key_store.rs
+++ generated/src/protocol_serde/shape_delete_custom_key_store.rs
@@ -15,29 +15,36 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "CustomKeyStoreHasCMKsException" => super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreHasCmKsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "CustomKeyStoreHasCMKsException" => {
+            super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreHasCmKsException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::CustomKeyStoreHasCmKsExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_custom_key_store_has_cmks_exception::de_custom_key_store_has_cmks_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::CustomKeyStoreHasCmKsExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_custom_key_store_has_cmks_exception::de_custom_key_store_has_cmks_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "CustomKeyStoreInvalidStateException" => {
             super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreInvalidStateException({
                 #[allow(unused_mut)]
@@ -44,8 +51,24 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(_response_body, output).map_err(super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "CustomKeyStoreNotFoundException" => {
+            super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreNotFoundException({
+                #[allow(unused_mut)]
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
                     output =
-                        super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
+                        super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
                             _response_body,
                             output,
                         )
@@ -59,24 +82,6 @@
                 tmp
             })
         }
-        "CustomKeyStoreNotFoundException" => super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
         "KMSInternalException" => super::super::operation::delete_custom_key_store::DeleteCustomKeyStoreError::KmsInternalException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -122,3 +127,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_delete_custom_key_store(
+    _value: &[u8],
+    mut builder: super::super::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreOutputBuilder,
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

### `src/protocol_serde/shape_delete_imported_key_material.rs`

```diff
--- reference/src/protocol_serde/shape_delete_imported_key_material.rs
+++ generated/src/protocol_serde/shape_delete_imported_key_material.rs
@@ -15,30 +15,31 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "DependencyTimeoutException" => super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::DependencyTimeoutException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "DependencyTimeoutException" => {
+            super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::DependencyTimeoutException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "InvalidArnException" => super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::InvalidArnException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -69,21 +70,26 @@
             }
             tmp
         }),
-        "KMSInvalidStateException" => super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::KmsInvalidStateException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "KMSInvalidStateException" => {
+            super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::KmsInvalidStateException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "NotFoundException" => super::super::operation::delete_imported_key_material::DeleteImportedKeyMaterialError::NotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
```

### `src/protocol_serde/shape_dependency_timeout_exception.rs`

```diff
--- reference/src/protocol_serde/shape_dependency_timeout_exception.rs
+++ generated/src/protocol_serde/shape_dependency_timeout_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_dependency_timeout_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::DependencyTimeoutExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::DependencyTimeoutExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::DependencyTimeoutExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_derive_shared_secret.rs`

```diff
--- reference/src/protocol_serde/shape_derive_shared_secret.rs
+++ generated/src/protocol_serde/shape_derive_shared_secret.rs
@@ -25,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -55,8 +58,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -70,9 +74,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,8 +92,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -131,8 +138,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::derive_shared_secret::DeriveSharedSecretError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_describe_custom_key_stores.rs`

```diff
--- reference/src/protocol_serde/shape_describe_custom_key_stores.rs
+++ generated/src/protocol_serde/shape_describe_custom_key_stores.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -30,11 +26,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -122,26 +119,26 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CustomKeyStores" => {
-                    builder = builder.set_custom_key_stores(super::super::protocol_serde::shape_custom_key_stores_list::de_custom_key_stores_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "NextMarker" => {
-                    builder = builder.set_next_marker(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "Truncated" => {
-                    builder = builder.set_truncated(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "CustomKeyStores" => {
+                        builder = builder.set_custom_key_stores(
+                            super::super::protocol_serde::shape_custom_key_stores_list::de_custom_key_stores_list(tokens, _value, depth + 1)?,
+                        );
+                    }
+                    "NextMarker" => {
+                        builder = builder.set_next_marker(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "Truncated" => {
+                        builder = builder.set_truncated(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_describe_key.rs`

```diff
--- reference/src/protocol_serde/shape_describe_key.rs
+++ generated/src/protocol_serde/shape_describe_key.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_key::DescribeKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::describe_key::DescribeKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -110,8 +113,10 @@
 pub(crate) fn de_describe_key(
     _value: &[u8],
     mut builder: super::super::operation::describe_key::builders::DescribeKeyOutputBuilder,
-) -> ::std::result::Result<super::super::operation::describe_key::builders::DescribeKeyOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::describe_key::builders::DescribeKeyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -122,7 +127,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "KeyMetadata" => {
-                    builder = builder.set_key_metadata(super::super::protocol_serde::shape_key_metadata::de_key_metadata(tokens, _value, depth + 1)?);
+                    builder = builder.set_key_metadata(super::super::protocol_serde::shape_key_metadata::de_key_metadata(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_disable_key.rs`

```diff
--- reference/src/protocol_serde/shape_disable_key.rs
+++ generated/src/protocol_serde/shape_disable_key.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::disable_key::DisableKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::disable_key::DisableKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +70,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::disable_key::DisableKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::disable_key::DisableKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,3 +123,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_disable_key(
+    _value: &[u8],
+    mut builder: super::super::operation::disable_key::builders::DisableKeyOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::disable_key::builders::DisableKeyOutputBuilder,
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

### `src/protocol_serde/shape_disable_key_rotation.rs`

```diff
--- reference/src/protocol_serde/shape_disable_key_rotation.rs
+++ generated/src/protocol_serde/shape_disable_key_rotation.rs
@@ -25,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::disable_key_rotation::DisableKeyRotationError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::disable_key_rotation::DisableKeyRotationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -85,8 +88,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::disable_key_rotation::DisableKeyRotationError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::disable_key_rotation::DisableKeyRotationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -115,9 +119,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::disable_key_rotation::DisableKeyRotationError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::disable_key_rotation::DisableKeyRotationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -156,3 +162,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_disable_key_rotation(
+    _value: &[u8],
+    mut builder: super::super::operation::disable_key_rotation::builders::DisableKeyRotationOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::disable_key_rotation::builders::DisableKeyRotationOutputBuilder,
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

### `src/protocol_serde/shape_disconnect_custom_key_store.rs`

```diff
--- reference/src/protocol_serde/shape_disconnect_custom_key_store.rs
+++ generated/src/protocol_serde/shape_disconnect_custom_key_store.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -30,12 +26,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(_response_body, output).map_err(super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -51,11 +42,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -110,3 +102,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_disconnect_custom_key_store(
+    _value: &[u8],
+    mut builder: super::super::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreOutputBuilder,
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

### `src/protocol_serde/shape_dry_run_operation_exception.rs`

```diff
--- reference/src/protocol_serde/shape_dry_run_operation_exception.rs
+++ generated/src/protocol_serde/shape_dry_run_operation_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_dry_run_operation_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::DryRunOperationExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::DryRunOperationExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::DryRunOperationExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_enable_key.rs`

```diff
--- reference/src/protocol_serde/shape_enable_key.rs
+++ generated/src/protocol_serde/shape_enable_key.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::enable_key::EnableKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::enable_key::EnableKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +70,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::enable_key::EnableKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::enable_key::EnableKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -134,3 +138,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_enable_key(
+    _value: &[u8],
+    mut builder: super::super::operation::enable_key::builders::EnableKeyOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::enable_key::builders::EnableKeyOutputBuilder,
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

### `src/protocol_serde/shape_enable_key_rotation.rs`

```diff
--- reference/src/protocol_serde/shape_enable_key_rotation.rs
+++ generated/src/protocol_serde/shape_enable_key_rotation.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::enable_key_rotation::EnableKeyRotationOutput, super::super::operation::enable_key_rotation::EnableKeyRotationError>
-{
+) -> std::result::Result<
+    super::super::operation::enable_key_rotation::EnableKeyRotationOutput,
+    super::super::operation::enable_key_rotation::EnableKeyRotationError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
@@ -23,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +88,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,9 +119,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::enable_key_rotation::EnableKeyRotationError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -133,8 +141,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::enable_key_rotation::EnableKeyRotationOutput, super::super::operation::enable_key_rotation::EnableKeyRotationError>
-{
+) -> std::result::Result<
+    super::super::operation::enable_key_rotation::EnableKeyRotationOutput,
+    super::super::operation::enable_key_rotation::EnableKeyRotationError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::enable_key_rotation::builders::EnableKeyRotationOutputBuilder::default();
@@ -152,3 +162,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_enable_key_rotation(
+    _value: &[u8],
+    mut builder: super::super::operation::enable_key_rotation::builders::EnableKeyRotationOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::enable_key_rotation::builders::EnableKeyRotationOutputBuilder,
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

### `src/protocol_serde/shape_encrypt.rs`

```diff
--- reference/src/protocol_serde/shape_encrypt.rs
+++ generated/src/protocol_serde/shape_encrypt.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,9 +71,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +89,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +135,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -166,8 +174,8 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::encrypt::builders::EncryptOutputBuilder::default();
-        output =
-            super::super::protocol_serde::shape_encrypt::de_encrypt(_response_body, output).map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
+        output = super::super::protocol_serde::shape_encrypt::de_encrypt(_response_body, output)
+            .map_err(super::super::operation::encrypt::EncryptError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -186,7 +194,8 @@
 pub(crate) fn de_encrypt(
     _value: &[u8],
     mut builder: super::super::operation::encrypt::builders::EncryptOutputBuilder,
-) -> ::std::result::Result<super::super::operation::encrypt::builders::EncryptOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::operation::encrypt::builders::EncryptOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_encryption_algorithm_spec_list.rs`

```diff
--- reference/src/protocol_serde/shape_encryption_algorithm_spec_list.rs
+++ generated/src/protocol_serde/shape_encryption_algorithm_spec_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::EncryptionAlgorithmSpec>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::EncryptionAlgorithmSpec>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_expired_import_token_exception.rs`

```diff
--- reference/src/protocol_serde/shape_expired_import_token_exception.rs
+++ generated/src/protocol_serde/shape_expired_import_token_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_expired_import_token_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ExpiredImportTokenExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ExpiredImportTokenExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::ExpiredImportTokenExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_generate_data_key.rs`

```diff
--- reference/src/protocol_serde/shape_generate_data_key.rs
+++ generated/src/protocol_serde/shape_generate_data_key.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::generate_data_key::GenerateDataKeyOutput, super::super::operation::generate_data_key::GenerateDataKeyError> {
+) -> std::result::Result<
+    super::super::operation::generate_data_key::GenerateDataKeyOutput,
+    super::super::operation::generate_data_key::GenerateDataKeyError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
@@ -22,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +58,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,9 +74,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +92,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +138,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -162,7 +173,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::generate_data_key::GenerateDataKeyOutput, super::super::operation::generate_data_key::GenerateDataKeyError> {
+) -> std::result::Result<
+    super::super::operation::generate_data_key::GenerateDataKeyOutput,
+    super::super::operation::generate_data_key::GenerateDataKeyError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::generate_data_key::builders::GenerateDataKeyOutputBuilder::default();
```

### `src/protocol_serde/shape_generate_data_key_pair.rs`

```diff
--- reference/src/protocol_serde/shape_generate_data_key_pair.rs
+++ generated/src/protocol_serde/shape_generate_data_key_pair.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -25,8 +29,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -55,8 +62,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -70,9 +78,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,8 +96,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -131,8 +142,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -156,22 +168,26 @@
             }
             tmp
         }),
-        "UnsupportedOperationException" => super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::UnsupportedOperationException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnsupportedOperationException" => {
+            super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::UnsupportedOperationException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::generate_data_key_pair::GenerateDataKeyPairError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_generate_data_key_pair_without_plaintext.rs`

```diff
--- reference/src/protocol_serde/shape_generate_data_key_pair_without_plaintext.rs
+++ generated/src/protocol_serde/shape_generate_data_key_pair_without_plaintext.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled(generic)),
+        None => {
+            return Err(
+                super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -30,7 +34,9 @@
                         _response_body,
                         output,
                     )
-                    .map_err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled)?;
+                    .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -46,8 +52,9 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::DisabledExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(_response_body, output).map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -63,11 +70,13 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                            .map_err(
-                                super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
-                            )?;
+                    output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -83,11 +92,13 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                            .map_err(
-                                super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
-                            )?;
+                    output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -103,11 +114,13 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                            .map_err(
-                                super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
-                            )?;
+                    output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -123,8 +136,13 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::KeyUnavailableExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_key_unavailable_exception::de_key_unavailable_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_key_unavailable_exception::de_key_unavailable_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -141,7 +159,9 @@
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::KmsInternalExceptionBuilder::default();
                     output = super::super::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled)?;
+                        .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -157,11 +177,13 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                            .map_err(
-                                super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
-                            )?;
+                    output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(
+                        super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                    )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -178,7 +200,9 @@
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::NotFoundExceptionBuilder::default();
                     output = super::super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled)?;
+                        .map_err(
+                            super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                        )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -189,24 +213,28 @@
             })
         }
         "UnsupportedOperationException" => {
-            super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::UnsupportedOperationException({
-                #[allow(unused_mut)]
-                let mut tmp = {
+            super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::UnsupportedOperationException(
+                {
                     #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
+                    let mut tmp = {
+                        #[allow(unused_mut)]
+                        let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
+                        output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::unhandled,
+                        )?;
+                        let output = output.meta(generic);
+                        output.build()
+                    };
+                    if tmp.message.is_none() {
+                        tmp.message = _error_message;
+                    }
+                    tmp
+                },
+            )
         }
         _ => super::super::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError::generic(generic),
     })
```

### `src/protocol_serde/shape_generate_data_key_without_plaintext.rs`

```diff
--- reference/src/protocol_serde/shape_generate_data_key_without_plaintext.rs
+++ generated/src/protocol_serde/shape_generate_data_key_without_plaintext.rs
@@ -26,9 +26,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -38,21 +40,23 @@
                 tmp
             })
         }
-        "DisabledException" => super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::DisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "DisabledException" => {
+            super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::DisabledException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::DisabledExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::DisabledExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "DryRunOperationException" => {
             super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::DryRunOperationException({
                 #[allow(unused_mut)]
@@ -59,9 +63,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -77,9 +83,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -95,9 +103,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -113,8 +123,9 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::KeyUnavailableExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_key_unavailable_exception::de_key_unavailable_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_key_unavailable_exception::de_key_unavailable_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -130,8 +141,9 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::KmsInternalExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -147,9 +159,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -159,21 +173,23 @@
                 tmp
             })
         }
-        "NotFoundException" => super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::NotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "NotFoundException" => {
+            super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::NotFoundException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::NotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::NotFoundExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::generic(generic),
     })
 }
@@ -189,9 +205,11 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = super::super::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextOutputBuilder::default();
-        output = super::super::protocol_serde::shape_generate_data_key_without_plaintext::de_generate_data_key_without_plaintext(_response_body, output)
-            .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
+        let mut output =
+            super::super::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextOutputBuilder::default();
+        output =
+            super::super::protocol_serde::shape_generate_data_key_without_plaintext::de_generate_data_key_without_plaintext(_response_body, output)
+                .map_err(super::super::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -202,7 +220,10 @@
 ) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    super::super::protocol_serde::shape_generate_data_key_without_plaintext_input::ser_generate_data_key_without_plaintext_input_input(&mut object, input)?;
+    super::super::protocol_serde::shape_generate_data_key_without_plaintext_input::ser_generate_data_key_without_plaintext_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
```

### `src/protocol_serde/shape_generate_mac.rs`

```diff
--- reference/src/protocol_serde/shape_generate_mac.rs
+++ generated/src/protocol_serde/shape_generate_mac.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,9 +53,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +71,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +117,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::generate_mac::GenerateMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -171,8 +176,10 @@
 pub(crate) fn de_generate_mac(
     _value: &[u8],
     mut builder: super::super::operation::generate_mac::builders::GenerateMacOutputBuilder,
-) -> ::std::result::Result<super::super::operation::generate_mac::builders::GenerateMacOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::generate_mac::builders::GenerateMacOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_generate_random.rs`

```diff
--- reference/src/protocol_serde/shape_generate_random.rs
+++ generated/src/protocol_serde/shape_generate_random.rs
@@ -4,7 +4,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::generate_random::GenerateRandomOutput, super::super::operation::generate_random::GenerateRandomError> {
+) -> std::result::Result<super::super::operation::generate_random::GenerateRandomOutput, super::super::operation::generate_random::GenerateRandomError>
+{
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
@@ -17,24 +18,22 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "CustomKeyStoreInvalidStateException" => super::super::operation::generate_random::GenerateRandomError::CustomKeyStoreInvalidStateException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "CustomKeyStoreInvalidStateException" => {
+            super::super::operation::generate_random::GenerateRandomError::CustomKeyStoreInvalidStateException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(_response_body, output).map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "CustomKeyStoreNotFoundException" => super::super::operation::generate_random::GenerateRandomError::CustomKeyStoreNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -58,8 +57,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -88,9 +90,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::generate_random::GenerateRandomError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -108,7 +112,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::generate_random::GenerateRandomOutput, super::super::operation::generate_random::GenerateRandomError> {
+) -> std::result::Result<super::super::operation::generate_random::GenerateRandomOutput, super::super::operation::generate_random::GenerateRandomError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::generate_random::builders::GenerateRandomOutputBuilder::default();
```

### `src/protocol_serde/shape_get_key_last_usage.rs`

```diff
--- reference/src/protocol_serde/shape_get_key_last_usage.rs
+++ generated/src/protocol_serde/shape_get_key_last_usage.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_key_last_usage::GetKeyLastUsageOutput, super::super::operation::get_key_last_usage::GetKeyLastUsageError> {
+) -> std::result::Result<
+    super::super::operation::get_key_last_usage::GetKeyLastUsageOutput,
+    super::super::operation::get_key_last_usage::GetKeyLastUsageError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::get_key_last_usage::GetKeyLastUsageError::unhandled)?;
@@ -22,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_key_last_usage::GetKeyLastUsageError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_key_last_usage::GetKeyLastUsageError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,7 +92,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_key_last_usage::GetKeyLastUsageOutput, super::super::operation::get_key_last_usage::GetKeyLastUsageError> {
+) -> std::result::Result<
+    super::super::operation::get_key_last_usage::GetKeyLastUsageOutput,
+    super::super::operation::get_key_last_usage::GetKeyLastUsageError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::get_key_last_usage::builders::GetKeyLastUsageOutputBuilder::default();
```

### `src/protocol_serde/shape_get_key_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_key_policy.rs
+++ generated/src/protocol_serde/shape_get_key_policy.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_key_policy::GetKeyPolicyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_key_policy::GetKeyPolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +70,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_key_policy::GetKeyPolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_key_policy::GetKeyPolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_get_key_rotation_status.rs`

```diff
--- reference/src/protocol_serde/shape_get_key_rotation_status.rs
+++ generated/src/protocol_serde/shape_get_key_rotation_status.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -25,8 +29,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -70,8 +77,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -95,22 +103,26 @@
             }
             tmp
         }),
-        "UnsupportedOperationException" => super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::UnsupportedOperationException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnsupportedOperationException" => {
+            super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::UnsupportedOperationException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::get_key_rotation_status::GetKeyRotationStatusError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_get_parameters_for_import.rs`

```diff
--- reference/src/protocol_serde/shape_get_parameters_for_import.rs
+++ generated/src/protocol_serde/shape_get_parameters_for_import.rs
@@ -15,30 +15,31 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "DependencyTimeoutException" => super::super::operation::get_parameters_for_import::GetParametersForImportError::DependencyTimeoutException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "DependencyTimeoutException" => {
+            super::super::operation::get_parameters_for_import::GetParametersForImportError::DependencyTimeoutException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "InvalidArnException" => super::super::operation::get_parameters_for_import::GetParametersForImportError::InvalidArnException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -74,8 +75,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -99,22 +101,26 @@
             }
             tmp
         }),
-        "UnsupportedOperationException" => super::super::operation::get_parameters_for_import::GetParametersForImportError::UnsupportedOperationException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "UnsupportedOperationException" => {
+            super::super::operation::get_parameters_for_import::GetParametersForImportError::UnsupportedOperationException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::get_parameters_for_import::GetParametersForImportError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::get_parameters_for_import::GetParametersForImportError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_get_public_key.rs`

```diff
--- reference/src/protocol_serde/shape_get_public_key.rs
+++ generated/src/protocol_serde/shape_get_public_key.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,9 +70,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +88,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +134,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -158,9 +165,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_public_key::GetPublicKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -248,7 +257,11 @@
                 }
                 "EncryptionAlgorithms" => {
                     builder = builder.set_encryption_algorithms(
-                        super::super::protocol_serde::shape_encryption_algorithm_spec_list::de_encryption_algorithm_spec_list(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_encryption_algorithm_spec_list::de_encryption_algorithm_spec_list(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "SigningAlgorithms" => {
```

### `src/protocol_serde/shape_import_key_material.rs`

```diff
--- reference/src/protocol_serde/shape_import_key_material.rs
+++ generated/src/protocol_serde/shape_import_key_material.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::import_key_material::ImportKeyMaterialOutput, super::super::operation::import_key_material::ImportKeyMaterialError>
-{
+) -> std::result::Result<
+    super::super::operation::import_key_material::ImportKeyMaterialOutput,
+    super::super::operation::import_key_material::ImportKeyMaterialError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
@@ -23,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -38,9 +43,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ExpiredImportTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_expired_import_token_exception::de_expired_import_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
+                output = super::super::protocol_serde::shape_expired_import_token_exception::de_expired_import_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -87,8 +94,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidCiphertextExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_ciphertext_exception::de_invalid_ciphertext_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_ciphertext_exception::de_invalid_ciphertext_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -102,9 +112,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidImportTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_import_token_exception::de_invalid_import_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_import_token_exception::de_invalid_import_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -133,8 +145,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -163,9 +176,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::import_key_material::ImportKeyMaterialError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -183,8 +198,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::import_key_material::ImportKeyMaterialOutput, super::super::operation::import_key_material::ImportKeyMaterialError>
-{
+) -> std::result::Result<
+    super::super::operation::import_key_material::ImportKeyMaterialOutput,
+    super::super::operation::import_key_material::ImportKeyMaterialError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::import_key_material::builders::ImportKeyMaterialOutputBuilder::default();
```

### `src/protocol_serde/shape_incorrect_key_material_exception.rs`

```diff
--- reference/src/protocol_serde/shape_incorrect_key_material_exception.rs
+++ generated/src/protocol_serde/shape_incorrect_key_material_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_incorrect_key_material_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::IncorrectKeyMaterialExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::IncorrectKeyMaterialExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::IncorrectKeyMaterialExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_incorrect_trust_anchor_exception.rs`

```diff
--- reference/src/protocol_serde/shape_incorrect_trust_anchor_exception.rs
+++ generated/src/protocol_serde/shape_incorrect_trust_anchor_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_incorrect_trust_anchor_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::IncorrectTrustAnchorExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::IncorrectTrustAnchorExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::IncorrectTrustAnchorExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_alias_name_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_alias_name_exception.rs
+++ generated/src/protocol_serde/shape_invalid_alias_name_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_invalid_alias_name_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidAliasNameExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidAliasNameExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidAliasNameExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_ciphertext_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_ciphertext_exception.rs
+++ generated/src/protocol_serde/shape_invalid_ciphertext_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_invalid_ciphertext_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidCiphertextExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidCiphertextExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidCiphertextExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_grant_id_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_grant_id_exception.rs
+++ generated/src/protocol_serde/shape_invalid_grant_id_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_invalid_grant_id_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidGrantIdExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidGrantIdExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidGrantIdExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_grant_token_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_grant_token_exception.rs
+++ generated/src/protocol_serde/shape_invalid_grant_token_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_invalid_grant_token_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidGrantTokenExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidGrantTokenExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidGrantTokenExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_import_token_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_import_token_exception.rs
+++ generated/src/protocol_serde/shape_invalid_import_token_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_invalid_import_token_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidImportTokenExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidImportTokenExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidImportTokenExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_key_usage_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_key_usage_exception.rs
+++ generated/src/protocol_serde/shape_invalid_key_usage_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_invalid_key_usage_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidKeyUsageExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidKeyUsageExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidKeyUsageExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_key_agreement_algorithm_spec_list.rs`

```diff
--- reference/src/protocol_serde/shape_key_agreement_algorithm_spec_list.rs
+++ generated/src/protocol_serde/shape_key_agreement_algorithm_spec_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::KeyAgreementAlgorithmSpec>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::KeyAgreementAlgorithmSpec>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_key_last_usage_data.rs`

```diff
--- reference/src/protocol_serde/shape_key_last_usage_data.rs
+++ generated/src/protocol_serde/shape_key_last_usage_data.rs
@@ -24,7 +24,10 @@
                         "Operation" => {
                             builder = builder.set_operation(
                                 ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| super::super::types::KeyLastUsageTrackingOperation::from(u.as_ref())))
+                                    .map(|s| {
+                                        s.to_unescaped()
+                                            .map(|u| super::super::types::KeyLastUsageTrackingOperation::from(u.as_ref()))
+                                    })
                                     .transpose()?,
                             );
                         }
```

### `src/protocol_serde/shape_key_metadata.rs`

```diff
--- reference/src/protocol_serde/shape_key_metadata.rs
+++ generated/src/protocol_serde/shape_key_metadata.rs
@@ -144,7 +144,11 @@
                         }
                         "SigningAlgorithms" => {
                             builder = builder.set_signing_algorithms(
-                                super::super::protocol_serde::shape_signing_algorithm_spec_list::de_signing_algorithm_spec_list(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_signing_algorithm_spec_list::de_signing_algorithm_spec_list(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "KeyAgreementAlgorithms" => {
@@ -161,7 +165,11 @@
                         }
                         "MultiRegionConfiguration" => {
                             builder = builder.set_multi_region_configuration(
-                                super::super::protocol_serde::shape_multi_region_configuration::de_multi_region_configuration(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_multi_region_configuration::de_multi_region_configuration(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "PendingDeletionWindowInDays" => {
@@ -172,15 +180,17 @@
                             );
                         }
                         "MacAlgorithms" => {
-                            builder = builder.set_mac_algorithms(super::super::protocol_serde::shape_mac_algorithm_spec_list::de_mac_algorithm_spec_list(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
+                            builder = builder.set_mac_algorithms(
+                                super::super::protocol_serde::shape_mac_algorithm_spec_list::de_mac_algorithm_spec_list(tokens, _value, depth + 1)?,
+                            );
                         }
                         "XksKeyConfiguration" => {
                             builder = builder.set_xks_key_configuration(
-                                super::super::protocol_serde::shape_xks_key_configuration_type::de_xks_key_configuration_type(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_xks_key_configuration_type::de_xks_key_configuration_type(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "CurrentKeyMaterialId" => {
@@ -199,9 +209,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::key_metadata_correct_errors(builder).build().map_err(|err| {
-                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
-            })?))
+            Ok(Some(super::super::serde_util::key_metadata_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_key_unavailable_exception.rs`

```diff
--- reference/src/protocol_serde/shape_key_unavailable_exception.rs
+++ generated/src/protocol_serde/shape_key_unavailable_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_key_unavailable_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::KeyUnavailableExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::KeyUnavailableExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::KeyUnavailableExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_kms_invalid_signature_exception.rs`

```diff
--- reference/src/protocol_serde/shape_kms_invalid_signature_exception.rs
+++ generated/src/protocol_serde/shape_kms_invalid_signature_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_kms_invalid_signature_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::KmsInvalidSignatureExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::KmsInvalidSignatureExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::KmsInvalidSignatureExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_kms_invalid_state_exception.rs`

```diff
--- reference/src/protocol_serde/shape_kms_invalid_state_exception.rs
+++ generated/src/protocol_serde/shape_kms_invalid_state_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_kms_invalid_state_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::KmsInvalidStateExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::KmsInvalidStateExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::KmsInvalidStateExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_aliases.rs`

```diff
--- reference/src/protocol_serde/shape_list_aliases.rs
+++ generated/src/protocol_serde/shape_list_aliases.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_aliases::ListAliasesError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_aliases::ListAliasesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -125,8 +128,10 @@
 pub(crate) fn de_list_aliases(
     _value: &[u8],
     mut builder: super::super::operation::list_aliases::builders::ListAliasesOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_aliases::builders::ListAliasesOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_aliases::builders::ListAliasesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_grants.rs`

```diff
--- reference/src/protocol_serde/shape_list_grants.rs
+++ generated/src/protocol_serde/shape_list_grants.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_grants::ListGrantsError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_grants::ListGrantsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantIdExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_grant_id_exception::de_invalid_grant_id_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_grants::ListGrantsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_grant_id_exception::de_invalid_grant_id_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_grants::ListGrantsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -97,8 +101,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_grants::ListGrantsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_grants::ListGrantsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -155,8 +160,10 @@
 pub(crate) fn de_list_grants(
     _value: &[u8],
     mut builder: super::super::operation::list_grants::builders::ListGrantsOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_grants::builders::ListGrantsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_grants::builders::ListGrantsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_key_policies.rs`

```diff
--- reference/src/protocol_serde/shape_list_key_policies.rs
+++ generated/src/protocol_serde/shape_list_key_policies.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_key_policies::ListKeyPoliciesOutput, super::super::operation::list_key_policies::ListKeyPoliciesError> {
+) -> std::result::Result<
+    super::super::operation::list_key_policies::ListKeyPoliciesOutput,
+    super::super::operation::list_key_policies::ListKeyPoliciesError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::list_key_policies::ListKeyPoliciesError::unhandled)?;
@@ -22,8 +25,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_key_policies::ListKeyPoliciesError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_key_policies::ListKeyPoliciesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +73,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_key_policies::ListKeyPoliciesError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_key_policies::ListKeyPoliciesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -101,7 +108,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_key_policies::ListKeyPoliciesOutput, super::super::operation::list_key_policies::ListKeyPoliciesError> {
+) -> std::result::Result<
+    super::super::operation::list_key_policies::ListKeyPoliciesOutput,
+    super::super::operation::list_key_policies::ListKeyPoliciesError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::list_key_policies::builders::ListKeyPoliciesOutputBuilder::default();
```

### `src/protocol_serde/shape_list_key_rotations.rs`

```diff
--- reference/src/protocol_serde/shape_list_key_rotations.rs
+++ generated/src/protocol_serde/shape_list_key_rotations.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_key_rotations::ListKeyRotationsOutput, super::super::operation::list_key_rotations::ListKeyRotationsError> {
+) -> std::result::Result<
+    super::super::operation::list_key_rotations::ListKeyRotationsOutput,
+    super::super::operation::list_key_rotations::ListKeyRotationsError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::list_key_rotations::ListKeyRotationsError::unhandled)?;
@@ -67,8 +70,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_key_rotations::ListKeyRotationsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_key_rotations::ListKeyRotationsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -97,9 +101,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::list_key_rotations::ListKeyRotationsError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_key_rotations::ListKeyRotationsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -117,7 +123,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_key_rotations::ListKeyRotationsOutput, super::super::operation::list_key_rotations::ListKeyRotationsError> {
+) -> std::result::Result<
+    super::super::operation::list_key_rotations::ListKeyRotationsOutput,
+    super::super::operation::list_key_rotations::ListKeyRotationsError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::list_key_rotations::builders::ListKeyRotationsOutputBuilder::default();
@@ -155,7 +164,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "Rotations" => {
-                    builder = builder.set_rotations(super::super::protocol_serde::shape_rotations_list::de_rotations_list(tokens, _value, depth + 1)?);
+                    builder = builder.set_rotations(super::super::protocol_serde::shape_rotations_list::de_rotations_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
```

### `src/protocol_serde/shape_list_keys.rs`

```diff
--- reference/src/protocol_serde/shape_list_keys.rs
+++ generated/src/protocol_serde/shape_list_keys.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_keys::ListKeysError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_keys::ListKeysError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -95,7 +98,8 @@
 pub(crate) fn de_list_keys(
     _value: &[u8],
     mut builder: super::super::operation::list_keys::builders::ListKeysOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_keys::builders::ListKeysOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::operation::list_keys::builders::ListKeysOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_resource_tags.rs`

```diff
--- reference/src/protocol_serde/shape_list_resource_tags.rs
+++ generated/src/protocol_serde/shape_list_resource_tags.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_resource_tags::ListResourceTagsOutput, super::super::operation::list_resource_tags::ListResourceTagsError> {
+) -> std::result::Result<
+    super::super::operation::list_resource_tags::ListResourceTagsOutput,
+    super::super::operation::list_resource_tags::ListResourceTagsError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::list_resource_tags::ListResourceTagsError::unhandled)?;
@@ -86,7 +89,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_resource_tags::ListResourceTagsOutput, super::super::operation::list_resource_tags::ListResourceTagsError> {
+) -> std::result::Result<
+    super::super::operation::list_resource_tags::ListResourceTagsOutput,
+    super::super::operation::list_resource_tags::ListResourceTagsError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::list_resource_tags::builders::ListResourceTagsOutputBuilder::default();
```

### `src/protocol_serde/shape_list_retirable_grants.rs`

```diff
--- reference/src/protocol_serde/shape_list_retirable_grants.rs
+++ generated/src/protocol_serde/shape_list_retirable_grants.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::list_retirable_grants::ListRetirableGrantsError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::list_retirable_grants::ListRetirableGrantsError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -25,8 +29,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_multi_region_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_multi_region_configuration.rs
+++ generated/src/protocol_serde/shape_multi_region_configuration.rs
@@ -20,30 +20,30 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "MultiRegionKeyType" => {
-                            builder = builder.set_multi_region_key_type(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| super::super::types::MultiRegionKeyType::from(u.as_ref())))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "MultiRegionKeyType" => {
+                                builder = builder.set_multi_region_key_type(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| super::super::types::MultiRegionKeyType::from(u.as_ref())))
+                                        .transpose()?,
+                                );
+                            }
+                            "PrimaryKey" => {
+                                builder = builder.set_primary_key(super::super::protocol_serde::shape_multi_region_key::de_multi_region_key(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?);
+                            }
+                            "ReplicaKeys" => {
+                                builder = builder.set_replica_keys(
+                                    super::super::protocol_serde::shape_multi_region_key_list::de_multi_region_key_list(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        "PrimaryKey" => {
-                            builder = builder.set_primary_key(super::super::protocol_serde::shape_multi_region_key::de_multi_region_key(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "ReplicaKeys" => {
-                            builder = builder.set_replica_keys(super::super::protocol_serde::shape_multi_region_key_list::de_multi_region_key_list(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_put_key_policy.rs`

```diff
--- reference/src/protocol_serde/shape_put_key_policy.rs
+++ generated/src/protocol_serde/shape_put_key_policy.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_key_policy::PutKeyPolicyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::put_key_policy::PutKeyPolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +70,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_key_policy::PutKeyPolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::put_key_policy::PutKeyPolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -130,9 +134,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::put_key_policy::PutKeyPolicyError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::put_key_policy::PutKeyPolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -168,3 +174,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_put_key_policy(
+    _value: &[u8],
+    mut builder: super::super::operation::put_key_policy::builders::PutKeyPolicyOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::put_key_policy::builders::PutKeyPolicyOutputBuilder,
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

### `src/protocol_serde/shape_re_encrypt.rs`

```diff
--- reference/src/protocol_serde/shape_re_encrypt.rs
+++ generated/src/protocol_serde/shape_re_encrypt.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +86,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidCiphertextExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_ciphertext_exception::de_invalid_ciphertext_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_ciphertext_exception::de_invalid_ciphertext_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -97,9 +104,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +122,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -158,8 +168,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::re_encrypt::ReEncryptError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -216,7 +227,10 @@
 pub(crate) fn de_re_encrypt(
     _value: &[u8],
     mut builder: super::super::operation::re_encrypt::builders::ReEncryptOutputBuilder,
-) -> ::std::result::Result<super::super::operation::re_encrypt::builders::ReEncryptOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::operation::re_encrypt::builders::ReEncryptOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_replicate_key.rs`

```diff
--- reference/src/protocol_serde/shape_replicate_key.rs
+++ generated/src/protocol_serde/shape_replicate_key.rs
@@ -82,8 +82,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::replicate_key::ReplicateKeyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::replicate_key::ReplicateKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -160,9 +161,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::replicate_key::ReplicateKeyError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::replicate_key::ReplicateKeyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -218,8 +221,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "ReplicaKeyMetadata" => {
-                    builder =
-                        builder.set_replica_key_metadata(super::super::protocol_serde::shape_key_metadata::de_key_metadata(tokens, _value, depth + 1)?);
+                    builder = builder.set_replica_key_metadata(super::super::protocol_serde::shape_key_metadata::de_key_metadata(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "ReplicaPolicy" => {
                     builder = builder.set_replica_policy(
```

### `src/protocol_serde/shape_retire_grant.rs`

```diff
--- reference/src/protocol_serde/shape_retire_grant.rs
+++ generated/src/protocol_serde/shape_retire_grant.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -37,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +71,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantIdExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_grant_id_exception::de_invalid_grant_id_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_grant_id_exception::de_invalid_grant_id_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,9 +87,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,8 +120,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::retire_grant::RetireGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -165,3 +173,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_retire_grant(
+    _value: &[u8],
+    mut builder: super::super::operation::retire_grant::builders::RetireGrantOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::retire_grant::builders::RetireGrantOutputBuilder,
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

### `src/protocol_serde/shape_revoke_grant.rs`

```diff
--- reference/src/protocol_serde/shape_revoke_grant.rs
+++ generated/src/protocol_serde/shape_revoke_grant.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -37,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,8 +71,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantIdExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_grant_id_exception::de_invalid_grant_id_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_grant_id_exception::de_invalid_grant_id_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -97,8 +102,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::revoke_grant::RevokeGrantError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -149,3 +155,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_revoke_grant(
+    _value: &[u8],
+    mut builder: super::super::operation::revoke_grant::builders::RevokeGrantOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::revoke_grant::builders::RevokeGrantOutputBuilder,
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

### `src/protocol_serde/shape_rotate_key_on_demand.rs`

```diff
--- reference/src/protocol_serde/shape_rotate_key_on_demand.rs
+++ generated/src/protocol_serde/shape_rotate_key_on_demand.rs
@@ -40,8 +40,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::rotate_key_on_demand::RotateKeyOnDemandError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::rotate_key_on_demand::RotateKeyOnDemandError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -100,8 +103,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::rotate_key_on_demand::RotateKeyOnDemandError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::rotate_key_on_demand::RotateKeyOnDemandError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -145,9 +149,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::rotate_key_on_demand::RotateKeyOnDemandError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::rotate_key_on_demand::RotateKeyOnDemandError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_schedule_key_deletion.rs`

```diff
--- reference/src/protocol_serde/shape_schedule_key_deletion.rs
+++ generated/src/protocol_serde/shape_schedule_key_deletion.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::schedule_key_deletion::ScheduleKeyDeletionError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::schedule_key_deletion::ScheduleKeyDeletionError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -25,8 +29,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::schedule_key_deletion::ScheduleKeyDeletionError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::schedule_key_deletion::ScheduleKeyDeletionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -70,8 +77,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::schedule_key_deletion::ScheduleKeyDeletionError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::schedule_key_deletion::ScheduleKeyDeletionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_sign.rs`

```diff
--- reference/src/protocol_serde/shape_sign.rs
+++ generated/src/protocol_serde/shape_sign.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::sign::SignError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::sign::SignError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::sign::SignError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::sign::SignError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,9 +71,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::sign::SignError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::sign::SignError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +89,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::sign::SignError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::sign::SignError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +135,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::sign::SignError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::sign::SignError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -166,7 +174,8 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::sign::builders::SignOutputBuilder::default();
-        output = super::super::protocol_serde::shape_sign::de_sign(_response_body, output).map_err(super::super::operation::sign::SignError::unhandled)?;
+        output =
+            super::super::protocol_serde::shape_sign::de_sign(_response_body, output).map_err(super::super::operation::sign::SignError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_tag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource.rs
+++ generated/src/protocol_serde/shape_tag_resource.rs
@@ -52,8 +52,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::tag_resource::TagResourceError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -134,3 +135,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_tag_resource(
+    _value: &[u8],
+    mut builder: super::super::operation::tag_resource::builders::TagResourceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::tag_resource::builders::TagResourceOutputBuilder,
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

### `src/protocol_serde/shape_unsupported_operation_exception.rs`

```diff
--- reference/src/protocol_serde/shape_unsupported_operation_exception.rs
+++ generated/src/protocol_serde/shape_unsupported_operation_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_unsupported_operation_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::UnsupportedOperationExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::UnsupportedOperationExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::UnsupportedOperationExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_untag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource.rs
+++ generated/src/protocol_serde/shape_untag_resource.rs
@@ -52,8 +52,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::untag_resource::UntagResourceError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::untag_resource::UntagResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,3 +120,36 @@
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

### `src/protocol_serde/shape_update_alias.rs`

```diff
--- reference/src/protocol_serde/shape_update_alias.rs
+++ generated/src/protocol_serde/shape_update_alias.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_alias::UpdateAliasError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_alias::UpdateAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_alias::UpdateAliasError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_alias::UpdateAliasError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,3 +123,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_alias(
+    _value: &[u8],
+    mut builder: super::super::operation::update_alias::builders::UpdateAliasOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_alias::builders::UpdateAliasOutputBuilder,
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

### `src/protocol_serde/shape_update_custom_key_store.rs`

```diff
--- reference/src/protocol_serde/shape_update_custom_key_store.rs
+++ generated/src/protocol_serde/shape_update_custom_key_store.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -42,11 +46,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CloudHsmClusterNotActiveExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(_response_body, output).map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -62,11 +62,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CloudHsmClusterNotFoundExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_found_exception::de_cloud_hsm_cluster_not_found_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_cloud_hsm_cluster_not_found_exception::de_cloud_hsm_cluster_not_found_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -82,12 +83,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CloudHsmClusterNotRelatedExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_cloud_hsm_cluster_not_related_exception::de_cloud_hsm_cluster_not_related_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_cloud_hsm_cluster_not_related_exception::de_cloud_hsm_cluster_not_related_exception_json_err(_response_body, output).map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -103,12 +99,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(_response_body, output).map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -124,11 +115,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::CustomKeyStoreNameInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_custom_key_store_name_in_use_exception::de_custom_key_store_name_in_use_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_custom_key_store_name_in_use_exception::de_custom_key_store_name_in_use_exception_json_err(_response_body, output).map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -138,24 +125,27 @@
                 tmp
             })
         }
-        "CustomKeyStoreNotFoundException" => super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::CustomKeyStoreNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "CustomKeyStoreNotFoundException" => {
+            super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::CustomKeyStoreNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "KMSInternalException" => super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::KmsInternalException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -209,11 +199,12 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::XksProxyInvalidResponseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_xks_proxy_invalid_response_exception::de_xks_proxy_invalid_response_exception_json_err(
-                        _response_body,
-                        output,
-                    )
-                    .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_xks_proxy_invalid_response_exception::de_xks_proxy_invalid_response_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -229,12 +220,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::XksProxyUriEndpointInUseExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_xks_proxy_uri_endpoint_in_use_exception::de_xks_proxy_uri_endpoint_in_use_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    output = super::super::protocol_serde::shape_xks_proxy_uri_endpoint_in_use_exception::de_xks_proxy_uri_endpoint_in_use_exception_json_err(_response_body, output).map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -249,23 +235,7 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::XksProxyUriInUseExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_xks_proxy_uri_in_use_exception::de_xks_proxy_uri_in_use_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "XksProxyUriUnreachableException" => super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyUriUnreachableException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::XksProxyUriUnreachableExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_xks_proxy_uri_unreachable_exception::de_xks_proxy_uri_unreachable_exception_json_err(
+                output = super::super::protocol_serde::shape_xks_proxy_uri_in_use_exception::de_xks_proxy_uri_in_use_exception_json_err(
                     _response_body,
                     output,
                 )
@@ -278,6 +248,27 @@
             }
             tmp
         }),
+        "XksProxyUriUnreachableException" => {
+            super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyUriUnreachableException({
+                #[allow(unused_mut)]
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::XksProxyUriUnreachableExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_xks_proxy_uri_unreachable_exception::de_xks_proxy_uri_unreachable_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "XksProxyVpcEndpointServiceInUseException" => {
             super::super::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceInUseException({
                 #[allow(unused_mut)]
@@ -356,3 +347,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_custom_key_store(
+    _value: &[u8],
+    mut builder: super::super::operation::update_custom_key_store::builders::UpdateCustomKeyStoreOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_custom_key_store::builders::UpdateCustomKeyStoreOutputBuilder,
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

### `src/protocol_serde/shape_update_custom_key_store_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_custom_key_store_input.rs
+++ generated/src/protocol_serde/shape_update_custom_key_store_input.rs
@@ -30,7 +30,10 @@
     if let Some(var_9) = &input.xks_proxy_authentication_credential {
         #[allow(unused_mut)]
         let mut object_10 = object.key("XksProxyAuthenticationCredential").start_object();
-        super::super::protocol_serde::shape_xks_proxy_authentication_credential_type::ser_xks_proxy_authentication_credential_type(&mut object_10, var_9)?;
+        super::super::protocol_serde::shape_xks_proxy_authentication_credential_type::ser_xks_proxy_authentication_credential_type(
+            &mut object_10,
+            var_9,
+        )?;
         object_10.finish();
     }
     if let Some(var_11) = &input.xks_proxy_connectivity {
```

### `src/protocol_serde/shape_update_key_description.rs`

```diff
--- reference/src/protocol_serde/shape_update_key_description.rs
+++ generated/src/protocol_serde/shape_update_key_description.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::update_key_description::UpdateKeyDescriptionError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::update_key_description::UpdateKeyDescriptionError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -25,8 +29,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_key_description::UpdateKeyDescriptionError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_key_description::UpdateKeyDescriptionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -70,8 +77,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_key_description::UpdateKeyDescriptionError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_key_description::UpdateKeyDescriptionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -125,3 +133,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_key_description(
+    _value: &[u8],
+    mut builder: super::super::operation::update_key_description::builders::UpdateKeyDescriptionOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_key_description::builders::UpdateKeyDescriptionOutputBuilder,
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

### `src/protocol_serde/shape_update_primary_region.rs`

```diff
--- reference/src/protocol_serde/shape_update_primary_region.rs
+++ generated/src/protocol_serde/shape_update_primary_region.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::update_primary_region::UpdatePrimaryRegionError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::update_primary_region::UpdatePrimaryRegionError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -70,8 +74,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -100,9 +105,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::UnsupportedOperationExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
+                output = super::super::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -141,3 +148,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_update_primary_region(
+    _value: &[u8],
+    mut builder: super::super::operation::update_primary_region::builders::UpdatePrimaryRegionOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::update_primary_region::builders::UpdatePrimaryRegionOutputBuilder,
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

### `src/protocol_serde/shape_verify.rs`

```diff
--- reference/src/protocol_serde/shape_verify.rs
+++ generated/src/protocol_serde/shape_verify.rs
@@ -22,8 +22,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DependencyTimeoutExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify::VerifyError::unhandled)?;
+                output = super::super::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::verify::VerifyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify::VerifyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::verify::VerifyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -67,9 +71,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::verify::VerifyError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::verify::VerifyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +89,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify::VerifyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::verify::VerifyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,9 +135,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidSignatureExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_kms_invalid_signature_exception::de_kms_invalid_signature_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::verify::VerifyError::unhandled)?;
+                output = super::super::protocol_serde::shape_kms_invalid_signature_exception::de_kms_invalid_signature_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::verify::VerifyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -144,8 +153,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify::VerifyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::verify::VerifyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -182,7 +192,8 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::verify::builders::VerifyOutputBuilder::default();
-        output = super::super::protocol_serde::shape_verify::de_verify(_response_body, output).map_err(super::super::operation::verify::VerifyError::unhandled)?;
+        output = super::super::protocol_serde::shape_verify::de_verify(_response_body, output)
+            .map_err(super::super::operation::verify::VerifyError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_verify_mac.rs`

```diff
--- reference/src/protocol_serde/shape_verify_mac.rs
+++ generated/src/protocol_serde/shape_verify_mac.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::DryRunOperationExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,9 +53,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
+                output = super::super::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +71,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -128,8 +132,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::KmsInvalidStateExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::verify_mac::VerifyMacError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -186,7 +191,10 @@
 pub(crate) fn de_verify_mac(
     _value: &[u8],
     mut builder: super::super::operation::verify_mac::builders::VerifyMacOutputBuilder,
-) -> ::std::result::Result<super::super::operation::verify_mac::builders::VerifyMacOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::operation::verify_mac::builders::VerifyMacOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_xks_key_already_in_use_exception.rs`

```diff
--- reference/src/protocol_serde/shape_xks_key_already_in_use_exception.rs
+++ generated/src/protocol_serde/shape_xks_key_already_in_use_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_xks_key_already_in_use_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::XksKeyAlreadyInUseExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::XksKeyAlreadyInUseExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::XksKeyAlreadyInUseExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_xks_key_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_xks_key_not_found_exception.rs
+++ generated/src/protocol_serde/shape_xks_key_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_xks_key_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::XksKeyNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::XksKeyNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::XksKeyNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_xks_proxy_uri_in_use_exception.rs`

```diff
--- reference/src/protocol_serde/shape_xks_proxy_uri_in_use_exception.rs
+++ generated/src/protocol_serde/shape_xks_proxy_uri_in_use_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_xks_proxy_uri_in_use_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::XksProxyUriInUseExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::XksProxyUriInUseExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::XksProxyUriInUseExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/types/_key_metadata.rs`

```diff
--- reference/src/types/_key_metadata.rs
+++ generated/src/types/_key_metadata.rs
@@ -559,7 +559,10 @@
         self
     }
     /// <p>The key agreement algorithm used to derive a shared secret.</p>
-    pub fn set_key_agreement_algorithms(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::KeyAgreementAlgorithmSpec>>) -> Self {
+    pub fn set_key_agreement_algorithms(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::KeyAgreementAlgorithmSpec>>,
+    ) -> Self {
         self.key_agreement_algorithms = input;
         self
     }
```
