use core::iter::Peekable;

use everything_structures::AbstractId;

use crate::{Span, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum FilteredToken<'source> {
    Abstract(AbstractId),
    NaturalNumber(u128),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Invalid(&'source str),
}

impl<'source> TryFrom<Token<'source>> for FilteredToken<'source> {
    type Error = ();

    fn try_from(token: Token<'source>) -> Result<Self, Self::Error> {
        match token {
            Token::Abstract(digits) => Ok(Self::Abstract(digits.into())),
            Token::OpeningParenthesis => Ok(Self::OpeningParenthesis),
            Token::ClosingParenthesis => Ok(Self::ClosingParenthesis),
            Token::OpeningBrace => Ok(Self::OpeningBrace),
            Token::ClosingBrace => Ok(Self::ClosingBrace),
            Token::Comma => Ok(Self::Comma),
            Token::Invalid(invalid) => Ok(Self::Invalid(invalid)),
            Token::NaturalNumber(digits) => Ok(Self::NaturalNumber(digits.into())),
            _ => Err(()),
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
