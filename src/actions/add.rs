use std::path::Path;

use crate::{config::Config, utils::{filesystem, run_command}};
use anyhow::{Result, bail};

pub fn execute(config: &Config, name: &str, extension: Option<String>) -> Result<()> {
    if name.is_empty() {
        bail!("Name cannot be empty.");
    }

    let ext = extension
        .as_deref()
        .unwrap_or(&config.file_extension)
        .trim_start_matches(".");

    let filename = format!("{}.{}", name, ext);

    if !filesystem::is_valid_filename(&filename) {
        bail!("'{}' contains invalid characters.", filename);
    }

    let dir = config.directory.join(&config.current_book);
    let path = dir.join(&filename);

    if filesystem::exists_case_aware(&dir, &filename)? {
        bail!("Note '{}' already exists.", filename);
    }

    create_file(&path)?;
    run_command(&config.editor, &path)?;

    Ok(())
}

fn create_file(path: &Path) -> Result<()> {
    std::fs::File::create(path)?;
    println!("Created file: '{}'.", path.display());
    Ok(())
}
