use crate::{config::Config, utils::filesystem};
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    let books = filesystem::get_directories_not_hidden(&config.directory)?;

    if books.is_empty() {
        println!("📚 No books found.");
        println!("   Create one with: fznote book add <name>");
        return Ok(());
    }

    println!("Books");
    println!("-------------------------------------------------");
    for book in books {
        if book == config.current_book {
            println!("[x] | {}", book);
        } else {
            println!("    | {}", book);
        }
    }

    Ok(())
}
