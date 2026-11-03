mod any_set;
mod byte;
mod character;
mod integer;
mod one_construction;

#[cfg(test)]
mod tests;
mod text;

pub use any_set::*;
pub use byte::*;
pub use character::*;
pub use integer::*;
pub use one_construction::*;
pub use text::*;

use std::{fmt::Debug, mem};

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
    Text(TextSet),
    Any(AnySet),
}

impl SetApi for Set {
    fn size(&self) -> u64 {
        match self {
            Self::Integer(x) => x.size(),
            Self::Character(x) => x.size(),
            Self::Byte(x) => x.size(),
            Self::OneConstruction(x) => x.size(),
            Self::Any(x) => x.size(),
            Self::Text(x) => x.size(),
        }
    }

    fn has(&self, object: &Object) -> bool {
        match self {
            Self::Integer(x) => x.has(object),
            Self::Character(x) => x.has(object),
            Self::Byte(x) => x.has(object),
            Self::OneConstruction(x) => x.has(object),
            Self::Any(x) => x.has(object),
            Self::Text(x) => x.has(object),
        }
    }
}

impl Set {
    /// Returns the empty [`Set`].
    #[must_use]
    pub fn empty() -> Self {
        Self::from([])
    }

    /// Tries to create a new [`Set`] primitive specialization variant from the given list of objects.
    /// If no specialization matches, then this function returns [`None`].
    fn try_new_primitive_specialization(objects: &[Object]) -> Option<Self> {
        IntegerSet::specialize_from(objects)
            .map(Self::Integer)
            .or_else(|| CharacterSet::specialize_from(objects).map(Self::Character))
            .or_else(|| ByteSet::specialize_from(objects).map(Self::Byte))
    }

    /// Creates a set from objects that are already sorted and deduplicated.
    #[must_use]
    fn from_sorted(objects: &mut [Object]) -> Self {
        debug_assert!(
            objects.windows(2).all(|pair| pair[0] < pair[1]),
            "the values must be sorted and deduplicated"
        );

        if let Some(set) = Self::try_new_primitive_specialization(objects) {
            return set;
        }

        if let Some(set) = TextSet::specialize_from(objects) {
            return Self::Text(set);
        }

        if let Some(set) = OneConstructionSet::specialize_from_moving(objects) {
            return Self::OneConstruction(set);
        }

        Self::Any(AnySet::from_sorted(objects.iter_mut().map(mem::take)))
    }

    #[must_use]
    pub fn add_object(self, object: Object) -> Self {
        if self.has(&object) {
            return self;
        }

        let mut values = self.into_iter().collect::<Vec<_>>();

        if let Err(index) = values.binary_search(&object) {
            values.insert(index, object);
        }

        Self::from_sorted(values.as_mut_slice())
    }

    #[must_use]
    pub fn remove_object(self, object: &Object) -> Self {
        if !self.has(object) {
            return self;
        }

        let mut values = self.into_iter().collect::<Vec<_>>();

        if let Some(index) = values.iter().position(|value| value == object) {
            values.remove(index);
        }

        Self::from_sorted(values.as_mut_slice())
    }
}

impl<const N: usize> From<[Object; N]> for Set {
    fn from(mut objects: [Object; N]) -> Self {
        Self::from(objects.as_mut_slice())
    }
}

impl<'a> From<&'a mut [Object]> for Set {
    fn from(objects: &'a mut [Object]) -> Self {
        objects.sort_unstable();

        let (unique, _) = objects.partition_dedup();

        Self::from_sorted(unique)
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
            Self::Text(x) => Debug::fmt(x, f),
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
            Self::Text(x) => SetValues::Text(x.into_iter()),
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
    Text(TextSetValues),
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
            Self::Text(x) => x.next(),
        }
    }
}
