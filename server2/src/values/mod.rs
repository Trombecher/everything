mod row;
mod tests;

pub use row::*;

use std::{hint::unreachable_unchecked, mem::transmute};

use crate::{
    constraints::{Constraint, EncodedConstraint},
    decode::{Decodable, PartiallyDecodable, read_bytes},
    email::Email,
    ff,
    lang::Language,
    schema::{EncodedSchema, Schema},
    time::{DateTime, Duration},
};
use crate::objects::ObjectId;

/// A value, encoded into a slice.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct EncodedValue([u8]);

impl EncodedValue {
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(slice: &[u8]) -> &Self {
        unsafe { transmute(slice) }
    }

    #[inline]
    #[must_use]
    pub fn new(slice: &[u8]) -> Option<&Self> {
        Self::validate(slice).then_some(unsafe { Self::new_unchecked(slice) })
    }

    fn validate(slice: &[u8]) -> bool {
        match slice.get(0).copied() {
            Some(ff::INTEGER | ff::FLOAT | ff::OBJECT) if slice.len() >= 9 => true,
            Some(ff::DURATION | ff::DATE_TIME) if slice.len() >= 17 => true,
            Some(ff::CHARACTER) if slice.len() >= 5 => unsafe {
                char::from_u32(u32::from_le_bytes(read_bytes::<4>(slice, 1))).is_some()
            },
            Some(ff::LANGUAGE) if slice.len() >= 3 => unsafe {
                Language::try_from(u16::from_le_bytes(read_bytes::<2>(slice, 1))).is_ok()
            },
            Some(ff::SCHEMA) if EncodedSchema::new(&slice[1..]).is_some() => true,
            // TODO: more
            _ => false,
        }
    }
}

impl<'a> PartiallyDecodable for &'a EncodedValue {
    type PartialOutput = PartiallyDecodedValue<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        match self.0.get(0).copied() {
            Some(ff::INTEGER) => unsafe {
                PartiallyDecodedValue::Integer(i64::from_le_bytes(read_bytes::<8>(&self.0, 1)))
            },
            Some(ff::FLOAT) => unsafe {
                PartiallyDecodedValue::Float(f64::from_le_bytes(read_bytes::<8>(&self.0, 1)))
            },
            Some(ff::CHARACTER) => unsafe {
                PartiallyDecodedValue::Character(char::from_u32_unchecked(u32::from_le_bytes(
                    read_bytes::<4>(&self.0, 1),
                )))
            },
            Some(ff::DURATION) => unsafe {
                PartiallyDecodedValue::Duration(Duration::from_nanos(i128::from_le_bytes(
                    read_bytes::<16>(&self.0, 1),
                )))
            },
            Some(ff::DATE_TIME) => unsafe {
                PartiallyDecodedValue::DateTime(DateTime::from(i128::from_le_bytes(
                    read_bytes::<16>(&self.0, 1),
                )))
            },
            // TODO: more
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

/// A value whose top layer has been decoded.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PartiallyDecodedValue<'a> {
    Integer(i64) = ff::INTEGER,
    Float(f64) = ff::FLOAT,
    Character(char) = ff::CHARACTER,
    Duration(Duration) = ff::DURATION,
    DateTime(DateTime) = ff::DATE_TIME,
    Object(Option<ObjectId>) = ff::OBJECT,
    Language(u32) = ff::LANGUAGE,

    Url(&'a str) = ff::URL,
    Color(&'a str) = ff::COLOR,
    Schema(&'a EncodedSchema) = ff::SCHEMA,
    Constraint(&'a EncodedConstraint) = ff::CONSTRAINT,

    Email(&'a Email) = ff::EMAIL,
    Text(&'a str) = ff::TEXT,
    Binary(&'a [u8]) = ff::BINARY,

    EncryptedEmail(&'a [u8]) = ff::ENC_EMAIL,
    EncryptedText(&'a [u8]) = ff::ENC_TEXT,
    EncryptedBinary(&'a [u8]) = ff::ENC_BINARY,
}

impl<'a> Decodable for PartiallyDecodedValue<'a> {
    type Output = Value;

    fn decode(&self) -> Self::Output {
        match *self {
            PartiallyDecodedValue::Integer(i) => Value::Integer(i),
            PartiallyDecodedValue::Float(f) => Value::Float(f),
            PartiallyDecodedValue::Character(c) => Value::Character(c),
            PartiallyDecodedValue::Duration(duration) => Value::Duration(duration),
            PartiallyDecodedValue::DateTime(date_time) => Value::DateTime(date_time),
            PartiallyDecodedValue::Object(object_id) => Value::Object(object_id),
            PartiallyDecodedValue::Language(_) => todo!(),
            PartiallyDecodedValue::Url(_) => todo!(),
            PartiallyDecodedValue::Color(_) => todo!(),
            PartiallyDecodedValue::Schema(schema) => Value::Schema(Box::new(schema.decode())),
            PartiallyDecodedValue::Constraint(con) => Value::Constraint(Box::new(con.decode())),
            PartiallyDecodedValue::Email(email) => Value::Email(email.into_boxed()),
            PartiallyDecodedValue::Text(text) => Value::Text(text.into()),
            PartiallyDecodedValue::Binary(bytes) => Value::Binary(bytes.into()),
            PartiallyDecodedValue::EncryptedEmail(enc) => Value::EncryptedEmail(enc.into()),
            PartiallyDecodedValue::EncryptedText(enc) => Value::EncryptedText(enc.into()),
            PartiallyDecodedValue::EncryptedBinary(enc) => Value::EncryptedBinary(enc.into()),
        }
    }
}

/// An owned value.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Value {
    Integer(i64) = ff::INTEGER,
    Float(f64) = ff::FLOAT,
    Character(char) = ff::CHARACTER,
    Duration(Duration) = ff::DURATION,
    DateTime(DateTime) = ff::DATE_TIME,
    Object(Option<ObjectId>) = ff::OBJECT,
    Language(u32) = ff::LANGUAGE,

    Url(String) = ff::URL,
    Color(String) = ff::COLOR,
    Schema(Box<Schema>) = ff::SCHEMA,
    Constraint(Box<Constraint>) = ff::CONSTRAINT,

    Email(Box<Email>) = ff::EMAIL,
    Text(String) = ff::TEXT,
    Binary(Vec<u8>) = ff::BINARY,

    EncryptedEmail(Vec<u8>) = ff::ENC_EMAIL,
    EncryptedText(Vec<u8>) = ff::ENC_TEXT,
    EncryptedBinary(Vec<u8>) = ff::ENC_BINARY,
}
