//! This module handles user management.

use rusqlite::{params, Connection};
use crate::error::Error;
use crate::objects::{ExistingUserID, UserID};

/// Returns `Some(ExistingUserID)` if the user id exists; `None` otherwise.
pub fn exists(
    conn: &Connection,
    user_id: UserID,
) -> Result<Option<ExistingUserID>, Error> {
    conn.prepare_cached("SELECT 1 FROM objects WHERE id = ?")
        .map_err(Error::from)?
        .exists(params![user_id])
        .map_err(Error::from)
        .map(|b| b.then_some(unsafe { ExistingUserID::new(user_id) }))
}