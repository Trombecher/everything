use std::fmt;

use ulid::Ulid;

use crate::structures::Structure;

type AbstractId = Ulid;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Id {
    Abstract(AbstractId),
    Structure(Structure),
}

impl Id {
    pub const CONTAINS: Self = Self::Abstract(Ulid(1));
    pub const AXIOMATIC: Self = Self::Abstract(Ulid(2));
    pub const COMPUTED: Self = Self::Abstract(Ulid(3));
    pub const STATEMENT_SUBJECT: Self = Self::Abstract(Ulid(4));
    pub const STATEMENT_TAG: Self = Self::Abstract(Ulid(5));
    pub const STATEMENT_VALUE: Self = Self::Abstract(Ulid(6));
    pub const STATEMENT: Self = Self::Abstract(Ulid(7));
    pub const KNOWLEDGE: Self = Self::Abstract(Ulid(8));
    pub const ZERO: Self = Self::Abstract(Ulid(9));
    pub const SUCESSOR_OF: Self = Self::Abstract(Ulid(10));
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abstract(id) => {
                f.write_str("@")?;
                id.0.fmt(f)
            }
            Self::Structure(s) => s.fmt(f),
        }
    }
}

impl From<Structure> for Id {
    fn from(structure: Structure) -> Self {
        Self::Structure(structure)
    }
}

impl From<AbstractId> for Id {
    fn from(id: AbstractId) -> Self {
        Self::Abstract(id)
    }
}
