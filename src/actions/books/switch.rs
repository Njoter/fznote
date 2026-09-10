use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config, name: &str) -> Result<String> {
    // Check if the book exists
    let book_path = config.directory.join(name);
    if !book_path.exists() {
        anyhow::bail!("Book '{}' does not exist.", name);
    }

    Ok(name.to_string())
}
