use std::num::NonZeroU64;

pub type ObjectId = NonZeroU64;

pub mod core {
    use super::*;
    use crate::content::{DecodedRow, Row};
    use crate::values::{DecodedValue, Schema};

    pub static ROWS: [Row; _] = [
        // Object "Tag"
        Row::encode(DecodedRow::Association(TAG, TAG_SCHEMA, None)),
        Row::encode(DecodedRow::Association(TAG, TAG_CONSTRAINT, None)), // TODO: $ has Tag.Schema and $ has (Tag.UniqueId or Tag.UniqueValue)
        Row::encode(DecodedRow::Association(TAG, TAG_INFERRED, None)),
        // Object "Tag.Schema"
        Row::encode(DecodedRow::Association(TAG_SCHEMA, TAG_SCHEMA, Some(DecodedValue::Schema(Schema::Schema)))),
        Row::encode(DecodedRow::Association(
            TAG_SCHEMA,
            TAG_PARENT,
            Some(DecodedValue::ObjectReference(Some(TAG))),
        )),
        Row::encode(DecodedRow::Association(TAG_SCHEMA, TAG_CONSTRAINT, None)), // TODO: $ has Tag
        Row::encode(DecodedRow::Association(TAG_SCHEMA, TAG_UNIQUE_ID, None)),
    ];

    pub const TAG: ObjectId = NonZeroU64::new(43).unwrap();
    pub const TAG_SCHEMA: ObjectId = NonZeroU64::new(74).unwrap();
    pub const TAG_PARENT: ObjectId = NonZeroU64::new(75).unwrap();
    pub const TAG_CONSTRAINT: ObjectId = NonZeroU64::new(79).unwrap();
    pub const TAG_INFERRED: ObjectId = NonZeroU64::new(80).unwrap();
    pub const TAG_INHERITABLE: ObjectId = NonZeroU64::new(81).unwrap();
    pub const TAG_UNIQUE_ID: ObjectId = NonZeroU64::new(82).unwrap();
    pub const TAG_UNIQUE_VALUE: ObjectId = NonZeroU64::new(84).unwrap();
}
