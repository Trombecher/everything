use std::fmt;

use crate::{constraints::Constraint, ff};

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

pub struct EncodedSchema([u8]);

#[derive(PartialEq, Copy, Clone)]
pub struct SchemaRef<'a>(&'a [u8]);

impl<'a> SchemaRef<'a> {
    pub fn new(_source: &'a [u8]) -> Result<Self, ()> {
        todo!()
    }

    pub fn parse(self) -> Schema {
        todo!()
    }
}

impl fmt::Debug for SchemaRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SchemaRef")
            .field(&self.0)
            .finish()
    }
}