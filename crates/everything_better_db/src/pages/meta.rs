use crate::{
    pages::{FreePage, PageIdLocation},
    sync::{U32LeLocation, U64LeLocation},
    unsafe_declare_page,
};

#[repr(C, align(4096))]
pub struct MetaPage {
    pub magic_bytes: MagicBytes,
    pub version: U32LeLocation,
    pub _reserved: U32LeLocation,
    pub allocator_next_free_page: PageIdLocation<FreePage>,
    pub allocator_pages_initialized: U64LeLocation,
}

unsafe_declare_page!(MetaPage);

pub struct MagicBytes {
    low: U64LeLocation,
    high: U64LeLocation,
}

impl MagicBytes {
    const EXPECTED_LOW: [u8; 8] = *b"EVERYTHI";
    const EXPECTED_HIGH: [u8; 8] = *b"NGDB    ";

    pub fn validate(&self) -> Result<(), ()> {
        let low = self.low.get().to_le_bytes();
        let high = self.high.get().to_le_bytes();

        (low == Self::EXPECTED_LOW && high == Self::EXPECTED_HIGH).ok_or(())
    }
}
