use crate::{config::Config, utils::{confirm, filesystem}};
use anyhow::Result;

// TODO: This should obviously use fzf selection instead of taking the name from input
pub fn execute(config: &Config, name: &str) -> Result<()> {
    if name == config.current_book {
        anyhow::bail!("Cannot delete current book '{}'. Please move to another book first.", name);
    }

    let path = config.directory.join(name);
    if !path.exists() {
        println!("No such book: {}", name);
        return Ok(());
    }

    // Count the files (excluding the .gitkeep)
    let files = filesystem::get_files_with_hidden(&path)?;
    let file_count = files.iter()
        .filter(|s| s.as_str() != ".gitkeep")
        .count();

    // Count the directories. There shouldn't be any, unless the user added them manually
    let directories = filesystem::get_directories_with_hidden(&path)?;
    let dir_count = directories.len();

    println!("Book '{}' contains {} notes.", name, file_count);
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
        println!("Book deleted: {}", name);
    }

    Ok(())
}
