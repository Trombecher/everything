mod lang;
mod schema;
mod time;
mod color;

use arrayvec::{ArrayString, ArrayVec};
use std::num::NonZeroU64;

pub use lang::*;
pub use schema::*;
pub use time::*;
pub use color::*;

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

#[derive(PartialEq, Clone, Debug)]
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
    Url(NonZeroU64), // TODO: fix this
    Color(Color),
    Email(NonZeroU64), // TODO: fix this
    Text(SpillableText),
    Binary(SpillableBinary),
    /// There is no inline storage because encrypted data is always big.
    Encrypted(NonZeroU64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum SpillableText {
    InlineMax(ArrayString<15>),
    Allocated(NonZeroU64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum SpillableBinary {
    Inline(ArrayVec<u8, 15>),
    Allocated(NonZeroU64),
}
