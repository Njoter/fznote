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

    if repo::has_uncommitted_changes(&repository)? {
        let msg = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        repo::commit_all(&repository, &msg)?;
        println!("Committed: {}", msg);
    } else {
        println!("No changes to commit.");
    }

    repo::push_origin(&repository)?;
    println!("Pushed to {}.", url);

    Ok(())
}
