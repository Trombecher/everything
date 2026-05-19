mod axiomatic;
#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Structure};
use tracing::instrument;

use crate::ext::{AbstractExt, StructureExt};

pub use axiomatic::*;

#[derive(Debug, Clone)]
pub enum QueryValues {
    Axiomatically(QueryValuesAxiomatically),
    Call {
        function_body: Object,
        parameter: Object,
    },
}

impl QueryValues {
    #[instrument(skip(knowledge))]
    pub fn new(knowledge: &Structure, subject: Object, tag: Object) -> Self {
        if let Object::Abstract(Abstract::KNOWLEDGE) = &tag {
            return Self::Axiomatically(match subject {
                Object::Structure(s) if s.is_knowledge().is_ok() => {
                    QueryValuesAxiomatically::EmptyStructure
                }
                _ => QueryValuesAxiomatically::None,
            });
        }

        match (
            QueryValuesAxiomatically::new(knowledge, tag.clone(), Abstract::AXIOMATIC.into())
                .next(),
            QueryValuesAxiomatically::new(knowledge, tag.clone(), Abstract::FUNCTION.into()).next(),
        ) {
            (Some(_), None) => {
                Self::Axiomatically(QueryValuesAxiomatically::new(knowledge, subject, tag))
            }
            (None, Some(function_body)) => {
                // tag is a function

                Self::Call {
                    function_body,
                    parameter: subject,
                }
            }
            _ => Self::Axiomatically(QueryValuesAxiomatically::None),
        }
    }
}
