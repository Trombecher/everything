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
                    ConstValue::Unit => (ff::VALUE_UNIT as u64, 0),
                    ConstValue::ObjectReference(ObjectId(id)) => {
                        (ff::VALUE_OBJECT_REFERENCE as u64, id.get())
                    }
                    ConstValue::Schema(schema) => {
                        let (byte, other) = match schema {
                            Schema::Unit => (ff::VALUE_UNIT, 0),
                            Schema::Integer => (ff::VALUE_INTEGER, 0),
                            Schema::Float => (ff::VALUE_FLOAT, 0),
                            Schema::Character => (ff::VALUE_CHARACTER, 0),
                            Schema::Duration => (ff::VALUE_DURATION, 0),
                            Schema::DateTime => (ff::VALUE_DATE_TIME, 0),
                            Schema::ObjectReference(id) => (
                                ff::VALUE_OBJECT_REFERENCE,
                                match id {
                                    None => 0,
                                    Some(ObjectId(id)) => id.get(),
                                },
                            ),
                            Schema::Schema => (ff::VALUE_SCHEMA, 0),
                            Schema::Language => (ff::VALUE_LANGUAGE, 0),
                            Schema::Url => (ff::VALUE_URL, 0),
                            Schema::Color => (ff::VALUE_COLOR, 0),
                            Schema::Email => (ff::VALUE_EMAIL, 0),
                            Schema::Text => (ff::VALUE_TEXT, 0),
                            Schema::Binary => (ff::VALUE_BINARY, 0),
                            Schema::Encrypted => (ff::VALUE_ENCRYPTED, 0),
                        };

                        (ff::VALUE_SCHEMA as u64 | ((byte as u64) << 8), other)
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
                    ([ff::VALUE_UNIT, 0, 0, 0, 0, 0, 0, 0], 0) => Value::Unit,
                    ([ff::VALUE_INTEGER, 0, 0, 0, 0, 0, 0, 0], b) => Value::Integer(b as _),
                    ([ff::VALUE_FLOAT, 0, 0, 0, 0, 0, 0, 0], 0) => Value::Float(f64::from_bits(b)),
                    ([ff::VALUE_CHARACTER, 0, 0, 0, remaining @ ..], 0) => {
                        Value::Character(char::from_u32(u32::from_le_bytes(remaining)).ok_or(())?)
                    }
                    ([ff::VALUE_DURATION, bytes @ ..], b) => {
                        Value::Duration(Duration::from_nanos(I120::from_le_bytes(unsafe {
                            transmute((bytes, b.to_le_bytes()))
                        })))
                    }
                    ([ff::VALUE_DATE_TIME, bytes @ ..], b) => {
                        Value::DateTime(DateTime::UNIX.const_add(Duration::from_nanos(
                            I120::from_le_bytes(unsafe { transmute((bytes, b.to_le_bytes())) }),
                        )))
                    }
                    ([ff::VALUE_OBJECT_REFERENCE, 0, 0, 0, 0, 0, 0, 0], b) if b != 0 => {
                        Value::ObjectReference(ObjectId(NonZeroU64::new(b).unwrap()))
                    }
                    (
                        [
                            ff::VALUE_SCHEMA,
                            ff::VALUE_OBJECT_REFERENCE,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ],
                        b,
                    ) => Value::Schema(Schema::ObjectReference(NonZeroU64::new(b).map(ObjectId))),
                    ([ff::VALUE_SCHEMA, byte, 0, 0, 0, 0, 0, 0], 0) => Value::Schema(match byte {
                        ff::VALUE_UNIT => Schema::Unit,
                        ff::VALUE_INTEGER => Schema::Integer,
                        ff::VALUE_FLOAT => Schema::Float,
                        ff::VALUE_CHARACTER => Schema::Character,
                        ff::VALUE_DURATION => Schema::Duration,
                        ff::VALUE_DATE_TIME => Schema::DateTime,
                        ff::VALUE_SCHEMA => Schema::Schema,
                        ff::VALUE_LANGUAGE => Schema::Language,
                        ff::VALUE_URL => Schema::Url,
                        ff::VALUE_COLOR => Schema::Color,
                        ff::VALUE_EMAIL => Schema::Email,
                        ff::VALUE_TEXT => Schema::Text,
                        ff::VALUE_BINARY => Schema::Binary,
                        ff::VALUE_ENCRYPTED => Schema::Encrypted,
                        ff::VALUE_OBJECT_REFERENCE => unreachable!(),
                        _ => return Err(()),
                    }),
                    ([ff::VALUE_LANGUAGE, 0, lang_bytes @ .., 0, 0, 0, 0], 0) => Value::Language(
                        Language::try_from(u16::from_le_bytes(lang_bytes)).map_err(|_| ())?,
                    ),
                    ([ff::VALUE_COLOR, 0, 0, 0, bytes_c1 @ ..], b) => {
                        let l = f32::from_le_bytes(bytes_c1);
                        let a = f32::from_bits((b & u32::MAX as u64) as u32);
                        let b = f32::from_bits((b >> u32::BITS as u64) as u32);

                        Value::Color(Color { l, a, b })
                    }
                    ([ff::VALUE_URL, ..], _) => todo!(),
                    ([ff::VALUE_URL_MAX, ..], _) => todo!(),
                    ([ff::VALUE_URL_REFERENCE, ..], _) => todo!(),
                    ([ff::VALUE_EMAIL, ..], _) => todo!(),
                    ([ff::VALUE_EMAIL_MAX, ..], _) => todo!(),
                    ([ff::VALUE_EMAIL_REFERENCE, ..], _) => todo!(),
                    ([ff::VALUE_TEXT, ..], _) => todo!(),
                    ([ff::VALUE_TEXT_MAX, ..], _) => todo!(),
                    ([ff::VALUE_TEXT_REFERENCE, ..], _) => todo!(),
                    ([ff::VALUE_BINARY, ..], _) => todo!(),
                    ([ff::VALUE_BINARY_MAX, ..], _) => todo!(),
                    ([ff::VALUE_BINARY_REFERENCE, ..], _) => todo!(),
                    ([ff::VALUE_ENCRYPTED, ..], _) => todo!(),
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
