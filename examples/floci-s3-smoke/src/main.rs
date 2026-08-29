use std::{
    env,
    error::Error,
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::{primitives::ByteStream, Client};

const PAYLOAD: &[u8] = b"better-aws floci smoke payload";
const KEY: &str = "smoke/payload.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let endpoint =
        env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://127.0.0.1:4566".to_owned());
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| "us-east-1".to_owned());
    let access_key = env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "test".to_owned());
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "test".to_owned());
    let bucket = format!(
        "better-aws-floci-{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos(),
        process::id()
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region))
        .endpoint_url(endpoint.clone())
        .credentials_provider(Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "floci-s3-smoke",
        ))
        .load()
        .await;
    let client = Client::new(&config);

    let result = exercise(&client, &bucket).await;
    let cleanup_result = cleanup(&client, &bucket).await;
    if let Err(error) = result {
        eprintln!("S3 Floci smoke failed at {endpoint} for bucket {bucket}: {error}");
        if let Err(cleanup_error) = cleanup_result {
            eprintln!("S3 Floci cleanup also failed for bucket {bucket}: {cleanup_error}");
        }
        return Err(error);
    }
    cleanup_result?;
    println!("S3 Floci Rust smoke test passed at {endpoint}");
    Ok(())
}

async fn exercise(client: &Client, bucket: &str) -> Result<(), Box<dyn Error>> {
    client.create_bucket().bucket(bucket).send().await?;
    client
        .put_object()
        .bucket(bucket)
        .key(KEY)
        .body(ByteStream::from_static(PAYLOAD))
        .send()
        .await?;

    let head = client.head_object().bucket(bucket).key(KEY).send().await?;
    if head.content_length() != Some(PAYLOAD.len() as i64) {
        return Err(format!(
            "HeadObject content length mismatch: expected {}, got {:?}",
            PAYLOAD.len(),
            head.content_length()
        )
        .into());
    }

    let object = client.get_object().bucket(bucket).key(KEY).send().await?;
    let bytes = object.body.collect().await?.into_bytes();
    if bytes.as_ref() != PAYLOAD {
        return Err("GetObject payload mismatch".into());
    }

    let listed = client.list_objects_v2().bucket(bucket).send().await?;
    if !listed
        .contents()
        .iter()
        .any(|object| object.key() == Some(KEY))
    {
        return Err("ListObjectsV2 did not return the uploaded key".into());
    }

    client
        .delete_object()
        .bucket(bucket)
        .key(KEY)
        .send()
        .await?;
    Ok(())
}

async fn cleanup(client: &Client, bucket: &str) -> Result<(), Box<dyn Error>> {
    let _ = client.delete_object().bucket(bucket).key(KEY).send().await;
    client.delete_bucket().bucket(bucket).send().await?;
    Ok(())
}
