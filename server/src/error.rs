//! This module contains error enum definitions.

/// The crate-level error enum.
pub enum Error {
    Internal(InternalError),
    Interface(InterfaceError),
}

/// This enum describes error that are not supposed to happen.
/// Any occurrence of this error is a bug in the server.
///
/// A 500 internal server should be returned to the client.
#[non_exhaustive]
pub enum InternalError {
    /// An error occurred in the database library.
    Database(rusqlite::Error),

    /// This error occurs when an upsert returned zero affected rows.
    UpsertYieldedZeroAffectedRows,

    /// This error occurs when a new file is inserted and the resulting rowid is not
    /// a valid file id.
    ///
    /// This error may indicate that the sqlite_sequence table is misconfigured.
    AutoFileIDIsNotValid,
}

impl InternalError {
    /// Some internal errors may be fixed by running a [crate::db::fix].
    pub const fn is_fix_recommended(&self) -> bool {
        match self {
            InternalError::Database(_) => false,
            InternalError::UpsertYieldedZeroAffectedRows => false,
            InternalError::AutoFileIDIsNotValid => true,
        }
    }
}

impl From<InternalError> for Error {
    #[inline]
    fn from(value: InternalError) -> Self {
        Self::Internal(value)
    }
}

impl From<rusqlite::Error> for InternalError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Database(value)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self::Internal(value.into())
    }
}

/// The errors from this enum commonly arise from interfacing with users.
/// This includes errors like validation errors, non-existing objects,
/// message format errors, bounds errors, login errors, etc.
#[non_exhaustive]
pub enum InterfaceError {
    ObjectDoesNotExist,
    CannotAssociateObjectWithInferredType,
    TypeMismatch,
    InvalidEventID,
    InvalidProcedureID,
    MalformedMessage,
}

impl From<InterfaceError> for Error {
    fn from(value: InterfaceError) -> Self {
        Self::Interface(value)
    }
}