# aws-sdk-conformance

This binary generates all packaged AWS SDK operations, compares the resulting
service trees with pinned reference trees, and writes a deterministic Markdown
report. It uses [`diffy`](https://docs.rs/diffy/latest/diffy/) in memory, so no
external diff program is required.

```text
cargo run -p aws-sdk-conformance -- conformance --manifest services-manifest.json
cargo run -p aws-sdk-conformance -- update-reference --manifest services-manifest.json
```

`services-manifest.json` selects services, pins the upstream commit, and defines
comparison exclusions. `update-reference` downloads that commit archive, refreshes reference
source, generates source-normalization `.patch` files, refreshes provider `model.json`
assets, and replaces all three data sets atomically. The conformance command atomically
replaces fresh all-operation output before comparison, then removes the same excluded
artifacts. Reference Rust patches rewrite parsed `crate::...` paths to relative
`super::...` paths and remove upstream inline `#[cfg(test)]` modules with their attached
attributes. The generator removes those test modules from generated output and writes the
all-operation snapshot with the relative paths expected by the patches; the comparator
only applies patches to the reference side. Checked-in reference patches are applied in
memory and never modify `conformance/reference`. The report places the completed
`compared/total` progress line immediately below each service heading, followed by its
match percentage. The output path is the summary report; detailed `diffy` patches and
explicit missing/extra/binary diagnostics are written to
`conformance/summary/<service>.md`. Exit status `1` means differences were written;
exit status `2` means generation or reporting failed.
