pub mod lex;
pub mod parse;

use std::ops::Range;

pub type SourceIndex = u32;

pub struct Span<T> {
    range: Range<SourceIndex>,
    value: T,
}
