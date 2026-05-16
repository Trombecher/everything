use everything_structures::{Object, Structure};

use crate::{
    LazyObject,
    ext::{KnowledgeError, StructureExt},
    query::{self, QuerySubjectsAndValuesAxiomatically, QuerySubjectsAxiomatically},
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
    pub fn query_values(&self, subject: Object, tag: Object) -> LazyObject {
        query::values(self.structure(), subject, tag, &mut Default::default())
    }

    #[inline]
    pub fn query_subjects_axiomatically(
        &self,
        tag: Object,
        value: Object,
    ) -> QuerySubjectsAxiomatically {
        query::subjects_axiomatically(&self.0, tag, value)
    }

    #[inline]
    pub fn query_subjects_and_values_axiomatically(
        &self,
        tag: Object,
    ) -> QuerySubjectsAndValuesAxiomatically {
        query::subjects_and_values_axiomatically(&self.0, tag)
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
