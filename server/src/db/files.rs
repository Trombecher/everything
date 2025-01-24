//! This module defines atomic file operations on the database.

use rusqlite::{params, Connection};
use crate::error::{Error, InternalError};
use crate::objects::{ExistingFileID, FileID, ObjectID};

pub fn create(connection: &Connection) -> Result<ExistingFileID, Error> {
    let id = connection.prepare_cached("INSERT INTO objects (type) values (1)")?
        .insert([])
        .map_err(Error::from)?;

    if let Ok(oid) = ObjectID::try_from(id) {
        Ok(unsafe { ExistingFileID::new(FileID(oid)) })
    } else {
        Err(Error::Internal(InternalError::AutoFileIDIsNotValid))
    }
}

pub fn exists(connection: &Connection, object_id: ObjectID) -> rusqlite::Result<Option<ExistingFileID>> {
    connection.prepare_cached("SELECT 1 FROM objects WHERE id = ? AND type = 1")?
        .exists(params![object_id])
        .map(|b| b.then_some(unsafe { ExistingFileID::new(FileID(object_id)) }))
}