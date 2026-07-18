use std::{num::NonZeroU64, sync::Mutex};

use crate::{
    error::Error,
    pages::{FreePage, Page, PageId},
    storage::Storage,
};

pub struct PageAllocator<S: Storage> {
    pub storage: S,
    /// Lock for concurrent access to the "allocator".
    lock: Mutex<()>,
}

impl<S: Storage> PageAllocator<S> {
    #[must_use]
    pub const fn new(storage: S) -> Self {
        Self {
            storage,
            lock: Mutex::new(()),
        }
    }

    #[inline(always)]
    pub fn free<P>(&self, page_id: PageId<P>) -> Result<(), Error>
    where
        Page: AsRef<P>,
    {
        self._free(page_id.id)
    }

    fn _free(&self, id: NonZeroU64) -> Result<(), Error> {
        let page_to_free = self.storage.resolve_page(PageId::<FreePage>::new(id))?;

        let _lock = self.lock.lock().unwrap();

        let current_next_free_page_id = self.storage.meta_page().allocator_next_free_page.raw.get();
        page_to_free.next_page.raw.set(current_next_free_page_id);

        self.storage
            .meta_page()
            .allocator_next_free_page
            .raw
            .set(id.get());

        // TODO: maybe give pages at the end of the initialized region
        // back to the uninialized region (?)

        Ok(())
    }

    pub fn allocate(&self) -> Result<&Page, Error> {
        let _lock = self.lock.lock().unwrap();

        match self.storage.meta_page().allocator_next_free_page.id() {
            Some(free_page_id) => {
                let free_page = self.storage.resolve_page(free_page_id)?;

                // Update next free page to be the next page in the first free page.
                self.storage
                    .meta_page()
                    .allocator_next_free_page
                    .raw
                    .set(free_page.next_page.raw.get());

                Ok(free_page.as_ref())
            }
            None => {
                // The list is empty, so we have to take one page from
                // the unclaimed pages.

                let next_page =
                    NonZeroU64::new(self.storage.meta_page().allocator_pages_initialized.get())
                        .map(PageId::<Page>::new)
                        .ok_or(Error::OutOfPages)?;

                let page = self
                    .storage
                    .resolve_page(next_page)
                    .map_err(|_| Error::OutOfPages)?;

                // Increment pages initialized.
                self.storage
                    .meta_page()
                    .allocator_pages_initialized
                    .set(next_page.id.get().wrapping_add(1));

                Ok(page)
            }
        }
    }
}
