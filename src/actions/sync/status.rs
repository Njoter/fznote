use anyhow::Result;
use crate::{config::Config, git::repo};

pub fn execute(config: &Config) -> Result<()> {
    let notes_dir = &config.directory;

    if !repo::is_repo(notes_dir) {
        println!("Not a git repo. Run 'fznote sync setup' to initialize.");
        return Ok(());
    }

    Ok(())
}
