mod file;
mod inmemory;
mod pages;
pub mod sync;

use crc32c::crc32c;
pub use inmemory::*;

use core::{marker::PhantomData, ptr::NonNull};

use crate::{
    const_assert,
    pages::{PAGE_SIZE_IN_BYTES, Page, RawPageId, storage::sync::MutableU32LeLocation},
};

pub trait Storage {
    type Error;

    /// Creates a reference to an opaque page.
    fn page(
        &self,
        page_id: RawPageId,
        skip_validation: bool,
    ) -> Option<PseudoReference<'_, PageWithCRC>>;

    /// Flushes dirty pages back to the storage medium.
    fn flush(&self) -> Result<(), Self::Error>;
}

/// A non null pointer to something whose lifetime
/// is bound.
pub struct PseudoReference<'page, T> {
    pointer: NonNull<T>,
    _marker: PhantomData<&'page T>,
}

impl<'page, T> PseudoReference<'page, T> {
    #[must_use]
    pub const fn new(pointer: NonNull<T>) -> Self {
        Self {
            pointer,
            _marker: PhantomData,
        }
    }

    pub const unsafe fn cast<U>(self) -> PseudoReference<'page, U> {
        PseudoReference {
            pointer: self.pointer.cast(),
            _marker: PhantomData,
        }
    }

    /// # SAFETY
    ///
    /// You need a page guard for this kind and the pointer must be valid
    /// for the given lifetime.
    #[must_use]
    pub const unsafe fn cast_as_page_ref<P: Page>(self) -> &'page P {
        unsafe { self.pointer.as_ptr().cast::<P>().as_ref_unchecked() }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("invalid page CRC")]
pub struct InvalidPageCRC;

/// A page with a crc header field.
///
/// **You should never have an actual reference to a struct of this type,
/// but via [`OpaquePageReference`].**
#[repr(C, align(4096))]
pub struct PageWithCRC {
    pub crc32c: MutableU32LeLocation,
    pub content: [u8; PAGE_SIZE_IN_BYTES - 4],
}

const_assert!(size_of::<PageWithCRC>() == PAGE_SIZE_IN_BYTES);
const_assert!(align_of::<PageWithCRC>() == PAGE_SIZE_IN_BYTES);

impl PageWithCRC {
    fn compute_crc(&self) -> u32 {
        crc32c(&self.content)
    }

    pub fn validate(&self) -> Result<(), InvalidPageCRC> {
        (self.crc32c.get() == self.compute_crc()).ok_or(InvalidPageCRC)
    }
}
