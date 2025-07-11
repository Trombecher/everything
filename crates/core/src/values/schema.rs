use crate::ff;
use crate::values::v;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Schema {
    Unit = ff::VALUE_UNIT,
    Integer = ff::VALUE_INTEGER,
    Float = ff::VALUE_FLOAT,
    Character = ff::VALUE_CHARACTER,
    Duration = ff::VALUE_DURATION,
    DateTime = ff::VALUE_DATE_TIME,

    /// Object reference, optionally with an object with constraint.
    ObjectReference(v::ObjectId2) = ff::VALUE_OBJECT_REFERENCE,
    Schema = ff::VALUE_SCHEMA,
    Language = ff::VALUE_LANGUAGE,
    Url = ff::VALUE_URL,
    Color = ff::VALUE_COLOR,
    Email = ff::VALUE_EMAIL,
    Text = ff::VALUE_TEXT,
    Binary = ff::VALUE_BINARY,
    Encrypted = ff::VALUE_ENCRYPTED,
}
