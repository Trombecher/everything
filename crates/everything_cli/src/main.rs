mod repl;
mod util;

use std::{fs::read_to_string, path::PathBuf, process::exit, time::Instant};

use clap::{CommandFactory, Parser, Subcommand};
use everything::{base::BASE, ext::StructureExt};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::Parsable;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;
use ulid::Ulid;

use crate::{repl::repl_main, util::handle_parse_error};

trait AbstractUlidExt {
    fn ulid() -> Self;
}

impl AbstractUlidExt for Abstract {
    fn ulid() -> Self {
        Self(Ulid::new().0)
    }
}

#[derive(Parser)]
#[command(version)]
#[command(name = "Everything CLI")]
#[command(about)]
#[command(long_about = None)]
struct Args {
    /// A command to
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Generates and prints a new, unique abstract object id.
    Gen,
    /// Parses the structure file and prints the structure.
    #[command(id = "pp")]
    ParseAndPrint {
        #[arg(id = "structure file path")]
        path: PathBuf,
        /// Minify. Default `false`.
        #[arg(short = 'm', long = "minify")]
        minify: bool,
    },
    #[command(id = "validate")]
    ValidateKnowledge {
        #[arg(id = "structure file path")]
        path: PathBuf,
    },
    #[command(id = "repl")]
    Repl,
    /// Prints the base knowledge required.
    #[command(id = "base")]
    Base,
}

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let command = if let Some(command) = Args::parse().command {
        command
    } else {
        Args::command().print_long_help().unwrap();
        exit(0);
    };

    match command {
        Command::Gen => {
            println!("{:?}", Object::Abstract(Abstract::ulid()))
        }
        Command::ParseAndPrint { path, minify } => {
            let input = read_to_string(path).expect("Reading from file failed");

            let now = Instant::now();
            let result = Object::parse(&input);

            println!("time parsing: {:?}", now.elapsed());

            let object = result.unwrap_or_else(|error| handle_parse_error(&input, &error));

            if minify {
                println!("{:?}", object);
            } else {
                println!("{:#?}", object);
            }
        }
        Command::ValidateKnowledge { path } => {
            let input = read_to_string(path).expect("Reading from file failed");
            let structure =
                Structure::parse(&input).unwrap_or_else(|error| handle_parse_error(&input, &error));

            if structure.is_knowledge().is_ok() {
                println!("Structure is knowledge")
            } else {
                eprintln!("Structure is not knowledge")
            }
        }
        Command::Repl {} => {
            repl_main();
        }
        Command::Base => {
            println!("{:?}", &*BASE);
        }
    }
}
