use std::path::Path;

use crate::{config::Config, utils::run_command};
use anyhow::Result;

pub fn execute(config: &Config, name: &str, extension: Option<String>) -> Result<()> {
    let ext = extension
        .as_deref()
        .unwrap_or(&config.file_extension)
        .trim_start_matches(".");

    let filename = format!("{}.{}", name, ext);

    let path = config.directory
        .join(&config.current_book)
        .join(&filename);

    if path.exists() {
        anyhow::bail!("Note '{}' already exists", filename);
    }

    create_file(&path)?;
    run_command(&config.editor, &path)?;

    Ok(())
}

fn create_file(path: &Path) -> Result<()> {
    std::fs::File::create(path)?;
    println!("Created file: '{}'", path.display());
    Ok(())
}
