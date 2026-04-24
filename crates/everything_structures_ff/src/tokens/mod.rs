#[cfg(test)]
mod tests;

use core::{
    hint::{assert_unchecked, unreachable_unchecked},
    mem::transmute,
};

use everything_structures::Byte;
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
    Text(&'source str),

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
