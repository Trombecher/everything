mod schema;
mod time;

pub use schema::*;
pub use time::*;

use crate::ff;
use crate::objects::ObjectId;

#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum DecodedValue {
    Integer(i64) = ff::INTEGER,
    Float(f64) = ff::FLOAT,
    Character(char) = ff::CHAR,
    Duration(Duration),
    DateTime(DateTime),
    ObjectReference(Option<ObjectId>),
    Schema(Schema),
    Constraint(),
    Language(),
    Url(),
    Color(),
    Email(),
    Text(),
    Binary(),
    EncryptedEmail(),
    EncryptedText(),
    EncryptedBinary(),
}
