use std::marker::PhantomData;

use everything_structures::{Object, Structure, ValuesIter};
use tracing::instrument;

use crate::{base, ext::ObjectExt, query::EMPTY_STRUCTURE};

#[cfg(test)]
mod tests;

/// Values from an axiomatic query.
#[derive(Clone)]
#[must_use]
pub enum AxiomaticQueryValues<'knowledge: 'item, 'subject: 'item, 'item> {
    Static(Option<&'static Object>),
    Borrowed(AxiomaticBorrowedQueryValues<'knowledge, 'subject, 'item>),
}

impl<'knowledge: 'item, 'subject: 'item, 'item> Iterator
    for AxiomaticQueryValues<'knowledge, 'subject, 'item>
{
    type Item = &'item Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Static(object) => object.take(),
            Self::Borrowed(axiomatic_borrowed_query_values) => {
                axiomatic_borrowed_query_values.next()
            }
        }
    }
}

#[derive(Clone)]
pub struct AxiomaticBorrowedQueryValues<'knowledge: 'item, 'subject: 'item, 'item> {
    values_from_subject: ValuesIter<'subject>,
    statements: ValuesIter<'knowledge>,
    subject: &'subject Object,
    tag: Object,
    _yield: PhantomData<&'item Object>,
}

impl<'knowledge: 'item, 'subject: 'item, 'item> Iterator
    for AxiomaticBorrowedQueryValues<'knowledge, 'subject, 'item>
{
    type Item = &'item Object;

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
                .values(Object::STATEMENT_SUBJECT)
                .next()
                .expect(":/");

            if statement_subject != self.subject {
                continue;
            }

            let statement_tag = statement.values(Object::STATEMENT_TAG).next().expect(":/");

            if statement_tag != &self.tag {
                continue;
            }

            let statement_value = statement
                .values(Object::STATEMENT_VALUE)
                .next()
                .expect(":/");

            // Now this value may be already been in
            // the subject if it is a structure. So we
            // need to dedup here.

            if let Object::Structure(structure) = &self.subject
                && structure.has_by_ref(statement_tag, statement_value)
            {
                continue;
            }

            return Some(statement_value);
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
#[instrument(skip(knowledge))]
#[inline]
pub fn query_values_axiomatically<'knowledge: 'item, 'subject: 'item, 'item>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
) -> AxiomaticQueryValues<'knowledge, 'subject, 'item> {
    match (subject, &tag) {
        (&Object::AXIOMATIC, &Object::AXIOMATIC) => {
            AxiomaticQueryValues::Static(Some(&base::AXIOMATIC_AXIOMATIC_CONSTRAINT))
        }
        (&Object::AXIOMATIC | &Object::COMPUTED, &Object::COMPUTED) => {
            AxiomaticQueryValues::Static(None)
        }
        _ => {
            let values_from_subject = match subject {
                Object::Abstract(_) => EMPTY_STRUCTURE.values(tag.clone()),
                Object::Structure(structure) => structure.values(tag.clone()),
            };

            let statements = knowledge.values(Object::CONTAINS);

            AxiomaticQueryValues::Borrowed(AxiomaticBorrowedQueryValues {
                values_from_subject,
                statements,
                subject,
                tag,
                _yield: PhantomData,
            })
        }
    }
}
