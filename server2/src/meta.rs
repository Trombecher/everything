//! This module exposes a [Meta] struct, that can be used to get/set metadata of the database.

use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    sync::Mutex,
};

use memmap2::MmapMut;

use crate::ObjectId;

const MAGIC_BYTES: [u8; 10] = *b"EVERYTHING";
const FILE_LEN: u64 = 128;

mod offsets {
    /// A `u32` indicating the version number.
    pub const VERSION: usize = 10;

    /// A `u64` indicating the last created object. If no objects were created yet, this is 0.
    pub const SEQUENCE: usize = 10;
}

pub struct Meta {
    file_handle: File,
    map: Mutex<MmapMut>,
}

impl Meta {
    pub fn new(root: &Path) -> Result<Self, ()> {
        let mut file_path = PathBuf::from(root);
        file_path.push("everything");

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|_| ())?;

        file.set_len(FILE_LEN);

        let mut map = unsafe { MmapMut::map_mut(&file).map_err(|_| ())? };

        // Ensure magic bytes
        if map[0..10] != MAGIC_BYTES {
            map[0..10].copy_from_slice(&MAGIC_BYTES);
        }

        Ok(Self {
            map: Mutex::new(map),
            file_handle: file,
        })
    }

    /// Reads a `T` from the location offset. `offset` must be smaller than 4096.
    #[inline]
    unsafe fn read<T: Copy>(&self, offset: usize) -> T {
        if offset as u64 >= FILE_LEN {
            panic!("Offset {} out of bounds for file size {}", offset, FILE_LEN)
        }

        if offset % align_of::<T>() != 0 {
            panic!("Unaligned read at offset {}", offset);
        }

        let lock = self.map.lock().unwrap();
        unsafe { *(lock.get_unchecked(offset) as *const u8).cast::<T>() }
    }

    #[inline]
    unsafe fn write<T: Copy>(&self, offset: usize, value: T, flush: bool) -> Result<(), ()> {
        if offset as u64 >= FILE_LEN {
            panic!("Offset {} out of bounds for file size {}", offset, FILE_LEN)
        }

        if offset % align_of::<T>() != 0 {
            panic!("Unaligned write at offset {}", offset);
        }

        let mut lock = self.map.lock().unwrap();
        unsafe {
            *(lock.get_unchecked_mut(offset) as *mut u8).cast::<T>() = value;
        }

        if flush {
            lock.flush_async_range(offset, size_of::<T>())
                .map_err(|_| ())?;
        }

        Ok(())
    }

    #[inline]
    pub fn sequence(&self) -> u64 {
        unsafe { self.read(offsets::SEQUENCE) }
    }

    #[inline]
    pub fn set_sequence(&self, value: u64) -> Result<(), ()> {
        // TODO: `false` for `flush` ?
        unsafe { self.write(offsets::SEQUENCE, value, false) }
    }

    /// Reads the current sequence, increments it and stores the result.
    /// Returns the incremented sequence value.
    #[inline]
    pub fn next_object_id(&self) -> Result<ObjectId, ()> {
        let lock = self.map.lock().unwrap();

        todo!()
    }
}
