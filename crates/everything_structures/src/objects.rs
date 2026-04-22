use std::{hash::Hash, num::NonZeroU128};

use crate::{Abstract, structures::Structure};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Abstract(Abstract),
    Structure(Structure),
}

impl Object {
    #[must_use]
    pub const fn new_natural_number(n: u128) -> Self {
        match NonZeroU128::new(n) {
            None => Self::Abstract(Abstract::ZERO),
            Some(n) => Self::Structure(Structure::NaturalNumber(n)),
        }
    }

    #[must_use]
    pub const fn exact_natural_number(&self) -> Option<u128> {
        if let Self::Abstract(Abstract::ZERO) = self {
            Some(0)
        } else if let Self::Structure(Structure::NaturalNumber(n)) = self {
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
