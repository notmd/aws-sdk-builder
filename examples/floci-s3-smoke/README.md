# Floci S3 runtime smoke test

This small Rust program checks that the AWS S3 runtime can talk to a local Floci
emulator. It creates a bucket, writes and reads one object, lists it, and deletes the
object and bucket.

Start the Docker emulator, then run from the repository root:

```text
docker compose up -d
scripts/check-s3-floci.sh
```

If you do not use Compose, the equivalent container command is:

```text
docker run --rm -d --name floci -p 4566:4566 floci/floci:latest
```

The launcher checks endpoint reachability and runs this Rust client. To run it
directly:

```text
cargo run --manifest-path examples/floci-s3-smoke/Cargo.toml
```

The default endpoint is `http://127.0.0.1:4566`; override it with
`AWS_ENDPOINT_URL`. This is a runtime/protocol smoke test, not proof that the new
generated Rust code has reached AWS SDK source parity.
