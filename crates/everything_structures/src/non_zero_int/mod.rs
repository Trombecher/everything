use std::fmt::Debug;

/// A non-zero integer that can store positive integer from 1 to (including) 2^127,
/// and negative integers from -1 to (including) -2^127.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroInteger(u128);

impl NonZeroInteger {
    #[must_use]
    pub const fn new(int: DestructuredNonZeroInteger) -> Self {
        match int {
            DestructuredNonZeroInteger::Positive(int) => Self(int.0),
            DestructuredNonZeroInteger::Negative(int) => -Self(int.0),
        }
    }

    #[must_use]
    pub const fn is_positive(self) -> bool {
        // Check whether the high bit is set to 1:
        self.0 & (1_u128.rotate_right(1)) == 0
    }

    #[must_use]
    pub const fn is_negative(self) -> bool {
        !self.is_positive()
    }

    #[must_use]
    const fn number_part(self) -> u128 {
        self.0 & (u128::MAX >> 1)
    }

    #[must_use]
    pub const fn get(self) -> DestructuredNonZeroInteger {
        let int = PositiveNonZeroInteger(self.number_part());

        if self.is_positive() {
            DestructuredNonZeroInteger::Positive(int)
        } else {
            DestructuredNonZeroInteger::Negative(int)
        }
    }
}

impl const core::ops::Neg for NonZeroInteger {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0 ^ 1_u128.rotate_right(1))
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DestructuredNonZeroInteger {
    Positive(PositiveNonZeroInteger),
    Negative(PositiveNonZeroInteger),
}

/// A positive non-zero integer, aka a non-zero natural number.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositiveNonZeroInteger(u128);

impl PositiveNonZeroInteger {
    /// The minimum; equal to 1.
    pub const MIN: Self = Self(0);

    /// The maximum; equal to 2^127.
    pub const MAX: Self = Self(u128::MAX >> 1);

    #[must_use]
    pub const fn new(int: u128) -> Option<Self> {
        if int > Self::MAX.0 {
            None
        } else {
            Some(Self(int))
        }
    }

    #[must_use]
    pub const fn get(self) -> u128 {
        unsafe { self.0.unchecked_add(1) }
    }

    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        if let Some(result) = self.get().checked_add(other.0) {
            Self::new(result)
        } else {
            None
        }
    }
}

impl Debug for PositiveNonZeroInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
