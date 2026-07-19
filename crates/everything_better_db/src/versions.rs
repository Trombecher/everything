use crate::pages::storage::sync::MutableU32LeLocation;

pub struct MutableDatabaseFormatVersionLocation(MutableU32LeLocation);

impl MutableDatabaseFormatVersionLocation {
    const V0_NUMBER: u32 = 0;

    pub fn get(&self) -> DatabaseFormatVersion {
        match self.0.get() {
            Self::V0_NUMBER => DatabaseFormatVersion::Version0,
            _ => DatabaseFormatVersion::Version0,
        }
    }

    pub fn set(&self, version: DatabaseFormatVersion) {
        self.0.set(match version {
            DatabaseFormatVersion::Version0 => Self::V0_NUMBER,
        });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum DatabaseFormatVersion {
    #[default]
    Version0,
}

impl DatabaseFormatVersion {
    pub const LATEST: Self = Self::Version0;
}
