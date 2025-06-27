mod tests;

use crate::values::Duration;
use std::ops::{Add, Sub};
use std::time::SystemTime;

/// Represents a particular moment in time.
///
/// Represented as the number of nanoseconds after January 1st, 1970 (UNIX epoch)
/// using an `i120`.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct DateTime(Duration);

impl DateTime {
    /// The 1st of January, year 2000.
    pub const UNIX: Self = Self(Duration::ZERO);

    /// The 1st of January, year 2000.
    pub const Y2K: Self = Self(Duration::from_millis(946_681_200_000));

    /// The current [DateTime].
    #[inline]
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }

    #[inline]
    pub const fn const_add(self, dur: Duration) -> Self {
        Self(self.0.const_add(dur))
    }

    #[inline]
    pub const fn diff(self, other: Self) -> Duration {
        self.0.const_sub(other.0)
    }
}

impl From<SystemTime> for DateTime {
    fn from(value: SystemTime) -> Self {
        Self(Duration::from_nanos(
            (value
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i128)
                .into(),
        ))
    }
}

impl Add<Duration> for DateTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<Duration> for DateTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<Self> for DateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.diff(rhs)
    }
}
