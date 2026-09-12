use std::{fs, path::Path};

use crate::config::Config;
use anyhow::{Context, Result};

// TODO: Switch to new book when created
pub fn execute(config: &Config, name: &str) -> Result<()> {
    let path = config.directory.join(name);

    if path.exists() {
        anyhow::bail!("Book '{}' already exists", name);
    }

    create_book(&path)?;
    println!("Created book: {}", name);

    Ok(())
}

fn create_book(book_dir: &Path) -> Result<()> {
    let gitkeep = book_dir.join(".gitkeep");

    fs::create_dir_all(&book_dir)
        .with_context(|| format!("Failed to create book directory: {}", book_dir.display()))?;

    if let Err(e) = fs::write(&gitkeep, "") {
        let _ = fs::remove_dir_all(&book_dir);
        return Err(e).with_context(|| format!("Failed to create: {}", gitkeep.display()));
    }

    Ok(())
}
