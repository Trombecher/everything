use crate::ff;
use crate::objects::ObjectId;
use crate::values::{Color, ConstValue, DateTime, Duration, I120, Language, Schema, Value};
use std::hint::unreachable_unchecked;
use std::mem::transmute;
use std::num::NonZeroU64;

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Row([u64; 4]);

impl Row {
    pub fn encode(row: DecodedRow) -> Self {
        todo!()
    }

    pub const fn const_encode(row: ConstDecodedRow) -> Self {
        match row {
            ConstDecodedRow::Association(ObjectId(target_id), ObjectId(tag_id), value) => {
                let (a, b) = match value {
                    ConstValue::Unit => (ff::UNIT as u64, 0),
                    ConstValue::ObjectReference(ObjectId(id)) => {
                        (ff::OBJECT_REFERENCE as u64, id.get())
                    }
                    ConstValue::Schema(schema) => {
                        let (byte, other) = match schema {
                            Schema::Unit => (ff::UNIT, 0),
                            Schema::Integer => (ff::INTEGER, 0),
                            Schema::Float => (ff::FLOAT, 0),
                            Schema::Character => (ff::CHARACTER, 0),
                            Schema::Duration => (ff::DURATION, 0),
                            Schema::DateTime => (ff::DATE_TIME, 0),
                            Schema::ObjectReference(id) => (
                                ff::OBJECT_REFERENCE,
                                match id {
                                    None => 0,
                                    Some(ObjectId(id)) => id.get(),
                                },
                            ),
                            Schema::Schema => (ff::SCHEMA, 0),
                            Schema::Language => (ff::LANGUAGE, 0),
                            Schema::Url => (ff::URL, 0),
                            Schema::Color => (ff::COLOR, 0),
                            Schema::Email => (ff::EMAIL, 0),
                            Schema::Text => (ff::TEXT, 0),
                            Schema::Binary => (ff::BINARY, 0),
                            Schema::Encrypted => (ff::ENCRYPTED, 0),
                        };

                        (ff::SCHEMA as u64 | ((byte as u64) << 8), other)
                    }
                };

                Self([target_id.get(), tag_id.get(), a, b])
            }
            ConstDecodedRow::FreeListNextFreeIndex(index) => Self([0, index.get(), 0, 0]),
            ConstDecodedRow::FreeListEnd => Self([0; 4]),
        }
    }

    pub fn decode(self) -> Result<DecodedRow, ()> {
        match self.0 {
            [0, 0, 0, 0] => Ok(DecodedRow::FreeListEnd),
            [0, index, 0, 0] => Ok(DecodedRow::FreeListNextFreeIndex(
                NonZeroU64::new(index).unwrap(),
            )),
            [object_id, tag_id, a, b] if let Some(tag_id) = NonZeroU64::new(tag_id) => {
                let tag_id = ObjectId(tag_id);
                let object_id = ObjectId(NonZeroU64::new(object_id).unwrap());

                let value = match (a.to_le_bytes(), b) {
                    ([ff::UNIT, 0, 0, 0, 0, 0, 0, 0], 0) => Value::Unit,
                    ([ff::INTEGER, 0, 0, 0, 0, 0, 0, 0], b) => Value::Integer(b as _),
                    ([ff::FLOAT, 0, 0, 0, 0, 0, 0, 0], 0) => Value::Float(f64::from_bits(b)),
                    ([ff::CHARACTER, 0, 0, 0, remaining @ ..], 0) => {
                        Value::Character(char::from_u32(u32::from_le_bytes(remaining)).ok_or(())?)
                    }
                    ([ff::DURATION, bytes @ ..], b) => {
                        Value::Duration(Duration::from_nanos(I120::from_le_bytes(unsafe {
                            transmute((bytes, b.to_le_bytes()))
                        })))
                    }
                    ([ff::DATE_TIME, bytes @ ..], b) => {
                        Value::DateTime(DateTime::UNIX.const_add(Duration::from_nanos(
                            I120::from_le_bytes(unsafe { transmute((bytes, b.to_le_bytes())) }),
                        )))
                    }
                    ([ff::OBJECT_REFERENCE, 0, 0, 0, 0, 0, 0, 0], b) if b != 0 => {
                        Value::ObjectReference(ObjectId(NonZeroU64::new(b).unwrap()))
                    }
                    ([ff::SCHEMA, ff::OBJECT_REFERENCE, 0, 0, 0, 0, 0, 0], b) => {
                        Value::Schema(Schema::ObjectReference(NonZeroU64::new(b).map(ObjectId)))
                    }
                    ([ff::SCHEMA, byte, 0, 0, 0, 0, 0, 0], 0) => Value::Schema(match byte {
                        ff::UNIT => Schema::Unit,
                        ff::INTEGER => Schema::Integer,
                        ff::FLOAT => Schema::Float,
                        ff::CHARACTER => Schema::Character,
                        ff::DURATION => Schema::Duration,
                        ff::DATE_TIME => Schema::DateTime,
                        ff::SCHEMA => Schema::Schema,
                        ff::LANGUAGE => Schema::Language,
                        ff::URL => Schema::Url,
                        ff::COLOR => Schema::Color,
                        ff::EMAIL => Schema::Email,
                        ff::TEXT => Schema::Text,
                        ff::BINARY => Schema::Binary,
                        ff::ENCRYPTED => Schema::Encrypted,
                        ff::OBJECT_REFERENCE => unreachable!(),
                        _ => return Err(()),
                    }),
                    ([ff::LANGUAGE, 0, lang_bytes @ .., 0, 0, 0, 0], 0) => Value::Language(
                        Language::try_from(u16::from_le_bytes(lang_bytes)).map_err(|_| ())?,
                    ),
                    ([ff::COLOR, 0, 0, 0, bytes_c1 @ ..], b) => {
                        let l = f32::from_le_bytes(bytes_c1);
                        let a = f32::from_bits((b & u32::MAX as u64) as u32);
                        let b = f32::from_bits((b >> u32::BITS as u64) as u32);

                        Value::Color(Color { l, a, b })
                    }
                    ([ff::URL, ..], _) => todo!(),
                    ([ff::URL_MAX, ..], _) => todo!(),
                    ([ff::URL_REFERENCE, ..], _) => todo!(),
                    ([ff::EMAIL, ..], _) => todo!(),
                    ([ff::EMAIL_MAX, ..], _) => todo!(),
                    ([ff::EMAIL_REFERENCE, ..], _) => todo!(),
                    ([ff::TEXT, ..], _) => todo!(),
                    ([ff::TEXT_MAX, ..], _) => todo!(),
                    ([ff::TEXT_REFERENCE, ..], _) => todo!(),
                    ([ff::BINARY, ..], _) => todo!(),
                    ([ff::BINARY_MAX, ..], _) => todo!(),
                    ([ff::BINARY_REFERENCE, ..], _) => todo!(),
                    ([ff::ENCRYPTED, ..], _) => todo!(),
                    _ => todo!(),
                };

                Ok(DecodedRow::Association(object_id, tag_id, value))
            }
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum DecodedRow {
    Association(ObjectId, ObjectId, Value),

    /// Row matches `[0, index != 0, 0, 0]`
    FreeListNextFreeIndex(NonZeroU64),

    /// Row matches `[0, 0, 0, 0]`
    FreeListEnd,
}

impl DecodedRow {
    pub unsafe fn assume_association(&self) -> (ObjectId, ObjectId, Value) {
        match self {
            DecodedRow::Association(object_id, object_id1, value) => {
                (*object_id, *object_id1, value.clone())
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ConstDecodedRow {
    Association(ObjectId, ObjectId, ConstValue),
    FreeListNextFreeIndex(NonZeroU64),
    FreeListEnd,
}

impl ConstDecodedRow {
    pub const fn const_into_decoded_row(self) -> DecodedRow {
        match self {
            ConstDecodedRow::Association(o, t, v) => {
                DecodedRow::Association(o, t, v.const_into_value())
            }
            ConstDecodedRow::FreeListNextFreeIndex(i) => DecodedRow::FreeListNextFreeIndex(i),
            ConstDecodedRow::FreeListEnd => DecodedRow::FreeListEnd,
        }
    }
}

impl Into<DecodedRow> for ConstDecodedRow {
    fn into(self) -> DecodedRow {
        self.const_into_decoded_row()
    }
}
