#![allow(uncommon_codepoints)]

pub mod values;
pub mod schema;
pub mod constraints;
pub mod ff;
pub mod decode;
pub mod lang;
pub mod stmt;
pub mod meta;
pub mod email;
pub mod time;

use std::{collections::HashMap, fs, num::NonZeroU64, path::{Path, PathBuf}};

use meta::Meta;
use tokio::fs::File;
use values::EncodedValue;

pub type ObjectId = NonZeroU64;

pub struct Database {
    root: PathBuf,
    meta: Meta,
    object_files: HashMap<ObjectId, File>
}

impl Database {
    pub fn new(root: &Path) -> Result<Self, ()> {
        if !root.exists() {
            return Err(())
        }

        

        todo!()
    }

    pub async fn create(&self) -> Result<ObjectId, ()> {
        self.meta.next_object_id()
    }

    pub async fn associate(&self, target: ObjectId, tag: ObjectId, value: Option<&EncodedValue>) {
        todo!()
    }
}