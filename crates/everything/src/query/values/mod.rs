mod axiomatic;
#[cfg(test)]
mod tests;

use std::array;
use std::borrow::Cow;

use everything_structures::{Object, Property, Structure, StructureValues};
use tracing::instrument;

use crate::ctx::EvaluationContext;
use crate::ext::{ObjectExt, StructureExt};

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
pub fn values<'knowledge: 'item, 'subject: 'item, 'item>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
    context: &mut EvaluationContext,
) -> QueryValuesResult<'knowledge, 'subject, 'item> {
    let initial_match = match (subject, &tag) {
        (&Object::AXIOMATIC, &Object::AXIOMATIC)
        | (&Object::AXIOMATIC | &Object::COMPUTED, &Object::COMPUTED) => InitialMatch::Axiomatic,
        (_, &Object::KNOWLEDGE) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            return QueryValuesResult::Axiomatic(match subject {
                Object::Structure(s) if s.is_knowledge().is_ok() => {
                    AxiomaticQueryValues::One(Cow::Owned(Object::Structure(Structure::Empty)))
                }
                // TODO: review this for abstract objects
                _ => AxiomaticQueryValues::None,
            });
        }
        _ => {
            match (
                values_axiomatically(knowledge, &tag, Object::AXIOMATIC).next(),
                values_axiomatically(knowledge, &tag, Object::COMPUTED).next(),
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
pub enum QueryValuesResult<'knowledge: 'item, 'subject: 'item, 'item> {
    Axiomatic(AxiomaticQueryValues<'knowledge, 'subject, 'item>),
    ComputationResult(Object),
}

impl<'knowlege: 'item, 'subject: 'item, 'item> QueryValuesResult<'knowlege, 'subject, 'item> {
    pub fn iter<'query>(&'query self) -> QueryValues<'query, 'knowlege, 'subject, 'item> {
        match self {
            Self::Axiomatic(axiomatic_iter) => QueryValues::Axiomatic(axiomatic_iter.clone()),
            Self::ComputationResult(object) => {
                let values = match object {
                    Object::Abstract(_) => StructureValues::None,
                    Object::Structure(structure) => structure.values(Object::CONTAINS),
                };

                QueryValues::ComputationResult(values)
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
                        value: value.into_owned(),
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
    ComputationResult(StructureValues<'query_result>),
}

impl<'query, 'knowledge: 'item, 'subject: 'item, 'item> Iterator
    for QueryValues<'query, 'knowledge, 'subject, 'item>
{
    type Item = Cow<'item, Object>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Axiomatic(axiomatic_iter) => axiomatic_iter.next(),
            Self::ComputationResult(iter) => iter.next(),
        }
    }
}
