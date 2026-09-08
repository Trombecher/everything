use everything_objects::{
    Abstract, CompositeProperties, CompositeTags, CompositeValues, Object, Property,
};

use crate::{
    base::AXIOMATIC_AXIOMATIC_CONSTRAINT,
    statements::{
        AbstractExtendedProperties, AbstractProperties, AbstractTags, AbstractValues,
        IndexedStatements, Statement,
    },
};

#[derive(Clone)]
pub enum QueryTags {
    Abstract(AbstractTags),
    Composite(CompositeTags),
}

impl Iterator for QueryTags {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Abstract(iter) => iter.next(),
            Self::Composite(iter) => iter.next(),
        }
    }
}

#[derive(Clone)]
pub struct QuerySubjects {
    pub(super) indexed_statements: IndexedStatements,
    pub(super) tag: Object,
    pub(super) value: Object,
}

impl Iterator for QuerySubjects {
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

#[derive(Clone)]
pub enum QueryTagsAndValues {
    Abstract(AbstractProperties),
    Composite(CompositeProperties),
}

impl Iterator for QueryTagsAndValues {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Abstract(iter) => iter.next(),
            Self::Composite(iter) => iter.next(),
        }
    }
}

#[derive(Clone)]
pub enum QueryValues {
    AxiomaticAxiomaticConstraint,
    Abstract(AbstractValues),
    Composite(CompositeValues),
}

impl core::fmt::Debug for QueryValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.clone()).finish()
    }
}

impl Iterator for QueryValues {
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

#[derive(Debug, Clone, PartialEq)]
pub struct SubjectAndTag {
    pub subject: Abstract,
    pub tag: Object,
}

#[derive(Clone)]
pub struct QuerySubjectsAndTags {
    pub(super) indexed_statements: IndexedStatements,
    pub(super) value: Object,
    pub(super) current_subject_with_properties: Option<(Abstract, AbstractProperties)>,
}

impl QuerySubjectsAndTags {
    fn find_next_tag(
        abstract_properties: &mut AbstractProperties,
        value: &Object,
    ) -> Option<Object> {
        abstract_properties.find_map(|property| {
            if &property.value == value {
                Some(property.tag)
            } else {
                None
            }
        })
    }
}

impl Iterator for QuerySubjectsAndTags {
    type Item = SubjectAndTag;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((subject, properties)) = &mut self.current_subject_with_properties {
            // Continue from previous iterator.

            if let Some(tag) = Self::find_next_tag(properties, &self.value) {
                return Some(SubjectAndTag {
                    subject: *subject,
                    tag,
                });
            }

            self.current_subject_with_properties = None;
        }

        // Either the iterator does not exist, or it is exhausted.

        for (subject, properties) in &mut self.indexed_statements {
            let mut abstract_properties = AbstractProperties {
                extended: properties.clone().into_iter(),
            };

            if let Some(tag) = Self::find_next_tag(&mut abstract_properties, &self.value) {
                // Save the iterator state.
                self.current_subject_with_properties = Some((subject, abstract_properties));

                return Some(SubjectAndTag { subject, tag });
            }

            // No tag found, we have to continue to the next subject.
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectAndValue {
    pub subject: Abstract,
    pub value: Object,
}

#[derive(Clone)]
pub struct QuerySubjectsAndValues {
    pub(super) indexed_statements: IndexedStatements,
    pub(super) tag: Object,
    pub(super) current_subject_with_properties: Option<(Abstract, AbstractProperties)>,
}

impl QuerySubjectsAndValues {
    fn find_next_value(
        abstract_properties: &mut AbstractProperties,
        tag: &Object,
    ) -> Option<Object> {
        abstract_properties.find_map(|property| {
            if &property.tag == tag {
                Some(property.value)
            } else {
                None
            }
        })
    }
}

impl Iterator for QuerySubjectsAndValues {
    type Item = SubjectAndValue;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((subject, properties)) = &mut self.current_subject_with_properties {
            // Continue from previous iterator.

            if let Some(value) = Self::find_next_value(properties, &self.tag) {
                return Some(SubjectAndValue {
                    subject: *subject,
                    value,
                });
            }

            self.current_subject_with_properties = None;
        }

        // Either the iterator does not exist, or it is exhausted.

        for (subject, properties) in &mut self.indexed_statements {
            let mut abstract_properties = AbstractProperties {
                extended: properties.clone().into_iter(),
            };

            if let Some(value) = Self::find_next_value(&mut abstract_properties, &self.tag) {
                // Save the iterator state.
                self.current_subject_with_properties = Some((subject, abstract_properties));

                return Some(SubjectAndValue { subject, value });
            }

            // No value found, we have to continue to the next subject.
        }

        None
    }
}

#[derive(Clone)]
pub struct StatementsIter {
    pub(super) indexed_statements: IndexedStatements,
    pub(super) current_subject_with_properties: Option<(Abstract, AbstractExtendedProperties)>,
}

impl Iterator for StatementsIter {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((subject, properties)) = &mut self.current_subject_with_properties {
            if let Some(property) = properties.next() {
                return Some(Statement {
                    subject: *subject,
                    property,
                });
            }

            // Exhausted
            self.current_subject_with_properties = None;
        }

        for (subject, properties) in &mut self.indexed_statements {
            let mut properties_iter = properties.clone().into_iter();

            if let Some(property) = properties_iter.next() {
                // This should happen immediately, since the sets are non-empty.
                self.current_subject_with_properties = Some((subject, properties_iter));

                return Some(Statement { subject, property });
            }
        }

        None
    }
}
