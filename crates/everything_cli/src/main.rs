mod names;
mod repl;
mod util;

use std::{
    fs::{OpenOptions, read_to_string},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    process::exit,
    time::Instant,
};

use clap::{CommandFactory, Parser, Subcommand};
use everything::{base::BASE, ext::StructureExt};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::{Parsable, Token, Tokenizer};
use std::fmt::Write as _;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;
use ulid::Ulid;

use crate::{names::NAMES, repl::Repl, util::handle_parse_error};

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
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// A command to
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Generates and prints a new, unique abstract object id.
    Gen {
        #[arg(id = "count", default_value_t = 1)]
        count: u32,
    },
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
    #[command(id = "tp")]
    TransformPretty {
        #[arg(id = "file path")]
        path: PathBuf,
    },
}

fn main() {
    let Args { command, debug } = Args::parse();

    if debug {
        tracing::subscriber::set_global_default(
            Registry::default().with(HierarchicalLayer::new(2)),
        )
        .unwrap();
    }

    let command = if let Some(command) = command {
        command
    } else {
        Args::command().print_long_help().unwrap();
        exit(0);
    };

    match command {
        Command::Gen { count } => {
            for _ in 0..count {
                println!("{:?}", Object::Abstract(Abstract::ulid()))
            }
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
            Repl::default().main_loop();
        }
        Command::Base => {
            println!("{:?}", &*BASE);
        }
        Command::TransformPretty { path } => {
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(path)
                .unwrap();

            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();

            let mut new_content = String::new();

            for token in Tokenizer::new(&file_content) {
                match token {
                    Token::Abstract(source)
                        if let Some(id) = source.parse()
                            && let Some((name, _)) = NAMES
                                .iter()
                                .find(|(_, object)| object == &Object::Abstract(Abstract(id))) =>
                    {
                        write!(new_content, "${name}").unwrap();
                    }
                    token => {
                        new_content.push_str(token.as_str());
                    }
                }
            }

            file.seek(SeekFrom::Start(0)).unwrap();
            file.write_all(new_content.as_bytes()).unwrap();
        }
    }
}
