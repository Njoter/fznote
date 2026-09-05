use std::{io::Write, path::PathBuf, process::{Command, Stdio}};
use anyhow::Result;

pub fn select_file(
    filenames: &[String],
    directory: PathBuf,
    preview_reader: String
) -> Result<Option<String>> {
    let input = filenames.join("\n");

    let preview_arg = format!("{} --color=always {}/{{}}", preview_reader, directory.display());

    let mut child = Command::new("fzf")
        .arg("--layout=reverse")
        .arg("--preview")
        .arg(preview_arg)
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
