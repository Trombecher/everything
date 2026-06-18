use everything_objects::{Composite, CompositeTags, Object};

use crate::{CompositeSetValues, ext::ObjectExt};

#[derive(Clone, Debug)]
pub struct QueryTags {
    tags_from_subject: CompositeTags,
    statements_from_knowledge: CompositeSetValues,
    subject: Object,
    value: Object,
}

impl QueryTags {
    pub fn new(knowledge: &Composite, subject: Object, value: Object) -> Self {
        let tags_from_subject = match &subject {
            Object::Abstract(_) => CompositeTags::None,
            Object::Composite(subject) => subject.tags(value.clone()),
        };

        Self {
            tags_from_subject,
            statements_from_knowledge: CompositeSetValues::new(knowledge),
            subject,
            value,
        }
    }
}

impl Iterator for QueryTags {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tag) = self.tags_from_subject.next() {
            return Some(tag);
        }

        self.statements_from_knowledge.find_map(|statement| {
            if statement.intrinsic_statement_subject().unwrap() != self.subject
                || statement.intrinsic_statement_value().unwrap() != self.value
            {
                return None;
            }

            Some(statement.intrinsic_statement_tag().unwrap())
        })
    }
}
