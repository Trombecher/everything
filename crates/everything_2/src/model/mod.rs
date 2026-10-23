use std::fmt::Debug;

mod atoms;
mod constructions;
mod non_zero_bigint;
mod sets;

pub use atoms::*;
pub use constructions::*;
pub use non_zero_bigint::*;
use num_bigint::BigInt;
pub use sets::*;

use crate::compute::Node;

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
    pub fn new_integer(integer: BigInt) -> Self {
        if let Some(non_zero_integer) = NonZeroBigInt::new(integer) {
            Self::Set(Set::Integer(IntegerSet {
                representing: non_zero_integer,
            }))
        } else {
            Self::Atom(Atom::ZERO)
        }
    }

    #[must_use]
    pub fn integer(&self) -> Option<BigInt> {
        match self {
            Self::Atom(Atom::ZERO) => Some(BigInt::ZERO),
            Self::Set(Set::Integer(integer)) => Some(integer.representing.get_as_ref().clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn truthy(&self) -> Option<bool> {
        match self {
            Self::Atom(atom) => bool::try_from(*atom).ok(),
            _ => None,
        }
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
        node.into_object()
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

impl From<BigInt> for Object {
    fn from(value: BigInt) -> Self {
        Self::new_integer(value)
    }
}

impl From<bool> for Object {
    fn from(truth: bool) -> Self {
        Atom::from(truth).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn size_of_object_is_good() {
        assert!(size_of::<Object>() <= 32);
    }
}
