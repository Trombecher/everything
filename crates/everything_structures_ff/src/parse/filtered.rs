use everything_structures::AbstractId;

use crate::{
    SourceIndex, Span, Token, TokenKind, extract_abstract, parse::ulid_from_iter::UlidFromIterator,
};

pub enum FilteredToken {
    Abstract(AbstractId),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Invalid,
}

/// Filters out whitespace and resolves abstract ids.
pub struct FilteredTokens<'source, I: Iterator<Item = Token>> {
    position: SourceIndex,
    tokens: I,
    source: &'source str,
}

impl<'source, I: Iterator<Item = Token>> FilteredTokens<'source, I> {
    #[must_use]
    #[inline]
    pub const fn new(tokens: I, source: &'source str) -> Self {
        Self {
            position: 0,
            source,
            tokens,
        }
    }
}

impl<'source, I: Iterator<Item = Token>> Iterator for FilteredTokens<'source, I> {
    type Item = Span<FilteredToken>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token = self.tokens.next();
            let length = token.map_or(0, |t| t.length);

            let range = self.position..self.position + length;
            self.position += length;

            match token.map(|t| t.kind) {
                Some(TokenKind::OpeningParenthesis) => {
                    return Some(Span {
                        range,
                        value: FilteredToken::OpeningParenthesis,
                    });
                }
                Some(TokenKind::ClosingParenthesis) => {
                    return Some(Span {
                        range,
                        value: FilteredToken::ClosingParenthesis,
                    });
                }
                Some(TokenKind::OpeningBrace) => {
                    return Some(Span {
                        range,
                        value: FilteredToken::OpeningBrace,
                    });
                }
                Some(TokenKind::ClosingBrace) => {
                    return Some(Span {
                        range,
                        value: FilteredToken::ClosingBrace,
                    });
                }
                Some(TokenKind::Comma) => {
                    return Some(Span {
                        range,
                        value: FilteredToken::Comma,
                    });
                }
                Some(TokenKind::Invalid) => {
                    return Some(Span {
                        range,
                        value: FilteredToken::Invalid,
                    });
                }
                Some(TokenKind::Whitespace) => {
                    // Ignore whitespace and wait for next token
                }
                Some(TokenKind::Abstract) => {
                    let id: AbstractId = extract_abstract(self.source, range.clone())
                        .collect::<UlidFromIterator>()
                        .0;

                    return Some(Span {
                        range,
                        value: FilteredToken::Abstract(id),
                    });
                }
                None => return None,
            }
        }
    }
}
