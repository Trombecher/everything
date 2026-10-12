use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::model::{Atom, Construction, Object};

#[cfg(test)]
mod tests;

/// A byte. Specialization for `{bit_0 on|off, ..., bit_7 on|off}`
/// (set of objects).
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Byte(pub u8);

impl Byte {
    #[must_use]
    pub fn from_bits(bits: [Bit; 8]) -> Self {
        // FIXME: optimize?
        Self(
            bits.iter()
                .copied()
                .rev()
                .map(|bit| match bit {
                    Bit::One => 1,
                    Bit::Zero => 0_u8,
                })
                .fold(0_u8, |total, bit| (total << 1) | bit),
        )
    }

    #[inline]
    #[must_use]
    pub fn bit(self, slot: BitSlot) -> Bit {
        Bit::from(self.0 & (1 << (slot as u8)) != 0)
    }

    #[must_use]
    pub fn has(self, object: &Object) -> bool {
        if let Object::Construction(Construction { constructor, value }) = object
            && let Object::Atom(atom_constructor) = constructor.as_ref()
            && let Object::Atom(atom_value) = value.as_ref()
            && let Ok(bit_slot) = BitSlot::try_from(*atom_constructor)
            && let Ok(bit) = Bit::try_from(*atom_value)
        {
            (self.0 >> u8::from(bit_slot)) & 1 == bit as u8
        } else {
            false
        }
    }

    #[must_use]
    pub fn values(self) -> ByteValues {
        ByteValues {
            byte: self,
            next_slot: Some(BitSlot::Slot0),
        }
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{:02X}", self.0)
    }
}

/// A bit can either be [`Bit::Zero`] or [`Bit::One`].
/// This is not used as a specialization.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bit {
    Zero,
    One,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { Self::One } else { Self::Zero }
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        value == Bit::One
    }
}

impl TryFrom<Atom> for Bit {
    type Error = ();

    fn try_from(atom: Atom) -> Result<Self, Self::Error> {
        match atom {
            Atom::BIT_OFF => Ok(Self::Zero),
            Atom::BIT_ON => Ok(Self::One),
            _ => Err(()),
        }
    }
}

impl From<Bit> for Atom {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => Self::BIT_OFF,
            Bit::One => Self::BIT_ON,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BitSlot {
    Slot0 = 0,
    Slot1 = 1,
    Slot2 = 2,
    Slot3 = 3,
    Slot4 = 4,
    Slot5 = 5,
    Slot6 = 6,
    Slot7 = 7,
}

impl BitSlot {
    #[must_use]
    pub const fn next(self) -> Option<BitSlot> {
        // I hope this gets lowered to an `inc`.

        match self {
            BitSlot::Slot0 => Some(BitSlot::Slot1),
            BitSlot::Slot1 => Some(BitSlot::Slot2),
            BitSlot::Slot2 => Some(BitSlot::Slot3),
            BitSlot::Slot3 => Some(BitSlot::Slot4),
            BitSlot::Slot4 => Some(BitSlot::Slot5),
            BitSlot::Slot5 => Some(BitSlot::Slot6),
            BitSlot::Slot6 => Some(BitSlot::Slot7),
            BitSlot::Slot7 => None,
        }
    }
}

impl TryFrom<Atom> for BitSlot {
    type Error = ();

    fn try_from(a: Atom) -> Result<Self, Self::Error> {
        match a {
            Atom::BIT_SLOT_0 => Ok(Self::Slot0),
            Atom::BIT_SLOT_1 => Ok(Self::Slot1),
            Atom::BIT_SLOT_2 => Ok(Self::Slot2),
            Atom::BIT_SLOT_3 => Ok(Self::Slot3),
            Atom::BIT_SLOT_4 => Ok(Self::Slot4),
            Atom::BIT_SLOT_5 => Ok(Self::Slot5),
            Atom::BIT_SLOT_6 => Ok(Self::Slot6),
            Atom::BIT_SLOT_7 => Ok(Self::Slot7),
            _ => Err(()),
        }
    }
}

impl From<BitSlot> for Atom {
    fn from(value: BitSlot) -> Self {
        match value {
            BitSlot::Slot0 => Self::BIT_SLOT_0,
            BitSlot::Slot1 => Self::BIT_SLOT_1,
            BitSlot::Slot2 => Self::BIT_SLOT_2,
            BitSlot::Slot3 => Self::BIT_SLOT_3,
            BitSlot::Slot4 => Self::BIT_SLOT_4,
            BitSlot::Slot5 => Self::BIT_SLOT_5,
            BitSlot::Slot6 => Self::BIT_SLOT_6,
            BitSlot::Slot7 => Self::BIT_SLOT_7,
        }
    }
}

#[derive(Clone)]
pub struct ByteValues {
    byte: Byte,
    next_slot: Option<BitSlot>,
}

impl Iterator for ByteValues {
    type Item = Construction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_slot {
            Some(slot) => {
                let bit = self.byte.bit(self.next_slot?);
                self.next_slot = slot.next();
                Some(Construction {
                    constructor: Box::new(Object::Atom(slot.into())),
                    value: Box::new(Object::Atom(bit.into())),
                })
            }
            None => None,
        }
    }
}
