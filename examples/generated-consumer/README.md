# Generated consumer

This fixture demonstrates the intended consumer contract. `build.rs` selects only `GetThing`; it does not contain a handwritten client or protocol implementation. The generated API is included from `OUT_DIR/aws_sdk.rs`.

Run from the repository root:

```bash
AWS_SDK_BUILD_SMITHY_INTEGRATION=1 scripts/check-smithy-integration.sh
```

