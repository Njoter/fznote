use crate::{config::Config, fzf, utils::run_command};
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_file(&config.directory, &config.preview_reader)? {
        Some(selected) => run_command("echo", &selected)?,
        None => println!("No file selected.")
    };

    Ok(())
}
