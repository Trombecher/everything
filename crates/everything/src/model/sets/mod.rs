mod any_set;
mod byte;
mod character;
mod integer;
mod one_construction;

#[cfg(test)]
mod tests;

pub use any_set::*;
pub use byte::*;
pub use character::*;
pub use integer::*;
pub use one_construction::*;

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

    #[must_use]
    pub fn add_object(self, object: Object) -> Self {
        let mut new_values = self.into_iter().collect::<Vec<_>>();
        new_values.push(object);
        Self::from(new_values.as_mut_slice())
    }

    #[must_use]
    pub fn remove_object(self, object: &Object) -> Self {
        let mut new_values = self
            .into_iter()
            .filter(|value| value != object)
            .collect::<Vec<_>>();

        Self::from(new_values.as_mut_slice())
    }
}

impl<const N: usize> From<[Object; N]> for Set {
    fn from(mut objects: [Object; N]) -> Self {
        Self::from(objects.as_mut_slice())
    }
}

impl<'a> From<&'a mut [Object]> for Set {
    fn from(objects: &'a mut [Object]) -> Self {
        let unique_len = {
            objects.sort_unstable();

            let (unique, _) = objects.partition_dedup();
            let unique_len = unique.len();

            if let Some(set) = Self::try_new_primitive_specialization(unique) {
                return set;
            }

            if let Some(set) = OneConstructionSet::specialize_from_moving(unique) {
                return Self::OneConstruction(set);
            }

            unique_len
        };

        Self::Any(AnySet::from_sorted(
            objects.iter_mut().map(mem::take).take(unique_len),
        ))
    }
}

/*
impl<'a> FromIterator<&'a Object> for Set {
    fn from_iter<I: IntoIterator<Item = &'a Object>>(iter: I) -> Self {
        Self::from_iter(iter.into_iter().cloned())
    }
}

impl FromIterator<Object> for Set {
    fn from_iter<I: IntoIterator<Item = Object>>(objects: I) -> Self {
        let mut objects = objects.into_iter().collect::<Vec<_>>();
        objects.sort_unstable();

        let (unique, _) = objects.partition_dedup();

        if let Some(set) = Self::try_new_primitive_specialization(unique) {
            return set;
        }

        if let Some(set) = OneConstructionSet::specialize_from(unique) {
            return Self::OneConstruction(set);
        }

        Self::Any(AnySet::from_sorted(unique.iter().cloned()))
    }
}
 */

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
