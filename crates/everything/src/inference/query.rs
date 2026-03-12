use std::marker::PhantomData;

use everything_structures::{Object, Property, Structure, ValuesIter};

use crate::{inference::compute::compute, objects};

// TODO
const TODO_OBJECT: Object = Object::Abstract(u128::from_be_bytes(*b"This is the todo"));

pub fn query_values<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item>(
    knowledge_root: &'knowledge Structure,
    subject: &'subject Object,
    tag: &'tag Object,
) -> QueryValuesIter<'knowledge, 'subject, 'tag, 'item> {
    match (subject, tag) {
        (&objects::AXIOMATIC, &objects::AXIOMATIC) => {
            return QueryValuesIter::Single(Some(&TODO_OBJECT));
        }
        (&objects::AXIOMATIC, &objects::COMPUTED) => return QueryValuesIter::Single(None),
        _ => {}
    }

    let maybe_constraint = query_values(knowledge_root, tag, &objects::AXIOMATIC).next();
    let maybe_computation_function = query_values(knowledge_root, tag, &objects::COMPUTED).next();

    match (maybe_constraint, maybe_computation_function) {
        (Some(_constraint_function), None) => {
            // Axiomatic (and ignore the constraint function).

            let values_from_subject = match subject {
                Object::Abstract(_) => None,
                Object::Structure(structure) => Some(structure.values(tag)),
            };

            let statements = knowledge_root.values(&objects::CONTAINS);

            QueryValuesIter::Axiomatic(AxiomaticIter {
                values_from_subject,
                statements,
                subject,
                tag,
                _yield: PhantomData,
            })
        }
        (None, Some(computation_function)) => {
            let result = compute(computation_function, subject.clone());

            QueryValuesIter::ComputationResult(())
        }
        _ => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            QueryValuesIter::Single(None)
        }
    }
}

pub enum QueryValuesIter<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> {
    Single(Option<&'static Object>),
    Axiomatic(AxiomaticIter<'knowledge, 'subject, 'tag, 'item>),
    ComputationResult(ComputationResultIter),
}

impl<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> Iterator
    for QueryValuesIter<'knowledge, 'subject, 'tag, 'item>
{
    type Item = &'item Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(object) => object.take(),
            Self::Axiomatic(axiomatic_iter) => axiomatic_iter.next(),
        }
    }
}

struct AxiomaticIter<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> {
    values_from_subject: Option<ValuesIter<'subject, 'tag>>,
    statements: ValuesIter<'knowledge, 'static>,
    subject: &'subject Object,
    tag: &'tag Object,
    _yield: PhantomData<&'item Object>,
}

impl<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> Iterator
    for AxiomaticIter<'knowledge, 'subject, 'tag, 'item>
{
    type Item = &'item Object;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.values_from_subject {
            if let Some(value) = iter.next() {
                return Some(value);
            }
        }

        while let Some(statement) = self.statements.next() {
            // TODO: better error msg

            let statement = match statement {
                Object::Abstract(_) => panic!(":/"),
                Object::Structure(structure) => structure,
            };

            let statement_subject = statement
                .values(&objects::STATEMENT_SUBJECT)
                .next()
                .expect(":/");

            if statement_subject != self.subject {
                continue;
            }

            let statement_tag = statement
                .values(&objects::STATEMENT_TAG)
                .next()
                .expect(":/");

            if statement_tag != self.tag {
                continue;
            }

            let statement_value = statement
                .values(&objects::STATEMENT_TAG)
                .next()
                .expect(":/");

            // Now this value may be already been in
            // the subject if it is a structure. So we
            // need to dedup here.

            if let Object::Structure(structure) = self.subject {
                if structure.has_by_ref(statement_tag, statement_value) {
                    continue;
                }
            }

            return Some(statement_value);
        }

        None
    }
}

struct ComputationResultIter {
    result: Object,
}
