use anyhow::Result;

use crate::{config::Config, fzf, utils::run_command};

pub fn execute(config: &Config, search: bool) -> Result<()> {
    if search {
        match fzf::select_from_content_search(&config.directory, &config.current_book, &config.preview_reader)? {
            Some(selected) => run_command(&config.reader, &selected)?,
            None => println!("No file selected.")
        }
    } else {
        match fzf::select_file(&config.directory, &config.current_book, &config.preview_reader)? {
            Some(selected) => run_command(&config.reader, &selected)?,
            None => println!("No file selected.")
        }
    }

    Ok(())
}
