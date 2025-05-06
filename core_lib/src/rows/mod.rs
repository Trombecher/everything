use crate::objects::ObjectId;
use crate::values::Value;

pub struct UsedRow {
    tag_object: ObjectId,
    value: Value,
}

pub struct FreeRow {
    next_free_row: u64,
}

pub enum Row {
    Used(UsedRow),
    Free(FreeRow),
}