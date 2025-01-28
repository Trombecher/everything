use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;

/// An object id that might not exist.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct ObjectId(pub i64);

impl From<ValidatedObjectId> for ObjectId {
    fn from(value: ValidatedObjectId) -> Self {
        value.0
    }
}

impl ToSql for ObjectId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl ToSql for ValidatedObjectId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

/// A file ID that is guaranteed to exist during the wrapping transaction.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct ValidatedObjectId(ObjectId);

impl ValidatedObjectId {
    #[inline]
    pub const unsafe fn new(id: ObjectId) -> Self {
        Self(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct UserDefinedObjectId(ObjectId);

impl UserDefinedObjectId {
    pub const MIN_ID: ObjectId = ObjectId(1024); // Magic number
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct ValidatedUserId(ValidatedObjectId);