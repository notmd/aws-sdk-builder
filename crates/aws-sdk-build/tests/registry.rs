use aws_sdk_build::registry;

#[test]
fn registry_contains_the_priority_services_and_pinned_sources() {
    for service in [
        "s3",
        "dynamodb",
        "lambda",
        "sqs",
        "sns",
        "sts",
        "iam",
        "kms",
        "cloudwatch-logs",
    ] {
        assert!(registry::lookup(service).is_ok(), "missing {service}");
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
    assert!(error.contains("models-manifest.json"));
}

#[test]
fn service_module_mapping_keeps_cloudwatch_logs_namespace() {
    let entry = registry::lookup("cloudwatch-logs").unwrap();
    assert_eq!(entry.crate_name, "aws-sdk-cloudwatchlogs");
    assert_eq!(entry.module_name, "aws_sdk_cloudwatchlogs");
}

#[test]
fn service_module_mapping_uses_ec2_auto_scaling_names() {
    let entry = registry::lookup("autoscaling").unwrap();
    assert_eq!(entry.crate_name, "aws-sdk-auto-scaling");
    assert_eq!(entry.module_name, "aws_sdk_auto_scaling");
}
