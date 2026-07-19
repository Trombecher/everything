use std::fs::File;

use memmap2::MmapMut;

use crate::{pages::Page, storage::Storage};

pub struct FileBasedStorage {
    file: File,
    mapped_content: MmapMut,
}
