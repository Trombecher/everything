use everything_objects::{
    Abstract, CompositeProperties, CompositeTags, CompositeValues, Object, Property,
};

use crate::{
    base::AXIOMATIC_AXIOMATIC_CONSTRAINT,
    statements::{AbstractProperties, AbstractTags, AbstractValues, IndexedStatements},
};

#[derive(Clone)]
pub enum QueryTags2 {
    Abstract(AbstractTags),
    Composite(CompositeTags),
}

impl Iterator for QueryTags2 {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Abstract(iter) => iter.next(),
            Self::Composite(iter) => iter.next(),
        }
    }
}

#[derive(Clone)]
pub struct QuerySubjects2 {
    pub(super) indexed_statements: IndexedStatements,
    pub(super) tag: Object,
    pub(super) value: Object,
}

impl Iterator for QuerySubjects2 {
    type Item = Abstract;

    fn next(&mut self) -> Option<Self::Item> {
        self.indexed_statements
            .find_map(|(abstract_object, properties)| {
                if properties
                    .iter()
                    .any(|property| property.tag == self.tag && property.value == self.value)
                {
                    Some(abstract_object)
                } else {
                    None
                }
            })
    }
}

pub enum QueryTagsAndValues2 {
    Abstract(AbstractProperties),
    Composite(CompositeProperties),
}

impl Iterator for QueryTagsAndValues2 {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Abstract(iter) => iter.next(),
            Self::Composite(iter) => iter.next(),
        }
    }
}

#[derive(Clone)]
pub enum QueryValues2 {
    AxiomaticAxiomaticConstraint,
    Abstract(AbstractValues),
    Composite(CompositeValues),
}

impl Iterator for QueryValues2 {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::AxiomaticAxiomaticConstraint => {
                *self = Self::Composite(CompositeValues::None);
                Some(AXIOMATIC_AXIOMATIC_CONSTRAINT.clone())
            }
            Self::Abstract(iter) => iter.next(),
            Self::Composite(iter) => iter.next(),
        }
    }
}
