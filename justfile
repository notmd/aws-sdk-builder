conformance:
    cargo run -p aws-sdk-conformance -- \
        --reference conformance/reference \
        --generated conformance/generated \
        --output conformance/summary.md \
        --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db
