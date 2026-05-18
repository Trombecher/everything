mod axiomatic;
#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Structure};
use tracing::instrument;

use crate::ext::{AbstractExt, StructureExt};

pub use axiomatic::*;

/*
/// Returns an iterator over all axiomatic and computed values
/// of the given `subject` with the given `tag` in the given `knowledge`
/// and `context`.
#[instrument(skip(knowledge))]
pub fn values(
    knowledge: &Structure,
    subject: Object,
    tag: Object,
    context: &mut EvaluationContext,
) -> QueryValues {
    enum InitialMatch {
        Axiomatic,
        Compute,
        None,
    }

    let initial_match = match (&subject, &tag) {
        (&Object::Abstract(Abstract::AXIOMATIC), &Object::Abstract(Abstract::AXIOMATIC))
        | (
            &Object::Abstract(Abstract::AXIOMATIC | Abstract::COMPUTED),
            &Object::Abstract(Abstract::COMPUTED),
        ) => InitialMatch::Axiomatic,
        (_, &Object::Abstract(Abstract::KNOWLEDGE)) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            return LazyObject::LazySetValues(LazySetValues::ValuesAxiomatically(match subject {
                Object::Structure(s) if s.is_knowledge().is_ok() => {
                    QueryValuesAxiomatically::EmptyStructure
                }
                // TODO: review this for abstract objects
                _ => QueryValuesAxiomatically::None,
            }));
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
        InitialMatch::Axiomatic => LazyObject::LazySetValues(LazySetValues::ValuesAxiomatically(
            values_axiomatically(knowledge, subject, tag),
        )),
        InitialMatch::Compute => tag.call(knowledge, array::from_ref(&subject), context),
        InitialMatch::None => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            LazyObject::LazySetValues(LazySetValues::ValuesAxiomatically(
                QueryValuesAxiomatically::None,
            ))
        }
    }
}
 */

#[instrument(skip(knowledge))]
pub fn values(knowledge: &Structure, subject: Object, tag: Object) -> QueryValues {
    if let Object::Abstract(Abstract::KNOWLEDGE) = &tag {
        return QueryValues::Axiomatically(match subject {
            Object::Structure(s) if s.is_knowledge().is_ok() => {
                QueryValuesAxiomatically::EmptyStructure
            }
            // TODO: review this for abstract objects
            _ => QueryValuesAxiomatically::None,
        });
    }

    match (
        values_axiomatically(knowledge, tag.clone(), Abstract::AXIOMATIC.into()).next(),
        values_axiomatically(knowledge, tag.clone(), Abstract::COMPUTED.into()).next(),
    ) {
        (Some(_), None) => {
            QueryValues::Axiomatically(values_axiomatically(knowledge, subject, tag))
        }
        (None, Some(function_body)) => {
            // tag is a function

            QueryValues::Call {
                function_body,
                parameter: subject,
            }
        }
        _ => QueryValues::Axiomatically(QueryValuesAxiomatically::None),
    }
}

#[derive(Debug, Clone)]
pub enum QueryValues {
    Axiomatically(QueryValuesAxiomatically),
    Call {
        function_body: Object,
        parameter: Object,
    },
}

impl QueryValues {
    /*
    pub fn call_into_lazy(self, knowledge: &Structure) -> LazyObject {
        match self {
            QueryValues::Axiomatically(query_values_axiomatically) => LazyObject::LazySetValues(
                LazySetValues::ValuesAxiomatically(query_values_axiomatically),
            ),
            QueryValues::Call {
                function_body,
                parameter,
            } => {
                let mut context = EvaluationContext::default();
                context.push();

                function_body.eval(knowledge, context)
            },
        }
    }
     */
}
