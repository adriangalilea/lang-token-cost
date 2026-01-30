mod commands;
mod error;
mod models;
mod storage;

use clap::{Parser, Subcommand};
use models::Priority;

#[derive(Parser)]
#[command(name = "todo", version, about = "A CLI todo manager")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new todo
    Add {
        text: String,
        #[arg(short, long, default_value = "medium")]
        priority: Priority,
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    /// List todos
    List {
        #[arg(short, long, help = "Show completed")]
        all: bool,
        #[arg(short, long)]
        tag: Option<String>,
        #[arg(short, long)]
        priority: Option<Priority>,
    },
    /// Mark a todo as done
    Done { id: u64 },
    /// Remove a todo
    Remove { id: u64 },
    /// Edit a todo
    Edit {
        id: u64,
        #[arg(long)]
        text: Option<String>,
        #[arg(short, long)]
        priority: Option<Priority>,
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// Show statistics
    Stats,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Add {
            text,
            priority,
            tags,
        } => commands::add(text, priority, tags),
        Command::List { all, tag, priority } => {
            commands::list_todos(all, tag.as_deref(), priority)
        }
        Command::Done { id } => commands::done(id),
        Command::Remove { id } => commands::remove(id),
        Command::Edit {
            id,
            text,
            priority,
            tags,
        } => commands::edit(id, text, priority, tags),
        Command::Stats => commands::stats(),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
