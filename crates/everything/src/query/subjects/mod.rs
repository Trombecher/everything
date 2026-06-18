use everything_objects::{Composite, Object};

use crate::{CompositeSetValues, ext::ObjectExt};

/// An iterator over all subjects in the knowledge
/// that have a given tag and value.
#[derive(Clone, Debug)]
pub struct QuerySubjects {
    statements_from_knowledge: CompositeSetValues,
    tag: Object,
    value: Object,
}

impl QuerySubjects {
    #[inline]
    pub fn new(knowledge: &Composite, tag: Object, value: Object) -> Self {
        Self {
            statements_from_knowledge: CompositeSetValues::new(knowledge),
            tag,
            value,
        }
    }
}

impl Iterator for QuerySubjects {
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
