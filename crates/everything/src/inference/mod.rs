mod compute;
mod error;
mod query;
pub mod validate;

pub use error::*;
use everything_structures::Structure;

#[derive(Clone)]
pub struct Knowledge(Structure);

impl Knowledge {
    #[inline]
    pub fn new(structure: Structure) -> Result<Self, ValidationError> {
        validate::knowledge(&structure).map(|()| Self(structure))
    }

    #[must_use]
    #[inline]
    pub fn structure(&self) -> &Structure {
        &self.0
    }
}

impl TryFrom<Structure> for Knowledge {
    type Error = ValidationError;

    fn try_from(value: Structure) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Into<Structure> for Knowledge {
    fn into(self) -> Structure {
        self.0
    }
}
