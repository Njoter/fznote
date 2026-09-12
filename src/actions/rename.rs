use std::{fs::rename, io::Write, path::{Path, PathBuf}};

use crate::{config::Config, fzf};
use anyhow::Result;

// TODO: Figure out what to do about file extensions
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

    match selected {
        Some(note_path) => {
            let current_book_dir = &config.directory.join(&config.current_book);

            match prompt_for_name(&current_book_dir)? {
                Some(new_name) => {
                    let new_path = current_book_dir.join(&new_name);
                    rename_file(&note_path, &new_path)?;

                    let old_name = match note_path.file_name() {
                        Some(name) => name.to_string_lossy().into_owned(),
                        None => note_path.to_string_lossy().into_owned(),
                    };

                    println!("Note renamed: {} -> {}", old_name, new_name);
                },
                None => println!("Rename cancelled: no new name given."),
            };
        },
        None => println!("No file selected.")
    };

    Ok(())
}

fn prompt_for_name(directory: &PathBuf) -> Result<Option<String>> {
    loop {
        print!("New name: ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            return Ok(None);
        }

        let path = directory.join(input);

        if path.exists() {
            println!("File already exists: {}", path.display());
        } else {
            return Ok(Some(input.to_owned()));
        }
    }
}

fn rename_file(old_path: &Path, new_path: &Path) -> Result<()> {
    rename(old_path, new_path)?;
    Ok(())
}
