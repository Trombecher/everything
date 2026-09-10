use core::convert::Infallible;
use std::io;

use crate::pages::{
    RawPageId,
    storage::{PageWithCRC, PseudoReference, Storage, pages::Pages},
};

pub struct InMemoryStorage {
    pages: Pages,
}

impl InMemoryStorage {
    pub fn new(max_pages: u64) -> Result<Self, io::Error> {
        Pages::new(max_pages).map(|pages| Self { pages })
    }
}

impl Storage for InMemoryStorage {
    type Error = Infallible;

    fn flush(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn page(
        &self,
        page_id: RawPageId,
        _skip_validation: bool,
    ) -> Option<PseudoReference<'_, PageWithCRC>> {
        self.pages
            .page(page_id)
            .map(|page| unsafe { page.cast::<PageWithCRC>() })
    }
}
