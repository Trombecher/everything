use everything_objects::{Composite, Object};

use crate::{
    ObjectOrSetValues,
    ext::{CompositeExt, KnowledgeError, ObjectExt},
    query::{
        QueryExists, QuerySubjects, QuerySubjectsAndTags, QuerySubjectsAndValues, QueryTags,
        QueryTagsAndValues, QueryValues,
    },
};

#[derive(Clone)]
pub struct Knowledge(Composite);

impl Knowledge {
    /// Creates a new knowledge from a Composite.
    ///
    /// * Returns `Ok(...)` if the given Composite is valid knowledge;
    /// * `Err(...)` otherwise.
    #[inline]
    pub fn new(composite: Composite) -> Result<Self, KnowledgeError> {
        composite.is_knowledge().map(|()| Self(composite))
    }

    /// Returns the underlying Composite.
    #[must_use]
    #[inline]
    pub fn composite(&self) -> &Composite {
        &self.0
    }

    #[inline]
    pub fn query_values(&self, subject: Object, tag: Object) -> QueryValues {
        QueryValues::new(self.composite(), subject, tag.clone())
    }

    #[inline]
    pub fn query_subjects(&self, tag: Object, value: Object) -> QuerySubjects {
        QuerySubjects::new(&self.0, tag, value)
    }

    #[inline]
    pub fn query_subjects_and_values(&self, tag: Object) -> QuerySubjectsAndValues {
        QuerySubjectsAndValues::new(&self.0, tag)
    }

    #[inline]
    pub fn query_tags(&self, subject: Object, value: Object) -> QueryTags {
        QueryTags::new(&self.0, subject, value)
    }

    #[inline]
    pub fn query_tags_and_values(&self, subject: Object) -> QueryTagsAndValues {
        QueryTagsAndValues::new(&self.0, subject)
    }

    #[inline]
    pub fn query_subjects_and_tags(&self, value: Object) -> QuerySubjectsAndTags {
        QuerySubjectsAndTags::new(&self.0, value)
    }

    #[inline]
    pub fn query_exists(&self, subject: Object, tag: Object, value: Object) -> bool {
        QueryExists::new(&self.0, subject, tag, value)
    }

    /// Evaluates the given node under `self`. If you need more control,
    /// use [`ObjectExt::evaluate`].
    #[inline]
    pub fn evaluate(&self, node: Object) -> ObjectOrSetValues {
        node.evaluate(&self.0, &mut Default::default())
    }

    /// Calls the callee with a parameter under `self`. If you need more
    /// control of this call (or need multiple arguments), use [`ObjectExt::call`].
    #[inline]
    pub fn call(&self, callee: Object, with: Object) -> ObjectOrSetValues {
        callee.call(
            &self.0,
            &[ObjectOrSetValues::Object(with)],
            &mut Default::default(),
        )
    }
}

impl TryFrom<Composite> for Knowledge {
    type Error = KnowledgeError;

    fn try_from(value: Composite) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Knowledge> for Composite {
    fn from(value: Knowledge) -> Self {
        value.0
    }
}
