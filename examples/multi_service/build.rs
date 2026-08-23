fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_builder_s3::compile(["GetObject"])?;
    aws_sdk_builder_s3::compile(["HeadObject"])?;
    aws_sdk_builder_sqs::compile(["SendMessage"])?;
    Ok(())
}
