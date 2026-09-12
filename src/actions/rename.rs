use std::{fs::rename, io::Write, path::{Path, PathBuf}};

use crate::{config::Config, fzf};
use anyhow::Result;

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

    if selected.is_some() {
        let book_dir = &config.directory.join(&config.current_book);

        match prompt_for_name(&book_dir)? {
            Some(new_path) => {
                rename_file(&selected.unwrap(), &new_path);
                return Ok(());
            },
            None => {
            println!("Rename cancelled: no new name given.");
                return Ok(());
            },
        }
    } else {
        println!("No file selected.");
    }

    Ok(())
}

fn prompt_for_name(directory: &PathBuf) -> Result<Option<PathBuf>> {
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
            return Ok(Some(path));
        }
    }
}

fn rename_file(old_path: &Path, new_path: &Path) -> Result<()> {
    rename(old_path, new_path)?;
    Ok(())
}
