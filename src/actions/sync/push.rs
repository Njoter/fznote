use anyhow::Result;
use crate::{config::Config, git::repo, utils::printer};

pub fn execute(config: &Config, message: Option<String>) -> Result<()> {
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

        let msg = message.unwrap_or_else(|| {
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });

        repo::commit_all(path, &msg)?;
    }

    printer::header(&format!("Pushing to {}", url));

    repo::push_origin(path)?;
    Ok(())
}
