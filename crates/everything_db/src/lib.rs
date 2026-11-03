//! Persistence for the Everything knowledge model.
//!
//! A knowledge base is stored in a single file whose format is specified in
//! `docs/spec/File Format v1.md`.

#![deny(clippy::pedantic)]

pub use db::*;
use everything::knowledge::Statement;

pub mod db;
mod format;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StatementChange<'a> {
    Add(&'a Statement),
    Remove(&'a Statement),
}
