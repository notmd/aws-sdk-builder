# Prompt: Rust-native modular AWS SDK generation

This file is the source of truth for the AWS SDK build/codegen rewrite. Follow it as a
complete project specification. Keep the implementation plan and the status/audit log
up to date as work proceeds. Do not treat the existing Smithy-CLI design as current; it
is historical context and must be replaced by this document.

## Goal

Build a Rust-only `aws-sdk-build` build dependency that lets a consumer select AWS
services and operations, then include a generated module with the same public API and
generated source semantics as the AWS SDK for Rust.

The consumer-facing workflow must be:

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_build::configure()
        .add("s3", ["AbortMultipartUpload", "CompleteMultipartUpload"])
        .add("dynamodb", ["GetItem"])
        .compile()?;
    Ok(())
}
```

```rust
// src/lib.rs
aws_sdk_build::include_sdk!();
```

`include_sdk!()` is the preferred facade. It expands to the stable generated
`OUT_DIR/aws_sdk.rs` entry point, so consumers do not need to spell out Cargo's
build-output path. The generated include remains a stable implementation detail and
is still tested directly by the conformance harness. Because a macro invoked from
`src/` must be available to the normal dependency graph, the consumer declares
`aws-sdk-build` in both `[dependencies]` and `[build-dependencies]` when it uses this
facade; the raw include fallback needs only the build dependency.

There must be no consumer-supplied model path, service shape ID, Smithy executable,
Maven coordinate, output directory, or codegen plugin configuration in this API.
`compile()` obtains `OUT_DIR` from Cargo and uses the model and generator shipped by
`aws-sdk-build`.

## Fixed source snapshots

All model and generated-code comparisons must use immutable source snapshots. At the
time this specification was written, the latest `main` commits were:

| Repository | Commit | Purpose |
| --- | --- | --- |
| [`awslabs/aws-sdk-rust`](https://github.com/awslabs/aws-sdk-rust/tree/3c6d526c9d4775f41a8ef1ed2ef574d1b14481db) | `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db` | AWS models and generated SDK golden source |
| [`smithy-lang/smithy-rs`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d) | `f1b64a9c0dd001d4bac4277fec4041da59c1f48d` | Behavioral reference for `aws/codegen-aws-sdk` and its client generator |

Record these SHAs, the snapshot date, every imported model’s SHA-256, and the AWS SDK
crate version used for each golden in a checked-in manifest. Never compare against a
moving branch during tests.

## Hard constraints

- The public library name is `aws_sdk_build`; the package name remains `aws-sdk-build`.
- The only required consumer build-script configuration is repeated
  `.add(service, operations)` calls followed by `.compile()`.
- The `service` argument is the short AWS service key used by the packaged registry,
  for example `"s3"`, `"dynamodb"`, or `"lambda"`.
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
  tree. Models are packaged assets of `aws-sdk-build`.
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
pub fn configure() -> Builder;

impl Builder {
    pub fn add<I, S>(self, service: impl Into<String>, operations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>;

    pub fn compile(self) -> Result<CompileReport, BuildError>;
}
```

`CompileReport` is for diagnostics and tests; the consumer does not need to inspect
it. `add("s3", [])` is the all-operations form. The builder must infer the consumer
crate name and output directory from Cargo rather than requiring `.out_dir(...)` or a
consumer-provided crate-name argument.

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

Move model ownership into `aws-sdk-build`:

```text
crates/aws-sdk-build/
  models/
    s3.json
    dynamodb.json
    ...
  models-manifest.json
```

The package must include the model assets when published. Loading a model means looking
up a registry entry by service key; it must not accept an arbitrary consumer path.
Model loading must support the AWS Smithy JSON AST used by the pinned AWS SDK snapshot,
including shared/prelude shapes, traits, endpoint rules, auth traits, streaming
shapes, event streams, and service-specific traits.

The registry manifest must contain, for every available service:

- short service key;
- AWS service shape ID;
- model filename and SHA-256;
- official `aws-sdk-*` crate name and Rust module name;
- snapshot commit and model version;
- operation names in the source order and canonical sorted order.

The initial implementation may land services in priority tiers, but every landed
service must be completely packaged and selectable. Do not leave a service entry that
silently falls back to a different model version.

## Rust codegen architecture

Port the behavior of the pinned
[`aws/codegen-aws-sdk`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk)
implementation to Rust. The Kotlin source is a behavioral reference, not a runtime
dependency and not a reason to retain a JVM build step.

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
  -> deterministic formatter and output installer
```

Suggested crate modules are guidance, not permission to duplicate logic:

```text
src/
  lib.rs                 # configure, add, compile
  registry.rs            # packaged models and snapshot manifest
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
    services/             # S3, DynamoDB, EC2, Glacier, Route 53, STS, etc.
  output.rs              # module facade, atomic install, manifest
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
unescaped string concatenation. Every generated file must have stable line endings,
stable ordering, and a generator version header.

## Generated output and dependencies

`compile()` must:

1. Read `OUT_DIR` and `CARGO_PKG_NAME` from the build-script environment.
2. Resolve the service selection against packaged models.
3. Generate into a private temporary directory on the same filesystem as `OUT_DIR`.
4. Write `aws_sdk.rs`, one internal Rust source tree per service, and a JSON manifest.
5. Validate the generated Rust syntax and output manifest.
6. Atomically replace only the generator-owned output paths after every validation passes.

The output manifest must record generator version, consumer crate name, snapshot SHA,
selected service keys, selected operations, generated source files, and runtime crate
requirements. It must not contain absolute paths or timestamps that destroy
reproducibility.

The consumer’s handwritten integration is:

```rust
aws_sdk_build::include_sdk!();
```

The macro expands to `include!(concat!(env!("OUT_DIR"), "/aws_sdk.rs"));`. If a
consumer needs to inspect generated output directly, that expansion is the stable
fallback, but normal source should use the macro.

If generated code needs Rust runtime crates, expose the exact required dependency
contract in the generated manifest and the example consumer. The build script cannot
silently edit `Cargo.toml`; either use a documented normal dependency surface or ship
the required runtime source in the build crate and include it. Whichever strategy is
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
aws_sdk_build::configure()
    .add("service-key", std::iter::empty::<&str>())
    .compile()?;
```

The harness must enumerate the model’s complete operation list and assert that every
operation is present in the generated module. It must also run selected-operation
cases, including S3 `AbortMultipartUpload`.

### Exact comparison

Compare the generated source with the pinned AWS SDK source file-by-file and fail on
the first unexpected difference. The only permitted normalization is:

- standalone reference crate root/module anchors versus the consumer-prefixed path;
- the outer `aws_sdk_<service>` wrapper required by `aws_sdk.rs`;
- the generated header’s generator identity, if the comparator explicitly checks the
  header separately.

Do not ignore whitespace, imports in general, docs, ordering, or generated files. A
second AST/token comparison using `syn` should verify public item names, signatures,
attributes, visibility, and nested module paths independent of formatting.

### Markdown diff reports with `diffy`

Conformance must use the Rust [`diffy`](https://docs.rs/diffy/latest/diffy/) crate to
produce the saved source comparison report. Do not shell out to `diff`, `git diff`, or
an external progress package. The reporter should use `diffy::create_patch` for every
changed UTF-8 source file and preserve the resulting unified patch in a fenced
`diff` block.

The report must be deterministic: no timestamps, absolute paths, ANSI color, or
machine-specific headings. Write it only after all selected services have been
compared, preferably by atomically replacing the destination file. A report must
contain a global summary followed by one section for every SDK service, sorted by
service key. The first content line in each service section is its progress line:

```markdown
## s3
**Progress:** `428/428` files compared · `428` matched · `0` mismatches · `0` missing · `0` extra
```

The progress line represents the complete service comparison, including files that
are equal, changed, missing from the reference, or unexpectedly generated. Each
mismatch must include its repository-relative path and a `diffy` unified patch whose
headers identify `reference/...` and `generated/...`. Missing, extra, and binary
files must remain visible as explicit diagnostics rather than being silently skipped.
The report summary must include total files, matched files, mismatches, missing files,
extra files, and read/parse errors. The CLI exits non-zero when differences exist, but
still leaves the complete Markdown report available for review.

The checked-in runner should expose an invocation equivalent to:

```text
cargo run -p aws-sdk-conformance -- \
  --reference conformance/reference \
  --generated conformance/generated \
  --output reports/aws-sdk-conformance.md \
  --snapshot <pinned-aws-sdk-rust-sha>
```

Every conformance run must save its deterministic Markdown output at
`reports/aws-sdk-conformance.md` and commit that summary plus one detailed report per
service under `reports/aws-sdk-conformance/` to version control for reviewable history.
The pinned reference and generated source trees must also be checked in under
`conformance/`, with provenance recorded in `conformance/manifest.json`. The report
must retain non-zero differences; a committed report is evidence of what was compared,
not a claim that parity passed. The report header must identify the pinned AWS SDK Rust
snapshot, and all snapshots/reports must be regenerated when the generator or pinned
reference changes. `just conformance` is the short form of the checked-in command.

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

The script may skip only when explicitly requested with `AWS_SDK_BUILD_SKIP_FLOCI=1`.
An ordinary missing or unhealthy Floci instance is a failed local smoke test, not a
passing test. Floci documents `http://localhost:4566` as its default endpoint and lists
S3, including multipart upload support, among its local services; keep this check to
basic operations so it stays fast and deterministic.

## Initial AWS SDK priority queue

This is an engineering priority queue, not an AWS-published popularity ranking. The
ordering uses broad application usage, frequency in AWS SDK examples and production
architectures, infrastructure centrality, and how well a service exercises distinct
codegen paths. Re-score it when usage data becomes available, but keep the snapshot
and rationale in version control.

| Tier | Service keys, in priority order | Why |
| --- | --- | --- |
| P0 | `s3`, `dynamodb`, `lambda`, `sqs`, `sns`, `sts`, `iam`, `kms` | Core application, storage, identity, messaging, and serverless APIs; broadest initial user value |
| P1 | `cloudwatch`, `cloudwatch-logs`, `ec2`, `ecr`, `ecs`, `eks`, `eventbridge`, `secrets-manager`, `ssm`, `rds` | Common deployment, observability, container, event, secret, parameter, and database workflows |
| P2 | `cloudfront`, `route-53`, `sfn`, `kinesis`, `firehose`, `athena`, `glue`, `redshift`, `cognito-identity-provider`, `sesv2` | High-use delivery, workflow, streaming, analytics, identity, and email services |
| P3 | `bedrock`, `bedrock-runtime`, `textract`, `rekognition`, `opensearch`, `wafv2`, `backup`, `appconfig`, `autoscaling`, `elasticache` | Important emerging, security, AI, and platform services with valuable specialized customizations |

For each tier, record the exact model filename and official crate/module mapping. For
example:

| Service key | Model file | Public module | Example operation path |
| --- | --- | --- | --- |
| `s3` | `s3.json` | `aws_sdk_s3` | `aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload` |
| `dynamodb` | `dynamodb.json` | `aws_sdk_dynamodb` | `aws_sdk_dynamodb::operation::get_item::GetItem` |
| `lambda` | `lambda.json` | `aws_sdk_lambda` | `aws_sdk_lambda::operation::invoke::Invoke` |
| `cloudwatch-logs` | `cloudwatch-logs.json` | `aws_sdk_cloudwatchlogs` | `aws_sdk_cloudwatchlogs::operation::put_log_events::PutLogEvents` |
| `secrets-manager` | `secrets-manager.json` | `aws_sdk_secretsmanager` | `aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValue` |

Do not claim a tier is complete until its all-operation golden comparison and clean
Rust-only consumer compile pass.

## Milestones and stop conditions

Work in small checkpoints. After each checkpoint, run its validation, repair failures,
and update the status/audit markdown before moving on.

- [ ] M1 — Replace the public API. Remove `.model`, `.service`, `.operations`,
  `.out_dir`, `.smithy`, and `.rust_client_codegen`; implement repeated `.add()` and
  `compile()` with Cargo environment discovery and typed diagnostics.
- [ ] M2 — Package the model registry. Add snapshot metadata/checksums, service and
  operation lookup, all-operation selection, and no consumer model inputs.
- [ ] M3 — Port the generic Smithy Rust generator. Cover AST names, shapes, operation
  modules, client/config, builders, and deterministic output on a small fixture.
- [ ] M4 — Port AWS codegen behavior. Add protocols, runtime wiring, endpoint/auth,
  retries, streaming, and the service decorators required by P0.
- [ ] M5 — Add the consumer facade. Generate `aws_sdk.rs`, nested service modules,
  stable public paths, output manifests, atomic installation, and the clean consumer
  example.
- [ ] M6 — Add conformance. Pin references, generate every operation for P0, compare
  source and tokens under the namespace-only normalization, and write deterministic
  `diffy` Markdown reports to the checked-in summary
  `reports/aws-sdk-conformance.md` plus one report per service. Keep the reference and
  generated source trees in `conformance/`, with a completed progress line at the top
  of every service report. Commit each snapshot/report so parity changes remain
  reviewable in git history.
- [ ] M6a — Add the Floci smoke test. Run the basic S3 operation sequence against the
  local emulator and record the endpoint, SDK versions, and result without treating it
  as source-conformance evidence.
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

- OpenAI, [Run long horizon tasks with Codex](https://developers.openai.com/blog/run-long-horizon-tasks-with-codex): use a durable spec, milestone plan, runbook, continuous verification, and status/audit log.
- [Pinned AWS SDK Rust models and generated SDK](https://github.com/awslabs/aws-sdk-rust/tree/3c6d526c9d4775f41a8ef1ed2ef574d1b14481db).
- [Pinned Smithy AWS Rust codegen reference](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk).
- [`diffy` Rust diff library](https://docs.rs/diffy/latest/diffy/), used for deterministic in-memory unified patches.
