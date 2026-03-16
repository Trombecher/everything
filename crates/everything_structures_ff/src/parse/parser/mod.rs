#[cfg(test)]
mod tests;

use std::iter::Peekable;

use everything_structures::{Object, Property, Structure};

use crate::{
    Span,
    parse::{Error, ErrorInfo, filtered::FilteredToken},
};

pub struct Parser<I: Iterator<Item = Span<FilteredToken>>> {
    tokens: Peekable<I>,
}

macro_rules! bail {
    ($range:expr, $($arg:expr),+) => {
        return Err(Box::new(ErrorInfo {message: format!($($arg),+), range: $range }))
    };
}

impl<I: Iterator<Item = Span<FilteredToken>>> Parser<I> {
    #[must_use]
    #[inline]
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_structure(&mut self) -> Result<Structure, Error> {
        match self.tokens.next() {
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => {}
            token => bail!(token.map(|s| s.range), "expected {{"),
        }

        self.parse_structure_continue()
    }

    fn parse_structure_continue(&mut self) -> Result<Structure, Error> {
        let mut properties = Vec::new();

        loop {
            // Start of property

            match self.tokens.next() {
                Some(Span {
                    value: FilteredToken::ClosingBrace,
                    ..
                }) => break,
                Some(Span {
                    value: FilteredToken::OpeningParenthesis,
                    ..
                }) => {}
                token => bail!(token.map(|s| s.range), "expected '(' or '}}'"),
            }

            let tag = self.parse_object()?;

            match self.tokens.next() {
                Some(Span {
                    value: FilteredToken::Comma,
                    ..
                }) => {}
                token => bail!(token.map(|s| s.range), "expected ','"),
            }

            let value = self.parse_object()?;

            properties.push(Property { tag, value });

            match self.tokens.next() {
                Some(Span {
                    value: FilteredToken::ClosingParenthesis,
                    ..
                }) => {}
                token => bail!(token.map(|s| s.range), "expected ')', got: {:?}", token),
            }
        }

        Ok(Structure::EMPTY.add(&mut properties))
    }

    fn parse_object(&mut self) -> Result<Object, Error> {
        match self.tokens.next() {
            Some(Span {
                value: FilteredToken::Abstract(id),
                ..
            }) => Ok(Object::Abstract(id)),
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => Ok(Object::Structure(self.parse_structure_continue()?)),
            token => bail!(token.map(|s| s.range), "expected @<<id>> or '{{'"),
        }
    }
}
