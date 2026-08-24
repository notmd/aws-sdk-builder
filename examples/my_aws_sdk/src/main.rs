#![allow(warnings)]

mod aws_s3_sdk {
    aws_sdk_builder_s3::include_sdk!();
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use super::aws_s3_sdk;

    #[tokio::test]
    async fn core_s3_operations_use_the_rest_runtime() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for request_number in 0..7 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = [0_u8; 8192];
                let bytes_read = stream.read(&mut request).unwrap();
                let request = String::from_utf8_lossy(&request[..bytes_read]);
                let request_line = request.lines().next().unwrap_or_default();
                let response = if request_line.starts_with("PUT /bucket/key") {
                    "HTTP/1.1 200 OK\r\nETag: \"test-etag\"\r\nContent-Length: 0\r\n\r\n".to_owned()
                } else if request_line.starts_with("PUT /bucket") {
                    "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".to_owned()
                } else if request_line.starts_with("HEAD ") {
                    "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\n".to_owned()
                } else if request_line.contains("list-type=2") {
                    let body =
                        "<ListBucketResult><Contents><Key>key</Key></Contents></ListBucketResult>";
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{body}",
                        body.len()
                    )
                } else if request_line.starts_with("GET ") {
                    "HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\npayload".to_owned()
                } else {
                    assert!(request_line.starts_with("DELETE "));
                    "HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n".to_owned()
                };
                assert!(request_number < 7);
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        let endpoint = format!("http://{address}");
        let config = aws_s3_sdk::Config::builder()
            .behavior_version_latest()
            .sleep_impl(aws_smithy_async::rt::sleep::TokioSleep::new())
            .credentials_provider(aws_credential_types::Credentials::for_tests())
            .region(aws_s3_sdk::config::Region::new("us-east-1"))
            .http_client(aws_smithy_http_client::Builder::new().build_http())
            .endpoint_url(endpoint)
            .build();
        let client = aws_s3_sdk::Client::from_conf(config);
        let bucket = "bucket";

        client.create_bucket().bucket(bucket).send().await.unwrap();
        client
            .put_object()
            .bucket(bucket)
            .key("key")
            .body(aws_s3_sdk::primitives::ByteStream::from_static(b"payload"))
            .send()
            .await
            .unwrap();
        let head = client
            .head_object()
            .bucket(bucket)
            .key("key")
            .send()
            .await
            .unwrap();
        assert_eq!(head.content_length(), Some(5));
        let object = client
            .get_object()
            .bucket(bucket)
            .key("key")
            .send()
            .await
            .unwrap();
        assert_eq!(
            object.body.collect().await.unwrap().into_bytes().as_ref(),
            b"payload"
        );
        let listed = client
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await
            .unwrap();
        assert!(
            listed
                .contents()
                .iter()
                .any(|object| object.key() == Some("key"))
        );
        client
            .delete_object()
            .bucket(bucket)
            .key("key")
            .send()
            .await
            .unwrap();
        client.delete_bucket().bucket(bucket).send().await.unwrap();
        server.join().unwrap();
    }
}
