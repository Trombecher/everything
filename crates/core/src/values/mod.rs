mod color;
mod lang;
mod schema;
mod time;
mod uri;

use crate::objects::ObjectId;
use crate::pages::PageId;
use crate::values::email::{Email, EmailMax};
use crate::values::inline::{InlineBytes, InlineStr, InlineStrMax};
use crate::values::uri::{Uri, UriMax};
use crate::{Error, ff};
pub use color::*;
pub use lang::*;
pub use schema::*;
use static_assertions::const_assert;
use std::fmt::Debug;
use std::num::NonZeroU64;
pub use time::*;

mod email;
mod inline;
pub mod v;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Value {
    Unit = ff::VALUE_UNIT,
    Integer(v::I64) = ff::VALUE_INTEGER,
    Float(v::F64) = ff::VALUE_FLOAT,
    Character(v::Char) = ff::VALUE_CHARACTER,
    Duration(Duration) = ff::VALUE_DURATION,
    DateTime(DateTime) = ff::VALUE_DATE_TIME,
    ObjectReference(v::ObjectId1) = ff::VALUE_OBJECT_REFERENCE,
    Schema(Schema) = ff::VALUE_SCHEMA,
    Language(v::Language) = ff::VALUE_LANGUAGE,
    Uri(Uri) = ff::VALUE_URL,
    UriMax(UriMax) = ff::VALUE_URL_MAX,
    UriSpilled(Spilled) = ff::VALUE_URL_SPILLED,
    Color(Color) = ff::VALUE_COLOR,
    Email(Email) = ff::VALUE_EMAIL,
    EmailMax(EmailMax) = ff::VALUE_EMAIL_MAX,
    EmailSpilled(Spilled) = ff::VALUE_EMAIL_SPILLED,
    Text(InlineStr) = ff::VALUE_TEXT,
    TextMax(InlineStrMax) = ff::VALUE_TEXT_MAX,
    TextSpilled(Spilled) = ff::VALUE_TEXT_SPILLED,
    Binary(InlineBytes) = ff::VALUE_BINARY,
    BinaryMax([u8; 15]) = ff::VALUE_BINARY_MAX,
    BinarySpilled(Spilled) = ff::VALUE_BINARY_SPILLED,
    Encrypted(Spilled) = ff::VALUE_ENCRYPTED,
}

const_assert!(size_of::<Value>() == 16);

pub type SpilledValueLength = [u8; 7];

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct Spilled {
    len: SpilledValueLength,
    page: PageId,
}

pub struct OpaqueValue(u64, u64);

impl TryFrom<OpaqueValue> for Value {
    type Error = Error;

    fn try_from(OpaqueValue(a, b): OpaqueValue) -> Result<Self, Self::Error> {
        match (a.to_le_bytes(), b) {
            ([ff::VALUE_UNIT, 0, 0, 0, 0, 0, 0, 0], 0) => Ok(Value::Unit),
            ([ff::VALUE_INTEGER, 0, 0, 0, 0, 0, 0, 0], b) => Ok(Value::Integer(b as _)),
            ([ff::VALUE_FLOAT, 0, 0, 0, 0, 0, 0, 0], 0) => Ok(Value::Float(f64::from_bits(b))),
            ([ff::VALUE_CHARACTER, 0, 0, 0, remaining @ ..], 0) => Ok(Value::Character(
                char::from_u32(u32::from_le_bytes(remaining)).ok_or(())?,
            )),
            ([ff::VALUE_DURATION, bytes @ ..], b) => Ok(Value::Duration(Duration::from_nanos(
                I120::from_le_bytes(unsafe { transmute((bytes, b.to_le_bytes())) }),
            ))),
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
            ([ff::VALUE_LANGUAGE, 0, lang_bytes @ .., 0, 0, 0, 0], 0) => {
                Value::Language(Language::try_from(u16::from_le_bytes(lang_bytes)).map_err(|_| ())?)
            }
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
            kind => Err(Error::InvalidValueKind(kind)),
        }
    }
}
