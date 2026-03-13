use std::marker::PhantomData;

use everything_structures::{Object, Structure, ValuesIter};

use crate::{
    inference::compute::call,
    objects::{self, StructureExt},
};

// TODO
const TODO_OBJECT: Object = Object::Abstract(999_999_999);

pub fn query_values<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item>(
    knowledge_root: &'knowledge Structure,
    subject: &'subject Object,
    tag: &'tag Object,
) -> QueryValuesResult<'knowledge, 'subject, 'tag, 'item> {
    println!("querying ({subject:?}, {tag:?}, ?)");

    match (subject, tag) {
        (&objects::AXIOMATIC, &objects::AXIOMATIC) => {
            return QueryValuesResult::Single(Some(&TODO_OBJECT));
        }
        (&objects::AXIOMATIC, &objects::COMPUTED) => return QueryValuesResult::Single(None),
        (_, &objects::KNOWLEDGE) => {
            // We could also use the computation result variant
            // but for that we would need to create a set structure.

            return QueryValuesResult::Single(match subject {
                // TODO: review this for abstract objects
                Object::Abstract(_) => None,
                Object::Structure(s) => s.is_knowledge().then_some(&EMPTY_OBJECT),
            });
        }
        _ => {}
    }

    let maybe_constraint_query = query_values(knowledge_root, tag, &objects::AXIOMATIC);
    let maybe_constraint = maybe_constraint_query.iter().next();

    let maybe_computation_function_query = query_values(knowledge_root, tag, &objects::COMPUTED);
    let maybe_computation_function = maybe_computation_function_query.iter().next();

    match (maybe_constraint, maybe_computation_function) {
        (Some(_constraint_function), None) => {
            // Axiomatic (and ignore the constraint function).

            let values_from_subject = match subject {
                Object::Abstract(_) => None,
                Object::Structure(structure) => Some(structure.values(tag)),
            };

            let statements = knowledge_root.values(&objects::CONTAINS);

            QueryValuesResult::Axiomatic(AxiomaticIter {
                values_from_subject,
                statements,
                subject: subject.clone(),
                tag,
                _yield: PhantomData,
            })
        }
        (None, Some(computation_function)) => {
            let result = call(computation_function, &subject);
            QueryValuesResult::ComputationResult(result)
        }
        _ => {
            // In case that there is none or both,
            // tag is not a `Tag` so we can return nothing.

            QueryValuesResult::Single(None)
        }
    }
}

pub enum QueryValuesResult<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> {
    Single(Option<&'static Object>),
    Axiomatic(AxiomaticIter<'knowledge, 'subject, 'tag, 'item>),
    ComputationResult(Object),
}

static EMPTY_STRUCTURE: Structure = Structure::EMPTY;
static EMPTY_OBJECT: Object = Object::Structure(Structure::EMPTY);

impl<'knowlege: 'item, 'subject: 'item, 'tag: 'item, 'item>
    QueryValuesResult<'knowlege, 'subject, 'tag, 'item>
{
    pub fn iter<'query>(&'query self) -> QueryValuesIter<'query, 'knowlege, 'subject, 'tag, 'item> {
        match self {
            Self::Single(object) => QueryValuesIter::Single(object.clone()),
            Self::Axiomatic(axiomatic_iter) => QueryValuesIter::Axiomatic(axiomatic_iter.clone()),
            Self::ComputationResult(object) => {
                let structure = match object {
                    Object::Abstract(_) => &EMPTY_STRUCTURE,
                    Object::Structure(structure) => structure,
                };

                QueryValuesIter::ComputationResult(structure.values(&objects::CONTAINS))
            }
        }
    }
}

pub enum QueryValuesIter<
    'query_result: 'item,
    'knowledge: 'item,
    'subject: 'item,
    'tag: 'item,
    'item,
> {
    Single(Option<&'static Object>),
    Axiomatic(AxiomaticIter<'knowledge, 'subject, 'tag, 'item>),
    ComputationResult(ValuesIter<'query_result, 'tag>),
}

impl<'query, 'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> Iterator
    for QueryValuesIter<'query, 'knowledge, 'subject, 'tag, 'item>
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
pub struct AxiomaticIter<'knowledge: 'item, 'subject: 'item, 'tag: 'item, 'item> {
    values_from_subject: Option<ValuesIter<'subject, 'tag>>,
    statements: ValuesIter<'knowledge, 'static>,
    subject: Object,
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

            if statement_subject != &self.subject {
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

            if let Object::Structure(structure) = &self.subject {
                if structure.has_by_ref(statement_tag, statement_value) {
                    continue;
                }
            }

            return Some(statement_value);
        }

        None
    }
}
