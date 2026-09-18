use std::{path::Path, process::Command};
use anyhow::{Ok, Result, bail, anyhow};

pub fn is_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

pub fn init_if_not_repo(path: &Path) -> Result<()> {
    if is_repo(path) {
        return Ok(());
    }

    let status = Command::new("git")
        .arg("init")
        .current_dir(path)
        .status()?;

    if !status.success() {
        bail!("Git init failed.")
    }

    Ok(())
}

pub fn origin_url(path: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(path)
        .output().ok()?;

    if !output.status.success() {
        return None;
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if url.is_empty() {
        return None;
    }

    Some(url)
}

pub fn set_remote(path: &Path, url: &str) -> Result<()> {
    // Remove the existing origin if any
    let _ = Command::new("git")
        .args(["remote", "remove", "origin"])
        .current_dir(path)
        .output();

    let status = Command::new("git")
        .args(["remote", "add", "origin", url])
        .current_dir(path)
        .status()?;

    if !status.success() {
        bail!("Failed to set remote URL.");
    }

    Ok(())
}

pub fn has_uncommitted_changes(path: &Path)  -> Result<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        bail!("Git status failed.")
    }

    Ok(!output.stdout.is_empty())
}

pub fn ahead_behind(path: &Path) -> Result<(u32, u32)> {
    let output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", "HEAD...origin/HEAD"])
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git rev-list failed: {}", stderr.trim());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut parts = stdout.split_whitespace();

    let ahead: u32 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| anyhow!("Unexpected git rev-list output: {:?}", stdout.trim()))?;

    let behind: u32 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| anyhow!("Unexpected git rev-list output: {:?}", stdout.trim()))?;

    Ok((ahead, behind))
}

pub fn commit_all(path: &Path, message: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .status()?;
    if !status.success() {
        bail!("Git add failed.")
    }

    let status = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(path)
        .status()?;
    if !status.success() {
        bail!("Git commit failed.")
    }

    Ok(())
}

pub fn push_origin(path: &Path) -> Result<()> {
    let status = Command::new("git")
        .args(["push", "-u", "origin", "HEAD"])
        .current_dir(path)
        .status()?;

    if !status.success() {
        bail!("Git push failed.")
    }

    Ok(())
}

pub fn pull_origin(path: &Path) -> Result<()> {
    let status = Command::new("git")
        .args(["pull", "--rebase", "origin", "HEAD"])
        .current_dir(path)
        .status()?;

    if !status.success() {
        bail!("Git pull failed.");
    }

    Ok(())
}
