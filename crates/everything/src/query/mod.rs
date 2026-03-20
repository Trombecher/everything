#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use everything_structures::{Object, Property, Structure, ValuesIter};
use tracing::instrument;

use crate::ctx::EvaluationContext;
use crate::{
    base,
    ext::{ObjectExt, StructureExt},
};

#[instrument(skip(knowledge))]
pub fn query_values<'knowledge: 'item, 'subject: 'item, 'item>(
    knowledge: &'knowledge Structure,
    subject: &'subject Object,
    tag: Object,
    ctx: &mut EvaluationContext,
) -> QueryValuesResult<'knowledge, 'subject, 'item> {
    match (subject, &tag) {
        (&Object::AXIOMATIC, &Object::AXIOMATIC) => {
            return QueryValuesResult::Single(Some(&base::AXIOMATIC_AXIOMATIC_CONSTRAINT));
        }
        (&Object::AXIOMATIC | &Object::COMPUTED, &Object::COMPUTED) => {
            return QueryValuesResult::Single(None);
        }
        (_, &Object::KNOWLEDGE) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            let result = QueryValuesResult::Single(match subject {
                // TODO: review this for abstract objects
                Object::Abstract(_) => None,
                Object::Structure(s) => s.is_knowledge().is_ok().then_some(&EMPTY_OBJECT),
            });

            return result;
        }
        _ => {}
    }

    let maybe_constraint_query = query_values(knowledge, &tag, Object::AXIOMATIC, ctx);
    let maybe_constraint = maybe_constraint_query.iter().next();

    let maybe_computation_function_query = query_values(knowledge, &tag, Object::COMPUTED, ctx);
    let maybe_computation_function = maybe_computation_function_query.iter().next();

    match (maybe_constraint, maybe_computation_function) {
        (Some(_), None) => {
            // Tag is axiomatic.

            let values_from_subject = match subject {
                Object::Abstract(_) => None,
                Object::Structure(structure) => Some(structure.values(tag.clone())),
            };

            let statements = knowledge.values(Object::CONTAINS);

            QueryValuesResult::Axiomatic(AxiomaticIter {
                values_from_subject,
                statements,
                subject: subject.clone(),
                tag,
                _yield: PhantomData,
            })
        }
        (None, Some(computation_function)) => {
            let result = computation_function.call(knowledge, subject, ctx);
            QueryValuesResult::ComputationResult(result)
        }
        _ => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            QueryValuesResult::Single(None)
        }
    }
}

pub enum QueryValuesResult<'knowledge: 'item, 'subject: 'item, 'item> {
    Single(Option<&'static Object>),
    Axiomatic(AxiomaticIter<'knowledge, 'subject, 'item>),
    ComputationResult(Object),
}

static EMPTY_STRUCTURE: Structure = Structure::EMPTY;
static EMPTY_OBJECT: Object = Object::Structure(Structure::EMPTY);

impl<'knowlege: 'item, 'subject: 'item, 'item> QueryValuesResult<'knowlege, 'subject, 'item> {
    pub fn iter<'query>(&'query self) -> QueryValuesIter<'query, 'knowlege, 'subject, 'item> {
        match self {
            Self::Single(object) => QueryValuesIter::Single(*object),
            Self::Axiomatic(axiomatic_iter) => QueryValuesIter::Axiomatic(axiomatic_iter.clone()),
            Self::ComputationResult(object) => {
                let structure = match object {
                    Object::Abstract(_) => &EMPTY_STRUCTURE,
                    Object::Structure(structure) => structure,
                };

                QueryValuesIter::ComputationResult(structure.values(Object::CONTAINS))
            }
        }
    }

    pub fn collect_to_set(self) -> Object {
        match self {
            Self::Single(None) => Structure::EMPTY.into(),
            Self::Single(Some(object)) => Structure::new(&mut [Property {
                tag: Object::CONTAINS,
                value: object.clone(),
            }])
            .into(),
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

pub enum QueryValuesIter<'query_result: 'item, 'knowledge: 'item, 'subject: 'item, 'item> {
    Single(Option<&'static Object>),
    Axiomatic(AxiomaticIter<'knowledge, 'subject, 'item>),
    ComputationResult(ValuesIter<'query_result>),
}

impl<'query, 'knowledge: 'item, 'subject: 'item, 'item> Iterator
    for QueryValuesIter<'query, 'knowledge, 'subject, 'item>
{
    type Item = &'item Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(object) => object.take(),
            Self::Axiomatic(axiomatic_iter) => axiomatic_iter.next(),
            Self::ComputationResult(iter) => iter.next(),
        }
    }
}

#[derive(Clone)]
pub struct AxiomaticIter<'knowledge: 'item, 'subject: 'item, 'item> {
    values_from_subject: Option<ValuesIter<'subject>>,
    statements: ValuesIter<'knowledge>,
    subject: Object,
    tag: Object,
    _yield: PhantomData<&'item Object>,
}

impl<'knowledge: 'item, 'subject: 'item, 'item> Iterator
    for AxiomaticIter<'knowledge, 'subject, 'item>
{
    type Item = &'item Object;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.values_from_subject
            && let Some(value) = iter.next()
        {
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

            if statement_subject != &self.subject {
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
