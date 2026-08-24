use aws_sdk_builder::registry;

#[test]
fn registry_contains_exactly_the_supported_service_crates() {
    let expected = [
        ("batch", "aws_sdk_batch"),
        ("bedrockruntime", "aws_sdk_bedrockruntime"),
        ("cloudwatchlogs", "aws_sdk_cloudwatchlogs"),
        ("codeartifact", "aws_sdk_codeartifact"),
        ("cognitoidentityprovider", "aws_sdk_cognitoidentityprovider"),
        ("config", "aws_sdk_config"),
        ("dynamodb", "aws_sdk_dynamodb"),
        ("iam", "aws_sdk_iam"),
        ("kms", "aws_sdk_kms"),
        ("lambda", "aws_sdk_lambda"),
        ("s3", "aws_sdk_s3"),
        ("sesv2", "aws_sdk_sesv2"),
        ("sns", "aws_sdk_sns"),
        ("sqs", "aws_sdk_sqs"),
        ("sts", "aws_sdk_sts"),
    ];
    assert_eq!(registry::entries().len(), expected.len());
    for (service, module_name) in expected {
        let entry = registry::lookup(service).unwrap();
        assert_eq!(entry.module_name, module_name);
        assert_eq!(entry.filename, "model.json");
    }
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
