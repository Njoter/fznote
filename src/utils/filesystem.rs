use std::{fs, path::Path};
use anyhow::{Result, bail};
use sanitize_filename::OptionsForCheck;

pub fn get_files_not_hidden(directory: &Path) -> Result<Vec<String>> {
    let mut filenames = Vec::new();
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if !name.starts_with('.') {
                    filenames.push(name.to_string());
                }
            }
        }
    }

    filenames.sort_by_key(|s| s.to_lowercase());
    Ok(filenames)
}

pub fn get_files_with_hidden(directory: &Path) -> Result<Vec<String>> {
    let mut filenames = Vec::new();
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                filenames.push(name.to_string());
            }
        }
    }

    filenames.sort_by_key(|s| s.to_lowercase());
    Ok(filenames)
}

pub fn get_directories_not_hidden(directory: &Path) -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if !name.starts_with('.') {
                    dirs.push(name.to_string());
                }
            }
        }
    }

    dirs.sort_by_key(|s| s.to_lowercase());
    Ok(dirs)
}

pub fn get_directories_with_hidden(directory: &Path) -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                dirs.push(name.to_string());
            }
        }
    }

    dirs.sort_by_key(|s| s.to_lowercase());
    Ok(dirs)
}

pub fn is_valid_filename(name: &str) -> bool {
    let options = OptionsForCheck {
        windows: true,
        truncate: true,
    };

    sanitize_filename::is_sanitized_with_options(name, options)
}

fn exists_case_insensitive(dir: &Path, filename: &str) -> Result<bool> {
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if name.eq_ignore_ascii_case(filename) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn exists_case_aware(directory: &Path, name: &str) -> Result<bool> {
    if cfg!(windows) {
        Ok(directory.join(name).exists())
    } else {
        exists_case_insensitive(directory, name)
    }
}

pub fn validate_book_name(directory: &Path, name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("Book name cannot be empty.");
    }
    if name.contains(['/', '\\']) {
        bail!("Book names can't contain '/' or '\\'.");
    }
    if !is_valid_filename(name) {
        bail!("'{}' contains invalid characters.", name);
    }
    if name.starts_with('.') {
        bail!("Book names can't start with '.'.");
    }
    if !directory.join(name).starts_with(directory) {
        bail!("Book name escapes the notes directory: {}.", name);
    }

    Ok(())
}
