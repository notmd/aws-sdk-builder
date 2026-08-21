fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_build::configure()
        .add("s3", ["AbortMultipartUpload", "CompleteMultipartUpload"])
        .add("dynamodb", ["GetItem"])
        .compile()?;
    Ok(())
}
