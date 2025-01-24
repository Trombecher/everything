use rusqlite::Connection;

#[inline]
pub fn get_db_version(conn: &Connection) -> Result<u32, rusqlite::Error> {
    conn.pragma_query_value(None, "user_version", |row| row.get(0))
}

#[inline]
pub fn set_db_version(conn: &Connection, new_version: u32) -> Result<(), rusqlite::Error> {
    conn.pragma_update(None, "user_version", new_version)
}