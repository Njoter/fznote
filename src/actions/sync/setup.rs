use anyhow::Result;
use crate::{config::Config, git::repo, utils::prompt};

pub fn execute(config: &Config, force: bool) -> Result<()> {
    let repository = repo::open_or_init(&config.directory)?;

    if !force {
        // Check if repo already has a remote with a URL
        if let Ok(remote) = repository.find_remote("origin") {
            if let Ok(url) = remote.url() {
                println!("Remote is already set up at {}.", url);
                println!();
                println!("Use `fznote sync setup --force` to change it.");
                return Ok(());
            }
        }
    }

    println!("Configuring git sync for {}.", config.directory.display());
    println!("Enter a remote URL to sync your notes with.");
    println!("(e.g. git@github.com:user/notes.git)");
    println!();

    let Some(url) = prompt::for_string("Remote URL: ")? else {
        println!("Sync setup cancelled: no URL provided.");
        return Ok(());
    };

    let _ = repository.remote_delete("origin");
    repository.remote("origin", &url)?;

    println!("Sync remote set to {}.", url);
    println!();
    println!("Run `fznote sync push` to push your notes.");

    Ok(())
}
