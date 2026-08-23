//! Build-time model provider for sns AWS service.

pub const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.json"));

pub const METADATA: aws_sdk_builder::ServiceMetadata = aws_sdk_builder::ServiceMetadata {
    key: "sns",
    service_shape_id: "com.amazonaws.sns#AmazonSimpleNotificationService",
    filename: "model.json",
    crate_name: "aws-sdk-sns",
    module_name: "aws_sdk_sns",
    sdk_version: Some("1.109.0"),
    model_sha256: "ab2f70d13a3e389a1ccc29471f926017bf0482d76804197aa2d305657feba7c3",
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
