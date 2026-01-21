use std::sync::atomic::AtomicU64;

use crate::{Error, page::Page, versions::Version};

#[repr(C, align(4096))]
pub struct MetaPage {
    /// Magic bytes "EVERYTHINGDB".
    pub magic_bytes: [u8; 12],
    /// The db version
    pub version: Version,
    /// The first page of the Write Ahead Log.
    ///
    /// This field may only be
    pub wal_first_page: AtomicU64,
}

impl MetaPage {
    pub const MAGIC_BYTES: [u8; 12] = *b"EVERYTHINGDB";

    pub(crate) fn initialize(page: &mut Page) {
        // Set magic bytes
        page.0[0..12].copy_from_slice(&Self::MAGIC_BYTES);

        // Set version to latest
        page.0[12..16].copy_from_slice(&(Version::LATEST as u32).to_le_bytes());
    }

    /// Validates a page to be used as a meta page.
    #[must_use]
    pub(crate) fn validate(page: &Page) -> Result<(), Error> {
        if page.0[0..12] != Self::MAGIC_BYTES {
            return Err(Error::MagicBytesMismatch);
        }

        let found_version = u32::from_le_bytes([page.0[12], page.0[13], page.0[14], page.0[15]]);

        if Version::new(found_version).is_none() {
            return Err(Error::InvalidVersion);
        }

        Ok(())
    }
}
