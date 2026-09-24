use anyhow::Result;
use crate::{config::Config, fzf};

pub fn execute(config: &Config) -> Result<Option<String>> {
    fzf::select_book(&config.directory)
}
