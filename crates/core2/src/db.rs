use std::path::Path;

use crate::{Error, page::MetaPage, storage::Pages, versions::Version};

pub struct Database {
    pages: Pages,
}

impl Database {
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let mut pages = Pages::from_file(path)?;

        if pages.pages().len() == 0 {
            // New db

            // Allocate meta page.
            pages.increase_size_to(1)?;

            MetaPage::initialize(pages.pages_mut().get_mut(0).unwrap());
        } else {
            // Existing db

            MetaPage::validate(pages.pages().get(0).unwrap())?;
        }

        Ok(Self { pages })
    }

    #[inline(always)]
    #[must_use]
    pub fn version(&self) -> Version {
        self.meta_page().version
    }

    #[inline(always)]
    #[must_use]
    fn meta_page(&self) -> &MetaPage {
        // SAFETY: meta page was validated on creation
        unsafe { self.pages.pages().get(0).unwrap().unsafe_as_meta() }
    }

    #[inline(always)]
    #[must_use]
    fn meta_page_mut(&mut self) -> &mut MetaPage {
        // SAFETY: meta page was validated on creation
        unsafe {
            self.pages
                .pages_mut()
                .get_mut(0)
                .unwrap()
                .unsafe_as_meta_mut()
        }
    }
}
