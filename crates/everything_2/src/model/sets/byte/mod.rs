use std::fmt::Debug;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::model::{Atom, Construction, Object, SetApi, SetSpecialization};

#[cfg(test)]
mod tests;

/// A byte. Specialization for `{bit_0 on|off, ..., bit_7 on|off}`
/// (set of objects).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteSet(pub u8);

impl SetSpecialization for ByteSet {
    fn specialize_from(objects: &[Object]) -> Option<Self> {
        if objects.len() == 8
            && let Some((_seen_slots, result)) = objects.iter().try_fold(
                (ByteSet(0), ByteSet(0)),
                |(seen_slots, resulting_byte_set), object| {
                    let Object::Construction(Construction { constructor, value }) = object else {
                        return None;
                    };

                    let bit_slot = if let Object::Atom(atom) = constructor.as_ref() {
                        BitSlot::try_from(*atom).ok()
                    } else {
                        None
                    }?;

                    if seen_slots.bit(bit_slot) == Bit::One {
                        // We already consumed a bit in this slot.
                        return None;
                    }

                    let bit = if let Object::Atom(atom) = value.as_ref() {
                        Bit::try_from(*atom).ok()
                    } else {
                        None
                    }?;

                    Some((
                        // Add this slot to seen slots.
                        seen_slots.set_slot_to(bit_slot, Bit::One),
                        // Add bit to set.
                        resulting_byte_set.set_slot_to(bit_slot, bit),
                    ))
                },
            )
        {
            Some(result)
        } else {
            None
        }
    }
}

impl SetApi for ByteSet {
    fn size(&self) -> u64 {
        8
    }

    fn has(&self, object: &Object) -> bool {
        if let Object::Construction(Construction { constructor, value }) = object
            && let Object::Atom(atom_constructor) = constructor.as_ref()
            && let Object::Atom(atom_value) = value.as_ref()
            && let Ok(bit_slot) = BitSlot::try_from(*atom_constructor)
            && let Ok(bit) = Bit::try_from(*atom_value)
        {
            (self.0 >> u8::from(bit_slot)) & 1 == u8::from(bit)
        } else {
            false
        }
    }
}

impl IntoIterator for ByteSet {
    type Item = Object;
    type IntoIter = ByteSetValues;

    fn into_iter(self) -> Self::IntoIter {
        ByteSetValues {
            byte: self,
            next_slot: Some(BitSlot::Slot0),
        }
    }
}

impl ByteSet {
    #[must_use]
    pub fn from_bits(bits: [Bit; 8]) -> Self {
        // FIXME: optimize?
        Self(
            bits.iter()
                .copied()
                .rev()
                .map(u8::from)
                .fold(0_u8, |total, bit| (total << 1) | bit),
        )
    }

    #[must_use]
    pub fn set_slot_to(self, slot: BitSlot, bit: Bit) -> Self {
        let set_with_bit_cleared = self.0 & !(1 << u8::from(slot));

        Self(set_with_bit_cleared | (u8::from(bit) << u8::from(slot)))
    }

    #[inline]
    #[must_use]
    pub fn bit(self, slot: BitSlot) -> Bit {
        Bit::from(self.0 & (1 << u8::from(slot)) != 0)
    }
}

impl Debug for ByteSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{:02X}", self.0)
    }
}

/// A bit can either be [`Bit::Zero`] or [`Bit::One`].
/// This is not used as a specialization.
#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Bit {
    Zero = 0,
    One = 1,
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
pub struct ByteSetValues {
    byte: ByteSet,
    next_slot: Option<BitSlot>,
}

impl Iterator for ByteSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_slot {
            Some(slot) => {
                let bit = self.byte.bit(self.next_slot?);
                self.next_slot = slot.next();
                Some(Object::Construction(Construction {
                    constructor: Box::new(Object::Atom(slot.into())),
                    value: Box::new(Object::Atom(bit.into())),
                }))
            }
            None => None,
        }
    }
}
