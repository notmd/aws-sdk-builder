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
    pub token_mismatches: Vec<String>,
    pub read_errors: Vec<String>,
    pub differences: Vec<FileDifference>,
}

impl ServiceReport {
    pub fn has_differences(&self) -> bool {
        self.mismatched_files != 0
            || !self.missing_files.is_empty()
            || !self.extra_files.is_empty()
            || !self.binary_mismatches.is_empty()
            || !self.token_mismatches.is_empty()
            || !self.read_errors.is_empty()
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

    pub fn token_mismatches(&self) -> usize {
        self.services
            .iter()
            .map(|service| service.token_mismatches.len())
            .sum()
    }

    /// Render a deterministic Markdown document. The progress line is deliberately
    /// the first content line after each service heading.
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# AWS SDK Conformance Report\n\n");
        if let Some(snapshot) = &self.snapshot {
            markdown.push_str("Snapshot: `");
            markdown.push_str(&escape_inline(snapshot));
            markdown.push_str("`\n\n");
        }
        markdown.push_str("**Summary:** ");
        markdown.push_str(&format_summary(self));
        markdown.push_str("\n\n");

        for service in &self.services {
            markdown.push_str("## ");
            markdown.push_str(&service.name);
            markdown.push('\n');
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
            markdown.push_str("` extra\n\n");

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

            append_diagnostics(
                &mut markdown,
                "Missing reference files",
                &service.missing_files,
            );
            append_diagnostics(
                &mut markdown,
                "Unexpected generated files",
                &service.extra_files,
            );
            append_diagnostics(
                &mut markdown,
                "Binary file differences",
                &service.binary_mismatches,
            );
            append_diagnostics(
                &mut markdown,
                "Rust token differences",
                &service.token_mismatches,
            );
            append_diagnostics(&mut markdown, "Read errors", &service.read_errors);
        }

        markdown
    }
}

/// Compare service directories below `reference_root` and `generated_root`.
///
/// Each immediate child directory is treated as an SDK service. Both roots may
/// contain a service that is absent from the other root; that condition is recorded
/// in the report instead of being treated as an I/O failure.
pub fn compare_directories(
    reference_root: impl AsRef<Path>,
    generated_root: impl AsRef<Path>,
    snapshot: Option<String>,
) -> Result<ConformanceReport, ReportError> {
    let reference_root = reference_root.as_ref();
    let generated_root = generated_root.as_ref();
    validate_root(reference_root, "reference")?;
    validate_root(generated_root, "generated")?;

    let services = service_names(reference_root, generated_root)?;
    let mut reports = Vec::with_capacity(services.len());
    for service in services {
        reports.push(compare_service(
            &service,
            &reference_root.join(&service),
            &generated_root.join(&service),
        )?);
    }

    Ok(ConformanceReport {
        snapshot,
        services: reports,
    })
}

/// Write a report using a temporary sibling file and an atomic rename.
pub fn write_markdown(
    path: impl AsRef<Path>,
    report: &ConformanceReport,
) -> Result<(), ReportError> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|source| ReportError::io(parent, source))?;

    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("report")
    ));
    fs::write(&temporary, report.to_markdown())
        .map_err(|source| ReportError::io(&temporary, source))?;
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
) -> Result<ServiceReport, ReportError> {
    let reference_files = collect_files(reference_root)?;
    let generated_files = collect_files(generated_root)?;
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
        token_mismatches: Vec::new(),
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
                if reference == generated {
                    report.matched_files += 1;
                } else {
                    report.mismatched_files += 1;
                    match (String::from_utf8(reference), String::from_utf8(generated)) {
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
                validate_rust_pair(
                    &relative_path,
                    reference_path,
                    generated_path,
                    &mut report.token_mismatches,
                    &mut report.read_errors,
                );
            }
            (None, None) => unreachable!("path came from one of the file maps"),
        }
    }

    Ok(report)
}

fn validate_rust_pair(
    relative_path: &Path,
    reference_path: &Path,
    generated_path: &Path,
    token_mismatches: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    if relative_path
        .extension()
        .and_then(|extension| extension.to_str())
        != Some("rs")
    {
        return;
    }
    let reference = match fs::read_to_string(reference_path) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!("reference/{}: {}", relative_path.display(), error));
            return;
        }
    };
    let generated = match fs::read_to_string(generated_path) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!("generated/{}: {}", relative_path.display(), error));
            return;
        }
    };
    let reference_file = match syn::parse_file(&reference) {
        Ok(file) => file,
        Err(error) => {
            errors.push(format!(
                "reference/{} parse error: {}",
                relative_path.display(),
                error
            ));
            return;
        }
    };
    let generated_file = match syn::parse_file(&generated) {
        Ok(file) => file,
        Err(error) => {
            errors.push(format!(
                "generated/{} parse error: {}",
                relative_path.display(),
                error
            ));
            return;
        }
    };
    let reference_tokens = quote::quote!(#reference_file).to_string();
    let generated_tokens = quote::quote!(#generated_file).to_string();
    if reference_tokens != generated_tokens {
        token_mismatches.push(relative_path.display().to_string());
    }
}

fn collect_files(root: &Path) -> Result<std::collections::BTreeMap<PathBuf, PathBuf>, ReportError> {
    let mut files = std::collections::BTreeMap::new();
    if !root.exists() {
        return Ok(files);
    }
    collect_files_recursively(root, root, &mut files)?;
    Ok(files)
}

fn collect_files_recursively(
    root: &Path,
    current: &Path,
    files: &mut std::collections::BTreeMap<PathBuf, PathBuf>,
) -> Result<(), ReportError> {
    for entry in fs::read_dir(current).map_err(|source| ReportError::io(current, source))? {
        let entry = entry.map_err(|source| ReportError::io(current, source))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| ReportError::io(&path, source))?;
        if file_type.is_dir() {
            collect_files_recursively(root, &path, files)?;
        } else if file_type.is_file() {
            let relative = path
                .strip_prefix(root)
                .expect("recursive paths must stay below root")
                .to_owned();
            files.insert(relative, path);
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
        "`{}/{} files compared` · `{}` matched · `{}` mismatches · `{}` missing · `{}` extra · `{}` read errors",
        report.compared_files(),
        report.total_files(),
        report.matched_files(),
        report.mismatched_files(),
        report.missing_files(),
        report.extra_files(),
        report.read_errors(),
    )
}

fn escape_inline(value: &str) -> String {
    value.replace('`', "\\`").replace('\n', " ")
}

#[derive(Debug)]
pub enum ReportError {
    InvalidRoot { label: String, path: PathBuf },
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
            Self::Io { path, source } => write!(formatter, "{}: {source}", path.display()),
        }
    }
}

impl Error for ReportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRoot { .. } => None,
            Self::Io { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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

        let report =
            compare_directories(reference.path(), generated.path(), Some("abc".to_owned()))
                .unwrap();
        let markdown = report.to_markdown();
        let progress = markdown.find("**Progress:**").unwrap();
        let diff = markdown.find("```diff").unwrap();

        assert!(progress < diff);
        assert!(markdown.contains("## s3\n**Progress:** `3/3` files compared · `1` matched · `1` mismatches · `0` missing · `1` extra"));
        assert!(markdown.contains("--- reference/operation/get.rs"));
        assert!(markdown.contains("+++ generated/operation/get.rs"));
        assert!(markdown.contains("Snapshot: `abc`"));
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

        let report =
            compare_directories(reference.path(), generated.path(), None::<String>).unwrap();
        assert!(!report.has_differences());
        assert_eq!(report.services[0].compared_files, 1);
        assert!(report
            .to_markdown()
            .contains("**Progress:** `1/1` files compared · `1` matched · `0` mismatches · `0` missing · `0` extra"));
    }

    #[test]
    fn write_markdown_creates_parent_and_replaces_the_report() {
        let reference = tempdir().unwrap();
        let generated = tempdir().unwrap();
        fs::create_dir_all(reference.path().join("s3")).unwrap();
        fs::create_dir_all(generated.path().join("s3")).unwrap();
        fs::write(reference.path().join("s3/lib.rs"), "old\n").unwrap();
        fs::write(generated.path().join("s3/lib.rs"), "new\n").unwrap();

        let report = compare_directories(reference.path(), generated.path(), None).unwrap();
        let output_root = tempdir().unwrap();
        let output = output_root.path().join("reports/conformance.md");
        write_markdown(&output, &report).unwrap();

        let markdown = fs::read_to_string(output).unwrap();
        assert!(markdown.contains("# AWS SDK Conformance Report"));
        assert!(markdown.contains("--- reference/lib.rs"));
        assert!(markdown.contains("+++ generated/lib.rs"));
    }
}
