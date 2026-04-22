#[cfg(test)]
mod tests;

use std::borrow::Cow;

use everything_structures::{Object, Property, Structure};
use fallible_iterator::{FallibleIterator, IteratorExt};
use tracing::{instrument, warn};

use crate::{
    ctx::{EvaluationContext, FunctionContext},
    ext::{KnowledgeError, NodeType, ObjectForm, StatementForm, StructureExt},
    query,
};

macro_rules! define_abstract {
    ($($id:ident = $n:literal),* $(,)?) => {
        $(const $id: Object = Object::Abstract($n);)*
    };
}

pub trait ObjectExt {
    // DO NOT CHANGE THESE!
    define_abstract!(
        CONTAINS = 1,
        AXIOMATIC = 2,
        COMPUTED = 3,
        STATEMENT_SUBJECT = 4,
        STATEMENT_TAG = 5,
        STATEMENT_VALUE = 6,
        STATEMENT = 7,
        KNOWLEDGE = 8,
        // NODE_FUNCTION_BODY = 11,
        NODE_LITERAL = 12,
        NODE_AND = 13,
        NODE_EXISTS = 14,
        NODE_PARAMETER = 15,
        // IS_NATURAL_NUMBER = 16,
        NODE_COUNT = 17,
        NODE_QUERY = 18,
        NODE_EQUAL = 19,
        NODE_OR = 20,
        NODE_XOR = 21,
        NODE_NOT = 22,
        NODE = 23,
        TAG = 24,
        NODE_FUNCTION_SELF = 25,
        // NODE_CALL_TARGET = 26,
        // NODE_CALL_PARAMETER = 27,
        // NODE_CALL = 28,
    );

    /// Extracts the first [Self::NODE_EXISTS] from `self`.
    fn node_exists<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>>;

    /// Extracts the first [Self::NODE_COUNT] from `self`.
    fn node_count<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>>;

    /// Extracts the first [Self::COMPUTED] from `self`.
    fn computed_body<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>>;

    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object;

    fn eval(&self, knowledge: &Structure, ctx: &mut EvaluationContext) -> Object;

    fn node_type(&self, knowledge: &Structure) -> Option<NodeType>;

    /// Counts how many items are in the set `self`.
    fn item_count(&self, knowledge: &Structure) -> usize;

    fn structure(&self) -> Option<&Structure>;

    fn is_truthy(&self, knowledge: &Structure) -> bool;

    /// Calls `self` with a list of parameters.
    /// If none are provided, `self` will just be evaluated.
    ///
    /// Node that it does not evaluate any parameters.
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[Object],
        ctx: &mut EvaluationContext,
    ) -> Object;

    fn to_natural_number(&self, knowledge: &Structure) -> Option<usize>;

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<usize>;

    fn node_literal<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>>;

    fn statement_form<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> StatementForm<'item>;

    fn node_query<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>>;

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError>;

    fn is_natural_number(&self, knowledge: &Structure) -> bool;
}

impl ObjectExt for Object {
    fn is_natural_number(&self, knowledge: &Structure) -> bool {
        if self == &Self::ZERO {
            return true;
        }

        let mut successor_of = query::values_axiomatically(knowledge, self, Self::SUCCESSOR_OF);

        if let Some(first) = successor_of.next()
            && successor_of.next().is_none()
        {
            first.is_natural_number(knowledge)
        } else {
            false
        }
    }

    fn item_count(&self, knowledge: &Structure) -> usize {
        query::values_axiomatically(knowledge, self, Object::CONTAINS).count()
    }

    fn structure(&self) -> Option<&Structure> {
        match self {
            Self::Abstract(_) => None,
            Self::Structure(structure) => Some(structure),
        }
    }

    fn is_truthy(&self, knowledge: &Structure) -> bool {
        query::values_axiomatically(knowledge, self, Object::CONTAINS)
            .next()
            .is_some()
    }

    #[instrument(skip(knowledge), ret)]
    fn node_type(&self, knowledge: &Structure) -> Option<NodeType> {
        NodeType::ALL
            .into_iter()
            .filter_map(|node_type| {
                query::values_axiomatically(knowledge, self, node_type.into())
                    .next()
                    .map(|_| node_type)
            })
            .next()
    }

    fn node_count<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>> {
        query::values_axiomatically(knowledge, self, Object::NODE_COUNT).next()
    }

    fn computed_body<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>> {
        query::values_axiomatically(knowledge, self, Object::COMPUTED).next()
    }

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<usize> {
        query::values_axiomatically(knowledge, self, Object::NODE_PARAMETER)
            .next()
            .and_then(|depth| depth.to_natural_number(knowledge))
    }

    fn node_literal<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Object>> {
        query::values_axiomatically(knowledge, self, Object::NODE_LITERAL).next()
    }

    fn statement_form<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> StatementForm<'item> {
        let subject: ObjectForm =
            query::values_axiomatically(knowledge, self, Object::STATEMENT_SUBJECT)
                .next()
                .into();

        let tag: ObjectForm = query::values_axiomatically(knowledge, self, Object::STATEMENT_TAG)
            .next()
            .into();

        let value: ObjectForm =
            query::values_axiomatically(knowledge, self, Object::STATEMENT_VALUE)
                .next()
                .into();

        StatementForm {
            subject,
            tag,
            value,
        }
    }

    fn node_query<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Self>> {
        query::values_axiomatically(knowledge, self, Object::NODE_QUERY).next()
    }

    fn node_exists<'knowledge: 'item, 'subject: 'item, 'item>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> Option<Cow<'item, Self>> {
        query::values_axiomatically(knowledge, self, Object::NODE_EXISTS).next()
    }

    #[instrument(skip(knowledge), ret)]
    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object {
        match self.node_type(knowledge) {
            Some(NodeType::Computed) => {
                Structure::new_computed(self.computed_body(knowledge).unwrap().capture(
                    knowledge,
                    additional_depth + 1,
                    ctx,
                ))
                .into()
            }
            Some(NodeType::Parameter) => {
                let depth = self.node_parameter_depth(knowledge).unwrap();

                if let Some(offset_depth) = depth.checked_sub(additional_depth) {
                    // The min additional depth is 1.
                    // So when the parameter depth is 1 it will refer to
                    // captured parameters at an additional depth of 1.

                    ctx.parameter_value(offset_depth)
                } else {
                    // This parameter refers to some inner, bound function,
                    // so keep it.

                    self.clone()
                }
            }
            _ => match self {
                Self::Abstract(a) => Object::Abstract(*a),
                Self::Structure(Structure::NaturalNumber(n)) => {
                    Self::Structure(Structure::NaturalNumber(*n))
                }
                Self::Structure(Structure::Empty) => Structure::Empty.into(),
                Self::Structure(Structure::Bytes(_)) => todo!(),
                Self::Structure(Structure::Text(_)) => todo!(),
                Self::Structure(Structure::Any(structure)) => structure
                    .as_ref()
                    .iter()
                    .map(|property| {
                        let value = property.value.capture(knowledge, additional_depth, ctx);

                        let result = if property.value == value {
                            Ok(())
                        } else {
                            value.is_valid(knowledge, false)
                        };

                        match result {
                            Ok(()) => Ok(Property {
                                tag: property.tag.clone(),
                                value,
                            }),
                            // TODO: debate box
                            Err(error) => Err((value, Box::new(error))),
                        }
                    })
                    .transpose_into_fallible()
                    .collect::<Vec<_>>()
                    .map(|mut properties| Self::Structure(Structure::new(&mut properties)))
                    .unwrap_or_else(|(o, error)| {
                        warn!("invalid object {o:?} with error {error:?}; replacing with {{}}");

                        Structure::Empty.into()
                    }),
            },
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn eval(&self, knowledge: &Structure, ctx: &mut EvaluationContext) -> Self {
        // TODO: better panic msgs

        match self.node_type(knowledge) {
            Some(NodeType::Count) => Self::new_natural_number(
                self.node_count(knowledge)
                    .expect("NodeType::Count asserts that this exists")
                    .eval(knowledge, ctx)
                    .item_count(knowledge) as u128,
            ),
            Some(NodeType::Query) => {
                // TODO: adjust constraint for query

                let statement_form = self
                    .node_query(knowledge)
                    .expect("Node::Query expects this");

                let statement_form = statement_form.statement_form(knowledge);

                let subject = Option::<Cow<Object>>::from(statement_form.subject)
                    .expect("cannot query with no subject")
                    .eval(knowledge, ctx);

                let tag = Option::<Cow<Object>>::from(statement_form.tag)
                    .expect("cannot query with no tag")
                    .eval(knowledge, ctx);

                let value = Option::<Cow<Object>>::from(statement_form.value)
                    .map(|c| c.eval(knowledge, ctx));

                let actual_qr = query::values(knowledge, &subject, tag, ctx);

                if let Some(value) = value {
                    // This is just equal to `NODE_EXISTS`.

                    for item in actual_qr.iter() {
                        if item.as_ref() == &value {
                            return Structure::new_bool(true).into();
                        }
                    }

                    Structure::new_bool(false).into()
                } else {
                    // Collect all values into a set.
                    actual_qr.collect_to_set()
                }
            }
            Some(NodeType::Literal) => self.node_literal(knowledge).unwrap().into_owned(),
            Some(NodeType::Computed) => self.capture(knowledge, 0, ctx),
            Some(NodeType::Parameter) => {
                ctx.parameter_value(self.node_parameter_depth(knowledge).unwrap())
            }
            Some(NodeType::Equal) => {
                let mut values = query::values_axiomatically(knowledge, self, Object::NODE_EQUAL)
                    .map(|value| value.eval(knowledge, ctx));

                let first = values.next().unwrap();
                let equal = values.all(|object| object == first);

                Structure::new_bool(equal).into()
            }
            Some(NodeType::Or) => {
                let mut values = query::values_axiomatically(knowledge, self, Object::NODE_OR)
                    .map(|value| value.eval(knowledge, ctx));

                Structure::new_bool(values.any(|o| o.is_truthy(knowledge))).into()
            }
            Some(NodeType::And) => {
                let mut values = query::values_axiomatically(knowledge, self, Object::NODE_AND)
                    .map(|value| value.eval(knowledge, ctx));

                Structure::new_bool(values.all(|value| value.is_truthy(knowledge))).into()
            }
            Some(NodeType::Exists) => {
                // TODO: discuss exists form

                let statement_form = self
                    .node_exists(knowledge)
                    .expect("NodeType::Exists asserts this");

                let StatementForm {
                    subject,
                    tag,
                    value,
                } = statement_form.statement_form(knowledge);

                let subject = Option::<Cow<Object>>::from(subject)
                    .expect("cannot query exists with no subject (not yet)")
                    .eval(knowledge, ctx);

                let tag = Option::<Cow<Object>>::from(tag).map(|tag| tag.eval(knowledge, ctx));
                let value = Option::<Cow<Object>>::from(value).map(|tag| tag.eval(knowledge, ctx));

                match (tag, value) {
                    // I think this should be fine.
                    (None, None) => Structure::new_bool(true).into(),
                    (Some(tag), None) => {
                        // Now we only need to check if one value exists.

                        let values_qr = query::values(knowledge, &subject, tag, ctx);

                        Structure::new_bool(values_qr.iter().next().is_some()).into()
                    }
                    // TODO: ?
                    (None, Some(_)) => Structure::new_bool(true).into(),
                    (Some(tag), Some(value)) => {
                        // TODO: perf

                        let values_qr = query::values(knowledge, &subject, tag, ctx);
                        Structure::new_bool(values_qr.iter().find(|v| **v == value).is_some())
                            .into()
                    }
                }
            }
            Some(NodeType::Not) => {
                let value = query::values_axiomatically(knowledge, self, Object::NODE_NOT)
                    .next()
                    .unwrap()
                    .eval(knowledge, ctx);

                Structure::new_bool(!value.is_truthy(knowledge)).into()
            }
            Some(ty) => todo!("{ty:?} not impl"),
            None if let Object::Structure(structure) = self => structure
                .properties()
                .map(|property| {
                    let value = property.value.eval(knowledge, ctx);

                    let result = if property.value == value {
                        Ok(())
                    } else {
                        value.is_valid(knowledge, false)
                    };

                    match result {
                        Ok(()) => Ok(Property {
                            tag: property.tag.clone(),
                            value,
                        }),
                        // TODO: debate box
                        Err(error) => Err((value, Box::new(error))),
                    }
                })
                .transpose_into_fallible()
                .collect::<Vec<_>>()
                .map(|mut properties| Object::Structure(Structure::new(&mut properties)))
                .unwrap_or_else(|(o, error)| {
                    warn!("invalid object {o:?} with error {error:?}; replacing with {{}}");

                    Structure::Empty.into()
                }),
            None => self.clone(),
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[Object],
        ctx: &mut EvaluationContext,
    ) -> Object {
        if let Some((parameter, next_parameters)) = parameters.split_first()
            && let Some(NodeType::Computed) = self.node_type(knowledge)
        {
            let body = self.computed_body(knowledge).unwrap();

            ctx.push(FunctionContext {
                function: self.clone(),
                parameter: parameter.clone(),
            });

            let result = body.call(knowledge, next_parameters, ctx);

            ctx.pop();

            result
        } else {
            self.eval(knowledge, ctx)
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn to_natural_number(&self, knowledge: &Structure) -> Option<usize> {
        if self == &Object::ZERO {
            Some(0)
        } else {
            query::values_axiomatically(knowledge, self, Object::SUCCESSOR_OF)
                .next()
                .and_then(|inner| inner.to_natural_number(knowledge))
                .map(|n| n + 1)
        }
    }

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError> {
        match self {
            Self::Abstract(_) => Ok(()),
            Self::Structure(structure) => structure.is_valid(knowledge, recursive),
        }
    }
}
