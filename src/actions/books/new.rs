use std::path::Path;

use crate::config::Config;
use anyhow::Result;

pub fn execute(config: &Config, name: &str) -> Result<()> {
    let path = config.directory.join(name);

    if path.exists() {
        anyhow::bail!("Book '{}' already exists", name);
    }

    create_book(&path)?;

    Ok(())
}

fn create_book(path: &Path) -> Result<()> {
    std::fs::create_dir(path)?;
    println!("Created book: '{}'", path.display());
    Ok(())
}
