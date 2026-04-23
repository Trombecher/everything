use everything_structures::{Abstract, Object, Structure, StructureValues};
use tracing::instrument;

use crate::{base, ext::AbstractExt};

#[cfg(test)]
mod tests;

/// An iterator over all axiomatic values the specified tag has on
/// an object. You can obtain an instance of this iterator by
/// calling [values_axiomatically].
#[derive(Clone)]
pub enum AxiomaticQueryValues<'knowledge, 'subject> {
    /// The variant that yields nothing.
    None,

    /// The variant that only yields [base::AXIOMATIC_AXIOMATIC_CONSTRAINT]
    /// and then nothing else.
    AxiomaticAxiomaticConstraint,

    /// The variant that only yields [Structure::Empty] and then nothing else.
    EmptyStructure,

    /// The variant that yields values from [AxiomaticBorrowedQueryValues].
    Borrowed(AxiomaticBorrowedQueryValues<'knowledge, 'subject>),
}

impl Iterator for AxiomaticQueryValues<'_, '_> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::AxiomaticAxiomaticConstraint => {
                *self = Self::None;
                Some(base::AXIOMATIC_AXIOMATIC_CONSTRAINT.clone())
            }
            Self::EmptyStructure => {
                *self = Self::None;
                Some(Structure::Empty.into())
            }
            Self::Borrowed(values) => values.next(),
        }
    }
}

impl std::fmt::Debug for AxiomaticQueryValues<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_list().entries(&mut this).finish()
    }
}

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
                .values(Abstract::STATEMENT_SUBJECT.into())
                .next()
                .expect(":/");

            if statement_subject != self.subject {
                continue;
            }

            let statement_tag = statement
                .any()
                .unwrap()
                .values(Abstract::STATEMENT_TAG.into())
                .next()
                .expect(":/");

            if statement_tag != &self.tag {
                continue;
            }

            let statement_value = statement
                .any()
                .unwrap()
                .values(Abstract::STATEMENT_VALUE.into())
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
pub fn values_axiomatically<'knowledge, 'subject>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
) -> AxiomaticQueryValues<'knowledge, 'subject> {
    match (subject, &tag) {
        (&Object::Abstract(Abstract::AXIOMATIC), &Object::Abstract(Abstract::AXIOMATIC)) => {
            AxiomaticQueryValues::AxiomaticAxiomaticConstraint
        }
        (
            &Object::Abstract(Abstract::AXIOMATIC | Abstract::COMPUTED),
            &Object::Abstract(Abstract::COMPUTED),
        ) => AxiomaticQueryValues::None,
        _ => {
            let values_from_subject = match subject {
                Object::Abstract(_) => Structure::Empty.values(tag.clone()),
                Object::Structure(structure) => structure.values(tag.clone()),
            };

            let statements = knowledge.values(Abstract::CONTAINS.into());

            AxiomaticQueryValues::Borrowed(AxiomaticBorrowedQueryValues {
                values_from_subject,
                statements,
                subject,
                tag,
            })
        }
    }
}
