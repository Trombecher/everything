#[cfg(test)]
mod tests;

use std::{
    hint::cold_path,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use tracing::warn;

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
    #[error("validation failed for both meta pages")]
    ValidationFailedForBothMetaPages,
}

struct AtomicCurrentMetaPage(AtomicBool);

impl AtomicCurrentMetaPage {
    const fn new(mp: CurrentMetaPage) -> Self {
        Self(AtomicBool::new(match mp {
            CurrentMetaPage::A => false,
            CurrentMetaPage::B => true,
        }))
    }

    fn set(&self, new: CurrentMetaPage) {
        self.0.store(
            match new {
                CurrentMetaPage::A => false,
                CurrentMetaPage::B => true,
            },
            Ordering::Relaxed,
        );
    }

    fn get(&self) -> CurrentMetaPage {
        if self.0.load(Ordering::Relaxed) {
            CurrentMetaPage::B
        } else {
            CurrentMetaPage::A
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum CurrentMetaPage {
    A = 0,
    B = 1,
}

impl CurrentMetaPage {
    fn other(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

impl From<CurrentMetaPage> for u64 {
    fn from(value: CurrentMetaPage) -> Self {
        u64::from(value as u8)
    }
}

pub struct PageAllocator<S: Storage> {
    /// ONLY MODIFY THIS IF YOU HAVE A LOCK.
    current_meta_page: AtomicCurrentMetaPage,

    /// The managed storage.
    pub mstorage: ManagedStorage<S>,

    /// Lock for concurrent access to the "allocator".
    lock: Mutex<()>,
}

impl<S: Storage> PageAllocator<S> {
    pub fn new(storage: S) -> Result<Self, Error> {
        let mstorage = ManagedStorage::new(storage);

        let first_meta_page = match mstorage.page(PageId::<MetaPage>::new(0)) {
            Ok(page) => Some(page),
            Err(mstorage::Error::PageValidationFailed { .. }) => None,
            Err(err) => return Err(err.into()),
        };

        let second_meta_page = match mstorage.page(PageId::<MetaPage>::new(1)) {
            Ok(page) => Some(page),
            Err(mstorage::Error::PageValidationFailed { .. }) => None,
            Err(err) => return Err(err.into()),
        };

        let current_meta_page = match (first_meta_page, second_meta_page) {
            (None, None) => {
                cold_path();

                return Err(Error::ValidationFailedForBothMetaPages);
            }
            (Some(_), None) => {
                cold_path();

                warn!(
                    "validation failed for meta page B. \
                    this indicates a power loss while trying to flush that page \
                    or some other form of data corruption. \
                    the db will default to meta page A"
                );

                CurrentMetaPage::A
            }
            (None, Some(_)) => {
                cold_path();

                warn!(
                    "validation failed for meta page A. \
                    this indicates a power loss while trying to flush that page \
                    or some other form of data corruption. \
                    the db will default to meta page B"
                );

                CurrentMetaPage::B
            }
            (Some(page_a), Some(page_b)) => {
                // Both pages are valid -> compare revisions.

                if page_a.revision_id.get() >= page_b.revision_id.get() {
                    CurrentMetaPage::A
                } else {
                    CurrentMetaPage::B
                }
            }
        };

        Ok(Self {
            mstorage,
            lock: Mutex::new(()),
            // TODO: check meta pages.
            current_meta_page: AtomicCurrentMetaPage::new(current_meta_page),
        })
    }

    pub fn meta_page(&self) -> Result<PageReference<'_, '_, MetaPage>, Error> {
        self.mstorage
            .page(PageId::<MetaPage>::new(self.current_meta_page.get().into()))
            .map_err(From::from)
    }

    /// # Panics
    ///
    /// Panics if the lock is
    pub fn allocate<P: Page>(&self) -> Result<PageReference<'_, '_, P>, Error> {
        // TODO: make this function not generic
        let _lock = self.lock.lock().unwrap();
        let meta_page = self.meta_page()?;

        match meta_page.allocator_next_free_page.get() {
            free_page_id if free_page_id.raw != 0 => {
                let free_page = self.mstorage.page(free_page_id)?;

                // Update next free page to be the next page in the first free page.
                meta_page
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
                let next_page_id = PageId::<P>::new(meta_page.allocator_pages_initialized.get());

                let next_page = self.mstorage.page(next_page_id)?;

                // Increment pages initialized.
                meta_page
                    .allocator_pages_initialized
                    .set(next_page_id.raw.wrapping_add(1));

                Ok(next_page)
            }
        }
    }

    pub fn free<P: Page>(&self, page_id: PageId<P>) -> Result<(), Error> {
        self.non_generic_free(page_id.raw)
    }

    fn non_generic_free(&self, id: RawPageId) -> Result<(), Error> {
        let page_to_free = self.mstorage.page(PageId::<FreePage>::new(id))?;

        let _lock = self.lock.lock().unwrap();
        let meta_page = self.meta_page()?;

        let current_next_free_page_id = meta_page.allocator_next_free_page.get();
        page_to_free.next_page.set(current_next_free_page_id);

        meta_page.allocator_next_free_page.raw.set(id);

        // TODO: maybe give pages at the end of the initialized region
        // back to the uninialized region (?)

        Ok(())
    }
}
