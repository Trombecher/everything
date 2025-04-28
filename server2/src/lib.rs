#![feature(file_lock)]
#![allow(uncommon_codepoints)]

mod ff;
mod istr;

pub mod constraints;
pub mod decode;
pub mod email;
pub mod lang;
pub mod meta;
pub mod res;
pub mod rows;
pub mod schema;
pub mod stmt;
pub mod time;
pub mod values;
pub mod objects;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use meta::Meta;
use tokio::fs::File;
use values::{EncodedValue, PartiallyDecodedValue, RowValueSlot};
use crate::objects::ObjectId;

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
        todo!()
    }

    pub async fn create(&self) -> Result<ObjectId, ()> {
        todo!()
    }

    pub async fn associate(&self, target: ObjectId, tag: ObjectId, value: Option<&EncodedValue>) {
        todo!()
    }
}
