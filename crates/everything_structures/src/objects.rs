use std::{fmt, hash::Hash, num::NonZeroU128};

use crate::{Abstract, Bit, BitSlot, Byte, structures::Structure};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Abstract(Abstract),
    Structure(Structure),
}

impl Object {
    pub fn new_natural_number(n: u128) -> Self {
        match NonZeroU128::new(n) {
            None => Self::Abstract(Abstract::ZERO),
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

impl From<BitSlot> for Object {
    fn from(value: BitSlot) -> Self {
        match value {
            BitSlot::Slot0 => Object::BIT_SLOT_0,
            BitSlot::Slot1 => Object::BIT_SLOT_0,
            BitSlot::Slot2 => Object::BIT_SLOT_0,
            BitSlot::Slot3 => todo!(),
            BitSlot::Slot4 => todo!(),
            BitSlot::Slot5 => todo!(),
            BitSlot::Slot6 => todo!(),
            BitSlot::Slot7 => todo!(),
        }
    }
}
