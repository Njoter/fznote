use anyhow::Result;
use crate::{config::Config, git::repo};

pub fn execute(config: &Config) -> Result<()> {
    let url = repo::open_if_repo(&config.directory)
        .and_then(|r| repo::origin_url(&r));

    let Some(url) = url else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    println!("Syncing with {}.", url);
    println!();
    println!("Run `fznote sync push` to push local changes.");
    println!("Run `fznote sync --help` to see all sync commands.");

    Ok(())
}
