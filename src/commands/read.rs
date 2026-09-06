use std::{path::PathBuf, process::Command};
use anyhow::Result;

use crate::{config::Config, fzf};

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_file(&config.directory, &config.preview_reader)? {
        Some(selected) => {
            print_with_reader(&selected, &config.reader)?;
        }
        None => {
            println!("No file selected.")
        }
    }

    Ok(())
}

fn print_with_reader(path: &PathBuf, reader: &str) -> Result<()> {
    let status = Command::new(reader)
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Cat exited with error");
    }

    Ok(())
}
