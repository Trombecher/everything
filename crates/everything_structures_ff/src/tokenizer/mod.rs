#[cfg(test)]
mod tests;

use core::{
    slice,
    str::{self, Chars},
};

use parser_tools::PeekableChars;

use crate::{Digit, Digits, Token};

#[inline]
#[must_use]
fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

#[must_use]
fn can_start_token(c: Option<char>) -> bool {
    is_whitespace(c.unwrap_or('x'))
        || matches!(
            c,
            Some('(' | '@' | ')' | ',' | '{' | '}' | '0'..='9') | None
        )
}

pub struct Tokenizer<'source> {
    chars: PeekableChars<'source>,
}

impl<'source> Tokenizer<'source> {
    #[must_use]
    pub const fn new(chars: Chars<'source>) -> Self {
        Self {
            chars: PeekableChars::new(chars),
        }
    }
}

impl<'source> Iterator for Tokenizer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.chars.as_str();

        match self.chars.next() {
            Some(c) if is_whitespace(c) => {
                while let Some(c) = self.chars.peek()
                    && is_whitespace(c)
                {
                    self.chars.next();
                }

                Some(Token::Whitespace(unsafe {
                    str::from_raw_parts(start.as_ptr(), start.len() - self.chars.as_str().len())
                }))
            }
            Some('#') => {
                while !matches!(self.chars.peek(), Some('\r' | '\n') | None) {
                    self.chars.next();
                }

                Some(Token::LineComment(unsafe {
                    str::from_raw_parts(start.as_ptr(), start.len() - self.chars.as_str().len())
                }))
            }
            Some('(') => Some(Token::OpeningParenthesis),
            Some(')') => Some(Token::ClosingParenthesis),
            Some('{') => Some(Token::OpeningBrace),
            Some('}') => Some(Token::ClosingBrace),
            Some(',') => Some(Token::Comma),
            Some('@') => {
                // Skip abstract id.

                match self.chars.peek() {
                    Some('0'..='9') => {}
                    _ => {
                        return Some(Token::Invalid(unsafe {
                            str::from_raw_parts(start.as_ptr(), 1)
                        }));
                    }
                }

                while self
                    .chars
                    .peek()
                    .is_some_and(|digit| Digit::try_from(digit).is_ok())
                {
                    self.chars.next();
                }

                Some(Token::Abstract(unsafe {
                    Digits::new_unchecked(slice::from_raw_parts(
                        start.as_ptr().add(1),
                        start.len() - self.chars.as_str().len() - 1,
                    ))
                }))
            }
            Some('0'..='9') => {
                while self
                    .chars
                    .peek()
                    .is_some_and(|digit| Digit::try_from(digit).is_ok())
                {
                    self.chars.next();
                }

                Some(Token::NaturalNumber(unsafe {
                    Digits::new_unchecked(slice::from_raw_parts(
                        start.as_ptr(),
                        start.len() - self.chars.as_str().len(),
                    ))
                }))
            }
            None => None,
            Some(_) => {
                // Invalid id

                while !can_start_token(self.chars.peek()) {
                    self.chars.next();
                }

                Some(Token::Invalid(unsafe {
                    str::from_raw_parts(start.as_ptr(), start.len() - self.chars.as_str().len())
                }))
            }
        }
    }
}
