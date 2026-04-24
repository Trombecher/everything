#[cfg(test)]
mod tests;

use core::{
    hint::{assert_unchecked, unreachable_unchecked},
    mem::transmute,
};

use alloc::sync::Arc;
use everything_structures::{Byte, BytesStructure, TextStructure};
use parser_tools::TokenLength;

#[derive(Clone, Copy, PartialEq, Debug)]
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
    Bytes(&'source str),

    /// A character literal `'ä'`.
    Character(&'source str),

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
            _ => 1,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Digits<'source>(pub &'source [Digit]);

impl From<Digits<'_>> for u128 {
    fn from(value: Digits) -> Self {
        value.0.iter().copied().fold(0_u128, |n, item| {
            n.saturating_mul(10)
                .saturating_add((item as u8 - b'0') as u128)
        })
    }
}

impl<'source> Digits<'source> {
    /// Transmutes a `&[u8]` into a `&[Digit]` inside `Self`.
    ///
    /// # SAFETY
    ///
    /// You must guarantee that the input slice is only ASCII digit bytes.
    pub const unsafe fn new_unchecked(digits: &'source [u8]) -> Self {
        Self(unsafe { transmute::<&[u8], &[Digit]>(digits) })
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
}

impl<'source> AsRef<str> for ByteSource<'source> {
    fn as_ref(&self) -> &'source str {
        self.0
    }
}

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

    pub fn real_byte_length(self) -> usize {
        let mut length = 0;
        let mut bytes = self.0.bytes();
        bytes.next();

        while let Some(byte) = bytes.next() {
            if byte == b'\\' {
                match bytes.next() {
                    Some(b'u' | b'x') => todo!(),
                    _ => {
                        length += 1;
                    }
                }
            } else if byte == b'"' {
                break;
            }

            length += 1;
        }

        length
    }

    pub fn parse(self) -> Option<TextStructure> {
        let len = self.real_byte_length();
        if len == 0 {
            return None;
        }

        let mut arc = Arc::new_uninit_slice(len);

        {
            // Populate Arc:

            let arc_ref = Arc::get_mut(&mut arc).unwrap();
            let mut bytes = arc_ref.iter_mut();

            for byte in self.0.bytes() {
                if byte == b'\\' {
                    todo!("escape")
                } else {
                    unsafe {
                        bytes.next().unwrap_unchecked().write(byte);
                    }
                }
            }
        }

        Some(unsafe {
            TextStructure::new_unchecked(BytesStructure::from_raw(arc.assume_init()).unwrap())
        })
    }
}
