use crate::error::BuildError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceMetadata {
    pub key: &'static str,
    pub filename: &'static str,
    pub module_name: &'static str,
    pub sdk_version: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrationTestAsset {
    pub path: &'static str,
    pub bytes: &'static [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct ServiceSource {
    pub metadata: ServiceMetadata,
    pub model: &'static [u8],
    pub protocol_tests: Option<&'static [u8]>,
    pub integration_tests: &'static [IntegrationTestAsset],
}

impl ServiceSource {
    pub const fn new(metadata: ServiceMetadata, model: &'static [u8]) -> Self {
        Self {
            metadata,
            model,
            protocol_tests: None,
            integration_tests: &[],
        }
    }
}

macro_rules! metadata {
    ($key:literal, $module:literal, $version:expr) => {
        ServiceMetadata {
            key: $key,
            filename: "model.json",
            module_name: $module,
            sdk_version: $version,
        }
    };
}

static ENTRIES: &[ServiceMetadata] = &[
    metadata!("dynamodb", "aws_sdk_dynamodb", Some("1.122.0")),
    metadata!("iam", "aws_sdk_iam", Some("1.121.0")),
    metadata!("kms", "aws_sdk_kms", Some("1.116.0")),
    metadata!("lambda", "aws_sdk_lambda", Some("1.140.0")),
    metadata!("s3", "aws_sdk_s3", Some("1.143.0")),
    metadata!("sns", "aws_sdk_sns", Some("1.109.0")),
    metadata!("sqs", "aws_sdk_sqs", Some("1.107.0")),
    metadata!("sts", "aws_sdk_sts", Some("1.112.0")),
];

pub fn entries() -> &'static [ServiceMetadata] {
    ENTRIES
}

pub fn lookup(key: &str) -> Result<ServiceMetadata, BuildError> {
    ENTRIES
        .iter()
        .copied()
        .find(|entry| entry.key == key)
        .ok_or_else(|| BuildError::UnknownService {
            service: key.to_owned(),
            registry: "aws-sdk-builder service registry".to_owned(),
        })
}
