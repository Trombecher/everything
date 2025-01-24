//! This module defines atomic object operations on the database.

use crate::proto::MessageBuffer;
use crate::types::{BuiltInTypeID, TypeID};
use rusqlite::{Connection, params};
use crate::error::Error;
use crate::objects::{ExistingObjectID, ObjectID};

pub fn exists(
    conn: &Connection,
    object_id: ObjectID
) -> Result<Option<ExistingObjectID>, rusqlite::Error> {
    conn.prepare_cached("SELECT 1 FROM objects WHERE id = ?")?
        .exists(params![object_id])
        .map(|exists| exists.then_some(unsafe { ExistingObjectID::new(object_id) }))
}

pub fn delete(connection: &Connection, object_id: ObjectID) -> rusqlite::Result<()> {
    connection.prepare_cached("DELETE FROM objects WHERE id = ?")?
        .execute(params![object_id])
        .map(|_| ())
}

pub fn all(
    connection: &Connection,
    limit: usize,
    offset: usize,
    rb: &mut MessageBuffer,
) -> Result<(), Error> {
    let reserved = rb.reserve::<u64>();
    let mut i = 0_u64;

    connection
        .prepare("SELECT id, type FROM objects LIMIT ? OFFSET ?")
        .map_err(Error::from)?
        .query_map(rusqlite::params![limit, offset], |row| {
            rb.encode(&row.get::<_, i64>(0)?); // id
            rb.encode(&row.get::<_, u8>(1)?); // type
            i += 1;
            Ok(())
        })
        .map_err(Error::from)?
        .for_each(|_| {});

    rb.encode_reserved(reserved, &i);
    Ok(())
}

pub fn query_types_of_object(
    connection: &Connection,
    object_id: i64,
    limit: usize,
    offset: usize,
    mb: &mut MessageBuffer,
) -> rusqlite::Result<()> {
    let mut selected_ids = Vec::with_capacity(limit.max(128)); // TODO: magic number

    connection
        .prepare_cached("SELECT id FROM types WHERE schema = NULL LIMIT ? OFFSET ?")?
        .query_map(params![limit, offset], |row| row.get::<_, i64>(0))?
        .collect_into(&mut selected_ids);

    Ok(())
}

pub fn query_by_type(
    connection: &Connection,
    type_id: TypeID,
    limit: usize,
    offset: usize,
    mb: &mut MessageBuffer,
) -> rusqlite::Result<()> {
    let reserved = mb.reserve::<u64>();
    let mut count = 0;

    match type_id {
        TypeID::BuiltIn(BuiltInTypeID::File) => {
            for id in connection
                .prepare_cached("SELECT id FROM objects WHERE type = 1 LIMIT ? OFFSET ?")?
                .query_map(params![limit, offset], |row| row.get::<_, i64>(0))?
            {
                mb.encode(&id?);
                count += 1
            }
        }
        TypeID::BuiltIn(_) => todo!(),
        TypeID::UserDefined(_) => todo!(),
    }

    mb.encode_reserved(reserved, &count);

    Ok(())
}

// 49   32
// File LastRead(_ < NOW - 10d)

// -> 49 && 32 && (!