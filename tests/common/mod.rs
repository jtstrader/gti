//! GTI test setup/cleanup helpers.

use std::{
    fs, io,
    path::PathBuf,
    process::{Command, Stdio},
    sync::LazyLock,
};

/*
 * NOTE: project_root is probably unnecessary but I kept seeing conflicting statements
 * on the directory that cargo chooses when you run in depending on where you run it from.
 * This is just a guarantee that everything will be located in the same place.
 */
pub static TEST_ENV_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match project_root::get_project_root() {
        Ok(path) => path.join("tests").join("env"),
        Err(e) => panic!("could not load test environment path with error: {}", e),
    });

/// Get a path to the test environment `.git` directory.
pub fn repo_git_dir() -> PathBuf {
    TEST_ENV_PATH.join(".git")
}

/// Initialize the git repository for testing.
pub fn setup() -> io::Result<()> {
    // Ensure that env is created first.
    if !TEST_ENV_PATH.exists() {
        fs::DirBuilder::new().create(&*TEST_ENV_PATH)?;
    }

    std::env::set_current_dir(&*TEST_ENV_PATH)?;
    Command::new("git")
        .arg("init")
        .stdout(Stdio::null())
        .status()?;
    Ok(())
}

/// Remove the git repository to cleanly test the next suite.
pub fn cleanup() -> io::Result<()> {
    fs::remove_dir_all(TEST_ENV_PATH.join(".git"))?;
    Ok(())
}
