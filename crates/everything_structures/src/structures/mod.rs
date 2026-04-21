mod any;
mod bin;
mod registries;
#[cfg(test)]
mod tests;
mod text;

use core::slice;
use std::{
    borrow::Cow,
    fmt::{Debug, Pointer},
    hash::Hash,
    num::NonZeroU128,
};

pub use any::*;
pub use bin::*;
pub use registries::*;
pub use text::*;

use crate::{Object, Property};

/// A structure is a set of properties. Natural numbers, text, and binary data
/// are stored more efficiently than an [AnyStructure].
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Structure {
    /// Optimized storage for `{(@SUCCESSOR_OF, n)}` where n is an exact natural number.
    NaturalNumber(NonZeroU128),
    Binary(BlobStructure),
    Text(TextStructure),
    Any(AnyStructure),
}

impl Structure {
    pub const EMPTY: Self = Self::Any(AnyStructure { properties: None });

    #[must_use]
    pub fn new(properties: &mut [Property]) -> Self {
        Self::EMPTY.add(properties)
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
        /*
        match (self, &remove_properties, &add_properties) {
            (_, [], []) => self.clone(),
            (
                Self::NaturalNumber(_),
                [],
                [
                    Property {
                        tag: Object::SUCCESSOR_OF,
                        value: o,
                    },
                ],
            ) if let Some(new_n) = o.exact_natural_number() => {
                Self::NaturalNumber(NonZeroU128::new(new_n.checked_add(1).unwrap()).unwrap())
            }
            _ => todo!(),
        };
         */

        GlobalRegistry.resolve(self, remove_properties, add_properties)
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
            Structure::NaturalNumber(n) => Properties::NaturalNumber(Some(*n)),
            Structure::Binary(_) => todo!(),
            Structure::Text(_) => todo!(),
            Structure::Any(any_structure) => Properties::Any(any_structure.as_ref().iter()),
        }
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
            Self::Any(any) => any.fmt(f),
            Self::NaturalNumber(n) => n.fmt(f),
            Self::Text(t) => t.fmt(f),
            Self::Binary(b) => b.fmt(f),
        }
    }
}

pub enum Properties<'structure> {
    Any(slice::Iter<'structure, Property>),
    NaturalNumber(Option<NonZeroU128>),
}

impl<'structure> Iterator for Properties<'structure> {
    type Item = Cow<'structure, Property>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Properties::Any(iter) => iter.next().map(Cow::Borrowed),
            Properties::NaturalNumber(property) => property.take().map(|n| {
                Cow::Owned(Property {
                    tag: Object::SUCCESSOR_OF,
                    value: Object::new_natural_number(n.get() - 1),
                })
            }),
        }
    }
}
