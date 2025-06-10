mod schema;
mod time;

pub use schema::*;
pub use time::*;

use crate::ff;
use crate::objects::ObjectId;

#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum DecodedValue {
    Unit,
    Integer(i64),
    Float(f64),
    Character(char),
    Duration(Duration),
    DateTime(DateTime),
    ObjectReference(ObjectId),
    Schema(Schema),
    Language(),
    Url(),
    Color(),
    Email(),
    Text(),
    Binary(),
    Encrypted()
}
