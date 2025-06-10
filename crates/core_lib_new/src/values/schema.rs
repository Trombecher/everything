use crate::objects::ObjectId;

#[derive(PartialEq, Clone)]
pub enum Schema {
    Unit,
    Integer,
    Float,
    Character,
    Duration,
    DateTime,
    
    /// Object reference, optionally with an object with constraint `$CONSTRAINT`.
    ObjectReference(Option<ObjectId>),
    Schema,
    Language,
    Url,
    Color,
    Email,
    Text,
    Binary,
    Encrypted
}