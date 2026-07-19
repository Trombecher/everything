mod allocator;
mod meta;
mod mstorage;
mod pam;
pub mod storage;

pub use allocator::*;
pub use meta::*;

use derive_where::derive_where;
use zerocopy::FromBytes;

use core::marker::PhantomData;

use crate::pages::storage::sync::MutableU64LeLocation;

pub unsafe trait Page {
    const KIND: PageKind;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PageKind {
    BTreeRoot,
    BTreeChild,
    Free,
    Meta,
}

#[repr(C, align(4096))]
pub struct FreePage {
    pub next_page: MutablePageIdLocation<FreePage>,
    // TODO: maybe duplicate or save free page otherwise...
    _rest: [u8; 4088],
}

#[macro_export]
macro_rules! unsafe_declare_page {
    ($Page:ty, $kind:expr) => {
        crate::const_assert!(size_of::<$Page>() == 4096);
        crate::const_assert!(align_of::<$Page>() == 4096);

        unsafe impl $crate::pages::Page for $Page {
            const KIND: PageKind = $kind;
        }

        // const _: () = is_from_bytes::<$Page>();
    };
}

unsafe_declare_page!(FreePage, PageKind::Free);

#[doc(hidden)]
const fn is_from_bytes<T: FromBytes>() {}

/// A little-endian u64 location that
pub struct MutablePageIdLocation<P: Page> {
    raw: MutableU64LeLocation,
    _marker: PhantomData<P>,
}

impl<P: Page> MutablePageIdLocation<P> {
    /// Returns the page id; or `None` if it is zero.
    #[inline]
    #[must_use]
    pub fn get(&self) -> PageId<P> {
        PageId::new(self.raw.get())
    }

    #[inline(always)]
    pub fn set(&self, value: PageId<P>) {
        self.raw.set(value.raw);
    }
}

pub type RawPageId = u64;

/// A raw page id that has additional information on what
/// page kind it should points to.
#[derive_where(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PageId<P: Page> {
    pub raw: RawPageId,
    pub _marker: PhantomData<P>,
}

impl<P: Page> PageId<P> {
    pub const fn new(raw: RawPageId) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }
}
