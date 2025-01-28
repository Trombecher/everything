use rusqlite::Connection;

pub mod objects;
pub mod config;
pub mod migrate;
pub mod types;

/// This upper limit should be respected for each allocation to mitigate DDOS attacks.
pub(self) const MAX_ALLOCATION_ITEMS: usize = 1024;

pub fn fix(conn: &Connection) -> Result<(), rusqlite::Error> {
    /*
    CREATE TABLE IF NOT EXISTS objects (\
        id INTEGER UNIQUE NOT NULL PRIMARY KEY AUTOINCREMENT,\
        type INTEGER NOT NULL\
    ) STRICT;\
     */

    conn.execute_batch("\
    CREATE TABLE IF NOT EXISTS iF (\
        id INTEGER UNIQUE NOT NULL PRIMARY KEY,\
        \"0\" BLOB\
    ) STRICT;\
    CREATE TABLE IF NOT EXISTS i10(\
        id INTEGER UNIQUE NOT NULL PRIMARY KEY\
    ) STRICT;\
    CREATE TABLE IF NOT EXISTS i11 (\
        id INTEGER UNIQUE NOT NULL PRIMARY KEY,\
        schema INTEGER,\
        flags  INTEGER NOT NULL,\
        parent INTEGER REFERENCES types (id)\
    ) STRICT;\
    CREATE TABLE config (\
        key INTEGER NOT NULL PRIMARY KEY UNIQUE,\
        value ANY\
    ) STRICT;\
    ")
}