# aws-sdk-build

aws-sdk-build is a Rust-only build dependency for generating a selected,
consumer-prefixed AWS SDK module from packaged Smithy JSON snapshots.

    [dependencies]
    aws-sdk-build = "0.1"

    [build-dependencies]
    aws-sdk-build = "0.1"

    // build.rs
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        aws_sdk_build::configure()
            .add("s3", ["AbortMultipartUpload", "CompleteMultipartUpload"])
            .add("dynamodb", ["GetItem"])
            .compile()?;
        Ok(())
    }

    // src/lib.rs
    aws_sdk_build::include_sdk!();

compile() discovers OUT_DIR and CARGO_PKG_NAME from Cargo. Service models are
immutable packaged assets selected by short service key; consumers do not
provide model paths, shape IDs, output directories, Smithy executables, or
codegen plugins. Empty operation iterators select all operations. Repeated
service entries are merged deterministically.

The generated facade exposes paths such as
consumer_crate_name::aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload.
Generated output includes a deterministic manifest and is installed atomically,
so a failed build cannot replace a previous result.

The pinned snapshot metadata and model checksums are in
crates/aws-sdk-build/models-manifest.json. The conformance harness is invoked
with:

    cargo run -p aws-sdk-conformance -- \
      --reference build/conformance/reference \
      --generated build/conformance/generated \
      --output reports/aws-sdk-conformance.md \
      --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db

For a local S3 emulator smoke test, start Floci yourself and run
scripts/check-s3-floci.sh. The launcher never starts or stops the emulator
and refuses non-loopback endpoints unless ALLOW_NONLOCAL_FLOCI=1 is set.
