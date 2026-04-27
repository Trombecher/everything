use std::{fs::read_to_string, path::PathBuf, process::exit, time::Instant};

use clap::{Parser, Subcommand};
use everything::ext::StructureExt;
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::{ErrorInfo, Parsable};
use ulid::Ulid;

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
}

fn handle_parse_error(input: &str, error: &ErrorInfo) -> ! {
    match &error.found {
        Some(found) => {
            let (start_line, start_col) = lc_from_index(input, found.range.start);
            let (end_line, end_col) = lc_from_index(input, found.range.end);

            eprintln!(
                "error while parsing at {}:{} (to {}:{}): found {:?}, expected {:?}",
                start_line + 1,
                start_col + 1,
                end_line + 1,
                end_col + 1,
                error.found,
                error.expected
            )
        }
        None => {
            eprintln!(
                "error while parsing at the end: expected {:?}",
                error.expected
            )
        }
    }

    exit(-1)
}

fn main() {
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
    }
}

fn lc_from_index(source: &str, index: u32) -> (u32, u32) {
    let slice = &source[..index as usize];

    let mut lines = 0;
    let mut cr = false;

    let mut chars = slice.chars();
    let mut last_line = slice;

    while let Some(c) = chars.next() {
        if c == '\r' {
            cr = true;

            lines += 1;
            last_line = chars.as_str();
        } else if c == '\n' {
            if !cr {
                lines += 1;
                last_line = chars.as_str();
            }

            cr = false;
        } else {
            cr = false;
        }
    }

    (lines, last_line.chars().count() as u32)
}
