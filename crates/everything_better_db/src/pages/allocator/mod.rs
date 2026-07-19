#[cfg(test)]
mod tests;

use std::sync::Mutex;

use crate::pages::{
    FreePage, MetaPage, Page, PageId, RawPageId,
    mstorage::{self, ManagedStorage, PageReference},
    storage::Storage,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("out of pages")]
    OutOfPages,
    #[error("")]
    ManagedStorage(#[from] mstorage::Error),
}

pub struct PageAllocator<S: Storage> {
    pub mstorage: ManagedStorage<S>,
    /// Lock for concurrent access to the "allocator".
    lock: Mutex<()>,
}

impl<S: Storage> PageAllocator<S> {
    #[must_use]
    pub const fn new(storage: S) -> Self {
        Self {
            mstorage: ManagedStorage::new(storage),
            lock: Mutex::new(()),
        }
    }

    pub fn meta_page<'storage>(
        &'storage self,
    ) -> Result<PageReference<'storage, 'storage, MetaPage>, Error> {
        self.mstorage
            .page(PageId::<MetaPage>::new(0))
            .map_err(From::from)
    }

    // TODO: make this function not generic
    pub fn allocate<'page, P: Page>(&'page self) -> Result<PageReference<'page, 'page, P>, Error> {
        let _lock = self.lock.lock().unwrap();
        let meta_page = self.meta_page()?;
        let super_block = meta_page.super_block();

        match super_block.allocator_next_free_page.get() {
            free_page_id if free_page_id.raw != 0 => {
                let free_page = self.mstorage.page(free_page_id)?;

                // Update next free page to be the next page in the first free page.
                super_block
                    .allocator_next_free_page
                    .set(free_page.next_page.get());

                Ok(free_page
                    .cast()
                    .map_err(mstorage::Error::from)
                    .map_err(Error::from)?)
            }
            _ => {
                // The list is empty, so we have to take one page from
                // the unclaimed pages.

                // TODO: out of pages???
                let next_page_id = PageId::<P>::new(super_block.allocator_pages_initialized.get());

                let next_page = self.mstorage.page(next_page_id)?;

                // Increment pages initialized.
                super_block
                    .allocator_pages_initialized
                    .set(next_page_id.raw.wrapping_add(1));

                Ok(next_page)
            }
        }
    }

    #[inline(always)]
    pub fn free<P: Page>(&self, page_id: PageId<P>) -> Result<(), Error> {
        self._free(page_id.raw)
    }

    fn _free(&self, id: RawPageId) -> Result<(), Error> {
        let page_to_free = self.mstorage.page(PageId::<FreePage>::new(id))?;

        let _lock = self.lock.lock().unwrap();
        let meta_page = self.meta_page().map_err(Error::from)?;
        let super_block = meta_page.super_block();

        let current_next_free_page_id = super_block.allocator_next_free_page.get();
        page_to_free.next_page.set(current_next_free_page_id);

        super_block.allocator_next_free_page.raw.set(id);

        // TODO: maybe give pages at the end of the initialized region
        // back to the uninialized region (?)

        Ok(())
    }
}
