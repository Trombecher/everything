mod any;
mod bin;
mod registry;
#[cfg(test)]
mod tests;
mod text;

use core::slice;
use std::{
    borrow::Cow,
    fmt::{Debug, Pointer},
    hash::Hash,
    mem::replace,
    num::NonZeroU128,
};

pub use any::*;
pub use bin::*;
pub use registry::*;
pub use text::*;

use crate::{Object, Property};

/// A structure is a set of properties. Natural numbers, text, binary data,
/// and the structure with no properties are stored more efficiently than an [AnyStructure].
/// These are called specializations.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Structure {
    /// The empty structure.
    Empty,
    /// Optimized storage for `{(@SUCCESSOR_OF, n)}` where n is an exact natural number.
    NaturalNumber(NonZeroU128),
    Binary(BlobStructure),
    Text(TextStructure),
    Any(AnyStructure),
}

impl Structure {
    #[must_use]
    pub fn new(properties: &mut [Property]) -> Self {
        Self::Empty.add(properties)
    }

    /// Adds the given properties to this structure.
    #[must_use]
    pub fn add(&self, properties: &mut [Property]) -> Self {
        self.change(&mut [], properties)
    }

    /// Removes the given properties from this structure.
    #[must_use]
    pub fn remove(&self, properties: &mut [Property]) -> Self {
        self.change(properties, &mut [])
    }

    /// Modifies this AnyStructure by adding and removing properties.
    /// Returns the modified AnyStructure.
    ///
    /// The properties need to be mutable because this method needs to
    /// reorder and dedup changes in-place to avoid unneccessary
    /// allocations.
    ///
    /// Note that first all indicated properties are removed from
    /// the AnyStructure and then all indicated properties are added.
    #[must_use]
    pub fn change(
        &self,
        remove_properties: &mut [Property],
        add_properties: &mut [Property],
    ) -> Structure {
        registry::resolve(self, remove_properties, add_properties)
    }

    pub fn any(&self) -> Option<&AnyStructure> {
        match self {
            Self::Any(any) => Some(any),
            _ => None,
        }
    }

    /// Returns an iterator over all properties:
    pub fn properties(&self) -> Properties {
        match self {
            Self::Empty => Properties::None,
            Self::NaturalNumber(n) => Properties::One(Property {
                tag: Object::SUCCESSOR_OF,
                value: Object::new_natural_number(n.get() - 1),
            }),
            Self::Binary(_) => todo!(),
            Self::Text(_) => todo!(),
            Self::Any(any_structure) => Properties::Any(any_structure.as_ref().iter()),
        }
    }

    /// Merges the properties of `self` and `other` into a new AnyStructure.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut add_properties = other
            .properties()
            .map(|property| property.into_owned())
            .collect::<Vec<_>>();

        self.add(add_properties.as_mut_slice())
    }
}

impl From<NonZeroU128> for Structure {
    fn from(value: NonZeroU128) -> Self {
        Self::NaturalNumber(value)
    }
}

impl From<AnyStructure> for Structure {
    fn from(value: AnyStructure) -> Self {
        Self::Any(value)
    }
}

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => <[(); 0] as std::fmt::Debug>::fmt(&[], f),
            Self::Any(any) => any.fmt(f),
            Self::NaturalNumber(n) => n.fmt(f),
            Self::Text(t) => t.fmt(f),
            Self::Binary(b) => b.fmt(f),
        }
    }
}

#[derive(Clone)]
pub enum Properties<'structure> {
    None,
    One(Property),
    Any(slice::Iter<'structure, Property>),
}

impl<'structure> Iterator for Properties<'structure> {
    type Item = Cow<'structure, Property>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::One(_) => match replace(self, Self::None) {
                Self::One(property) => Some(Cow::Owned(property)),
                _ => unreachable!(),
            },
            Self::Any(iter) => iter.next().map(Cow::Borrowed),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Any(iter) => iter.size_hint(),
            Self::None => (0, Some(0)),
            Self::One(_) => (1, Some(1)),
        }
    }
}
