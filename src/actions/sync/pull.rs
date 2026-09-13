use anyhow::Result;
use crate::{config::Config, git::repo};


pub fn execute(config: &Config) -> Result<()> {
    let Some(repository) = repo::open_if_repo(&config.directory) else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    let Some(url) = repo::origin_url(&repository) else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    println!("Pulling from {} ...", url);
    repo::pull_origin(&repository)?;
    println!("Done. Your notes are up to date.");

    Ok(())
}
