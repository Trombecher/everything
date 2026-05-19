use everything_structures::{Object, Structure};

use crate::{ext::ObjectExt, query::StructureSetValues};

#[derive(Debug, Clone, PartialEq)]
pub struct SubjectAndTag {
    pub subject: Object,
    pub tag: Object,
}

#[derive(Clone, Debug)]
pub struct QuerySubjectsAndTagsAxiomatically {
    statements_from_knowledge: StructureSetValues,
    value: Object,
}

impl QuerySubjectsAndTagsAxiomatically {
    pub fn new(knowledge: &Structure, value: Object) -> Self {
        Self {
            statements_from_knowledge: StructureSetValues::new(knowledge),
            value,
        }
    }
}

impl Iterator for QuerySubjectsAndTagsAxiomatically {
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
