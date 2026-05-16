mod axiomatic;
#[cfg(test)]
mod tests;

use std::array;

use everything_structures::{Abstract, Object, Structure, StructureValues};
use tracing::instrument;

use crate::ctx::EvaluationContext;
use crate::ext::{AbstractExt, ObjectExt, StructureExt};

pub use axiomatic::*;

enum InitialMatch {
    Axiomatic,
    Compute,
    None,
}

/// Returns an iterator over all axiomatic and computed values
/// of the given `subject` with the given `tag` in the given `knowledge`
/// and `context`.
#[instrument(skip(knowledge))]
pub fn values<'knowledge, 'subject>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
    context: &mut EvaluationContext,
) -> QueryValuesResult<'knowledge, 'subject> {
    let initial_match = match (subject, &tag) {
        (&Object::Abstract(Abstract::AXIOMATIC), &Object::Abstract(Abstract::AXIOMATIC))
        | (
            &Object::Abstract(Abstract::AXIOMATIC | Abstract::COMPUTED),
            &Object::Abstract(Abstract::COMPUTED),
        ) => InitialMatch::Axiomatic,
        (_, &Object::Abstract(Abstract::KNOWLEDGE)) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            return QueryValuesResult::Axiomatic(match subject {
                Object::Structure(s) if s.is_knowledge().is_ok() => {
                    AxiomaticQueryValues::EmptyStructure
                }
                // TODO: review this for abstract objects
                _ => AxiomaticQueryValues::None,
            });
        }
        _ => {
            match (
                values_axiomatically(knowledge, &tag, Abstract::AXIOMATIC.into()).next(),
                values_axiomatically(knowledge, &tag, Abstract::COMPUTED.into()).next(),
            ) {
                (Some(_), None) => InitialMatch::Axiomatic,
                (None, Some(_)) => InitialMatch::Compute,
                _ => InitialMatch::None,
            }
        }
    };

    match initial_match {
        InitialMatch::Axiomatic => {
            QueryValuesResult::Axiomatic(values_axiomatically(knowledge, subject, tag))
        }
        InitialMatch::Compute => {
            let result = tag.call(knowledge, array::from_ref(subject), context);
            QueryValuesResult::ComputationResult(result)
        }
        InitialMatch::None => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            QueryValuesResult::Axiomatic(AxiomaticQueryValues::None)
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum QueryValuesResult<'knowledge, 'subject> {
    Axiomatic(AxiomaticQueryValues<'knowledge, 'subject>),
    ComputationResult(Object),
}

impl<'knowlege, 'subject> QueryValuesResult<'knowlege, 'subject> {
    pub fn values<'query>(&'query self) -> QueryValues<'query, 'knowlege, 'subject> {
        match self {
            Self::Axiomatic(axiomatic_iter) => QueryValues::Axiomatic(axiomatic_iter.clone()),
            Self::ComputationResult(object) => {
                let values = match object {
                    Object::Abstract(_) => StructureValues::None,
                    Object::Structure(structure) => structure.values(Abstract::CONTAINS.into()),
                };

                QueryValues::ComputationResult(values)
            }
        }
    }

    pub fn collect_to_set(self) -> Object {
        match self {
            Self::Axiomatic(iterator) => iterator.collect_to_set().into(),
            Self::ComputationResult(result) => result, // <- this is expected to be a set
        }
    }
}

/// An iterator over all axiomatic and computed values
#[derive(Clone)]
pub enum QueryValues<'query_result, 'knowledge, 'subject> {
    Axiomatic(AxiomaticQueryValues<'knowledge, 'subject>),
    ComputationResult(StructureValues<'query_result>),
}

impl Iterator for QueryValues<'_, '_, '_> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Axiomatic(axiomatic_iter) => axiomatic_iter.next(),
            Self::ComputationResult(iter) => iter.next(),
        }
    }
}
