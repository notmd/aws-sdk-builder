use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use crate::error::BuildError;

pub fn resolve_executable<E, P>(
    explicit: Option<&Path>,
    environment: E,
    path_lookup: P,
) -> Result<PathBuf, BuildError>
where
    E: Fn(&str) -> Option<OsString>,
    P: Fn(&OsStr) -> Option<PathBuf>,
{
    let mut searched = Vec::new();

    if let Some(path) = explicit {
        searched.push(format!("explicit path {}", path.display()));
        if path.is_file() {
            return Ok(path.to_path_buf());
        }
    }

    if let Some(value) = environment("SMITHY_CLI") {
        let path = PathBuf::from(value);
        searched.push(format!("SMITHY_CLI {}", path.display()));
        if path.is_file() {
            return Ok(path);
        }
    } else {
        searched.push("SMITHY_CLI (not set)".to_owned());
    }

    if let Some(path) = path_lookup(OsStr::new("smithy")) {
        searched.push(format!("PATH {}", path.display()));
        if path.is_file() {
            return Ok(path);
        }
    } else {
        searched.push("PATH (smithy not found)".to_owned());
    }

    Err(BuildError::SmithyExecutableNotFound { searched })
}

pub fn resolve_from_environment(explicit: Option<&Path>) -> Result<PathBuf, BuildError> {
    resolve_executable(explicit, |name| env::var_os(name), find_on_path)
}

pub fn run(executable: &Path, cwd: &Path) -> Result<ExitStatus, BuildError> {
    let output = Command::new(executable)
        .arg("build")
        .current_dir(cwd)
        .output()
        .map_err(|source| BuildError::SmithySpawn {
            command: format!("{} build", executable.display()),
            source,
        })?;
    if output.status.success() {
        return Ok(output.status);
    }

    Err(BuildError::SmithyToolFailed {
        command: format!("{} build", executable.display()),
        status: output.status.to_string(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

fn find_on_path(name: &OsStr) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).find_map(|directory| {
        let candidate = directory.join(name);
        fs::metadata(&candidate)
            .ok()
            .filter(|metadata| metadata.is_file())
            .map(|_| candidate)
    })
}
