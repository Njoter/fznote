use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Dependencies;

mod config;
mod commands;
mod fzf;

#[derive(Parser)]
#[command(name = "fznote")]
#[command(about = "A fuzzy note manager powered by fzf")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(Subcommand)]
enum Action {
    // Add a new note
    Add {
        name: String,
    }
}

fn main() -> Result<()> {
    // Parse CLI
    let cli = Cli::parse();

    // Check dependencies
    let deps = Dependencies::check();
    deps.ensure_fzf()?;
    deps.warn_optional();

    // Load config
    let config = config::Config::load()?;

    // Execute action or default to print
    match cli.action {
        Some(Action::Add { name }) => {
            commands::add(config, &name)?;
        }
        None => {
            commands::print(config)?;
        },
    }

    Ok(())
}
