use std::fmt::Debug;

mod atoms;
mod composites;
mod constructions;
mod non_zero_bigint;
#[cfg(test)]
mod tests;

pub use atoms::*;
pub use composites::*;
pub use constructions::*;
pub use non_zero_bigint::*;
use num_bigint::BigInt;

use crate::compute::Node;

/// An object that represents a thing.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Atom(Atom),
    Construction(Construction),
    Composite(Composite),
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
    pub const fn dummy() -> Self {
        Self::Atom(Atom(0))
    }

    #[must_use]
    pub fn new_integer(integer: BigInt) -> Self {
        if let Some(non_zero_integer) = NonZeroBigInt::new(integer) {
            Self::Composite(Composite::Integer(IntegerComposite {
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
            Self::Composite(Composite::Integer(integer)) => {
                Some(integer.representing.get_as_ref().clone())
            }
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
        node.into()
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => Debug::fmt(&atom, f),
            Self::Construction(construction) => Debug::fmt(&construction, f),
            Self::Composite(composite) => Debug::fmt(&composite, f),
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

impl From<Composite> for Object {
    fn from(value: Composite) -> Self {
        Self::Composite(value)
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

/// An [`Object`] that is not a construction.
///
/// Atoms and composites have properties and are sets; constructions have
/// neither. APIs that query properties or set items take this type instead of
/// [`Object`], so that passing a construction is rejected at compile time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtomOrComposite {
    Atom(Atom),
    Composite(Composite),
}

impl TryFrom<Object> for AtomOrComposite {
    type Error = Construction;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        match object {
            Object::Atom(atom) => Ok(Self::Atom(atom)),
            Object::Composite(composite) => Ok(Self::Composite(composite)),
            Object::Construction(construction) => Err(construction),
        }
    }
}

impl TryFrom<&Object> for AtomOrComposite {
    type Error = Construction;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match object {
            Object::Atom(atom) => Ok(Self::Atom(*atom)),
            Object::Composite(composite) => Ok(Self::Composite(composite.clone())),
            Object::Construction(construction) => Err(construction.clone()),
        }
    }
}

impl From<AtomOrComposite> for Object {
    fn from(value: AtomOrComposite) -> Self {
        match value {
            AtomOrComposite::Atom(atom) => Self::Atom(atom),
            AtomOrComposite::Composite(composite) => Self::Composite(composite),
        }
    }
}

impl From<Atom> for AtomOrComposite {
    fn from(atom: Atom) -> Self {
        Self::Atom(atom)
    }
}

impl From<Composite> for AtomOrComposite {
    fn from(composite: Composite) -> Self {
        Self::Composite(composite)
    }
}
