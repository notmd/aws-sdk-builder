# AWS SDK Conformance Report: codeartifact

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## codeartifact
**Progress:** `459/459` files compared · `457` matched · `2` mismatches · `0` missing · `0` extra · `99.56%` match (100.00% means fully matched)

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -24,9 +24,9 @@
 //! CodeArtifact is a fully managed artifact repository compatible with language-native package managers and build tools such as npm, Apache Maven, pip, and dotnet. You can use CodeArtifact to share packages with development teams and pull packages. Packages can be pulled from both public and CodeArtifact repositories. You can also create an upstream relationship between a CodeArtifact repository and another repository, which effectively merges their contents from the point of view of a package manager client.
 //!
 //! __CodeArtifact concepts__
-//!   - __Repository__: A CodeArtifact repository contains a set of [package versions](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html#welcome-concepts-package-version), each of which maps to a set of assets, or files. Repositories are polyglot, so a single repository can contain packages of any supported type. Each repository exposes endpoints for fetching and publishing packages using tools such as the __ npm __ CLI or the Maven CLI (__ mvn __). For a list of supported package managers, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).
-//!   - __Domain__: Repositories are aggregated into a higher-level entity known as a _domain_. All package assets and metadata are stored in the domain, but are consumed through repositories. A given package asset, such as a Maven JAR file, is stored once per domain, no matter how many repositories it's present in. All of the assets and metadata in a domain are encrypted with the same customer master key (CMK) stored in Key Management Service (KMS). Each repository is a member of a single domain and can't be moved to a different domain. The domain allows organizational policy to be applied across multiple repositories, such as which accounts can access repositories in the domain, and which public repositories can be used as sources of packages. Although an organization can have multiple domains, we recommend a single production domain that contains all published artifacts so that teams can find and share packages across their organization.
-//!   - __Package__: A _package_ is a bundle of software and the metadata required to resolve dependencies and install the software. CodeArtifact supports npm, PyPI, Maven, NuGet, Swift, Ruby, Cargo, and generic package formats. For more information about the supported package formats and how to use CodeArtifact with them, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html). In CodeArtifact, a package consists of:
+//!   - __Repository__: A CodeArtifact repository contains a set of [package versions](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html#welcome-concepts-package-version), each of which maps to a set of assets, or files. Repositories are polyglot, so a single repository can contain packages of any supported type. Each repository exposes endpoints for fetching and publishing packages using tools such as the __npm__ CLI or the Maven CLI (__mvn__). For a list of supported package managers, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).
+//!   - __Domain__: Repositories are aggregated into a higher-level entity known as a _domain_. All package assets and metadata are stored in the domain, but are consumed through repositories. A given package asset, such as a Maven JAR file, is stored once per domain, no matter how many repositories it's present in. All of the assets and metadata in a domain are encrypted with the same customer master key (CMK) stored in Key Management Service (KMS).Each repository is a member of a single domain and can't be moved to a different domain.The domain allows organizational policy to be applied across multiple repositories, such as which accounts can access repositories in the domain, and which public repositories can be used as sources of packages.Although an organization can have multiple domains, we recommend a single production domain that contains all published artifacts so that teams can find and share packages across their organization.
+//!   - __Package__: A _package_ is a bundle of software and the metadata required to resolve dependencies and install the software. CodeArtifact supports npm, PyPI, Maven, NuGet, Swift, Ruby, Cargo, and generic package formats. For more information about the supported package formats and how to use CodeArtifact with them, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).In CodeArtifact, a package consists of:
 //!     - A _name_ (for example, webpack is the name of a popular npm package)
 //!     - An optional namespace (for example, @types in @types/node)
 //!     - A set of versions (for example, 1.0.0, 1.0.1, 1.0.2, etc.)
```

### `src/operation/get_authorization_token.rs`

```diff
--- reference/src/operation/get_authorization_token.rs
+++ generated/src/operation/get_authorization_token.rs
@@ -108,6 +108,7 @@
                 .expect("required fields set"),
         ));

+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "GetAuthorizationToken",
             "codeartifact",
```
