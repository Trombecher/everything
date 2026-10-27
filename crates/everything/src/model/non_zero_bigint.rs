use std::fmt::Debug;

use num_bigint::BigInt;

/// A [`BigInt`] that is non-zero.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonZeroBigInt {
    inner: BigInt,
}

impl Debug for NonZeroBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl NonZeroBigInt {
    #[must_use]
    pub fn new(bigint: BigInt) -> Option<Self> {
        if bigint == BigInt::ZERO {
            None
        } else {
            Some(Self { inner: bigint })
        }
    }

    #[must_use]
    pub fn get_as_ref(&self) -> &BigInt {
        &self.inner
    }

    #[must_use]
    pub fn get(self) -> BigInt {
        self.inner
    }
}
