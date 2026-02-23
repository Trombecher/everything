use crate::FactPattern;

pub enum Error {
    CouldNotProveTheorem(FactPattern),
}
