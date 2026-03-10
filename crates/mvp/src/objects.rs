use std::fmt;

use crate::structures::Structure;

type AbstractId = u64;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Id {
    Abstract(AbstractId),
    Structure(Structure),
}

impl Id {
    pub const CONTAINS: Self = Self::Abstract(1);
    pub const AXIOMATIC: Self = Self::Abstract(2);
    pub const COMPUTED: Self = Self::Abstract(3);
    pub const STATEMENT_SUBJECT: Self = Self::Abstract(4);
    pub const STATEMENT_TAG: Self = Self::Abstract(5);
    pub const STATEMENT_VALUE: Self = Self::Abstract(6);
    pub const STATEMENT: Self = Self::Abstract(7);
    pub const KNOWLEDGE: Self = Self::Abstract(8);
}

impl fmt::Debug for Id {
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
