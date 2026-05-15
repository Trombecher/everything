#[cfg(test)]
mod tests;

use std::{hash::Hash, num::NonZeroI128};

use crate::{Abstract, structures::Structure};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Abstract(Abstract),
    Structure(Structure),
}

impl Object {
    /// Creates an integer object. This will yield a structure
    /// iff the given integer is non-zero; [`Abstract::ZERO`] else.
    #[must_use]
    pub const fn new_integer(integer: i128) -> Self {
        match NonZeroI128::new(integer) {
            None => Self::Abstract(Abstract::ZERO),
            Some(n) => Self::Structure(Structure::Integer(n)),
        }
    }

    /// Tries to extract the integer value out of an object
    /// iff the object is an exact integer value.
    #[must_use]
    pub const fn exact_integer(&self) -> Option<i128> {
        if let Self::Abstract(Abstract::ZERO) = self {
            Some(0)
        } else if let Self::Structure(Structure::Integer(n)) = self {
            Some(n.get())
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abstract(id) => id.fmt(f),
            Self::Structure(s) => s.fmt(f),
        }
    }
}

impl From<Abstract> for Object {
    fn from(value: Abstract) -> Self {
        Self::Abstract(value)
    }
}

impl From<Structure> for Object {
    fn from(structure: Structure) -> Self {
        Self::Structure(structure)
    }
}
