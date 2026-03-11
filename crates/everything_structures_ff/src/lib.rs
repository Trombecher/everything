mod lex;
mod parse;

use std::ops::Range;

pub use lex::*;
pub use parse::*;

pub struct Span<T> {
    range: Range<SourceIndex>,
    value: T,
}
