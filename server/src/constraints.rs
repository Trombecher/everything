//! This module describes constraints in the data model.

use std::fmt::Write;
use crate::error::Error;
use crate::tags::TagId;
use crate::values::Value;
use rusqlite::Connection;

/// When annotating a field to be of type `Object`, one can constrain
/// the object in terms of tags. The following operations are available:
///
/// - `+` "and", binary operation.
/// - `!` "not", unary operation. Specifies that the contained
/// expression should be negated.
/// - `|` "or", binary operation.
///
/// Grouping is available via `(` and `)`.
pub enum Constraint<'a> {
    And(Box<Constraint<'a>>, Box<Constraint<'a>>),
    Or(Box<Constraint<'a>>, Box<Constraint<'a>>),
    Not(Box<Constraint<'a>>),
    Tag {
        id: TagId,
        match_value: Option<Value<'a>>,
    },
}

impl<'a> Constraint<'a> {
    pub(crate) fn build_query(
        &self,
        conn: &Connection,
        out: &mut String,
        params: &mut Vec<Value<'a>>
    ) -> Result<(), Error> {
        match self {
            Constraint::And(a, b) => {
                out.push_str("SELECT id FROM (");
                a.build_query(conn, out, params)?;
                out.push_str(" INTERSECT ");
                b.build_query(conn, out, params)?;
                out.push(')');
            }
            Constraint::Or(a, b) => {
                out.push_str("SELECT id FROM (");
                a.build_query(conn, out, params)?;
                out.push_str(" UNION ");
                b.build_query(conn, out, params)?;
                out.push(')');
            }
            Constraint::Not(con) => {
                out.push_str("SELECT id FROM objects EXCEPT ");
                con.build_query(conn, out, params)?;
            }
            Constraint::Tag {
                id,
                match_value
            } => {
                let id: i64 = (*id).into();
                write!(out, "SELECT id FROM a{id:X}").unwrap();
                
                // TODO: validate id and schema
                
                if let Some(value) = match_value {
                    params.push(*value);
                    write!(out, " WHERE _ = ?{}", params.len()).unwrap();
                }
            }
        }

        Ok(())
    }
}