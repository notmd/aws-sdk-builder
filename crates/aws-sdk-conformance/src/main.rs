use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

#[allow(dead_code)]
mod report;

use report::{compare_directories, write_reports};

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
    let services = service_names(&reference)?;
    let operation_count = aws_sdk_build::generate_all(&generated, &services)?;
    eprintln!(
        "generated {} all-operation service snapshot(s), {} operation(s)",
        services.len(),
        operation_count
    );
    let formatted_files = format_generated_sources(&generated)?;
    eprintln!(
        "formatted {} generated Rust snapshot file(s) with rustfmt",
        formatted_files
    );
    let report = compare_directories(reference, generated, snapshot)?;
    write_reports(output, &report)?;
    eprintln!(
        "compared {} service(s): {}/{} files matched",
        report.services.len(),
        report.matched_files(),
        report.total_files()
    );
    Ok(report.has_differences())
}

fn format_generated_sources(root: &Path) -> Result<usize, String> {
    let mut files = Vec::new();
    collect_rust_files(root, &mut files)?;
    files.sort();

    if files.is_empty() {
        return Ok(0);
    }

    let output = Command::new("rustfmt")
        .args([
            "--edition",
            "2021",
            "--config",
            "max_width=150,skip_children=true",
        ])
        .args(&files)
        .output()
        .map_err(|error| format!("failed to run rustfmt: {error}"))?;
    if !output.status.success() {
        let mut message = String::from_utf8_lossy(&output.stderr).into_owned();
        if message.trim().is_empty() {
            message = format!("rustfmt exited with {}", output.status);
        }
        return Err(format!("rustfmt failed for generated sources: {message}"));
    }

    Ok(files.len())
}

fn collect_rust_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|error| format!("{}: {error}", root.display()))? {
        let entry = entry.map_err(|error| format!("{}: {error}", root.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_rust_files(&path, files)?;
        } else if file_type.is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("rs")
        {
            files.push(path);
        }
    }
    Ok(())
}

fn service_names(reference: &Path) -> Result<Vec<String>, String> {
    if !reference.is_dir() {
        return Err(format!(
            "reference root is not a directory: {}",
            reference.display()
        ));
    }
    let mut services = BTreeSet::new();
    for entry in fs::read_dir(reference).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        if entry
            .file_type()
            .map_err(|error| error.to_string())?
            .is_dir()
        {
            services.insert(entry.file_name().to_string_lossy().into_owned());
        }
    }
    if services.is_empty() {
        return Err(format!(
            "reference root has no service directories: {}",
            reference.display()
        ));
    }
    Ok(services.into_iter().collect())
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
    "Usage: aws-sdk-conformance --reference DIR --generated DIR --output FILE [--snapshot SHA]\n\nGenerates all packaged operations for each reference service into DIR, then compares the source directories and writes a summary report plus one deterministic diffy Markdown report per service.\nExit status: 0 means equal, 1 means differences were reported, 2 means the runner failed."
}
