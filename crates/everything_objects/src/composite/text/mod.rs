#[cfg(test)]
mod tests;

use std::mem::take;

use crate::{Abstract, BytesComposite, Composite, Object, Property};

/// Represents an array of unicode characters.
///
/// The underlying [`BytesComposite`] will ALWAYS be valid UTF-8.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextComposite(BytesComposite);

impl TextComposite {
    /// Creates a new text Composite from the
    /// given string slice.
    #[must_use]
    pub fn new(s: &str) -> Option<Self> {
        BytesComposite::new(s.as_bytes()).map(Self)
    }

    /// Creates a new [TextComposite] without validating that
    /// the bytes are valid UTF-8.
    ///
    /// # Safety
    ///
    /// The bytes must contain valid UTF-8.
    pub const unsafe fn new_unchecked(bytes: BytesComposite) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub fn parts(&self) -> (char, &str) {
        let mut chars = self.as_ref().chars();
        let head = unsafe { chars.next().unwrap_unchecked() };

        (head, chars.as_str())
    }

    /// Creates a new text Composite from a head char and a tail string slice.
    pub fn from_parts(head: char, tail: &str) -> Self {
        let mut char_utf8 = [0_u8; 4];
        head.encode_utf8(&mut char_utf8);

        Self(BytesComposite::from_parts(&char_utf8[..head.len_utf8()], tail.as_bytes()).unwrap())
    }

    /// Iterates over the head and tail values.
    pub fn properties(&self) -> TextCompositeProperties {
        TextCompositeProperties::TailAndItem(self.clone())
    }

    pub fn has(&self, tag: &Object, value: &Object) -> bool {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => {
                value == &Object::Composite(Composite::Character(self.parts().0))
            }
            Object::Abstract(Abstract::LIST_TAIL)
                if let Object::Composite(Composite::Empty) = value =>
            {
                // Tail is empty
                self.as_ref().len() == 1
            }
            Object::Abstract(Abstract::LIST_TAIL)
                if let Object::Composite(Composite::Text(tail)) = &value =>
            {
                // Tail is non empty
                tail.as_ref() == self.parts().1
            }
            _ => false,
        }
    }

    pub fn values(&self, tag: Object) -> TextCompositeValues {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => TextCompositeValues::ListItem(self.parts().0),
            Object::Abstract(Abstract::LIST_TAIL) => TextCompositeValues::Tail(self.clone()),
            _ => TextCompositeValues::None,
        }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &BytesComposite {
        &self.0
    }

    #[must_use]
    pub fn into_bytes(self) -> BytesComposite {
        self.0
    }
}

impl AsRef<str> for TextComposite {
    fn as_ref(&self) -> &str {
        // SAFETY: .0 always contains valid UTF-8 and it cannot be changed.
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl std::fmt::Debug for TextComposite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct MaybeEmptyTextComposite(pub Option<TextComposite>);

impl AsRef<str> for MaybeEmptyTextComposite {
    fn as_ref(&self) -> &str {
        match &self.0 {
            None => "",
            Some(text) => text.as_ref(),
        }
    }
}

impl TryFrom<&Composite> for MaybeEmptyTextComposite {
    type Error = ();

    fn try_from(value: &Composite) -> Result<Self, Self::Error> {
        match value {
            Composite::Empty => Ok(Self(None)),
            Composite::Text(text) => Ok(Self(Some(text.clone()))),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum TextCompositeProperties {
    #[default]
    None,
    Tail(TextComposite),
    TailAndItem(TextComposite),
}

impl Iterator for TextCompositeProperties {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match take(self) {
            Self::TailAndItem(text) => {
                let (item, _) = text.parts();
                *self = Self::Tail(text);

                Some(Property::new_list_item(Object::from(Composite::from(item))))
            }
            Self::Tail(text) => {
                let (_, tail) = text.parts();

                Some(Property::new_list_tail(Object::from(Composite::from(tail))))
            }
            Self::None => None,
        }
    }
}

#[derive(Clone, Default)]
pub enum TextCompositeValues {
    #[default]
    None,
    ListItem(char),
    Tail(TextComposite),
}

impl Iterator for TextCompositeValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match take(self) {
            Self::None => None,
            Self::ListItem(c) => Some(Object::Composite(Composite::from(c))),
            Self::Tail(text) => {
                let (_, tail) = text.parts();
                Some(Object::Composite(Composite::from(tail)))
            }
        }
    }
}
