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

    // TODO: Figure out this whole thing.
    // Currently, local can't have unpushed commits.
    // Can't pull if uncommitted changes.
    // If remote is ahead, push will fail, so it has to pull first.
    if repo::has_uncommitted_changes(path)? {
        println!("Status: You have uncommitted changes.");
        println!();
        println!("Run `fznote sync push` to push them.");
    } else {
        println!("Status: clean");
        println!();
        println!("Run `fznote sync pull` to fetch remote changes.");
    }

    let (ahead, behind) = repo::ahead_behind(path)?;

    match (ahead, behind) {
        (0, 0) => {
            println!("up to date with remote.");
        }
        (a, 0) => {
            println!("{} unpushed commit(s)", a);
            println!();
            println!("Run `fznote sync push` to push them.");
        }
        (0, b) => {
            println!("{} unpulled commit(s)", b);
            println!();
            println!("Run `fznote sync pull` to fetch them.");
        }
        (a, b) => {
            println!("{} ahead, {} behind", a, b);
            println!();
            println!("Run `fznote sync pull` first, then `fznote sync push`.");
        }
    }

    Ok(())
}
