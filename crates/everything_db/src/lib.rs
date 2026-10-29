#![deny(clippy::pedantic)]

use everything::knowledge::Statement;

pub mod db;
pub mod format;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StatementChange<'a> {
    Add(&'a Statement),
    Remove(&'a Statement),
}
