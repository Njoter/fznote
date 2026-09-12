use anyhow::Result;
use crate::{config::Config, git::repo::{self}};

pub fn execute(config: &Config) -> Result<()> {
    let Some(repository) = repo::open_if_repo(&config.directory) else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    let Some(url) = repo::origin_url(&repository) else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    println!("Sync remote set up at {}.", url);
    println!();
    println!("Run `fznote sync push` to push local changes.");
    println!("Run `fznote sync --help` to see all sync commands.");
    println!();

    let changed_paths = repo::changed_paths(&repository)?;
    if changed_paths.is_empty() {
        println!("No changes to commit.");
        return Ok(());
    }

    println!("Changes to commit:");
    println!();
    for entry in changed_paths {
        println!("  {}", entry);
    }

    Ok(())
}
