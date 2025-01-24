use rusqlite::{params, Connection};
use crate::cache::IAC;
use crate::db;
use crate::error::{Error, InterfaceError, InternalError};
use crate::objects::{ExistingObjectID, FileID, ObjectID};
use crate::permissions::PermissionLevel;
use crate::types::{BuiltInTypeID, TypeID};
use crate::values::{DateTime, GenericValue};

pub async fn associate<'client_message>(
    conn: &Connection,
    iac: &IAC,
    value: GenericValue<'client_message>,
    permission_level: PermissionLevel,
    object_id: ObjectID,
    type_id: TypeID,
) -> Result<(), Error> {
    macro_rules! extract {
        ($item:path) => {
            match value {
                $item(x) => x,
                _ => return Err(Error::Interface(InterfaceError::TypeMismatch)),
            }
        };
    }

    let object_id: ExistingObjectID = db::objects::exists(conn, object_id)
        .map_err(Error::from)?
        .unwrap();

    match type_id {
        TypeID::BuiltIn(b) if b.is_inferred() =>
            return Err(Error::Interface(InterfaceError::CannotAssociateObjectWithInferredType)),
        TypeID::BuiltIn(BuiltInTypeID::Content) => {
            let content = extract!(GenericValue::Binary);

            // Update associations table "a2B", 0x2B = 43 = Content(Binary).
            match conn.prepare_cached("INSERT INTO a2B (id, _) VALUES (?, ?) ON CONFLICT (id) DO UPDATE SET _ = ?")?
                .insert(params![object_id, content, content]) {
                Ok(0) => Ok(()),
                Ok(_) => Err(Error::Internal(InternalError::UpsertYieldedZeroAffectedRows)),
                Err(e) => Err(Error::Internal(InternalError::Database(e)))
            }?;

            set_last_written(conn, object_id, DateTime::now())?;

            // Cache invalidation
            iac.sha256.lock().await.remove(&object_id.into());
            iac.invalidate_image(FileID(object_id.into())).await;
        }
        TypeID::BuiltIn(BuiltInTypeID::LastWritten) => {
            let date_time = match value {
                GenericValue::DateTime(date_time) => date_time,
                _ => return Err(Error::Interface(InterfaceError::TypeMismatch)),
            };

            set_last_written(conn, object_id, date_time)?;
        }
        TypeID::BuiltIn(BuiltInTypeID::Author) => {
            let author = extract!(GenericValue::UserID);

            // Update associations table "a29", 0x29 = 41 = Author(UserID).
            match conn.prepare_cached("INSERT INTO a29 (id, _) VALUES (?, ?) ON CONFLICT (id) DO UPDATE SET _ = ?")?
                .insert(params![object_id, author, author]) {
                Ok(0) => Ok(()),
                Ok(_) => Err(Error::Internal(InternalError::UpsertYieldedZeroAffectedRows)),
                Err(e) => Err(Error::Internal(InternalError::Database(e)))
            }?;
        }
        _ => todo!()
    }

    Ok(())
}

/// Associates an object with a "last written" datetime.
///
/// This is internal function for de-duplication of code.
fn set_last_written(
    conn: &Connection,
    object_id: ExistingObjectID,
    date_time: DateTime
) -> Result<(), Error> {
    // Update associations table "a21", 0x21 = 33 = LastWritten(DateTime).
    match conn
        .prepare_cached("INSERT INTO a21 (id, _) VALUES (?, ?) ON CONFLICT DO UPDATE SET _ = ?")
        .map_err(|e| Error::Internal(InternalError::Database(e)))?
        .insert(params![object_id, date_time, date_time]) {
        Ok(0) => Ok(()),
        Ok(_) => Err(Error::Internal(InternalError::UpsertYieldedZeroAffectedRows)),
        Err(e) => Err(Error::Internal(InternalError::Database(e)))
    }
}