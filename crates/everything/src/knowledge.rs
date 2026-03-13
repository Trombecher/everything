use everything_structures::{Object, Structure};

use crate::{
    ext::StructureExt,
    query::{QueryValuesResult, query_values},
};

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

    /// Query the knowledge.
    #[inline]
    #[must_use]
    pub fn query_values<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item>(
        &'knowledge self,
        subject: &'subject Object,
        tag: &'tag Object,
    ) -> QueryValuesResult<'knowledge, 'subject, 'tag, 'item> {
        query_values(self.structure(), subject, tag)
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
