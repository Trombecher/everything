//! Module containing code that implements the computation.

mod capture;
mod evaluate;
mod lazy;
mod nodes;
pub mod operations;

pub use capture::*;
pub use evaluate::*;
pub use lazy::*;
pub use nodes::*;

use crate::model::{Atom, Object};

/// Represents a result of a computation that could be undefined.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MaybeUndefined<T> {
    Defined(T),
    Undefined,
}

impl<T> MaybeUndefined<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> MaybeUndefined<U> {
        match self {
            Self::Defined(t) => MaybeUndefined::Defined(f(t)),
            Self::Undefined => MaybeUndefined::Undefined,
        }
    }
}

impl<T> From<MaybeUndefined<T>> for Object
where
    Object: From<T>,
{
    fn from(value: MaybeUndefined<T>) -> Self {
        match value {
            MaybeUndefined::Defined(value) => value.into(),
            MaybeUndefined::Undefined => Atom::UNDEFINED.into(),
        }
    }
}

impl<T> From<Option<T>> for MaybeUndefined<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Self::Defined(t),
            None => Self::Undefined,
        }
    }
}
