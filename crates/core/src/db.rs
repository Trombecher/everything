use crate::sp::{FileBasedStorageProvider, InMemoryStorageProvider, StorageProvider};
use crate::{Error, query::Query};
use std::path::Path;

pub struct Database<P: StorageProvider> {
    provider: P,
}

impl Database<InMemoryStorageProvider> {
    /// Creates a new in-memory database.
    #[inline]
    pub fn new_in_memory() -> Self {
        // Magic number, IDK
        const INITIAL_PAGE_COUNT: usize = 1024;

        Database {
            provider: InMemoryStorageProvider::new(INITIAL_PAGE_COUNT),
        }
    }
}

impl Database<FileBasedStorageProvider> {
    /// Creates a new database that operates on a file.
    #[inline]
    pub async fn new(path: Box<Path>) -> Result<Database<FileBasedStorageProvider>, Error> {
        Ok(Database {
            provider: FileBasedStorageProvider::new(path).await?,
        })
    }
}

impl<P: StorageProvider> Database<P> {
    /// Creates a new database from a storage provider.
    #[inline]
    pub fn new_from_provider(provider: P) -> Self {
        Self { provider }
    }

    /// Calls the flush behavior of the storage provider.
    #[inline]
    pub async fn flush(&self) -> Result<(), Error> {
        self.provider.flush().await
    }

    #[inline]
    pub fn version(&self) -> u32 {
        self.provider.contents().meta.version
    }

    /// Queries the database. More information at [crate::query].
    #[inline]
    pub fn query<Q: for<'a> Query<'a, P>>(&self, query: Q) -> <Q as Query<P>>::Output {
        query.query(self)
    }
}
