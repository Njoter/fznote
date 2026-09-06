use std::{path::Path, process::Command};
use anyhow::{Context, Result};

pub fn run_command(program: &str, path: &Path) -> Result<()> {
    let status = Command::new(program)
        .arg(path)
        .status()
        .with_context(|| format!("Failed to run {}", program))?;

    if !status.success() {
        anyhow::bail!("{} exited with error.", program);
    }

    Ok(())
}
