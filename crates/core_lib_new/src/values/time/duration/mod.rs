use std::cmp::Ord;
use std::ops::{Add, AddAssign, Sub};

const fn const_min(a: i128, b: i128) -> i128 {
    if a < b {
        a
    } else {
        b
    }
}

const fn const_max(a: i128, b: i128) -> i128 {
    if a > b {
        a
    } else {
        b
    }
}

mod tests;

/// A duration of time, measured in nanoseconds. **Maybe negative.**.
///
/// Represented as a `i120`. Values at the boundary will saturate with
/// [Duration::MIN] and [Duration::MAX].
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Duration(i128);

impl Duration {
    // TODO: check these values
    pub const MIN: Self = Self(!Self::MAX.0);
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(i128::MAX >> 8);

    #[inline]
    pub const fn as_nanos(self) -> i128 {
        self.0
    }

    #[inline]
    pub const fn as_micros(self) -> i128 {
        self.as_nanos() / 1_000
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
    pub const fn from_nanos(nanos: i128) -> Self {
        if nanos < Self::MIN.0 {
            Self::MIN
        } else if nanos > Self::MAX.0 {
            Self::MAX
        } else {
            Self(nanos)
        }
    }

    #[inline]
    pub const fn from_micros(micros: i128) -> Self {
        Self::from_nanos(micros * 1_000)
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
    pub const fn const_sub(self, other: Duration) -> Self {
        Self(const_max(const_min(self.0.saturating_sub(other.0), Self::MAX.0), Self::MIN.0))
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value.as_nanos() as i128)
    }
}

impl const Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(const_min(const_max(self.0.saturating_add(rhs.0), Self::MIN.0), Self::MAX.0))
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0).max(Self::MIN.0))
    }
}