mod any;
mod bin;
mod registries;
mod str;

use std::{fmt::Debug, hash::Hash};

pub use any::*;
pub use bin::*;
pub use registries::*;
pub use str::*;

#[derive(Clone)]
pub enum Structure<R: Registry = GlobalRegistry> {
    Any(AnyStructure<R>),
    NaturalNumber(u128),
}

impl<R: Registry> From<u128> for Structure<R> {
    fn from(value: u128) -> Self {
        Self::NaturalNumber(value)
    }
}

impl<R: Registry> From<AnyStructure<R>> for Structure<R> {
    fn from(value: AnyStructure<R>) -> Self {
        Self::Any(value)
    }
}

impl<R: Registry> Debug for Structure<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any(arg0) => f.debug_tuple("Any").field(arg0).finish(),
            Self::NaturalNumber(arg0) => f.debug_tuple("NaturalNumber").field(arg0).finish(),
        }
    }
}

impl<R: Registry> PartialEq for Structure<R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any(l0), Self::Any(r0)) => l0 == r0,
            (Self::NaturalNumber(l0), Self::NaturalNumber(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<R: Registry> Eq for Structure<R> {}

impl<R: Registry> PartialOrd for Structure<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<R: Registry> Ord for Structure<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Any(any_a), Self::Any(any_b)) => any_a.cmp(any_b),
            (Self::Any(any_structure), Self::NaturalNumber(_)) => todo!(),
            (Self::NaturalNumber(_), Self::Any(any_structure)) => todo!(),
            (Self::NaturalNumber(m), Self::NaturalNumber(n)) => m.cmp(n),
        }
    }
}

impl<R: Registry> Hash for Structure<R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
