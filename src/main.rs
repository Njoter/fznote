use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Dependencies;

mod config;
mod commands;
mod fzf;
mod utils;

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

        // Fle extension override
        #[arg(short = 'x', long)]
        extension: Option<String>,
    },
    // Select a file to read with configured reader
    Read,
    Delete,
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
        Some(Action::Add { name, extension }) => {
            commands::add(&config, &name, extension)?;
        }
        Some(Action::Read) => commands::read(&config)?,
        Some(Action::Delete) => commands::delete(&config)?,
        None => commands::print(&config)?,
    }

    Ok(())
}
