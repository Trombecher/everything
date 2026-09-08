#[cfg(test)]
mod tests;

use everything_objects::{Abstract, Composite, Object, Property};
use fallible_iterator::{FallibleIterator, IteratorExt};
use tracing::{debug, instrument, warn};

use crate::{
    ObjectOrSetValues, SetValues,
    ctx::{EvaluationContext, FunctionContext},
    ext::{AbstractExt, CompositeExt, KnowledgeError, iter::IteratorExtNextAndLast},
    nodes::{
        BinaryNode, CallNode, FilterNode, IfNode, MapNode, Node, PredicateNode, QueryExistsNode,
        QuerySubjectsAndTagsNode, QuerySubjectsAndValuesNode, QuerySubjectsNode,
        QueryTagsAndValuesNode, QueryTagsNode, QueryValuesNode, Task, UnwrapOrNode,
    },
    statements::{QueryValues, SimpleStatement, Statements},
};

/// An extension trait implemented for [`Object`], providing many useful functions.
pub trait ObjectExt {
    /// Extracts the first (and last) [`Abstract::FUNCTION`] from `self`.
    fn node_function_body(&self, statements: &Statements) -> Option<Object>;

    fn capture(
        &self,
        statements: &Statements,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> ObjectOrSetValues;

    /// Evaluates `self` under the given statements and evaluation context
    /// by reducing expressions.
    ///
    /// If you don't know what to pass into the context, pass
    /// `&mut Default::default()`.
    fn evaluate(
        &self,
        statements: &Statements,
        context: &mut EvaluationContext,
    ) -> ObjectOrSetValues;

    /// Parses a node from `self`.
    fn node(&self, statements: &Statements) -> Option<Node>;

    /// Returns an iterator over all set items.
    fn set_values(&self, statements: &Statements) -> QueryValues;

    fn composite(&self) -> Option<&Composite>;

    fn is_truthy(&self, statements: &Statements) -> bool;

    /// Calls `self` with a list of parameters.
    /// If none are provided, `self` will just be evaluated.
    ///
    /// Note that it does not evaluate any parameters.
    fn call(
        &self,
        statements: &Statements,
        parameters: &[ObjectOrSetValues],
        ctx: &mut EvaluationContext,
    ) -> ObjectOrSetValues;

    fn to_integer(&self, statements: &Statements) -> Option<i128>;

    /// Checks whether this object is valid.
    ///
    /// # Errors
    ///
    /// This function will return an error if it is not valid.
    fn is_valid(&self, statements: &Statements, recursive: bool) -> Result<(), KnowledgeError>;

    fn is_natural_number(&self, statements: &Statements) -> bool;

    fn add(&self, statements: &Statements, other: &Object) -> Object;

    /// Parses a binary node by querying (axiomatically)
    /// for `left_tag` and `right_tag`.
    fn binary_node(
        &self,
        statements: &Statements,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode>;

    fn multiply(&self, statements: &Statements, other: &Object) -> Object;

    fn new_node(node: Node) -> Self;
}

impl ObjectExt for Object {
    #[allow(clippy::too_many_lines)]
    fn new_node(node: Node) -> Self {
        match node {
            Node::IsAbstract(inner) => Composite::new(&mut [Property {
                tag: Abstract::NODE_IS_ABSTRACT.into(),
                value: inner,
            }])
            .into(),
            Node::Call(CallNode { callee, with }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_CALL_CALLEE.into(),
                    value: callee,
                },
                Property {
                    tag: Abstract::NODE_CALL_WITH.into(),
                    value: with,
                },
            ])
            .into(),
            Node::Function(body) => Composite::new(&mut [Property {
                tag: Abstract::FUNCTION.into(),
                value: body,
            }])
            .into(),
            Node::Literal(literal) => Composite::new(&mut [Property {
                tag: Abstract::NODE_LITERAL.into(),
                value: literal,
            }])
            .into(),
            Node::And(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_AND_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_AND_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::FunctionSelf(depth) => Composite::new(&mut [Property {
                tag: Abstract::NODE_FUNCTION_SELF.into(),
                value: Object::new_integer(i128::from(depth)),
            }])
            .into(),
            Node::Parameter(depth) => Composite::new(&mut [Property {
                tag: Abstract::NODE_PARAMETER.into(),
                value: Object::new_integer(i128::from(depth)),
            }])
            .into(),
            Node::Count(object) => Composite::new(&mut [Property {
                tag: Abstract::NODE_COUNT.into(),
                value: object,
            }])
            .into(),
            Node::QueryExists(exists) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_QUERY_SUBJECT.into(),
                    value: exists.subject,
                },
                Property {
                    tag: Abstract::NODE_QUERY_TAG.into(),
                    value: exists.tag,
                },
                Property {
                    tag: Abstract::NODE_QUERY_VALUE.into(),
                    value: exists.value,
                },
            ])
            .into(),
            Node::QuerySubjects(subjects) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_QUERY_TAG.into(),
                    value: subjects.tag,
                },
                Property {
                    tag: Abstract::NODE_QUERY_VALUE.into(),
                    value: subjects.value,
                },
            ])
            .into(),
            Node::QueryTags(query) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_QUERY_SUBJECT.into(),
                    value: query.subject,
                },
                Property {
                    tag: Abstract::NODE_QUERY_VALUE.into(),
                    value: query.value,
                },
            ])
            .into(),
            Node::QueryValues(query) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_QUERY_SUBJECT.into(),
                    value: query.subject,
                },
                Property {
                    tag: Abstract::NODE_QUERY_TAG.into(),
                    value: query.tag,
                },
            ])
            .into(),
            Node::QuerySubjectsAndTags(query) => Composite::new(&mut [Property {
                tag: Abstract::NODE_QUERY_VALUE.into(),
                value: query.value,
            }])
            .into(),
            Node::QueryTagsAndValues(query) => Composite::new(&mut [Property {
                tag: Abstract::NODE_QUERY_SUBJECT.into(),
                value: query.subject,
            }])
            .into(),
            Node::QuerySubjectsAndValues(query) => Composite::new(&mut [Property {
                tag: Abstract::NODE_QUERY_TAG.into(),
                value: query.tag,
            }])
            .into(),
            Node::Statements => Abstract::NODE_STATEMENTS.into(),
            Node::Equal(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_EQUAL_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_EQUAL_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::Or(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_OR_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_OR_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::Xor(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_XOR_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_XOR_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::Not(node) => Composite::new(&mut [Property {
                tag: Abstract::NODE_NOT.into(),
                value: node,
            }])
            .into(),
            Node::Add(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_ADD_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_ADD_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::Union(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_UNION_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_UNION_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::Map(MapNode {
                set,
                mapper_function,
            }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_MAP_SET.into(),
                    value: set,
                },
                Property {
                    tag: Abstract::NODE_MAP_MAPPER.into(),
                    value: mapper_function,
                },
            ])
            .into(),
            Node::Filter(FilterNode {
                set,
                filter_function,
            }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_FILTER_SET.into(),
                    value: set,
                },
                Property {
                    tag: Abstract::NODE_FILTER_FILTER.into(),
                    value: filter_function,
                },
            ])
            .into(),
            Node::Every(PredicateNode { set, predicate }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_EVERY_SET.into(),
                    value: set,
                },
                Property {
                    tag: Abstract::NODE_EVERY_PREDICATE.into(),
                    value: predicate,
                },
            ])
            .into(),
            Node::Any(PredicateNode { set, predicate }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_ANY_SET.into(),
                    value: set,
                },
                Property {
                    tag: Abstract::NODE_ANY_PREDICATE.into(),
                    value: predicate,
                },
            ])
            .into(),
            Node::Less(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_LESS_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_LESS_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
            Node::If(IfNode {
                condition,
                otherwise,
                then,
            }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_IF_CONDITION.into(),
                    value: condition,
                },
                Property {
                    tag: Abstract::NODE_IF_THEN.into(),
                    value: then,
                },
                Property {
                    tag: Abstract::NODE_IF_ELSE.into(),
                    value: otherwise,
                },
            ])
            .into(),
            Node::UnwrapOr(UnwrapOrNode { set, default }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_UNWRAP_OR_SET.into(),
                    value: set,
                },
                Property {
                    tag: Abstract::NODE_UNWRAP_OR_DEFAULT.into(),
                    value: default,
                },
            ])
            .into(),
            Node::Multiply(BinaryNode { left, right }) => Composite::new(&mut [
                Property {
                    tag: Abstract::NODE_MULTIPLY_LEFT.into(),
                    value: left,
                },
                Property {
                    tag: Abstract::NODE_MULTIPLY_RIGHT.into(),
                    value: right,
                },
            ])
            .into(),
        }
    }

    fn is_natural_number(&self, statements: &Statements) -> bool {
        if self.exact_integer().is_some() {
            // Fast path of exact natural numbers.
            return true;
        }

        let mut successor_of = statements.query_values(self.clone(), Abstract::SUCCESSOR_OF.into());

        if let Some(first) = successor_of.next()
            && successor_of.next().is_none()
        {
            first.is_natural_number(statements)
        } else {
            false
        }
    }

    fn set_values(&self, statements: &Statements) -> QueryValues {
        statements.query_values(self.clone(), Abstract::CONTAINS.into())
    }

    fn composite(&self) -> Option<&Composite> {
        match self {
            Self::Abstract(_) => None,
            Self::Composite(composite) => Some(composite),
        }
    }

    fn is_truthy(&self, statements: &Statements) -> bool {
        self.set_values(statements).next().is_some()
    }

    fn binary_node(
        &self,
        statements: &Statements,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode> {
        let left = statements
            .query_values(self.clone(), left_tag)
            .next_and_last()?;

        let right = statements
            .query_values(self.clone(), right_tag)
            .next_and_last()?;

        Some(BinaryNode { left, right })
    }

    #[instrument(skip(statements), ret)]
    #[allow(clippy::too_many_lines)]
    fn node(&self, statements: &Statements) -> Option<Node> {
        let mut node = self.node_function_body(statements).map(Node::Function);

        macro_rules! xor_with {
            ($e:expr) => {{
                let variant = $e;

                if variant.is_some() {
                    if node.is_some() {
                        return None;
                    }

                    node = variant;
                }
            }};
        }

        xor_with!(
            statements
                .query_values(self.clone(), Abstract::NODE_LITERAL.into())
                .next_and_last()
                .map(Node::Literal)
        );

        fn node_function_self(this: &Object, statements: &Statements) -> Option<u32> {
            let depth = statements
                .query_values(this.clone(), Abstract::NODE_FUNCTION_SELF.into())
                .next_and_last()?
                .to_integer(statements)?;

            u32::try_from(depth).ok()
        }
        xor_with!(node_function_self(self, statements).map(Node::FunctionSelf));

        fn node_parameter_depth(this: &Object, statements: &Statements) -> Option<u32> {
            let depth = statements
                .query_values(this.clone(), Abstract::NODE_PARAMETER.into())
                .next_and_last()?
                .to_integer(statements)?;

            u32::try_from(depth).ok()
        }
        xor_with!(node_parameter_depth(self, statements).map(Node::Parameter));

        fn node_call(this: &Object, statements: &Statements) -> Option<CallNode> {
            let callee = statements
                .query_values(this.clone(), Abstract::NODE_CALL_CALLEE.into())
                .next_and_last()?;

            let with = statements
                .query_values(this.clone(), Abstract::NODE_CALL_WITH.into())
                .next_and_last()?;

            Some(CallNode { callee, with })
        }
        xor_with!(node_call(self, statements).map(Node::Call));

        xor_with!(
            statements
                .query_values(self.clone(), Abstract::NODE_COUNT.into())
                .next_and_last()
                .map(Node::Count)
        );

        {
            let subject = statements
                .query_values(self.clone(), Abstract::NODE_QUERY_SUBJECT.into())
                .next_and_last();

            let tag = statements
                .query_values(self.clone(), Abstract::NODE_QUERY_TAG.into())
                .next_and_last();

            let value = statements
                .query_values(self.clone(), Abstract::NODE_QUERY_VALUE.into())
                .next_and_last();

            // This can be made lazy
            if node.is_some() && (subject.is_some() || tag.is_some() || value.is_some()) {
                // This will produce a node.

                return None;
            }

            match (subject, tag, value) {
                (Some(subject), Some(tag), Some(value)) => {
                    node = Some(Node::QueryExists(QueryExistsNode {
                        subject,
                        tag,
                        value,
                    }));
                }
                (Some(subject), Some(tag), None) => {
                    node = Some(Node::QueryValues(QueryValuesNode { subject, tag }));
                }
                (Some(subject), None, Some(value)) => {
                    node = Some(Node::QueryTags(QueryTagsNode { subject, value }));
                }
                (None, Some(tag), Some(value)) => {
                    node = Some(Node::QuerySubjects(QuerySubjectsNode { tag, value }));
                }
                (Some(subject), None, None) => {
                    node = Some(Node::QueryTagsAndValues(QueryTagsAndValuesNode { subject }));
                }
                (None, Some(tag), None) => {
                    node = Some(Node::QuerySubjectsAndValues(QuerySubjectsAndValuesNode {
                        tag,
                    }));
                }
                (None, None, Some(value)) => {
                    node = Some(Node::QuerySubjectsAndTags(QuerySubjectsAndTagsNode {
                        value,
                    }));
                }
                (None, None, None) => {
                    // This is not one of the seven nodes, so do nothing.
                }
            }
        }

        xor_with!((self == &Abstract::NODE_STATEMENTS.into()).then_some(Node::Statements));

        xor_with!(
            statements
                .query_values(self.clone(), Abstract::NODE_NOT.into())
                .next_and_last()
                .map(Node::Not)
        );

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_AND_LEFT.into(),
                Abstract::NODE_AND_RIGHT.into(),
            )
            .map(Node::And)
        );

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_OR_LEFT.into(),
                Abstract::NODE_OR_RIGHT.into(),
            )
            .map(Node::Or)
        );

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_EQUAL_LEFT.into(),
                Abstract::NODE_EQUAL_RIGHT.into(),
            )
            .map(Node::Equal)
        );

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_XOR_LEFT.into(),
                Abstract::NODE_XOR_RIGHT.into(),
            )
            .map(Node::Xor)
        );

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_ADD_LEFT.into(),
                Abstract::NODE_ADD_RIGHT.into(),
            )
            .map(Node::Add)
        );

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_UNION_LEFT.into(),
                Abstract::NODE_UNION_RIGHT.into(),
            )
            .map(Node::Union)
        );

        fn node_map(this: &Object, statements: &Statements) -> Option<MapNode> {
            let set_expression = statements
                .query_values(this.clone(), Abstract::NODE_MAP_SET.into())
                .next()?;
            let mapper_function_expression = statements
                .query_values(this.clone(), Abstract::NODE_MAP_MAPPER.into())
                .next()?;

            Some(MapNode {
                set: set_expression,
                mapper_function: mapper_function_expression,
            })
        }
        xor_with!(node_map(self, statements).map(Node::Map));

        fn node_filter(this: &Object, statements: &Statements) -> Option<FilterNode> {
            let set = statements
                .query_values(this.clone(), Abstract::NODE_FILTER_SET.into())
                .next_and_last()?;

            let filter = statements
                .query_values(this.clone(), Abstract::NODE_FILTER_FILTER.into())
                .next_and_last()?;

            Some(FilterNode {
                set,
                filter_function: filter,
            })
        }
        xor_with!(node_filter(self, statements).map(Node::Filter));

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_LESS_LEFT.into(),
                Abstract::NODE_LESS_RIGHT.into(),
            )
            .map(Node::Less)
        );

        fn node_if(object: &Object, statements: &Statements) -> Option<IfNode> {
            let condition = statements
                .query_values(object.clone(), Abstract::NODE_IF_CONDITION.into())
                .next_and_last()?;

            let then = statements
                .query_values(object.clone(), Abstract::NODE_IF_THEN.into())
                .next_and_last()?;

            let otherwise = statements
                .query_values(object.clone(), Abstract::NODE_IF_ELSE.into())
                .next_and_last()?;

            Some(IfNode {
                condition,
                then,
                otherwise,
            })
        }
        xor_with!(node_if(self, statements).map(Node::If));

        fn node_unwrap_or(this: &Object, statements: &Statements) -> Option<UnwrapOrNode> {
            let set = statements
                .query_values(this.clone(), Abstract::NODE_UNWRAP_OR_SET.into())
                .next_and_last()?;

            let default = statements
                .query_values(this.clone(), Abstract::NODE_UNWRAP_OR_DEFAULT.into())
                .next_and_last()?;

            Some(UnwrapOrNode { set, default })
        }
        xor_with!(node_unwrap_or(self, statements).map(Node::UnwrapOr));

        xor_with!(
            self.binary_node(
                statements,
                Abstract::NODE_MULTIPLY_LEFT.into(),
                Abstract::NODE_MULTIPLY_RIGHT.into(),
            )
            .map(Node::Multiply)
        );

        xor_with!(
            statements
                .query_values(self.clone(), Abstract::NODE_IS_ABSTRACT.into())
                .next_and_last()
                .map(Node::IsAbstract)
        );

        fn node_every(this: &Object, statements: &Statements) -> Option<PredicateNode> {
            let set = statements
                .query_values(this.clone(), Abstract::NODE_EVERY_SET.into())
                .next_and_last()?;

            let predicate = statements
                .query_values(this.clone(), Abstract::NODE_EVERY_PREDICATE.into())
                .next_and_last()?;

            Some(PredicateNode { set, predicate })
        }
        xor_with!(node_every(self, statements).map(Node::Every));

        fn node_any(this: &Object, statements: &Statements) -> Option<PredicateNode> {
            let set = statements
                .query_values(this.clone(), Abstract::NODE_ANY_SET.into())
                .next_and_last()?;

            let predicate = statements
                .query_values(this.clone(), Abstract::NODE_ANY_PREDICATE.into())
                .next_and_last()?;

            Some(PredicateNode { set, predicate })
        }
        xor_with!(node_any(self, statements).map(Node::Any));

        node
    }

    fn node_function_body(&self, statements: &Statements) -> Option<Object> {
        statements
            .query_values(self.clone(), Abstract::FUNCTION.into())
            .next_and_last()
    }

    #[instrument(skip(statements), ret)]
    fn capture(
        &self,
        statements: &Statements,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> ObjectOrSetValues {
        match self.node(statements) {
            Some(Node::Function(body)) => Self::new_node(Node::Function(
                body.capture(statements, additional_depth + 1, ctx)
                    .into_object(),
            ))
            .into(),
            Some(Node::Parameter(depth)) => {
                if let Some(offset_depth) = (depth as usize).checked_sub(additional_depth) {
                    // The min additional depth is 1.
                    // So when the parameter depth is 1 it will refer to
                    // captured parameters at an additional depth of 1.

                    // We also need to escape the parameter because it may contain
                    // a node (which is already evaluated because parameters
                    // are always evaluated before the function).

                    ObjectOrSetValues::Object(Object::new_node(Node::Literal(
                        ctx.parameter_value(offset_depth).into_object(),
                    )))
                } else {
                    // This parameter refers to some inner, bound function,
                    // so keep it.

                    ObjectOrSetValues::Object(self.clone())
                }
            }
            _ => match self {
                Self::Composite(Composite::Any(composite)) => composite
                    .as_ref()
                    .iter()
                    .map(|property| {
                        let value = property
                            .value
                            .capture(statements, additional_depth, ctx)
                            .into_object();

                        let result = if property.value == value {
                            Ok(())
                        } else {
                            value.is_valid(statements, false)
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
                    .map_or_else(
                        |(o, error)| {
                            warn!("invalid object {o:?} with error {error:?}; replacing with {{}}");

                            ObjectOrSetValues::Object(Composite::Empty.into())
                        },
                        |mut properties| {
                            ObjectOrSetValues::Object(Self::Composite(Composite::new(
                                &mut properties,
                            )))
                        },
                    ),
                _ => ObjectOrSetValues::Object(self.clone()),
            },
        }
    }

    fn multiply(&self, statements: &Statements, other: &Object) -> Object {
        if let Some(left) = self.to_integer(statements)
            && let Some(right) = other.to_integer(statements)
        {
            if let Some(product) = left.checked_mul(right) {
                Object::new_integer(product)
            } else {
                Abstract::ARITHMETIC_OVERFLOW.into()
            }
        } else {
            Abstract::UNDEFINED.into()
        }
    }

    #[instrument(skip(statements), ret)]
    #[allow(clippy::too_many_lines)]
    fn evaluate(
        &self,
        statements: &Statements,
        context: &mut EvaluationContext,
    ) -> ObjectOrSetValues {
        let mut tasks = vec![Task::Eval(self.clone())];
        let mut evaluated = Vec::<ObjectOrSetValues>::new();

        while let Some(task) = tasks.pop() {
            debug!("doing task {task:?}");

            match task {
                Task::Eval(object) => match object.node(statements) {
                    Some(Node::IsAbstract(inner)) => {
                        tasks.push(Task::IsAbstract);
                        tasks.push(Task::Eval(inner));
                    }
                    Some(Node::Call(CallNode { callee, with })) => {
                        tasks.push(Task::Call);
                        tasks.push(Task::Eval(callee));
                        tasks.push(Task::Eval(with));
                    }
                    Some(Node::Multiply(BinaryNode { left, right })) => {
                        tasks.push(Task::Multiply);
                        tasks.push(Task::Eval(right));
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::UnwrapOr(UnwrapOrNode { set, default })) => {
                        tasks.push(Task::PartialUnwrapOr { default });
                        tasks.push(Task::Eval(set));
                    }
                    Some(Node::Function(_)) => {
                        evaluated.push(object.capture(statements, 0, context));
                    }
                    Some(Node::Literal(object)) => {
                        evaluated.push(object.into());
                    }
                    Some(Node::And(BinaryNode { left, right })) => {
                        tasks.push(Task::PartialAnd { right });
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::FunctionSelf(depth)) => evaluated.push(ObjectOrSetValues::Object(
                        context.function_self(depth as usize),
                    )),
                    Some(Node::Parameter(depth)) => {
                        evaluated.push(context.parameter_value(depth as usize));
                    }
                    Some(Node::Count(object)) => {
                        tasks.push(Task::Count);
                        tasks.push(Task::Eval(object));
                    }
                    Some(Node::QueryExists(query)) => {
                        tasks.push(Task::QueryExists);
                        tasks.push(Task::Eval(query.value));
                        tasks.push(Task::Eval(query.tag));
                        tasks.push(Task::Eval(query.subject));
                    }
                    Some(Node::QueryValues(query)) => {
                        tasks.push(Task::QueryValues);
                        tasks.push(Task::Eval(query.tag));
                        tasks.push(Task::Eval(query.subject));
                    }
                    Some(Node::QuerySubjects(query)) => {
                        tasks.push(Task::QuerySubjects);
                        tasks.push(Task::Eval(query.value));
                        tasks.push(Task::Eval(query.tag));
                    }
                    Some(Node::QueryTags(query)) => {
                        tasks.push(Task::QueryTags);
                        tasks.push(Task::Eval(query.value));
                        tasks.push(Task::Eval(query.subject));
                    }
                    Some(Node::QuerySubjectsAndTags(query)) => {
                        tasks.push(Task::QuerySubjectsAndTags);
                        tasks.push(Task::Eval(query.value));
                    }
                    Some(Node::QuerySubjectsAndValues(query)) => {
                        tasks.push(Task::QuerySubjectsAndValues);
                        tasks.push(Task::Eval(query.tag));
                    }
                    Some(Node::QueryTagsAndValues(query)) => {
                        tasks.push(Task::QueryTagsAndValues);
                        tasks.push(Task::Eval(query.subject));
                    }
                    Some(Node::Statements) => {
                        // Evaluate straight to an iterator over
                        // all statements:

                        evaluated.push(ObjectOrSetValues::SetValues(SetValues::Statements(
                            statements.iter_owned(),
                        )));
                    }
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
                    Some(Node::Every(PredicateNode { set, predicate })) => {
                        tasks.push(Task::Every);
                        tasks.push(Task::Eval(predicate));
                        tasks.push(Task::Eval(set));
                    }
                    Some(Node::Any(PredicateNode { set, predicate })) => {
                        tasks.push(Task::Any);
                        tasks.push(Task::Eval(predicate));
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
                    None if let Object::Composite(Composite::Any(any_composite)) = object => {
                        let result = any_composite
                            .properties()
                            .map(|property| {
                                // TODO: DEBATE THIS BS
                                debug!("eval on property {property:?}");

                                let value =
                                    property.value.evaluate(statements, context).into_object();

                                let result = if property.value == value {
                                    Ok(())
                                } else {
                                    value.is_valid(statements, false)
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
                            .map_or_else(|(o, error)| {
                                warn!(
                                    "invalid object {o:?} with error {error:?}; replacing with {{}}"
                                );

                                ObjectOrSetValues::Object(Object::Composite(Composite::Empty))
                            }, |mut properties| {
                                Object::Composite(Composite::new(&mut properties)).into()
                            });

                        evaluated.push(result);
                    }
                    None => evaluated.push(object.clone().into()),
                },
                Task::PopContext => {
                    context.pop();
                }
                Task::Call => {
                    let callee = evaluated.pop().unwrap().into_object();
                    let parameter_value = evaluated.pop().unwrap();

                    tasks.push(Task::Eval(callee.node_function_body(statements).unwrap()));

                    context.push(FunctionContext {
                        function: callee,
                        parameter: parameter_value,
                    });
                }
                Task::PartialAnd { right } => {
                    let mut left = evaluated.pop().unwrap();

                    if left.is_truthy(statements) {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Eval(right));
                    } else {
                        evaluated
                            .push(ObjectOrSetValues::Object(Composite::new_bool(false).into()));
                    }
                }
                Task::Count => {
                    let target = evaluated.pop().unwrap();

                    evaluated.push(
                        Self::new_integer(target.set_values(statements).correct_count() as i128)
                            .into(),
                    );
                }
                Task::QuerySubjectsAndTags => {
                    let value = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QuerySubjectsAndTags(statements.query_subjects_and_tags(value))
                            .into(),
                    );
                }
                Task::QueryTagsAndValues => {
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QueryTagsAndValues(statements.query_tags_and_values(subject))
                            .into(),
                    );
                }
                Task::QueryTags => {
                    let value = evaluated.pop().unwrap().into_object();
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated
                        .push(SetValues::QueryTags(statements.query_tags(subject, value)).into());
                }
                Task::QueryValues => {
                    let tag = evaluated.pop().unwrap().into_object();
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::QueryValues(
                        statements.query_values(subject, tag.clone()),
                    )));
                }
                Task::QuerySubjects => {
                    let value = evaluated.pop().unwrap().into_object();
                    let tag = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QuerySubjects(statements.query_subjects(tag, value)).into(),
                    );
                }
                Task::QuerySubjectsAndValues => {
                    let tag = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QuerySubjectsAndValues(
                            statements.query_subjects_and_values(tag),
                        )
                        .into(),
                    );
                }
                Task::QueryExists => {
                    let value = evaluated.pop().unwrap().into_object();
                    let tag = evaluated.pop().unwrap().into_object();
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(ObjectOrSetValues::Object(
                        Composite::new_bool(statements.exists(SimpleStatement {
                            subject,
                            tag,
                            value,
                        }))
                        .into(),
                    ));
                }
                Task::ToBoolean => {
                    let mut object = evaluated.pop().unwrap();

                    evaluated.push(ObjectOrSetValues::Object(
                        Composite::new_bool(object.is_truthy(statements)).into(),
                    ));
                }
                Task::Equal => {
                    let right = evaluated.pop().unwrap().into_object();
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(Object::Composite(Composite::new_bool(left == right)).into());
                }
                Task::PartialOr { right } => {
                    if evaluated.pop().unwrap().is_truthy(statements) {
                        evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(true).into()));
                    } else {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Eval(right));
                    }
                }
                Task::Xor => {
                    let right = evaluated.pop().unwrap().is_truthy(statements);
                    let left = evaluated.pop().unwrap().is_truthy(statements);

                    evaluated.push(ObjectOrSetValues::Object(
                        Composite::new_bool((left || right) && !(left && right)).into(),
                    ));
                }
                Task::IsAbstract => {
                    let object = evaluated.pop().unwrap();

                    evaluated.push(
                        Object::Composite(Composite::new_bool(matches!(
                            object,
                            ObjectOrSetValues::Object(Object::Abstract(_))
                        )))
                        .into(),
                    );
                }
                Task::Not => {
                    let mut object = evaluated.pop().unwrap();

                    evaluated.push(ObjectOrSetValues::Object(
                        Composite::new_bool(!object.is_truthy(statements)).into(),
                    ));
                }
                Task::Add => {
                    let right = evaluated.pop().unwrap().into_object();
                    // TODO: (perf) maybe short circuit sets into UNDEFINED.
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(left.add(statements, &right).into());
                }
                Task::Multiply => {
                    // TODO: (perf) maybe short circuit sets into UNDEFINED.
                    let right = evaluated.pop().unwrap().into_object();
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(left.multiply(statements, &right).into());
                }
                Task::Union => {
                    let right = evaluated.pop().unwrap();
                    let left = evaluated.pop().unwrap();

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::Union {
                        left: Box::new(left.set_values(statements)),
                        right: Box::new(right.set_values(statements)),
                    }));
                }
                Task::Map => {
                    let mapper = evaluated.pop().unwrap().into_object();
                    let set = evaluated.pop().unwrap().set_values(statements);

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::Map {
                        statements: statements.clone(),
                        set: Box::new(set),
                        mapper_function: mapper,
                    }));
                }
                Task::Filter => {
                    let filter = evaluated.pop().unwrap().into_object();
                    let set = evaluated.pop().unwrap().set_values(statements);

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::Filter {
                        statements: statements.clone(),
                        set: Box::new(set),
                        filter_function: filter,
                    }));
                }
                Task::Every => {
                    let predicate = evaluated.pop().unwrap().into_object();
                    let mut set = evaluated.pop().unwrap().set_values(statements);

                    let every_value_matches_predicate = set.all(|value| {
                        predicate
                            .call(statements, &[value.into()], context)
                            .is_truthy(statements)
                    });

                    evaluated.push(ObjectOrSetValues::Object(
                        Composite::new_bool(every_value_matches_predicate).into(),
                    ));
                }
                Task::Any => {
                    let predicate = evaluated.pop().unwrap().into_object();
                    let mut set = evaluated.pop().unwrap().set_values(statements);

                    let any_value_matches_predicate = set.any(|value| {
                        predicate
                            .call(statements, &[value.into()], context)
                            .is_truthy(statements)
                    });

                    evaluated.push(ObjectOrSetValues::Object(
                        Composite::new_bool(any_value_matches_predicate).into(),
                    ));
                }
                Task::Less => {
                    let right = evaluated.pop().unwrap();
                    let left = evaluated.pop().unwrap();

                    evaluated.push(
                        Object::Composite(Composite::new_bool(match (left, right) {
                            (ObjectOrSetValues::Object(left), ObjectOrSetValues::Object(right))
                                if let Some(left) = left.to_integer(statements)
                                    && let Some(right) = right.to_integer(statements) =>
                            {
                                left < right
                            }
                            // TODO: more things here
                            (left, right) => {
                                left.set_values(statements).correct_count()
                                    < right.set_values(statements).correct_count()
                            }
                        }))
                        .into(),
                    );
                }
                Task::PartialIf { then, otherwise } => {
                    if evaluated.pop().unwrap().is_truthy(statements) {
                        tasks.push(Task::Eval(then));
                    } else {
                        tasks.push(Task::Eval(otherwise));
                    }
                }
                Task::PartialUnwrapOr { default } => {
                    let mut set_values = evaluated.pop().unwrap().set_values(statements);

                    if let Some(inner) = set_values.next_and_last() {
                        evaluated.push(ObjectOrSetValues::Object(inner));
                    } else {
                        tasks.push(Task::Eval(default));
                    }
                }
            }
        }

        let last = evaluated.pop().unwrap();
        if evaluated.pop().is_some() {
            unreachable!()
        }

        last
    }

    #[instrument(skip(statements), ret)]
    fn call(
        &self,
        statements: &Statements,
        parameters: &[ObjectOrSetValues],
        ctx: &mut EvaluationContext,
    ) -> ObjectOrSetValues {
        /*
        if self == &Object::Abstract(Abstract::KNOWLEDGE)
            && let Some(parameter) = parameters.first()
        {
            return match parameter.clone().into_object() {
                Object::Composite(composite) => {
                    Object::Composite(Composite::new_bool(composite.is_statements().is_ok()))
                }
                Object::Abstract(_) => Object::Composite(Composite::Empty),
            }
            .into();
        }
         */

        if let Some((parameter, next_parameters)) = parameters.split_first()
            && let Some(Node::Function(body)) = self.node(statements)
        {
            ctx.push(FunctionContext {
                function: self.clone(),
                parameter: parameter.clone(),
            });

            let result = body.call(statements, next_parameters, ctx);

            ctx.pop();

            result
        } else {
            self.evaluate(statements, ctx)
        }
    }

    #[instrument(skip(statements), ret)]
    fn to_integer(&self, statements: &Statements) -> Option<i128> {
        if let Some(n) = self.exact_integer() {
            // Fast path for exact natural numbers.
            Some(n)
        } else if let Some(predecessor) = statements
            .query_values(self.clone(), Abstract::SUCCESSOR_OF.into())
            .next_and_last()
        {
            predecessor
                .to_integer(statements)
                .map(|n| n.checked_add(1).expect("yo shi too big"))
        } else if let Some(successor) = statements
            .query_values(self.clone(), Abstract::PREDECESSOR_OF.into())
            .next_and_last()
        {
            successor
                .to_integer(statements)
                .map(|n| n.checked_sub(1).expect("yo shi too small"))
        } else {
            None
        }
    }

    fn is_valid(&self, statements: &Statements, recursive: bool) -> Result<(), KnowledgeError> {
        match self {
            Self::Abstract(_) => Ok(()),
            Self::Composite(composite) => composite.is_valid(statements, recursive),
        }
    }

    fn add(&self, statements: &Statements, other: &Object) -> Object {
        if let Some(left) = self.to_integer(statements)
            && let Some(right) = other.to_integer(statements)
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
