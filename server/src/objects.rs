use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;

/// An object ID that might not exist.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct ObjectID(i64);

impl ObjectID {
    pub const MIN_ID: ObjectID = ObjectID(1024); // magic number
}

impl From<ExistingObjectID> for ObjectID {
    fn from(value: ExistingObjectID) -> Self {
        value.0
    }
}

impl ToSql for ObjectID {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl TryFrom<i64> for ObjectID {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < Self::MIN_ID.0 {
            Ok(ObjectID(value))
        } else {
            Err(())
        }
    }
}

impl ToSql for ExistingObjectID {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

/// A file ID that is guaranteed to exist during the wrapping transaction.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct ExistingObjectID(ObjectID);

impl ExistingObjectID {
    #[inline]
    pub const unsafe fn new(id: ObjectID) -> Self {
        Self(id)
    }
}

macro_rules! impl_sub_type_of_object {
    ($name:ident, $existing_name:ident) => {
        /// Something that might not exist.
        #[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
        pub struct $name(pub ObjectID);

        impl ToSql for $name {
            fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
                self.0.to_sql()
            }
        }

        /// Something that is guaranteed to exist during the wrapping transaction.
        #[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
        pub struct $existing_name($name);

        impl $existing_name {
            #[inline]
            pub const unsafe fn new(id: $name) -> Self {
                Self(id)
            }
        }

        impl From<$existing_name> for $name {
            fn from(value: $existing_name) -> Self {
                value.0
            }
        }

        impl ToSql for $existing_name {
            fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
                self.0.to_sql()
            }
        }
    };
}

impl_sub_type_of_object!(FileID, ExistingFileID);
impl_sub_type_of_object!(DirectoryID, ExistingDirectoryID);
impl_sub_type_of_object!(UserID, ExistingUserID);
impl_sub_type_of_object!(GroupID, ExistingGroupID);