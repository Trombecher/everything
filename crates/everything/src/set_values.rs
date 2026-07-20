use everything_objects::{
    Abstract, AnyCompositeProperties, Composite, CompositeProperties, Object, Property,
};

use crate::{
    ctx::EvaluationContext,
    ext::{ObjectExt, PropertyExt},
    query::{
        QuerySubjects, QuerySubjectsAndTags, QuerySubjectsAndValues, QueryTags, QueryTagsAndValues,
        QueryValues, SubjectAndTag, SubjectAndValue,
    },
};

use crate::ext::AbstractExt;

/// An iterator over all set values of a given Composite.
#[derive(Clone)]
pub struct CompositeSetValues {
    properties: CompositeProperties,
}

impl CompositeSetValues {
    pub fn new(composite: &Composite) -> Self {
        Self {
            properties: match composite {
                Composite::Empty
                | Composite::Integer(_)
                | Composite::Bytes(_)
                | Composite::Text(_)
                | Composite::Byte(_)
                | Composite::Character(_) => {
                    // These do not have set values.
                    Composite::Empty.properties()
                }
                Composite::Any(any) => {
                    CompositeProperties::Any(AnyCompositeProperties::new_starting_from_tag(
                        any.clone(),
                        Abstract::CONTAINS.into(),
                    ))
                }
            },
        }
    }
}

impl Iterator for CompositeSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        // We don't use find_map here because we
        // don't want to continue if the tag does not match
        // CONTAINS (iterator is sorted).

        self.properties.next().and_then(|property| {
            (property.tag == Abstract::CONTAINS.into()).then_some(property.value)
        })
    }
}

impl std::fmt::Debug for CompositeSetValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.clone()).finish()
    }
}

#[derive(Clone)]
pub enum ObjectOrSetValues {
    Object(Object),
    SetValues(SetValues),
}

impl From<Object> for ObjectOrSetValues {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl From<SetValues> for ObjectOrSetValues {
    fn from(value: SetValues) -> Self {
        Self::SetValues(value)
    }
}

impl std::fmt::Debug for ObjectOrSetValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(object) => object.fmt(f),
            Self::SetValues(iter) => iter.fmt(f),
        }
    }
}

impl ObjectOrSetValues {
    /// Interprets the [`Self::AxiomaticQueryValues`] variant as an iterator
    /// over the set items and returns an iterator over set items.
    #[inline]
    pub fn set_values(&self, knowledge: &Composite) -> SetValues {
        match self {
            Self::SetValues(values) => values.clone(),
            Self::Object(eager) => SetValues::QueryValues(eager.set_values(knowledge)),
        }
    }

    /// Converts `self` into an object.
    ///
    /// * If `self` was [`LazyObject::Eager`], it just returns that object.
    /// * If `self` was [`LazyObject::LazySetValues`], it collects all
    ///   values into a set and returns that.
    pub fn into_object(self) -> Object {
        match self {
            Self::Object(object) => object,
            Self::SetValues(iterator) => iterator.collect_to_set().into(),
        }
    }

    /// Determines if `self` is "truthy", i.e. iff it has at
    /// least one property.
    pub fn is_truthy(&mut self, knowledge: &Composite) -> bool {
        match self {
            ObjectOrSetValues::Object(object) => object.is_truthy(knowledge),
            ObjectOrSetValues::SetValues(iter) => iter.next().is_some(),
        }
    }
}

#[derive(Clone)]
pub enum SetValues {
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

    /// Maps every value of a given set with a mapper function.
    Map {
        knowledge: Composite,
        set: Box<Self>,
        /// This function is captured.
        mapper_function: Object,
    },

    /// Retains all values for that the filter function returns a truthy value.
    Filter {
        knowledge: Composite,
        set: Box<Self>,
        /// This function is captured.
        filter_function: Object,
    },
}

impl std::fmt::Debug for SetValues {
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
            Self::QuerySubjects(subjects) => subjects.next(),
            Self::QueryTags(tags) => tags.next(),
            Self::QuerySubjectsAndTags(subjects_and_tags) => {
                subjects_and_tags
                    .next()
                    .map(|SubjectAndTag { subject, tag }| {
                        Composite::new(&mut [
                            Property::new_statement_subject(subject),
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
                        Property::new_statement_subject(subject),
                        Property::new_statement_value(value),
                    ])
                    .into()
                })
            }
            Self::Map {
                knowledge,
                set,
                mapper_function,
            } => set.next().map(|item| {
                mapper_function
                    .call(
                        knowledge,
                        &[ObjectOrSetValues::Object(item)],
                        &mut EvaluationContext::default(),
                    )
                    .into_object()
            }),
            Self::Filter {
                knowledge,
                set,
                filter_function,
            } => loop {
                // Eat until some match is found.

                let item = set.next()?;

                if filter_function
                    .call(
                        knowledge,
                        &[ObjectOrSetValues::Object(item.clone())],
                        &mut Default::default(),
                    )
                    .is_truthy(knowledge)
                {
                    break Some(item);
                }
            },
        }
    }
}
