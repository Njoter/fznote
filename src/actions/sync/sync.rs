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

    if repo::has_uncommitted_changes(path)? {
        printer::header("Committing your changes");
        let message = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        repo::commit_all(path, &message)?;
    }

    repo::fetch(path)?;
    let (mut ahead, behind) = repo::ahead_behind(path)?;

    if ahead == 0 && behind == 0 {
        println!("Already up to date.");
        return Ok(());
    }

    if behind > 0 {
        printer::header(&format!("Pulling from {}", url));
        repo::pull_origin(path)?;
        (ahead, _) = repo::ahead_behind(path)?;
    }

    if ahead > 0 {
        printer::header(&format!("Pushing to {}", url));
        repo::push_origin(path)?;
    }

    Ok(())
}
