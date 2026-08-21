# my_aws_sdk consumer

This fixture is a binary consumer of the generated S3 client. `build.rs` selects
`CreateBucket` and `HeadBucket`, and `src/main.rs` includes the stable
`OUT_DIR/aws_sdk.rs` facade. Its unit test talks to the local Floci endpoint,
creates a fixed bucket, tolerates the two modeled duplicate-bucket errors, and
then heads the bucket.

Run from the repository root:

```bash
cargo check --manifest-path examples/my_aws_sdk/Cargo.toml
cargo test --manifest-path examples/my_aws_sdk/Cargo.toml
```

The generated client defaults to `http://localhost:4566`, so start Floci before
running the test.
