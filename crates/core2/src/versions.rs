#[derive(Copy, Clone, Debug)]
pub enum Version {
    V1 = 1,
}

impl Version {
    /// The latest version of the db file format.
    pub const LATEST: Self = Self::V1;

    #[inline(always)]
    #[must_use]
    pub const fn new(number: u32) -> Option<Self> {
        match number {
            1 => Some(Self::V1),
            _ => None,
        }
    }
}

impl TryFrom<u32> for Version {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
