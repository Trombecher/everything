use crate::StatementPattern;

#[derive(Debug)]
pub enum Error {
    CouldNotProveTheorem(StatementPattern),
}
