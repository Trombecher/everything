use std::path::PathBuf;

use clap::{Parser, Subcommand};
use everything_structures::{AbstractId, Object};

#[derive(Parser)]
#[command(version)]
#[command(name = "Everything CLI")]
#[command(about)]
#[command(long_about = None)]
#[command(arg_required_else_help(true))]
struct Args {
    /// A command to
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Generates and prints a new, unique abstract object id.
    Gen,
    /// Loads a structure file.
    Load {
        #[arg(id = "structure file path")]
        path: PathBuf,
    },
}

fn main() {
    let Args { command } = Args::parse();

    let command = command.unwrap();

    match command {
        Command::Gen => {
            println!("{:?}", Object::Abstract(AbstractId::new()))
        }
        Command::Load { path } => {
            println!("Loading {:?}", path)
        }
    }
}
