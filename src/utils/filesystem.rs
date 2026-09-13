use std::{fs, path::Path};
use anyhow::Result;

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

    filenames.sort();
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

    filenames.sort();
    Ok(filenames)
}

pub fn get_directories_not_hidden(directory: &Path) -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if !name.starts_with(".") {
                    dirs.push(name.to_string());
                }
            }
        }
    }

    dirs.sort();
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

    dirs.sort();
    Ok(dirs)
}
