//! # Temp Ignore Git (TIG)
//!
//! TIG is a wrapper around git that streamlines temporarily ignoring files changes when
//! running commands. It's particularly useful if a build script, outside of your control,
//! makes minor changes to a repository that does not need to be tracked.
//!
//! It is possible in vanilla git to ignore these changes, but can become cumbersome when
//! re-running scripts when relevant major changes have already been made but not yet
//! committed.

mod tig;

use std::{io, process::exit};
use tig::TigManager;

fn main() -> io::Result<()> {
    let repo_git_dir = match tig::git_validate_status() {
        Ok(path) => path,
        Err(e) => {
            fallback_log!(e);
            exit(1);
        }
    };

    let _tig = TigManager::new(&repo_git_dir)?;

    Ok(())
}
