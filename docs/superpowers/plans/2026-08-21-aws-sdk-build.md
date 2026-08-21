# Modular AWS SDK Build-Time Code Generation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `aws-sdk-build`, a tonic-style Rust build dependency that prunes a local AWS Smithy model to selected operations and invokes smithy-rs AWS Rust codegen to produce an includable modular client.

**Architecture:** A small public builder validates configuration and delegates to focused model, pruning, Smithy configuration, process-runner, and output modules. The build script writes a deterministic pruned JSON model and Smithy build configuration into a temporary directory, invokes the installed Smithy CLI with Maven dependencies for smithy-rs `codegen-aws-sdk`, and atomically copies the generated projection plus an include root and manifest to `OUT_DIR`.

**Tech Stack:** Rust 2021, Cargo workspace, `serde`/`serde_json`, `thiserror`, `tempfile`, Smithy CLI, smithy-rs `codegen-aws-sdk` and `codegen-client` Maven artifacts, AWS `aws-runtime` and Smithy runtime crates in generated consumers.

**Spec:** `docs/superpowers/specs/2026-08-21-aws-sdk-build-design.md`

## Global Constraints

- The consumer-facing crate is named `aws-sdk-build` and is used only as a `[build-dependencies]` entry.
- Model input is a local pinned Smithy JSON AST file or directory; the default build is offline and never downloads models.
- The builder API must support `model`, `service`, `operations`, `out_dir`, `smithy`, `rust_client_codegen`, and `compile`.
- Omitted operations select every operation on the service; an explicitly empty operation list is an error.
- Generated code is written below `OUT_DIR` and included through `include!(concat!(env!("OUT_DIR"), "/aws_sdk.rs"));`.
- Smithy invocation must resolve an explicit path first, then `SMITHY_CLI`, then `smithy` on `PATH`.
- Generation failures must leave the previously generated output untouched.
- Unit tests must run without a Smithy installation, network access, AWS credentials, or a JDK.
- Generated runtime code may use `aws-runtime` and Smithy runtime crates but must not depend on any `aws-sdk-*` service crate.
- Use `cargo fmt --check`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` as the local verification gates.

### Task 1: Create the workspace and public builder contract

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `crates/aws-sdk-build/Cargo.toml`
- Create: `crates/aws-sdk-build/src/lib.rs`
- Create: `crates/aws-sdk-build/src/config.rs`
- Create: `crates/aws-sdk-build/src/error.rs`
- Test: `crates/aws-sdk-build/src/lib.rs`
- Create: `README.md`

**Interfaces:**
- Produces `aws_sdk_build::configure() -> Builder`.
- Produces `Builder::model`, `Builder::service`, `Builder::operations`, `Builder::out_dir`, `Builder::smithy`, `Builder::rust_client_codegen`, and `Builder::compile`.
- `compile` returns `Result<CompileReport, BuildError>` where `CompileReport` exposes `generated_root`, `manifest`, and the sorted selected operation names.

- [ ] **Step 1: Write the failing API test**

Add an inline unit test that constructs the documented builder, calls a validation-only helper exposed internally as `Builder::validate`, and asserts that a missing model path produces `BuildError::MissingModel`. Also assert that `operations([])` produces `BuildError::EmptyOperations`.

```rust
#[test]
fn builder_rejects_missing_model_and_empty_operation_selection() {
    let missing = configure()
        .model("does-not-exist.json")
        .service("example#Service")
        .operations(["GetThing"])
        .validate()
        .expect_err("missing model must fail before tool invocation");
    assert!(matches!(missing, BuildError::MissingModel { .. }));

    let empty = configure()
        .model("model.json")
        .service("example#Service")
        .operations(std::iter::empty::<&str>())
        .validate()
        .expect_err("empty operation selection must be rejected");
    assert!(matches!(empty, BuildError::EmptyOperations));
}
```

- [ ] **Step 2: Run the focused test and verify the expected missing-API failure**

Run: `cargo test -p aws-sdk-build builder_rejects_missing_model_and_empty_operation_selection`

Expected: compilation fails because the workspace, `configure`, `Builder`, and `BuildError` do not exist.

- [ ] **Step 3: Add the minimal workspace and builder types**

Create the workspace manifest with resolver 2 and `crates/aws-sdk-build` as its member. Define the builder as an owned fluent configuration:

```rust
pub struct Builder {
    model: Option<PathBuf>,
    service: Option<String>,
    operations: Option<Vec<String>>,
    out_dir: Option<PathBuf>,
    smithy: Option<PathBuf>,
    rust_client_codegen: String,
}

pub fn configure() -> Builder {
    Builder::default()
}

impl Builder {
    pub fn model<P: Into<PathBuf>>(mut self, path: P) -> Self { self.model = Some(path.into()); self }
    pub fn service(mut self, service: impl Into<String>) -> Self { self.service = Some(service.into()); self }
    pub fn operations<I, S>(mut self, operations: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.operations = Some(operations.into_iter().map(Into::into).collect()); self
    }
    pub fn out_dir<P: Into<PathBuf>>(mut self, path: P) -> Self { self.out_dir = Some(path.into()); self }
    pub fn smithy<P: Into<PathBuf>>(mut self, path: P) -> Self { self.smithy = Some(path.into()); self }
    pub fn rust_client_codegen(mut self, coordinate: impl Into<String>) -> Self {
        self.rust_client_codegen = coordinate.into(); self
    }
}
```

Set the default codegen coordinate to `software.amazon.smithy.rust:codegen-aws-sdk:0.1.25`; allow the method above to override it. Implement typed validation errors with `thiserror` and keep `validate` `pub(crate)` so unit tests exercise the same path as `compile`.

- [ ] **Step 4: Run the focused test and verify it passes**

Run: `cargo test -p aws-sdk-build builder_rejects_missing_model_and_empty_operation_selection`

Expected: PASS with one test and zero failures.

- [ ] **Step 5: Add initial user documentation and commit**

Document build-dependency setup, the `build.rs` snippet, the generated include, the local model requirement, and Smithy CLI prerequisites in `README.md`. Run:

```bash
cargo fmt --check
git diff --check
git add Cargo.toml .gitignore crates/aws-sdk-build README.md
git commit -m "feat: add AWS SDK build builder contract"
```

### Task 2: Load Smithy JSON AST and prune the operation closure

**Files:**
- Create: `crates/aws-sdk-build/src/model.rs`
- Create: `crates/aws-sdk-build/src/prune.rs`
- Modify: `crates/aws-sdk-build/src/lib.rs`
- Modify: `crates/aws-sdk-build/src/error.rs`
- Create: `crates/aws-sdk-build/tests/model_selection.rs`
- Create: `tests/fixtures/selection-model.json`

**Interfaces:**
- `model::load(path: &Path) -> Result<Model, BuildError>`.
- `Model::select(service_id: &str, operations: Option<&[String]>) -> Result<Selection, BuildError>`.
- `Selection::document() -> &serde_json::Value` and `Selection::operations() -> &[String]`.
- `Selection::write_json(path: &Path) -> Result<(), BuildError>`.

- [ ] **Step 1: Write the failing model-selection tests**

Create a Smithy JSON fixture with service `example#Service`, operations `GetThing` and `DeleteThing`, disjoint input/output structures, a shared error, and `UnreachableShape`. Test that selecting `GetThing` retains the service, selected operation, selected input/output, shared error, and every member target while excluding `DeleteThing`, its disjoint shapes, and `UnreachableShape`. Test omitted operations select both operations, an unknown service fails with the service ID, and an unknown operation fails with the operation name.

```rust
#[test]
fn selection_keeps_transitive_shapes_but_excludes_unselected_operations() {
    let model = load(fixture("selection-model.json")).unwrap();
    let selection = model.select("example#Service", Some(&["GetThing".into()])).unwrap();
    let shapes = selection.document()["shapes"].as_object().unwrap();

    assert!(shapes.contains_key("example#GetThing"));
    assert!(shapes.contains_key("example#GetThingInput"));
    assert!(shapes.contains_key("example#GetThingOutput"));
    assert!(shapes.contains_key("example#SharedError"));
    assert!(!shapes.contains_key("example#DeleteThing"));
    assert!(!shapes.contains_key("example#DeleteThingInput"));
    assert!(!shapes.contains_key("example#UnreachableShape"));
}
```

- [ ] **Step 2: Run the model tests and verify they fail for missing loader/pruner behavior**

Run: `cargo test -p aws-sdk-build --test model_selection`

Expected: compilation fails because the fixture, loader, and selection APIs are not implemented.

- [ ] **Step 3: Implement JSON loading and deterministic shape indexing**

Parse the root as `serde_json::Value`, require an object-valued `shapes` member, and preserve all non-shape root metadata. Read a file path directly; if the path is a directory, recursively collect `.json` files in lexical path order and merge their shape maps, rejecting duplicate shape IDs. Use `BTreeMap<String, Value>` for indexed shapes and return errors containing the source path and shape ID.

- [ ] **Step 4: Implement service/operation lookup and fixed-point closure**

Resolve the service shape's `operations` array into shape IDs. If `operations` is `None`, select every service operation; if it is `Some`, match either the exact operation shape ID or the operation's terminal name. Start the work queue with the service and selected operation IDs. For each retained shape, enqueue references from `target`, `input`, `output`, `errors`, `members`, `key`, `value`, `resource`, and `identifiers` fields. Also enqueue trait shape IDs that are present in the source shape map, while ignoring scalar trait values. Preserve all Smithy prelude/trait definitions required by the model root, sort shape IDs, and emit a new root JSON document with stable pretty formatting.

- [ ] **Step 5: Run the model tests and verify they pass**

Run: `cargo test -p aws-sdk-build --test model_selection`

Expected: PASS with all selection, validation, directory-merge, and deterministic-output assertions passing.

- [ ] **Step 6: Wire selection into builder validation and commit**

Make `validate` load the model, resolve the selected operations, canonicalize paths, and return an internal validated config. Add `cargo:rerun-if-changed=<model>` only in `compile`, never in unit-test validation. Run the focused tests, `cargo fmt --check`, and commit:

```bash
cargo test -p aws-sdk-build --test model_selection
cargo fmt --check
git add crates/aws-sdk-build tests/fixtures/selection-model.json
git commit -m "feat: prune Smithy models to selected operations"
```

### Task 3: Generate Smithy configuration and implement the process runner

**Files:**
- Create: `crates/aws-sdk-build/src/smithy.rs`
- Create: `crates/aws-sdk-build/src/runner.rs`
- Modify: `crates/aws-sdk-build/src/lib.rs`
- Modify: `crates/aws-sdk-build/src/error.rs`
- Create: `crates/aws-sdk-build/tests/smithy_config.rs`
- Create: `crates/aws-sdk-build/tests/runner.rs`

**Interfaces:**
- `smithy::BuildConfig::new(model_file, service_id, output_dir, codegen_coordinate) -> BuildConfig`.
- `BuildConfig::to_json() -> serde_json::Value`.
- `runner::resolve_executable(explicit: Option<&Path>, env: impl Fn(&str) -> Option<OsString>, path_lookup: impl Fn(&OsStr) -> Option<PathBuf>) -> Result<PathBuf, BuildError>`.
- `runner::run(executable: &Path, cwd: &Path) -> Result<ExitStatus, BuildError>`.

- [ ] **Step 1: Write failing configuration and resolution tests**

Assert that the generated Smithy config has `version: "1.0"`, `outputDirectory: "output"`, an import for the pruned model, a projection named `aws-sdk`, and the `rust-client-codegen` plugin containing the selected service, module name, fluent-client setting, and the AWS customization flag. Assert executable precedence: explicit path, then `SMITHY_CLI`, then PATH; assert an error includes all lookup sources when no executable is found.

- [ ] **Step 2: Run focused tests and verify the expected missing-module failure**

Run: `cargo test -p aws-sdk-build --test smithy_config --test runner`

Expected: compilation fails because Smithy config and runner modules are absent.

- [ ] **Step 3: Implement deterministic Smithy config generation**

Write the pruned model as `model.json` under a temporary workspace. Generate a config using relative paths and:

```json
{
  "version": "1.0",
  "outputDirectory": "output",
  "imports": ["model.json"],
  "maven": {
    "dependencies": [
      "software.amazon.smithy.rust:codegen-aws-sdk:0.1.25"
    ]
  },
  "projections": {
    "aws-sdk": {
      "plugins": {
        "rust-client-codegen": {
          "service": "example#Service",
          "module": "generated_sdk",
          "moduleVersion": "0.1.0",
          "moduleAuthors": ["aws-sdk-build"],
          "codegen": { "includeFluentClient": true },
          "customizationConfig": {
            "awsSdk": { "awsSdkBuild": false, "suppressReadme": true }
          }
        }
      }
    }
  }
}
```

Use the configured coordinate in place of the default and ensure the JSON output is stable. Configure `module` from the service terminal name converted to snake-case and append `_sdk`.

- [ ] **Step 4: Implement executable resolution and command execution**

Resolve an explicit path only if it exists and is a file; resolve `SMITHY_CLI` as a path; otherwise search each PATH component for an executable named `smithy`. Run `[executable, "build"]` with the temporary workspace as current directory. Capture stdout and stderr, return a structured error for spawn failure/non-zero exit, and include command, status, and output in the error. Do not shell-escape or invoke through a shell.

- [ ] **Step 5: Run focused tests and verify they pass**

Run: `cargo test -p aws-sdk-build --test smithy_config --test runner`

Expected: PASS with config snapshots, executable precedence, and process-error assertions passing without Smithy installed.

- [ ] **Step 6: Commit the runner milestone**

Run `cargo fmt --check`, `cargo test -p aws-sdk-build`, then commit:

```bash
git add crates/aws-sdk-build
git commit -m "feat: invoke Smithy Rust codegen from build scripts"
```

### Task 4: Materialize output atomically and expose generated include metadata

**Files:**
- Create: `crates/aws-sdk-build/src/output.rs`
- Modify: `crates/aws-sdk-build/src/lib.rs`
- Modify: `crates/aws-sdk-build/src/runner.rs`
- Create: `crates/aws-sdk-build/tests/output.rs`

**Interfaces:**
- `output::install(generated_dir, out_dir, operations, service) -> Result<CompileReport, BuildError>`.
- Generated `OUT_DIR/aws_sdk.rs` includes the Smithy projection's Rust `lib.rs` using a path relative to `OUT_DIR`.
- Generated `OUT_DIR/aws_sdk_build_manifest.json` records `service`, sorted `operations`, and generated Rust file paths.

- [ ] **Step 1: Write the failing atomic-install tests**

Create a fake generated projection containing `src/lib.rs`, `Cargo.toml`, and a nested module. Assert that installation writes `aws_sdk.rs` and the manifest, lists all generated Rust files, and excludes the generated service's Cargo manifest from the include root. Pre-create an old `aws_sdk.rs`, force an installation error, and assert its contents remain unchanged.

- [ ] **Step 2: Run the focused output tests and verify they fail**

Run: `cargo test -p aws-sdk-build --test output`

Expected: compilation fails because the output installer is absent.

- [ ] **Step 3: Implement staging and atomic replacement**

Build all generated artifacts in a temporary sibling directory under `OUT_DIR`. Locate the projection output at `output/aws-sdk` or the first projection directory containing `src/lib.rs`. Copy Rust source files into `OUT_DIR/generated`, create `aws_sdk.rs` with a single `include!` of the generated `src/lib.rs` using `include!(concat!(env!("OUT_DIR"), "/generated/src/lib.rs"));`, and serialize the manifest with sorted relative paths. Rename the complete staging directory into place only after every copy and manifest write succeeds; on Unix use a same-filesystem rename so partial output is never visible.

- [ ] **Step 4: Wire `compile` end-to-end through staging**

Create a `tempfile::TempDir`, write the selected model/config there, invoke the runner, install output, print `cargo:rerun-if-changed` for the original model path and `cargo:rerun-if-env-changed=SMITHY_CLI`, and return the report. Ensure `compile` rejects a missing `out_dir` with a clear error instead of defaulting to the current directory.

- [ ] **Step 5: Run focused tests and commit**

Run:

```bash
cargo test -p aws-sdk-build
cargo fmt --check
git add crates/aws-sdk-build
git commit -m "feat: install generated SDK output atomically"
```

### Task 5: Add a real consumer fixture and Smithy-rs integration path

**Files:**
- Create: `examples/generated-consumer/Cargo.toml`
- Create: `examples/generated-consumer/build.rs`
- Create: `examples/generated-consumer/src/lib.rs`
- Create: `examples/generated-consumer/src/main.rs`
- Create: `examples/generated-consumer/model/service.json`
- Create: `examples/generated-consumer/README.md`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Create: `scripts/check-smithy-integration.sh`

**Interfaces:**
- The example's `build.rs` contains only model/service/operation selection and `compile()`.
- The example includes `OUT_DIR/aws_sdk.rs` and demonstrates generated `Client`, operation builder, and input/output names.
- The integration script runs the example's `cargo check` only when `smithy` is installed and reports a clear skip otherwise; CI can enable it with `AWS_SDK_BUILD_SMITHY_INTEGRATION=1`.

- [ ] **Step 1: Add the failing compile contract test**

Add a test that reads the example `build.rs` and asserts it calls `configure`, `model`, `service`, `operations`, and `compile`, and does not contain a hand-written client or protocol implementation. Add a compile-time example assertion that includes the generated root. It must initially fail because the example and generated output do not exist.

- [ ] **Step 2: Run the example contract test and verify it fails for missing files**

Run: `cargo test -p aws-sdk-build --test consumer_contract`

Expected: failure identifying the absent example build script or include target.

- [ ] **Step 3: Create the Smithy fixture and consumer**

Use an AWS-like Smithy JSON model containing an HTTP protocol trait, an endpoint trait, `GetThing`, `PutThing`, shared error, and disjoint payload shapes. The build script selects only `GetThing`. The consumer's normal dependencies use the smithy-rs checkout's current runtime version policy: every crate listed in `buildSrc/src/main/kotlin/CrateSet.kt` under `StableCrates` is pinned to `1.1.7`, and every other AWS/Smithy runtime crate emitted by the generator is pinned to `0.60.6`. Keep the example's crate feature set minimal.

- [ ] **Step 4: Implement the integration check script**

When `AWS_SDK_BUILD_SMITHY_INTEGRATION=1`, require `smithy` and run `cargo check --manifest-path examples/generated-consumer/Cargo.toml`; otherwise print `smithy integration skipped: install Smithy CLI and set AWS_SDK_BUILD_SMITHY_INTEGRATION=1`. The script must use `set -eu`, pass through Smithy output, and return the cargo exit status.

- [ ] **Step 5: Run the contract test and default offline checks**

Run:

```bash
cargo test -p aws-sdk-build --test consumer_contract
cargo test --workspace
```

Expected: the contract test and all offline unit tests pass. The real Smithy compile remains an explicit environment-dependent gate.

- [ ] **Step 6: Run the real Smithy integration when available and document evidence**

If `smithy` is present, run `AWS_SDK_BUILD_SMITHY_INTEGRATION=1 scripts/check-smithy-integration.sh`; inspect `OUT_DIR` diagnostics on failure and fix the Smithy config, runtime versions, or generated include path. If it is absent, do not claim end-to-end completion; document the exact prerequisite and keep the goal active.

- [ ] **Step 7: Commit the consumer milestone**

Run `cargo fmt --check`, the offline workspace tests, and commit:

```bash
git add Cargo.toml README.md examples scripts
git commit -m "test: add Smithy-generated modular consumer"
```

### Task 6: Full verification and requirement audit

**Files:**
- Modify: `README.md`
- Modify: `docs/superpowers/specs/2026-08-21-aws-sdk-build-design.md` only if implementation evidence requires a clarified constraint
- Create: `docs/verification/2026-08-21-aws-sdk-build.md`

- [ ] **Step 1: Run the complete offline verification suite**

Run exactly:

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

Record the exit status and test counts in the verification note.

- [ ] **Step 2: Run the Smithy-rs integration gate**

Run the integration script with the pinned codegen coordinate. Verify generated output has `Client`, `Config`, `GetThing` builder/input/output, and no `PutThing` symbols or disjoint `PutThing` shapes. Verify generated Cargo metadata contains no `aws-sdk-*` dependency and does contain `aws-runtime`/Smithy runtime dependencies.

- [ ] **Step 3: Audit every requirement against evidence**

In the verification note, link each requirement to a source file and command output: build.rs API, selected-operation pruning, Smithy invocation, AWS-shaped generated API, runtime dependency boundary, atomic failure behavior, and full test gates. Explicitly mark the Smithy integration as verified or unavailable; do not mark the overall goal complete if the real generator path has not compiled.

- [ ] **Step 4: Final verification before any completion claim**

Re-run the relevant command after the final edit, inspect `git status --short`, and only then report status. If all requirements are proven, call `update_goal` with `status: "complete"`; otherwise keep the goal active and report the concrete remaining evidence gap.
