use anyhow::Result;
use crate::{config::Config, git::repo, utils::printer};


pub fn execute(config: &Config) -> Result<()> {
    let path = &config.directory;

    if !repo::is_repo(path) {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    }

    let Some(url) = repo::origin_url(path) else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    printer::header(&format!("Pulling from {}", url));

    repo::pull_origin(path)?;

    Ok(())
}
