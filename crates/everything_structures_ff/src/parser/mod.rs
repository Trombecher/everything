#[cfg(test)]
mod tests;

use core::iter::Peekable;

use alloc::{boxed::Box, vec::Vec};
use everything_structures::{Object, Property, Structure};
use parser_tools::Span;

pub type Error<'source> = Box<ErrorInfo<'source>>;

#[derive(PartialEq, Debug, Clone)]
pub struct ErrorInfo<'source> {
    pub found: Option<Span<FilteredToken<'source>>>,
    pub expected: &'static str,
}

use crate::FilteredToken;

pub struct Parser<'source, I: Iterator<Item = Span<FilteredToken<'source>>>> {
    tokens: Peekable<I>,
}

macro_rules! bail {
    ($token:expr, $expected:literal) => {
        return Err(Box::new(ErrorInfo {
            expected: $expected,
            found: $token,
        }))
    };
}

impl<'source, I: Iterator<Item = Span<FilteredToken<'source>>>> Parser<'source, I> {
    #[must_use]
    #[inline]
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_structure(&mut self) -> Result<Structure, Error<'source>> {
        match self.tokens.next() {
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => self.parse_explicit_structure(),
            Some(Span {
                value: FilteredToken::Object(Object::Structure(s)),
                ..
            }) => Ok(s),
            token => bail!(token, "expected a structure"),
        }
    }

    fn parse_explicit_structure(&mut self) -> Result<Structure, Error<'source>> {
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
                token => bail!(token, "expected '(' or '}}'"),
            }

            let tag = self.parse_object()?;

            match self.tokens.next() {
                Some(Span {
                    value: FilteredToken::Comma,
                    ..
                }) => {}
                token => bail!(token, "expected ','"),
            }

            let value = self.parse_object()?;

            properties.push(Property { tag, value });

            match self.tokens.next() {
                Some(Span {
                    value: FilteredToken::ClosingParenthesis,
                    ..
                }) => {}
                token => bail!(token, "expected ')'"),
            }

            // Skip trailing or seperating commas
            if let Some(Span {
                value: FilteredToken::Comma,
                ..
            }) = self.tokens.peek()
            {
                self.tokens.next();
            }
        }

        Ok(Structure::new(&mut properties))
    }

    fn parse_object(&mut self) -> Result<Object, Error<'source>> {
        match self.tokens.next() {
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => Ok(Object::Structure(self.parse_explicit_structure()?)),
            Some(Span {
                value: FilteredToken::Object(o),
                ..
            }) => Ok(o),
            token => bail!(token, "expected @<<id>> or '{{'"),
        }
    }
}
