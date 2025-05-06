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
    None = ff::NONE,
    Integer = ff::INTEGER,
    Float = ff::FLOAT,
    Character = ff::CHARACTER,
    Duration = ff::DURATION,
    DateTime = ff::DATE_TIME,
    Object(Constraint) = ff::OBJECT,
    OptionalObject(Constraint) = ff::OPT_OBJECT,
    Language = ff::LANGUAGE,
    Url = ff::URL,
    Color = ff::COLOR,
    Schema = ff::SCHEMA,
    Contraint = ff::CONSTRAINT,
    Email = ff::EMAIL,
    Text = ff::TEXT,
    Binary = ff::BINARY,
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
                ff::NONE
                | ff::INTEGER
                | ff::FLOAT
                | ff::CHARACTER
                | ff::DURATION
                | ff::DATE_TIME
                | ff::LANGUAGE
                | ff::URL
                | ff::COLOR
                | ff::EMAIL
                | ff::TEXT
                | ff::BINARY
                | ff::SCHEMA
                | ff::CONSTRAINT,
            ) => true,
            Some(ff::OBJECT | ff::OPT_OBJECT) => EncodedConstraint::validate(&slice[1..]),
            _ => false,
        }
    }
}

/// A partially decoded schema.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PartiallyDecodedSchema<'a> {
    None = ff::NONE,
    Integer = ff::INTEGER,
    Float = ff::FLOAT,
    Character = ff::CHARACTER,
    Duration = ff::DURATION,
    DateTime = ff::DATE_TIME,
    Object(&'a EncodedConstraint) = ff::OBJECT,
    OptionalObject(&'a EncodedConstraint) = ff::OPT_OBJECT,
    Language = ff::LANGUAGE,
    Url = ff::URL,
    Color = ff::COLOR,
    Schema = ff::SCHEMA,
    Contraint = ff::CONSTRAINT,
    Email = ff::EMAIL,
    Text = ff::TEXT,
    Binary = ff::BINARY,
}

impl<'a> PartiallyDecodable for &'a EncodedSchema {
    type PartialOutput = PartiallyDecodedSchema<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        match self.0.get(0).copied() {
            Some(ff::NONE) => PartiallyDecodedSchema::None,
            Some(ff::INTEGER) => PartiallyDecodedSchema::Integer,
            Some(ff::FLOAT) => PartiallyDecodedSchema::Float,
            Some(ff::CHARACTER) => PartiallyDecodedSchema::Character,
            Some(ff::DURATION) => PartiallyDecodedSchema::Duration,
            Some(ff::DATE_TIME) => PartiallyDecodedSchema::DateTime,
            Some(ff::OBJECT) => PartiallyDecodedSchema::Object(unsafe {
                EncodedConstraint::new_unchecked(&self.0[1..])
            }),
            Some(ff::OPT_OBJECT) => PartiallyDecodedSchema::OptionalObject(unsafe {
                EncodedConstraint::new_unchecked(&self.0[1..])
            }),
            Some(ff::LANGUAGE) => PartiallyDecodedSchema::Language,
            Some(ff::URL) => PartiallyDecodedSchema::Url,
            Some(ff::COLOR) => PartiallyDecodedSchema::Color,
            Some(ff::SCHEMA) => PartiallyDecodedSchema::Schema,
            Some(ff::CONSTRAINT) => PartiallyDecodedSchema::Contraint,
            Some(ff::EMAIL) => PartiallyDecodedSchema::Email,
            Some(ff::TEXT) => PartiallyDecodedSchema::Text,
            Some(ff::BINARY) => PartiallyDecodedSchema::Binary,
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
