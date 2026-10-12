use std::{cmp::Ordering, fmt::Debug, num::NonZeroI128, sync::Arc};

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

    /// No specialization.
    Any(AnySet),
}

impl Set {
    #[must_use]
    pub fn empty() -> Self {
        Self::new(&[])
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(objects: &[Object]) -> Self {
        match objects {
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::SUCCESSOR_OF))
                    && let Some(integer) = value.integer()
                    && integer >= 0 =>
            {
                Self::Integer(
                    NonZeroI128::new(integer.checked_add(1).expect("numbers too large")).unwrap(),
                )
            }
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::PREDECESSOR_OF))
                    && let Some(integer) = value.integer()
                    && integer <= 0 =>
            {
                Self::Integer(
                    NonZeroI128::new(integer.checked_sub(1).expect("numbers too small")).unwrap(),
                )
            }
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::CODE_POINT))
                    && let Some(integer) = value.integer()
                    && let Some(character) = u32::try_from(integer)
                        .ok()
                        .and_then(|u| char::try_from(u).ok()) =>
            {
                Self::Character(character)
            }
            _ => Self::Any(AnySet::new(objects)),
        }
    }

    #[must_use]
    pub fn values(self) -> SetValues {
        match self {
            Self::Integer(non_zero) => SetValues::Integer(non_zero),
            Self::Any(any_set) => SetValues::Any(any_set.values()),
            Self::Character(character) => SetValues::Character(character),
            Self::Byte(byte) => SetValues::Byte(byte.values()),
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
        }
    }
}

#[derive(Clone)]
pub enum SetValues {
    Integer(NonZeroI128),
    Character(char),
    Byte(ByteValues),
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
        }
    }
}

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
