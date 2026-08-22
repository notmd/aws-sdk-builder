# Smithy Rust reverse-engineering notes

This is a concrete behavior ledger for future work on `aws-sdk-build`. It records what
current Smithy Rust code does, where behavior lives, and how each rule maps to the Rust
port. It complements [`smithy-codegen-design.md`](smithy-codegen-design.md), which is the
architecture summary.

## Reference snapshots

Two snapshots matter. Do not mix them:

- **Parity pin:** Smithy Rust commit
  [`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d).
  `Prompt.md`, conformance work, and golden behavior use this pin.
- **Inspection mirror:** `/tmp/smithy-rs`, release `1.1.7`, commit
  `56ee88c5c6edd967967656f1e29f46b229105e79` (`release-2026-08-21`). This mirror is
  useful for reading current source and dependency boundaries. It is not a replacement
  conformance pin.

Inspection mirror build versions:

- Smithy model/codegen dependencies: `1.73.0`.
- Smithy Rust codegen publication: `0.1.25`.
- Stable runtime crates: `1.1.7`.
- Unstable runtime crates: `0.60.6`.
- Kotlin: `2.1.0`.
- Rust MSRV recorded by upstream: `1.94.1`.

Primary upstream dependency declarations:

- `codegen-core/build.gradle.kts`: Smithy codegen core, `toml4j`, AWS traits,
  protocol-test traits, waiters, protocol traits, and local Rust runtime.
- `codegen-client/build.gradle.kts`: `codegen-core`, rules engine, waiters, protocol
  traits, and runtime test dependencies.
- `aws/codegen-aws-sdk/build.gradle.kts`: `codegen-core`, `codegen-client`, AWS
  traits, rules engine, AWS endpoints, smoke-test traits/model, and AWS runtime.
- `gradle/libs.versions.toml`: exact Smithy library version and module names.

### Codegen-logic dependencies outside `smithy-rs`

Runtime crates are not part of this question. Smithy Rust's Kotlin generator owns most
rendering and customization logic, but it relies heavily on Java Smithy libraries from
separate Smithy repositories and Maven artifacts:

- `software.amazon.smithy:smithy-model`: `Model`, shapes, traits, `ModelTransformer`,
  `Walker`, `ServiceIndex`, `TopDownIndex`, `OperationIndex`, `HttpBindingIndex`,
  `NullableIndex`, selectors, nodes, and model loading. This is largest semantic
  dependency.
- `software.amazon.smithy:smithy-codegen-core`: `Symbol`, `SymbolProvider`,
  `WriterDelegator`, `SymbolWriter`, dependency containers, and `CodegenException`.
- `software.amazon.smithy:smithy-aws-traits`: AWS protocol/auth/checksum/service/S3
  traits consumed by AWS decorators and protocol selection.
- `software.amazon.smithy:smithy-protocol-traits`: protocol traits such as RPC V2 CBOR.
- `software.amazon.smithy:smithy-protocol-test-traits`: modeled HTTP protocol test
  cases and test trait accessors.
- `software.amazon.smithy:smithy-waiters`: waiter model/index APIs used by client
  generation.
- `software.amazon.smithy:smithy-rules-engine`: endpoint rule AST/evaluation types.
- `software.amazon.smithy:smithy-aws-endpoints`: AWS endpoint rule data/functions.
- `software.amazon.smithy:smithy-build` and Smithy Gradle plugins: plugin context,
  file manifests, model projections, and build entrypoint. Mostly integration, but
  needed to invoke upstream generator.

AWS SDK generation additionally uses `smithy-aws-iam-traits`,
`smithy-aws-cloudformation-traits`, smoke-test traits/models, and protocol-test
artifacts. These mostly feed service discovery, endpoint/auth metadata, or test
coverage; they do not replace the core renderer.

Utility dependencies with observable output impact:

- Jsoup: documentation/HTML handling.
- Jackson CBOR: CBOR model/protocol test handling.
- Toml4j: Cargo manifest parsing/rendering support.

Kotlin stdlib, Gradle, JUnit/Kotest, Ktlint, and JReleaser support build/test/publish
workflow, not Smithy model semantics. No separate hidden code generator appears in
`smithy-rs`; semantic behavior comes from these Smithy Java APIs plus Kotlin source in
`codegen-core`, `codegen-client`, and `aws/codegen-aws-sdk`.

## External Smithy API behavior

Source mirror: `/tmp/smithy`, commit `0f7323128b0606a1b94b1ac482c94d3800a22708`.
Maven artifact version used by the inspection mirror: `1.73.0`. These APIs are not
runtime dependencies of the consumer. They are semantic inputs to code generation and
must be represented by Rust code when JVM execution is forbidden.

### `smithy-model`: immutable model plus cached knowledge

Sources: `smithy-model/src/main/java/software/amazon/smithy/model/Model.java`,
`transform/ModelTransformer.java`, `loader/ModelAssembler.java`.

- `Model` stores shape IDs, shapes, metadata, and a lazy trait cache. Shape lookup is
  exact by `ShapeId`; `expectShape` fails with a diagnostic instead of returning a
  fallback.
- Shape/type/category sets are immutable views backed by caches. The backing maps and
  sets are not a canonical generation order. Generators must impose ordering where
  output order matters.
- Knowledge indexes are memoized in a model blackboard. `getKnowledge` constructs an
  index outside `ConcurrentHashMap.computeIfAbsent` because indexes recursively request
  other indexes. `putIfAbsent` makes concurrent callers converge on one instance.
- `Model.assembler()` loads Smithy IDL/JSON, imported files/JAR models, manually added
  shapes/traits/models, metadata, and the prelude. It discovers trait providers and
  validators through `ServiceLoader`.
- Assembly applies the 1.0-to-2.0 model transform before full validation. Unknown traits
  can fail, warn, or be quietly accepted depending on assembler properties. Validation
  events aggregate and may be emitted concurrently.
- `ModelTransformer` operations rebuild a consistent model, not an in-place JSON map.
  `replaceShapes`, `removeShapes`, `renameShapes`, `mapShapes`, `mapTraits`, and
  `filterTraits` preserve shape references and reject invalid trait targets. Transformer
  plugins also load through `ServiceLoader`.

Rust consequence: raw `serde_json::Value` access is not equivalent to Smithy's model
API. Port must add exact shape IDs, typed traits, metadata, source/member order, model
transforms, unknown-trait policy, and structured diagnostics.

### Knowledge indexes provide normalized semantics

Sources under `smithy-model/.../knowledge/`.

- `OperationIndex` resolves operation input/output/error structures and reverse bindings.
  It treats `smithy.api#Unit` as no meaningful input/output, preserves member order in
  returned member maps, and does not validate broken references itself.
- `HttpBindingIndex` computes explicit and implicit HTTP bindings. Binding priority is
  header, prefix header, request query/query params, payload, request label, response
  code, then unbound members. Unbound members become document bindings when no payload
  exists; with a payload they remain `UNBOUND`.
- `HttpBindingIndex` also owns response status resolution, body detection, content type,
  and timestamp defaults. Timestamp defaults are `HTTP_DATE` for headers/prefix headers,
  `DATE_TIME` for query/labels, and protocol-provided default for document/payload.
  Payload content type derives from event stream, document-like target, `@mediaType`,
  blob, or string in that order.
- `ServiceIndex` returns protocol traits in deterministic `TreeMap` order. Effective auth
  follows the modeled `auth` trait order; without it, auth definitions sort by shape ID.
  `NO_AUTH_AWARE` injects synthetic `smithy.api#noAuth` when effective auth is empty.
- `NullableIndex` has distinct `CLIENT`, `CLIENT_CAREFUL`, `SERVER`, and Smithy 1.0
  zero-value modes. Client mode honors `InputTrait` and `ClientOptionalTrait`; careful
  mode treats structure/union targets as optional; server mode uses required/default
  traits. Union/set members and map keys are never nullable; list/map values are nullable
  only for sparse containers.
- `TopDownIndex` traverses only resource and operation-binding relationships, then keeps
  both insertion-order and shape-ID-sorted closures. `ServiceGenerator` uses sorted
  operations.
- `PaginatedIndex` merges service-level and operation-level pagination traits, resolves
  nested token/items paths, and emits pagination info only when input and output token
  bindings are valid.

These indexes explain why protocol code must not independently inspect member traits in
arbitrary order. Rust port needs one normalized binding/nullability/index layer shared by
all serializers, parsers, builders, endpoint generation, and docs.

### `smithy-codegen-core`: symbols are dependency-bearing declarations

Sources under `smithy-codegen-core/src/main/java/software/amazon/smithy/codegen/core/`.

- `Symbol` stores namespace, name, declaration file, definition file, references,
  arbitrary typed properties, and package dependencies. It is not merely a type-name
  string.
- `SymbolProvider` defines type names, member names, file/module placement, and reserved
  word handling. `cache` provides thread-safe memoization.
- `SymbolWriter` formats symbols through `T`, automatically records imports and symbol
  dependencies, and distinguishes `USE` from `DECLARE` references.
- `WriterDelegator` maps definition filenames to writers in a `TreeMap`, reuses writers,
  inserts separators between writes, aggregates dependencies, and flushes files in sorted
  filename order. It is intentionally not thread-safe.
- `DependencyTracker` preserves added dependency entries and supports lookup by package,
  type, and properties. It does not deduplicate automatically.

Rust consequence: current `names.rs` plus direct string rendering covers only part of
this contract. Symbol records need module/file ownership, references, dependency metadata,
reserved-word context, and declaration-versus-use behavior.

### `smithy-rules-engine`: ordered, typed endpoint rules

Sources under `smithy-rules-engine/src/main/java/software/amazon/smithy/rulesengine/`.

`EndpointRuleSetTrait` stores raw rule-set `Node` and lazily parses it into
`EndpointRuleSet`. Parsing loads built-ins/functions/auth validators through SPI. The
rule set requires `version`, `parameters`, and ordered `rules`; construction type-checks
all parameters and rules. Rules evaluate in list order: each rule is considered only when
previous rules did not match. Endpoint rules therefore need a typed AST, scoped parameter
values, built-in function registry, type checking, ordered fallback, and diagnostics.

`smithy-aws-endpoints` supplies AWS built-ins and endpoint rule data. It is not equivalent
to selecting a fixed endpoint URL from the service model.

### AWS trait semantics

Sources under `smithy-aws-traits/src/main/java/software/amazon/smithy/aws/traits/`.

- `ServiceTrait` carries `sdkId`, ARN namespace, CloudFormation name, CloudTrail source,
  docs ID, endpoint prefix, and optional CloudWatch namespace. `sdkId` drives SDK naming;
  endpoint prefix is explicitly not a unique naming key.
- `RestXmlTrait` carries `noErrorWrapping`; protocol selection must preserve this bit.
- `HttpChecksumTrait` carries request-required state, request algorithm member,
  validation-mode member, and ordered response algorithms. The location name is
  `x-amz-checksum-` plus lowercased algorithm name.
- `SigV4Trait` carries signing name. It is metadata, not a hardcoded service lookup.
- Other AWS traits define ARN templates, auth schemes, streaming/checksum behavior,
  S3 XML customizations, endpoint discovery, and paginator metadata.

Trait providers parse `Node` into typed immutable trait objects, preserve source locations,
and cache original nodes. Rust typed accessors must preserve unknown raw traits instead of
discarding them.

### Protocol tests and waiters

`smithy-protocol-test-traits` defines modeled request/response/malformed-request test
cases. `smithy-protocol-tests` and `smithy-aws-protocol-tests` provide test models and
fixtures. These artifacts do not drive ordinary type generation, but they are semantic
wire-format evidence and should become Rust protocol fixtures.

`smithy-waiters` models ordered acceptors, matcher state, documentation, tags, deprecation,
and retry delays. Defaults are `minDelay = 2` seconds and `maxDelay = 120` seconds.
Waiter acceptor order is observable.

## Exact client-generation order

Source: `codegen-client/.../ClientCodegenVisitor.kt`.

`ClientCodegenVisitor` is main orchestration boundary. Current order:

1. Load `ClientRustSettings`.
2. Create base `RustSymbolProviderConfig` with runtime config, rename exceptions,
   nullability mode, module provider, and builder naming.
3. Apply baseline model transforms, in this order:
   - flatten and remove mixins;
   - copy service-level errors onto operations;
   - box recursive shapes;
   - add normalized error `message` fields when enabled;
   - add synthetic input/output structures to every operation;
   - normalize event-stream operations and unions;
   - mark operations incompatible with stalled-stream protection.
4. Resolve the service from the transformed baseline model.
5. Select protocol through `ClientProtocolLoader`, using decorator-adjusted protocol
   factories. Protocol selection is data-driven from service protocol traits.
6. Apply decorator model transforms.
7. Resolve service again because a decorator may replace or modify the service shape.
8. Construct the symbol provider and `ClientCodegenContext`.
9. Install module documentation customization and instantiate protocol implementation.
10. Create `RustCrate`, then create the protocol-specific operation generator.
11. Walk the service closure with `DirectedWalker`; each reachable shape accepts the
    visitor exactly once from the model graph.
12. Run decorator `extras`.
13. Finalize the crate. This injects inline dependencies, writes `lib.rs` and
    `Cargo.toml`, then flushes writers.
14. Upstream invokes `cargo fmt -- --config max_width=150` after finalization. This is
    upstream behavior; this repository keeps formatting in conformance, not in the
    Rust generator.

Important subtlety: protocol selection happens before decorator model transforms. A
model transform may alter shape contents or even rebuild the service, but it does not
silently cause a second protocol-selection pass. Port code should preserve this
boundary unless parity evidence proves otherwise.

## Model normalization invariants

Source: `codegen-core/.../transformers/OperationNormalizer.kt` and the baseline
transform chain above.

`OperationNormalizer` adds input and output structures even when the Smithy operation
has no input or output. IDs are built in a synthetic namespace:

```text
<operation namespace>.synthetic#<OperationName>Input
<operation namespace>.synthetic#<OperationName>Output
```

Each synthetic shape carries `SyntheticInputTrait` or `SyntheticOutputTrait` with:

- the owning operation ID;
- the original modeled shape ID, or null when operation omitted that shape.

Input shapes receive `InputTrait` when the original input omitted it. Existing modeled
structures are renamed into synthetic IDs, not copied as unrelated anonymous types.
Shape-ID conflicts fail immediately. Downstream code can distinguish modeled versus
synthetic input/output by trait and must use `originalId` when wire behavior depends on
the original shape.

This normalization explains several generated-output details:

- every operation gets stable public `Input` and `Output` symbols;
- empty operations still have builders and protocol paths;
- serializer/deserializer function names may contain `_input` or `_output`;
- XML root validation must inspect the original output shape when the output is synthetic.

Do not replace this with `Option<ShapeId>` branches spread across serializers. Normalize
once, then make generators consume the invariant.

## Reachability and visitor ownership

Source: `codegen-core/.../DirectedWalker.kt` and `ClientCodegenVisitor.kt`.

`DirectedWalker` delegates to Smithy `Walker` but accepts only relationships whose
`direction` is `DIRECTED`. It is a graph traversal, not a scan of every shape in the
model and not a scan of generated source. The service closure includes operations,
inputs, outputs, errors, nested members, collections, maps, unions, enums, streaming
shapes, and model metadata reached through directed relationships.

The shape visitor owns different output responsibilities:

- `ServiceShape`: service errors, config, runtime plugins, and public re-exports.
- `StructureShape`: structure plus builder; modeled errors use `ErrorGenerator` instead
  of the ordinary structure path.
- String shapes with `EnumTrait`: client enum generation. Plain strings emit no type
  source of their own.
- `UnionShape`: union enum, optional schema support, and event-stream error module.
- `OperationShape`: operation orchestration, request serializer, response deserializer,
  endpoint interceptor, protocol tests, operation error, and customizable fluent impl.

Structures, builders, enums, and unions are rendered into private modules and re-exported
through public symbols. This is deliberate. It prevents duplicate definitions while
keeping public paths stable. A port that writes every shape directly into one public
module will diverge in module layout, visibility, docs, and import behavior.

## Symbols and modules

Source: `codegen-core/.../RustSymbolProvider.kt` and the client module provider.

`RustSymbolProvider` owns more than type names. It supplies:

- shape, member, operation-error, event-stream-error, and builder symbols;
- module placement for shapes, builders, operation errors, and event-stream errors;
- nullability and required-field decisions through configured Smithy indexes;
- reserved-word and rename behavior;
- runtime symbol paths and dependency metadata.

`WrappingSymbolProvider` is the intended extension point. AWS decorators wrap or augment
base naming behavior; they do not replace generic naming logic with service branches.

Protocol helper names use the same symbol provider. `ProtocolFunctions.shapeModuleName`
creates `shape_<context-name>` modules. `shapeFunctionName` creates `ser_...` or `de_...`
functions and appends `_input` or `_output` for synthetic operation shapes. Reserved
words and double-underscore map names receive Rust-specific escaping/attributes.

## RustCrate and writer semantics

Source: `codegen-core/.../CodegenDelegator.kt`.

`RustCrate` is the sole crate-level owner for generated files. Generators request a
writer through symbols/modules instead of opening paths directly.

Rules that affect parity:

- `useShapeWriter` resolves the shape's owning module through the symbol provider.
- `withModule` rejects conflicting duplicate module properties.
- Inline modules are rendered into their parent; file modules create a file and a
  dependency that emits exactly one `mod` statement.
- `inPrivateModuleWithReexport` writes the private definition, then writes the public
  `pub use` path.
- Public modules require documentation from `ModuleDocProvider` or an explicit
  documentation override. Missing public docs are an error.
- Inline dependencies are lazy. `finalize` repeatedly discovers used inline dependencies,
  renders each once, and continues until no new dependency remains.
- Cargo dependencies are deduplicated, features merged, sorted by dependency name, and
  identical compile/test dependencies are reduced to the compile dependency unless a
  dev-only feature requires both.
- Finalization writes `src/lib.rs`, `Cargo.toml`, and all buffered source files only
  after generators finish.

This lazy dependency model explains why protocol helper discovery order can change the
file tree or helper ordering. Emit a helper by registering a dependency, not by eagerly
writing a guessed file.

## Protocol selection and operation generation

### Protocol selection

Source: `codegen-core/.../protocols/ProtocolLoader.kt` and client protocol loader.

A service advertises protocol traits. The loader filters the supported protocol map to
those traits and returns the first matching entry. No match raises an error containing
both offered and supported protocol IDs. Map order is therefore behavior. Never choose a
protocol by service name or JSON object iteration order.

### Operation generator

Source: `codegen-client/.../generators/OperationGenerator.kt`.

The operation generator renders in this order:

1. operation marker struct and standard derives;
2. operation orchestration methods;
3. additional runtime plugins and operation span fields from customizations;
4. runtime plugin composition and config override support;
5. operation customization sections;
6. operation runtime plugin generator;
7. response deserializer generator;
8. request serializer generator;
9. endpoint parameter interceptor generator.

The orchestrator erases typed input into runtime interceptor context, invokes the shared
runtime orchestrator, maps service errors back to the operation error type, finalizes the
context, and downcasts the typed output. It also creates a tracing span with service,
operation, and an internal invocation ID. Transport and retry behavior live in runtime
plugins and protocol generators, not in a per-operation transport implementation.

`ServiceGenerator` sorts contained operations by shape ID before generating service-level
error/config code. Operation visitor order and service-level operation listing are not the
same concern; preserve explicit sorting where upstream does it.

## Protocol helper ownership and Rest XML behavior

Source: `codegen-core/.../protocols/ProtocolFunctions.kt`, protocol serializer/parser
generators, and `RestXmlParserGenerator.kt`.

All protocol serialization/deserialization helpers should be emitted through
`ProtocolFunctions`:

- cross-operation helpers use `protocol_serde`;
- shape helpers use `protocol_serde::shape_<name>`;
- each helper returns a runtime symbol and is materialized only when used;
- helper modules are public crate modules with generated `mod` declarations;
- function names derive from shape/member context, not operation-name conditionals.

Rest XML parser root validation has a modeled exception. The parser looks through a
synthetic output to the original output shape. If the original output lacks
`AllowInvalidXmlRoot`, it checks the root element name and returns `XmlDecodeError` on a
mismatch. A service decorator may add that trait to an explicit allowlist. Therefore an
invalid-root exception belongs in a model transform/decorator, not in a generic
`operation_name == ...` parser branch.

S3's current decorator additionally replaces the Rest XML protocol factory, so S3 XML
behavior is a protocol customization layered over generic Rest XML behavior.

## Decorator composition

Sources: `codegen-core/.../customize/CoreCodegenDecorator.kt`,
`codegen-client/.../customize/ClientCodegenDecorator.kt`, and
`aws/codegen-aws-sdk/.../AwsCodegenDecorator.kt`.

Decorator hooks cover model transforms, protocol maps, symbol providers, structure and
builder generation, errors, operation sections, auth, endpoints, runtime plugins,
module docs, `lib.rs`, Cargo manifest, and protocol tests.

Composition rules are easy to get wrong:

- decorators are sorted by ascending `order`;
- `combineCustomizations` folds from the right over that sorted list;
- upstream documents that lower numeric order is applied last;
- additive hooks use `addCustomizations` and cannot remove earlier entries;
- combine hooks can replace or remove existing customization entries;
- classpath decorators are discovered with `ServiceLoader`, filtered by
  `classpathDiscoverable`, then combined with explicit extras.

Do not assume `base + customizations` is always correct. First identify whether the
upstream hook is additive or replacement-capable, then preserve order and removal
semantics.

AWS applies a global decorator set plus conditional decorators. Conditions use exact
service shape IDs, namespaces, protocol traits, or explicit model metadata. Current S3
customization in the inspection mirror illustrates proper exception placement:

- override the Rest XML protocol factory;
- add `AllowInvalidXmlRoot` to three known output shapes;
- transform S3 HTTP paths and endpoint tests;
- add synthetic no-auth support;
- make selected S3 booleans/numbers optional;
- add endpoint built-in setters;
- customize response-error detection and retry classifiers;
- parse empty S3 HEAD error bodies as `NotFound` for HTTP 404.

These rules are conditional decorator behavior. They are not permission to add S3 tests
or operation-name branches to generic renderers.

## Related runtime boundary

Upstream codegen and runtime are separate products. Codegen emits references to runtime
crates through `RuntimeType`; `RustCrate` turns used runtime symbols into Cargo
requirements and generated imports. Important runtime areas include:

- `aws-smithy-runtime` and `aws-smithy-runtime-api` for orchestration/plugins;
- `aws-smithy-http` and `aws-smithy-types` for HTTP/types;
- `aws-smithy-json`, `aws-smithy-query`, `aws-smithy-xml`, and `aws-smithy-cbor` for
  protocol support;
- `aws-smithy-eventstream` for event streams;
- `aws-smithy-checksums` and AWS runtime crates for checksums, retries, auth, and
  endpoint behavior.

The current repository intentionally has a smaller downstream `aws-runtime` contract.
Parity work must either add reusable runtime abstractions or document an exact supported
contract. It must not emit references to unavailable upstream runtime crates by accident.

### Dependencies outside this repository

If `that repository` means this `better_aws` checkout, upstream has a much larger
runtime surface. `crates/aws-sdk-build/Cargo.toml` only depends directly on
`serde_json`, `sha2`, `tempfile`, and `thiserror`. The consumer fixture adds a small
runtime subset: `aws-runtime`, `aws-types`, `aws-smithy-runtime-api`,
`aws-smithy-types`, `http`, and Tokio.

Upstream AWS SDK generation expects these additional first-party crates, all absent from
this checkout unless supplied through crates.io:

- AWS: `aws-config`, `aws-credential-types`, `aws-runtime-api`, `aws-sigv4`.
- Smithy runtime: `aws-smithy-async`, `aws-smithy-checksums`, `aws-smithy-eventstream`,
  `aws-smithy-http`, `aws-smithy-http-client`, `aws-smithy-json`,
  `aws-smithy-observability`, `aws-smithy-runtime`, `aws-smithy-schema`,
  `aws-smithy-query`, and `aws-smithy-xml`.
- Optional or test-facing runtime: `aws-smithy-cbor`, compression, mocks, protocol-test,
  WASM, and runtime API proc-macro crates.

Those first-party runtime crates are present in the upstream `smithy-rs` source tree,
but not in this checkout. Their external Cargo dependencies include Tokio, `tracing`,
`bytes`, both `http` 0.2 and 1.x, both `http-body` 0.4 and 1.x, Hyper, `futures-util`,
`pin-project-lite`, `time`, `uuid`, `percent-encoding`, Base64, checksum/hash crates,
`zeroize`, and optional TLS stacks such as Rustls or Hyper-Rustls. These are runtime
transitives, not codegen model dependencies.

The JVM codegen side also depends on artifacts outside the upstream repository's source:
Kotlin/JDK, Gradle, Maven Central Smithy artifacts (`smithy-codegen-core`,
`smithy-model`, AWS/protocol/rules/waiter traits), plus Jsoup, Jackson CBOR, Toml4j,
JUnit/Kotest, Ktlint, and JReleaser. The complete direct list is in the inspection
mirror's `gradle/libs.versions.toml` and module `build.gradle.kts` files.

## Mapping to current Rust port

Current Rust files combine several upstream responsibilities:

- `crates/aws-sdk-build/src/model.rs`: packaged model load, checksum validation,
  operation selection, directed shape-reference closure, protocol trait selection, and
  a small model customization pass.
- `crates/aws-sdk-build/src/names.rs`: Rust module/type naming.
- `crates/aws-sdk-build/src/codegen.rs`: service facade, types/builders/errors, client,
  operation builders, request binding, Rest XML serialization/parsing, and documentation.
- `crates/aws-sdk-build/src/output.rs`: staging, validation, and atomic installation.
- `crates/aws-sdk-build/src/config.rs`: deterministic service/operation selection.

Current `model.rs` already uses `BTreeMap`/`BTreeSet` closure and model-derived protocol
selection. Current `codegen.rs` sorts selected operations and shapes before writing. These
are useful parity foundations.

Known architectural migration targets:

- split `codegen.rs` into context, symbols/modules, shape generators, operation plans,
  protocol bindings, and decorators;
- replace raw `serde_json::Value` traversal in renderers with typed indexed accessors;
- model synthetic input/output and recursive boxing as explicit transforms;
- move service-specific request-ID and metadata behavior into declarative decorators;
- replace direct protocol `if protocol == RestXml` branching with protocol factories and
  shared binding plans;
- replace current direct S3 and `dynamodb`/`lambda` output conditionals with metadata or
  conditional decorator inputs;
- make helper discovery and correction ordering follow lazy runtime symbol registration;
- keep generated output free of temporary manifests and let conformance own formatting.

The goal is not to reproduce Kotlin class names. The goal is to preserve these data
boundaries and observable ordering rules in Rust.

## Mismatch-debugging loop

For each conformance mismatch:

1. Identify exact reference/generated path pair.
2. Classify mismatch: model transform, closure, symbol/module, shape, builder,
   protocol binding, serializer, parser, runtime, decorator, docs, dependency, ordering,
   formatting, or installer.
3. Locate upstream source using the map below.
4. Find model trait/shape metadata that selects the behavior.
5. Add a reusable Rust rule and focused test; do not patch one operation literal.
6. Regenerate all-operation snapshots.
7. Run `just conformance` and compare matched/mismatched/missing/extra counts.
8. Record the checkpoint in `docs/aws-sdk-build-status.md`.

Useful targeted reads in `/tmp/smithy-rs`:

- Pipeline: `codegen-client/.../ClientCodegenVisitor.kt`.
- Context: `codegen-core/.../CodegenContext.kt`.
- Model transforms: `codegen-client/.../ClientCodegenVisitor.kt` and
  `codegen-core/.../transformers/OperationNormalizer.kt`.
- Closure: `codegen-core/.../DirectedWalker.kt`.
- Symbols/modules: `codegen-core/.../RustSymbolProvider.kt` and client module provider.
- Crate/files/dependencies: `codegen-core/.../CodegenDelegator.kt`.
- Shape generation: `SchemaGenerator.kt`, `StructureGenerator.kt`, `BuilderGenerator.kt`,
  `UnionGenerator.kt`.
- Protocol selection: `ProtocolLoader.kt` and client protocol loader.
- Protocol helper ownership: `ProtocolFunctions.kt`.
- Rest XML parsing: `parse/RestXmlParserGenerator.kt` and
  `parse/XmlBindingTraitParserGenerator.kt`.
- Operation pipeline: client `generators/OperationGenerator.kt`.
- Service pipeline: client `generators/ServiceGenerator.kt`.
- Decorators: core/client `*CodegenDecorator.kt`.
- AWS composition: `aws/codegen-aws-sdk/.../AwsCodegenDecorator.kt`.
- S3 exception placement: `aws/.../customize/s3/S3Decorator.kt`.

Use `git -C /tmp/smithy-rs show <commit>:<path>` for a pinned source read. Keep
conformance behavior tied to the project parity pin, and record any upstream-version
comparison separately.
