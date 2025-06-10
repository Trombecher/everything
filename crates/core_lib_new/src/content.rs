use crate::ff;
use crate::objects::ObjectId;
use std::num::NonZeroU64;
use crate::values::DecodedValue;

#[repr(C, align(4096))]
pub struct Content {
    magic_bytes: [u8; 12],
    version: u32,
    entries: [Row],
}

impl Content {}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Row([u64; 4]);

impl Row {
    pub const fn encode(row: DecodedRow) -> Self {
        match row {
            DecodedRow::Association(target_id, tag_id, value) => {
                let (a, b) = match value {
                    None => (ff::NO_VALUE as u64, 0),
                    Some(DecodedValue::Float(f)) => (ff::FLOAT as u64, f.to_bits()),
                    Some(DecodedValue::Integer(i)) => (ff::INTEGER as u64, i as _),
                    Some(DecodedValue::ObjectReference(id)) => (
                        ff::OBJECT_REFERENCE as u64,
                        match id {
                            None => 0,
                            Some(x) => x.get(),
                        },
                    ),
                    Some(DecodedValue::Character(c)) => {
                        (ff::CHAR as u64 | (c as u32 as u64) << 32, 0)
                    }
                };

                Self([target_id.get(), tag_id.get(), a, b])
            }
            DecodedRow::FreeListNextFreeIndex(index) => Self([0, index.get(), 0, 0]),
            DecodedRow::FreeListEnd => Self([0; 4]),
        }
    }

    pub const fn decode(self) -> Result<DecodedRow, ()> {
        match self.0 {
            [0, 0, 0, 0] => Ok(DecodedRow::FreeListEnd),
            [0, index, 0, 0] => Ok(DecodedRow::FreeListNextFreeIndex(
                NonZeroU64::new(index).unwrap(),
            )),
            [object_id, tag_id, a, b] if let Some(tag_id) = NonZeroU64::new(tag_id) => {
                let object_id = NonZeroU64::new(object_id).unwrap();

                let value = match a.to_le_bytes() {
                    [ff::NO_VALUE, ..] => None,
                    [ff::INTEGER, ..] => Some(DecodedValue::Integer(b as _)),
                    [ff::FLOAT, ..] => Some(DecodedValue::Float(f64::from_bits(b))),
                    [ff::CHAR, .., a, b, c, d] => Some(DecodedValue::Character(
                        match char::from_u32(u32::from_le_bytes([a, b, c, d])) {
                            Some(char) => char,
                            None => return Err(()),
                        },
                    )),
                    [ff::OBJECT_REFERENCE, ..] => {
                        Some(DecodedValue::ObjectReference(NonZeroU64::new(b)))
                    }
                    _ => return Err(()),
                };

                Ok(DecodedRow::Association(object_id, tag_id, value))
            }
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum DecodedRow {
    Association(ObjectId, ObjectId, DecodedValue),

    /// Row matches `[0, index != 0, 0, 0]`
    FreeListNextFreeIndex(NonZeroU64),

    /// Row matches `[0, 0, 0, 0]`
    FreeListEnd,
}