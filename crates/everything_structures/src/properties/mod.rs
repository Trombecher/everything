#[cfg(test)]
mod tests;

use std::{hash::Hash, num::NonZeroI128};

use crate::{Abstract, Bit, BitSlot, objects::Object};

/// A property has a tag and a value, both objects.
#[derive(Clone, PartialEq, Eq, Ord, Hash, PartialOrd)]
pub struct Property {
    pub tag: Object,
    pub value: Object,
}

impl Property {
    /// Creates a `SUCCESSOR_OF` property for the given `object`.
    /// It will look like this.
    ///
    /// ```plain
    /// (@SUCCESSOR_OF, object)
    /// ```
    #[must_use]
    pub const fn new_successor_of(object: Object) -> Self {
        Self {
            tag: Object::Abstract(Abstract::SUCCESSOR_OF),
            value: object,
        }
    }

    /// Creates a `PREDECESSOR_OF` property for the given `object`.
    /// It will look like this.
    ///
    /// ```plain
    /// (@PREDECESSOR_OF, object)
    /// ```
    #[must_use]
    pub const fn new_predecessor_of(object: Object) -> Self {
        Self {
            tag: Object::Abstract(Abstract::PREDECESSOR_OF),
            value: object,
        }
    }

    /// Calls either [`Self::new_successor_of`] or
    /// [`Self::new_predecessor_of`], depending on whether
    /// the given number is positive or negative (respectively).
    ///
    /// This function returns a [`Property`] which (alone as
    /// a structure) would represent the given `n`. So it
    /// decrements (if positive) and increments `n`
    /// (if negative) before converting it to an object.
    pub const fn new_integer(n: NonZeroI128) -> Self {
        let n = n.get();

        if n > 0 {
            Self::new_successor_of(Object::new_integer(n - 1))
        } else if n < 0 {
            Self::new_predecessor_of(Object::new_integer(n + 1))
        } else {
            unreachable!()
        }
    }

    #[must_use]
    pub const fn new_character(c: char) -> Self {
        Self {
            tag: Object::Abstract(Abstract::CODE_POINT),
            value: Object::new_integer(c as u32 as i128),
        }
    }

    #[must_use]
    pub const fn new_list_item(item: Object) -> Self {
        Self {
            tag: Object::Abstract(Abstract::LIST_ITEM),
            value: item,
        }
    }

    #[must_use]
    pub const fn new_list_tail(tail: Object) -> Self {
        Self {
            tag: Object::Abstract(Abstract::LIST_TAIL),
            value: tail,
        }
    }

    #[must_use]
    pub const fn new_bit_slot(slot: BitSlot, bit: Bit) -> Self {
        Self {
            tag: Object::Abstract(slot.to_abstract()),
            value: Object::Abstract(bit.to_abstract()),
        }
    }
}

impl std::fmt::Debug for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
}
