use crate::{decode::PartiallyDecodable, rows::Row};
use std::slice::from_raw_parts;

use crate::{
    ObjectId,
    istr::InlineStr,
    lang::Language,
    res::ResourceId,
    time::{DateTime, Duration},
};

use super::{EncodedValue, PartiallyDecodedValue};

/// This struct contains the three `u64`s, making up a value in a [Row].
///
/// Because this comes from disk, we cannot rely on the correctness of the value encoding,
/// as the data may have been altered from outside.
pub struct RowValueSlot(pub [u64; 3]);

#[repr(transparent)]
pub struct RowStr(RowBytes);

pub enum RowBytes {
    Extern(ResourceId),
    Max(InlineStr<31>),
}
