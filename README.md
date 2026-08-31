# better-aws

Rust tooling that transforms pinned AWS SDK service crates into modular crates
with one opt-in Cargo feature per operation. The full requirements are in
[`Prompt.md`](Prompt.md).

The codemod is driven by `services-manifest.json` and each service's Smithy
model from the downloaded SDK archive. It downloads the pinned AWS SDK source
into a temporary directory, uses `syn` to modify the Rust module graph, updates
`Cargo.toml`, and writes outputs such as:

The `aws-sdk-modularizer` package is the workspace root package and its CLI
sources live under `src/`.

```text
crates/aws_sdk_s3/
├── Cargo.toml
├── src/
├── DIFF.MD
└── DIFF.diff
```

For an operation named `HeadBucket`, the generated crate declares
`op_head_bucket` and gates the public module as:

```rust
#[cfg(feature = "op_head_bucket")]
pub mod head_bucket;
```

Operation features are never added to Cargo's default feature list. Generated
service crates omit their upstream `tests/` directories, and those paths are
excluded from both diff artifacts.

## Commands

Run the complete manifest-driven validation loop with:

```text
just conformance
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
git diff --check
```

The conformance command reports operation coverage and validates feature gates,
test exclusion, generated crate compilation, and reproducible diffs.

## Prompt for the next change loop

Copy and send this prompt when starting the next implementation change:

```text
/goal Read Prompt.md, README.md, services-manifest.json,
docs/aws-sdk-modularizer-status.md, the current git status, and the local
/tmp/smithy-rs checkout before changing anything.

Continue the AWS SDK per-operation modularizer from the current repository state.
Choose the smallest concrete missing behavior or failing coverage case. Keep the
implementation generic and driven by the manifest and Smithy model JSON. Use
syn for Rust AST/module changes and a structural TOML parser for Cargo.toml.
Do not revive the old builder, snapshot, projection, or compatibility pipeline.
Clean up obsolete code, dependencies, workspace members, fixtures, examples,
and documentation after proving they are no longer referenced; do not leave
dead compatibility shims behind.
Do not hardcode service or operation names unless the model lacks the required
information and the matching Smithy-RS special handling is documented and
tested.

For each service, preserve the manifest-defined package/library/output names,
add one non-default op_<snake_case_operation> feature per model operation, and
gate the complete public operation surface. Strip the generated service crate's
tests/ directory and exclude tests/** from DIFF.MD and DIFF.diff.

After the change, run all of these and fix failures rather than stopping early:

  just conformance
  cargo check --workspace
  cargo test --workspace
  cargo fmt --all -- --check
  git diff --check

Confirm that operation coverage increases or, for a documentation-only change,
explicitly record that coverage is unchanged. Confirm cargo check --workspace
passes, no generated service crate contains tests/, and neither diff artifact
contains tests/ hunks. After every commit, append a checkpoint to
docs/aws-sdk-modularizer-status.md with the commit hash/date, objective, changed
files and generic rule, command results, coverage delta, blocker, and one next
action. Summarize the changed generic rule, verification results, coverage
delta, cleanup performed, and any remaining blocker.
```
