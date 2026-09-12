use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::{Config, Dependencies};

mod config;
mod actions;
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
    Add {
        name: String,

        #[arg(short = 'x', long)]
        extension: Option<String>,
    },
    Read {
        #[arg(short = 's', long)]
        search: bool,
    },
    Delete {
        #[arg(short = 's', long)]
        search: bool,
    },
    Edit {
        #[arg(short = 's', long)]
        search: bool,
    },
    Path {
        #[arg(short = 's', long)]
        search: bool,
    },
    Rename {
        #[arg(short = 's', long)]
        search: bool,
    }
    Books {
        #[command(subcommand)]
        action: Option<BookAction>,
    },
}

#[derive(Subcommand)]
enum BookAction {
    New {
        name: String,
    },
    Delete {
        name: String,
    },
    Switch {
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
    let mut config = Config::load()?;

    // Execute action or default to print
    match cli.action {
        Some(Action::Add    { name, extension }) => actions::add(&config, &name, extension)?,
        Some(Action::Read   { search }) => actions::read(&config, search)?,
        Some(Action::Delete { search }) => actions::delete(&config, search)?,
        Some(Action::Edit   { search }) => actions::edit(&config, search)?,
        Some(Action::Path   { search }) => actions::path(&config, search)?,
        Some(Action::Rename { search }) => {
            match actions::rename(&config, search)? {
                Some(path) => {

                },
                None => println!("No file selected.")
            }
        }
        Some(Action::Books  { action }) => {
            match action {
                Some(BookAction::New    { name }) => actions::books::new(&config, &name)?,
                Some(BookAction::Delete { name }) => actions::books::delete(&config, &name)?,
                Some(BookAction::Switch { name }) => {
                    let book_name = actions::books::switch(&config, &name)?;
                    config.current_book = book_name;
                    config.save()?;
                },

                None => actions::books::list(&config)?,
            }
        }

        None => actions::print(&config)?,
    }

    Ok(())
}
