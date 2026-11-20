use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use everything::{
    knowledge::Statement,
    model::{Atom, Object},
};

use super::*;

/// A temporary directory that is removed when it goes out of scope.
struct TempDirectory(PathBuf);

impl TempDirectory {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("the clock is after the Unix epoch")
            .as_nanos();
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let path =
            env::temp_dir().join(format!("everything_db_test_{}_{nanos}_{id}", process::id()));

        fs::create_dir_all(&path).expect("the temporary directory must be creatable");

        Self(path)
    }

    fn database_path(&self) -> PathBuf {
        self.0.join("knowledge.everything")
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn statement(atom: u128, object: Object) -> Statement {
    Statement {
        atom: Atom(atom),
        object,
    }
}

fn load(path: &Path) -> PersistentKnowledge {
    PersistentKnowledge::load(path, false).expect("the database must be loadable")
}

#[test]
fn load_creates_an_empty_database_if_missing() {
    let directory = TempDirectory::new();
    let path = directory.database_path();

    let database = PersistentKnowledge::load(&path, true).expect("the database must be created");

    assert!(path.is_file());
    assert_eq!(database.path(), path.as_path());
    assert!(database.knowledge().map.is_empty());
}

#[test]
fn load_without_create_fails_for_a_missing_file() {
    let directory = TempDirectory::new();
    let path = directory.database_path();

    match PersistentKnowledge::load(&path, false) {
        Err(Error::Open(_)) => {}
        Err(error) => panic!("unexpected error: {error}"),
        Ok(_) => panic!("the database must not be created"),
    }

    assert!(!path.exists());
}

#[test]
fn load_does_not_overwrite_an_existing_database() {
    let directory = TempDirectory::new();
    let path = directory.database_path();

    {
        let mut database =
            PersistentKnowledge::load(&path, true).expect("the database must be created");
        let statement = statement(1, Object::Atom(Atom(2)));

        database
            .store([StatementChange::Add(&statement)].into_iter())
            .expect("the change must be stored");
    }

    let database = PersistentKnowledge::load(&path, true).expect("the database must be loaded");

    assert_eq!(database.knowledge().map.len(), 1);
}

#[test]
fn store_persists_changes_and_updates_the_knowledge() {
    let directory = TempDirectory::new();
    let path = directory.database_path();
    let mut database =
        PersistentKnowledge::load(&path, true).expect("the database must be created");

    let statement = statement(1, Object::Atom(Atom(2)));

    database
        .store([StatementChange::Add(&statement)].into_iter())
        .expect("the change must be stored");

    assert_eq!(database.knowledge().map, load(&path).knowledge().map);
    assert!(database.knowledge().map.contains_key(&Atom(1)));

    database
        .store([StatementChange::Remove(&statement)].into_iter())
        .expect("the change must be stored");

    assert!(database.knowledge().map.is_empty());
    assert_eq!(database.knowledge().map, load(&path).knowledge().map);
}

#[test]
fn compact_preserves_the_knowledge_and_removes_the_change_log() {
    let directory = TempDirectory::new();
    let path = directory.database_path();
    let mut database =
        PersistentKnowledge::load(&path, true).expect("the database must be created");

    let statements = [
        statement(1, Object::Atom(Atom(2))),
        statement(3, Object::Atom(Atom(4))),
    ];

    database
        .store(
            [
                StatementChange::Add(&statements[0]),
                StatementChange::Add(&statements[1]),
                StatementChange::Remove(&statements[1]),
            ]
            .into_iter(),
        )
        .expect("the changes must be stored");

    let size_before = fs::metadata(&path)
        .expect("the database file must exist")
        .len();
    database.compact().expect("the database must compact");
    let size_after = fs::metadata(&path)
        .expect("the database file must exist")
        .len();

    assert!(size_after < size_before);
    assert!(!compacted_path(&path).exists());
    assert_eq!(database.knowledge().map.len(), 1);
    assert_eq!(database.knowledge().map, load(&path).knowledge().map);
}

#[test]
fn store_after_compact_appends_incremental_changes() {
    let directory = TempDirectory::new();
    let path = directory.database_path();
    let mut database =
        PersistentKnowledge::load(&path, true).expect("the database must be created");

    let first = statement(1, Object::Atom(Atom(2)));

    database
        .store([StatementChange::Add(&first)].into_iter())
        .expect("the change must be stored");
    database.compact().expect("the database must compact");

    let second = statement(3, Object::Atom(Atom(4)));

    database
        .store([StatementChange::Add(&second)].into_iter())
        .expect("the change must be stored");

    let reloaded = load(&path);

    assert_eq!(reloaded.knowledge().map, database.knowledge().map);
    assert_eq!(reloaded.knowledge().map.len(), 2);
}
