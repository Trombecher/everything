//! Dummy structs that lock enum fields of [Value] into the right place.

use crate::objects::ObjectId;

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct I64 {
    _padding: [u8; 7],
    pub value: i64,
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct F64 {
    _padding: [u8; 7],
    pub value: i64,
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct Char {
    _padding: [u8; 3],
    pub value: u32,
    _padding2: [u8; 8],
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct ObjectId1 {
    _padding: [u8; 7],
    pub value: ObjectId,
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct ObjectId2 {
    _padding: [u8; 6],
    pub value: Option<ObjectId>,
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct Language {
    _padding: [u8; 1],
    pub value: crate::values::Language,
    _padding2: [u8; 12],
}
