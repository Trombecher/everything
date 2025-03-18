#![feature(if_let_guard)]

mod values;
mod schema;
mod constraints;
mod ff;

use std::num::NonZeroU64;

use values::{Value, ValueRef};

pub type ObjectId = NonZeroU64;

pub struct Database {

}

// Row:
//
// aabbccddeeffgghh

impl Database {
    fn resolve_exact(name: &str) -> Option<ObjectId> {
        
    }

    pub fn set(target: ObjectId, tag: ObjectId, value: Option<ValueRef>) {

    }
}