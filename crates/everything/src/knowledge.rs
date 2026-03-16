use everything_structures::{Object, Structure};

use crate::{
    ext::{KnowledgeError, StructureExt},
    query::{QueryValuesResult, query_values},
};

#[derive(Clone)]
pub struct Knowledge(Structure);

impl Knowledge {
    #[inline]
    pub fn new(structure: Structure) -> Result<Self, KnowledgeError> {
        structure.is_knowledge().map(|()| Self(structure))
    }

    #[must_use]
    #[inline]
    pub fn structure(&self) -> &Structure {
        &self.0
    }

    /// Query the knowledge.
    #[inline]
    #[must_use]
    pub fn query_values<'knowledge: 'item, 'subject: 'item, 'item>(
        &'knowledge self,
        subject: &'subject Object,
        tag: Object,
    ) -> QueryValuesResult<'knowledge, 'subject, 'item> {
        query_values(self.structure(), subject, tag, &mut Default::default())
    }
}

impl TryFrom<Structure> for Knowledge {
    type Error = KnowledgeError;

    fn try_from(value: Structure) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Into<Structure> for Knowledge {
    fn into(self) -> Structure {
        self.0
    }
}
