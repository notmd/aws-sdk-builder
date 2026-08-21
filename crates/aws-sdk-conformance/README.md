# aws-sdk-conformance

This crate compares generated AWS SDK service trees with pinned reference trees and
writes a deterministic Markdown report. It uses [`diffy`](https://docs.rs/diffy/latest/diffy/)
in memory, so no external diff program is required.

```text
cargo run -p aws-sdk-conformance -- \
  --reference build/conformance/reference \
  --generated build/conformance/generated \
  --output reports/aws-sdk-conformance.md \
  --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db
```

Both input roots contain one directory per SDK service. The report places the completed
`compared/total` progress line immediately below each service heading, followed by
`diffy` unified patches and explicit missing/extra/binary diagnostics. Exit status `1`
means differences were written; exit status `2` means the reporter itself failed.
