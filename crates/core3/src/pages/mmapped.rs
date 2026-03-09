use std::{fs::OpenOptions, path::Path, sync::Arc};

use memmap2::MmapMut;
use tokio::{fs::File, task::spawn_blocking};

use crate::{
    Error,
    pages::{PAGE_SIZE, PageProvider},
};

pub struct MemoryMapPageProvider {
    map: MmapMut,
}

impl MemoryMapPageProvider {
    pub async fn new(path: Option<Arc<Path>>) -> Result<Self, Error> {
        let map = if let Some(path) = path {
            let (std_file, map) = spawn_blocking(move || {
                let std_file = OpenOptions::new().create(true).write(true).open(path)?;
                std_file.lock()?;

                let map = unsafe { MmapMut::map_mut(&std_file)? };

                Ok((std_file, map))
            })
            .await
            .unwrap()
            .map_err(Error::Io)?;

            let file = File::from_std(std_file);

            if map.len() == 0 {
                // New file -> init db
            } else {
                // Existing file
            }

            // TODO: check magic bytes and some more

            map
        } else {
            // No file, anon map.

            let map = spawn_blocking(|| MmapMut::map_anon(PAGE_SIZE as usize))
                .await
                .unwrap()
                .map_err(Error::Io)?;

            map
        };

        Ok(Self { map })
    }
}

impl PageProvider for MemoryMapPageProvider {
    async fn page<'backend>(
        &'backend self,
        page_index: u64,
    ) -> Result<&'backend super::Page, crate::Error> {
        todo!()
    }

    async fn flush_page(&self, page_index: u64) -> Result<(), crate::Error> {
        todo!()
    }
}
