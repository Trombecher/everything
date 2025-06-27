use crate::{
    content::{Block, ContentsMemMap}, fp::InvalidFormatPolicy, objects::{core::ROWS, ObjectId}, query::Query, values::Value, Error
};
use memmap2::MmapMut;
use tracing::{debug, info};
use std::num::NonZeroU64;
use std::path::Path;
use tokio::{
    fs::{File, OpenOptions},
    sync::RwLock,
};

pub struct Database {
    path: Box<Path>,
    file: File,
    /// This rw lock exists for flushing the file.
    data: RwLock<ContentsMemMap>,

    /// The validation id, cached because it is frequently accessed and not updated.
    cached_validation_id: u64,
}

impl Database {
    /// Creates a new database or opens the file.
    pub async fn new(path: Box<Path>) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .await
            .map_err(Into::<Error>::into)?;

        let is_new_file = file.metadata().await.unwrap().len() == 0;

        file.set_len(4096).await.map_err(Into::<Error>::into)?;

        debug!("Successfully set the file len of {:?} to 4096.", &path);

        let mut data =
            unsafe { ContentsMemMap::new(MmapMut::map_mut(&file).map_err(Into::<Error>::into)?) };

        if is_new_file {
            data.magic_bytes = *b"EVERYTHINGDB";
            data.version = 1;
            data.ifp.set(InvalidFormatPolicy::Error);

            debug!("Successfully initialized database.");
        } else {
            if !data.ifp.is_valid() {
                return Err(Error::InvalidValueForInvalidFormatPolicy)
            }
        }

        data.validation_id += 1;

        Ok(Self {
            cached_validation_id: data.validation_id,
            path,
            data: RwLock::new(data),
            file,
        })
    }

    #[inline]
    pub async fn invalid_format_policy(&self) -> InvalidFormatPolicy {
        self.data.read().await.ifp.get()
    }

    #[inline]
    pub async fn set_invalid_format_policy(&self, policy: InvalidFormatPolicy) {
        self.data.read().await.ifp.set(policy);
    }

    /// Ensures that database changes are written to disk.
    #[inline]
    pub async fn flush(&self) -> Result<(), Error> {
        debug!("Waiting for write lock on data.");

        let data_guard = self.data.write().await;

        debug!("Starting flushing...");
        self.file.sync_all().await.map_err(Into::<Error>::into)?;

        let _ = data_guard;

        info!("Successfully flushed database.");

        Ok(())
    }

    #[inline]
    pub async fn version(&self) -> u32 {
        self.data.read().await.version
    }

    /// Returns the number of blocks in use.
    #[inline]
    pub async fn blocks_used(&self) -> u64 {
        self.data
            .read()
            .await
            .blocks_list_meta
            .read()
            .await
            .used_block_count
    }

    #[inline]
    pub async fn blocks_reserved(&self) -> u64 {
        let data = self.data.read().await;
        let meta = data.blocks_list_meta.read().await;
        meta.free_block_count + meta.used_block_count
    }

    /// Returns the number of free (but allocated) blocks.
    #[inline]
    pub async fn blocks_free(&self) -> u64 {
        self.data
            .read()
            .await
            .blocks_list_meta
            .read()
            .await
            .free_block_count
    }

    async fn allocate_block(&self) -> NonZeroU64 {
        let data = self.data.read().await;

        let mut free_list = data.blocks_list_meta.write().await;

        free_list.free_block_count = free_list.free_block_count.saturating_sub(1);
        free_list.used_block_count = free_list.used_block_count.saturating_add(1);

        let block_id = free_list.free_list_head;

        let block = &data.blocks[block_id.get() as usize];

        let next_free_block = match block {
            Block::FreeList(Some(next)) => *next,
            Block::FreeList(None) => panic!("Free list is empty."),
            _ => panic!(
                "Head of free list is not a free list block (block_id = {}).",
                block_id.get()
            ),
        };

        block_id
    }

    /// Queries the database. More information at [crate::query].
    #[inline]
    pub fn query<Q: for<'a> Query<'a>>(&self, query: Q) -> <Q as Query>::Output {
        query.query(self)
    }
}