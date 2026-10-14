use std::{cmp::Ordering, fmt::Debug, mem, num::NonZeroI128, sync::Arc};

use crate::model::{Atom, Byte, ByteValues, Construction, Object};

/// A set of objects. Contains specializations for certain sets
/// which are represented optimally in memory.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Set {
    /// Specialization for `{successor_of n}` and `{predecessor_of n}`.
    Integer(NonZeroI128),

    /// Specialization for `{code_point i}`.
    Character(char),

    /// Specialization for bytes.
    Byte(Byte),

    /// Specialization for `{x y}`.
    OneConstruction(Construction),

    /// No specialization.
    Any(AnySet),
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
    pub fn new_move<const N: usize>(mut objects: [Object; N]) -> Self {
        if let Some(set) = Self::try_new_primitive_specialization(&objects) {
            set
        } else {
            match objects.as_slice() {
                [Object::Construction(Construction { .. })] => {
                    // We have to do this dance to move the construction
                    // out of the object array because we cannot match
                    // on a generic array. I hope this will get optimized.

                    Self::OneConstruction(
                        mem::take(objects.first_mut().unwrap())
                            .destructure_construction()
                            .unwrap(),
                    )
                }
                _ => Self::Any(AnySet::new_move(objects)),
            }
        }
    }

    /// Tries to create a new [`Set`] primitive specialization variant from the given list of objects.
    /// If no specialization matches, then this function returns [`None`].
    fn try_new_primitive_specialization(objects: &[Object]) -> Option<Self> {
        match objects {
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::SUCCESSOR_OF))
                    && let Some(integer) = value.integer()
                    && integer >= 0 =>
            {
                Some(Self::Integer(
                    NonZeroI128::new(integer.checked_add(1).expect("numbers too large")).unwrap(),
                ))
            }
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::PREDECESSOR_OF))
                    && let Some(integer) = value.integer()
                    && integer <= 0 =>
            {
                Some(Self::Integer(
                    NonZeroI128::new(integer.checked_sub(1).expect("numbers too small")).unwrap(),
                ))
            }
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::CODE_POINT))
                    && let Some(integer) = value.integer()
                    && let Some(character) = u32::try_from(integer)
                        .ok()
                        .and_then(|u| char::try_from(u).ok()) =>
            {
                Some(Self::Character(character))
            }
            _ => None,
        }
    }

    /// The number of objects in this set.
    #[must_use]
    pub fn size(&self) -> u64 {
        match self {
            Set::Byte(_) => 8,
            Set::Any(any_set) => any_set.as_slice().len() as u64,
            Set::Integer(_) | Set::Character(_) | Self::OneConstruction(_) => 1,
        }
    }

    /// Returns an iterator over all objects ("values") in this set.
    #[must_use]
    pub fn values(self) -> SetValues {
        match self {
            Self::Integer(non_zero) => SetValues::Integer(non_zero),
            Self::Any(any_set) => SetValues::Any(any_set.values()),
            Self::Character(character) => SetValues::Character(character),
            Self::Byte(byte) => SetValues::Byte(byte.values()),
            Self::OneConstruction(construction) => SetValues::OneConstruction(construction),
        }
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(non_zero) => Debug::fmt(non_zero, f),
            Self::Any(any_set) => Debug::fmt(any_set, f),
            Self::Character(character) => write!(f, "'{character}'"),
            Self::Byte(byte) => Debug::fmt(byte, f),
            Self::OneConstruction(construction) => f.debug_set().entry(construction).finish(),
        }
    }
}

/// An iterator over objects ("values") of a set object.
#[derive(Clone)]
pub enum SetValues {
    Integer(NonZeroI128),
    Character(char),
    Byte(ByteValues),
    OneConstruction(Construction),
    Any(AnySetValues),
}

impl Iterator for SetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Character(character) => {
                let character = *character;

                *self = Set::empty().values();

                Some(Object::Construction(Construction {
                    constructor: Box::new(Object::Atom(Atom::CODE_POINT)),
                    value: Box::new(Object::new_integer(i128::from(u32::from(character)))),
                }))
            }
            Self::Integer(non_zero_integer) => {
                let non_zero_integer = *non_zero_integer;

                // None remaining.
                *self = Set::empty().values();

                match non_zero_integer.get() {
                    positive_integer if positive_integer > 0 => {
                        Some(Object::Construction(Construction {
                            constructor: Box::new(Object::Atom(Atom::SUCCESSOR_OF)),
                            value: Box::new(Object::new_integer(
                                positive_integer.checked_sub(1).unwrap(),
                            )),
                        }))
                    }
                    negative_integer if negative_integer < 0 => {
                        Some(Object::Construction(Construction {
                            constructor: Box::new(Object::Atom(Atom::PREDECESSOR_OF)),
                            value: Box::new(Object::new_integer(
                                negative_integer.checked_add(1).unwrap(),
                            )),
                        }))
                    }
                    _ => unreachable!(),
                }
            }
            Self::Any(any_set_values) => any_set_values.next(),
            Self::Byte(byte_values) => byte_values.next().map(Object::Construction),
            Self::OneConstruction(_) => match mem::replace(self, Set::empty().values()) {
                SetValues::OneConstruction(construction) => Some(construction.into()),
                _ => unreachable!(),
            },
        }
    }
}

/// A set that is no specialization.
#[derive(Clone)]
pub struct AnySet {
    objects: Arc<[Object]>,
}

impl AnySet {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    fn new(objects: &[Object]) -> Self {
        let mut objects: Arc<[Object]> = Arc::from(objects);
        Arc::get_mut(&mut objects).unwrap().sort();

        Self { objects }
    }

    fn new_move<const N: usize>(mut objects: [Object; N]) -> Self {
        objects.sort();

        Self {
            objects: Arc::new(objects),
        }
    }

    #[must_use]
    pub fn has(&self, object: &Object) -> bool {
        self.objects.binary_search(object).is_ok()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Object] {
        &self.objects
    }

    #[must_use]
    pub fn values(self) -> AnySetValues {
        AnySetValues::new(self)
    }
}

impl Debug for AnySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.objects.iter()).finish()
    }
}

impl PartialEq for AnySet {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.objects, &other.objects) || self.objects == other.objects
    }
}

impl Eq for AnySet {}

impl PartialOrd for AnySet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnySet {
    fn cmp(&self, other: &Self) -> Ordering {
        if Arc::ptr_eq(&self.objects, &other.objects) {
            Ordering::Equal
        } else {
            self.objects.cmp(&other.objects)
        }
    }
}

#[derive(Clone)]
pub struct AnySetValues {
    set: AnySet,
    next_index: usize,
}

impl AnySetValues {
    #[must_use]
    pub fn new(set: AnySet) -> Self {
        Self { set, next_index: 0 }
    }
}

impl Iterator for AnySetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.set
            .as_slice()
            .get(self.next_index)
            .cloned()
            .inspect(|_| self.next_index += 1)
    }
}
