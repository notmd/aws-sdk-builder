#![allow(warnings)]

mod aws_s3_sdk {
    aws_sdk_builder_s3::include_sdk!();
}

mod aws_sqs_sdk {
    aws_sdk_builder_sqs::include_sdk!();
}

fn main() {
    let _s3: Option<aws_s3_sdk::Client> = None;
    let _sqs: Option<aws_sqs_sdk::Client> = None;
}
