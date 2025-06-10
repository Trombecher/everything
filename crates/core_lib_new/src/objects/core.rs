use super::*;
use crate::content::{DecodedRow, Row};
use crate::values::{DecodedValue, Schema};

macro_rules! define_objects {
    ($($id:ident = $e:expr)+) => {
        $(
            pub const $id: ObjectId = NonZeroU64::new($e).unwrap();
        )+
    };
}

define_objects![
    TAG = 1
    TAG_SCHEMA = 2
    TAG_PARENT = 3
    TAG_CONSTRAINT = 4
    TAG_INFERRED = 5
    TAG_INHERITABLE = 6
    TAG_UNIQUE_ID = 7
    TAG_UNIQUE_VALUE = 8
    CONSTRAINT = 9
    CONSTRAINT_LEFT = 10
    CONSTRAINT_RIGHT = 11
    CONSTRAINT_TAG = 12
    CONSTRAINT_WITH_VALUE = 13
    HAS_TAG_CONSTRAINT = 14
];

pub static ROWS: [Row; _] = [
    // Object "Tag"
    Row::encode(DecodedRow::Association(TAG, TAG_SCHEMA, DecodedValue::Unit)),
    Row::encode(DecodedRow::Association(
        TAG,
        TAG_CONSTRAINT,
        DecodedValue::Unit,
    )), // TODO: $ has Tag.Schema
    Row::encode(DecodedRow::Association(
        TAG,
        TAG_INFERRED,
        DecodedValue::Unit,
    )),
    // Object "Tag.Schema"
    Row::encode(DecodedRow::Association(
        TAG_SCHEMA,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::Schema),
    )),
    Row::encode(DecodedRow::Association(
        TAG_SCHEMA,
        TAG_PARENT,
        DecodedValue::ObjectReference(TAG),
    )),
    Row::encode(DecodedRow::Association(
        TAG_SCHEMA,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(HAS_TAG_CONSTRAINT),
    )),
    // Object "Tag.Parent"
    Row::encode(DecodedRow::Association(
        TAG_PARENT,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::ObjectReference(Some(HAS_TAG_CONSTRAINT))),
    )),
    Row::encode(DecodedRow::Association(
        TAG_PARENT,
        TAG_PARENT,
        DecodedValue::ObjectReference(TAG),
    )),
    Row::encode(DecodedRow::Association(
        TAG_PARENT,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(HAS_TAG_CONSTRAINT),
    )),
    // Object "Tag.Constraint"
    Row::encode(DecodedRow::Association(
        TAG_CONSTRAINT,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::ObjectReference(Some(CONSTRAINT))),
    )),
    Row::encode(DecodedRow::Association(
        TAG_CONSTRAINT,
        TAG_PARENT,
        DecodedValue::ObjectReference(TAG),
    )),
    Row::encode(DecodedRow::Association(
        TAG_CONSTRAINT,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(HAS_TAG_CONSTRAINT),
    )),
    Row::encode(DecodedRow::Association(
        TAG_CONSTRAINT,
        TAG_UNIQUE_ID,
        DecodedValue::Unit,
    )),
    // Object "Tag.Inferred"
    Row::encode(DecodedRow::Association(
        TAG_INFERRED,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::Unit),
    )),
    Row::encode(DecodedRow::Association(
        TAG_INFERRED,
        TAG_PARENT,
        DecodedValue::ObjectReference(TAG),
    )),
    Row::encode(DecodedRow::Association(
        TAG_INFERRED,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(HAS_TAG_CONSTRAINT),
    )),
    // Object "Tag.UniqueId"
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_ID,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::Unit),
    )),
    // TODO: unique id among objects
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_ID,
        TAG_PARENT,
        DecodedValue::ObjectReference(TAG),
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_ID,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(HAS_TAG_AND_NOT_INFERRED_CONSTRAINT),
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_ID,
        TAG_UNIQUE_ID,
        DecodedValue::Unit,
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_ID,
        TAG_UNIQUE_VALUE,
        DecodedValue::Unit,
    )),
    // Object "Tag.UniqueValue"
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_VALUE,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::Unit),
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_VALUE,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::ObjectReference(None)),
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_VALUE,
        TAG_PARENT,
        DecodedValue::ObjectReference(TAG),
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_VALUE,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(HAS_TAG_AND_NOT_INFERRED_CONSTRAINT),
    )),
    Row::encode(DecodedRow::Association(
        TAG_UNIQUE_VALUE,
        TAG_UNIQUE_ID,
        DecodedValue::Unit,
    )),
    // Object "Tag.Constraint"
    Row::encode(DecodedRow::Association(
        CONSTRAINT,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::Unit),
    )),
    // Object "Constraint.Left"
    Row::encode(DecodedRow::Association(
        CONSTRAINT_LEFT,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::ObjectReference(Some(CONSTRAINT))),
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_LEFT,
        TAG_PARENT,
        DecodedValue::ObjectReference(CONSTRAINT),
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_LEFT,
        TAG_UNIQUE_ID,
        DecodedValue::Unit,
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_LEFT,
        TAG_CONSTRAINT,
        HAS_RIGHT_CONSTRAINT,
    )),
    // Object "Constraint.Right"
    Row::encode(DecodedRow::Association(
        CONSTRAINT_RIGHT,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::ObjectReference(Some(CONSTRAINT))),
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_RIGHT,
        TAG_PARENT,
        DecodedValue::ObjectReference(CONSTRAINT),
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_RIGHT,
        TAG_UNIQUE_ID,
        DecodedValue::Unit,
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_RIGHT,
        TAG_CONSTRAINT,
        HAS_LEFT_CONSTRAINT,
    )),
    // Object "Constraint.Tag"
    Row::encode(DecodedRow::Association(
        CONSTRAINT_TAG,
        TAG_SCHEMA,
        DecodedValue::Schema(Schema::ObjectReference(Some(TAG))),
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_TAG,
        TAG_PARENT,
        DecodedValue::ObjectReference(CONSTRAINT)
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_TAG,
        TAG_CONSTRAINT,
        DecodedValue::ObjectReference(CONSTRAINT)
    )),
    Row::encode(DecodedRow::Association(
        CONSTRAINT_TAG,
        TAG_UNIQUE_ID,
        DecodedValue::Unit,
    )),
    // Object "Has tag constraint" root
    Row::encode(DecodedRow::Association(
        HAS_TAG_CONSTRAINT,
        CONSTRAINT_TAG,
        DecodedValue::ObjectReference(TAG)
    )),
    Row::encode(DecodedRow::Association(
        HAS_TAG_CONSTRAINT
    ))
];
