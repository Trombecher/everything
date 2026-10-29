#![deny(clippy::pedantic)]

use everything::knowledge::Statement;

pub mod format;

pub enum StatementChange<'a> {
    Add(&'a Statement),
    Remove(&'a Statement),
}
