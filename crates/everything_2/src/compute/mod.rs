//! Module containing code that implements the computation.

mod capture;
mod evaluate;
mod lazy;
mod nodes;
mod operations;

pub use capture::*;
pub use evaluate::*;
pub use lazy::*;
pub use nodes::*;
pub use operations::*;

use crate::model::{Atom, Object};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MaybeUndefined<T> {
    Defined(T),
    Undefined,
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
