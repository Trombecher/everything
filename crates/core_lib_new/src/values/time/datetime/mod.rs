mod tests;

use std::ops::{Add, Sub};
use std::time::SystemTime;
use crate::values::Duration;

/// Represents a particular moment in time.
///
/// Represented as the number of nanoseconds after January 1st, 1970 (UNIX epoch).
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct DateTime(Duration);

impl DateTime {
    /// The 1st of January, year 2000.
    pub const UNIX: Self = Self(Duration::ZERO);

    /// The 1st of January, year 2000.
    pub const Y2K: Self = Self(Duration::from_nanos(946_684_800_000_000_000));

    /// The current [DateTime].
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }
    
    pub const fn const_add(self, dur: Duration) -> Self {
        Self(self.0 + dur)
    }
}

impl From<SystemTime> for DateTime {
    fn from(value: SystemTime) -> Self {
        Self(
            Duration::from_nanos(value
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i128),
        )
    }
}

impl const Add<Duration> for DateTime {
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