use std::path::PathBuf;

pub(super) fn notes_directory() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("fznote/notes")
}

pub(super) fn reader() -> String {
    if which::which("bat").is_ok() {
        "bat".to_string()
    } else {
        "cat".to_string()
    }
}

pub(super) fn editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| {
        if which::which("vim").is_ok() {
            "vim".to_string()
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
