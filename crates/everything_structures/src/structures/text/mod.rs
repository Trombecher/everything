#[cfg(test)]
mod tests;

use std::{fmt::Debug, hint::unreachable_unchecked};

use crate::{Abstract, BytesStructure, Object, Property, Structure};

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
    pub fn parts(&self) -> (char, &str) {
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

    pub fn has(&self, tag: &Object, value: &Object) -> bool {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => {
                value == &Object::Structure(Structure::Character(self.parts().0))
            }
            Object::Abstract(Abstract::LIST_TAIL)
                if let Object::Structure(Structure::Empty) = value =>
            {
                // Tail is empty
                self.as_ref().len() == 1
            }
            Object::Abstract(Abstract::LIST_TAIL)
                if let Object::Structure(Structure::Text(tail)) = &value =>
            {
                // Tail is non empty
                tail.as_ref() == self.parts().1
            }
            _ => false,
        }
    }

    pub fn values<'text>(&'text self, tag: Object) -> TextStructureValues<'text> {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => TextStructureValues::ListItem(self.parts().0),
            Object::Abstract(Abstract::LIST_TAIL) => TextStructureValues::Tail(self.parts().1),
            _ => TextStructureValues::None,
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
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: test order

        match self.index {
            0 => {
                self.index += 1;
                Some(Property::list_tail(Object::from(Structure::from(
                    self.head,
                ))))
            }
            1 => {
                self.index += 1;
                Some(Property::list_item(Object::from(Structure::from(
                    self.tail,
                ))))
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

#[derive(Clone)]
pub enum TextStructureValues<'text> {
    None,
    ListItem(char),
    Tail(&'text str),
}

impl Iterator for TextStructureValues<'_> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::ListItem(c) => {
                let c = *c;
                *self = Self::None;

                Some(Object::Structure(Structure::from(c)))
            }
            Self::Tail(bytes) => {
                let bytes = *bytes;
                *self = Self::None;

                Some(Object::Structure(Structure::from(bytes)))
            }
        }
    }
}
