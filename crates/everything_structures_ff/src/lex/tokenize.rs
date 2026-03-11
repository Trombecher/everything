use std::{
    iter::{self, Peekable},
    str::Chars,
};

use crate::{SourceIndex, Token, TokenKind};

#[inline]
#[must_use]
fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

#[must_use]
fn can_start_token(c: Option<char>) -> bool {
    is_whitespace(c.unwrap_or('x')) || matches!(c, Some('(' | '@' | ')' | ',' | '{' | '}') | None)
}

fn next_token(chars: &mut Peekable<Chars>) -> Option<Token> {
    // Skip whitespace

    match chars.next() {
        Some(c) if is_whitespace(c) => {
            let mut length = 1_u32;

            while let Some(c) = chars.peek().copied()
                && is_whitespace(c)
            {
                chars.next();
                length += 1;
            }

            Some(Token {
                kind: TokenKind::Whitespace,
                length,
            })
        }
        Some('(') => Some(Token {
            kind: TokenKind::OpeningParenthesis,
            length: 1,
        }),
        Some(')') => Some(Token {
            kind: TokenKind::ClosingParenthesis,
            length: 1,
        }),
        Some('{') => Some(Token {
            kind: TokenKind::OpeningBrace,
            length: 1,
        }),
        Some('}') => Some(Token {
            kind: TokenKind::ClosingBrace,
            length: 1,
        }),
        Some(',') => Some(Token {
            kind: TokenKind::Comma,
            length: 1,
        }),
        Some('@') => {
            // Skip abstract id.

            match chars.next() {
                Some('0'..='9') => {}
                _ => {
                    return Some(Token {
                        kind: TokenKind::Invalid,
                        length: 2,
                    });
                }
            }

            let mut length: SourceIndex = 2;

            while let Some('0'..='9') = chars.peek().copied() {
                chars.next();
                length += 1;
            }

            Some(Token {
                kind: TokenKind::Abstract,
                length: length,
            })
        }
        None => None,
        Some(c) => {
            // Invalid id

            let mut length: SourceIndex = c.len_utf8() as SourceIndex;

            while !can_start_token(chars.peek().copied()) {
                length += chars.next().map_or(0, |c| c.len_utf8() as SourceIndex);
            }

            Some(Token {
                kind: TokenKind::Invalid,
                length,
            })
        }
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut chars = input.chars().peekable();
    iter::from_fn(move || next_token(&mut chars))
}
