use super::PAGE_SIZE;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicU64};

macro_rules! unsafe_page_conversion {
    ($Page:ty) => {
        impl $Page {
            pub const fn from_page(page: &Page) -> &Self {
                unsafe { &*(page as *const Page as *const Self) }
            }
        }
    };
}

#[repr(C, align(4096))]
pub struct Page(pub [u8; PAGE_SIZE as usize]);

#[repr(C, align(4096))]
pub struct MetaPage {
    /// "EVERYTHINGDB" in ascii.
    pub magic_bytes: [AtomicU8; 12],
    /// Version
    pub version: AtomicU32,

    pub free_list_locked: AtomicBool,
    pub free_list_pop: AtomicU64,
    pub free_list_append: AtomicU64,
}

unsafe_page_conversion!(MetaPage);

#[repr(C, align(4096))]
pub struct FreePage {
    pub next_free_page: AtomicU64,
}

unsafe_page_conversion!(FreePage);
