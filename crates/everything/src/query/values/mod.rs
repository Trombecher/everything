mod axiomatic;
#[cfg(test)]
mod tests;

use std::array;

use everything_structures::{Abstract, Object, Structure, StructureValues};
use tracing::instrument;

use crate::LazyObject;
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
pub fn values(
    knowledge: &Structure,
    subject: Object,
    tag: Object,
    context: &mut EvaluationContext,
) -> LazyObject {
    let initial_match = match (&subject, &tag) {
        (&Object::Abstract(Abstract::AXIOMATIC), &Object::Abstract(Abstract::AXIOMATIC))
        | (
            &Object::Abstract(Abstract::AXIOMATIC | Abstract::COMPUTED),
            &Object::Abstract(Abstract::COMPUTED),
        ) => InitialMatch::Axiomatic,
        (_, &Object::Abstract(Abstract::KNOWLEDGE)) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            return LazyObject::LazySetValues(match subject {
                Object::Structure(s) if s.is_knowledge().is_ok() => {
                    AxiomaticQueryValues::EmptyStructure
                }
                // TODO: review this for abstract objects
                _ => AxiomaticQueryValues::None,
            });
        }
        _ => {
            match (
                values_axiomatically(knowledge, tag.clone(), Abstract::AXIOMATIC.into()).next(),
                values_axiomatically(knowledge, tag.clone(), Abstract::COMPUTED.into()).next(),
            ) {
                (Some(_), None) => InitialMatch::Axiomatic,
                (None, Some(_)) => InitialMatch::Compute,
                _ => InitialMatch::None,
            }
        }
    };

    match initial_match {
        InitialMatch::Axiomatic => {
            LazyObject::LazySetValues(values_axiomatically(knowledge, subject, tag))
        }
        InitialMatch::Compute => tag.call(knowledge, array::from_ref(&subject), context),
        InitialMatch::None => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            LazyObject::LazySetValues(AxiomaticQueryValues::None)
        }
    }
}

/*
impl QueryValuesResult {
    pub fn values(&self) -> QueryValues {
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
 */

/// An iterator over all axiomatic and computed values
#[derive(Clone)]
pub enum QueryValues {
    Axiomatic(AxiomaticQueryValues),
    ComputationResult(StructureValues),
}

impl Iterator for QueryValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Axiomatic(axiomatic_iter) => axiomatic_iter.next(),
            Self::ComputationResult(iter) => iter.next(),
        }
    }
}
