use everything_structures::{Object, Property, Structure, StructureProperties};

use crate::{StructureSetValues, ext::ObjectExt};

#[derive(Clone)]
pub struct QueryTagsAndValues {
    properties_from_subject: StructureProperties,
    statements_from_knowledge: StructureSetValues,
    subject: Object,
}

impl QueryTagsAndValues {
    pub fn new(knowledge: &Structure, subject: Object) -> Self {
        let properties_from_subject = match &subject {
            Object::Abstract(_) => StructureProperties::Empty,
            Object::Structure(structure) => structure.properties(),
        };

        let statements_from_knowledge = StructureSetValues::new(knowledge);

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
