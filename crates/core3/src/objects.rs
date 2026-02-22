use crate::{
    hash::{DataHash, StructureHash},
    u126::U126,
};

const ENCODE_ABSTRACT: u8 = 0;
const ENCODE_INLINE_DATA: u8 = 1;
const ENCODE_DATA_HASH: u8 = 2;
const ENCODE_STRUCTURE_HASH: u8 = 3;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    Abstract(U126) = ENCODE_ABSTRACT,
    InlineData(U126) = ENCODE_INLINE_DATA,
    DataHash(DataHash) = ENCODE_DATA_HASH,
    StructureHash(StructureHash) = ENCODE_STRUCTURE_HASH,
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct PackedId(u128);

impl PackedId {
    pub const M_TAG: Self = Self::pack(Id::Abstract(U126::new(1).unwrap()));
    pub const M_UNIQUE: Self = Self::pack(Id::Abstract(U126::new(2).unwrap()));
    pub const M_INFERRED: Self = Self::pack(Id::Abstract(U126::new(3).unwrap()));
    pub const M_REQUIRES: Self = Self::pack(Id::Abstract(U126::new(4).unwrap()));

    #[must_use]
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: u128) -> Self {
        Self(value)
    }

    #[must_use]
    #[inline(always)]
    pub const fn pack(id: Id) -> Self {
        match id {
            Id::Abstract(id) => unsafe {
                Self::new_unchecked(((ENCODE_ABSTRACT as u128) << 126) | id.unwrap())
            },
            Id::InlineData(inline) => unsafe {
                Self::new_unchecked(((ENCODE_INLINE_DATA as u128) << 126) | inline.unwrap())
            },
            Id::DataHash(object_hash) => unsafe {
                Self::new_unchecked(
                    ((ENCODE_INLINE_DATA as u128) << 126) | object_hash.unwrap().unwrap(),
                )
            },
            Id::StructureHash(object_hash) => unsafe {
                Self::new_unchecked(
                    ((ENCODE_STRUCTURE_HASH as u128) << 126) | object_hash.unwrap().unwrap(),
                )
            },
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn unpack(self) -> Id {
        let value = self.0;
        let variant = (value >> 126) as u8;
        let content = U126::new(value & U126::MAX.unwrap()).unwrap();

        match variant {
            ENCODE_ABSTRACT => Id::Abstract(content),
            ENCODE_INLINE_DATA => Id::InlineData(content),
            ENCODE_DATA_HASH => Id::DataHash(unsafe { DataHash::new_unchecked(content) }),
            ENCODE_STRUCTURE_HASH => {
                Id::StructureHash(unsafe { StructureHash::new_unchecked(content) })
            }
            _ => unreachable!(),
        }
    }
}
