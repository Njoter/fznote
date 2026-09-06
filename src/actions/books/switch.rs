use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &mut Config, name: &str) -> Result<()> {
    // Check if the book exists
    let book_path = config.directory.join(name);
    if !book_path.exists() {
        anyhow::bail!("Book '{}' does not exist.", name);
    }

    // Update 'current_book' and save the config
    config.current_book = name.to_string();
    config.save()?;

    Ok(())
}
