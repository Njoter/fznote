use anyhow::Result;
use crate::{config::Config, fzf, utils::run_command};

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_file(&config.directory, &config.current_book,  &config.preview_reader)? {
        Some(selected) => run_command("cat", &selected)?,
        None => println!("No file selected.")
    }

    Ok(())
}
