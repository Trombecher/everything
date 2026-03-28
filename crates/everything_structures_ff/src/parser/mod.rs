#[cfg(test)]
mod tests;

use core::{iter::Peekable, num::NonZeroU128};

use alloc::{boxed::Box, vec::Vec};
use everything_structures::{AnyStructure, Object, Property, Structure};
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
            }) => self.parse_structure_continue(),
            Some(Span {
                value: FilteredToken::NaturalNumber(n),
                ..
            }) if let Some(n) = NonZeroU128::new(n) => Ok(Structure::NaturalNumber(n)),
            token => bail!(token, "expected '{{' or a positive integer"),
        }
    }

    fn parse_structure_continue(&mut self) -> Result<Structure, Error<'source>> {
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

        Ok(Structure::Any(AnyStructure::new(&mut properties)))
    }

    fn parse_object(&mut self) -> Result<Object, Error<'source>> {
        match self.tokens.next() {
            Some(Span {
                value: FilteredToken::Abstract(id),
                ..
            }) => Ok(Object::Abstract(id)),
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => Ok(Object::Structure(self.parse_structure_continue()?)),
            Some(Span {
                value: FilteredToken::NaturalNumber(n),
                ..
            }) => match NonZeroU128::new(n) {
                Some(n) => Ok(Object::Structure(Structure::NaturalNumber(n))),
                None => Ok(Object::ZERO),
            },
            token => bail!(token, "expected @<<id>> or '{{'"),
        }
    }
}
