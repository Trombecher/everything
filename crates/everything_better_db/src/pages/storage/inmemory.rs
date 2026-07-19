use core::slice;
use std::{convert::Infallible, io, num::NonZeroUsize, ptr::NonNull};

use memmap2::MmapMut;

use crate::{
    convert::safe_u64_to_usize,
    pages::{
        RawPageId,
        storage::{OpaquePage, OpaquePageReference, Storage},
    },
};

pub struct InMemoryStorage {
    /// INVARIANT: the length of this map is
    /// a multiple of [`Page::SIZE`] AND the pointer
    /// is aligned to [`Page::SIZE`].
    map: MmapMut,
}

impl InMemoryStorage {
    pub fn new(max_pages: NonZeroUsize) -> Result<Self, io::Error> {
        let map = MmapMut::map_anon(max_pages.get() * OpaquePage::SIZE)?;

        if !map.as_ptr().is_aligned_to(OpaquePage::SIZE) {
            panic!("got a memory map slice that is not aligned to OS page size.")
        }

        Ok(Self { map })
    }

    fn pages(&self) -> &[OpaquePage] {
        let bytes = self.map.as_ref();

        // SAFETY: `bytes` is aligned to Page::Size
        // and Page is equivalent to [u8; Page::Size].
        unsafe { slice::from_raw_parts(bytes.as_ptr() as _, bytes.len() / OpaquePage::SIZE) }
    }
}

impl Storage for InMemoryStorage {
    type Error = Infallible;

    fn flush(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn page<'page>(&'page self, page_id: RawPageId) -> Option<OpaquePageReference<'page>> {
        self.pages()
            .get(safe_u64_to_usize(page_id))
            .map(|page| unsafe { OpaquePageReference::new(NonNull::from(page)) })
    }
}
