use core::slice;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use memmap2::MmapMut;

use crate::{
    Error,
    page::{PAGE_SIZE, Page},
};

pub struct Pages {
    file: Option<File>,
    map: MmapMut,
}

impl Pages {
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .map_err(Error::Io)?;

        file.lock().map_err(Error::Io)?;

        let map = unsafe { MmapMut::map_mut(&file) }.map_err(Error::Io)?;

        if map.len() % PAGE_SIZE != 0 {
            return Err(Error::FileSizeIsNotPageAligned);
        }

        Ok(Self {
            file: Some(file),
            map,
        })
    }

    pub fn from_memory() -> Result<Self, Error> {
        let map = MmapMut::map_anon(PAGE_SIZE * 256).map_err(Error::Io)?;

        Ok(Self { file: None, map })
    }

    pub fn increase_size_to(&mut self, pages: usize) -> Result<(), Error> {
        if pages <= self.pages().len() {
            return Ok(());
        }

        if let Some(file) = &self.file {
            file.set_len((pages * PAGE_SIZE) as u64)
                .map_err(Error::Io)?;

            self.map = unsafe { MmapMut::map_mut(file) }.map_err(Error::Io)?
        } else {
            todo!()
        }

        Ok(())
    }

    #[inline(always)]
    pub fn pages(&self) -> &[Page] {
        let bytes: &[u8] = &self.map;

        unsafe { slice::from_raw_parts(bytes.as_ptr() as *const Page, bytes.len() / PAGE_SIZE) }
    }

    #[inline(always)]
    pub fn pages_mut(&mut self) -> &mut [Page] {
        let bytes: &mut [u8] = &mut self.map;

        unsafe {
            slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut Page, bytes.len() / PAGE_SIZE)
        }
    }
}
