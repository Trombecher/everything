use std::{fmt::Debug, hint::unreachable_unchecked};

use crate::{BytesStructure, Object};

/// Represents an array of unicode characters.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStructure(BytesStructure);

impl TextStructure {
    /// Creates a new text structure from the
    /// given string slice.
    #[must_use]
    pub fn new(s: &str) -> Option<Self> {
        BytesStructure::new(s.as_bytes()).map(Self)
    }

    #[must_use]
    pub fn parts<'text>(&'text self) -> (char, &'text str) {
        let mut chars = self.as_ref().chars();
        let head = chars.next().unwrap(); // FIXME: this could be unchecked.

        (head, chars.as_str())
    }

    /// Creates a new text structure from a head char and a tail string slice.
    pub fn from_parts(head: char, tail: &str) -> Self {
        let mut char_utf8 = [0_u8; 4];
        head.encode_utf8(&mut char_utf8);

        Self(BytesStructure::from_parts(&char_utf8[..head.len_utf8()], tail.as_bytes()).unwrap())
    }

    /// Iterates over the head and tail values.
    pub fn properties<'text>(&'text self) -> TextStructureProperties<'text> {
        let (head, tail) = self.parts();

        TextStructureProperties {
            head,
            tail,
            index: 0,
        }
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

#[derive(Clone)]
pub struct TextStructureProperties<'text> {
    head: char,
    tail: &'text str,
    index: u8,
}

impl<'text> Iterator for TextStructureProperties<'text> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(Object::from(self.head))
            }
            1 => {
                self.index += 1;
                Some(Object::from(self.tail))
            }
            2 => None,
            _ => unsafe {
                // SAFETY: this will never be reached because at
                // `index == 2`, index is not incremented.
                unreachable_unchecked()
            },
        }
    }
}
