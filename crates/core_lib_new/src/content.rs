use crate::ff;
use crate::objects::ObjectId;
use crate::values::inline::InlineContent;
use crate::values::{DateTime, Duration, Schema, Value, VariableContent};
use std::num::NonZeroU64;

#[repr(C, align(4096))]
pub struct Content {
    magic_bytes: [u8; 12],
    version: u32,
    entries: [Row],
}

impl Content {}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Row([u64; 4]);

impl Row {
    pub const fn encode(row: DecodedRow) -> Self {
        match row {
            DecodedRow::Association(target_id, tag_id, value) => {
                let (a, b) = match value {
                    Value::Unit => (ff::UNIT as u64, 0),
                    Value::Float(f) => (ff::FLOAT as u64, f.to_bits()),
                    Value::Integer(i) => (ff::INTEGER as u64, i as _),
                    Value::ObjectReference(id) => (ff::OBJECT_REFERENCE as u64, id.get()),
                    Value::Character(c) => (ff::CHARACTER as u64 | (c as u32 as u64) << 32, 0),
                    Value::Schema(s) => match s {
                        Schema::Unit => (((ff::UNIT as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::ObjectReference(id) => (
                            ((ff::OBJECT_REFERENCE as u64) << 8) | ff::SCHEMA as u64,
                            if let Some(id) = id { id.get() } else { 0 },
                        ),
                        Schema::Integer => (((ff::INTEGER as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Float => (((ff::FLOAT as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Character => (((ff::CHARACTER as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Duration => (((ff::DURATION as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::DateTime => (((ff::DATE_TIME as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Schema => (((ff::SCHEMA as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Language => (((ff::LANGUAGE as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Url => (((ff::URL as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Color => (((ff::COLOR as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Email => (((ff::EMAIL as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Text => (((ff::TEXT as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Binary => (((ff::BINARY as u64) << 8) | ff::SCHEMA as u64, 0),
                        Schema::Encrypted => (((ff::ENCRYPTED as u64) << 8) | ff::SCHEMA as u64, 0),
                    },
                    _ => todo!(),
                };

                Self([target_id.get(), tag_id.get(), a, b])
            }
            DecodedRow::FreeListNextFreeIndex(index) => Self([0, index.get(), 0, 0]),
            DecodedRow::FreeListEnd => Self([0; 4]),
        }
    }

    pub fn decode(self) -> Result<DecodedRow, ()> {
        match self.0 {
            [0, 0, 0, 0] => Ok(DecodedRow::FreeListEnd),
            [0, index, 0, 0] => Ok(DecodedRow::FreeListNextFreeIndex(
                NonZeroU64::new(index).unwrap(),
            )),
            [object_id, tag_id, a, b] if tag_id != 0 => {
                // These should not zero-check
                let tag_id = NonZeroU64::new(tag_id).unwrap();
                let object_id = NonZeroU64::new(object_id).unwrap();

                let value = match a.to_le_bytes() {
                    [ff::UNIT, ..] => Value::Unit,
                    [ff::INTEGER, ..] => Value::Integer(b as _),
                    [ff::FLOAT, ..] => Value::Float(f64::from_bits(b)),
                    [ff::CHARACTER, .., a, b, c, d] => {
                        Value::Character(match char::from_u32(u32::from_le_bytes([a, b, c, d])) {
                            Some(char) => char,
                            None => return Err(()),
                        })
                    }
                    [ff::DURATION, a, b, c, d, e, f, g] => {
                        let a = u64::from_le_bytes([0, a, b, c, d, e, f, g]) as i128;
                        Value::Duration(Duration::from_nanos((a << 64) | b as i128))
                    }
                    [ff::DATE_TIME, a, b, c, d, e, f, g] => {
                        let a = u64::from_le_bytes([0, a, b, c, d, e, f, g]) as i128;
                        Value::DateTime(
                            DateTime::UNIX + Duration::from_nanos((a << 64) | b as i128),
                        )
                    }
                    [ff::OBJECT_REFERENCE, ..] if b != 0 => {
                        Value::ObjectReference(NonZeroU64::new(b).unwrap())
                    }
                    [ff::SCHEMA, ff::UNIT, ..] => Value::Schema(Schema::Unit),
                    [ff::SCHEMA, ff::INTEGER, ..] => Value::Schema(Schema::Integer),
                    [ff::SCHEMA, ff::FLOAT, ..] => Value::Schema(Schema::Float),
                    [ff::SCHEMA, ff::CHARACTER, ..] => Value::Schema(Schema::Character),
                    [ff::SCHEMA, ff::DURATION, ..] => Value::Schema(Schema::Duration),
                    [ff::SCHEMA, ff::DATE_TIME, ..] => Value::Schema(Schema::DateTime),
                    [ff::SCHEMA, ff::OBJECT_REFERENCE, ..] => {
                        Value::Schema(Schema::ObjectReference(NonZeroU64::new(b)))
                    }
                    [ff::SCHEMA, ff::SCHEMA, ..] => Value::Schema(Schema::Schema),
                    [ff::SCHEMA, ff::LANGUAGE, ..] => Value::Schema(Schema::Language),
                    [ff::SCHEMA, ff::URL, ..] => Value::Schema(Schema::Url),
                    [ff::SCHEMA, ff::COLOR, ..] => Value::Schema(Schema::Color),
                    [ff::SCHEMA, ff::EMAIL, ..] => Value::Schema(Schema::Email),
                    [ff::SCHEMA, ff::TEXT, ..] => Value::Schema(Schema::Text),
                    [ff::SCHEMA, ff::BINARY, ..] => Value::Schema(Schema::Binary),
                    [ff::SCHEMA, ff::ENCRYPTED, ..] => Value::Schema(Schema::Encrypted),
                    [ff::LANGUAGE, _, _, _, a, b, c, d] => {
                        match char::from_u32(u32::from_le_bytes([a, b, c, d])) {
                            Some(c) => Value::Character(c),
                            None => return Err(()),
                        }
                    }
                    [ff::BINARY, len @ 0..15, a, b, c, d, e, f] => {
                        let mut content = [a, b, c, d, e, f, 0, 0, 0, 0, 0, 0, 0, 0];
                        (&mut content[6..]).copy_from_slice(&b.to_le_bytes());

                        Value::Binary(VariableContent::Inline(InlineContent::try_from(
                            &content[..],
                        )?))
                    }
                    [ff::BINARY_MAX, a, b, c, d, e, f, g] => {
                        let mut content = [a, b, c, d, e, f, g, 0, 0, 0, 0, 0, 0, 0, 0];
                        (&mut content[7..]).copy_from_slice(&b.to_le_bytes());
                        Value::Binary(VariableContent::InlineMax(content))
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
    Association(ObjectId, ObjectId, Value),

    /// Row matches `[0, index != 0, 0, 0]`
    FreeListNextFreeIndex(NonZeroU64),

    /// Row matches `[0, 0, 0, 0]`
    FreeListEnd,
}
