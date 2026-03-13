mod compute;
mod query;

pub use compute::*;
pub use query::*;

use everything_structures::Structure;

use crate::objects::StructureExt;

#[derive(Clone)]
pub struct Knowledge(Structure);

impl Knowledge {
    #[inline]
    pub fn new(structure: Structure) -> Option<Self> {
        structure.is_knowledge().then_some(Self(structure))
    }

    #[must_use]
    #[inline]
    pub fn structure(&self) -> &Structure {
        &self.0
    }
}

impl TryFrom<Structure> for Knowledge {
    type Error = ();

    fn try_from(value: Structure) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl Into<Structure> for Knowledge {
    fn into(self) -> Structure {
        self.0
    }
}
