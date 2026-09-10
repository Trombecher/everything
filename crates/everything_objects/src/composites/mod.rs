mod any;
mod byte;
mod bytes;
mod registry;
#[cfg(test)]
mod tests;
mod text;

use std::{fmt::Debug, hash::Hash, num::NonZeroI128};

pub use any::*;
pub use byte::*;
pub use bytes::*;
pub use text::*;

use crate::{Abstract, Object, Property};

/// A Composite is a set of [`Property`]s. Natural numbers, text, binary data,
/// and the Composite with no properties are stored more efficiently than an
/// [`AnyComposite`]. These are called specializations.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Composite {
    /// The empty Composite `{}`.
    Empty,

    /// An integer. Positive integers are of this form
    ///
    /// ```plain
    /// {(@SUCCESSOR_OF, <positive integer> | @ZERO)}
    /// ```
    ///
    /// and negative integers are of this form:
    ///
    /// ```plain
    /// {(@PREDECESSOR_OF, <negative integer> | @ZERO)}
    /// ```
    ///
    /// Note that the associated field in this variant stores
    /// the **actual integer that this Composite represents**,
    /// not the value of the property.
    Integer(NonZeroI128),

    /// A list of bytes.
    ///
    /// ```plain
    /// {
    ///     (@LIST_ITEM, <byte>),
    ///     (@LIST_TAIL, <binary>)
    /// }
    /// ```
    Bytes(BytesComposite),

    /// A list of characters.
    ///
    /// ```plain
    /// {
    ///     (@LIST_ITEM, <char>),
    ///     (@LIST_TAIL, <text>)
    /// }
    /// ```
    Text(TextComposite),

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

    /// Any Composite, a Composite that is not a specialization.
    Any(AnyComposite),
}

impl Composite {
    /// Creates a [`Composite`] from the given properties.
    #[must_use]
    pub fn new(properties: &mut [Property]) -> Self {
        Self::Empty.add(properties)
    }

    /// Adds the given properties to this [`Composite`].
    #[must_use]
    pub fn add(&self, properties: &mut [Property]) -> Self {
        self.change(&mut [], properties)
    }

    /// Removes the given properties from this [`Composite`].
    #[must_use]
    pub fn remove(&self, properties: &mut [Property]) -> Self {
        self.change(properties, &mut [])
    }

    /// Modifies this composite by adding and removing properties.
    /// Returns the modified composite.
    ///
    /// The properties need to be mutable because this method needs to
    /// reorder and dedup changes in-place to avoid unneccessary
    /// allocations.
    ///
    /// Note that first all indicated properties are removed from
    /// the composite and then all indicated properties are added.
    #[must_use]
    pub fn change(
        &self,
        remove_properties: &mut [Property],
        add_properties: &mut [Property],
    ) -> Composite {
        registry::resolve(self, remove_properties, add_properties)
    }

    /// Returns `Some(_)` if `self` is an [`AnyComposite`];
    /// else `None`.
    pub fn any(&self) -> Option<&AnyComposite> {
        match self {
            Self::Any(any) => Some(any),
            _ => None,
        }
    }

    /// Returns an iterator over all properties of `self`.
    pub fn properties(&self) -> CompositeProperties {
        CompositeProperties::new(self)
    }

    /// Merges the properties of `self` and `other` into a new AnyComposite.
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
            Self::Integer(non_zero) => {
                let self_as_property = Property::new_integer(*non_zero);
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
    pub fn values(&self, tag: Object) -> CompositeValues {
        match self {
            Self::Integer(non_zero) => {
                if Property::new_integer(*non_zero).tag == tag {
                    CompositeValues::Integer(*non_zero)
                } else {
                    CompositeValues::None
                }
            }
            Self::Bytes(bytes) => CompositeValues::Bytes(bytes.values(tag)),
            Self::Text(text) => CompositeValues::Text(text.values(tag)),
            Self::Any(any_composite) => CompositeValues::Any(any_composite.values(tag)),
            Self::Empty => CompositeValues::None,
            Self::Byte(byte) => CompositeValues::Byte(byte.values(tag)),
            Self::Character(c) => {
                if tag == Abstract::CODE_POINT.into() {
                    CompositeValues::CodePoint(*c)
                } else {
                    CompositeValues::None
                }
            }
        }
    }

    /// Returns an iterator over all tags that this value has in `self`.
    pub fn tags(&self, value: Object) -> CompositeTags {
        match self {
            Self::Empty => CompositeTags::None,
            Self::Character(c) => {
                if value == Object::from(Composite::from(*c)) {
                    CompositeTags::CodePoint
                } else {
                    CompositeTags::None
                }
            }
            Self::Integer(non_zero) => {
                let self_as_property = Property::new_integer(*non_zero);

                if self_as_property.value == value {
                    if self_as_property.tag == Object::Abstract(Abstract::SUCCESSOR_OF) {
                        CompositeTags::SuccessorOf
                    } else {
                        CompositeTags::PredecessorOf
                    }
                } else {
                    CompositeTags::None
                }
            }
            Self::Bytes(bytes) => {
                let (item, tail) = bytes.parts();

                if Object::from(Composite::from(item)) == value {
                    CompositeTags::ListItem
                } else if bytes.as_ref() == tail {
                    CompositeTags::Tail
                } else {
                    CompositeTags::None
                }
            }
            Self::Text(text) => {
                let (item, tail) = text.parts();

                if Object::from(Composite::from(item)) == value {
                    CompositeTags::ListItem
                } else if text.as_ref() == tail {
                    CompositeTags::Tail
                } else {
                    CompositeTags::None
                }
            }
            Self::Any(any_composite) => CompositeTags::Any(any_composite.tags(value)),
            Self::Byte(byte) => CompositeTags::Byte(byte.tags(value)),
        }
    }

    /// Extracts and clones `self` into some bytes which may be empty.
    #[must_use]
    pub fn exact_bytes(&self) -> Option<MaybeEmptyBytesComposite> {
        MaybeEmptyBytesComposite::try_from(self).ok()
    }

    /// Extracts and clones `self` into some text which may be empty.
    #[must_use]
    pub fn exact_text(&self) -> Option<MaybeEmptyTextComposite> {
        MaybeEmptyTextComposite::try_from(self).ok()
    }
}

impl From<char> for Composite {
    fn from(value: char) -> Self {
        Self::Character(value)
    }
}

impl From<AnyComposite> for Composite {
    fn from(value: AnyComposite) -> Self {
        Self::Any(value)
    }
}

impl From<&[u8]> for Composite {
    fn from(slice: &[u8]) -> Self {
        BytesComposite::new(slice).map_or(Self::Empty, Self::Bytes)
    }
}

impl From<&str> for Composite {
    fn from(slice: &str) -> Self {
        TextComposite::new(slice).map_or(Self::Empty, Self::Text)
    }
}

impl From<Byte> for Composite {
    fn from(value: Byte) -> Self {
        Self::Byte(value)
    }
}

impl From<BytesComposite> for Composite {
    fn from(value: BytesComposite) -> Self {
        Self::Bytes(value)
    }
}

impl From<TextComposite> for Composite {
    fn from(value: TextComposite) -> Self {
        Self::Text(value)
    }
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("{}"),
            Self::Any(any) => any.fmt(f),
            Self::Integer(n) => n.fmt(f),
            Self::Text(t) => t.fmt(f),
            Self::Bytes(b) => b.fmt(f),
            Self::Character(c) => write!(f, "'{c}'"),
            Self::Byte(b) => b.fmt(f),
        }
    }
}

impl<const N: usize> PartialEq<[Property; N]> for Composite {
    fn eq(&self, other: &[Property; N]) -> bool {
        self == other.as_slice()
    }
}

impl PartialEq<[Property]> for Composite {
    fn eq(&self, other: &[Property]) -> bool {
        self.properties().eq_by(other.iter(), |a, b| &a == b)
    }
}

/// An iterator over all [`Property`]s of a [`Composite`].
///
/// You can get an instance of this type via [`Composite::properties`].
#[derive(Clone)]
pub enum CompositeProperties {
    Empty,
    Integer(NonZeroI128),
    CodePoint(char),
    Byte(ByteProperties),
    Any(AnyCompositeProperties),
    Text(TextCompositeProperties),
    Bytes(BytesCompositeProperties),
}

impl CompositeProperties {
    pub fn new(composite: &Composite) -> Self {
        match composite {
            Composite::Empty => Self::Empty,
            Composite::Integer(n) => Self::Integer(*n),
            Composite::Bytes(bytes) => Self::Bytes(bytes.properties()),
            Composite::Text(text) => Self::Text(text.properties()),
            Composite::Any(any) => Self::Any(any.properties()),
            Composite::Character(c) => Self::CodePoint(*c),
            Composite::Byte(byte) => Self::Byte(byte.properties()),
        }
    }
}

impl Iterator for CompositeProperties {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Integer(n) => {
                let property = Property::new_integer(*n);
                *self = Self::Empty;

                Some(property)
            }
            Self::CodePoint(c) => {
                let c = *c;
                *self = Self::Empty;

                Some(Property::new_character(c))
            }
            Self::Byte(properties) => properties.next(),
            Self::Any(properties) => properties.next(),
            Self::Text(properties) => properties.next(),
            Self::Bytes(propeties) => propeties.next(),
        }
    }
}

impl std::fmt::Debug for CompositeProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.clone()).finish()
    }
}

/// An iterator over all [`Object`]s that are values in
/// the properties of a [`Composite`].
///
/// You can get an instance of this type from [`Composite::values`].
#[derive(Clone)]
pub enum CompositeValues {
    None,
    /// Either "successor of" iff the value is positive;
    /// else "predecessor of".
    Integer(NonZeroI128),
    CodePoint(char),
    Byte(ByteValues),
    Bytes(BytesCompositeValues),
    Text(TextCompositeValues),
    Any(AnyCompositeValues),
}

impl Iterator for CompositeValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::Integer(n) => {
                let value = Property::new_integer(*n).value;
                *self = Self::None;

                Some(value)
            }
            Self::CodePoint(c) => {
                let c = *c;
                *self = Self::None;

                Some(Object::Composite(Composite::from(c)))
            }
            Self::Bytes(bytes) => bytes.next(),
            Self::Text(text) => text.next(),
            Self::Any(any) => any.next(),
            Self::Byte(byte) => byte.next(),
        }
    }
}

impl std::fmt::Debug for CompositeValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_set().entries(&mut this).finish()
    }
}

/// An iterator over all [`Object`]s that are tags in
/// a [`Composite`].
///
/// You can get an instance of this type from [`Composite::tags`].
#[derive(Clone)]
pub enum CompositeTags {
    None,
    SuccessorOf,
    PredecessorOf,
    ListItem,
    Tail,
    CodePoint,
    Any(AnyCompositeTags),
    Byte(ByteTags),
}

impl Iterator for CompositeTags {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::SuccessorOf => {
                *self = Self::None;
                Some(Object::Abstract(Abstract::SUCCESSOR_OF))
            }
            Self::PredecessorOf => {
                *self = Self::None;
                Some(Object::Abstract(Abstract::PREDECESSOR_OF))
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
            Self::Any(any) => any.next(),
            Self::Byte(byte) => byte.next().map(|slot| Object::from(Abstract::from(slot))),
        }
    }
}

impl std::fmt::Debug for CompositeTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_set().entries(&mut this).finish()
    }
}
