mod inmemory;
pub mod sync;

pub use inmemory::*;

use core::{marker::PhantomData, ptr::NonNull};

use crate::pages::{MutablePageKindLocation, Page, RawPageId, storage::sync::MutableU32LeLocation};

pub trait Storage {
    type Error;

    /// Creates a reference to an opaque page.
    fn page(&self, page_id: RawPageId) -> Option<OpaquePageReference<'_>>;

    /// Flushes dirty pages back to the storage medium.
    fn flush(&self) -> Result<(), Self::Error>;
}

/// A reference to an opaque page.
pub struct OpaquePageReference<'page> {
    pointer: NonNull<OpaquePage>,
    _marker: PhantomData<&'page OpaquePage>,
}

impl OpaquePageReference<'_> {
    /// # SAFETY
    ///
    /// The pointer must be correctly aligned and
    /// valid for the page.
    #[must_use]
    pub const unsafe fn new(pointer: NonNull<OpaquePage>) -> Self {
        Self {
            pointer,
            _marker: PhantomData,
        }
    }

    /// # SAFETY
    ///
    /// You need a page guard for this kind.
    #[must_use]
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
    pub crc32c: MutableU32LeLocation,
    pub kind: MutablePageKindLocation,
    pub bytes: [u8; 4088],
}

impl OpaquePage {
    pub const SIZE: usize = size_of::<Self>();
}
