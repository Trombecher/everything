//! Module handling abstract objects.

use std::fmt::Debug;

/// An (ideally) globally unique abstract object identifier.
/// **This number should not contain any semantic meaning.
/// Please use ULIDs or some other generator that is a
/// combination of randomness and time.**
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Abstract(pub u128);

impl Abstract {
    /// The abstract object 0.
    pub const ZERO: Self = Self(9);

    /// Denotes that the current object is a successor of some child number.
    pub const SUCCESSOR_OF: Self = Self(10);

    /// The slot for the item value in an list.
    pub const LIST_ITEM: Self = Self(5347);

    /// Denotes the rest of the list.
    pub const LIST_TAIL: Self = Self(4353);

    /// Denotes the value of the character.
    pub const CODE_POINT: Self = Self(6969);

    /// Denotes the bit zero/off/no.
    pub const BIT_0: Self = Self(9843);

    /// Denotes the bit one/on/yes.
    pub const BIT_1: Self = Self(6767);

    pub const BIT_SLOT_0: Self = Self(5000);
    pub const BIT_SLOT_1: Self = Self(5001);
    pub const BIT_SLOT_2: Self = Self(5002);
    pub const BIT_SLOT_3: Self = Self(5003);
    pub const BIT_SLOT_4: Self = Self(5004);
    pub const BIT_SLOT_5: Self = Self(5005);
    pub const BIT_SLOT_6: Self = Self(5006);
    pub const BIT_SLOT_7: Self = Self(5007);
}

impl Debug for Abstract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}
