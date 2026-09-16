use anyhow::{Ok, Result};
use crate::{config::Config, fzf};

pub fn execute(mut config: Config) -> Result<()> {
    match fzf::select_book(&config.directory)? {
        Some(book) => {
            config.current_book = book.clone();
            println!("Switched to book: {}", &book);
            return Ok(());
        },
        None => {
            println!("No book selected.");
            return Ok(());
        }
    }
}
