# Checked-in conformance snapshots

This directory contains the immutable P0 inputs for the source comparison:

- `reference/` is copied from the AWS SDK for Rust `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
  snapshot.
- `generated/` is the checked-in all-operation output from the Rust-only
  `aws-sdk-build-rust-native-0.1.0` generator using the packaged models in this
  repository.
- `manifest.json` records the provenance and expected file counts.

Run `just conformance` to compare these trees and refresh the summary table in
`summary.md` plus the per-service Markdown reports under `summary/`. Differences
intentionally produce exit status 1. Percentages are based on exact file matches;
100.00% means fully matched.
