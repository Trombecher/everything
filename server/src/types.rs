use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::objects::ObjectId;

#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PrimitiveType {
    Decimal = 2,
    Integer = 3,
    String = 4,
    Duration = 5,
    DateTime = 6,
    Boolean = 7,
    Character = 8,
    URL = 9,
    Binary = 10,
    Color = 11,
    Email = 12,
}

/// An object id that is supposed to be a type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserDefinedTypeId(pub ObjectId);