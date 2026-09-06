use std::{path::PathBuf, process::Command};
use anyhow::Result;

use crate::{config::Config, fzf};

pub fn execute(config: &Config) -> Result<()> {
    match fzf::select_file(&config.directory, &config.preview_reader)? {
        Some(selected) => {
            print_with_cat(&selected)?;
        }
        None => {
            println!("No file selected.")
        }
    }

    Ok(())
}

fn print_with_cat(path: &PathBuf) -> Result<()> {
    let status = Command::new("cat")
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Cat exited with error");
    }

    Ok(())
}
