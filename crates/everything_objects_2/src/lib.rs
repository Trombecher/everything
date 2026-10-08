#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use std::{fmt::Debug, num::NonZeroI128};

use crate::{atoms::Atom, constructions::Construction, sets::Set};

pub mod atoms;
pub mod constructions;
mod iter;
pub mod sets;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Atom(Atom),
    Construction(Construction),
    Set(Set),
}

impl Object {
    #[must_use]
    pub const fn new_integer(integer: i128) -> Self {
        if let Some(non_zero_integer) = NonZeroI128::new(integer) {
            Self::Set(Set::Integer(non_zero_integer))
        } else {
            Self::Atom(Atom::ZERO)
        }
    }

    #[must_use]
    pub fn integer(&self) -> Option<i128> {
        match self {
            Self::Atom(Atom::ZERO) => Some(0),
            Self::Set(Set::Integer(integer)) => Some(integer.get()),
            _ => None,
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => Debug::fmt(&atom, f),
            Self::Construction(construction) => Debug::fmt(&construction, f),
            Self::Set(set) => Debug::fmt(&set, f),
        }
    }
}

impl From<Atom> for Object {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

impl From<Construction> for Object {
    fn from(value: Construction) -> Self {
        Self::Construction(value)
    }
}

impl From<Set> for Object {
    fn from(value: Set) -> Self {
        Self::Set(value)
    }
}
