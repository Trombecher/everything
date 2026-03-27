use crate::{Span, tokens::Token};

pub struct Spanify<'source, I: Iterator<Item = Token<'source>>> {
    index: u32,
    tokens: I,
}

impl<'source, I: Iterator<Item = Token<'source>>> Spanify<'source, I> {
    #[must_use]
    pub const fn new(tokens: I) -> Self {
        Self { index: 0, tokens }
    }
}

impl<'source, I: Iterator<Item = Token<'source>>> Iterator for Spanify<'source, I> {
    type Item = Span<Token<'source>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.tokens.next();

        let range = self.index..self.index + next.map_or(0, |t| t.length() as u32);

        self.index = range.end;

        next.map(|token| Span {
            value: token,
            range,
        })
    }
}
