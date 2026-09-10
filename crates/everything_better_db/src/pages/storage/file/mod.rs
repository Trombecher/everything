mod page_cache;
mod page_table;

use std::{
    fs::{File, OpenOptions},
    io,
    path::PathBuf,
};

use crate::pages::storage::{Storage, file::page_table::PageTable, pages::Pages};

pub struct FileBasedStorage {
    file_path: PathBuf,
    file: File,
    page_cache: Pages,
    page_table: PageTable,
}

impl FileBasedStorage {
    pub fn new(path: PathBuf, page_cache_size_in_pages: u64) -> Result<Self, io::Error> {
        let file = OpenOptions::new().read(true).write(true).open(&path)?;

        Ok(Self {
            file_path: path,
            file,
            page_cache: Pages::new(page_cache_size_in_pages)?,
            page_table: PageTable::new(page_cache_size_in_pages),
        })
    }
}

impl Storage for FileBasedStorage {
    type Error = io::Error;

    fn page(
        &self,
        page_id: crate::pages::RawPageId,
        skip_validation: bool,
    ) -> Option<super::PseudoReference<'_, super::PageWithCRC>> {
        if let Some(cache_slot_index) = self.page_table.cache_slot_index(page_id) {
            // Page found in page cache.

            super::PseudoReference::new(pointer)
        } else {
            // Page must be loaded into the cache.
        }
    }

    fn flush(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
