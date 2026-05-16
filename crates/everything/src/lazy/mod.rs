use everything_structures::{Object, Property, Structure};

use crate::{
    ext::{ObjectExt, PropertyExt},
    query::{AxiomaticQueryValues, QueryValuesResult},
};

/// Either an object or an iterator over values.
pub enum ObjectOrAxiomaticQueryValues<'knowledge, 'subject> {
    Object(Object),

    /// An iterator over all set values.
    AxiomaticQueryValues(AxiomaticQueryValues<'knowledge, 'subject>),
}

impl From<Object> for ObjectOrAxiomaticQueryValues<'_, '_> {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl<'knowledge, 'subject> From<AxiomaticQueryValues<'knowledge, 'subject>>
    for ObjectOrAxiomaticQueryValues<'knowledge, 'subject>
{
    fn from(value: AxiomaticQueryValues<'knowledge, 'subject>) -> Self {
        Self::AxiomaticQueryValues(value)
    }
}

impl<'knowledge, 'subject> From<QueryValuesResult<'knowledge, 'subject>>
    for ObjectOrAxiomaticQueryValues<'knowledge, 'subject>
{
    fn from(value: QueryValuesResult<'knowledge, 'subject>) -> Self {
        match value {
            QueryValuesResult::Axiomatic(axiomatic_query_values) => {
                Self::AxiomaticQueryValues(axiomatic_query_values)
            }
            QueryValuesResult::ComputationResult(object) => Self::Object(object),
        }
    }
}

impl std::fmt::Debug for ObjectOrAxiomaticQueryValues<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(object) => object.fmt(f),
            Self::AxiomaticQueryValues(iter) => f
                .debug_set()
                .entries(iter.clone().map(Property::new_contains))
                .finish(),
        }
    }
}

impl<'knowledge, 'subject> ObjectOrAxiomaticQueryValues<'knowledge, 'subject> {
    /// Interprets the [`Self::AxiomaticQueryValues`] variant as an iterator
    /// over the set items and returns an iterator over set items.
    #[inline]
    pub fn set_values(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> AxiomaticQueryValues<'knowledge, 'subject> {
        match self {
            Self::AxiomaticQueryValues(values) => values.clone(),
            Self::Object(eager) => eager.set_values(knowledge),
        }
    }

    pub fn into_set(self) -> Object {
        match self {
            Self::Object(object) => object,
            Self::AxiomaticQueryValues(iterator) => iterator.collect_to_set().into(),
        }
    }

    pub fn is_truthy(&mut self, knowledge: &Structure) -> bool {
        match self {
            ObjectOrAxiomaticQueryValues::Object(object) => object.is_truthy(knowledge),
            ObjectOrAxiomaticQueryValues::AxiomaticQueryValues(iter) => iter.next().is_some(),
        }
    }
}
