use rusqlite::Connection;
use crate::constraints::Constraint;
use crate::db;
use crate::error::Error;
use crate::objects::{ObjectId, ValidatedUserId};
use crate::permissions::PermissionLevel;
use crate::proto::MessageBuffer;

pub enum IncomingMessage<'a> {
    Event(IncomingEvent),
    Call {
        id: u64,
        procedure: Procedure<'a>
    }
}

pub enum IncomingEvent {
    DeleteObject {
        id: ObjectId,
    }
}

pub enum Procedure<'a> {
    Query {
        constraint: Option<Constraint<'a>>,
        limit: u16,
        offset: u64,
    },
    CreateObject,
}

impl<'a> Procedure<'a> {
    pub fn call(
        self,
        conn: &Connection,
        user: ValidatedUserId,
        pl: PermissionLevel,
        mb: MessageBuffer
    ) -> Result<(), Error> {
        match self {
            Procedure::Query { constraint, limit, offset } => {
                db::objects::query(conn, constraint, limit, offset);
            }
        }
    }
}