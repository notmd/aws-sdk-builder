fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_builder_s3::compile([
        "CreateBucket",
        "PutObject",
        "HeadObject",
        "GetObject",
        "ListObjectsV2",
        "DeleteObject",
        "DeleteBucket",
    ])?;
    Ok(())
}
