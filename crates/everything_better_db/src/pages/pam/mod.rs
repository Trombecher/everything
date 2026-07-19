#[cfg(test)]
mod tests;

use std::{fmt::Debug, hint::cold_path, sync::Mutex};

use crate::pages::{PageKind, RawPageId};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error(
        "could not open page {page_id}: requested {requested:?} but page is already used as {page_in_use_as:?}"
    )]
    InvalidPageAccess {
        page_id: RawPageId,
        page_in_use_as: PageKind,
        requested: PageKind,
    },
    #[error("could not open page {page_id}: its use count is exhausted")]
    PageUseCountExhausted { page_id: RawPageId },
}

#[derive(PartialEq, Debug)]
struct OpenPage {
    page_id: RawPageId,
    info: OpenPageInfo,
}

/// Info about an open page.
#[derive(PartialEq, Debug, Clone)]
pub struct OpenPageInfo {
    pub used_as: PageKind,
    pub uses_minus_one: u32,
}

impl OpenPageInfo {
    #[inline(always)]
    pub fn real_use_count(&mut self) -> u64 {
        self.uses_minus_one as u64 + 1
    }

    #[inline(always)]
    pub fn increment_use_count(&mut self) -> Result<(), ()> {
        match self.uses_minus_one.checked_add(1) {
            Some(new_ref_count) => {
                self.uses_minus_one = new_ref_count;
                Ok(())
            }
            None => {
                // Will probably not happen.
                cold_path();

                return Err(());
            }
        }
    }

    #[inline(always)]
    pub fn decrement_use_count(&mut self) -> Result<(), ()> {
        if let Some(new_page_use_count) = self.uses_minus_one.checked_sub(1) {
            self.uses_minus_one = new_page_use_count;
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct PageAccessGuard<'a> {
    pam: &'a PageAccessManager,
    page_id: RawPageId,
}

impl<'pam> PageAccessGuard<'pam> {
    pub const fn pam(&self) -> &PageAccessManager {
        &self.pam
    }

    pub const fn page_id(&self) -> RawPageId {
        self.page_id
    }
}

impl Debug for PageAccessGuard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageAccessGuard")
            .field("page_index", &self.page_id)
            .finish()
    }
}

impl Drop for PageAccessGuard<'_> {
    fn drop(&mut self) {
        self.pam.close_page(self.page_id);
    }
}

/// A very inefficient implementation of a page access manager.
pub struct PageAccessManager {
    open_pages: Mutex<Vec<OpenPage>>,
}

impl PageAccessManager {
    pub const fn new() -> Self {
        Self {
            open_pages: Mutex::new(Vec::new()),
        }
    }

    pub fn page_info(&self, page_id: RawPageId) -> Option<OpenPageInfo> {
        self.open_pages
            .lock()
            .unwrap()
            .iter()
            .find(|open_page| open_page.page_id == page_id)
            .map(|open_page| open_page.info.clone())
    }

    pub fn open_page_as<'pam>(
        &'pam self,
        page_id: RawPageId,
        requested_use: PageKind,
    ) -> Result<PageAccessGuard<'pam>, Error> {
        let mut open_pages = self.open_pages.lock().unwrap();

        let maybe_already_open_page = open_pages
            .iter_mut()
            .find(|open_page| open_page.page_id == page_id);

        match maybe_already_open_page {
            None => {
                // Page is not open.

                open_pages.push(OpenPage {
                    page_id,
                    info: OpenPageInfo {
                        used_as: requested_use,
                        uses_minus_one: 0,
                    },
                });

                Ok(PageAccessGuard { pam: self, page_id })
            }
            Some(already_open_page) => {
                if already_open_page.info.used_as == requested_use {
                    if let Err(()) = already_open_page.info.increment_use_count() {
                        return Err(Error::PageUseCountExhausted { page_id });
                    }

                    Ok(PageAccessGuard { pam: self, page_id })
                } else {
                    // Requested page use kind does not match with already open page.
                    // This either indicates a bug in the database or a malformed file format.

                    Err(Error::InvalidPageAccess {
                        page_id,
                        page_in_use_as: already_open_page.info.used_as,
                        requested: requested_use,
                    })
                }
            }
        }
    }

    /// This method must only be called by a valid page guard.
    fn close_page(&self, page_id: RawPageId) {
        let mut open_pages = self.open_pages.lock().unwrap();

        let (index_of_open_page, open_page) = open_pages
            .iter_mut()
            .enumerate()
            .find(|(_, page)| page.page_id == page_id)
            .expect("no open page found for `page_index` from a `PageAccessGuard`; this indicates a critical bug in the code");

        if let Err(()) = open_page.info.decrement_use_count() {
            // Page is unused after this guard has been dropped.
            // We can use `swap_remove` to remove the entry because our
            // `Vec` is not sorted.

            open_pages.swap_remove(index_of_open_page);
        }
    }
}
