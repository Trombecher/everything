use everything_structures::{Abstract, Object, Structure, StructureProperties};

use crate::ext::{AbstractExt, ObjectExt};

pub fn subjects_and_values_axiomatically(
    knowledge: &Structure,
    tag: Object,
) -> QuerySubjectsAndValuesAxiomatically {
    QuerySubjectsAndValuesAxiomatically {
        knowledge_properties: knowledge.properties(),
        knowledge: knowledge.clone(),
        tag,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectAndValue {
    pub subject: Object,
    pub value: Object,
}

#[derive(Clone, Debug)]
pub struct QuerySubjectsAndValuesAxiomatically {
    // TODO: debate this
    knowledge: Structure,
    knowledge_properties: StructureProperties,
    tag: Object,
}

impl Iterator for QuerySubjectsAndValuesAxiomatically {
    type Item = SubjectAndValue;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let property = self.knowledge_properties.next()?;

            if property.tag != Object::Abstract(Abstract::CONTAINS) {
                continue;
            }

            match property.value.statement_tag(&self.knowledge) {
                Some(statement_tag) if statement_tag == self.tag => {}
                _ => continue,
            }

            let Some(subject) = property.value.statement_subject(&self.knowledge) else {
                continue;
            };

            let Some(value) = property.value.statement_value(&self.knowledge) else {
                continue;
            };

            break Some(SubjectAndValue { subject, value });
        }
    }
}
