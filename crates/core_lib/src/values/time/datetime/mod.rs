mod tests;

use std::time::SystemTime;

/// Represents a particular moment in time.
///
/// Represented as the number of nanoseconds after January 1st, 1970 (UNIX epoch).
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct DateTime(i128);

impl DateTime {
    /// The 1st of January, year 2000.
    pub const UNIX: Self = Self(0);

    /// The 1st of January, year 2000.
    pub const Y2K: Self = Self(946_684_800_000_000_000);

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

impl TryFrom<i128> for DateTime {
    type Error = ();

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<[u8; 15]> for DateTime {
    fn from(value: [u8; 15]) -> Self {
        todo!()
    }
}