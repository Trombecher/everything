//! This module defines atomic object operations on the database.

use crate::constraints::Constraint;
use crate::error::Error;
use crate::objects::{ObjectId, ValidatedObjectId};
use rusqlite::{Connection, params, params_from_iter};

pub fn create(conn: &Connection) -> Result<ValidatedObjectId, Error> {
    conn.prepare_cached("INSERT INTO objects VALUES ()")
        .map_err(Error::from)?
        .insert([])
        .map_err(Error::from)
        .map(|id| unsafe { ValidatedObjectId::new(ObjectId(id)) })
}

pub fn validate(
    conn: &Connection,
    object_id: ObjectId,
) -> Result<Option<ValidatedObjectId>, rusqlite::Error> {
    conn.prepare_cached("SELECT 1 FROM objects WHERE id = ?")?
        .exists(params![object_id])
        .map(|exists| exists.then_some(unsafe { ValidatedObjectId::new(object_id) }))
}

pub fn delete(connection: &Connection, object_id: ObjectId) -> rusqlite::Result<()> {
    connection
        .prepare_cached("DELETE FROM objects WHERE id = ?")?
        .execute(params![object_id])
        .map(|_| ())
}

/// Collects all objects ids into a [Vec].
fn all(connection: &Connection, limit: u16, offset: u64) -> Result<Vec<ValidatedObjectId>, Error> {
    let mut object_ids = Vec::with_capacity(limit.max(1024) as usize);

    for id in connection
        .prepare("SELECT id FROM objects LIMIT ? OFFSET ?")
        .map_err(Error::from)?
        .query_map(rusqlite::params![limit, offset], |row| row.get::<_, i64>(0))
        .map_err(Error::from)?
    {
        object_ids.push(unsafe { ValidatedObjectId::new(ObjectId(id?)) });
    }

    Ok(object_ids)
}

pub fn query_types_of_object(
    connection: &Connection,
    object_id: i64,
    limit: usize,
    offset: usize,
) -> rusqlite::Result<()> {
    let mut selected_ids = Vec::with_capacity(limit.max(128)); // TODO: magic number

    connection
        .prepare_cached("SELECT id FROM types WHERE schema = NULL LIMIT ? OFFSET ?")?
        .query_map(params![limit, offset], |row| row.get::<_, i64>(0))?
        .collect_into(&mut selected_ids);

    Ok(())
}

pub fn query(
    conn: &Connection,
    constraint: Option<Constraint>,
    limit: u16,
    offset: u64,
) -> Result<Vec<ValidatedObjectId>, Error> {
    let constraint = match constraint {
        Some(c) => c,
        None => return all(conn, limit, offset),
    };

    let mut p = Vec::new();
    let mut s = String::new();

    let mut ids = Vec::new();

    constraint.build_query(conn, &mut s, &mut p)?;

    for id in conn
        .prepare(&s)
        .map_err(Error::from)?
        .query_map(params_from_iter(p.iter()), |row| row.get::<_, i64>(0))
        .map_err(Error::from)?
    {
        unsafe {
            ids.push(ValidatedObjectId::new(ObjectId(id?)));
        }
    }

    Ok(ids)
}
