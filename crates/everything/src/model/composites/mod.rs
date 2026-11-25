mod any_composite;
mod byte;
mod character;
mod integer;
mod one_construction;
mod set;

#[cfg(test)]
mod tests;
mod text;

pub use any_composite::*;
pub use byte::*;
pub use character::*;
pub use integer::*;
pub use one_construction::*;
pub use set::*;
pub use text::*;

use std::{fmt::Debug, mem};

use crate::model::Object;

use self::set::contains_member;

/// A helper trait.
pub trait CompositeApi: IntoIterator<Item = Object> + Sized + Debug + Clone + Ord
where
    Self::IntoIter: Clone,
{
    /// The number of objects in this composite.
    fn size(&self) -> u64;

    fn has(&self, object: &Object) -> bool;

    #[doc(alias = "into_iter")]
    fn values(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub trait CompositeSpecialization:
    CompositeApi + IntoIterator<Item = Object> + Sized + Debug + Clone + Ord
where
    Self::IntoIter: Clone,
{
    fn specialize_from(objects: &[Object]) -> Option<Self>;
}

/// A composite of objects. Contains specializations for certain composites
/// which are represented optimally in memory.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Composite {
    Integer(IntegerComposite),
    Character(CharacterComposite),
    Byte(ByteComposite),
    OneConstruction(OneConstructionComposite),
    Text(TextComposite),
    Set(SetComposite),
    Any(AnyComposite),
}

impl CompositeApi for Composite {
    fn size(&self) -> u64 {
        match self {
            Self::Integer(x) => x.size(),
            Self::Character(x) => x.size(),
            Self::Byte(x) => x.size(),
            Self::OneConstruction(x) => x.size(),
            Self::Any(x) => x.size(),
            Self::Text(x) => x.size(),
            Self::Set(x) => x.size(),
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
            Self::Set(x) => x.has(object),
        }
    }
}

impl Composite {
    /// Returns the empty [`Composite`].
    #[must_use]
    pub fn empty() -> Self {
        Self::from([])
    }

    /// Creates a composite that is the set with the given `items`, as defined
    /// in chapter 4 of the common models.
    ///
    /// Every item becomes a `contains ·· %item%` member (see [`SetComposite`]),
    /// so the result has set semantics regardless of which specialization
    /// represents it. The items are cloned; use
    /// [`Self::from_set_items_array`] to move them instead.
    #[must_use]
    pub fn from_set_items(items: &[Object]) -> Self {
        let mut items = items.to_vec();

        items.sort_unstable();
        items.dedup();

        let size = items.len();

        Self::from_sorted_set_items(items, size)
    }

    /// Creates a composite that is the set with the given `items`, moving them.
    ///
    /// This is the moving counterpart of [`Self::from_set_items`] for a number
    /// of items that is known at compile time.
    #[must_use]
    pub fn from_set_items_array<const N: usize>(mut items: [Object; N]) -> Self {
        items.sort_unstable();

        let size = items.partition_dedup().0.len();

        Self::from_sorted_set_items(items.into_iter().take(size), size)
    }

    /// Creates the canonical composite for `size` sorted and deduplicated set
    /// items.
    ///
    /// This specializes directly instead of building intermediate `contains`
    /// constructions and running the general specialization chain.
    fn from_sorted_set_items(items: impl IntoIterator<Item = Object>, size: usize) -> Self {
        match size {
            0 => Self::empty(),
            1 => Self::OneConstruction(OneConstructionComposite::new(contains_member(
                items
                    .into_iter()
                    .next()
                    .expect("the set has exactly one item"),
            ))),
            _ => Self::Set(SetComposite::from_sorted_items(items)),
        }
    }

    /// Tries to create a new [`Composite`] primitive specialization variant from the given list of objects.
    /// If no specialization matches, then this function returns [`None`].
    fn try_new_primitive_specialization(objects: &[Object]) -> Option<Self> {
        IntegerComposite::specialize_from(objects)
            .map(Self::Integer)
            .or_else(|| CharacterComposite::specialize_from(objects).map(Self::Character))
            .or_else(|| ByteComposite::specialize_from(objects).map(Self::Byte))
    }

    /// Creates a composite from objects that are already sorted and deduplicated.
    #[must_use]
    fn from_sorted(objects: &mut [Object]) -> Self {
        debug_assert!(
            objects.windows(2).all(|pair| pair[0] < pair[1]),
            "the values must be sorted and deduplicated"
        );

        if let Some(composite) = Self::try_new_primitive_specialization(objects) {
            return composite;
        }

        if let Some(composite) = TextComposite::specialize_from(objects) {
            return Self::Text(composite);
        }

        if let Some(composite) = OneConstructionComposite::specialize_from_moving(objects) {
            return Self::OneConstruction(composite);
        }

        if let Some(composite) = SetComposite::specialize_from(objects) {
            return Self::Set(composite);
        }

        Self::Any(AnyComposite::from_sorted(objects.iter_mut().map(mem::take)))
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

impl<const N: usize> From<[Object; N]> for Composite {
    fn from(mut objects: [Object; N]) -> Self {
        Self::from(objects.as_mut_slice())
    }
}

impl<'a> From<&'a mut [Object]> for Composite {
    fn from(objects: &'a mut [Object]) -> Self {
        objects.sort_unstable();

        let (unique, _) = objects.partition_dedup();

        Self::from_sorted(unique)
    }
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(x) => Debug::fmt(x, f),
            Self::Any(x) => Debug::fmt(x, f),
            Self::Character(x) => Debug::fmt(x, f),
            Self::Byte(x) => Debug::fmt(x, f),
            Self::OneConstruction(x) => Debug::fmt(x, f),
            Self::Text(x) => Debug::fmt(x, f),
            Self::Set(x) => Debug::fmt(x, f),
        }
    }
}

impl IntoIterator for Composite {
    type Item = Object;
    type IntoIter = CompositeValues;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Integer(x) => CompositeValues::Integer(x.into_iter()),
            Self::Any(x) => CompositeValues::Any(x.into_iter()),
            Self::Character(x) => CompositeValues::Character(x.into_iter()),
            Self::Byte(x) => CompositeValues::Byte(x.into_iter()),
            Self::OneConstruction(x) => CompositeValues::OneConstruction(x.into_iter()),
            Self::Text(x) => CompositeValues::Text(x.into_iter()),
            Self::Set(x) => CompositeValues::Set(x.into_iter()),
        }
    }
}

/// An iterator over objects ("values") of a composite object.
#[derive(Clone)]
pub enum CompositeValues {
    Integer(IntegerCompositeValues),
    Character(CharacterCompositeValues),
    Byte(ByteCompositeValues),
    OneConstruction(OneConstructionCompositeValues),
    Any(AnyCompositeValues),
    Text(TextCompositeValues),
    Set(SetCompositeValues),
}

impl Iterator for CompositeValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Character(x) => x.next(),
            Self::Integer(x) => x.next(),
            Self::Any(x) => x.next(),
            Self::Byte(x) => x.next(),
            Self::OneConstruction(x) => x.next(),
            Self::Text(x) => x.next(),
            Self::Set(x) => x.next(),
        }
    }
}
