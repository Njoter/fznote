use anyhow::Result;
use crate::{config::Config, fzf, utils::run_command};

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_from_content_search(&config.directory, &config.preview_reader)? {
        Some(selected) => run_command(&config.reader, &selected)?,
        None => println!("No file selected.")
    }

    Ok(())
}
