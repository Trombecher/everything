#![feature(str_from_raw_parts)]
#![no_std]

extern crate alloc;

mod filtered_tokens;
mod parser;
mod tokenizer;
mod tokens;

use core::ops::Range;

pub use filtered_tokens::*;
pub use parser::*;
use parser_tools::Spanify;
pub use tokenizer::*;
pub use tokens::*;

use everything_structures::Structure;

pub fn parse_structure<'source>(input: &'source str) -> Result<Structure, Error<'source>> {
    let mut parser = Parser::new(FilterTokens::new(Spanify::new(Tokenizer::new(
        input.chars(),
    ))));
    parser.parse_structure()
}
