# Checked-in conformance snapshots

This directory contains checked-in conformance inputs and reports:

- `reference/` is copied from the AWS SDK for Rust repository and the exact commit in
  [`../services-manifest.json`](../services-manifest.json).
- `generated/` is the all-operation output from the Rust-only generator using the
  packaged service models.
- `summary.md` and `summary/` contain deterministic comparison reports.

Run `just conformance-sync` to download the pinned upstream commit archive, refresh selected
service reference trees, and update each service provider's `model.json`. The updater
removes configured Cargo metadata, README files, tests, and benches while preserving
source and license files. It stages all data first and replaces snapshots only after
validation succeeds.

Run `just conformance` to regenerate every selected service, remove the same excluded
artifacts from generated snapshots, compare the sanitized trees, refresh reports, and
update derived file counts in `services-manifest.json`. Differences intentionally
produce exit status 1. Percentages are based on exact file matches; 100.00% means
fully matched.

Normal conformance runs use checked-in data and do not clone or invoke an external
code generator. `just conformance-sync` is the explicit network operation.
