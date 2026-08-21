fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_build::configure()
        .add("s3", [])
        .add("dynamodb", [])
        .add("lambda", [])
        .add("sqs", [])
        .add("sns", [])
        .add("sts", [])
        .add("iam", [])
        .add("kms", [])
        .compile()?;
    Ok(())
}
