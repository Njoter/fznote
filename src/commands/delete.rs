use std::path::Path;

use crate::{config::Config, fzf, utils::confirm};
use anyhow::Result;

pub fn execute(config: &Config, search: bool) -> Result<()> {
    if search {
        match fzf::select_from_content_search(&config.directory, &config.preview_reader)? {
            Some(selected) => delete_file(&selected)?,
            None => println!("No file selected.")
        }
    } else {
        match fzf::select_file(&config.directory, &config.preview_reader)? {
            Some(selected) => delete_file(&selected)?,
            None => println!("No file selected.")
        }
    }

    Ok(())
}

fn delete_file(path: &Path) -> Result<()> {
    let message = format!("Are you sure you want to delete: '{}'", path.display());
    let cancel_msg = "Deletion cancelled.";

    if confirm(&message, &cancel_msg)? {
        std::fs::remove_file(path)?;
        println!("File removed: {}", path.display());
    }
    Ok(())
}
