use std::sync::atomic::{AtomicU8, Ordering};

use crate::{
    pages::{
        FreePage, MutablePageIdLocation, PageKind,
        storage::sync::{ImmutableU32LeLocation, MutableU32LeLocation, MutableU64LeLocation},
    },
    unsafe_declare_page,
};

#[repr(C, align(4096))]
pub struct MetaPage {
    pub magic_bytes: MagicBytes,
    pub current_super_block: CurrentSuperBlockLocation,
    pub super_block_a: SuperBlock,
    pub super_block_b: SuperBlock,
}

unsafe_declare_page!(MetaPage, PageKind::Meta);

impl MetaPage {
    pub fn super_block(&self) -> &SuperBlock {
        match self.current_super_block.get() {
            CurrentSuperBlock::A => &self.super_block_a,
            CurrentSuperBlock::B => &self.super_block_b,
        }
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentSuperBlock {
    A,
    B,
}

pub struct SuperBlock {
    pub version: MutableU32LeLocation,
    pub _reserved: ImmutableU32LeLocation,
    pub allocator_next_free_page: MutablePageIdLocation<FreePage>,
    pub allocator_pages_initialized: MutableU64LeLocation,
}

pub struct MagicBytes {
    low: MutableU64LeLocation,
    high: MutableU64LeLocation,
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
