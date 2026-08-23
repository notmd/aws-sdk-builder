conformance-sync:
    cargo run -p aws-sdk-conformance -- update-reference --manifest services-manifest.json

conformance:
    cargo run -p aws-sdk-conformance -- conformance --manifest services-manifest.json
