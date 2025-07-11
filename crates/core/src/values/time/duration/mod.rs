use crate::values::time::i120::I120;
use std::cmp::Ord;
use std::ops::{Add, Sub};

mod tests;

/// A duration of time, measured in nanoseconds. **Maybe negative.**. Represented as a [I120].
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Duration(I120);

impl Duration {
    pub const MIN: Self = Self(I120::MIN);
    pub const ZERO: Self = Self(I120::from_i128(0));
    pub const MAX: Self = Self(I120::MAX);

    #[inline]
    pub const fn as_nanos(self) -> I120 {
        self.0
    }

    #[inline]
    pub const fn as_micros(self) -> i128 {
        let nanos = self.as_nanos().as_i128();
        nanos / 1_000
    }

    #[inline]
    pub const fn as_millis(self) -> i128 {
        self.as_micros() / 1_000
    }

    #[inline]
    pub const fn as_secs(self) -> i128 {
        self.as_millis() / 1_000
    }

    #[inline]
    pub const fn as_mins(self) -> i128 {
        self.as_secs() / 60
    }

    #[inline]
    pub const fn as_hours(self) -> i128 {
        self.as_mins() / 60
    }

    #[inline]
    pub const fn as_days(self) -> i128 {
        self.as_hours() / 24
    }

    #[inline]
    pub const fn as_weeks(self) -> i128 {
        self.as_days() / 7
    }

    #[inline]
    pub const fn from_nanos(nanos: I120) -> Self {
        Self(nanos)
    }

    #[inline]
    pub const fn from_micros(micros: i128) -> Self {
        Self::from_nanos(I120::from_i128(micros * 1_000))
    }

    #[inline]
    pub const fn from_millis(millis: i128) -> Self {
        Self::from_micros(millis * 1_000)
    }

    #[inline]
    pub const fn from_secs(secs: i128) -> Self {
        Self::from_millis(secs * 1_000)
    }

    #[inline]
    pub const fn from_mins(mins: i128) -> Self {
        Self::from_secs(mins * 60)
    }

    #[inline]
    pub const fn from_hours(hours: i128) -> Self {
        Self::from_mins(hours * 60)
    }

    #[inline]
    pub const fn from_days(days: i128) -> Self {
        Self::from_hours(days * 24)
    }

    #[inline]
    pub const fn from_weeks(weeks: i128) -> Self {
        Self::from_days(weeks * 7)
    }

    #[inline]
    pub const fn const_add(self, other: Duration) -> Self {
        Self(self.0.const_add(other.0))
    }

    #[inline]
    pub const fn const_sub(self, other: Duration) -> Self {
        Self(self.0.const_sub(other.0))
    }

    #[inline]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self((value.as_nanos() as i128).into())
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
