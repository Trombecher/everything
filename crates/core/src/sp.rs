//! Storage providers.

use crate::Error;
use crate::alloc::Allocator;
use crate::pages::{PageId, UnknownPage};
use memmap2::MmapMut;
use static_assertions::const_assert;
use std::mem::transmute;
use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::sync::RwLock;
use tracing::{debug, info};

#[repr(align(4096))]
pub struct MetaPage {
    /// Magic bytes "EVERYTHINGDB".
    pub magic_bytes: [u8; 12],
    /// The version of the database.
    pub version: u32,
    /// The root validation ID. Incremented on open.
    pub validation_id: u64,
    /// Allocator, manages pages.
    pub allocator: RwLock<Allocator>,
    /// The current data snapshot.
    pub snapshot: RwLock<Snapshot>,
    /// The first page of the WAL.
    pub wal: PageId,
    /// Writer threads must acquire a read lock on this
    /// lock before modifying any of the contents.
    /// Flush calls acquire a write lock.
    ///
    /// This is so that nothing can be written
    /// when the db is flushed to disk.
    /// Readers, however, are not blocked by this
    /// and should try to acquire this lock.
    ///
    /// This lock should only be used by
    /// [FileBasedStorageProvider].
    pub writer_lock: RwLock<()>,
}

const_assert!(size_of::<MetaPage>() <= 4096);

#[derive(Copy, Clone)]
pub struct Snapshot {
    pub object_id_btree_root_node: PageId,
    pub tag_id_btree_root_node: PageId,
    pub value_btree_root_node: PageId,
}

#[repr(C, align(4096))]
pub struct Contents {
    pub meta: MetaPage,
    pub pages: [UnknownPage],
}

pub trait StorageProvider {
    const USES_WRITER_LOCK: bool;

    fn contents(&self) -> &Contents;
    fn ensure_page_count(&self, count: usize);
    async fn flush(&self) -> Result<(), Error>;
}

pub struct FileBasedStorageProvider {
    file: File,
    path: Box<Path>,
    data: MmapMut,
}

impl FileBasedStorageProvider {
    pub async fn new(path: Box<Path>) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .await
            .map_err(Into::<Error>::into)?;

        let is_new_file = file.metadata().await?.len() == 0;

        if is_new_file {
            file.set_len(4096).await.map_err(Into::<Error>::into)?;

            debug!("Successfully set the file len of {:?} to 4096.", &path);
        }

        let mut data = unsafe { MmapMut::map_mut(&file).map_err(Into::<Error>::into)? };

        let contents = unsafe { &mut *(&mut data[..] as *mut [u8] as *mut Contents) };

        if is_new_file {
            contents.meta.magic_bytes = *b"EVERYTHINGDB";
            contents.meta.version = 1;

            // contents.meta.ifp.set(InvalidFormatPolicy::Error);

            debug!("Successfully initialized database.");
        } else {
            /*
            if !data.ifp.is_valid() {
                return Err(Error::InvalidValueForInvalidFormatPolicy);
            }
             */
        }

        contents.meta.validation_id += 1;

        Ok(Self { path, data, file })
    }
}

impl StorageProvider for FileBasedStorageProvider {
    const USES_WRITER_LOCK: bool = true;

    #[inline]
    fn contents(&self) -> &Contents {
        unsafe { &*(&self.data[..] as *const [u8] as *const Contents) }
    }

    fn ensure_page_count(&self, count: usize) {
        todo!()
    }

    async fn flush(&self) -> Result<(), Error> {
        debug!("Waiting for writer lock on data.");

        let data_guard = self.contents().meta.writer_lock.write().await;

        debug!("Starting flushing...");
        self.file.sync_all().await.map_err(Into::<Error>::into)?;

        let _ = data_guard;

        info!("Successfully flushed database.");

        Ok(())
    }
}

pub struct InMemoryStorageProvider {
    allocation: Box<Contents>,
}

impl InMemoryStorageProvider {
    pub fn new(initial_page_count: usize) -> Self {
        Self {
            allocation: unsafe {
                transmute(Box::<[UnknownPage]>::new_zeroed_slice(initial_page_count))
            },
        }
    }
}

impl StorageProvider for InMemoryStorageProvider {
    const USES_WRITER_LOCK: bool = false;

    fn contents(&self) -> &Contents {
        &self.allocation
    }

    fn ensure_page_count(&self, count: usize) {
        todo!()
    }

    async fn flush(&self) -> Result<(), Error> {
        // No-op
        Ok(())
    }
}
