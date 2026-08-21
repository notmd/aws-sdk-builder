conformance:
    cargo run -p aws-sdk-conformance -- \
        --reference conformance/reference \
        --generated conformance/generated \
        --output reports/aws-sdk-conformance.md \
        --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db
