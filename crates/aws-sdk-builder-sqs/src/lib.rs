//! Build-time model provider for sqs AWS service.

pub const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.json"));

pub const METADATA: aws_sdk_builder::ServiceMetadata = aws_sdk_builder::ServiceMetadata {
    key: "sqs",
    service_shape_id: "com.amazonaws.sqs#AmazonSQS",
    filename: "model.json",
    crate_name: "aws-sdk-sqs",
    module_name: "aws_sdk_sqs",
    sdk_version: Some("1.107.0"),
    model_sha256: "38eef06ce5642b8a052282fd19a3ab275e0046d375474e2ff022d3e33aaeab72",
};

pub fn model() -> &'static [u8] {
    MODEL
}

pub fn source() -> aws_sdk_builder::ServiceSource {
    aws_sdk_builder::ServiceSource::new(METADATA, MODEL)
}

pub fn compile<O: aws_sdk_builder::OperationNames>(
    operations: O,
) -> Result<aws_sdk_builder::CompileReport, aws_sdk_builder::BuildError> {
    aws_sdk_builder::compile(METADATA, MODEL, operations)
}

#[cfg(test)]
mod tests {
    #[test]
    fn packaged_model_is_valid() {
        aws_sdk_builder::validate_model(super::METADATA, super::MODEL).unwrap();
    }
}
