mod color;
mod email;
mod lang;
mod schema;
mod time;
mod uri;

use crate::objects::ObjectId;
use crate::pages::PageId;
use crate::{ff, Error};
use arrayvec::{ArrayString, ArrayVec};
use std::fmt::Debug;
use std::intrinsics::transmute_unchecked;
use std::num::NonZeroU64;

pub use color::*;
pub use email::*;
pub use lang::*;
pub use schema::*;
pub use time::*;
pub use uri::*;

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

#[derive(Clone, Debug, Copy)]
#[repr(C, packed)]
pub struct Spilled {
    len: u64,
    pub page: PageId,
}

impl Spilled {
    const MIN_LEN: u64 = 16;

    pub fn len(self) -> u64 {
        self.len
    }

    pub fn new(page: PageId, len: u64) -> Option<Self> {
        if len >= Self::MIN_LEN {
            Some(Self { len, page })
        } else {
            None
        }
    }
}

#[derive(Clone)]
#[repr(align(8))]
pub struct OpaqueValue([u8; 16]);

impl TryFrom<OpaqueValue> for Value {
    type Error = Error;

    fn try_from(OpaqueValue(bytes): OpaqueValue) -> Result<Self, Self::Error> {
        match bytes {
            [ff::VALUE_UNIT, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] => Ok(Value::Unit),
            [ff::VALUE_INTEGER, 0, 0, 0, 0, 0, 0, 0, int_bytes @ ..] => {
                Ok(Value::Integer(i64::from_le_bytes(int_bytes)))
            }
            [ff::VALUE_FLOAT, 0, 0, 0, 0, 0, 0, 0, float_bytes @ ..] => {
                Ok(Value::Float(f64::from_le_bytes(float_bytes)))
            }
            [
                ff::VALUE_CHARACTER,
                0,
                0,
                0,
                remaining @ ..,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ] => Ok(Value::Character(
                char::from_u32(u32::from_le_bytes(remaining)).ok_or(Error::Other)?,
            )),
            [ff::VALUE_DURATION, bytes @ ..] => Ok(Value::Duration(Duration::from_nanos(
                I120::from_bytes(bytes),
            ))),
            [ff::VALUE_DATE_TIME, bytes @ ..] => Ok(Value::DateTime(
                DateTime::UNIX.const_add(Duration::from_nanos(I120::from_bytes(bytes))),
            )),
            [ff::VALUE_OBJECT_REFERENCE, 0, 0, 0, 0, 0, 0, 0, bytes @ ..]
                if let Some(b) = NonZeroU64::new(u64::from_le_bytes(bytes)) =>
            {
                Ok(Value::ObjectReference(ObjectId(b)))
            }
            [
                ff::VALUE_SCHEMA,
                ff::VALUE_OBJECT_REFERENCE,
                0,
                0,
                0,
                0,
                0,
                0,
                bytes @ ..,
            ] => Ok(Value::Schema(Schema::ObjectReference(
                NonZeroU64::new(u64::from_le_bytes(bytes)).map(ObjectId),
            ))),
            [
                ff::VALUE_SCHEMA,
                byte,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ] => Ok(Value::Schema(match byte {
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
            [
                ff::VALUE_LANGUAGE,
                0,
                lang_bytes @ ..,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ] => Ok(Value::Language(
                Language::try_from(u16::from_le_bytes(lang_bytes)).map_err(|_| Error::Other)?,
            )),
            [ff::VALUE_COLOR, 0, 0, 0, bytes_c1 @ ..] => {
                let mut x = bytes_c1.array_chunks::<4>().copied();
                let l = f32::from_le_bytes(x.next().unwrap());
                let a = f32::from_le_bytes(x.next().unwrap());
                let b = f32::from_le_bytes(x.next().unwrap());

                Ok(Value::Color(Color { l, a, b }))
            }
            [ff::VALUE_URI, len, bytes @ ..] => {
                let s = if len <= 14
                    && let Ok(x) = str::from_utf8(&bytes)
                {
                    x
                } else {
                    return Err(Error::Other);
                };

                let uri = Uri::try_from(s).map_err(|_| Error::Other)?;

                Ok(Value::Uri(uri))
            }
            [ff::VALUE_URI_MAX, bytes @ ..] => {
                let s = str::from_utf8(&bytes).map_err(|_| Error::Other)?;

                Ok(Value::Uri(Uri::try_from(s).map_err(|_| Error::Other)?))
            }
            [ff::VALUE_EMAIL, len, bytes @ ..] => {
                if len > 14 {
                    return Err(Error::Other);
                }

                let s = str::from_utf8(&bytes[..len as usize]).map_err(|_| Error::Other)?;

                Ok(Value::Email(Email::try_from(s).map_err(|_| Error::Other)?))
            }
            [ff::VALUE_EMAIL_MAX, bytes @ ..] => {
                let arr = str::from_utf8(&bytes).map_err(|_| Error::Other)?;

                Ok(Value::Email(
                    Email::try_from(arr).map_err(|_| Error::Other)?,
                ))
            }
            [ff::VALUE_TEXT, ..] => todo!(),
            [ff::VALUE_TEXT_MAX, ..] => todo!(),
            [ff::VALUE_BINARY, ..] => todo!(),
            [ff::VALUE_BINARY_MAX, ..] => todo!(),
            [
                v @ (ff::VALUE_URI_SPILLED
                | ff::VALUE_TEXT_SPILLED
                | ff::VALUE_EMAIL_SPILLED
                | ff::VALUE_BINARY_SPILLED
                | ff::VALUE_ENCRYPTED),
                p0,
                p1,
                p2,
                p3,
                p4,
                p5,
                p6,
                len @ ..,
            ] => {
                let page = NonZeroU64::new(u64::from_le_bytes([p0, p1, p2, p3, p4, p5, p6, 0]))
                    .ok_or(Error::Other)?;

                let len = u64::from_le_bytes(len);

                let spilled = match v {
                    ff::VALUE_URI_SPILLED => Value::UriSpilled,
                    ff::VALUE_TEXT_SPILLED => Value::TextSpilled,
                    ff::VALUE_EMAIL_SPILLED => Value::EmailSpilled,
                    ff::VALUE_BINARY_SPILLED => Value::BinarySpilled,
                    ff::VALUE_ENCRYPTED => Value::Encrypted,
                    _ => unreachable!(),
                };

                Ok(spilled(Spilled::new(page, len).ok_or(Error::Other)?))
            }
            [kind, ..] => Err(Error::InvalidValueKind(kind)),
        }
    }
}

impl Into<OpaqueValue> for Value {
    fn into(self) -> OpaqueValue {
        match self {
            Self::Unit => OpaqueValue(concat_arrays([ff::VALUE_UNIT], [0; 15])),
            Self::Integer(i) => OpaqueValue(concat_arrays(
                [ff::VALUE_INTEGER, 0, 0, 0, 0, 0, 0, 0],
                i.to_le_bytes(),
            )),
            Self::Float(f) => OpaqueValue(concat_arrays(
                [ff::VALUE_FLOAT, 0, 0, 0, 0, 0, 0, 0],
                f.to_le_bytes(),
            )),
            Self::Character(c) => OpaqueValue(concat_arrays(
                concat_arrays([ff::VALUE_CHARACTER, 0, 0, 0], (c as u32).to_le_bytes()),
                [0; 8],
            )),
            Self::Duration(d) => {
                OpaqueValue(concat_arrays([ff::VALUE_DURATION], d.as_nanos().to_bytes()))
            }
            Self::DateTime(d) => OpaqueValue(concat_arrays(
                [ff::VALUE_DATE_TIME],
                (d - DateTime::UNIX).as_nanos().to_bytes(),
            )),
            Self::ObjectReference(r) => OpaqueValue(concat_arrays(
                [ff::VALUE_OBJECT_REFERENCE, 0, 0, 0, 0, 0, 0, 0],
                r.0.get().to_le_bytes(),
            )),
            Self::Schema(_) => todo!(),
            Self::Language(_) => todo!(),
            Self::Uri(_) => todo!(),
            Self::UriSpilled(_) => todo!(),
            Self::Color(_) => todo!(),
            Self::Email(_) => todo!(),
            Self::EmailSpilled(_) => todo!(),
            Self::Text(_) => todo!(),
            Self::TextSpilled(_) => todo!(),
            Self::Binary(_) => todo!(),
            Self::BinarySpilled(_) => todo!(),
            Self::Encrypted(_) => todo!(),
        }
    }
}
