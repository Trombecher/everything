use rusqlite::Connection;

pub mod files;
pub mod objects;
pub mod config;
mod migrate;
mod types;
mod users;

pub fn fix(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("\
    CREATE TABLE IF NOT EXISTS objects (\
        id INTEGER UNIQUE NOT NULL PRIMARY KEY AUTOINCREMENT,\
        type INTEGER NOT NULL\
    ) STRICT;\
    CREATE TABLE IF NOT EXISTS types (\
        id INTEGER UNIQUE NOT NULL PRIMARY KEY,\
        schema INTEGER,\
        flags  INTEGER NOT NULL,\
        parent INTEGER REFERENCES types (id)\
    ) STRICT;\
    CREATE TABLE config (
        key INTEGER NOT NULL PRIMARY KEY UNIQUE,
        value ANY
    ) STRICT;\
    ")
}