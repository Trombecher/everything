mod allocator;
mod meta;

pub use allocator::*;
use derive_where::derive_where;
pub use meta::*;

use std::{marker::PhantomData, mem::transmute, num::NonZeroU64, sync::atomic::AtomicU8};

use crate::sync::U64LeLocation;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PageKind {
    BTreeRoot,
    BTreeChild,
    Free,
}

#[macro_export]
macro_rules! unsafe_declare_page {
    ($DeclarePage:ty) => {
        crate::const_assert!(size_of::<$DeclarePage>() == crate::pages::Page::SIZE);

        impl AsRef<$DeclarePage> for crate::pages::Page {
            fn as_ref(&self) -> &$DeclarePage {
                unsafe { &*(self as *const _ as *const $DeclarePage) }
            }
        }

        impl AsRef<crate::pages::Page> for $DeclarePage {
            fn as_ref(&self) -> &crate::pages::Page {
                unsafe { &*(self as *const _ as *const crate::pages::Page) }
            }
        }
    };
}

pub struct PageIdLocation<P>
where
    Page: AsRef<P>,
{
    pub raw: U64LeLocation,
    pub _marker: PhantomData<P>,
}

impl<P> PageIdLocation<P>
where
    Page: AsRef<P>,
{
    /// Returns the page id; or `None` if it is zero.
    #[inline]
    #[must_use]
    pub fn id(&self) -> Option<PageId<P>> {
        NonZeroU64::new(self.raw.get()).map(PageId::new)
    }
}

#[derive_where(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PageId<P>
where
    Page: AsRef<P>,
{
    pub id: NonZeroU64,
    pub _marker: PhantomData<P>,
}

impl<P> PageId<P>
where
    Page: AsRef<P>,
{
    pub const fn new(id: NonZeroU64) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

#[repr(C, align(4096))]
pub struct Page {
    pub bytes: [AtomicU8; 4096],
}

impl Page {
    pub const SIZE: usize = size_of::<Self>();

    /// SAFETY:
    ///
    /// slice must be aligned to [`Self::SIZE`] bytes.
    #[inline]
    pub const unsafe fn from_ref(bytes: &[AtomicU8; Self::SIZE]) -> &Self {
        unsafe { transmute(bytes) }
    }
}

impl AsRef<Self> for Page {
    fn as_ref(&self) -> &Self {
        self
    }
}
