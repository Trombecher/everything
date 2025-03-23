use std::{hint::unreachable_unchecked, mem::transmute};

use crate::{
    constraints::{Constraint, EncodedConstraint},
    decode::{Decodable, PartiallyDecodable},
    ff,
};

/// Describes the schema of the value of a tag association.
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Schema {
    None = ff::schema::NONE,
    Integer = ff::value::INTEGER,
    Float = ff::value::FLOAT,
    Character = ff::value::CHARACTER,
    Duration = ff::value::DURATION,
    DateTime = ff::value::DATE_TIME,
    Object(Constraint) = ff::value::OBJECT,
    OptionalObject(Constraint) = ff::schema::OPT_OBJECT,
    Language = ff::value::LANGUAGE,
    Url = ff::value::URL,
    Color = ff::value::COLOR,
    Schema = ff::value::SCHEMA,
    Contraint = ff::value::CONSTRAINT,
    Email = ff::value::EMAIL,
    Text = ff::value::TEXT,
    Binary = ff::value::BINARY,
    // TODO: encrypted values in schema definition
}

/// An encoded schema.
#[derive(Debug, PartialEq)]
pub struct EncodedSchema([u8]);

impl EncodedSchema {
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

    pub(crate) fn validate(slice: &[u8]) -> bool {
        match slice.get(0).copied() {
            Some(
                ff::schema::NONE
                | ff::value::INTEGER
                | ff::value::FLOAT
                | ff::value::CHARACTER
                | ff::value::DURATION
                | ff::value::DATE_TIME
                | ff::value::LANGUAGE
                | ff::value::URL
                | ff::value::COLOR
                | ff::value::EMAIL
                | ff::value::TEXT
                | ff::value::BINARY
                | ff::value::SCHEMA
                | ff::value::CONSTRAINT,
            ) => true,
            Some(ff::value::OBJECT | ff::schema::OPT_OBJECT) => EncodedConstraint::validate(&slice[1..]),
            _ => false,
        }
    }
}

/// A partially decoded schema.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PartiallyDecodedSchema<'a> {
    None = ff::schema::NONE,
    Integer = ff::value::INTEGER,
    Float = ff::value::FLOAT,
    Character = ff::value::CHARACTER,
    Duration = ff::value::DURATION,
    DateTime = ff::value::DATE_TIME,
    Object(&'a EncodedConstraint) = ff::value::OBJECT,
    OptionalObject(&'a EncodedConstraint) = ff::schema::OPT_OBJECT,
    Language = ff::value::LANGUAGE,
    Url = ff::value::URL,
    Color = ff::value::COLOR,
    Schema = ff::value::SCHEMA,
    Contraint = ff::value::CONSTRAINT,
    Email = ff::value::EMAIL,
    Text = ff::value::TEXT,
    Binary = ff::value::BINARY,
}

impl<'a> PartiallyDecodable for &'a EncodedSchema {
    type PartialOutput = PartiallyDecodedSchema<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        match self.0.get(0).copied() {
            Some(ff::schema::NONE) => PartiallyDecodedSchema::None,
            Some(ff::value::INTEGER) => PartiallyDecodedSchema::Integer,
            Some(ff::value::FLOAT) => PartiallyDecodedSchema::Float,
            Some(ff::value::CHARACTER) => PartiallyDecodedSchema::Character,
            Some(ff::value::DURATION) => PartiallyDecodedSchema::Duration,
            Some(ff::value::DATE_TIME) => PartiallyDecodedSchema::DateTime,
            Some(ff::value::OBJECT) => PartiallyDecodedSchema::Object(unsafe {
                EncodedConstraint::new_unchecked(&self.0[1..])
            }),
            Some(ff::schema::OPT_OBJECT) => PartiallyDecodedSchema::OptionalObject(unsafe {
                EncodedConstraint::new_unchecked(&self.0[1..])
            }),
            Some(ff::value::LANGUAGE) => PartiallyDecodedSchema::Language,
            Some(ff::value::URL) => PartiallyDecodedSchema::Url,
            Some(ff::value::COLOR) => PartiallyDecodedSchema::Color,
            Some(ff::value::SCHEMA) => PartiallyDecodedSchema::Schema,
            Some(ff::value::CONSTRAINT) => PartiallyDecodedSchema::Contraint,
            Some(ff::value::EMAIL) => PartiallyDecodedSchema::Email,
            Some(ff::value::TEXT) => PartiallyDecodedSchema::Text,
            Some(ff::value::BINARY) => PartiallyDecodedSchema::Binary,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<'a> Decodable for PartiallyDecodedSchema<'a> {
    type Output = Schema;

    fn decode(&self) -> Self::Output {
        use PartiallyDecodedSchema as P;

        match self {
            P::None => Schema::None,
            P::Integer => Schema::Integer,
            P::Float => Schema::Float,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
