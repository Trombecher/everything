#![allow(uncommon_codepoints)]

pub mod constraints;
pub mod decode;
pub mod email;
mod ff;
mod istr;
pub mod lang;
pub mod meta;
mod res;
pub mod rows;
pub mod schema;
pub mod stmt;
pub mod time;
pub mod values;

use std::{
    collections::HashMap,
    fs,
    num::NonZeroU64,
    path::{Path, PathBuf},
};

use meta::Meta;
use tokio::fs::File;
use values::{EncodedValue, PartiallyDecodedValue, RowValueSlot};

pub type ObjectId = NonZeroU64;

pub struct Database {
    root: PathBuf,
    meta: Meta,
    object_files: HashMap<ObjectId, File>,
}

impl Database {
    pub fn new(root: &Path) -> Result<Self, ()> {
        if !root.exists() {
            return Err(());
        }

        todo!()
    }

    pub(crate) fn decode_row_value(&self, row_value: RowValueSlot) -> PartiallyDecodedValue {
        match row_value.0[0] {}
    }

    pub async fn create(&self) -> Result<ObjectId, ()> {
        self.meta.next_object_id()
    }

    pub async fn associate(&self, target: ObjectId, tag: ObjectId, value: Option<&EncodedValue>) {
        todo!()
    }
}
