mod color;
mod lang;
mod schema;
mod time;
mod uri;

use crate::objects::ObjectId;
use crate::pages::PageId;
use crate::values::email::Email;
use crate::values::uri::Uri;
use crate::{ff, Error};
use arrayvec::{ArrayString, ArrayVec};
pub use color::*;
pub use lang::*;
pub use schema::*;
use std::fmt::Debug;
use std::intrinsics::transmute_unchecked;
use std::num::NonZeroU64;
pub use time::*;

mod email;
mod inline;

const fn concat_arrays<const M: usize, const N: usize, T>(a: [T; M], b: [T; N]) -> [T; M + N] {
    #[repr(C, packed)]
    struct Pair<A, B>(A, B);

    unsafe { transmute_unchecked(Pair(a, b)) }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Value {
    Unit = ff::VALUE_UNIT,
    Integer(i64) = ff::VALUE_INTEGER,
    Float(f64) = ff::VALUE_FLOAT,
    Character(char) = ff::VALUE_CHARACTER,
    Duration(Duration) = ff::VALUE_DURATION,
    DateTime(DateTime) = ff::VALUE_DATE_TIME,
    ObjectReference(ObjectId) = ff::VALUE_OBJECT_REFERENCE,
    Schema(Schema) = ff::VALUE_SCHEMA,
    Language(Language) = ff::VALUE_LANGUAGE,
    Uri(Uri) = ff::VALUE_URI,
    UriSpilled(Spilled) = ff::VALUE_URI_SPILLED,
    Color(Color) = ff::VALUE_COLOR,
    Email(Email) = ff::VALUE_EMAIL,
    EmailSpilled(Spilled) = ff::VALUE_EMAIL_SPILLED,
    Text(ArrayString<15>) = ff::VALUE_TEXT,
    TextSpilled(Spilled) = ff::VALUE_TEXT_SPILLED,
    Binary(ArrayVec<u8, 15>) = ff::VALUE_BINARY,
    BinarySpilled(Spilled) = ff::VALUE_BINARY_SPILLED,
    Encrypted(Spilled) = ff::VALUE_ENCRYPTED,
}

pub type SpilledValueLength = [u8; 7];

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct Spilled {
    len: SpilledValueLength,
    page: PageId,
}

#[derive(Clone)]
pub struct OpaqueValue(u64, u64);

impl TryFrom<OpaqueValue> for Value {
    type Error = Error;

    fn try_from(OpaqueValue(a, b): OpaqueValue) -> Result<Self, Self::Error> {
        match (a.to_le_bytes(), b) {
            ([ff::VALUE_UNIT, 0, 0, 0, 0, 0, 0, 0], 0) => Ok(Value::Unit),
            ([ff::VALUE_INTEGER, 0, 0, 0, 0, 0, 0, 0], b) => Ok(Value::Integer(b as _)),
            ([ff::VALUE_FLOAT, 0, 0, 0, 0, 0, 0, 0], 0) => Ok(Value::Float(f64::from_bits(b))),
            ([ff::VALUE_CHARACTER, 0, 0, 0, remaining @ ..], 0) => Ok(Value::Character(
                char::from_u32(u32::from_le_bytes(remaining)).ok_or(Error::Other)?,
            )),
            ([ff::VALUE_DURATION, bytes @ ..], b) => Ok(Value::Duration(Duration::from_nanos(
                I120::from_le_bytes(concat_arrays(bytes, b.to_le_bytes())),
            ))),
            ([ff::VALUE_DATE_TIME, bytes @ ..], b) => Ok(Value::DateTime(
                DateTime::UNIX.const_add(Duration::from_nanos(I120::from_le_bytes(unsafe {
                    concat_arrays(bytes, b.to_le_bytes())
                }))),
            )),
            ([ff::VALUE_OBJECT_REFERENCE, 0, 0, 0, 0, 0, 0, 0], b) if b != 0 => Ok(
                Value::ObjectReference(ObjectId(NonZeroU64::new(b).unwrap())),
            ),
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
            ) => Ok(Value::Schema(Schema::ObjectReference(
                NonZeroU64::new(b).map(ObjectId),
            ))),
            ([ff::VALUE_SCHEMA, byte, 0, 0, 0, 0, 0, 0], 0) => Ok(Value::Schema(match byte {
                ff::VALUE_UNIT => Schema::Unit,
                ff::VALUE_INTEGER => Schema::Integer,
                ff::VALUE_FLOAT => Schema::Float,
                ff::VALUE_CHARACTER => Schema::Character,
                ff::VALUE_DURATION => Schema::Duration,
                ff::VALUE_DATE_TIME => Schema::DateTime,
                ff::VALUE_SCHEMA => Schema::Schema,
                ff::VALUE_LANGUAGE => Schema::Language,
                ff::VALUE_URI => Schema::Uri,
                ff::VALUE_COLOR => Schema::Color,
                ff::VALUE_EMAIL => Schema::Email,
                ff::VALUE_TEXT => Schema::Text,
                ff::VALUE_BINARY => Schema::Binary,
                ff::VALUE_ENCRYPTED => Schema::Encrypted,
                ff::VALUE_OBJECT_REFERENCE => unreachable!(),
                _ => return Err(Error::Other),
            })),
            ([ff::VALUE_LANGUAGE, 0, lang_bytes @ .., 0, 0, 0, 0], 0) => Ok(Value::Language(
                Language::try_from(u16::from_le_bytes(lang_bytes)).map_err(|_| Error::Other)?,
            )),
            ([ff::VALUE_COLOR, 0, 0, 0, bytes_c1 @ ..], b) => {
                let l = f32::from_le_bytes(bytes_c1);
                let a = f32::from_bits((b & u32::MAX as u64) as u32);
                let b = f32::from_bits((b >> u32::BITS as u64) as u32);

                Ok(Value::Color(Color { l, a, b }))
            }
            ([ff::VALUE_URI, ..], _) => todo!(),
            ([ff::VALUE_URI_MAX, ..], _) => todo!(),
            ([ff::VALUE_URI_SPILLED, ..], _) => todo!(),
            ([ff::VALUE_EMAIL, len, bytes @ ..], b) => {
                if len > 14 {
                    return Err(Error::Other);
                }

                let bytes: [u8; 14] = concat_arrays(bytes, b.to_le_bytes());
                let s = str::from_utf8(&bytes[..len as usize]).map_err(|_| Error::Other)?;

                let s = ArrayString::<15>::from(s).unwrap();
                Ok(Value::Email(Email::try_from(s).map_err(|_| Error::Other)?))
            }
            ([ff::VALUE_EMAIL_MAX, bytes @ ..], b) => {
                let bytes: [u8; 15] = concat_arrays(bytes, b.to_le_bytes());

                let arr =
                    ArrayString::<15>::from(str::from_utf8(&bytes).map_err(|_| Error::Other)?)
                        .unwrap();

                Ok(Value::Email(
                    Email::try_from(arr).map_err(|_| Error::Other)?,
                ))
            }
            ([ff::VALUE_EMAIL_SPILLED, len @ ..], page_id) if page_id != 0 => {
                Ok(Value::EmailSpilled(Spilled {
                    len,
                    page: NonZeroU64::new(page_id).unwrap(),
                }))
            }
            ([ff::VALUE_TEXT, ..], _) => todo!(),
            ([ff::VALUE_TEXT_MAX, ..], _) => todo!(),
            ([ff::VALUE_TEXT_SPILLED, ..], _) => todo!(),
            ([ff::VALUE_BINARY, ..], _) => todo!(),
            ([ff::VALUE_BINARY_MAX, ..], _) => todo!(),
            ([ff::VALUE_BINARY_SPILLED, ..], _) => todo!(),
            ([ff::VALUE_ENCRYPTED, len @ ..], page_id) if page_id != 0 => {
                Ok(Value::Encrypted(Spilled {
                    len,
                    page: NonZeroU64::new(page_id).unwrap(),
                }))
            }
            ([kind, ..], _) => Err(Error::InvalidValueKind(kind)),
        }
    }
}
