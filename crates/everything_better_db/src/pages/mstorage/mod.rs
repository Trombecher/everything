#[cfg(test)]
mod tests;

use std::ops::Deref;

use crate::pages::{
    Page, PageId, RawPageId,
    pam::{self, PageAccessGuard, PageAccessManager},
    storage::Storage,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    Pam(#[from] pam::Error),
    #[error("page id {page_id} out of bounds")]
    PageIdOutOfBounds { page_id: RawPageId },
}

/// A live and active reference to a page.
pub struct PageReference<'pam, 'page, P: Page> {
    guard: PageAccessGuard<'pam>,
    page: &'page P,
}

impl<'page, P: Page> Deref for PageReference<'_, 'page, P> {
    type Target = P;

    fn deref(&self) -> &'page Self::Target {
        self.page
    }
}

/// Manages a storage by guarding page access.
pub struct ManagedStorage<S: Storage> {
    storage: S,
    pam: PageAccessManager,
}

impl<S: Storage> ManagedStorage<S> {
    pub const fn new(storage: S) -> Self {
        Self {
            storage,
            pam: PageAccessManager::new(),
        }
    }

    pub fn page<'storage, P: Page>(
        &'storage self,
        page_id: PageId<P>,
    ) -> Result<PageReference<'storage, 'storage, P>, Error> {
        let Some(page_reference) = self.storage.page(page_id.raw) else {
            return Err(Error::PageIdOutOfBounds {
                page_id: page_id.raw,
            });
        };

        let guard = self.pam.open_page_as(page_id.raw, P::KIND)?;

        Ok(PageReference {
            guard,
            // SAFETY: because we successfully acquired
            // a page guard for that page AND `P` implements `Page`,
            // this cast is valid
            page: unsafe { page_reference.cast() },
        })
    }
}
