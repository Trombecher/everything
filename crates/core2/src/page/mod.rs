mod meta;

pub use meta::*;

pub const PAGE_SIZE: usize = 4096;

#[repr(align(4096))]
pub struct Page(pub [u8; PAGE_SIZE]);

impl Page {
    #[inline(always)]
    #[must_use]
    pub const unsafe fn unsafe_as_meta(&self) -> &MetaPage {
        unsafe { &*(self as *const Page).cast::<MetaPage>() }
    }

    #[must_use]
    #[inline(always)]
    pub const unsafe fn unsafe_as_meta_mut(&mut self) -> &mut MetaPage {
        unsafe { &mut *(self as *mut Page).cast::<MetaPage>() }
    }
}
