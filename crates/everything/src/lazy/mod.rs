use everything_structures::{Object, Property, Structure};

use crate::{
    ext::{ObjectExt, PropertyExt},
    query::AxiomaticQueryValues,
};

/// Either an object or an iterator over values.
#[derive(Clone)]
pub enum LazyObject {
    Eager(Object),

    /// An iterator over all set values.
    LazySetValues(AxiomaticQueryValues),
}

impl From<Object> for LazyObject {
    fn from(value: Object) -> Self {
        Self::Eager(value)
    }
}

impl From<AxiomaticQueryValues> for LazyObject {
    fn from(value: AxiomaticQueryValues) -> Self {
        Self::LazySetValues(value)
    }
}

impl std::fmt::Debug for LazyObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eager(object) => object.fmt(f),
            Self::LazySetValues(iter) => f
                .debug_set()
                .entries(iter.clone().map(Property::new_contains))
                .finish(),
        }
    }
}

impl LazyObject {
    /// Interprets the [`Self::AxiomaticQueryValues`] variant as an iterator
    /// over the set items and returns an iterator over set items.
    #[inline]
    pub fn set_values(&self, knowledge: &Structure) -> AxiomaticQueryValues {
        match self {
            Self::LazySetValues(values) => values.clone(),
            Self::Eager(eager) => eager.set_values(knowledge),
        }
    }

    pub fn into_set(self) -> Object {
        match self {
            Self::Eager(object) => object,
            Self::LazySetValues(iterator) => iterator.collect_to_set().into(),
        }
    }

    pub fn is_truthy(&mut self, knowledge: &Structure) -> bool {
        match self {
            LazyObject::Eager(object) => object.is_truthy(knowledge),
            LazyObject::LazySetValues(iter) => iter.next().is_some(),
        }
    }
}
