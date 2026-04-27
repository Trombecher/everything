mod any;
mod byte;
mod bytes;
mod registry;
#[cfg(test)]
mod tests;
mod text;

use std::{fmt::Debug, hash::Hash, num::NonZeroU128};

pub use any::*;
pub use byte::*;
pub use bytes::*;
pub use text::*;

use crate::{Abstract, Object, Property};

/// A structure is a set of [`Property`]s. Natural numbers, text, binary data,
/// and the structure with no properties are stored more efficiently than an
/// [`AnyStructure`]. These are called specializations.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Structure {
    /// The empty structure `{}`.
    Empty,

    /// A natural number of this form `{(@SUCCESSOR_OF, <NaturalNumber> | @ZERO)}`
    /// where n is an exact natural number.
    NaturalNumber(NonZeroU128),

    /// A list of bytes.
    ///
    /// ```plain
    /// {
    ///     (@LIST_ITEM, <byte>),
    ///     (@LIST_ITEM, <binary>)
    /// }
    /// ```
    Bytes(BytesStructure),

    /// A list of characters.
    ///
    /// ```plain
    /// {
    ///     (@LIST_ITEM, <char>),
    ///     (@LIST_TAIL, <text>)
    /// }
    /// ```
    Text(TextStructure),

    /// A byte is specialized storage for eight slots for bits.
    /// The LSB corresponds to slot 0.
    ///
    /// ```plain
    /// {
    ///     (@BIT_SLOT_0, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_1, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_2, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_3, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_4, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_5, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_6, @BIT_0 | @BIT_1),
    ///     (@BIT_SLOT_7, @BIT_0 | @BIT_1)
    /// }
    /// ```
    Byte(Byte),

    /// A character. It is of this form: `{(@CODE_POINT, n)}` where n is a natural number
    /// which is at most 0x10FFFF and not in the range 0xD800 (including) to 0xDFFF (excluding).
    Character(char),

    /// Any structure, a structure that is not a specialization.
    Any(AnyStructure),
}

impl Structure {
    /// Creates a structure from the given properties.
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

    /// Returns `Some(_)` if `self` is an [AnyStructure];
    /// else `None`.
    pub fn any(&self) -> Option<&AnyStructure> {
        match self {
            Self::Any(any) => Some(any),
            _ => None,
        }
    }

    /// Returns an iterator over all properties of `self`.
    pub fn properties<'structure>(&'structure self) -> StructureProperties<'structure> {
        match self {
            Self::Empty => StructureProperties::Empty,
            Self::NaturalNumber(n) => StructureProperties::SuccessorOf(n.get() - 1),
            Self::Bytes(bytes) => StructureProperties::Bytes(bytes.properties()),
            Self::Text(text) => StructureProperties::Text(text.properties()),
            Self::Any(any_structure) => StructureProperties::Any(any_structure.properties()),
            Self::Character(c) => StructureProperties::CodePoint(*c),
            Self::Byte(byte) => StructureProperties::Byte(byte.properties()),
        }
    }

    /// Merges the properties of `self` and `other` into a new AnyStructure.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut add_properties = other.properties().collect::<Vec<_>>();

        self.add(add_properties.as_mut_slice())
    }

    /// Checks if `self` has this property.
    #[must_use]
    pub fn has(&self, tag: &Object, value: &Object) -> bool {
        match self {
            Self::Empty => false,
            Self::NaturalNumber(non_zero) => {
                let self_as_property = Property::new_successor_of(non_zero.get() - 1);
                &self_as_property.tag == tag && &self_as_property.value == value
            }
            Self::Bytes(bytes) => bytes.has(tag, value),
            Self::Text(text) => text.has(tag, value),
            Self::Any(any) => any.has(tag, value),
            Self::Character(c) => {
                let self_as_property = Property::new_character(*c);
                &self_as_property.tag == tag && &self_as_property.value == value
            }
            Self::Byte(byte) => byte.has(tag, value),
        }
    }

    /// Determines if `self` is a subset of `other` by checking
    /// if `other` has every property of `self`.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.properties()
            .all(|property| other.has(&property.tag, &property.value))
    }

    /// Returns an iterator over all values that this tag has
    /// in `self`.
    #[must_use]
    pub fn values<'props>(&'props self, tag: Object) -> StructureValues<'props> {
        match self {
            Self::NaturalNumber(non_zero) => {
                if tag == Object::Abstract(Abstract::SUCCESSOR_OF) {
                    StructureValues::SuccessorOf(non_zero.get() - 1)
                } else {
                    StructureValues::None
                }
            }
            Self::Bytes(bytes) => StructureValues::Bytes(bytes.values(tag)),
            Self::Text(text) => StructureValues::Text(text.values(tag)),
            Self::Any(any_structure) => StructureValues::Any(any_structure.values(tag)),
            Self::Empty => StructureValues::None,
            Self::Byte(byte) => StructureValues::Byte(byte.values(tag)),
            Self::Character(c) => {
                if tag == Abstract::CODE_POINT.into() {
                    StructureValues::CodePoint(*c)
                } else {
                    StructureValues::None
                }
            }
        }
    }

    /// Returns an iterator over all tags that this value has in `self`.
    pub fn tags<'properties>(&'properties self, value: Object) -> StructureTags<'properties> {
        match self {
            Self::Empty => StructureTags::None,
            Self::Character(c) => {
                if value == Object::from(Structure::from(*c)) {
                    StructureTags::CodePoint
                } else {
                    StructureTags::None
                }
            }
            Self::NaturalNumber(non_zero) => {
                if Property::new_successor_of(non_zero.get() - 1).value == value {
                    StructureTags::SuccessorOf
                } else {
                    StructureTags::None
                }
            }
            Self::Bytes(bytes) => {
                let (item, tail) = bytes.parts();

                if Object::from(Structure::from(item)) == value {
                    StructureTags::ListItem
                } else if bytes.as_ref() == tail {
                    StructureTags::Tail
                } else {
                    StructureTags::None
                }
            }
            Self::Text(text) => {
                let (item, tail) = text.parts();

                if Object::from(Structure::from(item)) == value {
                    StructureTags::ListItem
                } else if text.as_ref() == tail {
                    StructureTags::Tail
                } else {
                    StructureTags::None
                }
            }
            Self::Any(any_structure) => StructureTags::Any(any_structure.tags(value)),
            Self::Byte(byte) => StructureTags::Byte(byte.tags(value)),
        }
    }

    /// Extracts and clones `self` into some bytes which may be empty.
    #[must_use]
    pub fn exact_bytes(&self) -> Option<MaybeEmptyBytesStructure> {
        MaybeEmptyBytesStructure::try_from(self).ok()
    }
}

impl From<char> for Structure {
    fn from(value: char) -> Self {
        Self::Character(value)
    }
}

impl From<AnyStructure> for Structure {
    fn from(value: AnyStructure) -> Self {
        Self::Any(value)
    }
}

impl From<&[u8]> for Structure {
    fn from(slice: &[u8]) -> Self {
        BytesStructure::new(slice).map_or(Self::Empty, Self::Bytes)
    }
}

impl From<&str> for Structure {
    fn from(slice: &str) -> Self {
        TextStructure::new(slice).map_or(Self::Empty, Self::Text)
    }
}

impl From<Byte> for Structure {
    fn from(value: Byte) -> Self {
        Self::Byte(value)
    }
}

impl From<BytesStructure> for Structure {
    fn from(value: BytesStructure) -> Self {
        Self::Bytes(value)
    }
}

impl From<TextStructure> for Structure {
    fn from(value: TextStructure) -> Self {
        Self::Text(value)
    }
}

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("{}"),
            Self::Any(any) => any.fmt(f),
            Self::NaturalNumber(n) => n.fmt(f),
            Self::Text(t) => t.fmt(f),
            Self::Bytes(b) => b.fmt(f),
            Self::Character(c) => write!(f, "'{c}'"),
            Self::Byte(b) => b.fmt(f),
        }
    }
}

impl<const N: usize> PartialEq<[Property; N]> for Structure {
    fn eq(&self, other: &[Property; N]) -> bool {
        self == other.as_slice()
    }
}

impl PartialEq<[Property]> for Structure {
    fn eq(&self, other: &[Property]) -> bool {
        self.properties().eq_by(other.iter(), |a, b| &a == b)
    }
}

/// An iterator over all [`Property`]s of a [`Structure`].
///
/// You can get an instance of this type via [`Structure::properties`].
#[derive(Clone)]
pub enum StructureProperties<'structure> {
    Empty,
    SuccessorOf(u128),
    CodePoint(char),
    Byte(ByteProperties),
    Any(AnyStructureProperties<'structure>),
    Text(TextStructureProperties<'structure>),
    Bytes(BytesStructureProperties<'structure>),
}

impl<'structure> Iterator for StructureProperties<'structure> {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::SuccessorOf(n) => {
                let n = *n;
                *self = Self::Empty;

                Some(Property::new_successor_of(n))
            }
            Self::CodePoint(c) => {
                let c = *c;
                *self = Self::Empty;

                Some(Property::new_character(c))
            }
            Self::Byte(properties) => properties.next(),
            Self::Any(properties) => properties.next().cloned(),
            Self::Text(properties) => properties.next(),
            Self::Bytes(propeties) => propeties.next(),
        }
    }
}

impl std::fmt::Debug for StructureProperties<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_set().entries(&mut this).finish()
    }
}

/// An iterator over all [`Object`]s that are values in
/// the properties of a [`Structure`].
///
/// You can get an instance of this type from [`Structure::values`].
#[derive(Clone)]
pub enum StructureValues<'properties> {
    None,
    SuccessorOf(u128),
    CodePoint(char),
    Byte(ByteValues),
    Bytes(BytesStructureValues<'properties>),
    Text(TextStructureValues<'properties>),
    Any(AnyStructureValues<'properties>),
}

impl<'properties> Iterator for StructureValues<'properties> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::SuccessorOf(n) => {
                let n = *n;
                *self = Self::None;

                Some(Object::new_natural_number(n))
            }
            Self::CodePoint(c) => {
                let c = *c;
                *self = Self::None;

                Some(Object::Structure(Structure::from(c)))
            }
            Self::Bytes(bytes) => bytes.next(),
            Self::Text(text) => text.next(),
            Self::Any(any) => any.next().cloned(),
            Self::Byte(byte) => byte.next(),
        }
    }
}

impl std::fmt::Debug for StructureValues<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_set().entries(&mut this).finish()
    }
}

/// An iterator over all [`Object`]s that are tags in
/// a [`Structure`].
///
/// You can get an instance of this type from [`Structure::tags`].
#[derive(Clone)]
pub enum StructureTags<'properties> {
    None,
    SuccessorOf,
    ListItem,
    Tail,
    CodePoint,
    Any(AnyStructureTags<'properties>),
    Byte(ByteTags),
}

impl Iterator for StructureTags<'_> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::SuccessorOf => {
                *self = Self::None;
                Some(Object::Abstract(Abstract::SUCCESSOR_OF))
            }
            Self::ListItem => {
                *self = Self::None;
                Some(Object::Abstract(Abstract::LIST_ITEM))
            }
            Self::Tail => {
                *self = Self::None;
                Some(Object::Abstract(Abstract::LIST_TAIL))
            }
            Self::CodePoint => {
                *self = Self::None;
                Some(Object::Abstract(Abstract::CODE_POINT))
            }
            Self::Any(any) => any.next().cloned(),
            Self::Byte(byte) => byte.next().map(|slot| Object::from(Abstract::from(slot))),
        }
    }
}

impl std::fmt::Debug for StructureTags<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_set().entries(&mut this).finish()
    }
}
