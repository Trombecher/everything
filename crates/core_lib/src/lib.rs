#![allow(uncommon_codepoints)]
#![allow(non_ascii_idents)]

pub mod ff;
pub mod error;
pub mod features;
pub mod meta;
pub mod objects;
pub mod res;
pub mod rows;
pub mod values;
pub mod versioning;

use crate::error::{Error, EverythingError};
use crate::ff::{OBJECTS_TAGS_PATH, RESOURCES_PATH, TAGS_OBJECTS_PATH};
use crate::objects::ObjectId;
use crate::res::ResourceId;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use meta::Meta;
use std::num::NonZeroU64;
use std::path;
use std::path::Path;
use tokio::fs::File;
use crate::values::Value;

const MAX_PATH: usize = 260;

type PathBuf = path::PathBuf;

#[inline]
fn encode_id(id: NonZeroU64, path: &mut PathBuf) {
    const BASE64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-";

    let mut id = id.get();

    while id > 0 {
        let digit = (id % 64) as usize;

        // path.push(BASE64[digit] as char);

        id /= 64;
    }
}

pub struct Database {
    /// The root of everything.
    root: PathBuf,

    /// The root of the resources.
    resources_root: PathBuf,

    /// Object -> (Tag, Value) index files root
    ot_root: PathBuf,

    /// Tag -> (Object, Value) index files root
    to_root: PathBuf,

    /// The memory-mapped metadata.
    meta: Meta,

    object_files: DashMap<ObjectId, File>,
    resources: DashMap<ResourceId, File>,
}

impl Database {
    pub async fn open(root: &Path) -> Result<Self, Error> {
        if !root.exists() {
            return EverythingError::RootPathDoesNotExist.into();
        }

        let meta = Meta::open(root).await?;

        Ok(Self {
            root: root.to_path_buf(),
            resources_root: root.join(RESOURCES_PATH).to_path_buf(),
            ot_root: root.join(OBJECTS_TAGS_PATH).to_path_buf(),
            to_root: root.join(TAGS_OBJECTS_PATH).to_path_buf(),
            meta,
            object_files: DashMap::new(),
            resources: DashMap::new(),
        })
    }

    async fn load_resource(
        &self,
        resource_id: ResourceId,
    ) -> Result<Option<Ref<ResourceId, File>>, Error> {
        if let Some(file) = self.resources.get(&resource_id) {
            Ok(Some(file))
        } else {
            let mut resource_path = self.resources_root.clone();
            encode_id(resource_id, &mut resource_path);

            Ok(None)

            // match File::open(resource_path).await {
            //     Ok(file) => Ok(Some(file)),
            //     Err(_) => Ok(None)
            // }
        }
    }

    pub fn activate_feature() {}

    pub async fn get_value(&self) -> Result<Option<Value>, Error> {
        todo!()
    }

    #[inline]
    pub fn sequence(&self) -> u64 {
        self.meta.sequence()
    }
}
