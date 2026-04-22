use std::fmt::Debug;

use crate::BytesStructure;

/// Represents an array of unicode characters.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStructure(BytesStructure);

impl TextStructure {
    /// Creates a new text structure from the
    /// given string slice.
    pub fn new(s: &str) -> Option<Self> {
        BytesStructure::new(s.as_bytes()).map(Self)
    }

    /// Creates a new text structure from a head char and a tail string slice.
    pub fn from_tail(head: char, tail: &str) -> Self {
        let mut char_utf8 = [0_u8; 4];
        head.encode_utf8(&mut char_utf8);

        Self(BytesStructure::from_parts(&char_utf8[..head.len_utf8()], tail.as_bytes()).unwrap())
    }
}

impl AsRef<str> for TextStructure {
    fn as_ref(&self) -> &str {
        // SAFETY: .0 always contains valid UTF-8 and it cannot be changed.
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl Debug for TextStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_ref())
    }
}
