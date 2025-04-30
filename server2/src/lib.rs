#![feature(file_lock)]
#![allow(uncommon_codepoints)]

mod ff;
mod istr;

pub mod constraints;
pub mod decode;
pub mod email;
mod error;
mod features;
pub mod lang;
pub mod meta;
pub mod objects;
pub mod res;
pub mod rows;
pub mod schema;
pub mod stmt;
pub mod time;
pub mod values;

use crate::objects::ObjectId;
use crate::res::ResourceId;
use dashmap::DashMap;
use meta::Meta;
use spathbuf::StackPathBuf;
use std::num::NonZeroU64;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use dashmap::mapref::one::Ref;
use tokio::fs::File;
use values::EncodedValue;
use crate::error::Error;

const MAX_PATH: usize = 260;

#[inline]
fn encode_id(id: NonZeroU64, path: &mut StackPathBuf<MAX_PATH>) {
    const BASE64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-";

    let mut id = id.get();

    while id > 0 {
        let digit = (id % 64) as usize;

        unsafe {
            #[cfg(windows)]
            path.unsafe_push_unit(BASE64[digit] as u16);

            // TODO: unix
        }

        id /= 64;
    }
}

pub struct Database {
    /// The root of everything.
    root: StackPathBuf<MAX_PATH>,

    /// The root of the resources.
    resources_root: StackPathBuf<MAX_PATH>,

    /// Object -> (Tag, Value) index files root
    ot_root: StackPathBuf<MAX_PATH>,

    /// Tag -> (Object, Value) index files root
    to_root: StackPathBuf<MAX_PATH>,

    /// The memory-mapped metadata.
    meta: Meta,

    object_files: DashMap<ObjectId, File>,
    resources: DashMap<ResourceId, File>,
}

impl Database {
    pub fn new(root: &Path) -> Result<Self, ()> {
        if !root.exists() {
            return Err(());
        }

        todo!()
    }

    async fn load_resource(&self, resource_id: ResourceId) -> Result<Option<Ref<ResourceId, File>>, Error> {
        if let Some(file) = self.resources.get(&resource_id) {
            Ok(Some(file))
        } else {
            let mut resource_path = self.resources_root.clone();
            unsafe {
                resource_path.unsafe_push_unit(0);
            }
            encode_id(resource_id, &mut resource_path);

            match File::open(resource_path).await {
                Ok(file) => {
                    
                    
                    Ok(Some(file))
                },
                Err(_) => {}
            }
            
        }
    }

    pub async fn associate(&self, target: ObjectId, tag: ObjectId, value: Option<&EncodedValue>) {
        todo!()
    }
}
