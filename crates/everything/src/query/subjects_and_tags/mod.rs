use everything_objects::{Composite, Object};

use crate::{CompositeSetValues, ext::ObjectExt};

#[derive(Debug, Clone, PartialEq)]
pub struct SubjectAndTag {
    pub subject: Object,
    pub tag: Object,
}

#[derive(Clone, Debug)]
pub struct QuerySubjectsAndTags {
    statements_from_knowledge: CompositeSetValues,
    value: Object,
}

impl QuerySubjectsAndTags {
    pub fn new(knowledge: &Composite, value: Object) -> Self {
        Self {
            statements_from_knowledge: CompositeSetValues::new(knowledge),
            value,
        }
    }
}

impl Iterator for QuerySubjectsAndTags {
    type Item = SubjectAndTag;

    fn next(&mut self) -> Option<Self::Item> {
        self.statements_from_knowledge.find_map(|statement| {
            if statement.intrinsic_statement_value().unwrap() != self.value {
                return None;
            }

            let subject = statement.intrinsic_statement_subject().unwrap();
            let tag = statement.intrinsic_statement_tag().unwrap();

            Some(SubjectAndTag { subject, tag })
        })
    }
}
