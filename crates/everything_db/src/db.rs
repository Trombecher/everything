use std::{
    fs::{self, File, OpenOptions},
    io::{self, ErrorKind, Write},
    path::PathBuf,
};

use everything_objects::{Composite, Object};
use everything_tff::{encode::Encoder, parse::Parser};
use memmap2::Mmap;

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
    fn from_file_and_content(file: File, content: Mmap, path: PathBuf) -> Result<Self, Error> {
        // Validate that content is UTF-8
        let source = str::from_utf8(&content).map_err(|_| Error::DbFileIsInvalidUTF8)?;

        let root = Parser::new(source)
            .parse_root()
            .map_err(Error::ErrorWhileParsingDbFile)?;

        file.unlock().map_err(Error::from)?;

        Ok(Self { path, root })
    }

    /// Opens the given database file. Errors if the database file could not be opened.
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let file = File::open(&path).map_err(Error::from)?;

        // Lock the file to prevent mutations. We need to rely on
        // the content staying valid UTF-8 after validation
        // (or else UB occurs).
        file.lock().map_err(Error::from)?;

        // Because the file is locked, this is safe.
        let content = unsafe { Mmap::map(&file) }?;

        Self::from_file_and_content(file, content, path)
    }

    /// Similar behaviour to [`Database::open`] but returns an empty database
    /// if the database file does not exist.
    pub async fn new(path: PathBuf) -> Result<Self, Error> {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::empty(path)),
            Err(error) => return Err(Error::from(error)),
        };

        file.lock().map_err(Error::from)?;

        let content = unsafe { Mmap::map(&file) }?;

        Self::from_file_and_content(file, content, path)
    }

    pub async fn save(&self) -> Result<(), io::Error> {
        let snapshot_file_path = self.path.with_added_extension("new");

        let mut snapshot_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&snapshot_file_path)?;

        snapshot_file.lock()?;

        // TODO: eventually replace this with a smaller buffer.
        let mut temp = String::new();

        Encoder::new(&mut temp)
            .encode_root(self.root.clone())
            .unwrap();

        snapshot_file.write_all(temp.as_bytes())?;
        snapshot_file.flush()?;

        snapshot_file.unlock()?;

        fs::rename(&snapshot_file_path, &self.path)?;

        Ok(())
    }
}
