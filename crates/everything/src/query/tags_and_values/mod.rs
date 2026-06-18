use everything_objects::{Composite, CompositeProperties, Object, Property};

use crate::{CompositeSetValues, ext::ObjectExt};

#[derive(Clone)]
pub struct QueryTagsAndValues {
    properties_from_subject: CompositeProperties,
    statements_from_knowledge: CompositeSetValues,
    subject: Object,
}

impl QueryTagsAndValues {
    pub fn new(knowledge: &Composite, subject: Object) -> Self {
        let properties_from_subject = match &subject {
            Object::Abstract(_) => CompositeProperties::Empty,
            Object::Composite(composite) => composite.properties(),
        };

        let statements_from_knowledge = CompositeSetValues::new(knowledge);

        Self {
            properties_from_subject,
            statements_from_knowledge,
            subject,
        }
    }
}

impl Iterator for QueryTagsAndValues {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.properties_from_subject.next() {
            return Some(next);
        }

        self.statements_from_knowledge.find_map(|statement| {
            if self.subject != statement.intrinsic_statement_subject().unwrap() {
                // Skip all statements from involving subjects.
                return None;
            }

            let tag = statement.intrinsic_statement_tag().unwrap();
            let value = statement.intrinsic_statement_value().unwrap();

            Some(Property { tag, value })
        })
    }
}
