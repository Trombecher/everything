// mod allocator;
mod meta;
mod mstorage;
mod pam;
mod storage;

// pub use allocator::*;
pub use meta::*;

use derive_where::derive_where;
use zerocopy::FromBytes;

use core::{marker::PhantomData, ptr::NonNull};

use crate::sync::U64LeLocation;

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
    pub next_page: PageIdLocation<FreePage>,
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
pub struct PageIdLocation<P: Page> {
    pub raw: U64LeLocation,
    pub _marker: PhantomData<P>,
}

impl<P: Page> PageIdLocation<P> {
    /// Returns the page id; or `None` if it is zero.
    #[inline]
    #[must_use]
    pub fn id(&self) -> PageId<P> {
        PageId::new(self.raw.get())
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

/// A reference to an opaque page.
pub struct OpaquePageReference<'page> {
    pointer: NonNull<OpaquePage>,
    _marker: PhantomData<&'page OpaquePage>,
}

impl<'page> OpaquePageReference<'page> {
    /// # SAFETY
    ///
    /// The pointer must be correctly aligned and
    /// valid for the page.
    pub const unsafe fn new(pointer: NonNull<OpaquePage>) -> Self {
        Self {
            pointer,
            _marker: PhantomData,
        }
    }

    pub const unsafe fn cast<'a, P: Page>(self) -> &'a P {
        unsafe { self.pointer.as_ptr().cast::<P>().as_ref_unchecked() }
    }
}

/// A struct representing a page that has not been yet interpreted.
///
/// **You should never have an actual reference to a struct of this type,
/// but via [`OpaquePageReference`].**
#[repr(C, align(4096))]
pub struct OpaquePage {
    pub bytes: [u8; 4096],
}

impl OpaquePage {
    pub const SIZE: usize = size_of::<Self>();
}
