use std::{
    env,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

mod manifest;
mod normalize;
mod updater;

#[allow(dead_code)]
mod report;

use report::write_reports;

fn main() -> ExitCode {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    if arguments
        .first()
        .is_some_and(|argument| argument == "update-reference")
    {
        return match updater::run(&arguments[1..]) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("aws-sdk-conformance: {error}");
                ExitCode::from(2)
            }
        };
    }

    let arguments = if arguments
        .first()
        .is_some_and(|argument| argument == "conformance")
    {
        &arguments[1..]
    } else {
        &arguments[..]
    };
    match run_conformance(arguments) {
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

fn run_conformance(arguments: &[OsString]) -> Result<bool, Box<dyn std::error::Error>> {
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        print_usage();
        return Ok(false);
    }

    let manifest_path = optional_path(arguments, "--manifest")
        .unwrap_or_else(|| PathBuf::from(manifest::DEFAULT_PATH));
    let manifest = manifest::ServicesManifest::load(&manifest_path)?;
    manifest::validate_registered_services(&manifest)?;
    let reference = optional_path(arguments, "--reference")
        .unwrap_or_else(|| manifest.root_path(&manifest_path, &manifest.roots.reference));
    let generated = optional_path(arguments, "--generated")
        .unwrap_or_else(|| manifest.root_path(&manifest_path, &manifest.roots.generated));
    let patches = optional_path(arguments, "--patches")
        .unwrap_or_else(|| manifest.root_path(&manifest_path, &manifest.roots.patches));
    let output = optional_path(arguments, "--output")
        .unwrap_or_else(|| manifest.root_path(&manifest_path, &manifest.roots.summary));
    let snapshot = optional_string(arguments, "--snapshot")?;
    if let Some(snapshot) = &snapshot
        && snapshot != &manifest.upstream.commit
    {
        return Err(format!(
            "--snapshot {snapshot} does not match manifest commit {}",
            manifest.upstream.commit
        )
        .into());
    }
    let snapshot = Some(manifest.upstream.commit.clone());
    let services = manifest
        .services
        .iter()
        .map(|service| service.key.clone())
        .collect::<Vec<_>>();
    let operation_count =
        aws_sdk_builder::generate_all(&generated, conformance_sources(&services)?)?;
    eprintln!(
        "generated {} all-operation service snapshot(s), {} operation(s)",
        services.len(),
        operation_count
    );
    let removed = normalize::strip_excluded(&generated, &manifest.comparison.exclude)?;
    eprintln!("removed {removed} excluded generated snapshot path(s)");
    let formatted_files = format_generated_sources(&generated)?;
    eprintln!(
        "formatted {} generated Rust snapshot file(s) with rustfmt",
        formatted_files
    );
    let report = report::compare_directories_with_policy_and_patches(
        &reference,
        &generated,
        Some(&patches),
        snapshot,
        &manifest.comparison.exclude,
    )?;
    write_reports(&output, &report)?;
    eprintln!(
        "compared {} service(s): {}/{} files matched",
        report.services.len(),
        report.matched_files(),
        report.total_files()
    );
    Ok(report.has_differences())
}

fn conformance_sources(services: &[String]) -> Result<Vec<aws_sdk_builder::ServiceSource>, String> {
    services
        .iter()
        .map(|service| match service.as_str() {
            "dynamodb" => Ok(aws_sdk_builder_dynamodb::source()),
            "iam" => Ok(aws_sdk_builder_iam::source()),
            "kms" => Ok(aws_sdk_builder_kms::source()),
            "lambda" => Ok(aws_sdk_builder_lambda::source()),
            "s3" => Ok(aws_sdk_builder_s3::source()),
            "sns" => Ok(aws_sdk_builder_sns::source()),
            "sqs" => Ok(aws_sdk_builder_sqs::source()),
            "sts" => Ok(aws_sdk_builder_sts::source()),
            other => Err(format!(
                "manifest service `{other}` has no registered builder crate"
            )),
        })
        .collect()
}

fn format_generated_sources(root: &Path) -> Result<usize, String> {
    let mut files = Vec::new();
    collect_rust_files(root, &mut files)?;
    files.sort();

    if files.is_empty() {
        return Ok(0);
    }

    let rustfmt_files = files
        .iter()
        .filter(|path| is_generated_rust_file(path))
        .cloned()
        .collect::<Vec<_>>();
    let output = Command::new("rustfmt")
        .args([
            "--edition",
            "2021",
            "--config",
            "max_width=150,skip_children=true",
        ])
        .args(&rustfmt_files)
        .output()
        .map_err(|error| format!("failed to run rustfmt: {error}"))?;
    if !output.status.success() {
        let mut message = String::from_utf8_lossy(&output.stderr).into_owned();
        if message.trim().is_empty() {
            message = format!("rustfmt exited with {}", output.status);
        }
        return Err(format!("rustfmt failed for generated sources: {message}"));
    }

    Ok(rustfmt_files.len())
}

fn is_generated_rust_file(path: &Path) -> bool {
    const GENERATED_HEADER: &[u8] =
        b"// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.";
    fs::read(path)
        .map(|bytes| {
            bytes
                .windows(GENERATED_HEADER.len())
                .any(|window| window == GENERATED_HEADER)
        })
        .unwrap_or(false)
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

fn optional_path(arguments: &[OsString], flag: &str) -> Option<PathBuf> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| PathBuf::from(&pair[1]))
}

fn optional_string(arguments: &[OsString], flag: &str) -> Result<Option<String>, String> {
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
    "Usage: aws-sdk-conformance [conformance] [--manifest FILE] [--reference DIR] [--generated DIR] [--patches DIR] [--output FILE] [--snapshot SHA]\n       aws-sdk-conformance update-reference [--manifest FILE] [--dry-run]\n\nThe conformance command generates all packaged operations, removes configured non-source files, applies checked-in reference normalization patches in memory, compares selected services, and writes deterministic reports. The update-reference command downloads the pinned upstream GitHub archive, refreshes reference trees, normalization patches, and service model.json files atomically, and exits without changing files on --dry-run. Exit status: 0 means equal or update succeeded, 1 means conformance differences, 2 means the runner failed."
}
