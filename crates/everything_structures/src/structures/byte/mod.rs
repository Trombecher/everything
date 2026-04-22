#[cfg(test)]
mod tests;

use crate::{Abstract, Object, Property};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Byte(pub u8);

impl Byte {
    #[inline]
    #[must_use]
    pub fn bit(self, slot: BitSlot) -> Bit {
        Bit::from(self.0 & (1 << (slot as u8)) != 0)
    }

    pub fn has(self, tag: &Object, value: &Object) -> bool {
        if let Object::Abstract(a) = tag
            && let Ok(slot) = BitSlot::try_from(*a)
            && let Object::Abstract(value) = value
            && *value == self.bit(slot).into()
        {
            true
        } else {
            false
        }
    }

    pub fn properties(self) -> ByteProperties {
        ByteProperties {
            byte: self,
            next_slot: Some(BitSlot::Slot0),
        }
    }

    pub fn values(self, tag: Object) -> ByteValues {
        if let Object::Abstract(tag) = tag
            && let Ok(slot) = BitSlot::try_from(tag)
        {
            ByteValues(Some(self.bit(slot)))
        } else {
            ByteValues(None)
        }
    }

    pub fn tags(self, value: Object) -> ByteTags {
        match value {
            Object::Abstract(Abstract::BIT_0) => ByteTags {
                // We invert because we ask for zeroes.
                slots: !self.0,
            },
            Object::Abstract(Abstract::BIT_1) => ByteTags { slots: self.0 },
            _ => ByteTags { slots: 0 },
        }
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "b{:X}", self.0)
    }
}

/// A bit can either be [Bit::Zero] or [Bit::One].
/// This is not used as a specialization.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    #[must_use]
    pub const fn to_abstract(self) -> Abstract {
        // FIXME: move into From when const traits are stabilized.

        match self {
            Self::Zero => Abstract::BIT_0,
            Self::One => Abstract::BIT_1,
        }
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { Self::One } else { Self::Zero }
    }
}

impl TryFrom<Abstract> for Bit {
    type Error = ();

    fn try_from(object: Abstract) -> Result<Self, Self::Error> {
        match object {
            Abstract::BIT_0 => Ok(Self::Zero),
            Abstract::BIT_1 => Ok(Self::One),
            _ => Err(()),
        }
    }
}

impl From<Bit> for Abstract {
    fn from(value: Bit) -> Self {
        value.to_abstract()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
}

impl BitSlot {
    #[must_use]
    pub const fn to_abstract(self) -> Abstract {
        // FIXME: move into From when const traits are stabilized.

        match self {
            Self::Slot0 => Abstract::BIT_SLOT_0,
            Self::Slot1 => Abstract::BIT_SLOT_1,
            Self::Slot2 => Abstract::BIT_SLOT_2,
            Self::Slot3 => Abstract::BIT_SLOT_3,
            Self::Slot4 => Abstract::BIT_SLOT_4,
            Self::Slot5 => Abstract::BIT_SLOT_5,
            Self::Slot6 => Abstract::BIT_SLOT_6,
            Self::Slot7 => Abstract::BIT_SLOT_7,
        }
    }

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

impl TryFrom<u8> for BitSlot {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Slot0),
            1 => Ok(Self::Slot1),
            2 => Ok(Self::Slot2),
            3 => Ok(Self::Slot3),
            4 => Ok(Self::Slot4),
            5 => Ok(Self::Slot5),
            6 => Ok(Self::Slot6),
            7 => Ok(Self::Slot7),
            _ => Err(()),
        }
    }
}

impl TryFrom<Abstract> for BitSlot {
    type Error = ();

    fn try_from(a: Abstract) -> Result<Self, Self::Error> {
        match a {
            Abstract::BIT_SLOT_0 => Ok(Self::Slot0),
            Abstract::BIT_SLOT_1 => Ok(Self::Slot1),
            Abstract::BIT_SLOT_2 => Ok(Self::Slot2),
            Abstract::BIT_SLOT_3 => Ok(Self::Slot3),
            Abstract::BIT_SLOT_4 => Ok(Self::Slot4),
            Abstract::BIT_SLOT_5 => Ok(Self::Slot5),
            Abstract::BIT_SLOT_6 => Ok(Self::Slot6),
            Abstract::BIT_SLOT_7 => Ok(Self::Slot7),
            _ => Err(()),
        }
    }
}

impl From<BitSlot> for Abstract {
    fn from(value: BitSlot) -> Self {
        value.to_abstract()
    }
}

#[derive(Clone)]
pub struct ByteProperties {
    byte: Byte,
    next_slot: Option<BitSlot>,
}

impl Iterator for ByteProperties {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_slot {
            Some(slot) => {
                let bit = self.byte.bit(self.next_slot?);
                self.next_slot = slot.next();
                Some(Property::bit_slot(slot, bit))
            }
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct ByteValues(Option<Bit>);

impl Iterator for ByteValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Object::Abstract(Abstract::from(self.0.take()?)))
    }
}

#[derive(PartialEq, Debug)]
pub struct ByteTags {
    /// The ones will be yielded.
    slots: u8,
}

impl Iterator for ByteTags {
    type Item = BitSlot;

    fn next(&mut self) -> Option<Self::Item> {
        let slot_index = self.slots.trailing_zeros();
        self.slots &= !1_u8.unbounded_shl(slot_index);

        BitSlot::try_from(slot_index as u8).ok()
    }
}
