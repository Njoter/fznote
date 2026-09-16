use crate::{config::Config, fzf::{self}, utils::{confirm, filesystem}};
use anyhow::{Result, bail};

// TODO: This should obviously use fzf selection instead of taking the name from input
pub fn execute(config: &Config) -> Result<()> {
    let selected = match fzf::select_book(&config.directory)? {
        Some(book) => Some(book),
        None => {
            println!("No book selected.");
            return Ok(())
        }
    };

    let book = selected.unwrap();

    if book == config.current_book {
        anyhow::bail!("Cannot delete current book '{}'. Please move to another book first.", book);
    }

    let path = config.directory.join(&book);
    if !path.exists() {
        bail!("No such book: {}", book);
    }

    // Count the files (excluding the .gitkeep)
    let files = filesystem::get_files_with_hidden(&path)?;
    let file_count = files.iter()
        .filter(|s| s.as_str() != ".gitkeep")
        .count();

    // Count the directories. There shouldn't be any, unless the user added them manually
    let directories = filesystem::get_directories_with_hidden(&path)?;
    let dir_count = directories.len();

    println!();
    println!("Book '{}' contains {} notes.", book, file_count);
    if dir_count == 1 {
        println!("The book contains 1 directory, for some reason.");
    } else if dir_count > 1 {
        println!("The book contains {} directories, for some reason.", dir_count);
    }
    println!();

    let message = if file_count == 0 && dir_count == 0 {
        "Delete the book?"
    } else {
        "Delete the book and all its contents?"
    };

    if confirm(&message, "Deletion cancelled.")? {
        std::fs::remove_dir_all(&path)?;
        println!("Book deleted: {}", book);
    }

    Ok(())
}
