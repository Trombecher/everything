mod any_set;
mod byte;
mod character;
mod integer;
mod one_construction;

pub use any_set::*;
pub use byte::*;
pub use character::*;
pub use integer::*;
pub use one_construction::*;

use std::fmt::Debug;

use crate::model::Object;

/// A helper trait.
pub trait SetApi: IntoIterator<Item = Object> + Sized + Debug + Clone + Ord
where
    Self::IntoIter: Clone,
{
    /// The number of objects in this set.
    fn size(&self) -> u64;

    fn has(&self, object: &Object) -> bool;

    #[doc(alias = "into_iter")]
    fn values(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub trait SetSpecialization:
    SetApi + IntoIterator<Item = Object> + Sized + Debug + Clone + Ord
where
    Self::IntoIter: Clone,
{
    fn specialize_from(objects: &[Object]) -> Option<Self>;
}

/// A set of objects. Contains specializations for certain sets
/// which are represented optimally in memory.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Set {
    Integer(IntegerSet),
    Character(CharacterSet),
    Byte(ByteSet),
    OneConstruction(OneConstructionSet),
    Any(AnySet),
}

impl SetApi for Set {
    fn size(&self) -> u64 {
        match self {
            Set::Integer(x) => x.size(),
            Set::Character(x) => x.size(),
            Set::Byte(x) => x.size(),
            Set::OneConstruction(x) => x.size(),
            Set::Any(x) => x.size(),
        }
    }

    fn has(&self, object: &Object) -> bool {
        match self {
            Set::Integer(x) => x.has(object),
            Set::Character(x) => x.has(object),
            Set::Byte(x) => x.has(object),
            Set::OneConstruction(x) => x.has(object),
            Set::Any(x) => x.has(object),
        }
    }
}

impl Set {
    /// Returns the empty [`Set`].
    #[must_use]
    pub fn empty() -> Self {
        Self::new_move([])
    }

    /// Creates a [`Set`] that is truthy.
    #[must_use]
    pub fn truthy() -> Self {
        Self::new_move([Set::empty().into()])
    }

    /// Creates a new [`Set`] from the given slice of objects. The objects will be cloned.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(objects: &[Object]) -> Self {
        Self::try_new_primitive_specialization(objects)
            .unwrap_or_else(|| Self::Any(AnySet::new(objects)))
    }

    /// Creates a new [`Set`] from the given array of objects.
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // this function does not panic!
    pub fn new_move<const N: usize>(objects: [Object; N]) -> Self {
        Self::try_new_primitive_specialization(&objects)
            .or_else(|| OneConstructionSet::specialize_from(&objects).map(Self::OneConstruction))
            .unwrap_or_else(|| Self::Any(AnySet::new_move(objects)))
    }

    /// Tries to create a new [`Set`] primitive specialization variant from the given list of objects.
    /// If no specialization matches, then this function returns [`None`].
    fn try_new_primitive_specialization(objects: &[Object]) -> Option<Self> {
        IntegerSet::specialize_from(objects)
            .map(Self::Integer)
            .or_else(|| CharacterSet::specialize_from(objects).map(Self::Character))
            .or_else(|| ByteSet::specialize_from(objects).map(Self::Byte))
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(x) => Debug::fmt(x, f),
            Self::Any(x) => Debug::fmt(x, f),
            Self::Character(x) => Debug::fmt(x, f),
            Self::Byte(x) => Debug::fmt(x, f),
            Self::OneConstruction(x) => Debug::fmt(x, f),
        }
    }
}

impl IntoIterator for Set {
    type Item = Object;
    type IntoIter = SetValues;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Integer(x) => SetValues::Integer(x.into_iter()),
            Self::Any(x) => SetValues::Any(x.into_iter()),
            Self::Character(x) => SetValues::Character(x.into_iter()),
            Self::Byte(x) => SetValues::Byte(x.into_iter()),
            Self::OneConstruction(x) => SetValues::OneConstruction(x.into_iter()),
        }
    }
}

/// An iterator over objects ("values") of a set object.
#[derive(Clone)]
pub enum SetValues {
    Integer(IntegerSetValues),
    Character(CharacterSetValues),
    Byte(ByteSetValues),
    OneConstruction(OneConstructionSetValues),
    Any(AnySetValues),
}

impl Iterator for SetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Character(x) => x.next(),
            Self::Integer(x) => x.next(),
            Self::Any(x) => x.next(),
            Self::Byte(x) => x.next(),
            Self::OneConstruction(x) => x.next(),
        }
    }
}
