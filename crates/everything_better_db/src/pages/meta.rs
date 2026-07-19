use std::sync::atomic::{AtomicU8, Ordering};

use zerocopy::FromBytes;

use crate::{
    pages::{FreePage, PageIdLocation},
    sync::{U32LeLocation, U64LeLocation},
};

#[repr(C, align(4096))]
pub struct MetaPage {
    pub magic_bytes: MagicBytes,
    pub current_super_block: CurrentSuperBlockLocation,
    pub super_block_a: SuperBlock,
    pub super_block_b: SuperBlock,
}

#[derive(FromBytes)]
pub struct CurrentSuperBlockLocation(AtomicU8);

impl CurrentSuperBlockLocation {
    pub fn get(&self) -> CurrentSuperBlock {
        if self.0.load(Ordering::Relaxed) & 0b1 == 0 {
            CurrentSuperBlock::A
        } else {
            CurrentSuperBlock::B
        }
    }

    pub fn set(&self, sb: CurrentSuperBlock) {
        self.0.store(sb as u8, Ordering::Relaxed);
    }
}

pub enum CurrentSuperBlock {
    A,
    B,
}

pub struct SuperBlock {
    pub version: U32LeLocation,
    pub _reserved: U32LeLocation,
    pub allocator_next_free_page: PageIdLocation<FreePage>,
    pub allocator_pages_initialized: U64LeLocation,
}

#[derive(FromBytes)]
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
