use std::iter::Peekable;

use everything_structures::{Change, Object, Property, Structure};

use crate::{
    Span, Token,
    parse::filtered::{FilteredToken, FilteredTokens},
};

mod error;
mod filtered;
#[cfg(test)]
mod tests;
mod ulid_from_iter;

pub use error::*;

pub struct Parser<'source, I: Iterator<Item = Token>> {
    tokens: Peekable<FilteredTokens<'source, I>>,
}

macro_rules! bail {
    ($($arg:expr),+) => {
        return Err(format!($($arg),+))
    };
}

impl<'source, I: Iterator<Item = Token>> Parser<'source, I> {
    #[must_use]
    #[inline]
    pub fn new(tokens: I, source: &'source str) -> Self {
        Self {
            tokens: FilteredTokens::new(tokens, source).peekable(),
        }
    }

    pub fn parse_input(&mut self) -> Result<_, Error> {
        match self.tokens.peek() {
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => {}
            _ => bail!("expected"),
        }

        self.tokens.next();

        Ok(self.parse_structure())
    }

    fn parse_structure(&mut self) -> Result<Structure, Error> {
        let mut properties = Vec::new();

        loop {
            // Start of property

            match self.tokens.peek() {
                Some(Span {
                    value: FilteredToken::ClosingBrace,
                    ..
                }) => break,
                Some(Span {
                    value: FilteredToken::OpeningParenthesis,
                    ..
                }) => {}
                _ => bail!("expected '(' or '}}'"),
            }

            // Skip '('.
            self.tokens.next()?;

            let tag = self.parse_object()?;

            match self.tokens.peek() {
                Some(Span {
                    value: FilteredToken::Comma,
                    ..
                }) => {}
                _ => bail!("expected ','"),
            }

            // Skip ','
            self.tokens.next()?;

            let value = self.parse_object()?;

            properties.push(Change::Add(Property { tag, value }));

            match self.tokens.peek() {
                Some(Span {
                    value: FilteredToken::ClosingParenthesis,
                    ..
                }) => {}
                _ => bail!("expected ')'"),
            }

            // Skip ')'
            self.tokens.next()?;
        }

        self.tokens.next()?; // Skip '}'

        Ok(Structure::EMPTY.change(&mut properties))
    }

    fn parse_object(&mut self) -> Result<Object, Error> {
        match self.tokens.peek() {
            Some(Span {
                value: FilteredToken::Abstract(id),
                ..
            }) => {
                let id = *id;
                self.tokens.next();

                Ok(Object::Abstract(id))
            }
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => {
                self.tokens.next();

                Ok(Object::Structure(self.parse_structure()?))
            }
            _ => bail!("expected @<<NAME>> or '{{'"),
        }
    }
}
