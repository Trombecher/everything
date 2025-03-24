//! This module declares the representations of [Duration] and [DateTime].
//!
//! ## Why not use std?
//!
//! * std's implementation of time does not allow for negative durations.
//! * A moment in time's representation is system-dependent and therefore
//! not suitable for a system-independant DBMS.

use std::time::SystemTime;

/// Represents a particular moment in time.
///
/// Represented as the number of nanoseconds after January 1st, 1970 (UNIX epoch).
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct DateTime(i128);

impl DateTime {
    /// The current [DateTime].
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }
}

impl From<SystemTime> for DateTime {
    fn from(value: SystemTime) -> Self {
        Self(
            value
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i128,
        )
    }
}

impl From<i128> for DateTime {
    fn from(value: i128) -> Self {
        Self(value)
    }
}

/// A duration of time, measured in nanoseconds. **May be negative.**
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Duration(i128);

impl Duration {
    #[inline]
    pub const fn as_nanos(self) -> i128 {
        self.0
    }

    #[inline]
    pub const fn as_micros(self) -> i128 {
        self.0 / 1_000
    }

    #[inline]
    pub const fn as_millis(self) -> i128 {
        self.0 / 1_000_000
    }

    #[inline]
    pub const fn as_secs(self) -> i128 {
        self.0 / 1_000_000_000
    }

    #[inline]
    pub const fn from_nanos(nanos: i128) -> Self {
        Self(nanos)
    }

    #[inline]
    pub const fn from_micros(micros: i128) -> Self {
        Self(micros * 1_000)
    }

    #[inline]
    pub const fn from_millis(millis: i128) -> Self {
        Self(millis * 1_000_000)
    }

    #[inline]
    pub const fn from_secs(secs: i128) -> Self {
        Self(secs * 1_000_000_000)
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value.as_nanos() as i128)
    }
}
