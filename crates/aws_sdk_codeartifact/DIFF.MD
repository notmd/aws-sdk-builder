# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/codeartifact`
- model: `models/codeartifact.json`
- package: `aws-sdk-codeartifact`
- library: `aws_sdk_codeartifact`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_associate_external_connection`: `AssociateExternalConnection`
- `op_copy_package_versions`: `CopyPackageVersions`
- `op_create_domain`: `CreateDomain`
- `op_create_package_group`: `CreatePackageGroup`
- `op_create_repository`: `CreateRepository`
- `op_delete_domain`: `DeleteDomain`
- `op_delete_domain_permissions_policy`: `DeleteDomainPermissionsPolicy`
- `op_delete_package`: `DeletePackage`
- `op_delete_package_group`: `DeletePackageGroup`
- `op_delete_package_versions`: `DeletePackageVersions`
- `op_delete_repository`: `DeleteRepository`
- `op_delete_repository_permissions_policy`: `DeleteRepositoryPermissionsPolicy`
- `op_describe_domain`: `DescribeDomain`
- `op_describe_package`: `DescribePackage`
- `op_describe_package_group`: `DescribePackageGroup`
- `op_describe_package_version`: `DescribePackageVersion`
- `op_describe_repository`: `DescribeRepository`
- `op_disassociate_external_connection`: `DisassociateExternalConnection`
- `op_dispose_package_versions`: `DisposePackageVersions`
- `op_get_associated_package_group`: `GetAssociatedPackageGroup`
- `op_get_authorization_token`: `GetAuthorizationToken`
- `op_get_domain_permissions_policy`: `GetDomainPermissionsPolicy`
- `op_get_package_version_asset`: `GetPackageVersionAsset`
- `op_get_package_version_readme`: `GetPackageVersionReadme`
- `op_get_repository_endpoint`: `GetRepositoryEndpoint`
- `op_get_repository_permissions_policy`: `GetRepositoryPermissionsPolicy`
- `op_list_allowed_repositories_for_group`: `ListAllowedRepositoriesForGroup`
- `op_list_associated_packages`: `ListAssociatedPackages`
- `op_list_domains`: `ListDomains`
- `op_list_package_groups`: `ListPackageGroups`
- `op_list_packages`: `ListPackages`
- `op_list_package_version_assets`: `ListPackageVersionAssets`
- `op_list_package_version_dependencies`: `ListPackageVersionDependencies`
- `op_list_package_versions`: `ListPackageVersions`
- `op_list_repositories`: `ListRepositories`
- `op_list_repositories_in_domain`: `ListRepositoriesInDomain`
- `op_list_sub_package_groups`: `ListSubPackageGroups`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_publish_package_version`: `PublishPackageVersion`
- `op_put_domain_permissions_policy`: `PutDomainPermissionsPolicy`
- `op_put_package_origin_configuration`: `PutPackageOriginConfiguration`
- `op_put_repository_permissions_policy`: `PutRepositoryPermissionsPolicy`
- `op_tag_resource`: `TagResource`
- `op_untag_resource`: `UntagResource`
- `op_update_package_group`: `UpdatePackageGroup`
- `op_update_package_group_origin_configuration`: `UpdatePackageGroupOriginConfiguration`
- `op_update_package_versions_status`: `UpdatePackageVersionsStatus`
- `op_update_repository`: `UpdateRepository`

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
