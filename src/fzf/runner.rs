use std::{io::Write, path::PathBuf, process::{Command, Stdio}};
use anyhow::Result;
use crate::utils::filesystem;

pub fn select_file(dir: &PathBuf, book: &str, preview_reader: &str) -> Result<Option<PathBuf>> {
    let book_path = dir.join(book);
    let filenames = filesystem::get_files_not_hidden(&book_path)?;
    let input = filenames.join("\n");
    let preview_cmd = build_preview_cmd(preview_reader, &book_path);

    let mut child = Command::new("fzf")
        .arg("--layout=reverse")
        .arg("--preview")
        .arg(preview_cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
        stdin.flush()?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        let selected = String::from_utf8(output.stdout)?;
        let selected = selected.trim();
        if !selected.is_empty() {
            return Ok(Some(book_path.join(selected)));
        }
    }

    Ok(None)
}

pub fn select_from_content_search(dir: &PathBuf, book: &str, preview_reader: &str) -> Result<Option<PathBuf>> {
    let book_path = dir.join(book);

    // Build ripgrep command
    let mut rg_cmd = Command::new("rg");
    rg_cmd
        .current_dir(&book_path)
        .arg("--with-filename")
        .arg("--line-number")
        .arg("--field-match-separator=\t")
        .arg(".");

    let mut rg_child = rg_cmd
        .stdout(Stdio::piped())
        .spawn()?;

    // Build fzf preview command
    let preview_cmd = build_search_preview_cmd(preview_reader, &book_path);
    
    // Pipe rg output to fzf
    let fzf_child = Command::new("fzf")
        //.arg("--layout=reverse")
        .arg("--exact")
        .arg("--delimiter=\t")
        .arg("--with-nth={3}") // Show only content
        .arg("--preview")
        .arg(&preview_cmd)
        .stdin(Stdio::from(rg_child.stdout.take().unwrap()))
        .stdout(Stdio::piped())
        .spawn()?;

    let output = fzf_child.wait_with_output()?;
    
    if output.status.success() {
        let selected = String::from_utf8(output.stdout)?;
        let selected = selected.trim();
        if !selected.is_empty() {
            // Split by tab to get filename (first field)
            let parts: Vec<&str> = selected.split('\t').collect();
            if let Some(filename) = parts.first() {
                return Ok(Some(book_path.join(filename)));
            }
        }
    }

    Ok(None)
}

// TODO: Preview books contents with fd
pub fn select_book(dir: &PathBuf) -> Result<Option<String>> {
    let books = filesystem::get_directories_not_hidden(dir)?;

    if books.is_empty() {
        return Ok(None);
    }

    let input = books.join("\n");

    let mut child = Command::new("fzf")
        .arg("--layout=reverse")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
        stdin.flush()?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        let selected = String::from_utf8(output.stdout)?;
        let selected = selected.trim();
        if !selected.is_empty() {
            return Ok(Some(selected.to_string()));
        }
    }

    Ok(None)
}

fn build_preview_cmd(reader: &str, dir: &PathBuf) -> String {
    let dir_string = dir.display();

    match reader {
        "bat" => format!("bat --color=always \"{}\"/{{}}", dir_string),
        _ => format!("{} \"{}\"/{{}}", reader, dir_string)
    }
}

fn build_search_preview_cmd(reader: &str, dir: &PathBuf) -> String {
    let dir_string = dir.display();

    match reader {
        "bat" => format!("bat --color=always --highlight-line {{2}} \"{}\"/{{1}}", dir_string),
        _ => format!("{} \"{}\"/{{1}}", reader, dir_string),
    }
}
