#![feature(if_let_guard)]

pub mod content;
mod ff;
pub mod objects;
pub mod values;

use std::num::NonZeroU64;
use crate::content::{Block, ContentsMemMap};
use memmap2::MmapMut;
use std::path::Path;
use tokio::fs::{File, OpenOptions};

pub struct Database {
    path: Box<Path>,
    file: File,
    data: ContentsMemMap,

    /// The validation id, cached because it is frequently accessed and not updated.
    cached_validation_id: u64,
}

impl Database {
    pub async fn new(path: Box<Path>) -> Result<Self, ()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .await
            .map_err(|_| ())?;

        file.set_len(4096).await.map_err(|_| ())?;

        let mut data = unsafe { ContentsMemMap::new(MmapMut::map_mut(&file).map_err(|_| ())?) };
        data.validation_id += 1;

        Ok(Self {
            cached_validation_id: data.validation_id,
            path,
            data,
            file,
        })
    }

    #[inline]
    pub fn version(&self) -> u32 {
        self.data.version
    }

    #[inline]
    pub async fn blocks_used(&self) -> u64 {
        self.data.blocks_list_meta.read().await.used_block_count
    }

    #[inline]
    pub async fn blocks_reserved(&self) -> u64 {
        let meta = self.data.blocks_list_meta.read().await;
        meta.free_block_count + meta.used_block_count
    }

    #[inline]
    pub async fn blocks_free(&self) -> u64 {
        self.data.blocks_list_meta.read().await.free_block_count
    }

    async fn allocate_block(&self) -> NonZeroU64 {
        let mut free_list = self.data.blocks_list_meta.write().await;

        free_list.free_block_count = free_list.free_block_count.saturating_sub(1);
        free_list.used_block_count = free_list.used_block_count.saturating_add(1);
        
        let block_id = free_list.free_list_head;
        
        let block = &self.data.blocks[block_id.get() as usize];
        
        let next_free_block = match block {
            Block::FreeList(Some(next)) => *next,
            Block::FreeList(None) => panic!("Free list is empty."),
            _ => panic!("Head of free list is not a free list block (block_id = {}).", block_id.get()),
        };
        
        block_id
    }
}
