#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Property, Structure};
use fallible_iterator::{FallibleIterator, IteratorExt};
use tracing::{debug, instrument, warn};

use crate::{
    ObjectOrSetValues, SetValues,
    ctx::{EvaluationContext, FunctionContext},
    ext::{
        AbstractExt, KnowledgeError, ObjectForm, PropertyExt, Statement, StatementForm,
        StructureExt, iter::IteratorExtNextAndLast,
    },
    nodes::{BinaryNode, CallNode, FilterNode, IfNode, MapNode, Node, Task, UnwrapOrNode},
    query::{
        QueryExists, QuerySubjects, QuerySubjectsAndTags, QuerySubjectsAndValues, QueryTags,
        QueryTagsAndValues, QueryValues,
    },
};

/// An extension trait implemented for [`Object`], providing many useful functions.
pub trait ObjectExt {
    /// Extracts the first (and last) [Abstract::NODE_COUNT] from `self`.
    fn node_count(&self, knowledge: &Structure) -> Option<Object>;

    /// Extracts the first (and last) [Abstract::FUNCTION] from `self`.
    fn function_body(&self, knowledge: &Structure) -> Option<Object>;

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
    ) -> ObjectOrSetValues;

    /// Evaluates `self` under the given knowledge and evaluation context
    /// by reducing expressions.
    ///
    /// If you don't know what to pass into the context, pass
    /// `&mut Default::default()`.
    fn evaluate(&self, knowledge: &Structure, context: &mut EvaluationContext)
    -> ObjectOrSetValues;

    /// Parses a node from `self`.
    fn node(&self, knowledge: &Structure) -> Option<Node>;

    /// Returns an iterator over all set items.
    fn set_values(&self, knowledge: &Structure) -> QueryValues;

    fn structure(&self) -> Option<&Structure>;

    fn is_truthy(&self, knowledge: &Structure) -> bool;

    /// Calls `self` with a list of parameters.
    /// If none are provided, `self` will just be evaluated.
    ///
    /// Note that it does not evaluate any parameters.
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[ObjectOrSetValues],
        ctx: &mut EvaluationContext,
    ) -> ObjectOrSetValues;

    fn to_integer(&self, knowledge: &Structure) -> Option<i128>;

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<u64>;

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
    fn node_multiply(&self, knowledge: &Structure) -> Option<BinaryNode>;

    fn add(&self, knowledge: &Structure, other: &Object) -> Object;

    fn node_function_self(&self, knowledge: &Structure) -> Option<u64>;

    fn node_not(&self, knowledge: &Structure) -> Option<Object>;

    /// Parses a binary node by querying (axiomatically)
    /// for `left_tag` and `right_tag`.
    fn binary_node(
        &self,
        knowledge: &Structure,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode>;
    fn node_unwrap_or(&self, knowledge: &Structure) -> Option<UnwrapOrNode>;

    fn node_if(&self, knowledge: &Structure) -> Option<IfNode>;

    fn node_less(&self, knowledge: &Structure) -> Option<BinaryNode>;
    fn node_call(&self, knowledge: &Structure) -> Option<CallNode>;

    fn intrinsic_statement_subject(&self) -> Option<Object>;

    fn intrinsic_statement_tag(&self) -> Option<Object>;

    fn intrinsic_statement_value(&self) -> Option<Object>;

    fn intrinsic_statement(&self) -> Option<Statement>;

    fn multiply(&self, knowledge: &Structure, other: &Object) -> Object;

    fn new_node(node: Node) -> Self;

    /// Creates a new query node, set up for value querying.
    fn new_node_query_values(subject: Object, tag: Object) -> Self;
}

impl ObjectExt for Object {
    fn new_node_query_values(subject: Object, tag: Object) -> Self {
        Self::new_node(Node::Query(
            Structure::new(&mut [
                Property::new_statement_subject(subject),
                Property::new_statement_tag(tag),
            ])
            .into(),
        ))
    }

    fn new_node(node: Node) -> Self {
        match node {
            Node::Knowledge => Abstract::NODE_KNOWLEDGE.into(),
            Node::Call(CallNode { callee, with }) => Structure::new(&mut [
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
            Node::Function(body) => Structure::new(&mut [Property {
                tag: Abstract::FUNCTION.into(),
                value: body,
            }])
            .into(),
            Node::Literal(literal) => Structure::new(&mut [Property {
                tag: Abstract::NODE_LITERAL.into(),
                value: literal,
            }])
            .into(),
            Node::And(BinaryNode { left, right }) => Structure::new(&mut [
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
            Node::FunctionSelf(depth) => Structure::new(&mut [Property {
                tag: Abstract::NODE_FUNCTION_SELF.into(),
                value: Object::new_integer(depth as i128),
            }])
            .into(),
            Node::Parameter(depth) => Structure::new(&mut [Property {
                tag: Abstract::NODE_PARAMETER.into(),
                value: Object::new_integer(depth as i128),
            }])
            .into(),
            Node::Count(object) => Structure::new(&mut [Property {
                tag: Abstract::NODE_COUNT.into(),
                value: object,
            }])
            .into(),
            Node::Query(query) => Structure::new(&mut [Property {
                tag: Abstract::NODE_QUERY.into(),
                value: query,
            }])
            .into(),
            Node::Equal(BinaryNode { left, right }) => Structure::new(&mut [
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
            Node::Or(BinaryNode { left, right }) => Structure::new(&mut [
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
            Node::Xor(BinaryNode { left, right }) => Structure::new(&mut [
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
            Node::Not(node) => Structure::new(&mut [Property {
                tag: Abstract::NODE_NOT.into(),
                value: node,
            }])
            .into(),
            Node::Add(BinaryNode { left, right }) => Structure::new(&mut [
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
            Node::Union(BinaryNode { left, right }) => Structure::new(&mut [
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
            }) => Structure::new(&mut [
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
            }) => Structure::new(&mut [
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
            Node::Less(BinaryNode { left, right }) => Structure::new(&mut [
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
            }) => Structure::new(&mut [
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
            Node::UnwrapOr(UnwrapOrNode { set, default }) => Structure::new(&mut [
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
            Node::Multiply(BinaryNode { left, right }) => Structure::new(&mut [
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

    fn is_natural_number(&self, knowledge: &Structure) -> bool {
        if self.exact_integer().is_some() {
            // Fast path of exact natural numbers.
            return true;
        }

        let mut successor_of =
            QueryValues::new(knowledge, self.clone(), Abstract::SUCCESSOR_OF.into());

        if let Some(first) = successor_of.next()
            && successor_of.next().is_none()
        {
            first.is_natural_number(knowledge)
        } else {
            false
        }
    }

    fn set_values(&self, knowledge: &Structure) -> QueryValues {
        QueryValues::new(knowledge, self.clone(), Abstract::CONTAINS.into())
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
            QueryValues::new(knowledge, self.clone(), Abstract::NODE_MAP_SET.into()).next()?;
        let mapper_function_expression =
            QueryValues::new(knowledge, self.clone(), Abstract::NODE_MAP_MAPPER.into()).next()?;

        Some(MapNode {
            set: set_expression,
            mapper_function: mapper_function_expression,
        })
    }

    fn node_filter(&self, knowledge: &Structure) -> Option<FilterNode> {
        let set = QueryValues::new(knowledge, self.clone(), Abstract::NODE_FILTER_SET.into())
            .next_and_last()?;

        let filter = QueryValues::new(knowledge, self.clone(), Abstract::NODE_FILTER_FILTER.into())
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

    fn node_multiply(&self, knowledge: &Structure) -> Option<BinaryNode> {
        self.binary_node(
            knowledge,
            Abstract::NODE_MULTIPLY_LEFT.into(),
            Abstract::NODE_MULTIPLY_RIGHT.into(),
        )
    }

    fn binary_node(
        &self,
        knowledge: &Structure,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode> {
        let left = QueryValues::new(knowledge, self.clone(), left_tag).next()?;
        let right = QueryValues::new(knowledge, self.clone(), right_tag).next()?;

        Some(BinaryNode { left, right })
    }

    #[instrument(skip(knowledge), ret)]
    fn node(&self, knowledge: &Structure) -> Option<Node> {
        let mut node = self.function_body(knowledge).map(Node::Function);

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

        xor_with!(self.node_literal(knowledge).map(Node::Literal));
        xor_with!(self.node_function_self(knowledge).map(Node::FunctionSelf));
        xor_with!(self.node_parameter_depth(knowledge).map(Node::Parameter));
        xor_with!(self.node_call(knowledge).map(Node::Call));
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
        xor_with!(self.node_unwrap_or(knowledge).map(Node::UnwrapOr));
        xor_with!(self.node_multiply(knowledge).map(Node::Multiply));

        if self == &Self::Abstract(Abstract::NODE_KNOWLEDGE) {
            if node.is_some() {
                return None;
            }

            node = Some(Node::Knowledge);
        }

        node
    }

    fn node_unwrap_or(&self, knowledge: &Structure) -> Option<UnwrapOrNode> {
        let set = QueryValues::new(knowledge, self.clone(), Abstract::NODE_UNWRAP_OR_SET.into())
            .next_and_last()?;

        let default = QueryValues::new(
            knowledge,
            self.clone(),
            Abstract::NODE_UNWRAP_OR_DEFAULT.into(),
        )
        .next_and_last()?;

        Some(UnwrapOrNode { set, default })
    }

    fn node_if(&self, knowledge: &Structure) -> Option<IfNode> {
        let condition =
            QueryValues::new(knowledge, self.clone(), Abstract::NODE_IF_CONDITION.into())
                .next_and_last()?;

        let then = QueryValues::new(knowledge, self.clone(), Abstract::NODE_IF_THEN.into())
            .next_and_last()?;

        let otherwise = QueryValues::new(knowledge, self.clone(), Abstract::NODE_IF_ELSE.into())
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

    fn node_call(&self, knowledge: &Structure) -> Option<CallNode> {
        let callee = QueryValues::new(knowledge, self.clone(), Abstract::NODE_CALL_CALLEE.into())
            .next_and_last()?;

        let with = QueryValues::new(knowledge, self.clone(), Abstract::NODE_CALL_WITH.into())
            .next_and_last()?;

        Some(CallNode { callee, with })
    }

    fn node_not(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(knowledge, self.clone(), Abstract::NODE_NOT.into()).next_and_last()
    }

    fn node_count(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(knowledge, self.clone(), Abstract::NODE_COUNT.into()).next_and_last()
    }

    fn function_body(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(knowledge, self.clone(), Abstract::FUNCTION.into()).next_and_last()
    }

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<u64> {
        let depth = QueryValues::new(knowledge, self.clone(), Abstract::NODE_PARAMETER.into())
            .next_and_last()?
            .to_integer(knowledge)?;

        u64::try_from(depth).ok()
    }

    fn node_literal(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(knowledge, self.clone(), Abstract::NODE_LITERAL.into()).next_and_last()
    }

    fn node_function_self(&self, knowledge: &Structure) -> Option<u64> {
        let depth = QueryValues::new(knowledge, self.clone(), Abstract::NODE_FUNCTION_SELF.into())
            .next_and_last()?
            .to_integer(knowledge)?;

        u64::try_from(depth).ok()
    }

    #[inline]
    fn intrinsic_statement_subject(&self) -> Option<Object> {
        match self {
            Object::Abstract(_) => None,
            Object::Structure(structure) => structure
                .values(Abstract::STATEMENT_SUBJECT.into())
                .next_and_last(),
        }
    }

    #[inline]
    fn intrinsic_statement_tag(&self) -> Option<Object> {
        match self {
            Object::Abstract(_) => None,
            Object::Structure(structure) => structure
                .values(Abstract::STATEMENT_TAG.into())
                .next_and_last(),
        }
    }

    #[inline]
    fn intrinsic_statement_value(&self) -> Option<Object> {
        match self {
            Object::Abstract(_) => None,
            Object::Structure(structure) => structure
                .values(Abstract::STATEMENT_VALUE.into())
                .next_and_last(),
        }
    }

    fn intrinsic_statement(&self) -> Option<Statement> {
        let subject = self.intrinsic_statement_subject()?;
        let tag = self.intrinsic_statement_tag()?;
        let value = self.intrinsic_statement_value()?;

        Some(Statement {
            subject,
            tag,
            value,
        })
    }

    fn statement_subject(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(
            knowledge,
            self.clone(),
            Object::Abstract(Abstract::STATEMENT_SUBJECT),
        )
        .next_and_last()
    }

    fn statement_tag(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(
            knowledge,
            self.clone(),
            Object::Abstract(Abstract::STATEMENT_TAG),
        )
        .next_and_last()
    }

    fn statement_value(&self, knowledge: &Structure) -> Option<Object> {
        QueryValues::new(
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
        QueryValues::new(
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
    ) -> ObjectOrSetValues {
        match self.node(knowledge) {
            Some(Node::Function(body)) => Self::new_node(Node::Function(
                body.capture(knowledge, additional_depth + 1, ctx)
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
                Self::Structure(Structure::Any(structure)) => structure
                    .as_ref()
                    .iter()
                    .map(|property| {
                        let value = property
                            .value
                            .capture(knowledge, additional_depth, ctx)
                            .into_object();

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
                        ObjectOrSetValues::Object(Self::Structure(Structure::new(&mut properties)))
                    })
                    .unwrap_or_else(|(o, error)| {
                        warn!("invalid object {o:?} with error {error:?}; replacing with {{}}");

                        ObjectOrSetValues::Object(Structure::Empty.into())
                    }),
                _ => ObjectOrSetValues::Object(self.clone()),
            },
        }
    }

    fn multiply(&self, knowledge: &Structure, other: &Object) -> Object {
        if let Some(left) = self.to_integer(knowledge)
            && let Some(right) = other.to_integer(knowledge)
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

    #[instrument(skip(knowledge), ret)]
    fn evaluate(
        &self,
        knowledge: &Structure,
        context: &mut EvaluationContext,
    ) -> ObjectOrSetValues {
        let mut tasks = vec![Task::Eval(self.clone())];
        let mut evaluated = Vec::<ObjectOrSetValues>::new();

        while let Some(task) = tasks.pop() {
            debug!("doing task {task:?}");

            match task {
                Task::Eval(object) => match object.node(knowledge) {
                    Some(Node::Knowledge) => {
                        evaluated.push(ObjectOrSetValues::Object(knowledge.clone().into()));
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
                        evaluated.push(object.capture(knowledge, 0, context).into());
                    }
                    Some(Node::Literal(object)) => {
                        evaluated.push(object.into());
                    }
                    Some(Node::And(BinaryNode { left, right })) => {
                        tasks.push(Task::PartialAnd { right });
                        tasks.push(Task::Eval(left));
                    }
                    Some(Node::FunctionSelf(depth)) => evaluated.push(ObjectOrSetValues::Object(
                        context.function_self(depth as usize).into(),
                    )),
                    Some(Node::Parameter(depth)) => {
                        evaluated.push(context.parameter_value(depth as usize));
                    }
                    Some(Node::Count(object)) => {
                        tasks.push(Task::Count);
                        tasks.push(Task::Eval(object));
                    }
                    Some(Node::Query(object)) => match object.statement_form(knowledge) {
                        StatementForm {
                            subject: ObjectForm::Any,
                            tag: ObjectForm::Any,
                            value: ObjectForm::Any,
                        } => {
                            todo!("hell nah")
                        }
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
                            tasks.push(Task::QuerySubjects);
                            tasks.push(Task::Eval(unevaluated_value));
                            tasks.push(Task::Eval(unevaluated_tag));
                        }
                        StatementForm {
                            subject: ObjectForm::Any,
                            tag: ObjectForm::Specific(unevaluated_tag),
                            value: ObjectForm::Any,
                        } => {
                            tasks.push(Task::QuerySubjectsAndValues);
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
                        StatementForm {
                            subject: ObjectForm::Specific(unevaluated_subject),
                            tag: ObjectForm::Any,
                            value: ObjectForm::Any,
                        } => {
                            tasks.push(Task::QueryTagsAndValues);
                            tasks.push(Task::Eval(unevaluated_subject));
                        }
                        StatementForm {
                            subject: ObjectForm::Any,
                            tag: ObjectForm::Any,
                            value: ObjectForm::Specific(unevaluated_value),
                        } => {
                            tasks.push(Task::QuerySubjectsAndTags);
                            tasks.push(Task::Eval(unevaluated_value));
                        }
                        StatementForm {
                            subject: ObjectForm::Specific(unevaluated_subject),
                            tag: ObjectForm::Any,
                            value: ObjectForm::Specific(unevaluated_value),
                        } => {
                            tasks.push(Task::QueryTags);
                            tasks.push(Task::Eval(unevaluated_value));
                            tasks.push(Task::Eval(unevaluated_subject));
                        }
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

                                let value =
                                    property.value.evaluate(knowledge, context).into_object();

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

                                ObjectOrSetValues::Object(Object::Structure(Structure::Empty))
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

                    tasks.push(Task::Eval(callee.function_body(knowledge).unwrap()));

                    context.push(FunctionContext {
                        function: callee,
                        parameter: parameter_value,
                    });
                }
                Task::PartialAnd { right } => {
                    let mut left = evaluated.pop().unwrap();

                    if left.is_truthy(knowledge) {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Eval(right));
                    } else {
                        evaluated
                            .push(ObjectOrSetValues::Object(Structure::new_bool(false).into()));
                    }
                }
                Task::Count => {
                    let target = evaluated.pop().unwrap();

                    evaluated.push(
                        Self::new_integer(target.set_values(knowledge).correct_count() as i128)
                            .into(),
                    );
                }
                Task::QuerySubjectsAndTags => {
                    let value = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QuerySubjectsAndTags(QuerySubjectsAndTags::new(
                            knowledge, value,
                        ))
                        .into(),
                    );
                }
                Task::QueryTagsAndValues => {
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QueryTagsAndValues(QueryTagsAndValues::new(knowledge, subject))
                            .into(),
                    );
                }
                Task::QueryTags => {
                    let value = evaluated.pop().unwrap().into_object();
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QueryTags(QueryTags::new(knowledge, subject, value)).into(),
                    );
                }
                Task::QueryValues => {
                    let tag = evaluated.pop().unwrap().into_object();
                    // TODO: make this lazy
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::QueryValues(
                        QueryValues::new(knowledge, subject, tag.clone()),
                    )));
                }
                Task::QuerySubjects => {
                    let value = evaluated.pop().unwrap().into_object();
                    let tag = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QuerySubjects(QuerySubjects::new(knowledge, tag, value)).into(),
                    );
                }
                Task::QuerySubjectsAndValues => {
                    let tag = evaluated.pop().unwrap().into_object();

                    evaluated.push(
                        SetValues::QuerySubjectsAndValues(QuerySubjectsAndValues::new(
                            knowledge, tag,
                        ))
                        .into(),
                    );
                }
                Task::QueryExists => {
                    let value = evaluated.pop().unwrap().into_object();
                    let tag = evaluated.pop().unwrap().into_object();
                    let subject = evaluated.pop().unwrap().into_object();

                    evaluated.push(ObjectOrSetValues::Object(
                        Structure::new_bool(QueryExists::new(
                            knowledge,
                            subject,
                            tag.clone(),
                            value,
                        ))
                        .into(),
                    ))
                }
                Task::ToBoolean => {
                    let mut object = evaluated.pop().unwrap();

                    evaluated.push(ObjectOrSetValues::Object(
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
                        evaluated.push(ObjectOrSetValues::Object(Structure::new_bool(true).into()));
                    } else {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Eval(right));
                    }
                }
                Task::Xor => {
                    let right = evaluated.pop().unwrap().is_truthy(knowledge);
                    let left = evaluated.pop().unwrap().is_truthy(knowledge);

                    evaluated.push(ObjectOrSetValues::Object(
                        Structure::new_bool((left || right) && !(left && right)).into(),
                    ));
                }
                Task::Not => {
                    let mut object = evaluated.pop().unwrap();

                    evaluated.push(ObjectOrSetValues::Object(
                        Structure::new_bool(!object.is_truthy(knowledge)).into(),
                    ));
                }
                Task::Add => {
                    let right = evaluated.pop().unwrap().into_object();
                    // TODO: (perf) maybe short circuit sets into UNDEFINED.
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(left.add(knowledge, &right).into());
                }
                Task::Multiply => {
                    // TODO: (perf) maybe short circuit sets into UNDEFINED.
                    let right = evaluated.pop().unwrap().into_object();
                    let left = evaluated.pop().unwrap().into_object();

                    evaluated.push(left.multiply(knowledge, &right).into())
                }
                Task::Union => {
                    let right = evaluated.pop().unwrap();
                    let left = evaluated.pop().unwrap();

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::Union {
                        left: Box::new(left.set_values(knowledge)),
                        right: Box::new(right.set_values(knowledge)),
                    }));
                }
                Task::Map => {
                    let mapper = evaluated.pop().unwrap().into_object();
                    let set = evaluated.pop().unwrap().set_values(knowledge);

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::Map {
                        knowledge: knowledge.clone(),
                        set: Box::new(set),
                        mapper_function: mapper,
                    }));
                }
                Task::Filter => {
                    let filter = evaluated.pop().unwrap().into_object();
                    let set = evaluated.pop().unwrap().set_values(knowledge);

                    evaluated.push(ObjectOrSetValues::SetValues(SetValues::Filter {
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
                            (ObjectOrSetValues::Object(left), ObjectOrSetValues::Object(right))
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
                Task::PartialUnwrapOr { default } => {
                    let mut set_values = evaluated.pop().unwrap().set_values(knowledge);

                    if let Some(inner) = set_values.next_and_last() {
                        evaluated.push(ObjectOrSetValues::Object(inner));
                    } else {
                        tasks.push(Task::Eval(default))
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

    #[instrument(skip(knowledge), ret)]
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[ObjectOrSetValues],
        ctx: &mut EvaluationContext,
    ) -> ObjectOrSetValues {
        if self == &Object::Abstract(Abstract::KNOWLEDGE)
            && let Some(parameter) = parameters.first()
        {
            return match parameter.clone().into_object() {
                Object::Structure(structure) => {
                    Object::Structure(Structure::new_bool(structure.is_knowledge().is_ok()))
                }
                Object::Abstract(_) => Object::Structure(Structure::Empty),
            }
            .into();
        }

        if let Some((parameter, next_parameters)) = parameters.split_first()
            && let Some(Node::Function(body)) = self.node(knowledge)
        {
            ctx.push(FunctionContext {
                function: self.clone(),
                parameter: parameter.clone(),
            });

            let result = body.call(knowledge, next_parameters, ctx);

            ctx.pop();

            result
        } else {
            self.evaluate(knowledge, ctx)
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn to_integer(&self, knowledge: &Structure) -> Option<i128> {
        if let Some(n) = self.exact_integer() {
            // Fast path for exact natural numbers.
            Some(n)
        } else if let Some(predecessor) =
            QueryValues::new(knowledge, self.clone(), Abstract::SUCCESSOR_OF.into()).next_and_last()
        {
            predecessor
                .to_integer(knowledge)
                .map(|n| n.checked_add(1).expect("yo shi too big"))
        } else if let Some(successor) =
            QueryValues::new(knowledge, self.clone(), Abstract::PREDECESSOR_OF.into())
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
