use crate::rows::Row;
use memmap2::MmapMut;
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};
use tokio::sync::RwLock;

/// A wrapper around
pub struct ContentsMemMap(MmapMut);

impl ContentsMemMap {
    #[inline]
    #[must_use]
    pub const unsafe fn new(mmap: MmapMut) -> Self {
        Self(mmap)
    }
}

impl Deref for ContentsMemMap {
    type Target = Contents;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.0[..] as *const [u8] as *const Self::Target) }
    }
}

impl DerefMut for ContentsMemMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut self.0[..] as *mut [u8] as *mut Self::Target) }
    }
}

#[repr(C, align(4096))]
pub struct Contents {
    /// Magic bytes, EVERYTHINGDB
    pub magic_bytes: [u8; 12],
    /// The version of the database.
    pub version: u32,
    /// The root validation ID. Incremented on open.
    pub validation_id: u64,

    pub blocks_list_meta: RwLock<BlockListMeta>,

    // /// The metadata for the _associations_ free list.
    // pub bfl_associations: RwLock<ChainedListMeta>,
    // /// The metadata for the 2048 bytes slot free list.
    // pub bfl_2048: RwLock<ChainedListMeta>,
    /// The blocks.
    pub blocks: [Block],
}

pub struct BlockListMeta {
    pub free_block_count: u64,
    pub used_block_count: u64,
    pub free_list_head: NonZeroU64,
}

pub struct ChainedListMeta {
    /// The index of the first item in the free list.
    pub head: u64,
    /// The index of the last item in the free list. If `Some`, it contains data.
    pub free_list_head: u64,
    /// The number of items in the free list.
    pub len: u64,
}

/// An (Everything) _Block_, contiguous block of 2^14 = 16,384 bytes (spanning four OS pages
/// á 4096 bytes), aligned at OS pages.
#[repr(C, align(4096), u32)]
pub enum Block {
    Associations {
        validation_id: u64,
        next_block_index: Option<NonZeroU64>,
        _unused_0: u64,
        rows: [Row; 511],
    },
    VariableBinary {
        next_block_index: Option<NonZeroU64>,
        remaining_len: u64,
        _unused_0: u64,
        content: [u8; 511 * 32],
    },
    /// Indicates that this block is used for variable text (UTF-8).
    VariableText {
        next_block_index: Option<NonZeroU64>,
        remaining_len: u64,
        _unused_0: u64,
        content: [u8; 511 * 32],
    },
    BinaryStorage64 {
        /// The number of used slots in this block. When zero, unchain this block from the freelist
        used_slot_count: NonZeroU64,
        slots: [(u8, [u8; 63]); 127],
    },
    /// Indicates that this block is free. `Some(block)` if there is a next block,
    /// `None` if this is the end of the free list.
    FreeList(Option<NonZeroU64>),
}
