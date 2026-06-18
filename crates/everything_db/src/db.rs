use std::{io::ErrorKind, path::PathBuf};

use everything_objects::{Composite, Object};
use everything_tff::{encode::Encoder, parse::Parser};
use memmap2::Mmap;
use tokio::{
    fs::{self, OpenOptions},
    io::{self, AsyncWriteExt},
    task::spawn_blocking,
};

use crate::Error;

pub struct Database {
    pub path: PathBuf,
    pub root: Object,
}

impl Database {
    /// Creates a new empty database.
    #[must_use]
    #[inline]
    pub const fn empty(path: PathBuf) -> Self {
        Self {
            path,
            root: Object::Composite(Composite::Empty),
        }
    }

    #[inline]
    async fn from_file_and_content(
        std_file: std::fs::File,
        content: Mmap,
        path: PathBuf,
    ) -> Result<Self, Error> {
        // validate that content is UTF-8
        let source = str::from_utf8(&content).map_err(|_| Error::DbFileIsInvalidUTF8)?;

        let root = Parser::new(source)
            .parse_root()
            .map_err(Error::ErrorWhileParsingDbFile)?;

        spawn_blocking(move || {
            std_file.unlock()?;

            Ok(())
        })
        .await
        .unwrap()
        .map_err(Error::Io)?;

        Ok(Self { path, root })
    }

    /// Opens the given database file. Errors if the database file could not be opened.
    pub async fn open(path: PathBuf) -> Result<Self, Error> {
        let (std_file, content, path) = spawn_blocking(move || {
            let std_file = std::fs::File::open(&path)?;
            // Lock the file to prevent mutations. We need to rely on
            // the content staying valid UTF-8 after validation
            // (or else UB occurs).
            std_file.lock()?;

            let content = unsafe { Mmap::map(&std_file) }?;

            Ok((std_file, content, path))
        })
        .await
        .unwrap()
        .map_err(Error::Io)?;

        Self::from_file_and_content(std_file, content, path).await
    }

    /// Similar behaviour to [`Database::open`] but returns an empty database
    /// if the database file does not exist.
    pub async fn new(path: PathBuf) -> Result<Self, Error> {
        let (returned, path) = spawn_blocking(move || {
            let std_file = match std::fs::File::open(&path) {
                Ok(file) => file,
                Err(err) if err.kind() == ErrorKind::NotFound => return Ok((None, path)),
                Err(err) => return Err(err),
            };

            // Lock the file to prevent mutations. We need to rely on
            // the content staying valid UTF-8 after validation
            // (or else UB occurs).
            std_file.lock()?;

            let content = unsafe { Mmap::map(&std_file) }?;

            Ok((Some((std_file, content)), path))
        })
        .await
        .unwrap()
        .map_err(Error::Io)?;

        let Some((std_file, content)) = returned else {
            return Ok(Self::empty(path));
        };

        Self::from_file_and_content(std_file, content, path).await
    }

    pub async fn save(&self) -> Result<(), io::Error> {
        let snapshot_file_path = self.path.with_added_extension("new");

        let mut snapshot_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&snapshot_file_path)
            .await?;

        // TODO: eventually replace this with a smaller buffer.
        let mut temp = String::new();

        Encoder::new(&mut temp)
            .encode_root(self.root.clone())
            .unwrap();

        snapshot_file.write_all(temp.as_bytes()).await?;
        snapshot_file.flush().await?;

        fs::rename(&snapshot_file_path, &self.path).await?;

        Ok(())
    }
}
