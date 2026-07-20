mod allocator;
mod meta;
mod mstorage;
mod pam;
pub mod storage;

pub use allocator::*;
pub use meta::*;

use derive_where::derive_where;

use core::marker::PhantomData;
use std::mem::transmute;

use crate::pages::storage::sync::{MutableU32LeLocation, MutableU64LeLocation};

/// # SAFETY
///
/// For some type to be a page, it must satisfy the following constraints:
///
/// * it MUST have an exact size (no padding) of 4096,
/// * it MUST have an alignment of 4096,
/// * it MUST be castable from raw bytes (accept any bit pattern), and
/// * it MUST have an atomic u32 at the start (CRC)
///   and another mutable atomic u32 followed right after that (page kind).
pub unsafe trait Page {
    const KIND: PageKind;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PageKind {
    BTreeRoot = u32::from_le_bytes(*b"BTRR"),
    BTreeChild = u32::from_le_bytes(*b"BTRC"),
    Free = u32::from_le_bytes(*b"FREE"),
    Meta = u32::from_le_bytes(*b"EVER"),
}

impl PageKind {
    pub const VALUES: [Self; 4] = [Self::BTreeChild, Self::BTreeRoot, Self::Free, Self::Meta];
}

pub struct MutablePageKindLocation(MutableU32LeLocation);

impl MutablePageKindLocation {
    pub fn get(&self) -> Result<PageKind, ()> {
        let got = self.0.get();

        // TODO: maybe SIMD

        [
            PageKind::BTreeRoot as u32,
            PageKind::BTreeChild as u32,
            PageKind::Free as u32,
            PageKind::Meta as u32,
        ]
        .contains(&got)
        .then(|| unsafe { transmute(got) })
        .ok_or(())
    }

    pub fn set(&self, page_kind: PageKind) {
        self.0.set(page_kind as u32);
    }
}

#[repr(C, align(4096))]
pub struct FreePage {
    pub crc32c: MutableU32LeLocation,
    pub page_kind: MutablePageKindLocation,

    /// A pointer to the next page in the free list.
    pub next_page: MutablePageIdLocation<FreePage>,

    // TODO: maybe duplicate or save free page otherwise...
    _rest: [u8; 4080],
}

#[macro_export]
macro_rules! unsafe_declare_page {
    ($Page:ty, $kind:expr) => {
        $crate::const_assert!(size_of::<$Page>() == 4096);
        $crate::const_assert!(align_of::<$Page>() == 4096);

        unsafe impl $crate::pages::Page for $Page {
            const KIND: PageKind = $kind;
        }

        // const _: () = is_from_bytes::<$Page>();
    };
}

unsafe_declare_page!(FreePage, PageKind::Free);

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
