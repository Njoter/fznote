use anyhow::Result;
use crate::{config::Config, git::repo, utils::printer};

pub fn execute(config: &Config) -> Result<()> {
    let path = &config.directory;

    if !repo::is_repo(path) {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    }

    let Some(url) = repo::origin_url(path) else {
        println!("Sync not set up. Run `fznote sync setup`.");
        return Ok(());
    };

    printer::header(&format!("Sync remote set up at {}", url));

    let is_clean = !repo::has_uncommitted_changes(path)?;

    repo::fetch(path)?;

    let (ahead, behind) = match repo::ahead_behind(path) {
        Ok(counts) => counts,
        Err(_) => {
            println!("Status: no remote data yet.");
            println!();
            println!("Run `fznote sync` to sync your notes.");
            return Ok(());
        }
    };

    let is_up_to_date = ahead == 0 && behind == 0;

    let local = if is_clean { "clean" } else { "uncommitted changes" };

    let remote = match (ahead, behind) {
        (0, 0) => "up to date".to_string(),
        (a, 0) => format!("{} commits ahead", a),
        (0, b) => format!("{} commits behind", b),
        (a, b) => format!("{} ahead, {} behind", a, b),
    };

    println!("Status: {}, {}.", local, remote);

    if !(is_clean && is_up_to_date) {
        println!();
        println!("Run `fznote sync` to sync your notes.");
    }

    Ok(())
}
