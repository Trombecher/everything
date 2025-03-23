mod tests;

use std::{hint::unreachable_unchecked, mem::transmute};

use crate::{
    constraints::{Constraint, EncodedConstraint}, decode::{read_bytes, Decodable, PartiallyDecodable}, email::Email, ff, lang::Language, schema::{EncodedSchema, Schema}, time::{DateTime, Duration}, ObjectId
};

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
            Some(ff::value::TRUE | ff::value::FALSE) => true,
            Some(ff::value::INTEGER | ff::value::FLOAT | ff::value::OBJECT) if slice.len() >= 9 => {
                true
            }
            Some(ff::value::DURATION | ff::value::DATE_TIME) if slice.len() >= 17 => true,
            Some(ff::value::CHARACTER) if slice.len() >= 5 => unsafe {
                char::from_u32(u32::from_le_bytes(read_bytes::<4>(slice, 1))).is_some()
            },
            Some(ff::value::LANGUAGE) if slice.len() >= 3 => unsafe {
                Language::try_from(u16::from_le_bytes(read_bytes::<2>(slice, 1))).is_ok()
            },
            Some(ff::value::SCHEMA) if EncodedSchema::new(&slice[1..]).is_some() => true,
            // TODO: more
            _ => false,
        }
    }
}

impl<'a> PartiallyDecodable for &'a EncodedValue {
    type PartialOutput = PartiallyDecodedValue<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        match self.0.get(0).copied() {
            Some(ff::value::TRUE) => PartiallyDecodedValue::True,
            Some(ff::value::FALSE) => PartiallyDecodedValue::False,
            Some(ff::value::INTEGER) => unsafe {
                PartiallyDecodedValue::Integer(i64::from_le_bytes(read_bytes::<8>(&self.0, 1)))
            },
            Some(ff::value::FLOAT) => unsafe {
                PartiallyDecodedValue::Float(f64::from_le_bytes(read_bytes::<8>(&self.0, 1)))
            },
            Some(ff::value::CHARACTER) => unsafe {
                PartiallyDecodedValue::Character(char::from_u32_unchecked(u32::from_le_bytes(
                    read_bytes::<4>(&self.0, 1),
                )))
            },
            Some(ff::value::DURATION) => unsafe {
                PartiallyDecodedValue::Duration(Duration::from_nanos(i128::from_le_bytes(
                    read_bytes::<16>(&self.0, 1),
                )))
            },
            Some(ff::value::DATE_TIME) => unsafe {
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
    True = ff::value::TRUE,
    False = ff::value::FALSE,
    Integer(i64) = ff::value::INTEGER,
    Float(f64) = ff::value::FLOAT,
    Character(char) = ff::value::CHARACTER,
    Duration(Duration) = ff::value::DURATION,
    DateTime(DateTime) = ff::value::DATE_TIME,
    Object(Option<ObjectId>) = ff::value::OBJECT,
    Language(u32) = ff::value::LANGUAGE,

    Url(&'a str) = ff::value::URL,
    Color(&'a str) = ff::value::COLOR,
    Schema(&'a EncodedSchema) = ff::value::SCHEMA,
    Contraint(&'a EncodedConstraint) = ff::value::CONSTRAINT,

    Email(&'a Email) = ff::value::EMAIL,
    Text(&'a str) = ff::value::TEXT,
    Binary(&'a [u8]) = ff::value::BINARY,

    EncryptedEmail(&'a [u8]) = ff::value::ENC_EMAIL,
    EncryptedText(&'a [u8]) = ff::value::ENC_TEXT,
    EncryptedBinary(&'a [u8]) = ff::value::ENC_BINARY,
}

impl<'a> Decodable for PartiallyDecodedValue<'a> {
    type Output = Value;

    fn decode(&self) -> Self::Output {
        match *self {
            PartiallyDecodedValue::True => Value::True,
            PartiallyDecodedValue::False => Value::False,
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
            PartiallyDecodedValue::Contraint(con) => Value::Contraint(Box::new(con.decode())),
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
    True = ff::value::TRUE,
    False = ff::value::FALSE,
    Integer(i64) = ff::value::INTEGER,
    Float(f64) = ff::value::FLOAT,
    Character(char) = ff::value::CHARACTER,
    Duration(Duration) = ff::value::DURATION,
    DateTime(DateTime) = ff::value::DATE_TIME,
    Object(Option<ObjectId>) = ff::value::OBJECT,
    Language(u32) = ff::value::LANGUAGE,

    Url(String) = ff::value::URL,
    Color(String) = ff::value::COLOR,
    Schema(Box<Schema>) = ff::value::SCHEMA,
    Contraint(Box<Constraint>) = ff::value::CONSTRAINT,

    Email(Box<Email>) = ff::value::EMAIL,
    Text(String) = ff::value::TEXT,
    Binary(Vec<u8>) = ff::value::BINARY,

    EncryptedEmail(Vec<u8>) = ff::value::ENC_EMAIL,
    EncryptedText(Vec<u8>) = ff::value::ENC_TEXT,
    EncryptedBinary(Vec<u8>) = ff::value::ENC_BINARY,
}
