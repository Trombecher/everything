use std::{fmt::Debug, hint::assert_unchecked};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct RangedU128<const MIN: u128, const MAX: u128>(u128);

impl<const MIN: u128, const MAX: u128> Debug for RangedU128<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<const MIN: u128, const MAX: u128> RangedU128<MIN, MAX> {
    #[inline(always)]
    #[must_use]
    pub const unsafe fn new_unchecked(value: u128) -> Self {
        Self(value)
    }

    #[inline(always)]
    #[must_use]
    pub const fn new(value: u128) -> Option<Self> {
        if Self::condition(value) {
            Some(unsafe { Self::new_unchecked(value) })
        } else {
            None
        }
    }

    #[must_use]
    const fn condition(value: u128) -> bool {
        MIN <= value && value <= MAX
    }

    #[must_use]
    pub const fn get(self) -> u128 {
        unsafe {
            assert_unchecked(Self::condition(self.0));
        }
        self.0
    }
}

impl<const MIN: u128, const MAX: u128> Into<u128> for RangedU128<MIN, MAX> {
    fn into(self) -> u128 {
        self.get()
    }
}

impl<const MIN: u128, const MAX: u128> TryFrom<u128> for RangedU128<MIN, MAX> {
    type Error = ();

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
