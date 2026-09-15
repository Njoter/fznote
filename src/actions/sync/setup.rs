use anyhow::Result;
use crate::{config::Config, git::repo, utils::prompt};

pub fn execute(config: &Config, force: bool) -> Result<()> {
    let path = &config.directory;

    repo::init_if_not_repo(path)?;

    if let Some(url) = repo::origin_url(path) {
        if force {
            println!("Current remote: {}", url);
            println!();
        } else {
            println!("Remote is already set up at {}.", url);
            println!();
            println!("Use `fznote sync setup --force` to change it.");
            return Ok(());
        }
    }

    println!("Configuring git sync for {}.", path.display());
    println!();
    println!("Enter a remote URL to sync your notes with.");
    println!("(e.g. git@github.com:user/notes.git)");
    println!();

    let Some(url) = prompt::for_string("Remote URL: ")? else {
        println!("Sync setup cancelled: no URL provided.");
        return Ok(());
    };

    repo::set_remote(path, &url)?;

    println!("Sync remote set to {}.", url);
    println!();
    println!("Run `fznote sync push` to push your notes.");

    Ok(())
}
