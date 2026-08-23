use crate::error::BuildError;

pub const AWS_SDK_RUST_SNAPSHOT: &str = "3c6d526c9d4775f41a8ef1ed2ef574d1b14481db";
pub const SMITHY_RS_SNAPSHOT: &str = "f1b64a9c0dd001d4bac4277fec4041da59c1f48d";
pub const GENERATOR_VERSION: &str = "aws-sdk-builder-rust-native-0.2.0";
pub const SMITHY_CODEGEN_VERSION: &str = "8c50a50e36736f932bd51898f14ee0fa84d47c09";
pub const REGISTRY_SOURCE: &str = "aws-sdk-builder service-crate registry";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceMetadata {
    pub key: &'static str,
    pub service_shape_id: &'static str,
    pub filename: &'static str,
    pub crate_name: &'static str,
    pub module_name: &'static str,
    pub sdk_version: Option<&'static str>,
    pub model_sha256: &'static str,
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
    ($key:literal, $shape:literal, $crate_name:literal, $module:literal, $checksum:literal, $version:expr) => {
        ServiceMetadata {
            key: $key,
            service_shape_id: $shape,
            filename: "model.json",
            crate_name: $crate_name,
            module_name: $module,
            sdk_version: $version,
            model_sha256: $checksum,
        }
    };
}

static ENTRIES: &[ServiceMetadata] = &[
    metadata!(
        "dynamodb",
        "com.amazonaws.dynamodb#DynamoDB_20120810",
        "aws-sdk-dynamodb",
        "aws_sdk_dynamodb",
        "7e9436596b667e2e0600de9c086205b38960012028bcd625ee786a5126d0f63e",
        Some("1.122.0")
    ),
    metadata!(
        "iam",
        "com.amazonaws.iam#AWSIdentityManagementV20100508",
        "aws-sdk-iam",
        "aws_sdk_iam",
        "c0cec5eeceeffdf03f310ae41c30d072041dbc274bc5a388cad525847de1bec2",
        Some("1.121.0")
    ),
    metadata!(
        "kms",
        "com.amazonaws.kms#TrentService",
        "aws-sdk-kms",
        "aws_sdk_kms",
        "82e414ec6095a69dabe0e7de071a5b9704c413d3277ed1b293d3cbcd8dd6ce5f",
        Some("1.116.0")
    ),
    metadata!(
        "lambda",
        "com.amazonaws.lambda#AWSGirApiService",
        "aws-sdk-lambda",
        "aws_sdk_lambda",
        "a041b314d008e743535e36e0ce3a2148ee28a46f6dbda79af4f03ba9f984f57b",
        Some("1.140.0")
    ),
    metadata!(
        "s3",
        "com.amazonaws.s3#AmazonS3",
        "aws-sdk-s3",
        "aws_sdk_s3",
        "85e30a81b268de1b569bc1611a77fac7422198402f616149164a900285eb44e7",
        Some("1.143.0")
    ),
    metadata!(
        "sns",
        "com.amazonaws.sns#AmazonSimpleNotificationService",
        "aws-sdk-sns",
        "aws_sdk_sns",
        "ab2f70d13a3e389a1ccc29471f926017bf0482d76804197aa2d305657feba7c3",
        Some("1.109.0")
    ),
    metadata!(
        "sqs",
        "com.amazonaws.sqs#AmazonSQS",
        "aws-sdk-sqs",
        "aws_sdk_sqs",
        "38eef06ce5642b8a052282fd19a3ab275e0046d375474e2ff022d3e33aaeab72",
        Some("1.107.0")
    ),
    metadata!(
        "sts",
        "com.amazonaws.sts#AWSSecurityTokenServiceV20110615",
        "aws-sdk-sts",
        "aws_sdk_sts",
        "1645206cfffb5c92e39b10c54cac340592954ef67a872092ca1bd6941a5e6840",
        Some("1.112.0")
    ),
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
