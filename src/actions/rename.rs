use std::path::Path;
use std::fs::rename;
use anyhow::Result;

use crate::{config::Config, fzf, utils::{filesystem, prompt}};

pub fn execute(config: &Config, search: bool) -> Result<()> {
    let selected = if search {
        fzf::select_from_content_search(
            &config.directory,
            &config.current_book,
            &config.preview_reader,
        )?
    } else {
        fzf::select_file(
            &config.directory,
            &config.current_book,
            &config.preview_reader,
        )?
    };

    let Some(note_path) = selected else {
        println!("No file selected.");
        return Ok(());
    };

    let current_book_dir = config.directory.join(&config.current_book);

    let Some(new_name) = prompt_for_name(&current_book_dir, &config.file_extension)? else {
        println!("Rename cancelled: no new name provided.");
        return Ok(());
    };

    let new_path = current_book_dir.join(&new_name);
    rename_file(&note_path, &new_path)?;

    let old_name = note_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| note_path.to_string_lossy().into_owned());

    println!("Note renamed: {} -> {}", old_name, new_name);

    Ok(())
}

fn prompt_for_name(directory: &Path, ext: &str) -> Result<Option<String>> {
    loop {
        let Some(name) = prompt::for_string("New name: ")? else {
            return Ok(None);
        };

        let filename = format!("{}.{}", name, ext);

        if !filesystem::is_valid_filename(&filename) {
            println!("'{}' contains invalid characters.", filename);
            continue;
        }

        if filesystem::exists_case_aware(directory, &filename)? {
            println!("A note named '{}' already exists.", filename);
            continue;
        }

        return Ok(Some(filename));
    }
}

fn rename_file(old_path: &Path, new_path: &Path) -> Result<()> {
    rename(old_path, new_path)?;
    Ok(())
}
