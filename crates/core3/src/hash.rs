use crate::u126::U126;

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct DataHash(U126);

impl DataHash {
    #[must_use]
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: U126) -> Self {
        Self(value)
    }

    #[must_use]
    #[inline(always)]
    pub const fn unwrap(self) -> U126 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct StructureHash(U126);

impl StructureHash {
    #[must_use]
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: U126) -> Self {
        Self(value)
    }

    #[must_use]
    #[inline(always)]
    pub const fn unwrap(self) -> U126 {
        self.0
    }
}
