use std::{fmt::Debug, num::NonZeroI128};

mod atoms;
mod constructions;
mod iter;
mod sets;

pub use atoms::*;
pub use constructions::*;
pub use sets::*;

use crate::compute::{self, Node};

/// An object that represents a thing.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Atom(Atom),
    Construction(Construction),
    Set(Set),
}

impl Default for Object {
    fn default() -> Self {
        Self::dummy()
    }
}

impl Object {
    /// Returns an object that may be used as a dummy value.
    /// The output of this function should not be relied upon
    /// besides the fact that this function will not allocate.
    #[must_use]
    pub fn dummy() -> Self {
        Self::Atom(Atom(0))
    }

    #[must_use]
    pub const fn new_integer(integer: i128) -> Self {
        if let Some(non_zero_integer) = NonZeroI128::new(integer) {
            Self::Set(Set::Integer(IntegerSet {
                representing: non_zero_integer,
            }))
        } else {
            Self::Atom(Atom::ZERO)
        }
    }

    #[must_use]
    pub fn integer(&self) -> Option<i128> {
        match self {
            Self::Atom(Atom::ZERO) => Some(0),
            Self::Set(Set::Integer(integer)) => Some(integer.representing.get()),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Self::Set(set) if set.size() == 0)
    }

    /// Destructures the [`Self::Construction`] variant
    /// into an [`Option`].
    #[must_use]
    pub fn destructure_construction(self) -> Option<Construction> {
        match self {
            Object::Construction(construction) => Some(construction),
            _ => None,
        }
    }

    #[must_use]
    pub fn new_node(node: Node) -> Self {
        compute::new_node(node)
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => Debug::fmt(&atom, f),
            Self::Construction(construction) => Debug::fmt(&construction, f),
            Self::Set(set) => Debug::fmt(&set, f),
        }
    }
}

impl From<Atom> for Object {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

impl From<Construction> for Object {
    fn from(value: Construction) -> Self {
        Self::Construction(value)
    }
}

impl From<Set> for Object {
    fn from(value: Set) -> Self {
        Self::Set(value)
    }
}
