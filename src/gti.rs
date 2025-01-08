//! # Git Temp Ignore (GTI)
//!
//! GTI is a wrapper around git that streamlines temporarily ignoring files changes when
//! running commands. It's particularly useful if a build script, outside of your control,
//! makes minor changes to a repository that does not need to be tracked.
//!
//! It is possible in vanilla git to ignore these changes, but can become cumbersome when
//! re-running scripts when relevant major changes have already been made but not yet
//! committed.

use std::{
    env,
    ffi::OsStr,
    fs::DirBuilder,
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum InitializationError {
    #[error("git is not installed")]
    GitNotInstalled,
    #[error("could not validate git installation; version check failed")]
    GitValidation,
    #[error("io error searching for git directory")]
    RepositoryUnsearchable(#[from] io::Error),
    #[error("git directory not found")]
    RepositoryNotFound,
}

/// Fallback logger macros in the case that the default logger has failed or is not initialized.
#[macro_export]
macro_rules! fallback_log {
    ($e:expr) => {
        println!("gti: {}", format!("{}", $e))
    };

    ($fmt:literal, $($args:expr),*) => {
        fallback_log!(format!($fmt, $($args),*))
    };
}

#[derive(Debug)]
pub(crate) struct GtiManager {
    #[allow(dead_code)]
    gti_dir: PathBuf,
}

impl GtiManager {
    /// Build a GTI manager that contains a path to the `.git` directory.
    pub fn new(repo_git_dir: &Path) -> io::Result<Self> {
        if !repo_git_dir.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                ".git directory not found",
            ));
        }

        // .git directory exists, meaning that we can properly initialize .git.
        let gti_dir = repo_git_dir.join("x-gti-info");
        if !gti_dir.exists() {
            DirBuilder::new().create(&gti_dir)?;
        }

        Ok(GtiManager { gti_dir })
    }
}

/// Validate the git installation. Check if git is installed and the version can be correctly obtained.
pub(crate) fn git_validate_status() -> Result<PathBuf, InitializationError> {
    let git_validate_status = Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match git_validate_status {
        Ok(pass) if !pass.success() => Err(InitializationError::GitValidation),
        Err(_) => Err(InitializationError::GitNotInstalled),
        _ => {
            // git ran successfully -- search for git directory
            let mut cur_dir = env::current_dir()?;
            let look_for = OsStr::new(".git");
            while cur_dir.parent().is_some() {
                let repo_git_dir = cur_dir
                    .read_dir()?
                    .flatten()
                    .map(|entry| entry.path())
                    .find(|path| match path.file_name() {
                        Some(s) => s == look_for,
                        _ => false,
                    });

                if let Some(s) = repo_git_dir {
                    return Ok(s);
                }

                // No .git found, pop cur_dir and go up a directory.
                cur_dir.pop();
            }

            Err(InitializationError::RepositoryNotFound)
        }
    }
}
