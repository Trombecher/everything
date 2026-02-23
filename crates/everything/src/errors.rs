use crate::StatementPattern;

pub enum Error {
    CouldNotProveTheorem(StatementPattern),
}
