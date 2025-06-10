use crate::objects::ObjectId;

#[derive(PartialEq, Clone)]
pub enum Schema {
    Unit,
    Integer,
    Float,
    Character,
    Duration,
    DateTime,
    
    /// Object reference, optionally with a constraint object.
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