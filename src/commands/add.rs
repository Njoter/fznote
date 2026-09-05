use std::process::Command;

use crate::config::Config;
use anyhow::Result;

pub fn execute(config: Config, name: &str) -> Result<()> {
    let path = config.directory.join(name);

    if path.exists() {
        anyhow::bail!("Note '{}' already exists", name);
    }

    std::fs::File::create(&path)?;
    println!("Created file: {}", path.display());

    let status = Command::new(&config.editor)
        .arg(&path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with error");
    }

    Ok(())
}
