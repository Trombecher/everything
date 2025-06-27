mod lang;
mod schema;
mod time;

use arrayvec::{ArrayString, ArrayVec};
pub use lang::*;
pub use schema::*;
use std::num::NonZeroU64;
pub use time::*;

use crate::objects::ObjectId;

#[derive(PartialEq, Clone)]
pub enum ConstValue {
    Unit,
    Schema(Schema),
    ObjectReference(ObjectId),
}

impl ConstValue {
    pub const fn const_into_value(self) -> Value {
        match self {
            ConstValue::Unit => Value::Unit,
            ConstValue::Schema(schema) => Value::Schema(schema),
            ConstValue::ObjectReference(object_id) => Value::ObjectReference(object_id),
        }
    }
}

impl Into<Value> for ConstValue {
    fn into(self) -> Value {
        self.const_into_value()
    }
}

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
    Language(Language),
    Url(NonZeroU64),
    Color(NonZeroU64),
    Email(NonZeroU64),
    Text(SpillableText),
    Binary(SpillableBinary),
    Encrypted(SpillableBinary),
}

#[derive(PartialEq, Clone)]
pub enum SpillableText {
    InlineMax(ArrayString<15>),
    Allocated(NonZeroU64),
}

#[derive(PartialEq, Clone)]
pub enum SpillableBinary {
    Inline(ArrayVec<u8, 15>),
    Allocated(NonZeroU64),
}
