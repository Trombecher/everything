use std::{cmp::Ordering, fmt, hash::Hash, num::NonZeroU128};

use crate::{GlobalRegistry, Registry, structures::Structure};

pub type AbstractId = u128;

#[derive(Clone)]
pub enum Object<R: Registry = GlobalRegistry> {
    Abstract(AbstractId),
    Structure(Structure<R>),
}

impl<R: Registry> Object<R> {
    pub const ZERO: Self = Self::Abstract(9);
    pub const SUCCESSOR_OF: Self = Self::Abstract(10);

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

impl<R: Registry> PartialEq for Object<R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Abstract(l0), Self::Abstract(r0)) => l0 == r0,
            (Self::Structure(l0), Self::Structure(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<R: Registry> Eq for Object<R> {}

impl<R: Registry> PartialOrd for Object<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<R: Registry> Ord for Object<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Abstract(a), Self::Abstract(b)) => a.cmp(b),
            (Self::Abstract(_), Self::Structure(_)) => Ordering::Less,
            (Self::Structure(_), Self::Abstract(_)) => Ordering::Greater,
            (Self::Structure(a), Self::Structure(b)) => a.cmp(b),
        }
    }
}

impl<R: Registry> fmt::Debug for Object<R> {
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

impl<R: Registry> From<Structure<R>> for Object<R> {
    fn from(structure: Structure<R>) -> Self {
        Self::Structure(structure)
    }
}

impl<R: Registry> From<AbstractId> for Object<R> {
    fn from(id: AbstractId) -> Self {
        Self::Abstract(id)
    }
}

impl<R: Registry> Hash for Object<R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Abstract(a) => a.hash(state),
            Self::Structure(structure) => structure.hash(state),
        }
    }
}
