use std::process::Command;
use anyhow::Result;

use crate::{config::Config, fzf};

pub fn execute(config: Config) -> Result<()> {
    let notes_dir = config.directory;
    let preview_reader = config.preview_reader;

    match fzf::select_file(notes_dir, preview_reader)? {
        Some(selected) => {
            print_with_cat(selected)?;
        }
        None => {
            println!("No file selected.")
        }
    }

    Ok(())
}

fn print_with_cat(path: String) -> Result<()> {
    let status = Command::new("cat")
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Cat exited with error");
    }

    Ok(())
}
