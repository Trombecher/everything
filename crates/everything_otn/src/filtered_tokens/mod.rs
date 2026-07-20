use core::iter::Peekable;

use everything_objects::{Abstract, Composite, Object};
use parser_tools::Span;

use crate::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum FilteredToken<'source> {
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Invalid(&'source str),
    Object(Object),
}

impl<'source> TryFrom<Token<'source>> for FilteredToken<'source> {
    type Error = ();

    fn try_from(token: Token<'source>) -> Result<Self, Self::Error> {
        match token {
            Token::Abstract(digits) => {
                if let Some(id) = digits.parse() {
                    Ok(Self::Object(Abstract(id).into()))
                } else {
                    Ok(Self::Invalid(digits.as_str()))
                }
            }
            Token::OpeningParenthesis => Ok(Self::OpeningParenthesis),
            Token::ClosingParenthesis => Ok(Self::ClosingParenthesis),
            Token::OpeningBrace => Ok(Self::OpeningBrace),
            Token::ClosingBrace => Ok(Self::ClosingBrace),
            Token::Comma => Ok(Self::Comma),
            Token::Invalid(invalid) => Ok(Self::Invalid(invalid)),
            Token::Integer(source) => {
                if let Some(n) = source.parse() {
                    Ok(Self::Object(Object::new_integer(n)))
                } else {
                    Ok(Self::Invalid(source.as_str()))
                }
            }
            Token::Byte(byte) => Ok(Self::Object(Object::Composite(byte.parse().into()))),
            Token::Bytes(bytes) => Ok(Self::Object(Object::Composite(bytes.parse().into()))),
            Token::Whitespace(_) => Err(()),
            Token::LineComment(_) => Err(()),
            Token::Character(character_source) => Ok(Self::Object(Object::Composite(
                character_source.parse().into(),
            ))),
            Token::Text(text_source) => Ok(Self::Object(
                text_source
                    .parse()
                    .map_or(Composite::Empty, Composite::Text)
                    .into(),
            )),
        }
    }
}

/// Filters out whitespace and resolves abstract ids.
pub struct FilterTokens<'source, I: Iterator<Item = Span<Token<'source>>>> {
    tokens: Peekable<I>,
}

impl<'source, I: Iterator<Item = Span<Token<'source>>>> FilterTokens<'source, I> {
    #[must_use]
    #[inline]
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }
}

impl<'source, I: Iterator<Item = Span<Token<'source>>>> Iterator for FilterTokens<'source, I> {
    type Item = Span<FilteredToken<'source>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Span {
            value: Token::Whitespace(_) | Token::LineComment(_),
            ..
        }) = self.tokens.peek()
        {
            self.tokens.next();
        }

        match self.tokens.next() {
            Some(Span {
                value: token,
                range,
            }) if let Ok(token) = token.try_into() => Some(Span {
                range,
                value: token,
            }),
            None => None,
            _ => unreachable!(),
        }
    }
}
