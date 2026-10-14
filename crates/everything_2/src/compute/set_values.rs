use std::fmt::Debug;

use crate::{knowledge::Knowledge, model::Object};

/// Either an object or an iterator over set values.
#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ObjectOrExtendedSetValues {
    Object(Object),
    ExtendedSetValues(ExtendedSetValues),
}

impl From<Object> for ObjectOrExtendedSetValues {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl From<ExtendedSetValues> for ObjectOrExtendedSetValues {
    fn from(value: ExtendedSetValues) -> Self {
        Self::ExtendedSetValues(value)
    }
}

impl std::fmt::Debug for ObjectOrExtendedSetValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(object) => Debug::fmt(object, f),
            Self::ExtendedSetValues(iter) => Debug::fmt(iter, f),
        }
    }
}

impl ObjectOrExtendedSetValues {
    /// Interprets the [`Self::AxiomaticQueryValues`] variant as an iterator
    /// over the set items and returns an iterator over set items.
    #[inline]
    #[must_use]
    pub fn extended_set_values(&self, statements: &Knowledge) -> ExtendedSetValues {
        match self {
            Self::ExtendedSetValues(values) => values.clone(),
            Self::Object(eager) => ExtendedSetValues::QueryValues(eager.set_values(statements)),
        }
    }

    /// Converts `self` into an object.
    ///
    /// * If `self` was [`LazyObject::Eager`], it just returns that object.
    /// * If `self` was [`LazyObject::LazySetValues`], it collects all
    ///   values into a set and returns that.
    #[must_use]
    pub fn into_object(self) -> Object {
        match self {
            Self::Object(object) => object,
            Self::SetValues(iterator) => iterator.collect_to_set().into(),
        }
    }

    /// Determines if `self` is "truthy", i.e. iff it has at
    /// least one property.
    pub fn is_truthy(&mut self, statements: &Knowledge) -> bool {
        match self {
            ObjectOrSetValues::Object(object) => object.is_truthy(statements),
            ObjectOrSetValues::SetValues(iter) => iter.next().is_some(),
        }
    }
}

/// An iterator over the elements of an object.
///
/// Certain variants are lazy meaning they will contain nested iterators
/// to prevent unnecessary allocations.
///
/// This iterator may yield duplicate objects,
/// however some variants only yield unique objects.
#[derive(Clone)]
pub enum ExtendedSetValues {
    /// Iterator over values of an object.
    QueryValues(QueryValues),

    /// Chains two iterators.
    Union { left: Box<Self>, right: Box<Self> },

    /// Iterator over subjects for a given tag and value.
    QuerySubjects(QuerySubjects),

    /// Iterator over subjects and values for a given tag.
    QuerySubjectsAndValues(QuerySubjectsAndValues),

    /// Iterator over tags and values for a given subject.
    QueryTagsAndValues(QueryTagsAndValues),

    /// Iterator over subjects and tags for a given value.
    QuerySubjectsAndTags(QuerySubjectsAndTags),

    /// Iterator over tags for a given subject and value.
    QueryTags(QueryTags),

    /// Iterator over all statements.
    Statements(StatementsIter),

    /// Maps every value of a given set with a mapper function.
    Map {
        knowledge: Knowledge,
        set: Box<Self>,
        /// This function is captured.
        mapper_function: Object,
    },

    /// Retains all values for that the filter function returns a truthy value.
    Filter {
        knowledge: Knowledge,
        set: Box<Self>,
        /// This function is captured.
        filter_function: Object,
    },
}

impl Debug for ExtendedSetValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(self.clone().map(Property::new_contains))
            .finish()
    }
}

impl SetValues {
    pub fn collect_to_set(self) -> Composite {
        let mut properties: Vec<_> = self.map(Property::new_contains).collect();
        Composite::new(&mut properties)
    }

    /// Counts the (remaining) set values of this iterator.
    /// It dedups
    #[must_use]
    pub fn correct_count(self) -> usize {
        match self {
            SetValues::QueryValues(axiomatic_query_values) => axiomatic_query_values.count(),
            iterator => {
                // This is expensive, no?
                let mut vec = iterator.collect::<Vec<_>>();
                vec.partition_dedup().0.len()
            }
        }
    }
}

impl Iterator for SetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::QueryValues(values) => values.next(),
            Self::Union { left, right } => left.next().or_else(|| right.next()),
            Self::QuerySubjects(subjects) => subjects.next().map(Into::into),
            Self::QueryTags(tags) => tags.next(),
            Self::QuerySubjectsAndTags(subjects_and_tags) => {
                subjects_and_tags
                    .next()
                    .map(|SubjectAndTag { subject, tag }| {
                        Composite::new(&mut [
                            Property::new_statement_subject(Object::Abstract(subject)),
                            Property::new_statement_tag(tag),
                        ])
                        .into()
                    })
            }
            Self::QueryTagsAndValues(tags_and_values) => {
                tags_and_values.next().map(|Property { tag, value }| {
                    Composite::new(&mut [
                        Property::new_statement_tag(tag),
                        Property::new_statement_value(value),
                    ])
                    .into()
                })
            }
            Self::QuerySubjectsAndValues(iter) => {
                iter.next().map(|SubjectAndValue { subject, value }| {
                    Composite::new(&mut [
                        Property::new_statement_subject(Object::Abstract(subject)),
                        Property::new_statement_value(value),
                    ])
                    .into()
                })
            }
            Self::Statements(iter) => iter
                .next()
                .map(|statements| statements.to_composite().into()),
            Self::Map {
                statements,
                set,
                mapper_function,
            } => set.next().map(|item| {
                mapper_function
                    .call(
                        statements,
                        &[ObjectOrSetValues::Object(item)],
                        &mut EvaluationContext::default(),
                    )
                    .into_object()
            }),
            Self::Filter {
                statements,
                set,
                filter_function,
            } => loop {
                // Eat until some match is found.

                let item = set.next()?;

                if filter_function
                    .call(
                        statements,
                        &[ObjectOrSetValues::Object(item.clone())],
                        &mut EvaluationContext::default(),
                    )
                    .is_truthy(statements)
                {
                    break Some(item);
                }
            },
        }
    }
}
