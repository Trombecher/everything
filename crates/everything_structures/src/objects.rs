use std::{fmt, hash::Hash, num::NonZeroU128};

use crate::structures::Structure;

pub type AbstractId = u128;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Abstract(AbstractId),
    Structure(Structure),
}

impl Object {
    /// The abstract object 0.
    pub const ZERO: Self = Self::Abstract(9);

    /// Denotes that the current object is a successor of some child number.
    pub const SUCCESSOR_OF: Self = Self::Abstract(10);

    /// The empty list.
    pub const EMPTY_LIST: Self = Self::Abstract(2312);

    /// The slot for the item value in an list.
    pub const LIST_ITEM: Self = Self::Abstract(5347);

    /// Denotes the rest of the list.
    pub const LIST_TAIL: Self = Self::Abstract(4353);

    pub fn new_natural_number(n: u128) -> Self {
        match NonZeroU128::new(n) {
            None => Self::ZERO,
            Some(n) => Self::Structure(Structure::NaturalNumber(n)),
        }
    }

    pub fn exact_natural_number(&self) -> Option<u128> {
        if self == &Self::ZERO {
            Some(0)
        } else if let Self::Structure(s) = self {
            s.exact_natural_number().map(Into::into)
        } else {
            None
        }
    }
}

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
