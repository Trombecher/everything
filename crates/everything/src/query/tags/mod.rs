use everything_structures::{Object, Structure, StructureTags};

use crate::{StructureSetValues, ext::ObjectExt};

#[derive(Clone, Debug)]
pub struct QueryTagsAxiomatically {
    tags_from_subject: StructureTags,
    statements_from_knowledge: StructureSetValues,
    subject: Object,
    value: Object,
}

impl QueryTagsAxiomatically {
    pub fn new(knowledge: &Structure, subject: Object, value: Object) -> Self {
        let tags_from_subject = match &subject {
            Object::Abstract(_) => StructureTags::None,
            Object::Structure(subject) => subject.tags(value.clone()),
        };

        Self {
            tags_from_subject,
            statements_from_knowledge: StructureSetValues::new(knowledge),
            subject,
            value,
        }
    }
}

impl Iterator for QueryTagsAxiomatically {
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
