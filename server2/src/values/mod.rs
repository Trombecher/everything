mod email;

pub use email::*;

use crate::{ff, schema::SchemaRef, ObjectId};

/// A value whose top layer has been decoded.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PartiallyDecodedValue<'a> {
    True = ff::value::TRUE,
    False = ff::value::FALSE,
    Integer(i64) = ff::value::INTEGER,
    Float(f64) = ff::value::FLOAT,
    Character(char) = ff::value::CHARACTER,
    Duration(i128) = ff::value::DURATION,
    DateTime(i128) = ff::value::DATE_TIME,
    NoObject = ff::value::NO_OBJECT,
    Object(ObjectId) = ff::value::OBJECT,
    Language(u32) = ff::value::LANGUAGE,
    
    Url(&'a str) = ff::value::URL,
    Color(&'a str) = ff::value::COLOR,
    Schema(SchemaRef<'a>) = ff::value::SCHEMA,
    Contraint(&'a [u8]) = ff::value::CONSTRAINT,
    
    Email(&'a Email) = ff::value::EMAIL,
    Text(&'a str) = ff::value::TEXT,
    Binary(&'a [u8]) = ff::value::BINARY,
    
    EncryptedEmail(&'a [u8]) = ff::value::ENC_EMAIL,
    EncryptedText(&'a [u8]) = ff::value::ENC_TEXT,
    EncryptedBinary(&'a [u8]) = ff::value::ENC_BINARY,
}

/// An owned value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    True,
    False,
    Integer(i64),
    Float(f64),
    Character(char),
    Duration(i128),
    DateTime(i128),
    Object(Option<ObjectId>),
    Language(u32),

    Url(String),
    Color(String),
    Schema(Vec<u8>),
    Contraint(Vec<u8>),
    
    Email(String),
    Text(String),
    Binary(Vec<u8>),
    
    EncryptedEmail(Vec<u8>),
    EncryptedText(Vec<u8>),
    EncryptedBinary(Vec<u8>),
}