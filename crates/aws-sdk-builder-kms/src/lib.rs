//! Build-time model provider for kms AWS service.

pub const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.json"));

pub const METADATA: aws_sdk_builder::ServiceMetadata = aws_sdk_builder::ServiceMetadata {
    key: "kms",
    service_shape_id: "com.amazonaws.kms#TrentService",
    filename: "model.json",
    crate_name: "aws-sdk-kms",
    module_name: "aws_sdk_kms",
    sdk_version: Some("1.116.0"),
    model_sha256: "82e414ec6095a69dabe0e7de071a5b9704c413d3277ed1b293d3cbcd8dd6ce5f",
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
