use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use everything::knowledge::Knowledge;
use memmap2::Mmap;

use crate::{
    StatementChange,
    format::{FileFormatApi, FileFormatApiContract, ParseError},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not open the database file")]
    Open(#[source] io::Error),
    #[error("could not memory-map the database file")]
    Map(#[source] io::Error),
    #[error("could not read the database file")]
    Parse(#[from] ParseError),
    #[error("could not write the database file")]
    Write(#[source] io::Error),
    #[error("could not replace the database file with its compacted copy")]
    Rename(#[source] io::Error),
}

/// Knowledge that is persisted in a file on disk.
///
/// The file is opened for reading and appending and stays open for the lifetime
/// of this value. It is only memory-mapped while it is being read, so no mapping
/// is kept around.
pub struct PersistantKnowledge {
    knowledge: Knowledge,
    file: File,
    /// The file `knowledge` was loaded from and `file` refers to.
    path: PathBuf,
}

impl PersistantKnowledge {
    /// Loads the knowledge base stored at `path`.
    ///
    /// The file is memory-mapped and read through [`FileFormatApi`]; the mapping
    /// is released as soon as the knowledge has been parsed. The file itself is
    /// kept open for [`Self::store`].
    ///
    /// If `create_if_missing` is `true` and the file does not exist, it is created
    /// containing an empty knowledge base and then loaded as usual.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Open`] if the file cannot be opened or created, [`Error::Map`]
    /// if it cannot be memory-mapped, and [`Error::Parse`] if its contents are not
    /// a valid knowledge base file.
    pub fn load(path: impl AsRef<Path>, create_if_missing: bool) -> Result<Self, Error> {
        Self::_load(path.as_ref(), create_if_missing)
    }

    #[allow(clippy::used_underscore_items)] // `_load` is the non-generic variant on purpose.
    fn _load(path: &Path, create_if_missing: bool) -> Result<Self, Error> {
        let file = match Self::open_file(path) {
            Ok(file) => file,
            Err(Error::Open(source))
                if create_if_missing && source.kind() == io::ErrorKind::NotFound =>
            {
                Self::create_empty_file(path)?;
                Self::open_file(path)?
            }
            Err(error) => return Err(error),
        };

        // SAFETY: The mapping is only read from while this function runs and is
        // never handed out. Concurrent modification of the file by other
        // processes may result in undefined behaviour; that is an accepted
        // limitation of memory maps and not caused by this crate.
        let mapped = unsafe { Mmap::map(&file) }.map_err(Error::Map)?;

        let knowledge = FileFormatApi::read_knowledge(&mapped)?;

        Ok(Self {
            knowledge,
            file,
            path: path.to_path_buf(),
        })
    }

    /// Returns the knowledge held by this database.
    #[must_use]
    pub fn knowledge(&self) -> &Knowledge {
        &self.knowledge
    }

    /// Writes all knowledge into a new file and atomically replaces the database
    /// file with it.
    ///
    /// Rewriting from scratch compacts the file: the change log is dropped and
    /// only the current knowledge remains. The compacted content is written to
    /// `<path>.compacting` first, so `<path>` is only replaced once that file is
    /// complete.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Open`] if the new file cannot be created, [`Error::Write`]
    /// if the knowledge cannot be written to it, and [`Error::Rename`] if it
    /// cannot be moved into place.
    pub fn compact(&mut self) -> Result<(), Error> {
        let compacted_path = compacted_path(&self.path);

        let result = Self::write_new_file(&self.knowledge, &compacted_path)
            .and_then(|()| fs::rename(&compacted_path, &self.path).map_err(Error::Rename));

        if result.is_err() {
            // Never leave a half-written compacted file behind.
            let _ = fs::remove_file(&compacted_path);
        }

        result?;

        // The file handle still refers to the replaced file, so it has to be
        // reopened. Assigning also drops the handle to the old file.
        self.file = Self::open_file(&self.path)?;

        Ok(())
    }

    /// Appends `changes` to the end of the database file.
    ///
    /// The changes are encoded into a vector first and then appended through the
    /// file handle held by this database, so a single call never leaves a
    /// partially encoded change behind. This only records the changes on disk;
    /// it does not apply them to [`Self::knowledge`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Write`] if the encoded changes cannot be appended.
    pub fn store<'a>(
        &mut self,
        changes: impl Iterator<Item = StatementChange<'a>>,
    ) -> Result<(), Error> {
        let mut encoded = Vec::new();
        FileFormatApi::change_knowledge_incremental(changes, &mut encoded);

        self.file.write_all(&encoded).map_err(Error::Write)
    }

    fn open_file(path: &Path) -> Result<File, Error> {
        // On Windows this opens with the standard library's default sharing
        // mode, which includes `FILE_SHARE_DELETE`. That is what allows the
        // file to be replaced by `compact` while the handle is open.
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)
            .map_err(Error::Open)
    }

    /// Creates `path` containing an empty knowledge base.
    fn create_empty_file(path: &Path) -> Result<(), Error> {
        match Self::write_new_file(&Knowledge::empty(), path) {
            // Another writer created the file between the failed open and this
            // call. Whatever it contains is loaded instead.
            Err(Error::Open(source)) if source.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            result => result,
        }
    }

    fn write_new_file(knowledge: &Knowledge, path: &Path) -> Result<(), Error> {
        let mut encoded = Vec::new();
        FileFormatApi::write_knowledge(knowledge, &mut encoded);

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(Error::Open)?;

        file.write_all(&encoded).map_err(Error::Write)?;

        // Make sure the content reaches the disk before the file is renamed into
        // place or opened for reading.
        file.sync_all().map_err(Error::Write)
    }
}

fn compacted_path(path: &Path) -> PathBuf {
    let mut compacted = path.as_os_str().to_os_string();
    compacted.push(".compacting");
    PathBuf::from(compacted)
}
