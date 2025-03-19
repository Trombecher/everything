use std::fmt;

use crate::{constraints::{Constraint, EncodedConstraint}, decode::{Decodable, PartiallyDecodable}, ff};

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
    Contraint(Constraint) = ff::value::CONSTRAINT,
    Email = ff::value::EMAIL,
    Text = ff::value::TEXT,
    Binary = ff::value::BINARY,

    // TODO: encrypted values in schema definition
}

/// An encoded schema.
#[derive(Debug, PartialEq)]
pub struct EncodedSchema([u8]);

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
    Contraint(&'a EncodedConstraint) = ff::value::CONSTRAINT,
    Email = ff::value::EMAIL,
    Text = ff::value::TEXT,
    Binary = ff::value::BINARY,
}

impl<'a> PartiallyDecodable for &'a EncodedSchema {
    type PartialOutput = PartiallyDecodedSchema<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        todo!()
    }
}

impl<'a> Decodable for PartiallyDecodedSchema<'a> {
    type Output = Schema;

    fn decode(&self) -> Self::Output {
        todo!()
    }
}