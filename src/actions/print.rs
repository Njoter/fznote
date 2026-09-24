use std::io::Write;

use anyhow::Result;
use crate::{config::Config, fzf};

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_file(&config.directory, &config.current_book,  &config.preview_reader)? {
        Some(selected) => {
            let bytes = std::fs::read(&selected)?;
            std::io::stdout().write_all(&bytes)?;
        }
        None => println!("No file selected.")
    }

    Ok(())
}
