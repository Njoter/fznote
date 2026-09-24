use std::path::PathBuf;

pub(super) fn notes_directory() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("fznote")
        .join("notes")
}

pub(super) fn reader() -> String {
    "bat".to_string()
}

pub(super) fn editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| {
        if which::which("nvim").is_ok() {
            "nvim".to_string()
        } else if which::which("vim").is_ok() {
            "vim".to_string()
        } else if cfg!(windows) {
            "edit".to_string()
        } else {
            "nano".to_string()
        }
    })
}

pub(super) fn file_extension() -> String {
    "md".to_string()
}

pub(super) fn current_book() -> String {
    "My Book".to_string()
}
