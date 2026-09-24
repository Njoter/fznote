use std::path::Path;
use crate::{config::Config, fzf, utils::{filesystem, prompt}};
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

    std::fs::rename(&old_path, &new_path)?;
    println!("Book renamed: {} -> {}", book, name);

    Ok(Some((book, name)))
}

fn prompt_for_name(directory: &Path) -> Result<Option<String>> {
    loop {
        let Some(name) = prompt::for_string("New name: ")? else {
            return Ok(None);
        };

        if let Err(e) = filesystem::validate_book_name(directory, &name) {
            println!("{}", e);
            continue;
        }

        if filesystem::exists_case_aware(directory, &name)? {
            println!("A book named '{}' already exists.", name);
            continue;
        }

        return Ok(Some(name));
    }
}
