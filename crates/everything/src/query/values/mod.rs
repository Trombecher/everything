mod axiomatic;
#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Structure};
use tracing::instrument;

use crate::ext::{AbstractExt, StructureExt};

pub use axiomatic::*;

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
        values_axiomatically(knowledge, tag.clone(), Abstract::FUNCTION.into()).next(),
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
