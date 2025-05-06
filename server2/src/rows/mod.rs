use std::mem::transmute;

use crate::{values::ValueRow};
use crate::objects::ObjectId;

pub struct UsedRow {
    tag_object: ObjectId,
    value: ValueRow,
}

pub struct FreeRow {
    next_free_row: u64,
}

pub enum Row {
    Used(UsedRow),
    Free(FreeRow),
}