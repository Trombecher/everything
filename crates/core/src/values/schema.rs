use crate::ff;
use crate::objects::ObjectId;

#[derive(PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum Schema {
    Unit = ff::UNIT,
    Integer = ff::INTEGER,
    Float = ff::FLOAT,
    Character = ff::CHARACTER,
    Duration = ff::DURATION,
    DateTime = ff::DATE_TIME,

    /// Object reference, optionally with an object with constraint `$CONSTRAINT`.
    ObjectReference(Option<ObjectId>) = ff::OBJECT_REFERENCE,
    Schema = ff::SCHEMA,
    Language = ff::LANGUAGE,
    Url = ff::URL,
    Color = ff::COLOR,
    Email = ff::EMAIL,
    Text = ff::TEXT,
    Binary = ff::BINARY,
    Encrypted = ff::ENCRYPTED,
}
