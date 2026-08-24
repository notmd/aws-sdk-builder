# Checked-in conformance snapshots

This directory contains checked-in conformance inputs and reports:

- `reference/` is copied from the AWS SDK for Rust repository and the exact commit in
  [`../services-manifest.json`](../services-manifest.json).
- `generated/` is the all-operation output from the Rust-only generator using the
  packaged service models.
- `summary.md` and `summary/` contain deterministic comparison reports.

Run `just conformance-sync` to download the pinned upstream commit archive, refresh selected
service reference trees, generate source-normalization `.patch` files under
`conformance/patches/<service>/`, and update each service provider's `model.json`. The
updater removes configured Cargo metadata, README files, LICENSE files, tests, and
benches from the snapshots. It stages references, patches, and models together, then
replaces all snapshots only after validation succeeds.

Run `just conformance` to regenerate every selected service, remove the same excluded
artifacts from generated snapshots, compare the sanitized trees, and refresh reports.
Differences intentionally produce exit status 1. Percentages are based on exact file
matches; 100.00% means fully matched. File counts live in the reports rather than in
`services-manifest.json`.

Normal conformance runs use checked-in data and do not clone or invoke an external
code generator. Checked-in reference patches are applied in memory and never modify
`conformance/reference`; `just conformance-sync` is the explicit network operation
that regenerates them. Rust normalization rewrites parsed `crate::...` paths to
`super::...` and removes inline `#[cfg(test)]` modules, including their attached
test-only attributes, before comparison.
