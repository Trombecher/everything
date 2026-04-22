use std::{fmt, hash::Hash, num::NonZeroU128};

use crate::objects::Object;

/// A property has a tag and a value, both objects.
#[derive(Clone, PartialEq, Eq, Ord, Hash, PartialOrd)]
pub struct Property {
    pub tag: Object,
    pub value: Object,
}

impl Property {
    /// Creates a `successor_of` property for the given `n`.
    /// It will look like this:
    ///
    /// ```plain
    /// (SUCCESSOR_OF, n - 1)
    /// ```
    pub fn successor_of(n: NonZeroU128) -> Self {
        Self {
            tag: Object::SUCCESSOR_OF,
            value: Object::new_natural_number(n.get() - 1),
        }
    }

    pub fn character(c: char) -> Self {
        Self {
            tag: Object::CODE_POINT,
            value: Object::new_natural_number(c as u32 as u128),
        }
    }
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
}
