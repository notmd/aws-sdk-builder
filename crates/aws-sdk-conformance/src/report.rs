use crate::{manifest::Exclusions, normalize};
use quote::ToTokens;
use std::{
    collections::BTreeSet,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

/// The result of comparing one reference/generated service pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceReport {
    pub name: String,
    pub total_files: usize,
    pub compared_files: usize,
    pub matched_files: usize,
    pub mismatched_files: usize,
    pub missing_files: Vec<String>,
    pub extra_files: Vec<String>,
    pub binary_mismatches: Vec<String>,
    pub read_errors: Vec<String>,
    pub differences: Vec<FileDifference>,
}

impl ServiceReport {
    pub fn has_differences(&self) -> bool {
        self.mismatched_files != 0
            || !self.missing_files.is_empty()
            || !self.extra_files.is_empty()
            || !self.binary_mismatches.is_empty()
            || !self.read_errors.is_empty()
    }

    /// Return the percentage of compared files that matched exactly.
    ///
    /// An empty service has no differences, so it is considered fully matched.
    pub fn match_percentage(&self) -> f64 {
        if self.total_files == 0 {
            100.0
        } else {
            (self.matched_files as f64 / self.total_files as f64) * 100.0
        }
    }

    /// Render the complete deterministic Markdown report for this service.
    pub fn to_markdown(&self, snapshot: Option<&str>) -> String {
        let mut markdown = String::from("# AWS SDK Conformance Report: ");
        markdown.push_str(&self.name);
        markdown.push_str("\n\n");
        if let Some(snapshot) = snapshot {
            markdown.push_str("Snapshot: `");
            markdown.push_str(&escape_inline(snapshot));
            markdown.push_str("`\n\n");
        }
        append_service_details(&mut markdown, self);
        markdown
    }
}

/// A changed text file and its unified `diffy` patch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDifference {
    pub path: String,
    pub patch: String,
}

/// The complete deterministic report for all selected services.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceReport {
    pub snapshot: Option<String>,
    pub services: Vec<ServiceReport>,
}

impl ConformanceReport {
    pub fn has_differences(&self) -> bool {
        self.services.iter().any(ServiceReport::has_differences)
    }

    pub fn total_files(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.total_files)
            .sum()
    }

    pub fn compared_files(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.compared_files)
            .sum()
    }

    pub fn matched_files(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.matched_files)
            .sum()
    }

    pub fn mismatched_files(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.mismatched_files)
            .sum()
    }

    pub fn missing_files(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.missing_files.len())
            .sum()
    }

    pub fn extra_files(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.extra_files.len())
            .sum()
    }

    pub fn read_errors(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.read_errors.len())
            .sum()
    }

    /// Return the arithmetic mean of the per-service match percentages.
    pub fn average_match_percentage(&self) -> f64 {
        if self.services.is_empty() {
            0.0
        } else {
            self.services
                .iter()
                .map(ServiceReport::match_percentage)
                .sum::<f64>()
                / self.services.len() as f64
        }
    }

    /// Render a summary table with links to the complete per-service reports.
    pub fn to_summary_markdown(&self, service_directory: impl AsRef<Path>) -> String {
        let service_directory = service_directory.as_ref();
        let mut markdown = String::from("# AWS SDK Conformance Report\n\n");
        if let Some(snapshot) = &self.snapshot {
            markdown.push_str("Snapshot: `");
            markdown.push_str(&escape_inline(snapshot));
            markdown.push_str("`\n\n");
        }
        markdown.push_str("**Summary:** ");
        markdown.push_str(&format_summary(self));
        markdown.push_str("\n\n");
        markdown.push_str(
            "| Service | Compared | Matched | Mismatches | Missing | Extra | Read errors | Match | Report |\n",
        );
        markdown.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n");
        for service in &self.services {
            markdown.push_str("| ");
            markdown.push_str(&escape_table(&service.name));
            markdown.push_str(" | ");
            markdown.push_str(&format!(
                "{}/{}",
                service.compared_files, service.total_files
            ));
            markdown.push_str(" | ");
            markdown.push_str(&service.matched_files.to_string());
            markdown.push_str(" | ");
            markdown.push_str(&service.mismatched_files.to_string());
            markdown.push_str(" | ");
            markdown.push_str(&service.missing_files.len().to_string());
            markdown.push_str(" | ");
            markdown.push_str(&service.extra_files.len().to_string());
            markdown.push_str(" | ");
            markdown.push_str(&service.read_errors.len().to_string());
            markdown.push_str(" | ");
            markdown.push_str(&format!("{:.2}%", service.match_percentage()));
            markdown.push_str(" | [report](");
            markdown.push_str(
                &service_directory
                    .join(format!("{}.md", service.name))
                    .display()
                    .to_string(),
            );
            markdown.push_str(") |\n");
        }
        markdown.push_str("| **Average** | — | — | — | — | — | — | **");
        markdown.push_str(&format!("{:.2}%", self.average_match_percentage()));
        markdown.push_str("** | — |\n");

        markdown
    }
}

/// Compare directories after applying checked-in source-normalization patches
/// to reference files in memory. The upstream reference tree remains intact.
pub fn compare_directories_with_policy_and_patches(
    reference_root: impl AsRef<Path>,
    generated_root: impl AsRef<Path>,
    patches_root: Option<&Path>,
    snapshot: Option<String>,
    exclusions: &Exclusions,
) -> Result<ConformanceReport, ReportError> {
    let reference_root = reference_root.as_ref();
    let generated_root = generated_root.as_ref();
    validate_root(reference_root, "reference")?;
    validate_root(generated_root, "generated")?;

    let services = service_names(reference_root, generated_root)?;
    let reports = std::thread::scope(|scope| {
        let workers = services
            .into_iter()
            .map(|service| {
                let reference_path = reference_root.join(&service);
                let generated_path = generated_root.join(&service);
                let patches_path = patches_root.map(|root| root.join(&service));
                scope.spawn(move || {
                    compare_service(
                        &service,
                        &reference_path,
                        &generated_path,
                        patches_path,
                        exclusions,
                    )
                })
            })
            .collect::<Vec<_>>();
        workers
            .into_iter()
            .map(|worker| worker.join().map_err(|_| ReportError::WorkerPanic)?)
            .collect::<Result<Vec<_>, ReportError>>()
    })?;

    Ok(ConformanceReport {
        snapshot,
        services: reports,
    })
}

/// Write the summary report and one complete report per service.
///
/// The summary path `conformance/summary.md` produces service reports at
/// `conformance/summary/<service>.md`. Each file is replaced atomically,
/// and stale Markdown files in that generated directory are removed after all
/// current service reports have been written.
pub fn write_reports(
    summary_path: impl AsRef<Path>,
    report: &ConformanceReport,
) -> Result<(), ReportError> {
    let summary_path = summary_path.as_ref();
    let parent = summary_path.parent().unwrap_or_else(|| Path::new("."));
    let service_directory_name = summary_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| ReportError::InvalidOutputPath(summary_path.to_owned()))?;
    let service_directory = parent.join(service_directory_name);
    fs::create_dir_all(&service_directory)
        .map_err(|source| ReportError::io(&service_directory, source))?;

    let mut current_reports = BTreeSet::new();
    for service in &report.services {
        let filename = format!("{}.md", service.name);
        current_reports.insert(filename.clone());
        write_text(
            &service_directory.join(&filename),
            &service.to_markdown(report.snapshot.as_deref()),
        )?;
    }

    let entries = fs::read_dir(&service_directory)
        .map_err(|source| ReportError::io(&service_directory, source))?;
    for entry in entries {
        let entry = entry.map_err(|source| ReportError::io(&service_directory, source))?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|source| ReportError::io(&path, source))?
            .is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("md")
            && !current_reports.contains(&entry.file_name().to_string_lossy().into_owned())
        {
            fs::remove_file(&path).map_err(|source| ReportError::io(&path, source))?;
        }
    }

    write_text(
        summary_path,
        &report.to_summary_markdown(Path::new(service_directory_name)),
    )
}

fn write_text(path: &Path, contents: &str) -> Result<(), ReportError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|source| ReportError::io(parent, source))?;

    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("report")
    ));
    let contents = format!("{}\n", contents.trim_end_matches('\n'));
    fs::write(&temporary, contents).map_err(|source| ReportError::io(&temporary, source))?;
    if let Err(source) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(ReportError::io(path, source));
    }
    Ok(())
}

fn validate_root(path: &Path, label: &str) -> Result<(), ReportError> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(ReportError::InvalidRoot {
            label: label.to_owned(),
            path: path.to_owned(),
        })
    }
}

fn service_names(reference_root: &Path, generated_root: &Path) -> Result<Vec<String>, ReportError> {
    let mut names = BTreeSet::new();
    for root in [reference_root, generated_root] {
        let entries = fs::read_dir(root).map_err(|source| ReportError::io(root, source))?;
        for entry in entries {
            let entry = entry.map_err(|source| ReportError::io(root, source))?;
            if entry
                .file_type()
                .map_err(|source| ReportError::io(entry.path(), source))?
                .is_dir()
            {
                names.insert(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    Ok(names.into_iter().collect())
}

fn compare_service(
    name: &str,
    reference_root: &Path,
    generated_root: &Path,
    patches_root: Option<PathBuf>,
    exclusions: &Exclusions,
) -> Result<ServiceReport, ReportError> {
    let reference_files = collect_files(reference_root, exclusions)?;
    let projection_root = generated_root.join("normalized");
    let generated_files = if projection_root.is_dir() {
        collect_files(&projection_root, exclusions)?
    } else {
        collect_files(generated_root, exclusions)?
    };
    let paths = reference_files
        .keys()
        .chain(generated_files.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut report = ServiceReport {
        name: name.to_owned(),
        total_files: paths.len(),
        compared_files: paths.len(),
        matched_files: 0,
        mismatched_files: 0,
        missing_files: Vec::new(),
        extra_files: Vec::new(),
        binary_mismatches: Vec::new(),
        read_errors: Vec::new(),
        differences: Vec::new(),
    };

    for relative_path in paths {
        let reference_path = reference_files.get(&relative_path);
        let generated_path = generated_files.get(&relative_path);
        match (reference_path, generated_path) {
            (Some(_), None) => report
                .missing_files
                .push(relative_path.display().to_string()),
            (None, Some(_)) => report.extra_files.push(relative_path.display().to_string()),
            (Some(reference_path), Some(generated_path)) => {
                let reference = match fs::read(reference_path) {
                    Ok(bytes) => bytes,
                    Err(source) => {
                        report.read_errors.push(format!(
                            "reference/{}: {}",
                            relative_path.display(),
                            source
                        ));
                        continue;
                    }
                };
                let generated = match fs::read(generated_path) {
                    Ok(bytes) => bytes,
                    Err(source) => {
                        report.read_errors.push(format!(
                            "generated/{}: {}",
                            relative_path.display(),
                            source
                        ));
                        continue;
                    }
                };
                let reference = match apply_reference_patch(
                    &reference,
                    patches_root.as_deref().and_then(|root| {
                        let patch_path = normalize::patch_path(root, &relative_path);
                        patch_path.exists().then_some(patch_path)
                    }),
                    &relative_path,
                ) {
                    Ok(reference) => reference,
                    Err(error) => {
                        report.read_errors.push(error);
                        continue;
                    }
                };
                if reference == generated || rust_tokens_equal(&reference, &generated) {
                    report.matched_files += 1;
                    continue;
                } else {
                    report.mismatched_files += 1;
                    match (
                        std::str::from_utf8(&reference),
                        std::str::from_utf8(&generated),
                    ) {
                        (Ok(reference), Ok(generated)) => {
                            let mut patch = diffy::create_patch(&reference, &generated).to_string();
                            patch = replace_patch_headers(&patch, &relative_path);
                            report.differences.push(FileDifference {
                                path: relative_path.display().to_string(),
                                patch,
                            });
                        }
                        _ => report
                            .binary_mismatches
                            .push(relative_path.display().to_string()),
                    }
                }
            }
            (None, None) => unreachable!("path came from one of the file maps"),
        }
    }

    Ok(report)
}

fn apply_reference_patch(
    reference: &[u8],
    patch_path: Option<PathBuf>,
    relative_path: &Path,
) -> Result<Vec<u8>, String> {
    let source = std::str::from_utf8(reference).map_err(|error| {
        format!(
            "reference file {} is not UTF-8: {error}",
            relative_path.display()
        )
    })?;
    let normalized = if let Some(patch_path) = patch_path {
        let patch = fs::read_to_string(&patch_path)
            .map_err(|error| format!("reference patch {}: {error}", patch_path.display()))?;
        let patch = diffy::Patch::from_str(&patch).map_err(|error| {
            format!(
                "reference patch {} for {} is invalid: {error}",
                patch_path.display(),
                relative_path.display()
            )
        })?;
        diffy::apply(source, &patch).map_err(|error| {
            format!(
                "reference patch {} for {} does not apply: {error}",
                patch_path.display(),
                relative_path.display()
            )
        })?
    } else {
        source.to_owned()
    };
    normalize::drop_inline_unit_tests(&normalized, relative_path).map(|source| source.into_bytes())
}

/// Compare parsed Rust sources without depending on rustfmt line breaks.
///
/// Token-stream equality retains comments and documentation strings, so source
/// changes in those texts still fail conformance. It only ignores layout
/// choices such as wrapping introduced by longer canonical module paths.
fn rust_tokens_equal(reference: &[u8], generated: &[u8]) -> bool {
    let (Ok(reference), Ok(generated)) = (
        std::str::from_utf8(reference),
        std::str::from_utf8(generated),
    ) else {
        return false;
    };
    let (Ok(reference), Ok(generated)) = (syn::parse_file(reference), syn::parse_file(generated))
    else {
        return false;
    };
    reference.to_token_stream().to_string() == generated.to_token_stream().to_string()
}

fn collect_files(
    root: &Path,
    exclusions: &Exclusions,
) -> Result<std::collections::BTreeMap<PathBuf, PathBuf>, ReportError> {
    let mut files = std::collections::BTreeMap::new();
    if !root.exists() {
        return Ok(files);
    }
    collect_files_recursively(root, root, Path::new(""), exclusions, &mut files)?;
    Ok(files)
}

fn collect_files_recursively(
    root: &Path,
    current: &Path,
    relative_root: &Path,
    exclusions: &Exclusions,
    files: &mut std::collections::BTreeMap<PathBuf, PathBuf>,
) -> Result<(), ReportError> {
    for entry in fs::read_dir(current).map_err(|source| ReportError::io(current, source))? {
        let entry = entry.map_err(|source| ReportError::io(current, source))?;
        let path = entry.path();
        let relative = relative_root.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|source| ReportError::io(&path, source))?;
        if exclusions.excludes(&relative) {
            continue;
        }
        if file_type.is_dir() {
            collect_files_recursively(root, &path, &relative, exclusions, files)?;
        } else if file_type.is_file() {
            let root_relative = path
                .strip_prefix(root)
                .expect("recursive paths must stay below root")
                .to_owned();
            files.insert(root_relative, path);
        }
    }
    Ok(())
}

fn replace_patch_headers(patch: &str, relative_path: &Path) -> String {
    let path = relative_path.display();
    let mut lines = patch.lines();
    let mut result = String::new();
    if let Some(first) = lines.next() {
        if first.starts_with("--- ") {
            result.push_str("--- reference/");
            result.push_str(&path.to_string());
            result.push('\n');
        } else {
            result.push_str(first);
            result.push('\n');
        }
    }
    if let Some(second) = lines.next() {
        if second.starts_with("+++ ") {
            result.push_str("+++ generated/");
            result.push_str(&path.to_string());
            result.push('\n');
        } else {
            result.push_str(second);
            result.push('\n');
        }
    }
    for line in lines {
        result.push_str(line);
        result.push('\n');
    }
    result
}

fn append_diagnostics(markdown: &mut String, title: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    markdown.push_str("### ");
    markdown.push_str(title);
    markdown.push_str("\n\n");
    for value in values {
        markdown.push_str("- `");
        markdown.push_str(&escape_inline(value));
        markdown.push_str("`\n");
    }
    markdown.push('\n');
}

fn format_summary(report: &ConformanceReport) -> String {
    format!(
        "`{}/{} files compared` · `{}` matched · `{}` mismatches · `{}` missing · `{}` extra · `{}` read errors · `{:.2}%` average match",
        report.compared_files(),
        report.total_files(),
        report.matched_files(),
        report.mismatched_files(),
        report.missing_files(),
        report.extra_files(),
        report.read_errors(),
        report.average_match_percentage(),
    )
}

fn escape_inline(value: &str) -> String {
    value.replace('`', "\\`").replace('\n', " ")
}

fn escape_table(value: &str) -> String {
    escape_inline(value).replace('|', "\\|")
}

fn append_progress(markdown: &mut String, service: &ServiceReport) {
    markdown.push_str("**Progress:** `");
    markdown.push_str(&service.compared_files.to_string());
    markdown.push('/');
    markdown.push_str(&service.total_files.to_string());
    markdown.push_str("` files compared · `");
    markdown.push_str(&service.matched_files.to_string());
    markdown.push_str("` matched · `");
    markdown.push_str(&service.mismatched_files.to_string());
    markdown.push_str("` mismatches · `");
    markdown.push_str(&service.missing_files.len().to_string());
    markdown.push_str("` missing · `");
    markdown.push_str(&service.extra_files.len().to_string());
    markdown.push_str("` extra · `");
    markdown.push_str(&format!("{:.2}%", service.match_percentage()));
    markdown.push_str("` match (100.00% means fully matched)\n\n");
}

fn append_service_details(markdown: &mut String, service: &ServiceReport) {
    markdown.push_str("## ");
    markdown.push_str(&service.name);
    markdown.push('\n');
    append_progress(markdown, service);

    for difference in &service.differences {
        markdown.push_str("### `");
        markdown.push_str(&escape_inline(&difference.path));
        markdown.push_str("`\n\n```diff\n");
        markdown.push_str(&difference.patch);
        if !difference.patch.ends_with('\n') {
            markdown.push('\n');
        }
        markdown.push_str("```\n\n");
    }

    append_diagnostics(markdown, "Missing reference files", &service.missing_files);
    append_diagnostics(markdown, "Unexpected generated files", &service.extra_files);
    append_diagnostics(
        markdown,
        "Binary file differences",
        &service.binary_mismatches,
    );
    append_diagnostics(markdown, "Read errors", &service.read_errors);
}

#[derive(Debug)]
pub enum ReportError {
    InvalidRoot { label: String, path: PathBuf },
    InvalidOutputPath(PathBuf),
    WorkerPanic,
    Io { path: PathBuf, source: io::Error },
}

impl ReportError {
    fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for ReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoot { label, path } => {
                write!(
                    formatter,
                    "{label} root is not a directory: {}",
                    path.display()
                )
            }
            Self::InvalidOutputPath(path) => write!(
                formatter,
                "report output path has no valid filename: {}",
                path.display()
            ),
            Self::WorkerPanic => formatter.write_str("conformance worker thread panicked"),
            Self::Io { path, source } => write!(formatter, "{}: {source}", path.display()),
        }
    }
}

impl Error for ReportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRoot { .. } | Self::InvalidOutputPath(_) | Self::WorkerPanic => None,
            Self::Io { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn compare_default(
        reference_root: &Path,
        generated_root: &Path,
        snapshot: Option<String>,
    ) -> ConformanceReport {
        compare_directories_with_policy_and_patches(
            reference_root,
            generated_root,
            None,
            snapshot,
            &Exclusions::default(),
        )
        .unwrap()
    }

    #[test]
    fn report_puts_progress_before_diffs_and_uses_diffy_headers() {
        let reference = tempdir().unwrap();
        let generated = tempdir().unwrap();
        fs::create_dir_all(reference.path().join("s3/operation")).unwrap();
        fs::create_dir_all(generated.path().join("s3/operation")).unwrap();
        fs::write(
            reference.path().join("s3/operation/get.rs"),
            "pub struct Get;\nold\n",
        )
        .unwrap();
        fs::write(
            generated.path().join("s3/operation/get.rs"),
            "pub struct Get;\nnew\n",
        )
        .unwrap();
        fs::write(reference.path().join("s3/same.rs"), "same\n").unwrap();
        fs::write(generated.path().join("s3/same.rs"), "same\n").unwrap();
        fs::write(generated.path().join("s3/extra.rs"), "extra\n").unwrap();

        let report = compare_default(reference.path(), generated.path(), Some("abc".to_owned()));
        let markdown = report.services[0].to_markdown(report.snapshot.as_deref());
        let progress = markdown.find("**Progress:**").unwrap();
        let diff = markdown.find("```diff").unwrap();

        assert!(progress < diff);
        assert!(markdown.contains("## s3\n**Progress:** `3/3` files compared · `1` matched · `1` mismatches · `0` missing · `1` extra"));
        assert!(markdown.contains("--- reference/operation/get.rs"));
        assert!(markdown.contains("+++ generated/operation/get.rs"));
        assert!(markdown.contains("Snapshot: `abc`"));
        assert!(markdown.contains("`33.33%` match (100.00% means fully matched)"));
        assert!((report.average_match_percentage() - 33.3333).abs() < 0.001);
    }

    #[test]
    fn equal_services_have_a_clean_progress_summary() {
        let reference = tempdir().unwrap();
        let generated = tempdir().unwrap();
        fs::create_dir_all(reference.path().join("dynamodb")).unwrap();
        fs::create_dir_all(generated.path().join("dynamodb")).unwrap();
        fs::write(
            reference.path().join("dynamodb/lib.rs"),
            "pub struct Client;\n",
        )
        .unwrap();
        fs::write(
            generated.path().join("dynamodb/lib.rs"),
            "pub struct Client;\n",
        )
        .unwrap();

        let report = compare_default(reference.path(), generated.path(), None);
        assert!(!report.has_differences());
        assert_eq!(report.services[0].compared_files, 1);
        assert_eq!(report.services[0].match_percentage(), 100.0);
        assert_eq!(report.average_match_percentage(), 100.0);
        assert!(report.services[0]
            .to_markdown(None)
            .contains("**Progress:** `1/1` files compared · `1` matched · `0` mismatches · `0` missing · `0` extra · `100.00%` match"));
    }

    #[test]
    fn excluded_files_do_not_affect_comparison_counts() {
        let reference = tempdir().unwrap();
        let generated = tempdir().unwrap();
        fs::create_dir_all(reference.path().join("s3/tests")).unwrap();
        fs::create_dir_all(generated.path().join("s3/tests")).unwrap();
        fs::write(reference.path().join("s3/README.md"), "reference").unwrap();
        fs::write(reference.path().join("s3/LICENSE"), "reference").unwrap();
        fs::write(generated.path().join("s3/tests/extra.rs"), "generated").unwrap();
        fs::write(generated.path().join("s3/LICENSE"), "generated").unwrap();
        fs::write(reference.path().join("s3/src.rs"), "pub struct Same;\n").unwrap();
        fs::write(generated.path().join("s3/src.rs"), "pub struct Same;\n").unwrap();
        let exclusions = Exclusions {
            files: vec!["README.md".to_owned(), "LICENSE".to_owned()],
            directories: vec!["tests".to_owned()],
        };

        let report = compare_directories_with_policy_and_patches(
            reference.path(),
            generated.path(),
            None,
            None,
            &exclusions,
        )
        .unwrap();

        assert!(!report.has_differences());
        assert_eq!(report.services[0].compared_files, 1);
        assert_eq!(report.services[0].matched_files, 1);
    }

    #[test]
    fn applies_reference_normalization_patch_before_comparing() {
        let reference = tempdir().unwrap();
        let generated = tempdir().unwrap();
        let patches = tempdir().unwrap();
        fs::create_dir_all(reference.path().join("s3")).unwrap();
        fs::create_dir_all(generated.path().join("s3")).unwrap();
        fs::create_dir_all(patches.path().join("s3")).unwrap();
        let original = "use crate::types::Thing;\n";
        let normalized = "use super::types::Thing;\n";
        fs::write(reference.path().join("s3/lib.rs"), original).unwrap();
        fs::write(generated.path().join("s3/lib.rs"), normalized).unwrap();
        fs::write(
            normalize::patch_path(patches.path().join("s3").as_path(), Path::new("lib.rs")),
            diffy::create_patch(original, normalized).to_string(),
        )
        .unwrap();

        let report = compare_directories_with_policy_and_patches(
            reference.path(),
            generated.path(),
            Some(patches.path()),
            None,
            &Exclusions::default(),
        )
        .unwrap();
        assert!(!report.has_differences());
        assert_eq!(report.services[0].matched_files, 1);
    }

    #[test]
    fn write_reports_splits_service_details_and_removes_stale_reports() {
        let reference = tempdir().unwrap();
        let generated = tempdir().unwrap();
        fs::create_dir_all(reference.path().join("s3")).unwrap();
        fs::create_dir_all(generated.path().join("s3")).unwrap();
        fs::write(reference.path().join("s3/lib.rs"), "old\n").unwrap();
        fs::write(generated.path().join("s3/lib.rs"), "new\n").unwrap();

        let report = compare_default(reference.path(), generated.path(), Some("abc".into()));
        let output_root = tempdir().unwrap();
        let output = output_root.path().join("reports/conformance.md");
        let service_directory = output_root.path().join("reports/conformance");
        fs::create_dir_all(&service_directory).unwrap();
        fs::write(service_directory.join("stale.md"), "stale\n").unwrap();

        write_reports(&output, &report).unwrap();

        let summary = fs::read_to_string(&output).unwrap();
        let service = fs::read_to_string(service_directory.join("s3.md")).unwrap();
        assert!(summary.contains("**Summary:**"));
        assert!(summary.contains("| Service | Compared | Matched | Mismatches | Missing | Extra | Read errors | Match | Report |"));
        assert!(
            summary
                .contains("| s3 | 1/1 | 0 | 1 | 0 | 0 | 0 | 0.00% | [report](conformance/s3.md) |")
        );
        assert!(summary.contains("| **Average** | — | — | — | — | — | — | **0.00%** | — |"));
        assert!(!summary.contains("```diff"));
        assert!(service.contains("# AWS SDK Conformance Report: s3"));
        assert!(service.contains("```diff"));
        assert!(!service_directory.join("stale.md").exists());
    }
}
