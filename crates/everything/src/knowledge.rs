use everything_structures::{Object, Structure};

use crate::{
    ext::{KnowledgeError, StructureExt},
    query::{self, QueryValuesResult},
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

    /// Query the knowledge. Query format:
    ///
    /// ```plain
    /// (subject, tag) -> (value)
    /// ```
    #[inline]
    #[must_use]
    pub fn query_values<'knowledge: 'item, 'subject: 'item, 'item>(
        &'knowledge self,
        subject: &'subject Object,
        tag: Object,
    ) -> QueryValuesResult<'knowledge, 'subject, 'item> {
        query::values(self.structure(), subject, tag, &mut Default::default())
    }
}

impl TryFrom<Structure> for Knowledge {
    type Error = KnowledgeError;

    fn try_from(value: Structure) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Knowledge> for Structure {
    fn from(value: Knowledge) -> Self {
        value.0
    }
}
