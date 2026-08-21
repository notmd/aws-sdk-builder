aws_sdk_build::include_sdk!();

fn main() {}

#[cfg(test)]
mod tests {
    use super::aws_sdk_s3;

    #[tokio::test]
    async fn creates_then_heads_a_bucket() {
        let bucket = "my-aws-sdk-test-bucket";
        let client = aws_sdk_s3::Client::new(&aws_sdk_s3::Config::default());

        match client.create_bucket().bucket(bucket).send().await {
            Ok(_) => {}
            Err(error)
                if error.is_bucket_already_exists() || error.is_bucket_already_owned_by_you() => {}
            Err(error) => panic!("CreateBucket failed for {bucket}: {error}"),
        }

        client
            .head_bucket()
            .bucket(bucket)
            .send()
            .await
            .unwrap_or_else(|error| panic!("HeadBucket failed for {bucket}: {error}"));
    }
}
