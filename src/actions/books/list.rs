use std::{fs, path::PathBuf};

use crate::config::Config;
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    let books = get_books(&config.directory)?;

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

fn get_books(dir: &PathBuf) -> Result<Vec<String>> {
    let mut books = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    books.push(name.to_string());
                }
            }
        }
    }

    books.sort();
    Ok(books)
}
