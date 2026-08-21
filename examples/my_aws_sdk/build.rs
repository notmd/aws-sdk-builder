fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_build::configure()
        .add("s3", ["CreateBucket", "HeadBucket"])
        .compile()?;
    Ok(())
}
