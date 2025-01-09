mod common;

use common::repo_git_dir;
use std::io;
use tig::TigManager;

#[test]
fn tig_init_folder_created() -> io::Result<()> {
    common::setup()?;

    let git_dir = &repo_git_dir();
    let tig = TigManager::new(git_dir);

    assert!(tig.is_ok());
    assert!(git_dir.join("x-tig-info").exists());

    common::cleanup()?;
    Ok(())
}
