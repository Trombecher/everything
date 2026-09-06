#[cfg(test)]
mod tests;

use core::{
    hint::{assert_unchecked, unreachable_unchecked},
    iter,
    num::NonZeroUsize,
    str,
};

use alloc::sync::Arc;
use everything_objects::{Byte, BytesComposite, TextComposite};
use parser_tools::TokenLength;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'source> {
    /// An abstract literal `@1234567890`
    Abstract(AbstractSource<'source>),

    /// `(`
    OpeningParenthesis,

    /// `)`
    ClosingParenthesis,

    /// `{`
    OpeningBrace,

    /// `}`
    ClosingBrace,

    /// `,`
    Comma,

    /// A whitespace token.
    Whitespace(&'source str),

    /// An invalid token.
    Invalid(&'source str),

    /// A line comment `#`.
    LineComment(&'source str),

    /// A byte literal `xA7`.
    Byte(ByteSource<'source>),

    /// A bytes literal `X5417A4EF00`.
    Bytes(BytesSource<'source>),

    /// A character literal `'ä'`.
    Character(CharacterSource<'source>),

    /// A text literal `"abcd_5390 ?*!\t"`, unescaped.
    Text(TextSource<'source>),

    /// A natural number literal.
    Integer(IntegerSource<'source>),
}

impl<'source> Token<'source> {
    pub fn as_str(self) -> &'source str {
        match self {
            Token::Abstract(source) => source.as_str(),
            Token::OpeningParenthesis => "(",
            Token::ClosingParenthesis => ")",
            Token::OpeningBrace => "{",
            Token::ClosingBrace => "}",
            Token::Comma => ",",
            Token::Whitespace(source) => source,
            Token::Invalid(source) => source,
            Token::LineComment(source) => source,
            Token::Byte(source) => source.as_str(),
            Token::Bytes(source) => source.as_str(),
            Token::Character(source) => source.as_str(),
            Token::Text(source) => source.as_str(),
            Token::Integer(source) => source.as_str(),
        }
    }
}

impl<'source> TokenLength for Token<'source> {
    fn length(&self) -> u32 {
        self.as_str().len() as u32
    }
}

/// Source string wrapper for an abstract object. Must match this regex:
///
/// ```regex
/// @\d+
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AbstractSource<'source>(&'source str);

impl<'source> AbstractSource<'source> {
    #[must_use]
    pub const unsafe fn new_unchecked(source: &'source str) -> Self {
        Self(source)
    }

    #[must_use]
    pub const fn length(self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn as_str(self) -> &'source str {
        self.0
    }

    #[must_use]
    pub fn parse(self) -> Option<u128> {
        let mut bytes = self.0.bytes();

        match bytes.next() {
            Some(b'@') => {}
            _ => unsafe { unreachable_unchecked() },
        }

        let mut n = 0_u128;

        loop {
            match bytes.next() {
                Some(digit @ b'0'..=b'9') => {
                    n = n
                        .checked_mul(10)
                        .and_then(|n| n.checked_add((digit - b'0') as u128))?;
                }
                None => break,
                _ => unsafe { unreachable_unchecked() },
            }
        }

        Some(n)
    }
}

/// A string that matches the following regex:
///
/// ```regex
/// -?\d+
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IntegerSource<'source>(&'source str);

impl<'source> IntegerSource<'source> {
    /// # SAFETY
    ///
    /// It must match the regex of this type.
    pub const unsafe fn new_unchecked(source: &'source str) -> Self {
        Self(source)
    }

    /// Parses the digits to a [`i128`]. Returns [`None`] iff the computation overflows.
    #[must_use]
    pub fn parse(self) -> Option<i128> {
        enum Info {
            Negative,
            Positive(i128),
        }

        let mut bytes = self.0.bytes();

        let info = match bytes.next() {
            Some(b'-') => Info::Negative,
            Some(digit @ b'0'..=b'9') => Info::Positive((digit - b'0') as i128),
            _ => unsafe { unreachable_unchecked() },
        };

        if let Info::Positive(mut n) = info {
            loop {
                match bytes.next() {
                    Some(digit @ b'0'..=b'9') => {
                        n = n
                            .checked_mul(10)
                            .and_then(|n| n.checked_add((digit - b'0') as i128))?;
                    }
                    None => break,
                    Some(_) => unsafe { unreachable_unchecked() },
                }
            }

            Some(n)
        } else {
            let mut n = 0_i128;

            loop {
                match bytes.next() {
                    Some(digit @ b'0'..=b'9') => {
                        n = n
                            .checked_mul(10)
                            .and_then(|n| n.checked_sub((digit - b'0') as i128))?;
                    }
                    None => break,
                    Some(_) => unsafe { unreachable_unchecked() },
                }
            }

            Some(n)
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        self.0
    }

    #[must_use]
    pub const fn length(self) -> usize {
        self.0.len()
    }
}

/// Source for a byte literal. Guaranteed to match this regex:
///
/// ```regex
/// x[\dA-Fa-f]{2}
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ByteSource<'source>(&'source str);

impl<'source> ByteSource<'source> {
    /// # Safety
    ///
    /// The given source string slice MUST satisfy the invariant
    /// described in the documentation of [ByteSource].
    #[must_use]
    pub const unsafe fn new_unchecked(source: &'source str) -> Self {
        Self(source)
    }

    #[must_use]
    pub fn parse(self) -> Byte {
        unsafe {
            let mut bytes = self.0.bytes();

            assert_unchecked(bytes.len() == 3);
            assert_unchecked(bytes.next() == Some(b'x'));

            let high = match bytes.next() {
                Some(value @ b'0'..=b'9') => value - b'0',
                Some(value @ b'A'..=b'F') => value - b'A' + 10,
                Some(value @ b'a'..=b'f') => value - b'a' + 10,
                _ => unreachable_unchecked(),
            };

            let low = match bytes.next() {
                Some(value @ b'0'..=b'9') => value - b'0',
                Some(value @ b'A'..=b'F') => value - b'A' + 10,
                Some(value @ b'a'..=b'f') => value - b'a' + 10,
                _ => unreachable_unchecked(),
            };

            Byte((high << 4) | low)
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        self.0
    }
}

/// The source of a text literal.
///
/// # Invariants
///
/// It always begins with a `"` and ends with a `"`. Its length is greater
/// or equal to two.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextSource<'source>(&'source str);

impl<'source> TextSource<'source> {
    /// # Safety
    ///
    /// See [TextSource].
    #[must_use]
    pub const unsafe fn new_unchecked(source: &'source str) -> Self {
        Self(source)
    }

    /// Returns the string content between the quotes.
    #[inline]
    pub fn content(self) -> &'source str {
        unsafe { str::from_raw_parts(self.0.as_ptr().add(1), self.0.len().unchecked_sub(2)) }
    }

    pub fn real_byte_length(self) -> usize {
        let mut length = 0;
        let mut bytes = self.content().bytes();

        while let Some(byte) = bytes.next() {
            if byte == b'\\' {
                match bytes.next() {
                    Some(b'u' | b'x') => todo!(),
                    _ => {
                        length += 1;
                    }
                }
            }

            length += 1;
        }

        length
    }

    pub fn parse(self) -> Option<TextComposite> {
        let len = NonZeroUsize::new(self.real_byte_length())?;

        let mut bytes = self.content().bytes();

        let processed_bytes = iter::from_fn(move || {
            let byte = bytes.next()?;

            if byte == b'\\' {
                todo!("escape")
            } else {
                Some(byte)
            }
        });

        let bytes_structure = BytesComposite::from_iter(processed_bytes, len);

        Some(unsafe { TextComposite::new_unchecked(bytes_structure) })
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        self.0
    }
}

/// Source for a bytes literal. Guaranteed to match this regex:
///
/// ```regex
/// X[\dA-Fa-f]{2,}
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BytesSource<'source>(&'source str);

impl<'source> BytesSource<'source> {
    /// Creates a new [`BytesSource`] without checking the invariants.
    ///
    /// # SAFETY
    ///
    /// See [`BytesSource`].
    #[must_use]
    pub const unsafe fn new_unchecked(source: &'source str) -> Self {
        Self(source)
    }

    /// Computes the byte length of the allocation
    /// if one would to parse the content and store it
    /// somewhere.
    #[must_use]
    pub const fn real_byte_length(self) -> usize {
        unsafe {
            // Remove 'X' and then divide by two
            // because two ASCII hexdigits make up
            // one byte.
            self.as_str().len().unchecked_sub(1) / 2
        }
    }

    #[must_use]
    pub fn parse(self) -> BytesComposite {
        let mut allocation = Arc::<[u8]>::new_uninit_slice(self.real_byte_length());
        let mut allocation_bytes =
            unsafe { Arc::get_mut(&mut allocation).unwrap_unchecked() }.iter_mut();

        let mut bytes = self.0.bytes();

        match bytes.next() {
            Some(b'X') => {}
            _ => unsafe { unreachable_unchecked() },
        }

        loop {
            let high = match bytes.next() {
                Some(value @ b'0'..=b'9') => value - b'0',
                Some(value @ b'A'..=b'F') => value - b'A' + 10,
                Some(value @ b'a'..=b'f') => value - b'a' + 10,
                Some(_) => unsafe { unreachable_unchecked() },
                None => break,
            };

            let low = match bytes.next() {
                Some(value @ b'0'..=b'9') => value - b'0',
                Some(value @ b'A'..=b'F') => value - b'A' + 10,
                Some(value @ b'a'..=b'f') => value - b'a' + 10,
                _ => unsafe { unreachable_unchecked() },
            };

            unsafe { allocation_bytes.next().unwrap_unchecked() }.write((high << 4) | low);
        }

        debug_assert!(allocation_bytes.next().is_none());

        unsafe { BytesComposite::from_raw(allocation.assume_init()).unwrap_unchecked() }
    }

    #[must_use]
    pub const fn as_str(self) -> &'source str {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CharacterSource<'source>(&'source str);

impl<'source> CharacterSource<'source> {
    /// Creates a new [`CharacterSource`] without checking the invariants.
    ///
    /// # Safety
    ///
    /// See [`CharacterSource`].
    #[must_use]
    pub const unsafe fn new_unchecked(source: &'source str) -> Self {
        Self(source)
    }

    #[must_use]
    pub fn content(self) -> &'source str {
        unsafe { str::from_raw_parts(self.0.as_ptr().add(1), self.0.len().unchecked_sub(2)) }
    }

    #[must_use]
    pub fn parse(self) -> char {
        let mut chars = self.content().chars();

        match chars.next() {
            Some('\\') => todo!("decode char escape"),
            None | Some('\'') => unsafe { unreachable_unchecked() },
            Some(c) => c,
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        self.0
    }
}
