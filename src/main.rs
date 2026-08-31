use std::env;

fn main() {
    if let Err(error) = aws_sdk_modularizer::conformance::run_cli(env::args_os().skip(1).collect()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
