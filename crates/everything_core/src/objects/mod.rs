use std::fmt::Debug;
use std::num::NonZeroU64;

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct ObjectId(pub NonZeroU64);

impl From<NonZeroU64> for ObjectId {
    fn from(value: NonZeroU64) -> Self {
        Self(value)
    }
}

impl Into<NonZeroU64> for ObjectId {
    fn into(self) -> NonZeroU64 {
        self.0
    }
}

pub mod core;
