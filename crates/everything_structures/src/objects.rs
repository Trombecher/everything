use std::{fmt, hash::Hash, num::NonZeroU128};

use crate::{Bit, Byte, structures::Structure};

pub type AbstractId = u128;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Abstract(AbstractId),
    Structure(Structure),
}

impl Object {
    /// The abstract object 0.
    pub const ZERO: Self = Self::Abstract(9);

    /// Denotes that the current object is a successor of some child number.
    pub const SUCCESSOR_OF: Self = Self::Abstract(10);

    /// The slot for the item value in an list.
    pub const LIST_ITEM: Self = Self::Abstract(5347);

    /// Denotes the rest of the list.
    pub const LIST_TAIL: Self = Self::Abstract(4353);

    /// Denotes the value of the character.
    pub const CODE_POINT: Self = Self::Abstract(6969);

    /// Denotes the bit zero/off/no.
    pub const BIT_0: Self = Self::Abstract(9843);

    /// Denotes the bit one/on/yes.
    pub const BIT_1: Self = Self::Abstract(6767);

    pub const BIT_SLOT_0: Self = Self::Abstract(5000);
    pub const BIT_SLOT_1: Self = Self::Abstract(5001);
    pub const BIT_SLOT_2: Self = Self::Abstract(5002);
    pub const BIT_SLOT_3: Self = Self::Abstract(5003);
    pub const BIT_SLOT_4: Self = Self::Abstract(5004);
    pub const BIT_SLOT_5: Self = Self::Abstract(5005);
    pub const BIT_SLOT_6: Self = Self::Abstract(5006);
    pub const BIT_SLOT_7: Self = Self::Abstract(5007);

    pub fn new_natural_number(n: u128) -> Self {
        match NonZeroU128::new(n) {
            None => Self::ZERO,
            Some(n) => Self::Structure(Structure::NaturalNumber(n)),
        }
    }

    pub fn exact_natural_number(&self) -> Option<u128> {
        if self == &Self::ZERO {
            Some(0)
        } else if let Self::Structure(Structure::NaturalNumber(n)) = self {
            Some(n.get())
        } else {
            None
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abstract(id) => {
                f.write_str("@")?;
                id.fmt(f)
            }
            Self::Structure(s) => s.fmt(f),
        }
    }
}

impl From<Structure> for Object {
    fn from(structure: Structure) -> Self {
        Self::Structure(structure)
    }
}

impl From<Bit> for Object {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => Self::BIT_0,
            Bit::One => Self::BIT_1,
        }
    }
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        Self::from(Structure::from(value))
    }
}

impl From<&[u8]> for Object {
    fn from(slice: &[u8]) -> Self {
        Self::Structure(Structure::from(slice))
    }
}

impl From<&str> for Object {
    fn from(slice: &str) -> Self {
        Self::Structure(Structure::from(slice))
    }
}

impl From<Byte> for Object {
    fn from(value: Byte) -> Self {
        Self::Structure(Structure::from(value))
    }
}
