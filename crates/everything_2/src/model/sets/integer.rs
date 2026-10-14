use std::{fmt::Debug, num::NonZeroI128};

use crate::model::{Atom, Construction, Object, SetApi, SetSpecialization};

/// Specialization for sets `{successor_of n}` and `{predecessor_of n}`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerSet {
    pub representing: NonZeroI128,
}

impl Debug for IntegerSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.representing, f)
    }
}

impl IntoIterator for IntegerSet {
    type Item = Object;
    type IntoIter = IntegerSetValues;

    fn into_iter(self) -> Self::IntoIter {
        IntegerSetValues {
            to_yield: Some(self.representing),
        }
    }
}

impl SetSpecialization for IntegerSet {
    fn specialize_from(objects: &[Object]) -> Option<Self> {
        match objects {
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::SUCCESSOR_OF))
                    && let Some(integer) = value.integer()
                    && integer >= 0 =>
            {
                Some(Self {
                    representing: NonZeroI128::new(
                        integer.checked_add(1).expect("numbers too large"),
                    )
                    .unwrap(),
                })
            }
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::PREDECESSOR_OF))
                    && let Some(integer) = value.integer()
                    && integer <= 0 =>
            {
                Some(Self {
                    representing: NonZeroI128::new(
                        integer.checked_sub(1).expect("numbers too small"),
                    )
                    .unwrap(),
                })
            }
            _ => None,
        }
    }
}

impl SetApi for IntegerSet {
    fn size(&self) -> u64 {
        1
    }

    fn has(&self, object: &Object) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub struct IntegerSetValues {
    to_yield: Option<NonZeroI128>,
}

impl Iterator for IntegerSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let representing = self.to_yield.take()?;

        match representing.get() {
            positive_integer if positive_integer > 0 => Some(Object::Construction(Construction {
                constructor: Box::new(Object::Atom(Atom::SUCCESSOR_OF)),
                value: Box::new(Object::new_integer(
                    positive_integer.checked_sub(1).unwrap(),
                )),
            })),
            negative_integer if negative_integer < 0 => Some(Object::Construction(Construction {
                constructor: Box::new(Object::Atom(Atom::PREDECESSOR_OF)),
                value: Box::new(Object::new_integer(
                    negative_integer.checked_add(1).unwrap(),
                )),
            })),
            _ => unreachable!(),
        }
    }
}
