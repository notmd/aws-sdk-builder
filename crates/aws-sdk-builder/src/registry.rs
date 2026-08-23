use crate::error::BuildError;

pub const AWS_SDK_RUST_SNAPSHOT: &str = "3c6d526c9d4775f41a8ef1ed2ef574d1b14481db";
pub const SMITHY_RS_SNAPSHOT: &str = "f1b64a9c0dd001d4bac4277fec4041da59c1f48d";
pub const GENERATOR_VERSION: &str = "aws-sdk-builder-rust-native-0.2.0";
pub const SMITHY_CODEGEN_VERSION: &str = "8c50a50e36736f932bd51898f14ee0fa84d47c09";
pub const REGISTRY_SOURCE: &str = "aws-sdk-builder service-crate registry";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceMetadata {
    pub key: &'static str,
    pub filename: &'static str,
    pub crate_name: &'static str,
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

    pub const fn with_fixtures(
        mut self,
        protocol_tests: Option<&'static [u8]>,
        integration_tests: &'static [IntegrationTestAsset],
    ) -> Self {
        self.protocol_tests = protocol_tests;
        self.integration_tests = integration_tests;
        self
    }
}

macro_rules! metadata {
    ($key:literal, $crate_name:literal, $module:literal, $version:expr) => {
        ServiceMetadata {
            key: $key,
            filename: "model.json",
            crate_name: $crate_name,
            module_name: $module,
            sdk_version: $version,
        }
    };
}

static ENTRIES: &[ServiceMetadata] = &[
    metadata!(
        "dynamodb",
        "aws-sdk-dynamodb",
        "aws_sdk_dynamodb",
        Some("1.122.0")
    ),
    metadata!("iam", "aws-sdk-iam", "aws_sdk_iam", Some("1.121.0")),
    metadata!("kms", "aws-sdk-kms", "aws_sdk_kms", Some("1.116.0")),
    metadata!(
        "lambda",
        "aws-sdk-lambda",
        "aws_sdk_lambda",
        Some("1.140.0")
    ),
    metadata!("s3", "aws-sdk-s3", "aws_sdk_s3", Some("1.143.0")),
    metadata!("sns", "aws-sdk-sns", "aws_sdk_sns", Some("1.109.0")),
    metadata!("sqs", "aws-sdk-sqs", "aws_sdk_sqs", Some("1.107.0")),
    metadata!("sts", "aws-sdk-sts", "aws_sdk_sts", Some("1.112.0")),
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
            registry: REGISTRY_SOURCE.to_owned(),
        })
}
