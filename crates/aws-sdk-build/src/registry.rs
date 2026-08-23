use crate::error::BuildError;

pub const AWS_SDK_RUST_SNAPSHOT: &str = "3c6d526c9d4775f41a8ef1ed2ef574d1b14481db";
pub const SMITHY_RS_SNAPSHOT: &str = "f1b64a9c0dd001d4bac4277fec4041da59c1f48d";
pub const GENERATOR_VERSION: &str = "aws-sdk-build-rust-native-0.2.0";
pub const REGISTRY_SOURCE: &str = "aws-sdk-build/models-manifest.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelEntry {
    pub key: &'static str,
    pub service_shape_id: &'static str,
    pub filename: &'static str,
    pub crate_name: &'static str,
    pub module_name: &'static str,
    /// Version recorded in the pinned AWS SDK source snapshot, when available.
    ///
    /// This is package metadata used only for generated crate documentation; it
    /// is deliberately separate from the Smithy service version in the model.
    pub sdk_version: Option<&'static str>,
    pub bytes: &'static [u8],
    pub protocol_tests: Option<&'static [u8]>,
}

macro_rules! entry {
    ($key:literal, $shape:literal, $file:literal, $crate_name:literal, $module:literal) => {
        ModelEntry {
            key: $key,
            service_shape_id: $shape,
            filename: $file,
            crate_name: $crate_name,
            module_name: $module,
            sdk_version: None,
            bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/models/", $file)),
            protocol_tests: None,
        }
    };
}

macro_rules! entry_with_sdk_version {
    ($key:literal, $shape:literal, $file:literal, $crate_name:literal, $module:literal, $version:literal) => {
        ModelEntry {
            key: $key,
            service_shape_id: $shape,
            filename: $file,
            crate_name: $crate_name,
            module_name: $module,
            sdk_version: Some($version),
            bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/models/", $file)),
            protocol_tests: None,
        }
    };
}

macro_rules! entry_with_protocol_tests_and_sdk_version {
    ($key:literal, $shape:literal, $file:literal, $crate_name:literal, $module:literal, $tests:literal, $version:literal) => {
        ModelEntry {
            key: $key,
            service_shape_id: $shape,
            filename: $file,
            crate_name: $crate_name,
            module_name: $module,
            sdk_version: Some($version),
            bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/models/", $file)),
            protocol_tests: Some(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/models/",
                $tests
            ))),
        }
    };
}

static ENTRIES: &[ModelEntry] = &[
    entry!(
        "appconfig",
        "com.amazonaws.appconfig#AmazonAppConfig",
        "appconfig.json",
        "aws-sdk-appconfig",
        "aws_sdk_appconfig"
    ),
    entry!(
        "athena",
        "com.amazonaws.athena#AmazonAthena",
        "athena.json",
        "aws-sdk-athena",
        "aws_sdk_athena"
    ),
    entry!(
        "autoscaling",
        "com.amazonaws.autoscaling#AutoScaling_2011_01_01",
        "auto-scaling.json",
        "aws-sdk-auto-scaling",
        "aws_sdk_auto_scaling"
    ),
    entry!(
        "backup",
        "com.amazonaws.backup#CryoControllerUserManager",
        "backup.json",
        "aws-sdk-backup",
        "aws_sdk_backup"
    ),
    entry!(
        "bedrock",
        "com.amazonaws.bedrock#AmazonBedrockControlPlaneService",
        "bedrock.json",
        "aws-sdk-bedrock",
        "aws_sdk_bedrock"
    ),
    entry!(
        "bedrock-runtime",
        "com.amazonaws.bedrockruntime#AmazonBedrockFrontendService",
        "bedrock-runtime.json",
        "aws-sdk-bedrockruntime",
        "aws_sdk_bedrockruntime"
    ),
    entry!(
        "cognito-identity-provider",
        "com.amazonaws.cognitoidentityprovider#AWSCognitoIdentityProviderService",
        "cognito-identity-provider.json",
        "aws-sdk-cognitoidentityprovider",
        "aws_sdk_cognitoidentityprovider"
    ),
    entry!(
        "cloudfront",
        "com.amazonaws.cloudfront#Cloudfront2020_05_31",
        "cloudfront.json",
        "aws-sdk-cloudfront",
        "aws_sdk_cloudfront"
    ),
    entry!(
        "cloudwatch",
        "com.amazonaws.cloudwatch#GraniteServiceVersion20100801",
        "cloudwatch.json",
        "aws-sdk-cloudwatch",
        "aws_sdk_cloudwatch"
    ),
    entry!(
        "cloudwatch-logs",
        "com.amazonaws.cloudwatchlogs#Logs_20140328",
        "cloudwatch-logs.json",
        "aws-sdk-cloudwatchlogs",
        "aws_sdk_cloudwatchlogs"
    ),
    entry_with_sdk_version!(
        "dynamodb",
        "com.amazonaws.dynamodb#DynamoDB_20120810",
        "dynamodb.json",
        "aws-sdk-dynamodb",
        "aws_sdk_dynamodb",
        "1.122.0"
    ),
    entry!(
        "ec2",
        "com.amazonaws.ec2#AmazonEC2",
        "ec2.json",
        "aws-sdk-ec2",
        "aws_sdk_ec2"
    ),
    entry!(
        "ecr",
        "com.amazonaws.ecr#AmazonEC2ContainerRegistry_V20150921",
        "ecr.json",
        "aws-sdk-ecr",
        "aws_sdk_ecr"
    ),
    entry!(
        "ecs",
        "com.amazonaws.ecs#AmazonEC2ContainerServiceV20141113",
        "ecs.json",
        "aws-sdk-ecs",
        "aws_sdk_ecs"
    ),
    entry!(
        "eks",
        "com.amazonaws.eks#AWSWesleyFrontend",
        "eks.json",
        "aws-sdk-eks",
        "aws_sdk_eks"
    ),
    entry!(
        "elasticache",
        "com.amazonaws.elasticache#AmazonElastiCacheV9",
        "elasticache.json",
        "aws-sdk-elasticache",
        "aws_sdk_elasticache"
    ),
    entry!(
        "eventbridge",
        "com.amazonaws.eventbridge#AWSEvents",
        "eventbridge.json",
        "aws-sdk-eventbridge",
        "aws_sdk_eventbridge"
    ),
    entry!(
        "firehose",
        "com.amazonaws.firehose#Firehose_20150804",
        "firehose.json",
        "aws-sdk-firehose",
        "aws_sdk_firehose"
    ),
    entry!(
        "glue",
        "com.amazonaws.glue#AWSGlue",
        "glue.json",
        "aws-sdk-glue",
        "aws_sdk_glue"
    ),
    entry_with_sdk_version!(
        "iam",
        "com.amazonaws.iam#AWSIdentityManagementV20100508",
        "iam.json",
        "aws-sdk-iam",
        "aws_sdk_iam",
        "1.121.0"
    ),
    entry!(
        "kinesis",
        "com.amazonaws.kinesis#Kinesis_20131202",
        "kinesis.json",
        "aws-sdk-kinesis",
        "aws_sdk_kinesis"
    ),
    entry_with_sdk_version!(
        "kms",
        "com.amazonaws.kms#TrentService",
        "kms.json",
        "aws-sdk-kms",
        "aws_sdk_kms",
        "1.116.0"
    ),
    entry_with_sdk_version!(
        "lambda",
        "com.amazonaws.lambda#AWSGirApiService",
        "lambda.json",
        "aws-sdk-lambda",
        "aws_sdk_lambda",
        "1.140.0"
    ),
    entry!(
        "opensearch",
        "com.amazonaws.opensearch#AmazonOpenSearchService",
        "opensearch.json",
        "aws-sdk-opensearch",
        "aws_sdk_opensearch"
    ),
    entry!(
        "rds",
        "com.amazonaws.rds#AmazonRDSv19",
        "rds.json",
        "aws-sdk-rds",
        "aws_sdk_rds"
    ),
    entry!(
        "redshift",
        "com.amazonaws.redshift#RedshiftServiceVersion20121201",
        "redshift.json",
        "aws-sdk-redshift",
        "aws_sdk_redshift"
    ),
    entry!(
        "rekognition",
        "com.amazonaws.rekognition#RekognitionService",
        "rekognition.json",
        "aws-sdk-rekognition",
        "aws_sdk_rekognition"
    ),
    entry!(
        "route-53",
        "com.amazonaws.route53#AWSDnsV20130401",
        "route-53.json",
        "aws-sdk-route53",
        "aws_sdk_route53"
    ),
    entry_with_protocol_tests_and_sdk_version!(
        "s3",
        "com.amazonaws.s3#AmazonS3",
        "s3.json",
        "aws-sdk-s3",
        "aws_sdk_s3",
        "protocol-tests/s3.json",
        "1.143.0"
    ),
    entry!(
        "secrets-manager",
        "com.amazonaws.secretsmanager#secretsmanager",
        "secrets-manager.json",
        "aws-sdk-secretsmanager",
        "aws_sdk_secretsmanager"
    ),
    entry!(
        "sesv2",
        "com.amazonaws.sesv2#SimpleEmailService_v2",
        "sesv2.json",
        "aws-sdk-sesv2",
        "aws_sdk_sesv2"
    ),
    entry!(
        "sfn",
        "com.amazonaws.sfn#AWSStepFunctions",
        "sfn.json",
        "aws-sdk-sfn",
        "aws_sdk_sfn"
    ),
    entry_with_sdk_version!(
        "sns",
        "com.amazonaws.sns#AmazonSimpleNotificationService",
        "sns.json",
        "aws-sdk-sns",
        "aws_sdk_sns",
        "1.109.0"
    ),
    entry_with_sdk_version!(
        "sqs",
        "com.amazonaws.sqs#AmazonSQS",
        "sqs.json",
        "aws-sdk-sqs",
        "aws_sdk_sqs",
        "1.107.0"
    ),
    entry!(
        "ssm",
        "com.amazonaws.ssm#AmazonSSM",
        "ssm.json",
        "aws-sdk-ssm",
        "aws_sdk_ssm"
    ),
    entry_with_sdk_version!(
        "sts",
        "com.amazonaws.sts#AWSSecurityTokenServiceV20110615",
        "sts.json",
        "aws-sdk-sts",
        "aws_sdk_sts",
        "1.112.0"
    ),
    entry!(
        "textract",
        "com.amazonaws.textract#Textract",
        "textract.json",
        "aws-sdk-textract",
        "aws_sdk_textract"
    ),
    entry!(
        "wafv2",
        "com.amazonaws.wafv2#AWSWAF_20190729",
        "wafv2.json",
        "aws-sdk-wafv2",
        "aws_sdk_wafv2"
    ),
];

pub fn entries() -> &'static [ModelEntry] {
    ENTRIES
}

pub fn lookup(key: &str) -> Result<ModelEntry, BuildError> {
    ENTRIES
        .iter()
        .copied()
        .find(|entry| entry.key == key)
        .ok_or_else(|| BuildError::UnknownService {
            service: key.to_owned(),
            registry: REGISTRY_SOURCE.to_owned(),
        })
}

pub fn manifest_json() -> &'static str {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/models-manifest.json"))
}

pub fn checksum(key: &str) -> Option<String> {
    let manifest = serde_json::from_str::<serde_json::Value>(manifest_json()).ok()?;
    manifest["services"]
        .as_array()?
        .iter()
        .find(|service| service["service_key"].as_str() == Some(key))
        .and_then(|service| service["model_sha256"].as_str())
        .map(ToOwned::to_owned)
}
