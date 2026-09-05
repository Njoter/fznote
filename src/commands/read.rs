use std::process::Command;
use anyhow::Result;

use crate::{config::Config, fzf};

pub fn execute(config: Config) -> Result<()> {
    let notes_dir = config.directory;
    let preview_reader = config.preview_reader;
    let reader = config.reader;

    match fzf::select_file(notes_dir, preview_reader)? {
        Some(selected) => {
            print_with_reader(selected, reader)?;
        }
        None => {
            println!("No file selected.")
        }
    }

    Ok(())
}

fn print_with_reader(path: String, reader: String) -> Result<()> {
    let status = Command::new(reader)
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Cat exited with error");
    }

    Ok(())
}
