mod axiomatic;
#[cfg(test)]
mod tests;

use std::array;

use everything_structures::{Object, Property, Structure, ValuesIter};
use tracing::instrument;

use crate::ctx::EvaluationContext;
use crate::ext::{ObjectExt, StructureExt};

use crate::base::IS_NATURAL_NUMBER;

pub use axiomatic::*;

enum InitialMatch<'a> {
    Axiomatic,
    ComputationFunction(&'a Object),
    None,
}

#[instrument(skip(knowledge))]
pub fn query_values<'knowledge: 'item, 'subject: 'item, 'item>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
    ctx: &mut EvaluationContext,
) -> QueryValuesResult<'knowledge, 'subject, 'item> {
    let initial_match = match (subject, &tag) {
        (&Object::AXIOMATIC, &Object::AXIOMATIC)
        | (&Object::AXIOMATIC | &Object::COMPUTED, &Object::COMPUTED) => InitialMatch::Axiomatic,
        (_, &Object::KNOWLEDGE) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            let result =
                QueryValuesResult::Axiomatic(AxiomaticQueryValues::Static(match subject {
                    // TODO: review this for abstract objects
                    Object::Abstract(_) => None,
                    Object::Structure(s) => s.is_knowledge().is_ok().then_some(&EMPTY_OBJECT),
                }));

            return result;
        }
        _ => {
            let maybe_constraint =
                query_values_axiomatically(knowledge, &tag, Object::AXIOMATIC).next();

            let maybe_computation_function =
                query_values_axiomatically(knowledge, &tag, Object::COMPUTED).next();

            match (maybe_constraint, maybe_computation_function) {
                (Some(_), None) => InitialMatch::Axiomatic,
                (None, Some(f)) => InitialMatch::ComputationFunction(f),
                _ => InitialMatch::None,
            }
        }
    };

    match initial_match {
        InitialMatch::Axiomatic => {
            QueryValuesResult::Axiomatic(query_values_axiomatically(knowledge, subject, tag))
        }
        InitialMatch::ComputationFunction(computation_function) => {
            let result = computation_function.call(knowledge, array::from_ref(subject), ctx);
            QueryValuesResult::ComputationResult(result)
        }
        InitialMatch::None => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            QueryValuesResult::Axiomatic(AxiomaticQueryValues::Static(None))
        }
    }
}

pub enum QueryValuesResult<'knowledge: 'item, 'subject: 'item, 'item> {
    Axiomatic(AxiomaticQueryValues<'knowledge, 'subject, 'item>),
    ComputationResult(Object),
}

static EMPTY_STRUCTURE: Structure = Structure::EMPTY;
static EMPTY_OBJECT: Object = Object::Structure(Structure::EMPTY);

impl<'knowlege: 'item, 'subject: 'item, 'item> QueryValuesResult<'knowlege, 'subject, 'item> {
    pub fn iter<'query>(&'query self) -> QueryValues<'query, 'knowlege, 'subject, 'item> {
        match self {
            Self::Axiomatic(axiomatic_iter) => QueryValues::Axiomatic(axiomatic_iter.clone()),
            Self::ComputationResult(object) => {
                let structure = match object {
                    Object::Abstract(_) => &EMPTY_STRUCTURE,
                    Object::Structure(structure) => structure,
                };

                QueryValues::ComputationResult(structure.values(Object::CONTAINS))
            }
        }
    }

    pub fn collect_to_set(self) -> Object {
        match self {
            Self::Axiomatic(axiomatic_iter) => {
                // Collect all values to a set.

                let mut properties: Vec<_> = axiomatic_iter
                    .map(|value| Property {
                        tag: Object::CONTAINS,
                        value: value.clone(),
                    })
                    .collect();

                Structure::new(&mut properties).into()
            }
            Self::ComputationResult(result) => result, // <- this is expected to be a set
        }
    }
}

pub enum QueryValues<'query_result: 'item, 'knowledge: 'item, 'subject: 'item, 'item> {
    Axiomatic(AxiomaticQueryValues<'knowledge, 'subject, 'item>),
    ComputationResult(ValuesIter<'query_result>),
}

impl<'query, 'knowledge: 'item, 'subject: 'item, 'item> Iterator
    for QueryValues<'query, 'knowledge, 'subject, 'item>
{
    type Item = &'item Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Axiomatic(axiomatic_iter) => axiomatic_iter.next(),
            Self::ComputationResult(iter) => iter.next(),
        }
    }
}
