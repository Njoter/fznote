use anyhow::Result;
use crate::{config::Config, fzf};

pub fn execute(config: &Config) -> Result<Option<String>> {
    match fzf::select_book(&config.directory)? {
        Some(name) => Ok(Some(name)),
        None => Ok(None),
    }
}
