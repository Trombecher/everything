use everything_structures::{Object, Structure, StructureValues};
use tracing::instrument;

use crate::{base, ext::ObjectExt};

#[cfg(test)]
mod tests;

/// Values from an axiomatic query.
pub type AxiomaticQueryValues<'knowledge, 'subject, 'item> =
    FixedOrMore<AxiomaticBorrowedQueryValues<'knowledge, 'subject, 'item>>;

#[derive(Clone)]
pub struct AxiomaticBorrowedQueryValues<'knowledge, 'subject> {
    values_from_subject: StructureValues<'subject>,
    statements: StructureValues<'knowledge>,
    subject: &'subject Object,
    tag: Object,
}

impl<'knowledge, 'subject> Iterator for AxiomaticBorrowedQueryValues<'knowledge, 'subject> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.values_from_subject.next() {
            return Some(value);
        }

        for statement in self.statements.by_ref() {
            // TODO: better error msg

            let statement = match statement {
                Object::Abstract(_) => panic!(":/"),
                Object::Structure(structure) => structure,
            };

            let statement_subject = statement
                .any()
                .unwrap()
                .values(Object::STATEMENT_SUBJECT)
                .next()
                .expect(":/");

            if statement_subject != self.subject {
                continue;
            }

            let statement_tag = statement
                .any()
                .unwrap()
                .values(Object::STATEMENT_TAG)
                .next()
                .expect(":/");

            if statement_tag != &self.tag {
                continue;
            }

            let statement_value = statement
                .any()
                .unwrap()
                .values(Object::STATEMENT_VALUE)
                .next()
                .expect(":/");

            // Now this value may be already been in
            // the subject if it is a structure. So we
            // need to dedup here.

            if let Object::Structure(structure) = &self.subject
                && structure.has(statement_tag, statement_value)
            {
                continue;
            }

            return Some(statement_value.clone());
        }

        None
    }
}

/// Query the knowledge for all values of the given subject with the
/// given tag, ignoring all computations that have to be made.
///
/// # Assumptions
///
/// * `tag` and all downstream tags are assumed to be axiomatic.
/// * `knowledge` is a superset of [crate::base::BASE].
#[instrument(skip(knowledge), ret)]
#[inline]
pub fn values_axiomatically<'knowledge: 'item, 'subject: 'item, 'item>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
) -> AxiomaticQueryValues<'knowledge, 'subject, 'item> {
    match (subject, &tag) {
        (&Object::AXIOMATIC, &Object::AXIOMATIC) => {
            AxiomaticQueryValues::One(&base::AXIOMATIC_AXIOMATIC_CONSTRAINT)
        }
        (&Object::AXIOMATIC | &Object::COMPUTED, &Object::COMPUTED) => AxiomaticQueryValues::None,
        _ => {
            let values_from_subject = match subject {
                Object::Abstract(_) => Structure::Empty.values(tag.clone()),
                Object::Structure(structure) => structure.values(tag.clone()),
            };

            let statements = knowledge.values(Object::CONTAINS);

            AxiomaticQueryValues::More(AxiomaticBorrowedQueryValues {
                values_from_subject,
                statements,
                subject,
                tag,
            })
        }
    }
}
