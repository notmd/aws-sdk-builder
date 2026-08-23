use aws_sdk_builder::registry;

#[test]
fn registry_contains_exactly_the_supported_service_crates() {
    let expected = [
        ("dynamodb", "aws-sdk-dynamodb", "aws_sdk_dynamodb"),
        ("iam", "aws-sdk-iam", "aws_sdk_iam"),
        ("kms", "aws-sdk-kms", "aws_sdk_kms"),
        ("lambda", "aws-sdk-lambda", "aws_sdk_lambda"),
        ("s3", "aws-sdk-s3", "aws_sdk_s3"),
        ("sns", "aws-sdk-sns", "aws_sdk_sns"),
        ("sqs", "aws-sdk-sqs", "aws_sdk_sqs"),
        ("sts", "aws-sdk-sts", "aws_sdk_sts"),
    ];
    assert_eq!(registry::entries().len(), expected.len());
    for (service, crate_name, module_name) in expected {
        let entry = registry::lookup(service).unwrap();
        assert_eq!(entry.crate_name, crate_name);
        assert_eq!(entry.module_name, module_name);
        assert_eq!(entry.filename, "model.json");
        assert_eq!(entry.model_sha256.len(), 64);
    }
    assert_eq!(
        registry::AWS_SDK_RUST_SNAPSHOT,
        "3c6d526c9d4775f41a8ef1ed2ef574d1b14481db"
    );
    assert_eq!(
        registry::SMITHY_RS_SNAPSHOT,
        "f1b64a9c0dd001d4bac4277fec4041da59c1f48d"
    );
}

#[test]
fn registry_rejects_unknown_service_with_source() {
    let error = registry::lookup("not-a-service").unwrap_err().to_string();
    assert!(error.contains("not-a-service"));
    assert!(error.contains("aws-sdk-builder"));
}

#[test]
fn registry_rejects_removed_services() {
    for service in ["appconfig", "cloudwatch", "ec2", "route-53", "wafv2"] {
        assert!(
            registry::lookup(service).is_err(),
            "removed service {service}"
        );
    }
}
