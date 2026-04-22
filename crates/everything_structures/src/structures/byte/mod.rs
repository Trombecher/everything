#[cfg(test)]
mod tests;

use std::fmt::Debug;

use crate::{Object, Property};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Byte(pub u8);

impl Byte {
    pub fn bits(self) -> BitsOfByte {
        BitsOfByte {
            byte: self.0,
            next_byte_index: 0,
        }
    }

    #[inline]
    pub fn bit_0(self) -> Bit {
        Bit::from(self.0 & 1 != 0)
    }

    #[inline]
    pub fn bit_1(self) -> Bit {
        Bit::from(self.0 & 2 != 0)
    }

    #[inline]
    pub fn bit_2(self) -> Bit {
        Bit::from(self.0 & 4 != 0)
    }

    #[inline]
    pub fn bit_3(self) -> Bit {
        Bit::from(self.0 & 8 != 0)
    }

    #[inline]
    pub fn bit_4(self) -> Bit {
        Bit::from(self.0 & 16 != 0)
    }

    #[inline]
    pub fn bit_5(self) -> Bit {
        Bit::from(self.0 & 32 != 0)
    }

    #[inline]
    pub fn bit_6(self) -> Bit {
        Bit::from(self.0 & 64 != 0)
    }

    #[inline]
    pub fn bit_7(self) -> Bit {
        Bit::from(self.0 & 128 != 0)
    }

    pub fn has(self, property: &Property) -> bool {
        match property.tag {
            Object::BIT_SLOT_0 => property.value == Object::from(self.bit_0()),
            Object::BIT_SLOT_1 => property.value == Object::from(self.bit_1()),
            Object::BIT_SLOT_2 => property.value == Object::from(self.bit_2()),
            Object::BIT_SLOT_3 => property.value == Object::from(self.bit_3()),
            Object::BIT_SLOT_4 => property.value == Object::from(self.bit_4()),
            Object::BIT_SLOT_5 => property.value == Object::from(self.bit_5()),
            Object::BIT_SLOT_6 => property.value == Object::from(self.bit_6()),
            Object::BIT_SLOT_7 => property.value == Object::from(self.bit_7()),
            _ => false,
        }
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "b{:X}", self.0)
    }
}

#[derive(Clone)]
pub struct BitsOfByte {
    byte: u8,
    next_byte_index: u8,
}

impl Iterator for BitsOfByte {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_byte_index >= 8 {
            None
        } else {
            // Extract bit.
            Some(Bit::from(self.byte & (1_u8 << self.next_byte_index) != 0))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next_byte_index.min(8) as usize;
        (remaining, Some(remaining))
    }
}

/// A bit can either be [Bit::Zero] or [Bit::One].
/// This is not used as a specialization.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { Self::One } else { Self::Zero }
    }
}

impl TryFrom<&Object> for Bit {
    type Error = ();

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match object {
            &Object::BIT_0 => Ok(Self::Zero),
            &Object::BIT_1 => Ok(Self::One),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct ByteProperties(pub BitsOfByte);

impl Iterator for ByteProperties {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Object::from)
    }
}
