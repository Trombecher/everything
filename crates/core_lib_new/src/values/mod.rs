mod schema;
mod time;
pub(crate) mod inline;
mod color;

use std::num::NonZeroU64;
pub use schema::*;
pub use time::*;

use crate::objects::ObjectId;
use crate::values::color::Color;
use crate::values::inline::InlineContent;

#[derive(PartialEq, Clone)]
pub enum Value {
    Unit,
    Integer(i64),
    Float(f64),
    Character(char),
    Duration(Duration),
    DateTime(DateTime),
    ObjectReference(ObjectId),
    Schema(Schema),
    Language(),
    Url(),
    Color(Color),
    Email(),
    Text(),
    Binary(VariableContent),
    Encrypted(),
}

#[derive(PartialEq, Clone)]
pub enum VariableContent {
    Inline(InlineContent),
    InlineMax([u8; 15]),
    Reference(NonZeroU64)
}
