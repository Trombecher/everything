use everything_objects::{Composite, Object};

use crate::{CompositeSetValues, ext::ObjectExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectAndValue {
    pub subject: Object,
    pub value: Object,
}

#[derive(Clone, Debug)]
pub struct QuerySubjectsAndValues {
    statements_from_knowledge: CompositeSetValues,
    tag: Object,
}

impl QuerySubjectsAndValues {
    pub fn new(knowledge: &Composite, tag: Object) -> Self {
        Self {
            statements_from_knowledge: CompositeSetValues::new(knowledge),
            tag,
        }
    }
}

impl Iterator for QuerySubjectsAndValues {
    type Item = SubjectAndValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.statements_from_knowledge.find_map(|statement| {
            if self.tag != statement.intrinsic_statement_tag().unwrap() {
                return None;
            }

            let subject = statement.intrinsic_statement_subject().unwrap();
            let value = statement.intrinsic_statement_value().unwrap();

            Some(SubjectAndValue { subject, value })
        })
    }
}
