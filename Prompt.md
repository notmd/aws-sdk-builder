# AWS SDK per-operation modularizer

`Prompt.md` is the source of truth for the Rust codemod in this repository.
Discard the existing AWS SDK builder, generated-artifact, normalized-projection,
and snapshot-conformance infrastructure. Build the new system described here
instead of incrementally migrating the old implementation or preserving a
compatibility mode.

## Objective

Create a Rust codemod that reads `services-manifest.json`, downloads the pinned
official AWS SDK Rust source for every manifest entry, reads that service's
Smithy model from the downloaded archive, and modifies the downloaded crate into a modular AWS SDK
crate with one Cargo feature per operation.

The official AWS SDK source is the transformation baseline. This is an AST
codemod, not a new Rust SDK renderer. The codemod must use `syn` to parse and
modify Rust source and a structural TOML library to modify `Cargo.toml`.
Never use regular expressions, line matching, ad-hoc string replacement, or a
hardcoded operation list to transform Rust code.

## Reset boundary

Remove the current builder and conformance pipeline, including its renderer,
provider crates, canonical `original.rs` artifacts, normalized projections,
reference patches, and old snapshot reports. Do not add a consumer/conformance
mode flag or a compatibility adapter for that architecture.

Keep only useful manifest, model, and upstream provenance data, rewriting them
as needed for the new workflow. Existing generated source is not an input or a
source of truth.

After the reset, clean up every obsolete implementation artifact. Use
`cargo metadata`, repository searches, and the workspace dependency graph to
prove whether old code is still referenced before removing it. Delete unused
builder/conformance modules, provider crates, stale manifests, snapshot trees,
reference patches, dead fixtures, obsolete examples, unused dependencies, and
documentation that describes the discarded architecture. Do not leave dead
code, abandoned compatibility shims, or unused Cargo workspace members behind.
Preserve only files required by the new codemod, its tests, the manifest/model
inputs, and current documentation.

Maintain `docs/aws-sdk-modularizer-status.md` as the durable checkpoint log for
long-running work. Read it before starting or resuming a change. Immediately
after every commit, append a checkpoint containing the commit hash and date,
objective, changed generic rule and files, commands run with pass/fail results,
operation coverage before and after, remaining blocker, and one concrete next
action. A status update must not be skipped because the commit was small; if it
cannot be included in the commit just created, leave it in the worktree and
carry it into the next commit.

The new Rust CLI should live in a dedicated workspace package named
`aws-sdk-modularizer` at the workspace root under `src/`. Its normal invocation
is:

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```

## Manifest-driven inputs

`services-manifest.json` must explicitly describe all inputs and outputs. Each
service entry must provide:

- the pinned upstream repository and immutable commit or revision;
- the path to the service crate in the downloaded SDK;
- the archive-relative Smithy JSON model path;
- the output directory;
- the Cargo package name and Rust library crate name.

Do not infer service support, output paths, package names, or operation lists in
Rust code. A model operation is discovered from the service shape's operation
targets. Validate that every selected service has exactly one service shape and
that every operation target can be mapped to the downloaded source.

Download and extract the pinned SDK into a temporary directory on each run.
Do not commit the upstream SDK or a download cache. Validate the requested
repository/revision and fail before writing output when the source or model is
missing, malformed, ambiguous, or inconsistent with the manifest.

## Smithy model and Smithy-RS reference

The JSON model is authoritative for operation identity, operation order, model
traits, and any relationship needed to decide which generated items belong to
an operation. Rust identifiers and feature names must follow the naming rules
used by Smithy-RS. For example:

```text
HeadBucket -> head_bucket -> op_head_bucket
```

Before implementing or extending the codemod, inspect the local
`/tmp/smithy-rs` checkout. In particular, use the Smithy-RS `RustCrate`,
`RustModule`, `CodegenDelegator`, `CargoTomlGenerator`, operation generators,
and AWS decorators as the reference for module ownership, crate manifests,
operation-specific customization, and special cases.

Do not hardcode service names or operation names. A special case is allowed
only when the model JSON does not contain the required information and Smithy-RS
has corresponding special handling for that service or model trait. Isolate
such logic in a typed, documented customization with a focused fixture and a
reference to the Smithy-RS behavior. Literal operation names may not be used as
general transformation rules.

## Operation feature flags

For every operation discovered from the model, add a Cargo feature named
`op_<snake_case_operation_name>`. New operation features must not be enabled by
default. Preserve unrelated upstream default features and never add an
`op_*` feature to Cargo's `default` feature list.

The public operation module declaration must have this form:

```rust
#[cfg(feature = "op_head_bucket")]
pub mod head_bucket;
```

The transform must gate the complete public operation surface consistently:

- the operation module declaration;
- the corresponding fluent client method and operation-specific impl items;
- operation builders and operation-only helpers;
- public re-exports and type aliases that exist only for that operation;
- operation-specific tests or examples that exist in retained input files.

Shared runtime code, configuration, and shapes used by multiple operations must
remain available. Do not gate shared items merely because one operation refers
to them. Conversely, an operation-specific public item must not remain exposed
when its `op_*` feature is disabled.

Use a module-aware `syn` AST transform. Resolve the Rust module graph, locate
external and inline modules, preserve existing attributes and visibility, and
add `cfg` attributes to the actual declarations and items derived from the
model. Reject duplicate mappings, unsupported syntax, invalid Rust, and
ambiguous operation ownership rather than guessing.

## Generated crate layout

For every service, copy the official service crate into the manifest-defined
output directory, then apply the AST and manifest transforms. For example:

```text
crates/aws_sdk_s3/
├── Cargo.toml
├── src/
├── DIFF.MD
└── DIFF.diff
```

Modify `Cargo.toml` structurally, keeping the filename unchanged. Update its
package and library names from the manifest and add all model-derived
operation features. Preserve upstream dependencies and metadata unless a
manifest-defined rename requires a corresponding update.

Strip the downloaded service crate's entire `tests/` directory from generated
output. Do not parse, transform, compile, or report those tests. The upstream
and transformed trees must both exclude `tests/**` before comparison, so test
file removal never appears in `DIFF.MD` or `DIFF.diff`. Codemod unit and
integration tests remain separate from generated service-crate output.

Stage each service in a temporary directory and install it atomically only
after parsing, transformation, validation, and diff generation succeed. A
failure must not leave a partially generated crate.

## Diff artifacts

Generate the artifacts by comparing the transformed crate with the downloaded
official service crate after applying the explicit `tests/**` exclusion:

### `DIFF.MD`

Write a deterministic human-readable report containing:

- upstream repository and revision;
- service and model paths;
- output package and library names;
- every generated operation feature and its model operation;
- changed source and manifest files;
- special customizations and their justification;
- an explicit statement that `tests/**` was excluded;
- the exact command needed to reproduce or apply the change.

### `DIFF.diff`

Write a deterministic unified patch suitable for `git apply`. Paths must be
relative to the generated crate and the patch must contain only included source
and manifest files. It must contain no `tests/**` hunks, including test-file
deletions. Do not include either diff artifact inside its own patch.

The diff must be reproducible from the same manifest, model, upstream revision,
and codemod version. Preserve unmodified upstream files byte-for-byte whenever
possible.

## Verification and conformance

Conformance validates the codemod transformation and the generated Cargo
feature configurations. It is not an exact source-parity comparison with the
upstream crate: `#[cfg]` attributes, operation features, manifest renames, and
the removal of `tests/**` are intentional differences. The upstream crate is
the transformation baseline and source for the per-service diff artifacts,
but a non-empty `DIFF.MD` or `DIFF.diff` is not itself a conformance failure.

Do not read checked-in reference trees, normalized projections, reference
patches, canonical `original.rs` files, or snapshot reports during conformance.
Stage the upstream and transformed crates in temporary directories and return
success only when the transformation invariants, feature matrix, diff
artifacts, and coverage report all pass. A conformance failure must leave
existing generated outputs unchanged.

The `aws-sdk-modularizer` CLI must support both the normal transformation and
the verification entry points:

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
cargo run -p aws-sdk-modularizer -- conformance --manifest services-manifest.json
```

`just conformance` must invoke the second command for every service in the
manifest. It must not invoke a service registry, provider crate, second
renderer, or snapshot comparator.

Add focused tests for:

- manifest parsing and revision validation;
- model service and operation discovery;
- Smithy-compatible Rust and feature naming;
- module-graph traversal and `syn` AST rewriting;
- exact `#[cfg(feature = "op_...")] pub mod ...;` output;
- client method, builder, and re-export gating;
- Cargo package/library renaming, default preservation, and feature defaults;
- duplicate, missing, malformed, and unsupported-operation failures;
- test-directory removal and exclusion from both diff formats;
- deterministic output, atomic installation, and reproducible patches;
- isolated Cargo checks for empty, singleton, and multiple-operation feature
  selections.

For each generated service crate, conformance must validate all of the
following:

- The model operation set and the generated `op_*` feature set are equal,
  with exactly one feature for each operation and no unknown operation
  features.
- Every operation module, operation-specific client method, builder/helper,
  re-export, type alias, and retained operation-specific test or example has
  the correct operation gate. Shared items remain available whenever required
  by any enabled operation.
- A normal `cargo check` with the upstream default features compiles with zero
  operation features enabled. This is the zero-operation case; it must not be
  confused with `--no-default-features`, which is a separate optional matrix
  if the generated crate explicitly promises that configuration.
- `cargo check --features op_<operation>` succeeds once for every model
  operation. “One operation feature” means every individual operation is
  checked, not one representative operation.
- Deterministic multiple-operation selections succeed, including the full
  operation set and model-derived groups of operations that share shapes,
  helpers, protocol machinery, or customizations. The exact selections must
  be recorded in the coverage report; do not choose them by hardcoded service
  or operation names.
- Every Cargo invocation for a feature selection is isolated so feature
  unification from another package or test cannot hide a missing gate.
- Public API probes confirm that enabled operations are usable and disabled
  operations are not publicly reachable; source inspection alone is not
  sufficient to prove the latter.
- No generated crate contains `tests/**`, and neither diff artifact contains
  `tests/**` paths or hunks.
- Each diff artifact is deterministic, lists the actual changed files, and
  reconstructs the transformed tree when applied to the excluded upstream
  baseline.

The operation coverage report must define mutually exclusive categories:

- `total`: operations discovered from the selected service shape;
- `transformed`: operations with exactly one source mapping, feature, and
  verified public-surface gate;
- `missing`: operations for which a required mapping, feature, or gate is
  absent;
- `ambiguous`: operations with multiple possible mappings or owners.

For every service, require
`total = transformed + missing + ambiguous`, and require zero `missing` and
zero `ambiguous` operations for conformance success. Coverage must never
decrease. Report the coverage delta after each run, but do not require an
increase for a bug fix, refactor, or documentation-only change that does not
add handling for a previously missing operation.

After every implementation change, run:

```text
just conformance
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
git diff --check
```

All commands above must pass. Record the operation coverage delta after each
run. A documentation-only change, bug fix, or refactor may leave coverage
unchanged, but must still run `just conformance` and report that fact
accurately.

Before declaring a reset or cleanup complete, run `cargo metadata` and targeted
repository searches to verify that removed code, dependencies, workspace
members, examples, fixtures, and documentation have no remaining consumers.
Every commit in the implementation history must have a corresponding status
checkpoint as described above.

## Non-negotiable constraints

- Rust and Cargo only for the codemod; do not invoke Smithy CLI, Java, Kotlin,
  Gradle, Maven, or an external code generator.
- The JSON model drives operation behavior; source inspection only locates the
  model-derived declarations to transform.
- No operation feature is enabled by default.
- No generated service `tests/` directory is retained or included in diffs.
- No hardcoded operation/service transformation branches except documented,
  model-insufficient Smithy-RS special handling.
- Never claim completion from compilation alone; completion requires full
  manifest coverage, passing `cargo check --workspace`, and reproducible
  per-crate diff artifacts.
