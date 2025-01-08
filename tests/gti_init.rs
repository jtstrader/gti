mod common;

use common::repo_git_dir;
use gti::GtiManager;
use std::io;

#[test]
fn gti_init_folder_created() -> io::Result<()> {
    common::setup()?;

    let git_dir = &repo_git_dir();
    let gti = GtiManager::new(git_dir);

    assert!(gti.is_ok());
    assert!(git_dir.join("x-gti-info").exists());

    common::cleanup()?;
    Ok(())
}
