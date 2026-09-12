use std::io::Write;

use anyhow::Result;
use crate::{config::Config, git::repo};

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

    let Some(url) = prompt_for_remote_url()? else {
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

// TODO: Write a default prompt in prompt.rs instead
fn prompt_for_remote_url() -> Result<Option<String>> {
    print!("Remote URL: ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    if input.is_empty() {
        return Ok(None);
    }
    
    Ok(Some(input))
}
