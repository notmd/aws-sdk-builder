use std::{
    env,
    ffi::OsString,
    fs,
    panic::AssertUnwindSafe,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    sync::Mutex,
};

use rayon::prelude::*;

const WORK_BATCH_SIZE: usize = 512;

mod canonical;
mod manifest;
mod normalize;
mod updater;

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

    if arguments
        .first()
        .is_some_and(|argument| argument == "refresh-patches")
    {
        return match refresh_patches(&arguments[1..]) {
            Ok(count) => {
                eprintln!("refreshed {count} reference normalization patch(es)");
                Ok(false)
            }
            Err(error) => Err(error),
        };
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
    let conformance = generate_conformance_snapshot(&generated, conformance_sources(&services)?)?;
    eprintln!(
        "generated {} all-operation service snapshot(s), {} operation(s)",
        services.len(),
        conformance.operation_count
    );
    let (removed, formatted_files) =
        normalize::strip_excluded_and_collect_rust_files(&generated, &manifest.comparison.exclude)?;
    eprintln!("removed {removed} excluded generated snapshot path(s)");
    let formatted_file_count = format_generated_sources(&generated, formatted_files)?;
    eprintln!(
        "formatted {} generated Rust snapshot file(s) with rustfmt",
        formatted_file_count
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
            "batch" => Ok(aws_sdk_builder_batch::source()),
            "bedrockruntime" => Ok(aws_sdk_builder_bedrockruntime::source()),
            "cloudwatchlogs" => Ok(aws_sdk_builder_cloudwatchlogs::source()),
            "codeartifact" => Ok(aws_sdk_builder_codeartifact::source()),
            "cognitoidentityprovider" => Ok(aws_sdk_builder_cognitoidentityprovider::source()),
            "config" => Ok(aws_sdk_builder_config::source()),
            "dynamodb" => Ok(aws_sdk_builder_dynamodb::source()),
            "iam" => Ok(aws_sdk_builder_iam::source()),
            "kms" => Ok(aws_sdk_builder_kms::source()),
            "lambda" => Ok(aws_sdk_builder_lambda::source()),
            "s3" => Ok(aws_sdk_builder_s3::source()),
            "sesv2" => Ok(aws_sdk_builder_sesv2::source()),
            "sns" => Ok(aws_sdk_builder_sns::source()),
            "sqs" => Ok(aws_sdk_builder_sqs::source()),
            "sts" => Ok(aws_sdk_builder_sts::source()),
            other => Err(format!(
                "manifest service `{other}` has no registered builder crate"
            )),
        })
        .collect()
}

fn refresh_patches(arguments: &[OsString]) -> Result<usize, Box<dyn std::error::Error>> {
    let manifest_path = optional_path(arguments, "--manifest")
        .unwrap_or_else(|| PathBuf::from(manifest::DEFAULT_PATH));
    let manifest = manifest::ServicesManifest::load(&manifest_path)?;
    let reference = optional_path(arguments, "--reference")
        .unwrap_or_else(|| manifest.root_path(&manifest_path, &manifest.roots.reference));
    let patches = optional_path(arguments, "--patches")
        .unwrap_or_else(|| manifest.root_path(&manifest_path, &manifest.roots.patches));
    Ok(normalize::write_reference_patches(
        &reference,
        &patches,
        &manifest.comparison.exclude,
    )?)
}

fn generate_conformance_snapshot(
    output_dir: &Path,
    services: Vec<aws_sdk_builder::ServiceSource>,
) -> Result<aws_sdk_builder::ConformanceSnapshot, String> {
    let parent = output_dir.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    let stage = tempfile::Builder::new()
        .prefix("aws-sdk-conformance-")
        .tempdir_in(parent)
        .map_err(|error| format!("{}: {error}", parent.display()))?;
    let stage_root = stage.path().to_owned();
    let counts = services
        .into_par_iter()
        .map(|service| {
            let service_key = service.metadata.key.to_owned();
            let service_output =
                stage_root.join(format!(".aws-sdk-conformance-service-{service_key}"));
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                aws_sdk_builder::generate_all(&service_output, [service])
                    .map_err(|error| error.to_string())
                    .map(|snapshot| {
                        let files = snapshot
                            .service_files
                            .get(&service_key)
                            .cloned()
                            .unwrap_or_default();
                        (snapshot, files)
                    })
                    .and_then(|(snapshot, files)| {
                        let generated_service = service_output.join(&service_key);
                        project_original(&generated_service, &files)
                            .map_err(|error| format!("{service_key}: {error}"))?;
                        Ok((snapshot.operation_count, files, service_output, service_key))
                    })
            }));
            match result {
                Ok(result) => result,
                Err(_) => Err("conformance generation worker panicked".to_owned()),
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<_>, String>>()?;
    let mut operation_count = 0;
    let mut service_files = std::collections::BTreeMap::new();
    for (count, files, service_output, service_key) in counts {
        operation_count += count;
        service_files.insert(service_key.clone(), files);
        let generated_service = service_output.join(&service_key);
        let destination = stage.path().join(&service_key);
        fs::rename(&generated_service, &destination)
            .map_err(|error| format!("{}: {error}", destination.display()))?;
        fs::remove_dir_all(&service_output)
            .map_err(|error| format!("{}: {error}", service_output.display()))?;
    }

    let backup = parent.join(format!(
        ".aws-sdk-conformance-backup-{}",
        std::process::id()
    ));
    if backup.exists() {
        return Err(format!(
            "conformance snapshot backup already exists: {}",
            backup.display()
        ));
    }
    let had_existing = output_dir.exists();
    if had_existing {
        fs::rename(output_dir, &backup)
            .map_err(|error| format!("{}: {error}", output_dir.display()))?;
    }
    if let Err(error) = fs::rename(stage.path(), output_dir) {
        if had_existing {
            let _ = fs::rename(&backup, output_dir);
        }
        return Err(format!("{}: {error}", output_dir.display()));
    }
    if had_existing {
        fs::remove_dir_all(&backup).map_err(|error| format!("{}: {error}", backup.display()))?;
    }

    Ok(aws_sdk_builder::ConformanceSnapshot {
        operation_count,
        service_files,
    })
}

fn project_original(
    generated_service: &Path,
    files: &std::collections::BTreeSet<String>,
) -> Result<(), String> {
    let original_path = generated_service.join(canonical::ORIGINAL_FILE);
    let original = fs::read_to_string(&original_path)
        .map_err(|error| format!("{}: {error}", original_path.display()))?;
    let split = canonical::split(&original, files)?;
    for (relative, source) in split {
        let source = normalize::prepare_canonical_projection(&source, Path::new(&relative))?;
        let destination = generated_service.join("normalized").join(&relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        fs::write(&destination, source)
            .map_err(|error| format!("{}: {error}", destination.display()))?;
    }
    Ok(())
}

fn format_generated_sources(root: &Path, files: Vec<PathBuf>) -> Result<usize, String> {
    let file_count = files.len();
    if files.is_empty() {
        return Ok(0);
    }

    // The canonical artifact is preserved byte-for-byte; only its normalized
    // projection is formatted and restored in the same batch.
    let batches = files
        .chunks(WORK_BATCH_SIZE)
        .map(<[PathBuf]>::to_vec)
        .collect();
    parallel_for_each(
        batches,
        |batch| run_rustfmt_and_restore(root, batch),
        "rustfmt worker thread panicked",
    )?;

    Ok(file_count)
}

fn run_rustfmt_and_restore(root: &Path, batch: Vec<PathBuf>) -> Result<(), String> {
    let output = Command::new("rustfmt")
        .args([
            "--edition",
            "2021",
            "--config",
            "max_width=150,skip_children=true",
        ])
        .args(&batch)
        .output()
        .map_err(|error| format!("failed to run rustfmt: {error}"))?;
    if !output.status.success() {
        let mut message = String::from_utf8_lossy(&output.stderr).into_owned();
        if message.trim().is_empty() {
            message = format!("rustfmt exited with {}", output.status);
        }
        return Err(format!("rustfmt failed for generated sources: {message}"));
    }
    batch.into_iter().try_for_each(|path| {
        let service = path
            .strip_prefix(root)
            .map_err(|error| error.to_string())?
            .components()
            .next()
            .and_then(|component| component.as_os_str().to_str())
            .ok_or_else(|| {
                format!(
                    "generated projection has no service component: {}",
                    path.display()
                )
            })?;
        let projection_root = root.join(service).join("normalized");
        let relative = path.strip_prefix(&projection_root).map_err(|error| {
            format!(
                "{} is outside normalized projection: {error}",
                path.display()
            )
        })?;
        if !relative.as_os_str().is_empty() {
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            if relative != Path::new("src/lib.rs") && source.contains("crate::") {
                let restored = normalize::restore_canonical_paths(&source, relative)?;
                if restored != source {
                    fs::write(&path, restored)
                        .map_err(|error| format!("{}: {error}", path.display()))?;
                }
            }
        }
        Ok(())
    })
}

fn parallel_for_each<T, F>(jobs: Vec<T>, operation: F, panic_message: &str) -> Result<(), String>
where
    T: Send,
    F: Fn(T) -> Result<(), String> + Sync,
{
    if jobs.is_empty() {
        return Ok(());
    }

    let worker_count = rayon::current_num_threads().min(jobs.len());
    let mut buckets = (0..worker_count)
        .map(|_| Vec::new())
        .collect::<Vec<Vec<T>>>();
    for (index, job) in jobs.into_iter().enumerate() {
        buckets[index % worker_count].push(job);
    }

    let results = Mutex::new(
        (0..worker_count)
            .map(|_| None)
            .collect::<Vec<Option<Result<(), String>>>>(),
    );
    rayon::scope(|scope| {
        let operation = &operation;
        for (index, bucket) in buckets.into_iter().enumerate() {
            let results = &results;
            scope.spawn(move |_| {
                let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    bucket.into_iter().try_for_each(operation)
                }))
                .unwrap_or_else(|_| Err(panic_message.to_owned()));
                if let Ok(mut results) = results.lock() {
                    results[index] = Some(result);
                }
            });
        }
    });

    let results = results.into_inner().map_err(|_| panic_message.to_owned())?;
    for result in results {
        result.ok_or_else(|| "conformance worker did not report a result".to_owned())??;
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

#[cfg(test)]
mod tests {
    use super::parallel_for_each;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn parallel_for_each_processes_every_job() {
        let processed = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&processed);
        parallel_for_each(
            (0..1024).collect(),
            move |_| {
                counter.fetch_add(1, Ordering::Relaxed);
                Ok(())
            },
            "worker thread panicked",
        )
        .unwrap();
        assert_eq!(processed.load(Ordering::Relaxed), 1024);
    }

    #[test]
    fn parallel_for_each_propagates_worker_errors() {
        let result = parallel_for_each(
            vec![0, 1, 2],
            |job| {
                if job == 1 {
                    Err("expected worker error".to_owned())
                } else {
                    Ok(())
                }
            },
            "worker thread panicked",
        );
        assert_eq!(result, Err("expected worker error".to_owned()));
    }
}
