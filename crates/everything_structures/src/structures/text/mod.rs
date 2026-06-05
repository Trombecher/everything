#[cfg(test)]
mod tests;

use std::mem::take;

use crate::{Abstract, BytesStructure, Object, Property, Structure};

/// Represents an array of unicode characters.
///
/// The underlying [`BytesStructure`] will ALWAYS be valid UTF-8.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStructure(BytesStructure);

impl TextStructure {
    /// Creates a new text structure from the
    /// given string slice.
    #[must_use]
    pub fn new(s: &str) -> Option<Self> {
        BytesStructure::new(s.as_bytes()).map(Self)
    }

    /// Creates a new [TextStructure] without validating that
    /// the bytes are valid UTF-8.
    ///
    /// # Safety
    ///
    /// The bytes must contain valid UTF-8.
    pub const unsafe fn new_unchecked(bytes: BytesStructure) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub fn parts(&self) -> (char, &str) {
        let mut chars = self.as_ref().chars();
        let head = unsafe { chars.next().unwrap_unchecked() };

        (head, chars.as_str())
    }

    /// Creates a new text structure from a head char and a tail string slice.
    pub fn from_parts(head: char, tail: &str) -> Self {
        let mut char_utf8 = [0_u8; 4];
        head.encode_utf8(&mut char_utf8);

        Self(BytesStructure::from_parts(&char_utf8[..head.len_utf8()], tail.as_bytes()).unwrap())
    }

    /// Iterates over the head and tail values.
    pub fn properties(&self) -> TextStructureProperties {
        TextStructureProperties::TailAndItem(self.clone())
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

    pub fn values(&self, tag: Object) -> TextStructureValues {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => TextStructureValues::ListItem(self.parts().0),
            Object::Abstract(Abstract::LIST_TAIL) => TextStructureValues::Tail(self.clone()),
            _ => TextStructureValues::None,
        }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &BytesStructure {
        &self.0
    }

    #[must_use]
    pub fn into_bytes(self) -> BytesStructure {
        self.0
    }
}

impl AsRef<str> for TextStructure {
    fn as_ref(&self) -> &str {
        // SAFETY: .0 always contains valid UTF-8 and it cannot be changed.
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl std::fmt::Debug for TextStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct MaybeEmptyTextStructure(pub Option<TextStructure>);

impl AsRef<str> for MaybeEmptyTextStructure {
    fn as_ref(&self) -> &str {
        match &self.0 {
            None => "",
            Some(text) => text.as_ref(),
        }
    }
}

impl TryFrom<&Structure> for MaybeEmptyTextStructure {
    type Error = ();

    fn try_from(value: &Structure) -> Result<Self, Self::Error> {
        match value {
            Structure::Empty => Ok(Self(None)),
            Structure::Text(text) => Ok(Self(Some(text.clone()))),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum TextStructureProperties {
    #[default]
    None,
    Tail(TextStructure),
    TailAndItem(TextStructure),
}

impl Iterator for TextStructureProperties {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match take(self) {
            Self::TailAndItem(text) => {
                let (item, _) = text.parts();
                *self = Self::Tail(text);

                Some(Property::new_list_item(Object::from(Structure::from(item))))
            }
            Self::Tail(text) => {
                let (_, tail) = text.parts();

                Some(Property::new_list_tail(Object::from(Structure::from(tail))))
            }
            Self::None => None,
        }
    }
}

#[derive(Clone, Default)]
pub enum TextStructureValues {
    #[default]
    None,
    ListItem(char),
    Tail(TextStructure),
}

impl Iterator for TextStructureValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match take(self) {
            Self::None => None,
            Self::ListItem(c) => Some(Object::Structure(Structure::from(c))),
            Self::Tail(text) => {
                let (_, tail) = text.parts();
                Some(Object::Structure(Structure::from(tail)))
            }
        }
    }
}
