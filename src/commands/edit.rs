use std::{path::Path, process::Command};

use anyhow::{Context, Result};
use crate::{config::Config, fzf, utils::run_command};

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_file(&config.directory, &config.preview_reader)? {
        Some(selected) => {
            run_command(&config.editor, &selected)?;
        }
        None => println!("No file selected."),
    }

    Ok(())
}
