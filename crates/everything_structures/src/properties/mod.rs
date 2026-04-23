#[cfg(test)]
mod tests;

use std::hash::Hash;

use crate::{Abstract, Bit, BitSlot, objects::Object};

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
    /// (SUCCESSOR_OF, n)
    /// ```
    #[must_use]
    pub const fn new_successor_of(n: u128) -> Self {
        Self {
            tag: Object::Abstract(Abstract::SUCCESSOR_OF),
            value: Object::new_natural_number(n),
        }
    }

    #[must_use]
    pub const fn new_character(c: char) -> Self {
        Self {
            tag: Object::Abstract(Abstract::CODE_POINT),
            value: Object::new_natural_number(c as u32 as u128),
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
