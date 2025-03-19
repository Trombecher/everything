mod values;
mod schema;
mod constraints;
mod ff;
mod decode;

use std::num::NonZeroU64;

use values::{Value};

pub type ObjectId = NonZeroU64;

pub struct Database {

}