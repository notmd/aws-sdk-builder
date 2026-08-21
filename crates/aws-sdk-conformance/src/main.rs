use std::{env, path::PathBuf, process::ExitCode};

use aws_sdk_conformance::{compare_directories, write_markdown};

fn main() -> ExitCode {
    match run() {
        Ok(has_differences) => {
            if has_differences {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(error) => {
            eprintln!("aws-sdk-conformance: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        print_usage();
        return Ok(false);
    }

    let reference = required_path(&arguments, "--reference")?;
    let generated = required_path(&arguments, "--generated")?;
    let output = required_path(&arguments, "--output")?;
    let snapshot = optional_string(&arguments, "--snapshot")?;
    let report = compare_directories(reference, generated, snapshot)?;
    write_markdown(output, &report)?;
    eprintln!(
        "compared {} service(s): {}/{} files matched",
        report.services.len(),
        report.matched_files(),
        report.total_files()
    );
    Ok(report.has_differences())
}

fn required_path(arguments: &[std::ffi::OsString], flag: &str) -> Result<PathBuf, String> {
    let value = arguments
        .windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
        .ok_or_else(|| format!("missing {flag}\n\n{}", usage()))?;
    Ok(PathBuf::from(value))
}

fn optional_string(arguments: &[std::ffi::OsString], flag: &str) -> Result<Option<String>, String> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| {
            pair[1]
                .to_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{flag} must be valid UTF-8"))
        })
        .transpose()
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: aws-sdk-conformance --reference DIR --generated DIR --output FILE [--snapshot SHA]\n\nCompares immediate service directories and writes a deterministic diffy Markdown report.\nExit status: 0 means equal, 1 means differences were reported, 2 means the runner failed."
}
