mod allocator;
mod kinds;
mod mmapped;

pub use allocator::*;
pub use kinds::*;
pub use mmapped::*;

use crate::Error;

pub const PAGE_SIZE: u16 = 4096;

pub trait PageProvider {
    /// Returns a reference to the nth page.
    async fn page<'backend>(&'backend self, page_index: u64) -> Result<&'backend Page, Error>;

    /// Syncs content back to disk.
    async fn flush_page(&self, page_index: u64) -> Result<(), Error>;
}
