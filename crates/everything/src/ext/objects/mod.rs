#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Property, Structure};
use fallible_iterator::{FallibleIterator, IteratorExt};
use tracing::{debug, instrument, warn};

use crate::{
    LazyObject, LazySetValues,
    ctx::{EvaluationContext, FunctionContext},
    ext::{
        AbstractExt, KnowledgeError, ObjectForm, StatementForm, StructureExt,
        iter::IteratorExtNextAndLast,
    },
    nodes::{BinaryNode, FilterNode, IfNode, MapNode, Node, Task},
    query::{self, QueryValues, QueryValuesAxiomatically},
};

pub trait ObjectExt {
    /// Extracts the first (and last) [Abstract::NODE_COUNT] from `self`.
    fn node_count(&self, knowledge: &Structure) -> Option<Object>;

    /// Extracts the first (and last) [Abstract::COMPUTED] from `self`.
    fn computed_body(&self, knowledge: &Structure) -> Option<Object>;

    fn node_equal(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn node_and(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn node_or(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn node_add(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn node_xor(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn node_union(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object;

    fn eval(&self, knowledge: &Structure, context: &mut EvaluationContext) -> LazyObject;

    /// Parses a node from `self`.
    fn node(&self, knowledge: &Structure) -> Option<Node>;

    /// Returns an iterator over all set items.
    fn set_values(&self, knowledge: &Structure) -> QueryValuesAxiomatically;

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
    ) -> LazyObject;

    fn to_integer(&self, knowledge: &Structure) -> Option<i128>;

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<u128>;

    fn node_literal(&self, knowledge: &Structure) -> Option<Object>;

    fn statement_subject(&self, knowledge: &Structure) -> Option<Object>;
    fn statement_tag(&self, knowledge: &Structure) -> Option<Object>;
    fn statement_value(&self, knowledge: &Structure) -> Option<Object>;

    fn statement_form(&self, knowledge: &Structure) -> StatementForm;

    fn node_query(&self, knowledge: &Structure) -> Option<Object>;

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError>;

    fn is_natural_number(&self, knowledge: &Structure) -> bool;
    fn node_map(&self, knowledge: &Structure) -> Option<MapNode>;
    fn node_filter(&self, knowledge: &Structure) -> Option<FilterNode>;

    fn add(&self, knowledge: &Structure, other: &Object) -> Object;

    fn node_function_self(&self, knowledge: &Structure) -> Option<u128>;

    fn node_not(&self, knowledge: &Structure) -> Option<Object>;

    /// Parses a binary node by querying (axiomatically)
    /// for `left_tag` and `right_tag`.
    fn binary_node(
        &self,
        knowledge: &Structure,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode>;

    fn node_if(&self, knowledge: &Structure) -> Option<IfNode>;

    fn node_less(&self, knowledge: &Structure) -> Option<BinaryNode>;
}

impl ObjectExt for Object {
    fn is_natural_number(&self, knowledge: &Structure) -> bool {
        if self.exact_integer().is_some() {
            // Fast path of exact natural numbers.
            return true;
        }

        let mut successor_of =
            query::values_axiomatically(knowledge, self.clone(), Abstract::SUCCESSOR_OF.into());

        if let Some(first) = successor_of.next()
            && successor_of.next().is_none()
        {
            first.is_natural_number(knowledge)
        } else {
            false
        }
    }

    fn set_values(&self, knowledge: &Structure) -> QueryValuesAxiomatically {
        query::values_axiomatically(knowledge, self.clone(), Abstract::CONTAINS.into())
    }

    fn structure(&self) -> Option<&Structure> {
        match self {
            Self::Abstract(_) => None,
            Self::Structure(structure) => Some(structure),
        }
    }

    fn is_truthy(&self, knowledge: &Structure) -> bool {
        self.set_values(knowledge).next().is_some()
    }

    fn node_equal(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_EQUAL_LEFT.into(),
            Abstract::NODE_EQUAL_RIGHT.into(),
        )
    }

    fn node_and(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_AND_LEFT.into(),
            Abstract::NODE_AND_RIGHT.into(),
        )
    }

    fn node_or(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_OR_LEFT.into(),
            Abstract::NODE_OR_RIGHT.into(),
        )
    }

    fn node_union(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_UNION_LEFT.into(),
            Abstract::NODE_UNION_RIGHT.into(),
        )
    }

    fn node_map(&self, knowledge: &Structure) -> Option<MapNode> {
        let set_expression =
            query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_MAP_SET.into())
                .next()?;
        let mapper_function_expression =
            query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_MAP_MAPPER.into())
                .next()?;

        Some(MapNode {
            set: set_expression,
            mapper_function: mapper_function_expression,
        })
    }

    fn node_filter(&self, knowledge: &Structure) -> Option<FilterNode> {
        let set =
            query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_FILTER_SET.into())
                .next_and_last()?;

        let filter = query::values_axiomatically(
            knowledge,
            self.clone(),
            Abstract::NODE_FILTER_FILTER.into(),
        )
        .next_and_last()?;

        Some(FilterNode {
            set: set,
            filter_function: filter,
        })
    }

    fn node_add(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_ADD_LEFT.into(),
            Abstract::NODE_ADD_RIGHT.into(),
        )
    }

    fn node_xor(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_XOR_LEFT.into(),
            Abstract::NODE_XOR_RIGHT.into(),
        )
    }

    fn binary_node(
        &self,
        knowledge: &Structure,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode> {
        let left = query::values_axiomatically(knowledge, self.clone(), left_tag).next()?;
        let right = query::values_axiomatically(knowledge, self.clone(), right_tag).next()?;

        Some(BinaryNode { left, right })
    }

    #[instrument(skip(knowledge), ret)]
    fn node(&self, knowledge: &Structure) -> Option<Node> {
        let mut node = self.computed_body(knowledge).map(Node::Computed);

        macro_rules! xor_with {
            ($e:expr) => {{
                let variant = $e;

                if variant.is_some() {
                    if node.is_some() {
                        return None;
                    } else {
                        node = variant;
                    }
                }
            }};
        }

        xor_with!(self.node_literal(knowledge).map(Node::Literal));
        xor_with!(self.node_function_self(knowledge).map(Node::FunctionSelf));
        xor_with!(self.node_parameter_depth(knowledge).map(Node::Parameter));
        xor_with!(self.node_count(knowledge).map(Node::Count));
        xor_with!(self.node_query(knowledge).map(Node::Query));
        xor_with!(self.node_not(knowledge).map(Node::Not));
        xor_with!(self.node_and(knowledge).map(Node::And));
        xor_with!(self.node_or(knowledge).map(Node::Or));
        xor_with!(self.node_equal(knowledge).map(Node::Equal));
        xor_with!(self.node_xor(knowledge).map(Node::Xor));
        xor_with!(self.node_add(knowledge).map(Node::Add));
        xor_with!(self.node_union(knowledge).map(Node::Union));
        xor_with!(self.node_map(knowledge).map(Node::Map));
        xor_with!(self.node_filter(knowledge).map(Node::Filter));
        xor_with!(self.node_less(knowledge).map(Node::Less));
        xor_with!(self.node_if(knowledge).map(Node::If));

        node
    }

    fn node_if(&self, knowledge: &Structure) -> Option<IfNode> {
        let condition = query::values_axiomatically(
            knowledge,
            self.clone(),
            Abstract::NODE_IF_CONDITION.into(),
        )
        .next_and_last()?;

        let then =
            query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_IF_THEN.into())
                .next_and_last()?;

        let otherwise =
            query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_IF_ELSE.into())
                .next_and_last()?;

        Some(IfNode {
            condition,
            then,
            otherwise,
        })
    }

    fn node_less(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_LESS_LEFT.into(),
            Abstract::NODE_LESS_RIGHT.into(),
        )
    }

    fn node_not(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_NOT.into())
            .next_and_last()
    }

    fn node_count(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_COUNT.into())
            .next_and_last()
    }

    fn computed_body(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self.clone(), Abstract::COMPUTED.into())
            .next_and_last()
    }

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<u128> {
        let depth =
            query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_PARAMETER.into())
                .next_and_last()?
                .to_integer(knowledge)?;

        if depth >= 0 {
            Some(depth as u128)
        } else {
            None
        }
    }

    fn node_literal(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self.clone(), Abstract::NODE_LITERAL.into())
            .next_and_last()
    }

    fn node_function_self(&self, knowledge: &Structure) -> Option<u128> {
        let depth = query::values_axiomatically(
            knowledge,
            self.clone(),
            Abstract::NODE_FUNCTION_SELF.into(),
        )
        .next_and_last()?
        .to_integer(knowledge)?;

        if depth >= 0 {
            Some(depth as u128)
        } else {
            None
        }
    }

    fn statement_subject(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(
            knowledge,
            self.clone(),
            Object::Abstract(Abstract::STATEMENT_SUBJECT),
        )
        .next_and_last()
    }

    fn statement_tag(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(
            knowledge,
            self.clone(),
            Object::Abstract(Abstract::STATEMENT_TAG),
        )
        .next_and_last()
    }

    fn statement_value(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(
            knowledge,
            self.clone(),
            Object::Abstract(Abstract::STATEMENT_VALUE),
        )
        .next_and_last()
    }

    fn statement_form(&self, knowledge: &Structure) -> StatementForm {
        let subject: ObjectForm = self.statement_subject(knowledge).into();
        let tag: ObjectForm = self.statement_tag(knowledge).into();
        let value: ObjectForm = self.statement_value(knowledge).into();

        StatementForm {
            subject,
            tag,
            value,
        }
    }

    fn node_query(&self, knowledge: &Structure) -> Option<Self> {
        query::values_axiomatically(
            knowledge,
            self.clone(),
            Object::Abstract(Abstract::NODE_QUERY),
        )
        .next()
    }

    #[instrument(skip(knowledge), ret)]
    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object {
        match self.node(knowledge) {
            Some(Node::Computed(body)) => Structure::new_node(Node::Computed(body.capture(
                knowledge,
                additional_depth + 1,
                ctx,
            )))
            .into(),
            Some(Node::Parameter(depth)) => {
                if let Some(offset_depth) = (depth as usize).checked_sub(additional_depth) {
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
                _ => self.clone(),
            },
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn eval(&self, knowledge: &Structure, context: &mut EvaluationContext) -> LazyObject {
        let mut tasks = vec![Task::Eval(self.clone())];
        let mut evaluated = Vec::<LazyObject>::new();

        while let Some(task) = tasks.pop() {
            debug!("doing task {task:?}");

            match task {
                Task::Eval(object) => match object.node(knowledge) {
                    Some(Node::Computed(_)) => {
                        evaluated.push(object.capture(knowledge, 0, context).into());
                    }
                    Some(Node::Literal(object)) => {
                        evaluated.push(object.into());
                    }
                    Some(Node::And(BinaryNode { left, right })) => {
                        tasks.push(Task::PartialAnd { right });
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::FunctionSelf(depth)) => evaluated.push(LazyObject::Eager(
                        context.function_self(depth as usize).into(),
                    )),
                    Some(Node::Parameter(depth)) => {
                        evaluated.push(LazyObject::Eager(
                            context.parameter_value(depth as usize).into(),
                        ));
                    }
                    Some(Node::Count(object)) => {
                        tasks.push(Task::Count);
                        tasks.push(Task::Eval(object));
                    }
                    Some(Node::Query(object)) => match object.statement_form(knowledge) {
                        StatementForm {
                            subject: ObjectForm::Specific(unevaluated_subject),
                            tag: ObjectForm::Specific(unevaluated_tag),
                            value: ObjectForm::Any,
                        } => {
                            tasks.push(Task::QueryValues);
                            tasks.push(Task::Eval(unevaluated_tag));
                            tasks.push(Task::Eval(unevaluated_subject));
                        }
                        StatementForm {
                            subject: ObjectForm::Any,
                            tag: ObjectForm::Specific(unevaluated_tag),
                            value: ObjectForm::Specific(unevaluated_value),
                        } => {
                            tasks.push(Task::QuerySubjectsAxiomatically);
                            tasks.push(Task::Eval(unevaluated_value));
                            tasks.push(Task::Eval(unevaluated_tag));
                        }
                        StatementForm {
                            subject: ObjectForm::Any,
                            tag: ObjectForm::Specific(unevaluated_tag),
                            value: ObjectForm::Any,
                        } => {
                            tasks.push(Task::QuerySubjectsAndValuesAxiomatically);
                            tasks.push(Task::Eval(unevaluated_tag));
                        }
                        StatementForm {
                            subject: ObjectForm::Specific(unevaluated_subject),
                            tag: ObjectForm::Specific(unevaluated_tag),
                            value: ObjectForm::Specific(unevaluated_value),
                        } => {
                            tasks.push(Task::QueryExists);
                            tasks.push(Task::Eval(unevaluated_value));
                            tasks.push(Task::Eval(unevaluated_tag));
                            tasks.push(Task::Eval(unevaluated_subject));
                        }
                        _ => todo!("query"),
                    },
                    Some(Node::Equal(BinaryNode { left, right })) => {
                        tasks.push(Task::Equal);
                        tasks.push(Task::Eval(right));
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::Or(BinaryNode { left, right })) => {
                        tasks.push(Task::PartialOr { right });
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::Xor(BinaryNode { left, right })) => {
                        tasks.push(Task::Xor);
                        tasks.push(Task::Eval(right));
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::Not(object)) => {
                        tasks.push(Task::Not);
                        tasks.push(Task::Eval(object));
                    }
                    Some(Node::Add(BinaryNode { left, right })) => {
                        tasks.push(Task::Add);
                        tasks.push(Task::Eval(right));
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::Union(BinaryNode { left, right })) => {
                        tasks.push(Task::Union);
                        tasks.push(Task::Eval(right));
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::Map(MapNode {
                        set,
                        mapper_function,
                    })) => {
                        tasks.push(Task::Map);
                        tasks.push(Task::Eval(mapper_function));
                        tasks.push(Task::Eval(set));
                    }
                    Some(Node::Filter(FilterNode {
                        set,
                        filter_function,
                    })) => {
                        tasks.push(Task::Filter);
                        tasks.push(Task::Eval(filter_function));
                        tasks.push(Task::Eval(set));
                    }
                    Some(Node::Less(BinaryNode { left, right })) => {
                        tasks.push(Task::Less);
                        tasks.push(Task::Eval(right));
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::If(IfNode {
                        condition,
                        then,
                        otherwise,
                    })) => {
                        tasks.push(Task::PartialIf { then, otherwise });
                        tasks.push(Task::Eval(condition));
                    }
                    None if let Object::Structure(Structure::Any(any_structure)) = object => {
                        let result = any_structure
                            .properties()
                            .map(|property| {
                                // TODO: DEBATE THIS BS
                                debug!("eval on property {property:?}");

                                let value = property.value.eval(knowledge, context).into_object();

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
                            .map(|mut properties| {
                                Object::Structure(Structure::new(&mut properties)).into()
                            })
                            .unwrap_or_else(|(o, error)| {
                                warn!(
                                    "invalid object {o:?} with error {error:?}; replacing with {{}}"
                                );

                                LazyObject::Eager(Object::Structure(Structure::Empty))
                            });

                        evaluated.push(result);
                    }
                    None => evaluated.push(object.clone().into()),
                },
                Task::PartialAnd { right } => {
                    let mut left = evaluated.pop().expect("and first");

                    if left.is_truthy(knowledge) {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Eval(right));
                    } else {
                        evaluated.push(LazyObject::Eager(Structure::new_bool(true).into()));
                    }
                }
                Task::Count => {
                    let target = evaluated.pop().expect("count needs sum");

                    evaluated.push(
                        Self::new_integer(target.set_values(knowledge).correct_count() as i128)
                            .into(),
                    );
                }
                Task::QueryValues => {
                    let tag = evaluated.pop().unwrap().into_object();
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    match query::values(knowledge, subject, tag.clone()) {
                        QueryValues::Axiomatically(query_values_axiomatically) => {
                            evaluated.push(LazyObject::LazySetValues(
                                LazySetValues::ValuesAxiomatically(query_values_axiomatically),
                            ));
                        }
                        QueryValues::Call {
                            function_body,
                            parameter,
                        } => {
                            tasks.push(Task::PopContext);
                            tasks.push(Task::Eval(function_body));

                            context.push(FunctionContext {
                                function: tag,
                                parameter,
                            });
                        }
                    }
                }
                Task::QuerySubjectsAxiomatically => {
                    let value = evaluated.pop().unwrap().into_object();
                    let tag = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        LazySetValues::SubjectsAxiomatically(query::subjects_axiomatically(
                            knowledge, tag, value,
                        ))
                        .into(),
                    );
                }
                Task::QuerySubjectsAndValuesAxiomatically => {
                    let tag = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        LazySetValues::SubjectsAndValuesAxiomatically(
                            query::subjects_and_values_axiomatically(knowledge, tag),
                        )
                        .into(),
                    );
                }
                Task::QueryExists => {
                    let value = evaluated.pop().unwrap().into_object();
                    let tag = evaluated.pop().unwrap().into_object();
                    let subject = evaluated.pop().unwrap().into_object();

                    match query::exists(knowledge, subject, tag.clone(), value) {
                        query::QueryExists::Axiomatically(exists) => {
                            evaluated.push(LazyObject::Eager(Structure::new_bool(exists).into()))
                        }
                        query::QueryExists::Call {
                            function_body,
                            parameter,
                        } => {
                            tasks.push(Task::PopContext);
                            tasks.push(Task::Eval(function_body));

                            context.push(FunctionContext {
                                function: tag,
                                parameter,
                            });
                        }
                    }
                }
                Task::ToBoolean => {
                    let mut object = evaluated.pop().unwrap();

                    evaluated.push(LazyObject::Eager(
                        Structure::new_bool(object.is_truthy(knowledge)).into(),
                    ))
                }
                Task::Equal => {
                    let right = evaluated.pop().unwrap().into_object();
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(Object::Structure(Structure::new_bool(left == right)).into());
                }
                Task::PartialOr { right } => {
                    if evaluated.pop().unwrap().is_truthy(knowledge) {
                        evaluated.push(LazyObject::Eager(Structure::new_bool(true).into()));
                    } else {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Eval(right));
                    }
                }
                Task::Xor => {
                    let right = evaluated.pop().unwrap().is_truthy(knowledge);
                    let left = evaluated.pop().unwrap().is_truthy(knowledge);

                    evaluated.push(LazyObject::Eager(
                        Structure::new_bool((left || right) && !(left && right)).into(),
                    ));
                }
                Task::Not => {
                    let mut object = evaluated.pop().unwrap();

                    evaluated.push(LazyObject::Eager(
                        Structure::new_bool(!object.is_truthy(knowledge)).into(),
                    ));
                }
                Task::Add => {
                    let right = evaluated.pop().unwrap().into_object();
                    // TODO: (perf) maybe short circuit sets into UNDEFINED.
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(left.add(knowledge, &right).into());
                }
                Task::Union => {
                    let right = evaluated.pop().unwrap();
                    let left = evaluated.pop().unwrap();

                    evaluated.push(LazyObject::LazySetValues(LazySetValues::Union {
                        left: Box::new(left.set_values(knowledge)),
                        right: Box::new(right.set_values(knowledge)),
                    }));
                }
                Task::Map => {
                    let mapper = evaluated.pop().unwrap().into_object();
                    let set = evaluated.pop().unwrap().set_values(knowledge);

                    evaluated.push(LazyObject::LazySetValues(LazySetValues::Map {
                        knowledge: knowledge.clone(),
                        set: Box::new(set),
                        mapper_function: mapper,
                    }));
                }
                Task::Filter => {
                    let filter = evaluated.pop().unwrap().into_object();
                    let set = evaluated.pop().unwrap().set_values(knowledge);

                    evaluated.push(LazyObject::LazySetValues(LazySetValues::Filter {
                        knowledge: knowledge.clone(),
                        set: Box::new(set),
                        filter_function: filter,
                    }));
                }
                Task::Less => {
                    let right = evaluated.pop().unwrap();
                    let left = evaluated.pop().unwrap();

                    evaluated.push(
                        Object::Structure(Structure::new_bool(match (left, right) {
                            (LazyObject::Eager(left), LazyObject::Eager(right))
                                if let Some(left) = left.to_integer(knowledge)
                                    && let Some(right) = right.to_integer(knowledge) =>
                            {
                                left < right
                            }
                            // TODO: more things here
                            (left, right) => {
                                left.set_values(knowledge).correct_count()
                                    < right.set_values(knowledge).correct_count()
                            }
                        }))
                        .into(),
                    );
                }
                Task::PartialIf { then, otherwise } => {
                    if evaluated.pop().unwrap().is_truthy(knowledge) {
                        tasks.push(Task::Eval(then));
                    } else {
                        tasks.push(Task::Eval(otherwise));
                    }
                }
                Task::PopContext => {
                    context.pop();
                }
            }
        }

        let last = evaluated.pop().unwrap();
        if evaluated.pop().is_some() {
            unreachable!()
        }

        last
    }

    #[instrument(skip(knowledge), ret)]
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[Object],
        ctx: &mut EvaluationContext,
    ) -> LazyObject {
        if let Some((parameter, next_parameters)) = parameters.split_first()
            && let Some(Node::Computed(body)) = self.node(knowledge)
        {
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
    fn to_integer(&self, knowledge: &Structure) -> Option<i128> {
        if let Some(n) = self.exact_integer() {
            // Fast path for exact natural numbers.
            Some(n)
        } else if let Some(predecessor) =
            query::values_axiomatically(knowledge, self.clone(), Abstract::SUCCESSOR_OF.into())
                .next_and_last()
        {
            predecessor
                .to_integer(knowledge)
                .map(|n| n.checked_add(1).expect("yo shi too big"))
        } else if let Some(successor) =
            query::values_axiomatically(knowledge, self.clone(), Abstract::PREDECESSOR_OF.into())
                .next_and_last()
        {
            successor
                .to_integer(knowledge)
                .map(|n| n.checked_sub(1).expect("yo shi too small"))
        } else {
            None
        }
    }

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError> {
        match self {
            Self::Abstract(_) => Ok(()),
            Self::Structure(structure) => structure.is_valid(knowledge, recursive),
        }
    }

    fn add(&self, knowledge: &Structure, other: &Object) -> Object {
        if let Some(left) = self.to_integer(knowledge)
            && let Some(right) = other.to_integer(knowledge)
        {
            if let Some(sum) = left.checked_add(right) {
                Object::new_integer(sum)
            } else {
                Abstract::ARITHMETIC_OVERFLOW.into()
            }
        } else {
            Abstract::UNDEFINED.into()
        }
    }
}
