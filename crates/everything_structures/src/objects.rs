use std::fmt;

use crate::structures::Structure;

pub type AbstractId = u128;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Abstract(AbstractId),
    Structure(Structure),
}

impl Object {}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abstract(id) => {
                f.write_str("@")?;
                id.fmt(f)
            }
            Self::Structure(s) => s.fmt(f),
        }
    }
}

impl From<Structure> for Object {
    fn from(structure: Structure) -> Self {
        Self::Structure(structure)
    }
}

impl From<AbstractId> for Object {
    fn from(id: AbstractId) -> Self {
        Self::Abstract(id)
    }
}
