#![feature(str_from_raw_parts)]
#![no_std]

extern crate alloc;

mod chars;
mod filtered_tokens;
mod parser;
mod spanify;
mod tokenizer;
mod tokens;

use core::ops::Range;

pub use filtered_tokens::*;
pub use parser::*;
pub use spanify::*;
pub use tokenizer::*;
pub use tokens::*;

use everything_structures::Structure;

pub type SourceIndex = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct Span<T> {
    pub range: Range<SourceIndex>,
    pub value: T,
}

pub fn parse_structure<'source>(input: &'source str) -> Result<Structure, Error<'source>> {
    let mut parser = Parser::new(FilterTokens::new(Spanify::new(Tokenizer::new(
        input.chars(),
    ))));
    parser.parse_structure()
}
