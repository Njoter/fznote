use std::{fs, io::Write, path::PathBuf, process::{Command, Stdio}};
use anyhow::Result;

pub fn select_file(dir: PathBuf, reader: String) -> Result<Option<String>> {
    let filenames = get_filenames(&dir)?;
    let input = filenames.join("\n");

    let preview_args = format!("{} {}/{{}}", reader, dir.display());

    let mut child = Command::new("fzf")
        .arg("--layout=reverse")
        .arg("--preview")
        .arg(preview_args)
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
            return Ok(Some(format!("{}/{}", dir.display(), selected)));
        }
    }

    Ok(None)
}

fn get_filenames(dir: &PathBuf) -> Result<Vec<String>> {
    let mut filenames = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    filenames.push(name.to_string());
                }
            }
        }
    }

    Ok(filenames)
}
