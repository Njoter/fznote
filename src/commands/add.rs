use std::process::Command;

use crate::config::Config;
use anyhow::Result;

pub fn execute(
    config: &Config,
    name: &str,
    extension: Option<String>
) -> Result<()> {
    let ext = extension.unwrap_or_else(|| config.file_extension.clone());
    let ext = ext.trim_start_matches('.').to_string();

    let filename = format!("{}.{}", name, ext);
    let path = config.directory.join(&filename);

    if path.exists() {
        anyhow::bail!("Note '{}' already exists", filename);
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
