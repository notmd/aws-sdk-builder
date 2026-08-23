#![allow(warnings)]

aws_sdk_builder::include_sdk!();

fn main() {
    let _s3: Option<aws_sdk_s3::Client> = None;
    let _sqs: Option<aws_sdk_sqs::Client> = None;
}
