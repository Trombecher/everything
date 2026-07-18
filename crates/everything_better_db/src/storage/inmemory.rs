use core::slice;
use std::{io, num::NonZeroUsize};

use memmap2::MmapMut;

use crate::{
    pages::{MetaPage, Page},
    storage::Storage,
};

pub struct InMemoryStorage {
    /// INVARIANT: the length of this map is
    /// a multiple of [`Page::SIZE`] AND the pointer
    /// is aligned to [`Page::SIZE`].
    map: MmapMut,
}

impl InMemoryStorage {
    pub fn new(max_pages: NonZeroUsize) -> Result<Self, io::Error> {
        let map = MmapMut::map_anon(max_pages.get() * Page::SIZE)?;

        if !map.as_ptr().is_aligned_to(Page::SIZE) {
            panic!("got a memory map slice that is not aligned to OS page size.")
        }

        Ok(Self { map })
    }

    fn pages(&self) -> &[Page] {
        let bytes = self.map.as_ref();

        // SAFETY: `bytes` is aligned to Page::Size
        // and Page is equivalent to [u8; Page::Size].
        unsafe { slice::from_raw_parts(bytes.as_ptr() as _, bytes.len() / Page::SIZE) }
    }
}

impl Storage for InMemoryStorage {
    fn meta_page(&self) -> &MetaPage {
        unsafe { self.pages().first().unwrap_unchecked() }.as_ref()
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn page(&self, index: usize) -> Option<&Page> {
        self.pages().get(index)
    }
}
