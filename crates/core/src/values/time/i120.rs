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
pub struct I120([u8; 15]);

impl I120 {
    pub const MAX: Self = Self([
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        0b0111_1111,
    ]);
    pub const MIN: Self = Self([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0b1000_0000]);

    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 15]) -> Self {
        Self(bytes)
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 15] {
        self.0
    }

    #[inline]
    pub const fn from_i128(x: i128) -> Self {
        let [bytes @ .., _] =
            const_clamp(x, Self::MIN.as_i128(), Self::MAX.as_i128()).to_le_bytes();
        Self(bytes)
    }

    #[inline]
    pub const fn as_i128(self) -> i128 {
        // Sign-extension
        let byte_15 = if self.0[14] & 0b1000_0000 != 0 {
            255_u8
        } else {
            0
        };

        i128::from_le_bytes(unsafe { transmute::<_, [u8; 16]>((self.0, byte_15)) })
    }

    #[inline]
    pub const fn const_add(self, other: Self) -> Self {
        Self::from_i128(const_clamp(
            self.as_i128().saturating_add(other.as_i128()),
            Self::MIN.as_i128(),
            Self::MAX.as_i128(),
        ))
    }

    #[inline]
    pub const fn const_sub(self, other: Self) -> Self {
        Self::from_i128(const_clamp(
            self.as_i128().saturating_sub(other.as_i128()),
            Self::MIN.as_i128(),
            Self::MAX.as_i128(),
        ))
    }

    #[inline]
    pub const fn abs(self) -> Self {
        Self::from_i128(self.as_i128().abs())
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
        Self::from_i128(x)
    }
}

impl Into<i128> for I120 {
    #[inline]
    fn into(self) -> i128 {
        self.as_i128()
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
        Self::from_i128(unsafe { self.as_i128().checked_neg().unwrap_unchecked() })
    }
}
