use crate::{config::Config, utils::confirm};
use anyhow::Result;

pub fn execute(config: &Config, name: &str) -> Result<()> {
    if name == config.current_book {
        anyhow::bail!("Cannot delete current book '{}'. Please move to another book first.", name);
    }

    let path = config.directory.join(name);

    if path.exists() {
        let message = format!("Are you sure you want to delete book '{}'?", name);
        let cancel_msg = "Deletion cancelled";

        if confirm(&message, &cancel_msg)? {
            std::fs::remove_dir(path)?;
            println!("Book deleted: {}", name);
        }
    } else {
        println!("No such book: {}", name);
    }

    Ok(())
}
