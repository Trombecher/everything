use everything_structures::{Object, Structure};

use crate::{StructureSetValues, ext::ObjectExt};

/// An iterator over all subjects in the knowledge
/// that have a given tag and value.
#[derive(Clone, Debug)]
pub struct QuerySubjectsAxiomatically {
    statements_from_knowledge: StructureSetValues,
    tag: Object,
    value: Object,
}

impl QuerySubjectsAxiomatically {
    #[inline]
    pub fn new(knowledge: &Structure, tag: Object, value: Object) -> Self {
        Self {
            statements_from_knowledge: StructureSetValues::new(knowledge),
            tag,
            value,
        }
    }
}

impl Iterator for QuerySubjectsAxiomatically {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.statements_from_knowledge.find_map(|statement| {
            if statement.intrinsic_statement_tag().unwrap() != self.tag
                || statement.intrinsic_statement_value().unwrap() != self.value
            {
                return None;
            }

            Some(statement.intrinsic_statement_subject().unwrap())
        })
    }
}
