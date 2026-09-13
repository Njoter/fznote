use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::{Config, Dependencies};

mod config;
mod actions;
mod fzf;
mod utils;
mod git;

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
    },
    Books {
        #[command(subcommand)]
        action: Option<BookAction>,
    },
    Sync {
        #[command(subcommand)]
        action: Option<SyncAction>,
    }
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

#[derive(Subcommand)]
enum SyncAction {
    Setup {
        #[arg(short = 'f', long)]
        force: bool,
    },
    Push {
        // TODO
    },
    Pull {
        // TODO
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
        Some(Action::Rename { search }) => actions::rename(&config, search)?,
        Some(Action::Books  { action }) => {
            match action {
                Some(BookAction::New    { name }) => actions::books::new(&config, &name)?,
                Some(BookAction::Delete { name }) => actions::books::delete(&config, &name)?,
                Some(BookAction::Switch { name }) => {
                    let book_name = actions::books::switch(&config, &name)?;
                    config.current_book = book_name.clone();
                    config.save()?;
                    println!("Switched to book: {}", book_name);
                },

                None => actions::books::list(&config)?,
            }
        },
        Some(Action::Sync { action }) => {
            match action {
                Some(SyncAction::Setup  { force }) => actions::sync::setup(&config, force)?,
                Some(SyncAction::Push   {  }) => actions::sync::push(&config)?,
                Some(SyncAction::Pull   {  }) => actions::sync::pull(&config)?,
                None => actions::sync::status(&config)?,
            }
        }

        None => actions::print(&config)?,
    }

    Ok(())
}
