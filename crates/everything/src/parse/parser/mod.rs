#[cfg(test)]
mod tests;

use fallible_iterator::FallibleIterator;
use labuf::LookaheadBuffer;

use crate::{
    Abstract, Object, Property, Statement, Structure,
    parse::{Error, Token},
};

pub struct Parser<I: FallibleIterator<Item = Token, Error = Error>> {
    tokens: LookaheadBuffer<I>,
}

macro_rules! bail {
    ($($arg:expr),+) => {
        return Err(format!($($arg),+))
    };
}

impl<I: FallibleIterator<Item = Token, Error = Error>> Parser<I> {
    #[must_use]
    #[inline]
    pub const fn new(tokens: I) -> Self {
        Self {
            tokens: LookaheadBuffer::new(tokens),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.tokens.peek()? {
            Some(Token::LeftParenthesis) => {}
            _ => bail!("expected '(' while parsing start of statement"),
        }

        self.tokens.next()?;

        let target = self.parse_expression();

        todo!()
    }

    fn parse_expression(&mut self) -> Result<Object, Error> {
        let left = match self.tokens.peek()? {
            Some(Token::AbstractObject(id)) => {
                let id = id.clone();
                self.tokens.next()?;

                Object::Abstract(Abstract(id))
            }
            Some(Token::Natural(n)) => {
                let n = *n;
                self.tokens.next()?;

                Object::from_natural(n)
            }
            Some(Token::Not) => {
                self.tokens.next()?;

                Object::node_not(self.parse_expression()?)
            }
            Some(Token::LeftBrace) => {
                // Parses structure

                self.tokens.next()?;
                let mut properties = Vec::new();

                loop {
                    // Start of property

                    match self.tokens.peek()? {
                        Some(Token::RightBrace) => break,
                        Some(Token::LeftParenthesis) => {}
                        _ => bail!("expected '(' or '}}'"),
                    }

                    self.tokens.next()?;

                    let tag = self.parse_expression()?;

                    match self.tokens.peek()? {
                        Some(Token::Comma) => {}
                        _ => bail!("expected ','"),
                    }

                    self.tokens.next()?;

                    let value = self.parse_expression()?;

                    properties.push(Property { tag, value });

                    // Handle trailing comma
                    match self.tokens.peek()? {
                        Some(Token::Comma) => {
                            self.tokens.next()?;
                        }
                        _ => {}
                    }

                    match self.tokens.peek()? {
                        Some(Token::RightParenthesis) => {}
                        _ => bail!("expected ')'"),
                    }

                    self.tokens.next()?;
                }

                self.tokens.next()?; // Skip '}'

                Object::Structure(Structure::new(&mut properties))
            }
            _ => bail!(
                "expected 'not', 'query', 'exists', a natural number, an abstract object, '(', or '{{'"
            ),
        };

        Ok(left)
    }
}

impl<I: FallibleIterator<Item = Token, Error = Error>> FallibleIterator for Parser<I> {
    type Item = Statement;

    type Error = Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        match self.tokens.peek()? {
            None => Ok(None),
            Some(_) => self.parse_statement().map(Some),
        }
    }
}
