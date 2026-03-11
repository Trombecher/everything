pub mod lex;
pub mod parse;

use std::ops::Range;

use everything_structures::Structure;

use crate::{
    lex::tokenize,
    parse::{Error, FilteredTokens, Parser},
};

pub type SourceIndex = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct Span<T> {
    pub range: Range<SourceIndex>,
    pub value: T,
}

pub fn parse_structure(input: &str) -> Result<Structure, Error> {
    let mut parser = Parser::new(FilteredTokens::new(tokenize(input), input));
    parser.parse_structure()
}
