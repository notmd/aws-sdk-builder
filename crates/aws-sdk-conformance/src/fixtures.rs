use aws_sdk_builder::IntegrationTestAsset;

pub static S3_PROTOCOL_TESTS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/s3-protocol-tests.json"
));

pub static S3_INTEGRATION_TESTS: &[IntegrationTestAsset] = &[
    IntegrationTestAsset {
        path: "tests/alternative-async-runtime.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/alternative-async-runtime.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/auth_scheme_preference.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/auth_scheme_preference.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/aws_chunked.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/aws_chunked.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/blns/LICENSE",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/blns/LICENSE"
        )),
    },
    IntegrationTestAsset {
        path: "tests/blns/blns.txt",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/blns/blns.txt"
        )),
    },
    IntegrationTestAsset {
        path: "tests/body_size_hint.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/body_size_hint.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/bucket-required.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/bucket-required.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/business_metrics.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/business_metrics.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/checksums.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/checksums.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/client_construction.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/client_construction.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/concurrency.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/concurrency.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/config-override.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/config-override.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/config_to_builder.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/config_to_builder.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/content-length-enforcement.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/content-length-enforcement.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/credential_features.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/credential_features.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/customizable-operation.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/customizable-operation.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/aws_chunked/chunk-signing.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/aws_chunked/chunk-signing.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/aws_chunked/custom-chunk-size.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/aws_chunked/custom-chunk-size.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/aws_chunked/no-chunking.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/aws_chunked/no-chunking.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/content-length-enforcement/get-object-long.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/content-length-enforcement/get-object-long.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/content-length-enforcement/get-object-short.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/content-length-enforcement/get-object-short.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/content-length-enforcement/head-object.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/content-length-enforcement/head-object.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/express/mixed-auths.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/express/mixed-auths.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/no_auth/get-object.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/no_auth/get-object.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/no_auth/head-object.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/no_auth/head-object.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/no_auth/list-objects-v2.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/no_auth/list-objects-v2.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/no_auth/list-objects.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/no_auth/list-objects.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/request-information-headers/slow-network-and-late-client-clock.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/request-information-headers/slow-network-and-late-client-clock.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/request-information-headers/three-retries_and-then-success.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/request-information-headers/three-retries_and-then-success.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/data/request-information-headers/three-successful-attempts.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/data/request-information-headers/three-successful-attempts.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/endpoints.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/endpoints.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/expires_interceptor.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/expires_interceptor.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/express.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/express.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/identity-cache.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/identity-cache.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/ignore-invalid-xml-body-root.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/ignore-invalid-xml-body-root.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/interceptors.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/interceptors.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/mocks.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/mocks.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/naughty-string-metadata.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/naughty-string-metadata.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/no_auth.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/no_auth.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/normalize-uri-path.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/normalize-uri-path.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/presigning.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/presigning.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/query-strings-are-correctly-encoded.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/query-strings-are-correctly-encoded.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/reconnects.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/reconnects.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/recursion-detection.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/recursion-detection.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/request_id.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/request_id.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/request_information_headers.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/request_information_headers.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/required-query-params.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/required-query-params.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/retry-classifier-customization.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/retry-classifier-customization.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/select-object-content.json",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/select-object-content.json"
        )),
    },
    IntegrationTestAsset {
        path: "tests/select-object-content.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/select-object-content.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/service_timeout_overrides.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/service_timeout_overrides.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/signing-it.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/signing-it.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/sigv4a_signing_region_set.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/sigv4a_signing_region_set.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/size-type.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/size-type.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/stalled-stream-protection.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/stalled-stream-protection.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/status-200-errors.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/status-200-errors.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/streaming-response.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/streaming-response.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/timeouts.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/timeouts.rs"
        )),
    },
    IntegrationTestAsset {
        path: "tests/token_bucket_time_source.rs",
        bytes: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../conformance/fixtures/integration-tests/s3/tests/token_bucket_time_source.rs"
        )),
    },
];
