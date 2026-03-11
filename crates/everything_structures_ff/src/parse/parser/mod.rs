#[cfg(test)]
mod tests;

use std::iter::Peekable;

use everything_structures::{Object, Property, Structure};

use crate::{
    Span,
    parse::{Error, filtered::FilteredToken},
};

pub struct Parser<I: Iterator<Item = Span<FilteredToken>>> {
    tokens: Peekable<I>,
}

macro_rules! bail {
    ($($arg:expr),+) => {
        return Err(format!($($arg),+))
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
        match self.tokens.peek() {
            Some(Span {
                value: FilteredToken::OpeningBrace,
                ..
            }) => {}
            _ => bail!("expected {{"),
        }

        self.tokens.next();

        self.parse_structure_continue()
    }

    fn parse_structure_continue(&mut self) -> Result<Structure, Error> {
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
            self.tokens.next();

            let tag = self.parse_object()?;

            match self.tokens.peek() {
                Some(Span {
                    value: FilteredToken::Comma,
                    ..
                }) => {}
                _ => bail!("expected ','"),
            }

            // Skip ','
            self.tokens.next();

            let value = self.parse_object()?;

            properties.push(Property { tag, value });

            match self.tokens.peek() {
                Some(Span {
                    value: FilteredToken::ClosingParenthesis,
                    ..
                }) => {}
                _ => bail!("expected ')'"),
            }

            // Skip ')'
            self.tokens.next();
        }

        self.tokens.next(); // Skip '}'

        Ok(Structure::EMPTY.change(&mut [], &mut properties))
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

                Ok(Object::Structure(self.parse_structure_continue()?))
            }
            _ => bail!("expected @<<id>> or '{{'"),
        }
    }
}
