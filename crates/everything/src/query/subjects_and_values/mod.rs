use everything_structures::{Object, Structure};

use crate::{StructureSetValues, ext::ObjectExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectAndValue {
    pub subject: Object,
    pub value: Object,
}

#[derive(Clone, Debug)]
pub struct QuerySubjectsAndValues {
    statements_from_knowledge: StructureSetValues,
    tag: Object,
}

impl QuerySubjectsAndValues {
    pub fn new(knowledge: &Structure, tag: Object) -> Self {
        Self {
            statements_from_knowledge: StructureSetValues::new(knowledge),
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
