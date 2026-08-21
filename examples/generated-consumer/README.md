# Generated consumer

This fixture demonstrates the intended consumer contract. `build.rs` selects two
S3 operations and `src/lib.rs` includes the stable `OUT_DIR/aws_sdk.rs` facade.

Run from the repository root:

```bash
cargo check --manifest-path examples/generated-consumer/Cargo.toml
```
