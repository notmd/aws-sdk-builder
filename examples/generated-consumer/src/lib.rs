aws_sdk_build::include_sdk!();

pub fn selected_operation_type()
-> aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload {
    aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload::new()
}

pub fn selected_operation_builder() -> aws_sdk_s3::operation::abort_multipart_upload::Builder {
    aws_sdk_s3::Client::new(&aws_sdk_s3::Config).abort_multipart_upload()
}

pub fn selected_model_builder() -> aws_sdk_s3::types::AbortMultipartUploadRequest {
    aws_sdk_s3::types::AbortMultipartUploadRequest::builder()
        .bucket("example-bucket")
        .key("example-key")
        .upload_id("example-upload")
        .build()
}
