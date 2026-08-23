# Prompt: Rust-native modular AWS SDK generation and exact parity

This file is the source of truth for the AWS SDK build/codegen rewrite. Follow it as a
complete project specification and durable runbook. Keep the implementation plan and
the status/audit log up to date as work proceeds. Do not treat the existing Smithy-CLI
design as current; it is historical context and must be replaced by this document.

The project is incomplete until the generated crates match the pinned crates under
`conformance/reference` exactly, subject only to the explicitly documented consumer
namespace/header normalization. A compiling approximation, a working S3 subset, or a
low-difference report is useful intermediate evidence but is not completion.

## Current state (2026-08-21)

The Rust-only implementation is operational but far from exact AWS SDK parity:

- `aws-sdk-builder` exposes the shared compile machinery and the
  `include_sdk!()` facade. The eight service provider crates package exactly one
  Smithy JSON model each: DynamoDB, IAM, KMS, Lambda, S3, SNS, SQS, and STS.
  Build scripts call the relevant provider, such as
  `aws_sdk_builder_s3::compile(["PutObject"])?`.
- The generator emits deterministic Rust source, service/client/config,
  operation/builders/errors, model shapes, an initial local HTTP runtime, and atomic
  output installation. The generator version is
  `aws-sdk-builder-rust-native-0.2.0`.
- The checked-in consumer fixture exercises the seven core S3 operations
  `CreateBucket`, `PutObject`, `HeadObject`, `GetObject`, `ListObjectsV2`,
  `DeleteObject`, and `DeleteBucket` against a deterministic in-process HTTP server.
- All eight checked-in P0 conformance services are regenerated from the packaged JSON
  models: DynamoDB, IAM, KMS, Lambda, S3, SNS, SQS, and STS.
- The latest conformance report compares 8,780 files: 16 exact matches, 1,800
  mismatches, 4,645 missing files, and 2,319 extra files (0.30% arithmetic-average
  match). `just conformance` therefore exits 1 by design until parity is achieved.
- `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  formatting, and the local generated S3 runtime test pass. These checks prove build
  health, not AWS SDK compatibility.
- Full Smithy protocol behavior is still missing or incomplete, including exact
  serializers/deserializers, endpoint rules, SigV4 auth, retries, checksums,
  pagination, presigning, event streams, and complete streaming/error semantics.

The status/audit log is [`docs/aws-sdk-builder-status.md`](docs/aws-sdk-builder-status.md).
Update it whenever a milestone, conformance metric, limitation, or verification result
changes. Never report the project as complete while the conformance report is
non-zero.

## Long-running goal mode and resumable execution

This task is deliberately multi-step. Use a durable goal with a measurable outcome,
explicit constraints, and verification criteria. In Codex CLI or the IDE extension,
start the work by entering `/goal` followed by the trigger prompt below. Keep related
work in the same session so the agent can resume from the existing repository state;
pause the goal before losing connectivity and resume it in the same session afterward.
For hosted ChatGPT work, keep the goal and source files in one project/chat. Use a
separate session only for genuinely independent work that will not write to the same
files.

Paste this to start or resume the project:

```text
/goal
Continue the Rust-native AWS SDK codegen parity project in this repository.

Outcome: make every generated service crate match the pinned AWS SDK Rust crates in
conformance/reference exactly, with only the documented consumer namespace and
generator-header normalization allowed. The current report is intentionally non-zero;
continue from the current files, status log, generated snapshots, and latest report.

Constraints:
- Keep codegen generic and driven by the packaged Smithy JSON model data. Do not add
  service-by-service or operation-name hardcoded branches to the generic generator.
- Make the generator emit valid Rust without invoking `rustfmt`/`cargo fmt` or adding
  custom layout logic. Let the conformance crate apply
  `rustfmt --edition 2021 --config max_width=150,skip_children=true` before source
  comparison, then keep whitespace significant after formatting.
- Use the pinned AWS SDK Rust source and
  https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk
  as behavioral/reference implementations; port behavior to Rust rather than invoking
  Java, Kotlin, Smithy CLI, Gradle, Maven, or a network generator.
- Work in small checkpoints. At the start of each checkpoint read Prompt.md,
  docs/aws-sdk-builder-status.md, git status, and the latest conformance summary.
- After every codegen-affecting change, regenerate the all-operation snapshots and run
  the conformance comparison. Verify that the exact-match count increases and the
  mismatch/missing/extra diff decreases; if it does not, diagnose the regression and
  repair or revert the checkpoint before continuing.
- After each checkpoint run the relevant focused tests, then cargo fmt, cargo test
  --workspace, cargo clippy --workspace --all-targets -- -D warnings, and git diff --check.
- Persist a concise checkpoint in docs/aws-sdk-builder-status.md: what changed, files,
  commands, conformance counts, remaining blocker, and the exact next step. Do not
  claim completion merely because a command compiles or a session is ending.
- Continue autonomously through safe local edits, generation, tests, and report review.
  Ask before destructive actions, external writes, credentials, or scope expansion.

Completion: the conformance command exits 0, every reference file is matched, every
generated file is justified by the exact reference tree or documented namespace
wrapper, all Rust-only consumer/build tests pass, and the status log contains current
reproducible evidence. If context is compacted or the session resumes, re-read the
checkpoint and continue from the recorded next step instead of restarting.
```

At every interruption or context compaction, leave a checkpoint before stopping:

1. Record the current milestone, changed files, commands run, exact conformance
   counts, and next smallest step in `docs/aws-sdk-builder-status.md`.
2. Leave generated snapshots and reports in their deterministic checked-in locations;
   do not summarize away failed comparisons.
3. On resume, inspect the checkpoint, `git diff`, the latest report, and the current
   generated/reference file pair before making a new change.

This workflow follows the current OpenAI guidance for long-running work: define the
outcome, constraints, and verification; keep related work together; and use `/goal`
to pause, resume, and steer a durable task. See the [long-running work guide](https://learn.chatgpt.com/docs/long-running-work).

## Agent memory model

Do not treat the model’s conversational memory or an internal context summary as the
project record. It is working memory and may be shortened or compacted during a long
run. The durable project memory is the repository:

- `Prompt.md` is the immutable specification, constraints, trigger prompt, and
  completion definition.
- `docs/aws-sdk-builder-status.md` is the current checkpoint: milestone, completed
  work, evidence, conformance counts, blockers, and exactly one next action.
- `conformance/manifest.json`, `conformance/summary.md`, and
  `conformance/summary/` are the reproducible parity evidence.
- Git history and the working-tree diff preserve implementation history; never use an
  unrecorded chat statement as the only explanation for a change.

At the beginning of every session, after compaction, and after `/goal` resume, read
those files plus `git status`, the latest report, and the relevant reference/generated
file pair. Reconstruct the plan from the files before editing. Before pausing, write a
checkpoint in this format:

```markdown
### Checkpoint: YYYY-MM-DD — Mx
- State: in progress / blocked / complete
- Changed: files and reusable rule implemented
- Evidence: commands and pass/fail results
- Conformance: before -> after counts for matched, mismatched, missing, extra
- Blocker: exact unresolved issue, or `none`
- Next action: one smallest concrete step
```

Keep the checkpoint concise and current; replace stale next actions instead of
appending a diary. Never put credentials, tokens, or private user data in the prompt,
status file, generated reports, or commit messages. If conversation memory conflicts
with the repository checkpoint, trust the repository and verify it with the code.

## Goal

Build a Rust-only `aws-sdk-builder` build dependency that lets a consumer select AWS
services and operations, then include generated modules that match the pinned AWS SDK
Rust crates in `conformance/reference` exactly: public API, source tree, generated
semantics, protocol behavior, runtime behavior, visibility, documentation, and
formatting, apart from the explicitly permitted consumer namespace/header rewrite.

The generator must be generic. Its input is the packaged Smithy JSON model plus generic
Smithy/AWS traits and protocol metadata; its output is the corresponding Rust SDK
crate. Do not encode one-off behavior with `if service == "s3"`, operation-name
switches, or hand-maintained lists of fields/types. Service-specific behavior must be
represented by model data, protocol traits, or a small declarative customization layer
whose inputs are also derived from the JSON model. A temporary compatibility exception
must be recorded as an explicit failing gap in the status log and must not become the
architecture.

Use the pinned [AWS SDK Rust repository](https://github.com/awslabs/aws-sdk-rust/tree/3c6d526c9d4775f41a8ef1ed2ef574d1b14481db)
and the pinned [Smithy Rust `aws/codegen-aws-sdk` implementation](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk)
as reference implementations. Port their behavior to reusable Rust code; do not
invoke their JVM build, use a Smithy executable, or copy service-specific output by
hand.

The consumer-facing workflow is:

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_builder_s3::compile(["AbortMultipartUpload", "CompleteMultipartUpload"])?;
    Ok(())
}
```

```rust
// src/lib.rs
aws_sdk_builder::include_sdk!();
```

`include_sdk!()` is the preferred facade. It expands to the stable generated
`OUT_DIR/aws_sdk.rs` entry point, so consumers do not need to spell out Cargo's
build-output path. The generated include remains a stable implementation detail and
is still tested directly by the conformance harness. Because a macro invoked from
`src/` must be available to the normal dependency graph, the consumer declares
`aws-sdk-builder` in both `[dependencies]` and `[build-dependencies]` when it uses this
facade; the raw include fallback needs only the build dependency.

There must be no consumer-supplied model path, service shape ID, Smithy executable,
Maven coordinate, output directory, or codegen plugin configuration in this API.
Each service provider's `compile()` obtains `OUT_DIR` from Cargo and passes its
packaged model to the shared generator in `aws-sdk-builder`.

## Fixed source snapshots

All model and generated-code comparisons must use immutable source snapshots. At the
time this specification was written, the latest `main` commits were:

| Repository | Commit | Purpose |
| --- | --- | --- |
| [`awslabs/aws-sdk-rust`](https://github.com/awslabs/aws-sdk-rust/tree/3c6d526c9d4775f41a8ef1ed2ef574d1b14481db) | `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db` | AWS models and generated SDK golden source |
| [`smithy-lang/smithy-rs`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d) | `f1b64a9c0dd001d4bac4277fec4041da59c1f48d` | Behavioral reference for `aws/codegen-aws-sdk` and its client generator |

Record these SHAs, the snapshot date, and the AWS SDK crate version used for each
golden in a checked-in manifest. Never compare against a moving branch during tests.

## Hard constraints

- The public library name is `aws_sdk_builder`; the package name remains `aws-sdk-builder`.
- The only required consumer build-script configuration is a call to the selected
  service provider's `compile(operations)` function.
- The service provider package name identifies the service, for example
  `aws-sdk-builder-s3`, `aws-sdk-builder-dynamodb`, or `aws-sdk-builder-lambda`.
- Operation names are Smithy/AWS operation names, for example `"AbortMultipartUpload"`.
- An empty operation iterator means all operations for that service. This is required
  by the all-operations conformance suite. A separate `.all_operations()` API is not
  needed.
- Repeated service entries are merged deterministically. Duplicate operation names are
  removed. If one entry selects all operations, later narrower entries cannot reduce it.
- Unknown services and unknown operations fail before any output is replaced, with a
  diagnostic containing the requested name and the registry/model source.
- `compile()` is deterministic for the same crate name, service selection, model
  snapshot, generator version, and Rust toolchain.
- No code path may invoke Smithy CLI, Java, Kotlin, Gradle, Maven, a shell, or a
  network downloader. A consumer build must need Cargo and the Rust toolchain only.
- Generated output must not require the consumer to copy Smithy models into its source
  tree. Each supported service model is a packaged asset of its matching
  `aws-sdk-builder-*` provider crate.
- Generated output must not depend on a generated `aws-sdk-*` service crate merely to
  provide the selected service API. Shared Rust runtime dependencies are allowed only
  when they are declared and documented as ordinary Rust dependencies.
- Failed generation must leave the previous `OUT_DIR/aws_sdk.rs` and generated module
  tree intact.
- Do not silently weaken the parity requirement by ignoring large classes of files,
  changing public names, or normalizing generated behavior beyond the explicitly
  allowed namespace prefix.

## Public build API

The builder should expose the following shape (exact generic bounds may be refined
without changing the call site):

```rust
pub fn compile<O: OperationNames>(operations: O)
    -> Result<CompileReport, BuildError>;
```

`CompileReport` is for diagnostics and tests; the consumer does not need to inspect
it. An empty operation array is the all-operations form. Each provider infers the
consumer crate name and output directory from Cargo rather than requiring an output
directory or a consumer-provided crate-name argument.

## Public namespace contract

The include file is a facade containing one module per selected service. For example,
the generated S3 module must make this path available to downstream users:

```rust
use consumer_crate_name::aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload;
```

Rules:

1. Convert the consumer package name to its Rust crate name by replacing `-` with `_`.
2. Convert the service key to the official AWS SDK crate/module name. For `s3`, this
   is `aws_sdk_s3`; for `cloudwatch-logs`, it is `aws_sdk_cloudwatchlogs`.
3. Preserve the official AWS SDK module tree below that prefix, including
   `operation::<snake_case_operation>`, builders, inputs, outputs, errors, model
   modules, protocol modules, and service configuration modules.
4. The source included into the consumer must compile when the generated modules are
   nested below `consumer_crate_name::aws_sdk_<service>`. Internal absolute paths must
   resolve through the generated service module; do not hard-code the build crate’s
   name into generated runtime code.
5. The conformance comparator may rewrite only the crate/module anchor needed to turn
   the standalone AWS SDK crate into the consumer-prefixed module. No type, field,
   operation, trait, serializer, deserializer, error, retry, endpoint, auth, or
   visibility difference is permitted.

`aws_sdk.rs` must be the stable include target. It may include additional files below
`OUT_DIR`, but the consumer must never need to know those paths.

## Packaged model registry

Move model ownership into service provider crates:

```text
crates/aws-sdk-builder-s3/
  model.json
  src/lib.rs
crates/aws-sdk-builder-dynamodb/
  model.json
  src/lib.rs
...
```

Each service package must include exactly its one model asset when published.
The core registry contains metadata only; loading a model means using
the provider selected by the build script, not an arbitrary consumer path.
Model loading must support the AWS Smithy JSON AST used by the pinned AWS SDK snapshot,
including shared/prelude shapes, traits, endpoint rules, auth traits, streaming
shapes, event streams, and service-specific traits.

The registry manifest must contain, for every available service:

- short service key;
- model filename;
- official `aws-sdk-*` crate name and Rust module name;
- snapshot commit and model version;
- operation names in the source order and canonical sorted order.

The initial implementation may land services in priority tiers, but every landed
service must be completely packaged and selectable. Do not leave a service entry that
silently falls back to a different model version.

## Rust codegen architecture

Use [`docs/smithy-codegen-design.md`](docs/smithy-codegen-design.md) as the reusable
architecture summary and targeted source map. Read the pinned `smithy-rs` files only
when the summary identifies the specific abstraction that needs clarification.

Port the behavior of the pinned
[`aws/codegen-aws-sdk`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk)
implementation to Rust. The Kotlin source is a behavioral reference, not a runtime
dependency and not a reason to retain a JVM build step.

### Generic model-driven rule

Every generated service and operation must be produced by the same generic pipeline
from its JSON model. The pipeline must discover operation names, input/output/error
shapes, required members, HTTP method/URI/query/header/payload bindings, protocol,
streaming, auth, endpoint, retry, checksum, pagination, and documentation behavior
from model data and Smithy traits. It must not contain service-specific or
operation-name-specific branches in the generic renderer. In particular, do not use
special cases such as `if operation_name == "ListObjectsV2"` or a dedicated S3
serializer to make one fixture pass. If the AWS reference uses a customization that
cannot be expressed by the shared Smithy traits, add a typed, declarative AWS
customization input and tests for it; keep the renderer generic and make the input
traceable to model metadata.

The reference source is evidence of behavior, not a template to copy selectively. For
each mismatch, identify the model trait, generic Smithy rule, AWS decorator, runtime
rule, or formatting rule that explains the reference output, implement that reusable
rule, regenerate all affected services, and confirm that the aggregate diff shrinks.

The AWS generator depends conceptually on generic Smithy client generation. Port the
minimum required generic layer in Rust as well; do not call a hidden Java/Kotlin
generator. Keep the implementation separated into testable stages:

```text
model registry
  -> Smithy AST parser and shape index
  -> operation/service selection and transitive closure
  -> Rust name and module resolution
  -> generic client/runtime code generation
  -> AWS decorators and service customizations
  -> valid Rust source emitter
  -> conformance formatter/comparator and output installer
```

Suggested crate modules are guidance, not permission to duplicate logic:

```text
src/
  lib.rs                 # shared compile entry point and include macro
  registry.rs            # service metadata and snapshot provenance
  smithy/
    ast.rs               # Smithy JSON AST
    shapes.rs            # shape graph and closure
    names.rs             # Rust and AWS SDK naming rules
  codegen/
    client.rs            # client/config/fluent operation baseline
    shapes.rs            # structures, enums, unions, lists, maps, primitives
    protocols.rs         # rest-json, rest-xml, json, query, ec2-query, event stream
    endpoints.rs         # endpoint rules and built-ins
    auth.rs              # AWS auth and signing configuration
    retries.rs            # retry classifiers and modeled retry traits
    decorators.rs        # AWS-wide decorators
  services/             # model-driven rendering, not a service registry
  output.rs              # module facade and atomic install
```

The implementation must cover the generated API surface used by the reference SDK:

- service `Client`, `Config`, and config builders;
- fluent operation builders and `send()` paths;
- operation structs, input/output/error types, and their builders;
- modeled errors, unhandled errors, error metadata, and conversions;
- structures, lists, maps, unions, enums, timestamps, blobs, documents, and sensitive
  values;
- request serialization and response deserialization for every protocol used by a
  landed service;
- endpoint resolution, auth/signing, retry behavior, checksums, streaming bodies,
  event streams, pagination, presigning, and service-specific customizations where the
  reference crate provides them;
- generated documentation and stable Rust visibility where parity tests cover them.

Prefer structured Rust writers or `proc_macro2`/`syn`-validated token generation over
unescaped string concatenation. Every generated file must be valid Rust with stable
line endings, stable ordering, and a generator version header.

### Formatting ownership

The code generator emits valid Rust source but does not format it. It must never invoke
`rustfmt` or `cargo fmt`, and it must not grow hand-written pretty-printing, line
wrapping, indentation, or width-fitting logic to imitate formatted snapshots. Keep
rendering logic structural and model-driven; let the conformance harness own source
formatting immediately before comparison.

The pinned Smithy-RS client generator is the reference for this boundary:
`ClientCodegenVisitor.kt` finalizes a generated crate and runs
`cargo fmt -- --config max_width=150`. It does not use a client-generator
`.rustfmt.toml`; the two `rustfmt.toml` files in the pinned checkout belong to
unrelated HTTP-server runtime crates and use `max_width = 120`. Because our checked-in
snapshots are individual Rust files without a temporary Cargo manifest, conformance
must apply the equivalent configuration directly to each generated file:

```text
rustfmt --edition 2021 --config max_width=150,skip_children=true <generated-file>
```

Run that normalization in the conformance crate before source comparison, rather than
inside `aws-sdk-builder`. `skip_children=true` prevents per-file formatting from
following nested `mod` declarations that are compared as separate snapshot files.
Formatting is normalization, not permission to ignore whitespace: after formatting,
whitespace, imports, docs, ordering, and generated files remain exact-comparison
inputs.

## Generated output and dependencies

`compile()` must:

1. Read `OUT_DIR` and `CARGO_PKG_NAME` from the build-script environment.
2. Resolve the service selection against packaged models.
3. Generate into a private temporary directory on the same filesystem as `OUT_DIR`.
4. Write only `aws_sdk.rs` and one internal Rust source tree per service.
5. Validate the generated Rust syntax.
6. Atomically replace only the generator-owned output paths after every validation passes.

The generated `OUT_DIR` output must contain only `aws_sdk.rs` and generated Rust source.
Do not write `aws_sdk_builder_manifest.json` or any other generated metadata file. The
`CompileReport` may carry diagnostics such as the consumer crate name and selected
operations in memory, but those values must not be persisted as generated files.

The consumer’s handwritten integration is:

```rust
aws_sdk_builder::include_sdk!();
```

The macro expands to `include!(concat!(env!("OUT_DIR"), "/aws_sdk.rs"));`. If a
consumer needs to inspect generated output directly, that expansion is the stable
fallback, but normal source should use the macro.

If generated code needs Rust runtime crates, expose the exact required dependency
contract in the example consumer and documentation. The build script cannot silently
edit `Cargo.toml`; either use a documented normal dependency surface or ship the
required runtime source in the build crate and include it. Whichever strategy is
chosen, test a clean consumer from a fresh checkout with no Smithy installation.

## Conformance infrastructure

Conformance is a first-class test target, not a smoke test. Build a harness that can
run locally and in CI with no external generator:

### Reference inputs

- Check out the AWS SDK Rust repository at the pinned commit above.
- Use its `aws-models` and generated `sdk/<service>` source as immutable reference
  inputs.
- Never use the latest crates.io release or an unpinned GitHub branch as a golden.

### All-operation generation

For every service in the current priority tier, call:

```rust
aws_sdk_builder_s3::compile(std::iter::empty::<&str>())?;
```

The harness must enumerate the model’s complete operation list and assert that every
operation is present in the generated module. It must also run selected-operation
cases, including S3 `AbortMultipartUpload`.

### Exact comparison

Before comparing, the conformance harness must format generated Rust files with the
pinned Smithy-equivalent configuration described above. Then compare the normalized
generated source with the pinned AWS SDK source file-by-file and fail on the first
unexpected difference. The only permitted normalization beyond that conformance-owned
formatting is:

- standalone reference crate root/module anchors versus the consumer-prefixed path;
- the outer `aws_sdk_<service>` wrapper required by `aws_sdk.rs`;
- the generated header’s generator identity, if the comparator explicitly checks the
  header separately.

Do not strip or otherwise ignore whitespace after formatting, imports in general, docs,
ordering, or generated files. A second AST/token comparison using `syn` should verify
public item names, signatures, attributes, visibility, and nested module paths
independent of formatting.

### Markdown diff reports with `diffy`

Conformance must use the Rust [`diffy`](https://docs.rs/diffy/latest/diffy/) crate to
produce the saved source comparison report. Do not shell out to `diff`, `git diff`, or
an external progress package. The reporter should use `diffy::create_patch` for every
changed UTF-8 source file and preserve the resulting unified patch in a fenced
`diff` block.

The report must be deterministic: no timestamps, absolute paths, ANSI color, or
machine-specific headings. Write it only after all selected services have been
compared, preferably by atomically replacing the destination file. The summary must
contain a global summary followed by a table with one row for every SDK service,
sorted by service key, including a link to that service's detailed report. Each
detailed service report keeps its progress line as the first content line after the
service heading:

```markdown
## s3
**Progress:** `428/428` files compared · `428` matched · `0` mismatches · `0` missing · `0` extra · `100.00%` match
```

The progress line represents the complete service comparison, including files that
are equal, changed, missing from the reference, or unexpectedly generated. Each
mismatch must include its repository-relative path and a `diffy` unified patch whose
headers identify `reference/...` and `generated/...`. Missing, extra, and binary
files must remain visible as explicit diagnostics rather than being silently skipped.
The report summary must include total files, matched files, mismatches, missing files,
extra files, read/parse errors, and the arithmetic average of per-service match
percentages. Each service progress line must include its exact-match percentage;
`100.00%` means every compared file matched. The CLI exits non-zero when differences
exist, but still leaves the complete Markdown report available for review.

The checked-in runner should expose an invocation equivalent to:

```text
cargo run -p aws-sdk-conformance -- \
  --reference conformance/reference \
  --generated conformance/generated \
  --output conformance/summary.md \
  --snapshot <pinned-aws-sdk-rust-sha>
```

Every conformance run must save its deterministic Markdown output at
`conformance/summary.md` and commit that summary plus one detailed report per service
under `conformance/summary/` to version control for reviewable history.
The pinned reference and generated source trees must also be checked in under
`conformance/`, with provenance recorded in `conformance/manifest.json`. The report
must retain non-zero differences; a committed report is evidence of what was compared,
not a claim that parity passed. The report header must identify the pinned AWS SDK Rust
snapshot, and all snapshots/reports must be regenerated when the generator or pinned
reference changes. `just conformance` is the short form of the checked-in command.

### Required codegen feedback loop

Every checkpoint that changes model loading, selection, naming, closure, rendering,
serialization, runtime support, output layout, or generated headers must run codegen
before the next checkpoint:

```text
cargo fmt --all
just conformance
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

Here `cargo fmt --all` formats the repository's handwritten Rust only. It must not be
implemented by, or substituted for, formatting inside the code generator; generated
snapshot formatting belongs to `just conformance`.

`just conformance` is expected to return 1 while differences remain, but its report is
mandatory evidence and must not be discarded. Compare the new report with the previous
checkpoint. The exact-match count must rise and the combined mismatch/missing/extra
count must fall, or the checkpoint must include a concrete explanation and a repair
before more work proceeds. A codegen change is not complete until both the selected
consumer fixture and the all-operation `conformance/generated` tree have been
regenerated. Record the before/after counts and the command result in
`docs/aws-sdk-builder-status.md`.

### Compile and negative checks

For each conformance case:

- `cargo check` the generated consumer with only Rust/Cargo tooling;
- prove the include path is exactly `OUT_DIR/aws_sdk.rs`;
- compile an external-use fixture containing
  `consumer_crate_name::aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload`;
- assert that a selected-operation build does not expose an unselected operation;
- assert that no Smithy CLI, Java, Kotlin, Gradle, Maven, or network process is
  spawned;
- run the test twice and compare output hashes;
- test failed generation against a pre-existing output and verify the old output is
  byte-for-byte unchanged.

Use a clear environment gate only for expensive full-service conformance; the default
unit suite must still cover the registry, API, naming, closure, writer, and atomic
installer without network access.

## Local Floci runtime smoke test

Add a small Rust example at `examples/floci-s3-smoke` and a launcher at
`scripts/check-s3-floci.sh`. This is a runtime/protocol smoke test, separate from
source conformance: it uses the normal Rust AWS S3 client against a developer’s local
Floci endpoint and proves that basic S3 request/response behavior works before the
generated client is complete.

Defaults:

```text
endpoint: http://127.0.0.1:4566
region:   us-east-1
access:   test
secret:   test
```

The launcher must allow `AWS_ENDPOINT_URL`, `AWS_DEFAULT_REGION`,
`AWS_ACCESS_KEY_ID`, and `AWS_SECRET_ACCESS_KEY` to be overridden. It must fail clearly
when the endpoint is unreachable, and it must never use real AWS credentials or a
non-local endpoint by accident. Refuse endpoints that do not resolve to loopback unless
an explicit `ALLOW_NONLOCAL_FLOCI=1` override is set.

The smoke test must create a unique bucket and exercise, at minimum:

1. `CreateBucket`;
2. `PutObject` with a small deterministic payload;
3. `HeadObject` and content-length verification;
4. `GetObject` and byte-for-byte payload verification;
5. `ListObjectsV2` and key verification;
6. `DeleteObject`;
7. `DeleteBucket`.

Always attempt cleanup after a failed assertion. Print the bucket/key and endpoint in
diagnostics, but never print credentials. The example must be runnable with:

```text
scripts/check-s3-floci.sh
```

The expected local emulator is a Docker container exposing port `4566`, for example
`docker compose up -d` or `docker run --rm -d --name floci -p 4566:4566 floci/floci:latest`.
The smoke launcher must not start, stop, or delete the user’s container.

The script may skip only when explicitly requested with `AWS_SDK_BUILDER_SKIP_FLOCI=1`.
An ordinary missing or unhealthy Floci instance is a failed local smoke test, not a
passing test. Floci documents `http://localhost:4566` as its default endpoint and lists
S3, including multipart upload support, among its local services; keep this check to
basic operations so it stays fast and deterministic.

## Supported AWS services

The refactor intentionally keeps only these eight service providers. No other AWS
service is registered in the core or included in the workspace.

| Tier | Service keys | Why |
| --- | --- | --- |
| P0 | `s3`, `dynamodb`, `lambda`, `sqs`, `sns`, `sts`, `iam`, `kms` | Core application, storage, identity, messaging, and serverless APIs |

For each tier, record the exact model filename and official crate/module mapping. For
example:

| Service key | Model file | Public module | Example operation path |
| --- | --- | --- | --- |
| `s3` | `s3.json` | `aws_sdk_s3` | `aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload` |
| `dynamodb` | `dynamodb.json` | `aws_sdk_dynamodb` | `aws_sdk_dynamodb::operation::get_item::GetItem` |
| `lambda` | `lambda.json` | `aws_sdk_lambda` | `aws_sdk_lambda::operation::invoke::Invoke` |
| `iam` | `model.json` | `aws_sdk_iam` | `aws_sdk_iam::operation::create_role::CreateRole` |
| `kms` | `model.json` | `aws_sdk_kms` | `aws_sdk_kms::operation::describe_key::DescribeKey` |
| `sns` | `model.json` | `aws_sdk_sns` | `aws_sdk_sns::operation::publish::Publish` |
| `sqs` | `model.json` | `aws_sdk_sqs` | `aws_sdk_sqs::operation::send_message::SendMessage` |
| `sts` | `model.json` | `aws_sdk_sts` | `aws_sdk_sts::operation::get_caller_identity::GetCallerIdentity` |

Do not claim a tier is complete until its all-operation golden comparison and clean
Rust-only consumer compile pass.

## Milestones and stop conditions

Work in small checkpoints. After each checkpoint, run its validation, repair failures,
and update the status/audit markdown before moving on.

- [x] M1 — Split the public API into the shared `aws-sdk-builder` core and one
  model-provider crate per supported service. Providers expose typed `compile()`
  calls with Cargo environment discovery and deterministic selection merging.
- [x] M2 — Package exactly one model in each of the eight service crates. The core
  registry stores metadata and the service packages contain no conformance
  fixtures or unrelated models.
- [ ] M3 — Port the generic Smithy Rust generator. The current renderer covers a
  useful subset of AST names, shapes, operation modules, client/config, builders, and
  deterministic output, but it still requires broad generic expansion and exact
  parity work.
- [ ] M4 — Port AWS codegen behavior. Initial local HTTP runtime wiring exists, but
  protocols, endpoint/auth, retries, streaming, and the service decorators required
  by P0 are not complete.
- [ ] M5 — Complete the consumer facade. `aws_sdk.rs`, nested service modules, stable
  public paths, atomic installation, and the consumer example exist;
  the generated API and runtime are not yet reference-equivalent.
- [ ] M6 — Complete conformance. References are pinned, every operation for the
  current P0 set is generated, and deterministic `diffy` Markdown reports are checked
  in, but source/token parity is still far from complete. Continue to compare
  source and tokens under the namespace-only normalization, and write deterministic
  `diffy` Markdown reports to the checked-in summary
  `conformance/summary.md` plus one report per service. Keep the reference and
  generated source trees in `conformance/`, with a completed progress line and match
  percentage at the top of every service report. Commit each snapshot/report so
  parity changes remain reviewable in git history.
- [ ] M6a — Complete the Floci smoke test. The launcher and Rust example exist, but a
  current live operation sequence is not completion evidence yet. Run the basic S3
  operation sequence against the local emulator and record the endpoint, SDK versions,
  and result without treating it as source-conformance evidence.
- [ ] M7 — Expand P1–P3. A service advances only after its full operation list passes
  the same parity and Rust-only build gates.
- [ ] M8 — Remove obsolete CLI code/docs and run the full audit. No stale API or
  Smithy-CLI prerequisite may remain in user-facing documentation.

Required verification gates at every relevant milestone:

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

The final completion claim requires the full conformance command, a clean consumer
build, an unchanged-output failure test, and a checked-in snapshot/priority audit.
If any of those is unavailable, report the exact missing evidence instead of marking
the project complete.

## Source references

- OpenAI, [Long-running work](https://learn.chatgpt.com/docs/long-running-work): use a durable `/goal` with a clear outcome, constraints, verification criteria, and resumable checkpoints in the same session.
- OpenAI, [Model guidance](https://developers.openai.com/api/docs/guides/latest-model): define autonomy/approval boundaries, name safe local actions, and track context during long sessions.
- OpenAI, [Compact a response](https://developers.openai.com/api/reference/java/resources/responses/methods/compact): compaction supports long conversations, but compacted state is opaque; preserve critical project state in repository files.
- [Pinned AWS SDK Rust models and generated SDK](https://github.com/awslabs/aws-sdk-rust/tree/3c6d526c9d4775f41a8ef1ed2ef574d1b14481db).
- [Pinned Smithy AWS Rust codegen reference](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk).
- [`diffy` Rust diff library](https://docs.rs/diffy/latest/diffy/), used for deterministic in-memory unified patches.
