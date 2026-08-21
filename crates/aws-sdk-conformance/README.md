# aws-sdk-conformance

This binary generates all packaged AWS SDK operations, compares the resulting
service trees with pinned reference trees, and writes a deterministic Markdown
report. It uses [`diffy`](https://docs.rs/diffy/latest/diffy/) in memory, so no
external diff program is required.

```text
cargo run -p aws-sdk-conformance -- \
  --reference conformance/reference \
  --generated conformance/generated \
  --output conformance/summary.md \
  --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db
```

The reference root determines which services are generated. The generated root
is atomically replaced with fresh all-operation output before comparison. The
report places the completed `compared/total` progress line immediately below
each service heading, followed by its match percentage. The output path is the
summary report; detailed `diffy` patches and explicit missing/extra/binary
diagnostics are written to `conformance/summary/<service>.md`. Exit status `1`
means differences were written; exit status `2` means generation or reporting
failed.
