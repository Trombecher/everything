use std::mem::transmute;

use crate::{values::RowValueSlot};
use crate::objects::ObjectId;

pub struct UsedRow {
    tag_object: ObjectId,
    value: RowValueSlot,
}

pub struct FreeRow {
    next_free_row: u64,
}

pub enum Row {
    Used(UsedRow),
    Free(FreeRow),
}

impl From<[u64; 4]> for Row {
    fn from(value: [u64; 4]) -> Self {
        unsafe { transmute(value) }
    }
}
