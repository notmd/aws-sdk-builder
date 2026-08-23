//! Build-time model provider for lambda AWS service.

pub const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.json"));

pub const METADATA: aws_sdk_builder::ServiceMetadata = aws_sdk_builder::ServiceMetadata {
    key: "lambda",
    service_shape_id: "com.amazonaws.lambda#AWSGirApiService",
    filename: "model.json",
    crate_name: "aws-sdk-lambda",
    module_name: "aws_sdk_lambda",
    sdk_version: Some("1.140.0"),
    model_sha256: "a041b314d008e743535e36e0ce3a2148ee28a46f6dbda79af4f03ba9f984f57b",
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
