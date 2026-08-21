# Reusable Smithy Rust codegen design

This document is the compact implementation reference for the Rust port in this
repository. It distills the architecture in the pinned Smithy Rust implementation so
future work can start from one design instead of repeatedly rereading the Kotlin
sources.

Reference snapshot:

- Repository: [`smithy-lang/smithy-rs`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d)
- Commit: `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`
- Generic core: [`codegen-core`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/codegen-core)
- Client layer: [`codegen-client`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/codegen-client)
- AWS layer: [`aws/codegen-aws-sdk`](https://github.com/smithy-lang/smithy-rs/tree/f1b64a9c0dd001d4bac4277fec4041da59c1f48d/aws/codegen-aws-sdk)

The target is not to reproduce Kotlin class names. The target is to preserve the
separation of concerns, the model-driven decisions, and the ordered generation hooks
while implementing them in Rust.

## Core invariant

The generator is a compiler:

```text
Smithy JSON model
  -> indexed model and traits
  -> normalized model
  -> service-directed shape closure
  -> symbols and module locations
  -> protocol/binding plan
  -> generic Rust crate generation
  -> ordered AWS/custom decorators
  -> deterministic formatting and validation
  -> generated crate
```

The generic pipeline must never branch on an operation name or service name to make a
fixture pass. An operation name is data used for lookup and naming. Service-specific
behavior is a decorator or a model/trait transformation selected by service shape ID,
namespace, protocol, or an explicit trait. The generic renderer consumes descriptors;
it does not know that a descriptor came from S3, DynamoDB, or STS.

## Reference architecture at a glance

| Responsibility | Smithy Rust reference | Rust implementation responsibility |
| --- | --- | --- |
| Shared read-only context | `CodegenContext` | Model, service, protocol, settings, symbols, docs, runtime, target |
| Client context | `ClientCodegenContext` | Client-only runtime and decorator state |
| Model preprocessing | `OperationNormalizer` and model transformers | Ordered immutable transforms over the JSON-derived IR |
| Reachability | `DirectedWalker` | Directed graph traversal from the selected service |
| Naming and placement | `RustSymbolProvider`, `ModuleProvider` | Shape/member/operation/error/builder symbols and modules |
| Source ownership | `RustCrate`, `RustWriter`, `RustModule` | Files, module declarations, imports, dependencies, features |
| Shape generation | `SchemaGenerator`, `StructureGenerator`, `BuilderGenerator`, `UnionGenerator` | Generic primitive/structure/list/map/enum/union/error generators |
| Protocol selection | `ProtocolLoader`, protocol factory map | Select protocol from service traits, never from service-name branches |
| HTTP binding | `HttpBindingResolver` | Resolve labels, query, headers, prefix headers, payload, body, status |
| Operation generation | `OperationGenerator` | Orchestration, request serializer, response deserializer, endpoint inputs |
| Service generation | `ServiceGenerator` | Config, service errors, runtime plugins, client re-exports |
| Extensibility | `ClientCodegenDecorator` and ordered composition | Model transforms and generation hooks with deterministic order |
| AWS behavior | `AwsCodegenDecorator` and conditional decorators | Auth, endpoint, retry, checksum, presign, paginator, docs, service fixes |
| Finalization | `RustCrate.finalize` | Write `lib.rs`, manifests, dependencies, format, validate, install |

## 1. Model loading and indexing

The input layer converts each packaged Smithy JSON AST into an indexed intermediate
representation. Do not let renderers repeatedly walk raw `serde_json::Value` trees.

The index should provide:

- `ShapeId -> Shape` lookup;
- shape kind and target resolution;
- member order and member target lookup;
- operation input/output/error references;
- service shape and contained operations;
- trait lookup on shapes and members;
- protocol, auth, endpoint, retry, checksum, pagination, event-stream, and
  documentation indexes;
- source order plus canonical sorted order for deterministic output.

Preserve raw trait values in addition to typed accessors. Typed accessors should cover
common Smithy and AWS traits, while unknown traits remain available for extensions and
future parity work. Every index query should return a structured error containing the
shape ID and the missing/invalid field; silent fallback to an unrelated shape is a
codegen bug.

Suggested Rust boundary:

```rust
struct ModelIndex {
    shapes: BTreeMap<ShapeId, Shape>,
    traits: TraitIndex,
    services: ServiceIndex,
    protocols: ProtocolIndex,
    operations: OperationIndex,
}

struct ServiceView<'a> {
    model: &'a ModelIndex,
    service: ShapeId,
    settings: CodegenSettings,
}
```

## 2. Ordered model normalization

The reference applies a baseline transformation before visiting shapes. The transforms
are ordered, produce a new model/IR, and make downstream generators operate on stable
invariants.

The baseline sequence is conceptually:

1. Flatten mixins and remove mixin-only shapes.
2. Copy service-level errors to operations where the client error model expects them.
3. Box recursive shapes so generated Rust types have finite layout.
4. Normalize error messages when configured.
5. Create synthetic input and output structures for every operation, including empty
   ones; retain the original shape ID as metadata.
6. Normalize event-stream operations and synthetic event-stream unions.
7. Apply protocol- or runtime-related normalization such as stalled-stream markers.
8. Apply AWS model transforms that are justified by traits or a conditional decorator.

The synthetic input/output step is especially important: every operation then has a
stable input and output symbol, and every later stage can use the same code path for
empty and non-empty operations. Synthetic shapes must carry an explicit marker so
documentation, serialization, and public API generation can distinguish them from
user-modeled shapes.

Use a transform interface rather than embedding these operations in every generator:

```rust
trait ModelTransform {
    fn name(&self) -> &'static str;
    fn apply(&self, model: ModelIndex) -> Result<ModelIndex, TransformError>;
}
```

Record the transform names and resulting model fingerprint in the output manifest.

The current Rust port also applies a compact declarative transform over the selected
JSON shape map before rendering. It recognizes the Smithy relationship used for S3’s
`Expires` header, changes its string target to a timestamp, and adds the synthetic raw
header member required by the AWS decorator. The renderer only consumes the resulting
traits and shapes; it does not contain an S3 or operation-name branch.

The request-ID decorator is similarly represented as a rendering plan derived from
service metadata. Every operation output receives the standard AWS request-ID fields,
builder setters, and RequestId implementation. A service whose aws.api#service
metadata advertises the s3 ARN namespace additionally receives the s3_request_id
helper and RequestIdExt fields. Response decoding populates both values from HTTP
headers, including empty outputs, while operation unhandled errors retain the same
header metadata.

## 3. Service-directed shape closure

Start at the selected service shape and traverse only directed relationships. The
closure includes the service, contained operations, synthetic input/output shapes,
errors, nested structures, collections, maps, unions, enum targets, streaming
targets, and any model shapes required by endpoint/auth/protocol metadata.

Do not generate every shape in a model by accident. Do not compute closure by scanning
strings in generated source. The closure is a graph operation over shape references.

Algorithm:

```text
queue = [service shape]
seen = {}
while queue is not empty:
    shape = queue.pop()
    if shape.id already in seen: continue
    mark shape.id seen
    enqueue every directed target relationship required by the shape
sort seen IDs canonically
generate each reachable shape exactly once
```

Keep an explanation for every closure edge (`member target`, `operation input`,
`operation error`, `protocol metadata`, or `decorator synthetic edge`) so missing or
unexpected generated files can be diagnosed from the model graph.

## 4. Symbols, names, nullability, and modules

All generated references must flow through one symbol provider. The symbol provider is
not just a `ShapeId -> String` map; it determines:

- Rust type, member, operation, error, and builder names;
- primitive mappings and wrappers;
- optionality and required-field behavior;
- recursive boxing;
- reserved-word escaping and raw identifiers;
- enum/union variant names;
- public/private visibility;
- the owning Rust module and source file;
- runtime crate paths and feature/dependency metadata;
- documentation and deprecation metadata.

The module provider must define the public tree before files are rendered. A shape is
rendered into its owning private module when appropriate, then re-exported from its
public symbol path. This prevents duplicate definitions and keeps file placement
deterministic.

The Rust port should expose a wrapping provider so AWS decorators can add or override
symbols without replacing the generic naming rules wholesale.

## 5. Source writer and crate ownership

Use a crate-level writer abstraction instead of writing ad hoc strings directly to
paths. The crate writer owns:

- one writer per generated file/module;
- module declarations and re-exports;
- symbol-based import collection and de-duplication;
- runtime dependency, Cargo feature, and inline dependency tracking;
- documentation requirements for public modules;
- duplicate-module detection;
- final `lib.rs`, manifest, and source-tree assembly.

Generation should be referentially stable: rendering the same shape twice targets the
same writer and either composes intentionally or fails with a duplicate-definition
diagnostic. Dependency rendering must be lazy but deterministic: if a generated symbol
uses an inline dependency, enqueue it and render it once in canonical order.

Prefer token-aware Rust writers (`syn`/`proc_macro2` or an equivalent validated
writer) over unconstrained string concatenation. If strings are used at an
intermediate stage, parse every generated Rust file before installation.

## 6. Protocol abstraction

Protocol selection is data-driven. A service advertises protocol traits; a protocol
loader finds the first supported protocol in a deterministic supported-protocol map.
The selected protocol factory provides a protocol implementation and a protocol support
matrix.

The generic protocol contract should cover:

```rust
trait Protocol {
    fn http_bindings(&self) -> &dyn HttpBindingResolver;
    fn request_serializer(&self) -> &dyn RequestSerializer;
    fn response_deserializer(&self) -> &dyn ResponseDeserializer;
    fn error_parser(&self) -> &dyn ErrorParser;
    fn payload_generator(&self) -> &dyn PayloadGenerator;
    fn support(&self) -> ProtocolSupport;
}
```

`HttpBindingResolver` is protocol-neutral and resolves the model’s member bindings:

- URI labels and greedy labels;
- query parameters and query maps;
- headers and prefix headers;
- request/response payloads and document bodies;
- response status codes and response headers;
- error body and error headers;
- timestamp formats, content types, and event-stream initial messages.

Protocol implementations then provide the syntax and semantics for Rest JSON, Rest
XML, AWS JSON, AWS Query, EC2 Query, RPC V2 CBOR, and event streams. The operation
generator should consume bindings and protocol interfaces, not contain JSON/XML/query
branches itself.

Protocol support must be explicit. Track request serialization, request body
serialization, response deserialization, error deserialization, and the corresponding
server-side capabilities. Refuse a service/protocol combination when a required
capability is unavailable instead of emitting a partial operation silently.

## 7. Generic shape generators

The shape visitor generates each reachable shape once. The common generators are:

- primitives, timestamps, blobs, documents, and constrained values;
- structures and their builders;
- lists and maps, including sparse and recursive cases;
- string and integer enums;
- unions with unknown-variant behavior where required;
- modeled errors and error metadata;
- schema/serde representations when the runtime contract requires them.

Shape generators receive the model, symbol provider, context, writer, and an ordered
customization list. They do not inspect service or operation names. Required fields,
defaults, optionality, sensitivity, flattening, XML names, and serde behavior come from
shape/member traits and the normalized model.

## 8. Generic service and operation generators

### Service generator

The service generator emits service-wide code:

- service error union and error metadata;
- `Config` and config builder;
- runtime components and service plugins;
- client construction and public re-exports;
- service documentation and Cargo feature wiring.

It obtains its operations from the service index in canonical order.

### Operation generator

The operation generator emits one operation module from an `OperationPlan` derived from
the model:

```rust
struct OperationPlan {
    operation: ShapeId,
    input: Symbol,
    output: Symbol,
    error: Symbol,
    protocol: ProtocolId,
    request_bindings: Vec<Binding>,
    response_bindings: Vec<Binding>,
    endpoint_inputs: Vec<EndpointInput>,
    auth_options: Vec<AuthOption>,
    retry_policy: RetryPolicy,
    checksum_policy: ChecksumPolicy,
    streaming: StreamingPlan,
}
```

It renders, in order:

1. operation marker and orchestration hooks;
2. runtime plugin composition;
3. request serialization and payload ownership;
4. endpoint parameter extraction;
5. transport invocation;
6. response deserialization;
7. modeled error parsing and error metadata;
8. protocol tests and customization sections.

The operation generator must not contain `if operation_name == ...` or
`if service == ...`. If two operations differ, their `OperationPlan` data or a
decorator customization must explain the difference.

## 9. Decorators and service customization

Smithy Rust separates the generic generator from AWS behavior with ordered decorators.
Decorators compose by adding or replacing hook results and can be conditionally applied
by exact service shape ID, namespace, protocol, or model trait.

Useful hook categories are:

- model transforms;
- protocol registration/override;
- symbol provider wrapping;
- structure, builder, enum, union, and error customization;
- operation sections;
- auth scheme options and signing;
- endpoint built-ins and endpoint rules;
- config and runtime plugins;
- retry classifiers and checksum behavior;
- documentation, `lib.rs`, Cargo manifest, and feature customization;
- protocol test generation;
- extra files or re-exports.

Use deterministic decorator order with a stable numeric order and explicit composition
rules. A decorator must be inspectable: record its name, predicate, order, and model
conditions in debug output or the generation manifest.

The AWS reference uses a global decorator set plus conditional decorators. For example,
S3 has a conditional decorator that can transform the model, select a Rest XML protocol
override, customize endpoint behavior, and add operation retry/error sections. That is
the correct place for a real S3 exception; it is not a reason to add S3 branches to the
generic operation renderer.

## 10. Runtime generation

Generated operations should use a shared runtime orchestrator rather than embedding a
different transport implementation in each operation. The runtime pipeline is:

```text
typed input
  -> endpoint parameters
  -> endpoint resolution
  -> auth scheme selection and signing
  -> request construction
  -> request interceptors/plugins
  -> protocol serialization
  -> transport
  -> response interceptors/plugins
  -> status/error classification
  -> protocol deserialization
  -> typed output or modeled/unhandled error
```

Keep the generated crate’s runtime contract explicit. The generated manifest must list
runtime crates, versions/features, and any inline source. Runtime concerns such as
retry, timeout, body streaming, checksums, event streams, and presigning belong in
shared runtime abstractions plus generated plans/plugins, not copied into every
operation.

## 11. Deterministic finalization

Finalization is a separate phase after all shape/service/operation writers have run:

1. materialize inline dependencies;
2. generate module declarations and re-exports;
3. generate `Cargo.toml`/manifest metadata and feature dependencies;
4. flush writers in canonical path order;
5. run the configured Rust formatter;
6. parse/validate every Rust file;
7. validate public symbol paths and manifest references;
8. compare against the reference tree;
9. atomically install the generated tree only after all validation succeeds.

Do not let timestamps, absolute paths, hash-map iteration order, host paths, or
parallel completion order enter generated output.

## 12. Testing strategy

Testing follows the same layers as generation:

### Unit tests

- JSON AST parsing and trait accessors;
- shape graph closure and closure explanations;
- every model transform, including synthetic input/output and recursive boxing;
- symbol names, reserved words, optionality, boxing, and module placement;
- binding resolution for every location and protocol;
- deterministic writer/import/dependency output;
- decorator ordering and conditional predicates.

### Protocol tests

For each protocol and binding form, test exact request method, URI, query, headers,
content type, body bytes, response parsing, error parsing, timestamps, blobs, maps,
flattening, sparse collections, and event-stream framing. Use model-provided protocol
test cases where available.

### Generated consumer tests

Compile a clean consumer using only Rust/Cargo dependencies. Test public paths,
builders, `send()`, errors, streaming bodies, endpoint overrides, and selected versus
all-operation generation.

### Golden conformance

Generate every selected service from the pinned JSON model and compare the complete
source tree to `conformance/reference`. Treat missing and extra files as failures.
Use the report to choose the next reusable rule, not to justify a one-off patch.

## 13. Implementation plan for this repository

The current project has the public API, packaged model registry, source snapshot
harness, a partial writer, and a small local S3 runtime. The next implementation order
should be:

1. Introduce a normalized internal IR and trait/index APIs in `model.rs`; stop passing
   raw JSON values through renderers.
2. Implement ordered model transforms and a closure object with edge explanations.
3. Split `codegen.rs` into context, symbols/modules, shape generators, service
   generation, operation planning, and finalization modules.
4. Replace operation/service-name branches with `OperationPlan`, protocol bindings,
   and conditional decorator interfaces.
5. Implement protocol factories and shared serializers/deserializers, beginning with
   Rest XML/Rest JSON and then AWS JSON/Query/EC2 Query/event streams.
6. Implement the shared runtime contract: endpoint rules, auth/signing, retries,
   checksums, streaming, pagination, presigning, and modeled error behavior.
7. Port AWS decorators as data/trait-driven Rust decorators, including S3 and STS
   behavior, instead of adding generic renderer exceptions.
8. Add focused protocol/generator tests and regenerate after every step.

At every step, run `just conformance` and compare the previous and current report. The
goal is an increasing exact-match count and decreasing mismatch/missing/extra count.
If a change improves one service but worsens the aggregate result, keep it only when
the report identifies a deliberate reference behavior and add the missing generic
rule before continuing.

## Mismatch diagnosis checklist

For each representative diff:

1. Identify the reference file and generated file.
2. Classify the difference: model/index, transform, closure, symbol/module, shape,
   protocol binding, serializer/deserializer, runtime, decorator, dependency, docs,
   formatting, or output installation.
3. Find the corresponding reference abstraction using the map above.
4. Add or fix the reusable Rust abstraction and a focused fixture/test.
5. Regenerate all operations and run conformance.
6. Record before/after metrics and the next mismatch class in the status log.

Never fix a mismatch by matching a literal operation name when the same rule could
appear in another service. Literal service IDs are permitted only in conditional
decorator predicates whose behavior is isolated from the generic generator and backed
by a model/reference explanation.

## Source map for targeted rereading

When a detail is unclear, read only the relevant pinned file:

- Context: `codegen-core/.../smithy/CodegenContext.kt`
- Model transforms: `codegen-core/.../smithy/transformers/OperationNormalizer.kt`
- Closure: `codegen-core/.../smithy/DirectedWalker.kt`
- Symbols: `codegen-core/.../smithy/RustSymbolProvider.kt`
- Crate/files/dependencies: `codegen-core/.../smithy/CodegenDelegator.kt`
- Shape types: `codegen-core/.../smithy/generators/SchemaGenerator.kt`,
  `StructureGenerator.kt`, `BuilderGenerator.kt`, and `UnionGenerator.kt`
- Protocol binding: `codegen-core/.../smithy/protocols/HttpBindingResolver.kt`
- Protocol selection: `codegen-core/.../smithy/protocols/ProtocolLoader.kt`
- Client pipeline: `codegen-client/.../smithy/ClientCodegenVisitor.kt`
- Operation pipeline: `codegen-client/.../smithy/generators/OperationGenerator.kt`
- Service pipeline: `codegen-client/.../smithy/generators/ServiceGenerator.kt`
- Customization hooks: `codegen-client/.../smithy/customize/ClientCodegenDecorator.kt`
- AWS decorator composition: `aws/codegen-aws-sdk/.../rustsdk/AwsCodegenDecorator.kt`
- S3-specific extension example: `aws/codegen-aws-sdk/.../rustsdk/customize/s3/S3Decorator.kt`

The local mirror is `/tmp/smithy-rs` at the pinned commit. Use it for detailed
inspection, but keep this document as the reusable architectural summary.
