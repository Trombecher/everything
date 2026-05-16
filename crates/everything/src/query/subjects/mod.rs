use everything_structures::{Abstract, Object, Structure, StructureProperties};

use crate::ext::{AbstractExt, ObjectExt};

pub fn subjects_axiomatically(
    knowledge: &Structure,
    tag: Object,
    value: Object,
) -> QuerySubjectsAxiomatically {
    // Knowledge will almost never be a specialization
    // which means that this properties() call is cheap.

    QuerySubjectsAxiomatically {
        knowledge_properties: knowledge.properties(),
        tag,
        value,
    }
}

/// An iterator over all subjects in the knowledge
/// that have a given tag and value.
#[derive(Clone, Debug)]
pub struct QuerySubjectsAxiomatically {
    knowledge_properties: StructureProperties,
    tag: Object,
    value: Object,
}

impl Iterator for QuerySubjectsAxiomatically {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let property = self.knowledge_properties.next()?;

            if property.tag != Object::Abstract(Abstract::CONTAINS) {
                continue;
            }

            let statement = property.value.structure().unwrap();
            let statement_tag = statement
                .values(Abstract::STATEMENT_TAG.into())
                .next()
                .unwrap();

            if statement_tag != self.tag {
                continue;
            }

            let statement_value = statement
                .values(Abstract::STATEMENT_VALUE.into())
                .next()
                .unwrap();

            if statement_value != self.value {
                continue;
            }

            break Some(
                statement
                    .values(Abstract::STATEMENT_SUBJECT.into())
                    .next()
                    .unwrap(),
            );
        }
    }
}
