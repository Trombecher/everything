use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use std::ops::{Add, Neg, Sub};

/// This function solely exists because const traits are unstable.
#[inline]
const fn const_clamp(x: i128, min: i128, max: i128) -> i128 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// A number with 120 bits. Addition and subtraction are saturating.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct I120(i128);

impl I120 {
    pub const MAX: Self = Self(i128::MAX >> 8);
    pub const MIN: Self = Self(!Self::MAX.0);

    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 15]) -> Self {
        let byte_16 = (bytes[14] & 0b10000000) >> 7;
        Self(unsafe { transmute((bytes, byte_16)) })
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 15] {
        let [bytes @ .., _] = self.0.to_le_bytes();
        bytes
    }

    #[inline]
    pub const fn const_add(self, other: Self) -> Self {
        Self(const_clamp(
            self.0.saturating_add(other.0),
            Self::MIN.0,
            Self::MAX.0,
        ))
    }

    #[inline]
    pub const fn const_sub(self, other: Self) -> Self {
        Self(const_clamp(
            self.0.saturating_sub(other.0),
            Self::MIN.0,
            Self::MAX.0,
        ))
    }

    #[inline]
    pub const fn const_from(x: i128) -> Self {
        Self(const_clamp(x, Self::MIN.0, Self::MAX.0))
    }

    #[inline]
    pub const fn const_into(self) -> i128 {
        self.0
    }

    #[inline]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl Debug for I120 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<i128> for I120 {
    #[inline]
    fn from(x: i128) -> Self {
        Self::const_from(x)
    }
}

impl Into<i128> for I120 {
    fn into(self) -> i128 {
        self.0
    }
}

impl Add for I120 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

impl Sub for I120 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

impl Neg for I120 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(unsafe { self.0.checked_neg().unwrap_unchecked() })
    }
}
