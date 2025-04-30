//! This module exposes a [Meta] struct, that can be used to get/set metadata of the database.

mod raw;

use crate::meta::raw::{MetaContent, RawMeta};
use crate::objects::CustomObjectId;
use std::{fs::OpenOptions, io, path::{Path, PathBuf}};

const MAGIC_BYTES: [u8; 12] = *b"EVERYTHINGKB";

#[repr(C, align(4096))]
struct MetaData {
    magic: [u8; 12],
    version: u32,
    sequence: u64,
}

unsafe impl MetaContent for MetaData {}

pub struct Meta {
    raw: RawMeta<MetaData>,
}

impl Meta {
    pub fn new(root: &Path) -> io::Result<Self> {
        let mut file_path = PathBuf::from(root);
        file_path.push("everything");

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        file.lock()?;
        
        let raw = RawMeta::<MetaData>::new(file);
        raw.lock().magic = MAGIC_BYTES;
        
        Ok(Self { raw })
    }
    
    #[inline]
    pub fn version(&self) -> u32 {
        self.raw.lock().version
    }
    
    /// Returns the current sequence
    #[inline]
    pub fn sequence(&self) -> u64 {
        self.raw.lock().sequence
    }

    #[inline]
    pub fn set_sequence(&self, value: u64) -> io::Result<()> {
        self.raw.lock().sequence = value;
        self.raw.flush()
    }

    /// Reads the current sequence, increments it and stores the result.
    /// Returns the incremented sequence value.
    #[inline]
    pub fn next_object_id(&self) -> io::Result<CustomObjectId> {
        let mut lock = self.raw.lock();
        let next_object_id = lock.sequence;
        
        if next_object_id == u64::MAX {
            todo!()
        }
        
        if let Ok(custom_object_id) = CustomObjectId::try_from(next_object_id) {
            lock.sequence = next_object_id + 1;
            Ok(custom_object_id)
        } else {
            todo!()
        }
    }
}
