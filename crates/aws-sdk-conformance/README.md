# aws-sdk-conformance

This crate compares generated AWS SDK service trees with pinned reference trees and
writes a deterministic Markdown report. It uses [`diffy`](https://docs.rs/diffy/latest/diffy/)
in memory, so no external diff program is required.

```text
cargo run -p aws-sdk-conformance -- \
  --reference conformance/reference \
  --generated conformance/generated \
  --output reports/aws-sdk-conformance.md \
  --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db
```

Both input roots contain one directory per SDK service. The report places the completed
`compared/total` progress line immediately below each service heading. The output path
is the summary report; detailed `diffy` patches and explicit missing/extra/binary
diagnostics are written to `reports/aws-sdk-conformance/<service>.md`. Exit status `1`
means differences were written; exit status `2` means the reporter itself failed.
