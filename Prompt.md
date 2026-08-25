# Rust-native AWS SDK codegen parity

`Prompt.md` is the source of truth for the Rust-only AWS SDK generator. The
implementation is complete only when every supported service matches the pinned AWS
SDK Rust source in `conformance/reference`, apart from the explicitly documented
module-anchor and generator-header normalization. Keep current evidence and
checkpoint history in `docs/aws-sdk-builder-status.md`; do not duplicate volatile
counts here.

## Parity priorities

Prioritize compile-valid, executable semantic parity first: generated Rust types, public
APIs, request and response bindings, serialization, deserialization, errors, and runtime
behavior. Ordering-only differences and documentation-only diffs are both last-priority
work. Keep them visible in reports, but defer them whenever any semantic code mismatch
remains; a formatting, ordering, or documentation match must never mask an executable
behavior difference.

## Objective and scope

Build a generic `aws-sdk-builder` build dependency that lets a consumer select a
service and operations, then include the generated API as Rust modules. Generation
must be driven by packaged Smithy JSON models and reusable Smithy/AWS rules. Do not
invoke Smithy CLI, Java, Kotlin, Gradle, Maven, a shell, or a network generator.

The initial supported services are `dynamodb`, `iam`, `kms`, `lambda`, `s3`, `sns`,
`sqs`, and `sts`. Every registered service must have exactly one packaged model and
must be tested against its complete pinned reference tree.

The reference implementations are the pinned AWS SDK Rust repository and the pinned
Smithy-RS checkout. Always inspect the Smithy-RS reference at:

```text
/tmp/smithy-rs
commit: f1b64a9c0dd001d4bac4277fec4041da59c1f48d
```

The pinned AWS SDK snapshot is:

```text
awslabs/aws-sdk-rust
commit: 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db
```

## Required migration: one artifact per service

Consumer and conformance must use the same generator and implementation. Remove
`consumer_namespace`, `consumer_crate`, and `relative_snapshot_paths` from the
rendering pipeline. Do not replace them with another consumer/conformance mode flag.

### Architecture decision and migration scope

This project deliberately uses one canonical, per-service `original.rs` for both
consumers and conformance. Conformance obtains its split files by transforming that
artifact; it must never invoke a second renderer or compare a consumer-specific
output. This decision is permanent unless this document is intentionally changed.

The current migration is an architecture refactor, not a new parity sprint. Before
editing, record the current `conformance/summary.md` as the baseline. The current
baseline is approximately 4,904 matched of 6,396 files, with 724 mismatches, 713
missing, and 55 extra files. The migration is successful only when coverage is
baseline-equivalent: exact matches do not decrease and mismatch/missing/extra counts
do not increase. Do not mix unrelated protocol or service parity work into this
migration. `just conformance` may continue to exit 1 because the baseline itself is
not fully conformant.

Generate one canonical large Rust file for each service:

```text
OUT_DIR/generated/<service>/original.rs
```

The service provider's `include_sdk!()` macro must include exactly its own
`original.rs`. Conformance must preserve the same artifact at:

```text
conformance/generated/<service>/original.rs
```

`original.rs` is the only generated implementation and the source of truth. It must
be valid when included at the consumer crate root or inside any caller-owned wrapper.
All generated references must be resolved from the Rust module tree; they may not
hard-code the consumer crate name, wrapper name, or renderer mode.

### Consumer contract

The intended API is:

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_builder_s3::compile(["PutObject", "GetObject"])?;
    Ok(())
}

// src/lib.rs
mod s3_import {
    aws_sdk_builder_s3::include_sdk!();
}
```

An empty operation iterator selects every operation. Repeated selections merge
deterministically; duplicates are removed, and selecting all cannot later be narrowed.
Unknown services or operations fail before replacing existing output. The provider
gets `OUT_DIR` and the Cargo package name from the build environment; callers do not
provide model paths, output paths, crate names, shape IDs, or generator commands.

The provider owns the macro, while the consumer owns the wrapper module. There is no
generated aggregate `aws_sdk.rs` facade. The generated module tree must preserve the
official service API, including clients, config, operations, builders, shapes,
errors, protocol modules, and service configuration.

## Canonical artifact and normalized projection

Because the pinned reference is split across files, conformance derives a comparison
projection from each canonical artifact:

```text
conformance/generated/<service>/original.rs
conformance/generated/<service>/normalized/src/lib.rs
conformance/generated/<service>/normalized/src/config.rs
conformance/generated/<service>/normalized/src/types.rs
conformance/generated/<service>/normalized/src/types/<shape>.rs
conformance/generated/<service>/normalized/src/operation/<operation>.rs
...
```

The projection must be produced by a module-aware `syn` transform or a shared
module-tree writer. Never split source with regular expressions or ad-hoc string
rewrites. Recursively materialize every inline module at its Rust module path and
retain the parent `mod`/`pub mod` declaration.

The transform must preserve:

- item order, visibility, attributes, documentation, and inner attributes;
- relative `super::` paths, macro scope, and nested module hierarchy;
- `include!`, `include_bytes!`, `file!`, `module_path!`, and macro behavior, or reject
  the projection when preservation cannot be proven.

Reject invalid Rust, duplicate module paths, unsupported inline constructs, and any
projection that is not deterministic. Check that the original and normalized forms
compile with the same public module paths, item names, signatures, visibility,
attributes, and token structure. The projection is a comparison view, never a second
codegen mode and never a way to hide semantic differences.

The required flow is:

```text
generate one original.rs per service
        ├── include original.rs in the consumer fixture
        └── split original.rs into normalized/src/**
                    └── rustfmt normalized files, then compare with reference/<service>
```

## Generic codegen rules

Use one model-driven pipeline for every service and operation. Do not add branches
such as `service == "s3"` or `operation == "ListObjectsV2"` to make a fixture pass.
Service-specific behavior must come from Smithy traits, protocol metadata, model
data, or a typed declarative AWS customization whose input is model-traceable.

The reusable stages are:

```text
packaged model
  -> Smithy AST and shape index
  -> selection and transitive closure
  -> Rust names and module paths
  -> generic client, shape, protocol, endpoint, auth, retry, and runtime generation
  -> AWS decorators/customizations
  -> valid canonical Rust source
  -> conformance projection, formatting, comparison, and atomic installation
```

The generated API must cover the reference behavior for clients/configuration,
operation builders, modeled and unhandled errors, structures/lists/maps/unions/enums,
timestamps/blobs/documents/sensitive values, all landed protocols, serialization and
deserialization, endpoint rules, auth/signing, retries, checksums, streaming,
pagination, presigning, event streams, documentation, and stable visibility.

Prefer structured Rust writers or validated `proc_macro2`/`syn` output over fragile
string concatenation. The generator emits valid, deterministic Rust with stable
ordering and a generator header, but does not run `rustfmt` or implement pretty-
printing. Formatting belongs to conformance.

### Smithy-RS reverse-engineering guide

[`docs/smithy-codegen-design.md`](docs/smithy-codegen-design.md) is the checked-in
reverse-engineering guide and must be read before extending the generator. It maps
the pinned Smithy-RS Kotlin implementation to the Rust abstractions needed here. Use
the local `/tmp/smithy-rs` checkout at the pinned commit for targeted source and test
inspection; do not invoke its JVM build or copy generated service output by hand.
The checkout is a full local source reference, not a generated-output cache. If it is
missing, create it once without using GitHub API enumeration:

```text
git clone https://github.com/smithy-lang/smithy-rs /tmp/smithy-rs
git -C /tmp/smithy-rs checkout --detach f1b64a9c0dd001d4bac4277fec4041da59c1f48d
```

If the directory already exists, preserve it and verify its remote and commit rather
than deleting or recloning it.

The upstream Smithy language repository is a second, distinct local reference at
`/tmp/smithy`. Use it for Smithy specification, model, trait, protocol, transform,
and validation behavior; use `/tmp/smithy-rs` for the Rust codegen implementation.
If it is missing, create it once and record/verify the checkout commit:

```text
git clone https://github.com/smithy-lang/smithy /tmp/smithy
git -C /tmp/smithy rev-parse HEAD
```

The currently available `/tmp/smithy` checkout is at
`0f7323128b0606a1b94b1ac482c94d3800a22708`. Preserve an existing checkout rather than
deleting or recloning it.

Trace behavior through these reference boundaries:

| Concern | Smithy-RS source map | Rust responsibility |
| --- | --- | --- |
| Context and model transforms | `CodegenContext`, `OperationNormalizer` | Indexed model, ordered transforms |
| Reachability and naming | `DirectedWalker`, `RustSymbolProvider`, `ModuleProvider` | Shape closure, symbols, module paths |
| Files and dependencies | `RustCrate`, `RustWriter`, `RustModule`, `CodegenDelegator` | Canonical writers, imports, re-exports |
| Shapes and operations | `SchemaGenerator`, `StructureGenerator`, `BuilderGenerator`, `OperationGenerator` | Generic Rust item and operation plans |
| Protocol/client pipeline | `ProtocolLoader`, `HttpBindingResolver`, `ClientCodegenVisitor` | Protocol bindings, runtime, serializers |
| Services and customization | `ServiceGenerator`, `ClientCodegenDecorator`, `AwsCodegenDecorator` | Ordered generic/AWS hooks |
| Service extension example | `aws/.../customize/s3/S3Decorator.kt` | Conditional model-driven decorator |

When diagnosing a mismatch, classify it as model/index, transform, closure,
symbol/module, shape, protocol, runtime, decorator, dependency, documentation,
formatting, or installation. Then inspect the corresponding reference abstraction and
tests, implement the reusable Rust rule, add a focused test, regenerate all services,
and record the result. Do not patch a literal operation name or add a generic renderer
branch merely because one snapshot differs. Service IDs are allowed only in isolated,
model-justified conditional decorators. Preserve the separation:

```text
Smithy model -> normalized IR -> closure -> symbols/modules -> generic generators
             -> ordered AWS decorators -> canonical original.rs -> normalized projection
```

The guide is architectural memory for future agents; do not remove it during prompt
compaction. Update it when the Rust port establishes a new reusable mapping.

### Packaged models and output installation

Each provider owns its model asset, for example:

```text
crates/aws-sdk-builder-s3/model.json
crates/aws-sdk-builder-s3/src/lib.rs
```

The core registry stores service metadata and snapshot provenance. Model parsing must
support the pinned Smithy JSON AST, shared/prelude shapes, traits, protocols,
endpoints, auth, streaming, event streams, and service traits.

`compile()` must read Cargo environment metadata, stage output in a temporary directory
on the same filesystem, validate generated Rust, then atomically replace only the
generator-owned `generated/<service>/original.rs` paths. Failed generation must leave
previous output byte-for-byte intact. Do not emit aggregate facades or public
selection metadata in `OUT_DIR`; conformance-only normalized files stay under
`conformance/generated`.

## Conformance

For every supported service, generate all operations by calling the provider with an
empty operation iterator, and also test selected-operation builds. Assert that every
modeled operation appears in all-operation output and that unselected operations do
not appear in selected output.

The harness must:

1. preserve `conformance/generated/<service>/original.rs`;
2. derive and validate `normalized/src/**` from that exact file;
3. format only normalized Rust files with the pinned equivalent of Smithy-RS:

   ```text
   rustfmt --edition 2021 --config max_width=150,skip_children=true <file>
   ```

4. compare the normalized tree file-by-file with the pinned reference tree;
5. save deterministic reports to `conformance/summary.md` and
   `conformance/summary/<service>.md`.

The comparator may apply only these explicit reference-side normalizations:

- the standalone reference crate/module anchor needed for the consumer-owned module;
- checked-in source patches for the documented global-import adjustment;
- generator-header identity, if checked separately.

Do not normalize away whitespace after formatting, imports, docs, ordering, files,
visibility, or behavior. Missing, extra, invalid, binary, and parse-error cases must
remain explicit. Use `diffy::create_patch` for changed UTF-8 files; do not shell out
to `diff` or `git diff`. Reports must be deterministic and include per-service and
global totals for compared, matched, mismatched, missing, extra, and error counts.
The command exits non-zero while differences remain, but must still write the report.

The short command is:

```text
just conformance
```

It must regenerate the all-operation snapshots, preserve the originals, derive the
normalized trees, format them, compare them, and leave the report available. For every
codegen-affecting checkpoint, record before/after exact-match and mismatch/missing/
extra counts in the status log. During the canonical-artifact migration, coverage must
remain baseline-equivalent; improvement is optional and unrelated parity work is out
of scope. A documentation-only change may leave coverage unchanged and should say so
explicitly.

## Verification and completion

After every codegen-affecting change run:

```text
cargo fmt --all
just conformance
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

For documentation-only edits, at minimum run `git diff --check` and the mandatory
`just conformance`; do not claim that documentation increased coverage. Keep focused
tests for model loading, naming, selection/closure, module splitting, deterministic
output, atomic installation, public API, and clean consumer compilation. Also test
that no external generator process is spawned and failed generation preserves old
output.

Migration completion requires consumer and conformance to use the same per-service
`original.rs`, normalized output to be proven equivalent to each original artifact,
consumer and all-operation builds to pass with Rust/Cargo only, and the conformance
report to remain baseline-equivalent. The command may exit 1 while those existing
parity gaps remain. Full project completion is separate and still requires
`just conformance` to exit 0 with every reference file matched.

## Durable checkpoints

At the start or resume of a checkpoint, read this file, the status log, `git status`,
and the latest conformance summary. Preserve unrelated user changes. After each
meaningful checkpoint update `docs/aws-sdk-builder-status.md` with:

```markdown
### Checkpoint: YYYY-MM-DD — Mx
- State: in progress / blocked / complete
- Changed: files and reusable rule
- Evidence: commands and pass/fail results
- Conformance: before -> after matched/mismatched/missing/extra
- Blocker: exact issue, or `none`
- Next action: one smallest concrete step
```

If work is interrupted or context is compacted, leave this checkpoint and resume from
the repository state, generated snapshots, reports, and next action. Never claim
completion merely because code compiles or a session ends. Never put credentials or
private data in repository prompts, reports, or commit messages.

## Milestones

- [x] M1 — Split the shared builder from the eight service providers.
- [x] M2 — Package exactly one model in each provider.
- [ ] M3 — Complete generic Smithy model, naming, shape, and module generation.
- [ ] M4 — Complete AWS protocols, endpoints, auth, retries, streaming, and decorators.
- [ ] M5 — Finish the consumer contract and canonical per-service `original.rs` path.
- [ ] M6 — Derive normalized trees from originals without regressing the current
  conformance baseline; exact parity remains a later project goal.
- [ ] M6a — Run the local S3 emulator smoke test as separate runtime evidence.
- [ ] M7 — Expand beyond the current P0 services only after full parity gates pass.
- [ ] M8 — Remove obsolete CLI documentation/code and complete the audit.

The S3 local smoke test is secondary to source conformance. When enabled, it should
exercise basic create/put/head/get/list/delete operations against a loopback Floci
endpoint and never use real credentials or a non-local endpoint accidentally.

## Copy-paste migration prompt

```text
/goal
Migrate this Rust AWS SDK codegen project to one canonical generated artifact per service.

Read Prompt.md, docs/smithy-codegen-design.md, docs/aws-sdk-builder-status.md, git
status, the latest conformance summary, and the pinned Smithy-RS reference in
/tmp/smithy-rs first. Preserve unrelated user changes. Use the Smithy-RS source map
to reverse-engineer behavior before implementing a rule.

Generate exactly:
  OUT_DIR/generated/<service>/original.rs

The provider's include_sdk!() must include that service's original.rs. Consumer and
conformance must use this same implementation. Remove consumer_namespace,
consumer_crate, and relative_snapshot_paths from the renderer; do not add another
mode flag. Resolve all generated paths from the Rust module tree so the file compiles
at crate root and below any caller-owned wrapper.

For each service, preserve:
  conformance/generated/<service>/original.rs

Then derive normalized/src/** by recursively splitting inline modules with syn or a
shared module-tree writer. Preserve module paths, visibility, attributes, docs, item
order, macro scope, and relative paths; reject invalid or ambiguous transforms. Format
only normalized files with rustfmt --edition 2021 --config max_width=150,skip_children=true,
then compare them file-by-file with conformance/reference/<service> using only the
checked-in reference-side patches and documented module/header normalization.

Add tests for consumer inclusion, no renderer-mode branching, deterministic splitting,
original/normalized public-API equivalence, selected/all-operation behavior, and
atomic failure preservation. After each codegen checkpoint run cargo fmt --all,
just conformance, cargo test --workspace, cargo clippy --workspace --all-targets
-- -D warnings, and git diff --check. Record exact before/after conformance counts and
the next action in docs/aws-sdk-builder-status.md. Preserve the baseline: exact
matches must not decrease and mismatch/missing/extra counts must not increase.
Do not pursue unrelated parity improvements in this migration.

Completion means the consumer and conformance use the same original.rs per service,
the normalized projection is equivalent, the current conformance baseline is
preserved, the consumer build passes, and the status log contains reproducible
evidence. `just conformance` may remain non-zero because existing parity gaps are
outside this migration.
```

## References

- [Pinned AWS SDK Rust source](https://github.com/awslabs/aws-sdk-rust/tree/3c6d526c9d4775f41a8ef1ed2ef574d1b14481db)
- [Pinned Smithy-RS AWS codegen](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk)
- [Smithy/codegen design notes](docs/smithy-codegen-design.md)
- [Status and audit log](docs/aws-sdk-builder-status.md)
- [`diffy` API](https://docs.rs/diffy/latest/diffy/)
