#[cfg(test)]
mod tests;

use core::{
    hint::{assert_unchecked, unreachable_unchecked},
    iter,
    mem::transmute,
    num::NonZeroUsize,
    str,
};

use everything_structures::{Byte, BytesStructure, TextStructure};
use parser_tools::TokenLength;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'source> {
    /// An abstract literal `@1234567890`
    Abstract(Digits<'source>),

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
    NaturalNumber(Digits<'source>),
}

impl<'source> TokenLength for Token<'source> {
    fn length(&self) -> u32 {
        match self {
            Self::Abstract(Digits(digits)) => digits.len() as u32 + 1,
            Self::Whitespace(ws) => ws.len() as u32,
            Self::Invalid(i) => i.len() as u32,
            Self::LineComment(lc) => lc.len() as u32,
            Self::NaturalNumber(Digits(digits)) => digits.len() as u32,
            Self::OpeningParenthesis
            | Self::ClosingParenthesis
            | Self::OpeningBrace
            | Self::ClosingBrace
            | Self::Comma => 1,
            Self::Byte(byte_source) => byte_source.as_str().len() as u32,
            Self::Bytes(bytes_source) => bytes_source.as_str().len() as u32,
            Self::Character(character_source) => character_source.as_str().len() as u32,
            Self::Text(text_source) => text_source.as_str().len() as u32,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
#[repr(u8)]
pub enum Digit {
    Zero = b'0',
    One = b'1',
    Two = b'2',
    Three = b'3',
    Four = b'4',
    Five = b'5',
    Six = b'6',
    Seven = b'7',
    Eight = b'8',
    Nine = b'9',
}

impl TryFrom<char> for Digit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Digits<'source>(pub &'source [Digit]);

impl<'source> Digits<'source> {
    /// Transmutes a `&[u8]` into a `&[Digit]` inside `Self`.
    ///
    /// # SAFETY
    ///
    /// You must guarantee that the input slice is only [`Digit`]s.
    pub const unsafe fn new_unchecked(digits: &'source [u8]) -> Self {
        Self(unsafe { transmute::<&[u8], &[Digit]>(digits) })
    }

    /// Parses the digits to a [`u128`]. Returns [`None`] iff the computation overflows.
    #[must_use]
    pub fn parse(self) -> Option<u128> {
        self.0.iter().copied().try_fold(0_u128, |n, item| {
            n.checked_mul(10)?.checked_add((item as u8 - b'0') as u128)
        })
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        unsafe { transmute(self.0) }
    }
}

/**
 * A byte in the format `x??` where `?` is an ASCII
 * hexadecimal digit.
 */
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

            // Maybe this can be improved.

            let high = match bytes.next().unwrap() {
                high @ b'0'..=b'9' => high - b'0',
                high @ b'A'..=b'F' => high - b'A' + 10,
                high @ b'a'..=b'f' => high - b'a' + 10,
                _ => unreachable_unchecked(),
            };

            let low = match bytes.next().unwrap() {
                low @ b'0'..=b'9' => low - b'0',
                low @ b'A'..=b'F' => low - b'A' + 10,
                low @ b'a'..=b'f' => low - b'a' + 10,
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

    pub fn parse(self) -> Option<TextStructure> {
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

        let bytes_structure = BytesStructure::from_iter(processed_bytes, len);

        Some(unsafe { TextStructure::new_unchecked(bytes_structure) })
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        self.0
    }
}

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

    #[must_use]
    pub fn parse(self) -> BytesStructure {
        todo!("parse bytes")
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
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
    pub fn parse(self) -> char {
        todo!("character source")
    }

    #[must_use]
    pub fn as_str(self) -> &'source str {
        self.0
    }
}
