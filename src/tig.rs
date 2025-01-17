//! # Temp Ignore Git (TIG)
//!
//! TIG is a wrapper around git that streamlines temporarily ignoring files changes when
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
pub enum InitializationError {
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
        println!("tig: {}", format!("{}", $e))
    };

    ($fmt:literal, $($args:expr),*) => {
        fallback_log!(format!($fmt, $($args),*))
    };
}

#[derive(Debug)]
pub struct TigManager {
    #[allow(dead_code)]
    tig_dir: PathBuf,
}

impl TigManager {
    /// Build a TIG manager that contains a path to the `.git` directory.
    pub fn new(repo_git_dir: &Path) -> io::Result<Self> {
        if !repo_git_dir.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                ".git directory not found",
            ));
        }

        // .git directory exists, meaning that we can properly initialize .git.
        let tig_dir = repo_git_dir.join("x-tig-info");
        if !tig_dir.exists() {
            DirBuilder::new().create(&tig_dir)?;
        }

        Ok(TigManager { tig_dir })
    }
}

/// Validate the git installation. Check if git is installed and the version can be correctly obtained.
pub fn git_validate_status() -> Result<PathBuf, InitializationError> {
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

mod tests {

    #[allow(unused_imports)]
    use super::*;

    /// By nature of this test existing within the repo of this project, it
    /// is expected to pass as long as file permissions for the repo are configured
    /// correctly. Should also work w/ GH Actions since the repo must first be
    /// cloned.
    #[test]
    fn git_validate_status_is_ok_when_in_git_repo() {
        let status = git_validate_status();
        assert!(status.is_ok(), "{}", format!("{:?}", status));

        let repo_git_dir = status.unwrap();
        assert!(
            repo_git_dir.file_name().is_some(),
            "repo git directory terminates in \"..\""
        );

        let file_name = repo_git_dir.file_name().unwrap().to_str();
        assert!(
            file_name.is_some(),
            "repo git directory does not contain valid unicode"
        );

        assert_eq!(file_name.unwrap(), ".git");
    }
}
