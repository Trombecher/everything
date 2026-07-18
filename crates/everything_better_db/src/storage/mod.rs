mod inmemory;

pub use inmemory::*;

use std::io;

use crate::{
    error::Error,
    pages::{MetaPage, Page, PageId},
};

pub trait Storage {
    fn meta_page(&self) -> &MetaPage;

    fn page(&self, index: usize) -> Option<&Page>;

    fn flush(&self) -> Result<(), io::Error>;

    fn resolve_page<P>(&self, id: PageId<P>) -> Result<&P, Error>
    where
        Page: AsRef<P>,
    {
        self.page(id.id.get().try_into().unwrap())
            .map(AsRef::as_ref)
            .ok_or_else(|| Error::PageIdDoesNotExist(id.id))
    }
}
