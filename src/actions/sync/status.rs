use anyhow::Result;
use crate::{config::Config, git::repo::{self}, utils::printer};

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

    printer::header(&format!("Sync remote set up at {}", url));

    if repo::has_uncommitted_changes(path)? {
        println!("Status: uncommitted changes");
        println!();
        println!("Run `fznote sync push` to push them.");
    } else {
        println!("Status: clean");
        println!();
        println!("Run `fznote sync pull` to fetch remote changes.");
    }

    Ok(())
}
