use std::{fs, path::Path};

use crate::config::Config;
use anyhow::{Context, Result, bail};

pub fn execute(config: &Config, name: &str) -> Result<String> {
    if name.contains(std::path::MAIN_SEPARATOR) {
        bail!("Book names can't contain '{}'.", std::path::MAIN_SEPARATOR);
    }

    if name.starts_with('.') {
        bail!("Book names can't start with '.'.");
    }

    let path = config.directory.join(name);
    if !path.starts_with(&config.directory) {
        bail!("Book name escapes the notes directory: {}.", name);
    }

    if path.exists() {
        anyhow::bail!("Book '{}' already exists.", name);
    }

    create_book(&path)?;
    println!("Created book: {}.", name);

    Ok(name.to_owned())
}

fn create_book(book_dir: &Path) -> Result<()> {
    // Create a .gitkeep in the new directory so that it gets added in git
    let gitkeep = book_dir.join(".gitkeep");

    fs::create_dir(&book_dir)
        .with_context(|| format!("Failed to create book directory: {}", book_dir.display()))?;

    if let Err(e) = fs::write(&gitkeep, "") {
        let _ = fs::remove_dir_all(&book_dir);
        return Err(e).with_context(|| format!("Failed to create: {}", gitkeep.display()));
    }

    Ok(())
}
