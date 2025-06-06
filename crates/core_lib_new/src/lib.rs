#![feature(if_let_guard)]
extern crate core;

pub mod content;
pub mod objects;
pub mod values;
mod ff;

use memmap2::MmapMut;
use std::path::Path;
use tokio::fs::{File, OpenOptions};

pub struct Database {
    path: Box<Path>,
    file: File,
    map: MmapMut,
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

        file.set_len(16).await.map_err(|_| ())?;

        let map = unsafe { MmapMut::map_mut(&file) }.map_err(|_| ())?;

        Ok(Self { path, map, file })
    }
}
