mod any;
mod bin;
mod registries;
mod str;
#[cfg(test)]
mod tests;

use std::{cmp::Ordering, fmt::Debug, hash::Hash, num::NonZeroU128};

pub use any::*;
pub use bin::*;
pub use registries::*;
pub use str::*;

#[derive(Clone)]
pub enum Structure<R: Registry = GlobalRegistry> {
    NaturalNumber(NonZeroU128),
    Any(AnyStructure<R>),
}

impl<R: Registry> Structure<R> {
    pub fn exact_natural_number(&self) -> Option<NonZeroU128> {
        match self {
            Self::Any(any) => any.exact_natural_number(),
            Self::NaturalNumber(n) => Some(*n),
        }
    }
}

impl<R: Registry> From<NonZeroU128> for Structure<R> {
    fn from(value: NonZeroU128) -> Self {
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
            (Self::NaturalNumber(m), Self::NaturalNumber(n)) => m == n,
            (Self::Any(any_a), Self::Any(any_b)) => any_a == any_b,
            (Self::NaturalNumber(m), Self::Any(any))
                if let Some(n) = any.exact_natural_number() =>
            {
                *m == n
            }
            (Self::Any(any), Self::NaturalNumber(n))
                if let Some(m) = any.exact_natural_number() =>
            {
                m == *n
            }
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
            (Self::Any(any), Self::NaturalNumber(n)) => {
                if let Some(m) = any.exact_natural_number() {
                    m.cmp(n)
                } else {
                    Ordering::Greater
                }
            }
            (Self::NaturalNumber(m), Self::Any(any)) => {
                if let Some(n) = any.exact_natural_number() {
                    m.cmp(&n)
                } else {
                    Ordering::Less
                }
            }
            (Self::NaturalNumber(m), Self::NaturalNumber(n)) => m.cmp(n),
        }
    }
}

impl<R: Registry> Hash for Structure<R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
