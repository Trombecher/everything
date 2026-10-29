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
