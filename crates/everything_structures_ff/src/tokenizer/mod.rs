#[cfg(test)]
mod tests;

use core::str;

use parser_tools::PeekableChars;

use crate::{
    AbstractSource, ByteSource, BytesSource, CharacterSource, IntegerSource, TextSource, Token,
};

#[must_use]
fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

#[must_use]
fn char_can_start_token(c: Option<char>) -> bool {
    is_whitespace(c.unwrap_or('x'))
        || matches!(
            c,
            Some('(' | '@' | ')' | ',' | '{' | '}' | '0'..='9' | 'X' | 'x' | '\'' | '"' | '-')
                | None
        )
}

/// An iterator over tokens.
#[derive(Clone)]
pub struct Tokenizer<'source> {
    chars: PeekableChars<'source>,
}

impl<'source> Tokenizer<'source> {
    /// Creates a new
    #[must_use]
    pub fn new(source: &'source str) -> Self {
        Self {
            chars: PeekableChars::new(source),
        }
    }

    fn skip_digits(&mut self) {
        while let Some('0'..='9') = self.chars.peek() {
            self.chars.next();
        }
    }
}

impl<'source> Iterator for Tokenizer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.chars.remaining().as_ptr();

        /// Returns a str slice from the start to the cursor of the chars.
        macro_rules! skipped {
            () => {
                str::from_raw_parts(
                    start,
                    self.chars.remaining().as_ptr().offset_from_unsigned(start),
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
                        return Some(Token::Invalid(unsafe { skipped!() }));
                    }
                }

                self.skip_digits();

                Some(Token::Abstract(unsafe {
                    AbstractSource::new_unchecked(skipped!())
                }))
            }
            Some('x') => {
                match self.chars.peek() {
                    Some(c) if c.is_ascii_hexdigit() => {}
                    _ => return Some(Token::Invalid(unsafe { skipped!() })),
                }

                self.chars.next();

                match self.chars.peek() {
                    Some(c) if c.is_ascii_hexdigit() => {}
                    _ => return Some(Token::Invalid(unsafe { skipped!() })),
                }

                self.chars.next();

                Some(Token::Byte(unsafe {
                    ByteSource::new_unchecked(skipped!())
                }))
            }
            Some('X') => {
                match self.chars.peek() {
                    Some(c) if c.is_ascii_hexdigit() => {}
                    _ => return Some(Token::Invalid(unsafe { skipped!() })),
                }

                self.chars.next();

                match self.chars.peek() {
                    Some(c) if c.is_ascii_hexdigit() => {}
                    _ => return Some(Token::Invalid(unsafe { skipped!() })),
                }

                self.chars.next();

                while let Some(c) = self.chars.peek()
                    && c.is_ascii_hexdigit()
                {
                    self.chars.next();
                }

                Some(Token::Bytes(unsafe {
                    BytesSource::new_unchecked(skipped!())
                }))
            }
            Some('-') => {
                // Ensure that there is at least one digit after '-'.
                match self.chars.peek() {
                    Some('0'..='9') => {}
                    _ => {
                        return Some(Token::Invalid(unsafe { skipped!() }));
                    }
                }

                self.chars.next();

                self.skip_digits();

                Some(Token::Integer(unsafe {
                    IntegerSource::new_unchecked(skipped!())
                }))
            }
            Some('0'..='9') => {
                self.skip_digits();

                Some(Token::Integer(unsafe {
                    IntegerSource::new_unchecked(skipped!())
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

                while !char_can_start_token(self.chars.peek()) {
                    self.chars.next();
                }

                Some(Token::Invalid(unsafe { skipped!() }))
            }
            None => None,
        }
    }
}
