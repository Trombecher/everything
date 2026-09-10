use std::{io, ptr::NonNull};

use memmap2::MmapMut;

use crate::{
    convert::safe_u64_to_usize,
    pages::{PAGE_SIZE_IN_BYTES, storage::PseudoReference},
};

/// A page full of bytes.
#[repr(C, align(4096))]
pub struct OpaquePage {
    pub bytes: [u8; 4096],
}

/// A set of pages.
pub struct Pages {
    /// INVARIANT: the length of this map is
    /// a multiple of [`PAGE_SIZE`] AND the pointer
    /// is aligned to [`PAGE_SIZE`].
    map: MmapMut,
}

impl Pages {
    pub fn new(page_count: u64) -> Result<Self, io::Error> {
        let map =
            MmapMut::map_anon(safe_u64_to_usize(page_count).saturating_mul(PAGE_SIZE_IN_BYTES))?;

        if !map.as_ptr().is_aligned_to(PAGE_SIZE_IN_BYTES) {
            return Err(io::Error::other(
                "got a memory map slice that is not aligned to OS page size",
            ));
        }

        Ok(Self { map })
    }

    pub fn pages(&self) -> NonNull<[OpaquePage]> {
        // Maybe this is invalid...
        let ptr = self.map.as_ptr();

        let len = self.map.len();

        NonNull::from_raw_parts(
            unsafe {
                // SAFETY: the map is aligned.

                NonNull::new_unchecked(ptr.cast_mut().cast::<OpaquePage>())
            },
            len / 4096,
        )
    }

    pub fn page(&self, page_index: u64) -> Option<PseudoReference<OpaquePage>> {
        let pages = self.pages();

        if page_index < pages.len() as u64 {
            Some(unsafe {
                PseudoReference::new(
                    pages
                        .cast::<OpaquePage>()
                        .add(safe_u64_to_usize(page_index)),
                )
            })
        } else {
            None
        }
    }
}
