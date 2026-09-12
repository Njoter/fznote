use anyhow::Result;
use crate::{config::Config, git::repo};

pub fn execute(config: &Config) -> Result<()> {
    let notes_dir = &config.directory;

    if repo::is_repo(notes_dir) {
        println!("{} is a git repo.", notes_dir.display());
    } else {
        println!("{} is not a git repository. Use fznote sync setup to setup the repository.", notes_dir.display());
    }

    Ok(())
}
