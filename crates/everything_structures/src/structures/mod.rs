mod any;
mod byte;
mod bytes;
mod registry;
#[cfg(test)]
mod tests;
mod text;

use std::{borrow::Cow, fmt::Debug, hash::Hash, iter::Map, num::NonZeroU128};

pub use any::*;
pub use byte::*;
pub use bytes::*;
pub use text::*;

use crate::{Object, Property, fixed_or_more::FixedOrMore};

/// A structure is a set of properties. Natural numbers, text, binary data,
/// and the structure with no properties are stored more efficiently than an [AnyStructure].
/// These are called specializations.
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

    /// Returns an iterator over all properties:
    pub fn properties<'structure>(&'structure self) -> StructureProperties<'structure> {
        match self {
            Self::Empty => StructureProperties::Empty,
            Self::NaturalNumber(n) => Properties::One(Cow::Owned(Property::successor_of(*n))),
            Self::Bytes(bytes) => StructureProperties::Bytes(bytes.properties()),
            Self::Text(text) => StructureProperties::Text(text.properties()),
            Self::Any(any_structure) => StructureProperties::Any(any_structure.properties()),
            Self::Character(c) => Properties::One(Cow::Owned(Property::character(*c))),
            Self::Byte(byte) => StructureProperties::Byte(ByteProperties(byte.bits())),
        }
    }

    /// Merges the properties of `self` and `other` into a new AnyStructure.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut add_properties = other.properties().map(Cow::into_owned).collect::<Vec<_>>();

        self.add(add_properties.as_mut_slice())
    }

    /// Checks if `self` has this property.
    #[must_use]
    pub fn has(&self, property: &Property) -> bool {
        match self {
            Self::Empty => false,
            Self::NaturalNumber(non_zero) => property == &Property::successor_of(*non_zero),
            Self::Character(c) => property == &Property::character(*c),
            Self::Bytes(bytes) => bytes.has(property),
            Self::Text(_) => todo!(),
            Self::Any(any) => any.has(property),
            Self::Byte(_) => todo!(),
        }
    }

    #[must_use]
    pub fn has_by_ref(&self, tag: &Object, value: &Object) -> bool {
        match self {
            Self::Empty => false,
            Self::NaturalNumber(non_zero) => {
                let self_as_property = Property::successor_of(*non_zero);
                &self_as_property.tag == tag && &self_as_property.value == value
            }
            Self::Bytes(_) => todo!(),
            Self::Text(_) => todo!(),
            Self::Any(any) => any
                .as_ref()
                .binary_search_by(|property| {
                    property
                        .tag
                        .cmp(tag)
                        .then_with(|| property.value.cmp(value))
                })
                .is_ok(),
            Self::Character(c) => {
                let self_as_property = Property::character(*c);
                &self_as_property.tag == tag && &self_as_property.value == value
            }
        }
    }

    /// Determines if `self` is a subset of `other` by checking
    /// if `other` has every property of `self`.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.properties().all(|property| other.has(&property))
    }

    /// Returns an iterator over all values that this tag has
    /// in `self`.
    #[must_use]
    pub fn values<'props>(&'props self, tag: Object) -> StructureValues<'props> {
        match self {
            Self::NaturalNumber(non_zero) if tag == Object::SUCCESSOR_OF => {
                StructureValues::One(Cow::Owned(Property::successor_of(*non_zero).value))
            }
            Self::Bytes(_) => todo!(),
            Self::Text(_) => todo!(),
            Self::Any(any_structure) => {
                StructureValues::More(any_structure.values(tag).map(Cow::Borrowed))
            }
            _ => StructureValues::None,
        }
    }

    /// Returns an iterator over all tags that this value has in `self`.
    pub fn tags<'properties>(&'properties self, value: Object) -> StructureTags<'properties> {
        match self {
            Self::NaturalNumber(non_zero) if Property::successor_of(*non_zero).value == value => {
                StructureTags::One(Cow::Owned(Object::SUCCESSOR_OF))
            }
            Self::Bytes(_) => todo!(),
            Self::Text(_) => todo!(),
            Self::Any(any_structure) => {
                StructureTags::More(any_structure.tags(value).map(Cow::Borrowed))
            }
            _ => FixedOrMore::None,
        }
    }
}

impl From<NonZeroU128> for Structure {
    fn from(value: NonZeroU128) -> Self {
        Self::NaturalNumber(value)
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
        self.properties().eq(other.iter().map(Cow::Borrowed))
    }
}

#[derive(Clone)]
pub enum StructureProperties<'structure> {
    Empty,
    SuccessorOf(u128),
    Byte(ByteProperties),
    Any(AnyStructureProperties<'structure>),
    Text(TextStructureProperties<'structure>),
    Bytes(BytesStructureProperties<'structure>),
}

impl<'structure> Iterator for StructureProperties<'structure> {
    type Item = Cow<'structure, Property>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::SuccessorOf(n) => {
                let n = *n;

                *self = Self::Empty;
                Some(Cow::Owned(Property::successor_of(n)))
            }
            Self::Byte(properties) => properties.next().map(Cow::Owned),
            Self::Any(properties) => properties.next().map(Cow::Borrowed),
            Self::Text(text_structure_properties) => todo!(),
            Self::Bytes(propeties) => propeties.next().map(Cow::Owned),
        }
    }
}

pub type StructureValues<'properties> = FixedOrMore<
    Map<AnyStructureValues<'properties>, fn(&'properties Object) -> Cow<'properties, Object>>,
>;

pub type StructureTags<'properties> = FixedOrMore<
    Map<AnyStructureTags<'properties>, fn(&'properties Object) -> Cow<'properties, Object>>,
>;
