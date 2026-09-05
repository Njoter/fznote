use std::{fs, path::PathBuf};
use anyhow::Result;

use crate::config::Dependencies;
mod config;
mod fzf;

fn main() -> Result<()> {
    let dependencies = Dependencies::check();
    dependencies.ensure_fzf()?;
    dependencies.warn_optional();

    let config = config::Config::load()?;
    let notes_directory = config.directory;
    let preview_reader = config.preview_reader;

    let filenames = get_filenames(&notes_directory)?;

    if let Some(selected) = fzf::select_file(&filenames, notes_directory, preview_reader)? {
        println!("Selected: {}", selected);
    };

    Ok(())
}

fn get_filenames(dir: &PathBuf) -> Result<Vec<String>> {
    let mut filenames = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    filenames.push(name.to_string());
                }
            }
        }
    }

    Ok(filenames)
}
