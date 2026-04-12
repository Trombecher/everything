mod any;
mod bin;
mod registries;
#[cfg(test)]
mod tests;
mod text;

use std::{
    cmp::Ordering,
    fmt::{Debug, Pointer},
    hash::Hash,
    num::NonZeroU128,
};

pub use any::*;
pub use bin::*;
pub use registries::*;
pub use text::*;

/// A structure is a set of properties. Natural numbers, text, and binary data
/// can be stored more efficiently than an [AnyStructure].
#[derive(Clone)]
pub enum Structure {
    NaturalNumber(NonZeroU128),
    Binary(BlobStructure),
    Text(TextStructure),
    Any(AnyStructure),
}

impl Structure {
    pub fn exact_natural_number(&self) -> Option<NonZeroU128> {
        match self {
            Self::Any(any) => any.exact_natural_number(),
            Self::NaturalNumber(n) => Some(*n),
            _ => None,
        }
    }

    pub fn any(&self) -> Option<&AnyStructure> {
        match self {
            Self::NaturalNumber(_) => None,
            Self::Any(any) => Some(any),
            _ => None,
        }
    }
}

impl From<NonZeroU128> for Structure {
    fn from(value: NonZeroU128) -> Self {
        Self::NaturalNumber(value)
    }
}

impl From<AnyStructure> for Structure {
    fn from(value: AnyStructure) -> Self {
        Self::Any(value)
    }
}

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any(any) => any.fmt(f),
            Self::NaturalNumber(n) => n.fmt(f),
            Self::Text(t) => t.fmt(f),
            Self::Binary(b) => b.fmt(f),
        }
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Binary(a), Self::Binary(b)) => a == b,
            (Self::Text(a), Self::Text(b)) => a == b,
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

impl Eq for Structure {}

impl PartialOrd for Structure {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Structure {
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

impl Hash for Structure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        // TODO
    }
}
