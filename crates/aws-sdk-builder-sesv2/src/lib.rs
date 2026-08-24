//! Build-time model provider for sesv2 AWS service.

pub const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.json"));

pub const METADATA: aws_sdk_builder::ServiceMetadata = aws_sdk_builder::ServiceMetadata {
    key: "sesv2",
    filename: "model.json",
    module_name: "aws_sdk_sesv2",
    sdk_version: Some("1.130.0"),
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

/// Includes the generated SES v2 SDK inside the caller's module.
#[macro_export]
macro_rules! include_sdk {
    () => {
        include!(concat!(env!("OUT_DIR"), "/generated/sesv2/src/lib.rs"));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn packaged_model_is_valid() {
        aws_sdk_builder::validate_model(super::METADATA, super::MODEL).unwrap();
    }
}
