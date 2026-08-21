use std::{fs, path::Path};

use aws_sdk_build::runner::{resolve_executable, run};
use tempfile::tempdir;

#[test]
fn executable_resolution_prefers_explicit_then_environment_then_path() {
    let directory = tempdir().unwrap();
    let explicit = directory.path().join("explicit-smithy");
    let environment = directory.path().join("environment-smithy");
    let path = directory.path().join("path-smithy");
    for file in [&explicit, &environment, &path] {
        fs::write(file, "").unwrap();
    }

    let resolved = resolve_executable(
        Some(&explicit),
        |_| Some(environment.as_os_str().to_owned()),
        |_| Some(path.clone()),
    )
    .unwrap();
    assert_eq!(resolved, fs::canonicalize(&explicit).unwrap());

    let resolved = resolve_executable(
        None,
        |_| Some(environment.as_os_str().to_owned()),
        |_| Some(path.clone()),
    )
    .unwrap();
    assert_eq!(resolved, fs::canonicalize(&environment).unwrap());

    let resolved = resolve_executable(None, |_| None, |_| Some(path.clone())).unwrap();
    assert_eq!(resolved, fs::canonicalize(&path).unwrap());
}

#[test]
fn missing_executable_error_identifies_all_lookup_sources() {
    let error = resolve_executable(
        Some(Path::new("/missing/explicit-smithy")),
        |_| None,
        |_| None,
    )
    .unwrap_err();
    let message = error.to_string();

    assert!(message.contains("SMITHY_CLI"));
    assert!(message.contains("PATH"));
}

#[cfg(unix)]
#[test]
fn failed_smithy_process_includes_command_and_stderr() {
    let error = run(Path::new("/bin/sh"), Path::new("/tmp")).unwrap_err();
    let message = error.to_string();

    assert!(message.contains("/bin/sh build"));
    assert!(message.contains("build"));
}
