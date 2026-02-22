use std::hint::assert_unchecked;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct U126(u128);

impl U126 {
    pub const MAX: Self = Self::new(u128::MAX >> 2).unwrap();

    const fn check(value: u128) -> bool {
        value <= (u128::MAX >> 2)
    }

    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(value: u128) -> Self {
        Self(value)
    }

    #[inline]
    #[must_use]
    pub const fn new(value: u128) -> Option<Self> {
        if Self::check(value) {
            Some(unsafe { Self::new_unchecked(value) })
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub const fn unwrap(self) -> u128 {
        self.0
    }
}

impl Into<u128> for U126 {
    fn into(self) -> u128 {
        unsafe {
            assert_unchecked(Self::check(self.0));
        }

        self.0
    }
}

impl TryFrom<u128> for U126 {
    type Error = ();

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
