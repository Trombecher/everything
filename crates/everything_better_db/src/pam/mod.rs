#[cfg(test)]
mod tests;

use std::{fmt::Debug, hint::cold_path, sync::Mutex};

use crate::pages::PageKind;

#[derive(Debug, PartialEq)]
pub enum PamError {
    InvalidPageAccess {
        page_index: usize,
        page_in_use_as: PageKind,
        requested: PageKind,
    },
    PageUseCountExhausted {
        page_index: usize,
    },
}

#[derive(PartialEq, Debug)]
struct OpenPage {
    page_index: usize,
    info: OpenPageInfo,
}

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
    pub page_index: usize,
}

impl Debug for PageAccessGuard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageAccessGuard")
            .field("page_index", &self.page_index)
            .finish()
    }
}

impl Drop for PageAccessGuard<'_> {
    fn drop(&mut self) {
        self.pam.close_page(self.page_index);
    }
}

/// A very inefficient implementation of a page access manager.
pub struct PageAccessManager {
    open_pages: Mutex<Vec<OpenPage>>,
}

impl PageAccessManager {
    pub const fn empty() -> Self {
        Self {
            open_pages: Mutex::new(Vec::new()),
        }
    }

    pub fn page_info(&self, page_index: usize) -> Option<OpenPageInfo> {
        self.open_pages
            .lock()
            .unwrap()
            .iter()
            .find(|open_page| open_page.page_index == page_index)
            .map(|open_page| open_page.info.clone())
    }

    pub fn open_page_as<'pam>(
        &'pam self,
        page_index: usize,
        requested_use: PageKind,
    ) -> Result<PageAccessGuard<'pam>, PamError> {
        let mut open_pages = self.open_pages.lock().unwrap();

        let maybe_already_open_page = open_pages
            .iter_mut()
            .find(|open_page| open_page.page_index == page_index);

        match maybe_already_open_page {
            None => {
                // Page is not open.

                open_pages.push(OpenPage {
                    page_index,
                    info: OpenPageInfo {
                        used_as: requested_use,
                        uses_minus_one: 0,
                    },
                });

                Ok(PageAccessGuard {
                    pam: self,
                    page_index,
                })
            }
            Some(already_open_page) => {
                if already_open_page.info.used_as == requested_use {
                    if let Err(()) = already_open_page.info.increment_use_count() {
                        return Err(PamError::PageUseCountExhausted { page_index });
                    }

                    Ok(PageAccessGuard {
                        pam: self,
                        page_index,
                    })
                } else {
                    // Requested page use kind does not match with already open page.
                    // This either indicates a bug in the database or a malformed file format.

                    Err(PamError::InvalidPageAccess {
                        page_index,
                        page_in_use_as: already_open_page.info.used_as,
                        requested: requested_use,
                    })
                }
            }
        }
    }

    /// This method must only be called by a valid page guard.
    fn close_page(&self, page_index: usize) {
        let mut open_pages = self.open_pages.lock().unwrap();

        let (index_of_open_page, open_page) = open_pages
            .iter_mut()
            .enumerate()
            .find(|(_, page)| page.page_index == page_index)
            .expect("no open page found for `page_index` from a `PageAccessGuard`; this indicates a critical bug in the code");

        if let Err(()) = open_page.info.decrement_use_count() {
            // Page is unused after this guard has been dropped.
            // We can use `swap_remove` to remove the entry because our
            // `Vec` is not sorted.

            open_pages.swap_remove(index_of_open_page);
        }
    }
}
