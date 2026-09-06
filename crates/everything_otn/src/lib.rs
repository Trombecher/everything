#![feature(str_from_raw_parts)]
#![no_std]

//! Parsing for the Everything Object Text Notation.

extern crate alloc;

mod filtered_tokens;
mod parser;
mod tokenizer;
mod tokens;

pub use filtered_tokens::*;
pub use parser::*;
use parser_tools::Spanify;
pub use tokenizer::*;
pub use tokens::*;

use everything_objects::{Composite, Object};

pub trait Parsable: Sized {
    fn parse<'source>(input: &'source str) -> Result<Self, Error<'source>>;
}

impl Parsable for Composite {
    fn parse<'source>(input: &'source str) -> Result<Self, Error<'source>> {
        let mut parser = Parser::new(FilterTokens::new(Spanify::new(Tokenizer::new(input))));
        parser.parse_composite()
    }
}

impl Parsable for Object {
    fn parse<'source>(input: &'source str) -> Result<Self, Error<'source>> {
        let mut parser = Parser::new(FilterTokens::new(Spanify::new(Tokenizer::new(input))));
        parser.parse_object()
    }
}
