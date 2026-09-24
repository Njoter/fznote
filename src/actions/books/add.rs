use std::{fs, path::Path};

use crate::{config::Config, utils::filesystem};
use anyhow::{Context, Result, bail};

pub fn execute(config: &Config, name: &str) -> Result<String> {
    filesystem::validate_book_name(&config.directory, name)?;

    let path = config.directory.join(name);

    if filesystem::exists_case_aware(&config.directory, name)? {
        bail!("Book '{}' already exists.", name);
    }

    create_book(&path)?;
    println!("Created book: {}.", name);

    Ok(name.to_owned())
}

fn create_book(book_dir: &Path) -> Result<()> {
    // Create a .gitkeep in the new directory so that it gets added in git
    let gitkeep = book_dir.join(".gitkeep");

    fs::create_dir(&book_dir)
        .with_context(|| format!("Failed to create book directory: {}.", book_dir.display()))?;

    if let Err(e) = fs::write(&gitkeep, "") {
        let _ = fs::remove_dir_all(&book_dir);
        return Err(e).with_context(|| format!("Failed to create: {}.", gitkeep.display()));
    }

    Ok(())
}
