mod util;

use std::{
    fs::read_to_string,
    io::{Write, stdin, stdout},
    path::PathBuf,
    time::Instant,
};

use clap::{Parser, Subcommand};
use everything::{
    ctx::EvaluationContext,
    ext::{ObjectExt, StructureExt},
};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::Parsable;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;
use ulid::Ulid;

use crate::util::handle_parse_error;

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
    #[command(id = "open")]
    Open {
        #[arg(id = "structure file path")]
        path: PathBuf,
    },
}

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let Args { command } = Args::parse();

    let command = command.unwrap();

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
        Command::Open { path } => {
            let input = read_to_string(&path).expect("Reading from file failed");
            let structure =
                Structure::parse(&input).unwrap_or_else(|error| handle_parse_error(&input, &error));

            println!("Loaded {}. Type ? for help.", path.display());

            let mut line = String::new();

            loop {
                print!("\n> ");
                stdout().flush().unwrap();

                line.clear();
                stdin().read_line(&mut line).unwrap();

                let line = line.trim();

                let (command, arg) = line.split_once(" ").unwrap_or((line, ""));

                match command {
                    "exit" => break,
                    "?" => {
                        println!(
                            "exit - exits REPL\n? - prints this message\neval <EXPR> - evaluate this expression"
                        );
                    }
                    "eval" => {
                        // TODO: make this not hard error.
                        let expression = Object::parse(arg).unwrap();

                        let output = expression.eval(&structure, &mut EvaluationContext::default());

                        print!("{:?}", output);
                    }
                    _ => {
                        println!("Unknown command {command}");
                    }
                }
            }
        }
    }
}
