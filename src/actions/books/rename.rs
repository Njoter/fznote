use std::path::Path;
use crate::{config::Config, fzf, utils::prompt};
use anyhow::Result;

pub fn execute(config: &Config) -> Result<Option<(String, String)>> {
    let Some(book) = fzf::select_book(&config.directory)? else {
        println!("No book selected.");
        return Ok(None);
    };

    let Some(name) = prompt_for_name(&config.directory)? else {
        println!("Rename cancelled: no new name provided.");
        return Ok(None);
    };

    let old_path = config.directory.join(&book);
    let new_path = config.directory.join(&name);

    rename_directory(&old_path, &new_path)?;
    println!("Book renamed: {} -> {}", book, name);

    Ok(Some((book, name)))
}

fn prompt_for_name(directory: &Path) -> Result<Option<String>> {
    loop {
        let Some(name) = prompt::for_string("New name: ")? else {
            return Ok(None);
        };

        let path = directory.join(&name);
        if path.exists() {
            println!("Directory already exists: {}", path.display());
            continue;
        }

        return Ok(Some(name));
    }
}

fn rename_directory(old_path: &Path, new_path: &Path) -> Result<()> {
    std::fs::rename(old_path, new_path)?;
    Ok(())
}
