#[cfg(test)]
mod tests;

use core::{
    slice,
    str::{self, Chars},
};

use parser_tools::PeekableChars;

use crate::{ByteSource, CharacterSource, Digit, Digits, TextSource, Token};

#[must_use]
fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

#[must_use]
fn can_start_token(c: Option<char>) -> bool {
    is_whitespace(c.unwrap_or('x'))
        || matches!(
            c,
            Some('(' | '@' | ')' | ',' | '{' | '}' | '0'..='9' | 'X' | 'x' | '\'' | '"') | None
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

        /// Returns a str slice from the start to the cursor of the chars.
        macro_rules! skipped {
            () => {
                str::from_raw_parts(
                    start.as_ptr(),
                    start.len().unchecked_sub(self.chars.as_str().len()),
                )
            };
        }

        match self.chars.next() {
            Some(c) if is_whitespace(c) => {
                while let Some(c) = self.chars.peek()
                    && is_whitespace(c)
                {
                    self.chars.next();
                }

                Some(Token::Whitespace(unsafe { skipped!() }))
            }
            Some('#') => {
                while !matches!(self.chars.peek(), Some('\r' | '\n') | None) {
                    self.chars.next();
                }

                Some(Token::LineComment(unsafe { skipped!() }))
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
            Some('x') => {
                for length in 1..3_usize {
                    // Skip two ascii hex digits.

                    if !self.chars.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
                        return Some(Token::Invalid(unsafe {
                            str::from_raw_parts(start.as_ptr(), length)
                        }));
                    }

                    self.chars.next();
                }

                Some(Token::Byte(unsafe {
                    ByteSource::new_unchecked(str::from_raw_parts(start.as_ptr(), 3))
                }))
            }
            Some('X') => todo!("bytes literals"),
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
            Some('"') => loop {
                match self.chars.next() {
                    Some('"') => {
                        break Some(Token::Text(unsafe {
                            TextSource::new_unchecked(skipped!())
                        }));
                    }
                    Some('\\') => todo!("escapes"),
                    Some(_) => {}
                    None => {
                        break Some(Token::Invalid(unsafe { skipped!() }));
                    }
                }
            },
            Some('\'') => {
                match self.chars.next() {
                    Some('\\') => todo!("escapes"),
                    Some('\'') | None => {
                        return Some(Token::Invalid(unsafe { skipped!() }));
                    }
                    Some(_) => {}
                };

                match self.chars.next() {
                    Some('\'') => Some(Token::Character(unsafe {
                        CharacterSource::new_unchecked(skipped!())
                    })),
                    _ => Some(Token::Invalid(unsafe { skipped!() })),
                }
            }
            Some(_) => {
                // Invalid id

                while !can_start_token(self.chars.peek()) {
                    self.chars.next();
                }

                Some(Token::Invalid(unsafe { skipped!() }))
            }
            None => None,
        }
    }
}
