mod inmemory;

pub use inmemory::*;

use std::io;

use crate::pages::{OpaquePageReference, RawPageId};

pub trait Storage {
    /// Creates a reference to an opaque page.
    fn page<'page>(&'page self, page_id: RawPageId) -> Option<OpaquePageReference<'page>>;

    /// Flushes dirty pages back to the storage medium.
    fn flush(&self) -> Result<(), io::Error>;
}
