use std::fmt::{Debug, Write};

use ulid::Ulid;

use crate::{objects, ranged::RangedU128};

/// A reference to an object. It has the following bit structure:
///
/// * `0<object_id>` -- A reference to an object. There are 127
/// * `10<inline_data>` -- A reference to a object identified by data.
///    The id can store up to 126 bits inline.
/// * `11<indirect_data_id>` -- A hash of the data. The data is stored externally.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct PackedId(pub u128);

impl PackedId {
    #[must_use]
    #[inline(always)]
    pub const fn unpack(self) -> Id {
        // These unwraps will get compiled out in release.

        if self.0 <= MAX_OBJECT_ID {
            return Id::Object(ObjectId(RangedU128::new(self.0).unwrap()));
        }

        let id = self.0 - MAX_OBJECT_ID - 1;

        if id <= MAX_INLINE_DATA {
            return Id::InlineData(RangedU128::new(id).unwrap());
        }

        Id::IndirectDataId(RangedU128::new(id - MAX_INLINE_DATA - 1).unwrap())
    }

    #[must_use]
    #[inline(always)]
    pub const fn pack(id: Id) -> Self {
        match id {
            Id::Object(v) => Self(v.0.get()),
            Id::InlineData(data) => Self(MAX_OBJECT_ID + 1 + data.get()),
            Id::IndirectDataId(id) => Self(MAX_OBJECT_ID + 1 + MAX_INLINE_DATA + 1 + id.get()),
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn next_object() -> Self {
        Self(Ulid::new().0 >> 1)
    }
}

impl Into<Id> for PackedId {
    fn into(self) -> Id {
        self.unpack()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct ObjectId(pub RangedU128<0, MAX_OBJECT_ID>);

impl Debug for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.get() == objects::META_TAG.0 {
            f.write_str("#tag")
        } else {
            self.0.fmt(f)
        }
    }
}

pub type InlineData = RangedU128<0, MAX_INLINE_DATA>;
pub type IndirectDataId = RangedU128<0, MAX_INDIRECT_DATA_ID>;

pub const MAX_OBJECT_ID: u128 = u128::MAX >> 1;
pub const MAX_INLINE_DATA: u128 = u128::MAX >> 2;
pub const MAX_INDIRECT_DATA_ID: u128 = u128::MAX >> 2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Id {
    Object(ObjectId),
    InlineData(InlineData),
    IndirectDataId(IndirectDataId),
}
