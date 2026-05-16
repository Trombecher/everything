#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Property, Structure};
use fallible_iterator::{FallibleIterator, IteratorExt};
use tracing::{instrument, warn};

use crate::{
    ObjectOrAxiomaticQueryValues,
    ctx::{EvaluationContext, FunctionContext},
    ext::{
        AbstractExt, BinaryNode, KnowledgeError, NodeType, ObjectForm, StatementForm, StructureExt,
        iter::IteratorExtNextAndLast,
    },
    query::{self, AxiomaticQueryValues},
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

    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object;

    fn eval<'knowledge, 'subject>(
        &'subject self,
        knowledge: &'knowledge Structure,
        context: &mut EvaluationContext,
    ) -> ObjectOrAxiomaticQueryValues<'knowledge, 'subject>;

    fn node_type(&self, knowledge: &Structure) -> Option<NodeType>;

    /// Returns an iterator over all set items.
    fn set_values<'subject, 'knowledge>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> AxiomaticQueryValues<'knowledge, 'subject>;

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

    fn to_integer(&self, knowledge: &Structure) -> Option<i128>;

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<u128>;

    fn node_literal(&self, knowledge: &Structure) -> Option<Object>;

    fn statement_form(&self, knowledge: &Structure) -> StatementForm;

    fn node_query(&self, knowledge: &Structure) -> Option<Object>;

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError>;

    fn is_natural_number(&self, knowledge: &Structure) -> bool;

    fn add(&self, knowledge: &Structure, other: &Object) -> Object;

    fn node_function_self(&self, knowledge: &Structure) -> Option<Object>;

    fn node_not(&self, knowledge: &Structure) -> Option<Object>;

    /// Parses a binary node by querying (axiomatically)
    /// for `left_tag` and `right_tag`.
    fn binary_node(
        &self,
        knowledge: &Structure,
        left_tag: Object,
        right_tag: Object,
    ) -> Option<BinaryNode>;
}

impl ObjectExt for Object {
    fn is_natural_number(&self, knowledge: &Structure) -> bool {
        if self.exact_integer().is_some() {
            // Fast path of exact natural numbers.
            return true;
        }

        let mut successor_of =
            query::values_axiomatically(knowledge, self, Abstract::SUCCESSOR_OF.into());

        if let Some(first) = successor_of.next()
            && successor_of.next().is_none()
        {
            first.is_natural_number(knowledge)
        } else {
            false
        }
    }

    fn set_values<'subject, 'knowledge>(
        &'subject self,
        knowledge: &'knowledge Structure,
    ) -> AxiomaticQueryValues<'knowledge, 'subject> {
        query::values_axiomatically(knowledge, self, Abstract::CONTAINS.into())
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
        let left = query::values_axiomatically(knowledge, self, left_tag).next()?;
        let right = query::values_axiomatically(knowledge, self, right_tag).next()?;

        Some(BinaryNode { left, right })
    }

    #[instrument(skip(knowledge), ret)]
    fn node_type(&self, knowledge: &Structure) -> Option<NodeType> {
        let mut node_type = self.computed_body(knowledge).map(NodeType::Computed);

        macro_rules! xor_with {
            ($e:expr) => {{
                let variant = $e;

                if variant.is_some() {
                    if node_type.is_some() {
                        return None;
                    } else {
                        node_type = variant;
                    }
                }
            }};
        }

        xor_with!(self.node_literal(knowledge).map(NodeType::Literal));
        xor_with!(
            self.node_function_self(knowledge)
                .map(NodeType::FunctionSelf)
        );
        xor_with!(
            self.node_parameter_depth(knowledge)
                .map(NodeType::Parameter)
        );
        xor_with!(self.node_count(knowledge).map(NodeType::Count));
        xor_with!(self.node_query(knowledge).map(NodeType::Query));
        xor_with!(self.node_not(knowledge).map(NodeType::Not));
        xor_with!(self.node_and(knowledge).map(NodeType::And));
        xor_with!(self.node_or(knowledge).map(NodeType::Or));
        xor_with!(self.node_equal(knowledge).map(NodeType::Equal));
        xor_with!(self.node_xor(knowledge).map(NodeType::Xor));
        xor_with!(self.node_add(knowledge).map(NodeType::Add));

        node_type
    }

    fn node_not(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self, Abstract::NODE_NOT.into()).next_and_last()
    }

    fn node_count(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self, Abstract::NODE_COUNT.into()).next_and_last()
    }

    fn computed_body(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self, Abstract::COMPUTED.into()).next_and_last()
    }

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<u128> {
        let depth = query::values_axiomatically(knowledge, self, Abstract::NODE_PARAMETER.into())
            .next_and_last()?
            .to_integer(knowledge)?;

        if depth >= 0 {
            Some(depth as u128)
        } else {
            None
        }
    }

    fn node_literal(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self, Abstract::NODE_LITERAL.into()).next_and_last()
    }

    fn node_function_self(&self, knowledge: &Structure) -> Option<Object> {
        query::values_axiomatically(knowledge, self, Abstract::NODE_FUNCTION_SELF.into())
            .next_and_last()
    }

    fn statement_form(&self, knowledge: &Structure) -> StatementForm {
        let subject: ObjectForm = query::values_axiomatically(
            knowledge,
            self,
            Object::Abstract(Abstract::STATEMENT_SUBJECT),
        )
        .next()
        .into();

        let tag: ObjectForm =
            query::values_axiomatically(knowledge, self, Object::Abstract(Abstract::STATEMENT_TAG))
                .next()
                .into();

        let value: ObjectForm = query::values_axiomatically(
            knowledge,
            self,
            Object::Abstract(Abstract::STATEMENT_VALUE),
        )
        .next()
        .into();

        StatementForm {
            subject,
            tag,
            value,
        }
    }

    fn node_query(&self, knowledge: &Structure) -> Option<Self> {
        query::values_axiomatically(knowledge, self, Object::Abstract(Abstract::NODE_QUERY)).next()
    }

    #[instrument(skip(knowledge), ret)]
    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object {
        match self.node_type(knowledge) {
            Some(NodeType::Computed(body)) => {
                Structure::new_computed(body.capture(knowledge, additional_depth + 1, ctx)).into()
            }
            Some(NodeType::Parameter(depth)) => {
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
    fn eval<'knowledge, 'subject>(
        &'subject self,
        knowledge: &'knowledge Structure,
        context: &mut EvaluationContext,
    ) -> ObjectOrAxiomaticQueryValues<'knowledge, 'subject> {
        // TODO: better panic msgs

        match self.node_type(knowledge) {
            Some(NodeType::Count(expression)) => Self::new_integer(
                expression
                    .eval(knowledge, context)
                    .set_values(knowledge)
                    .count() as i128,
            )
            .into(),
            Some(NodeType::Query(statement_form_object)) => {
                // TODO: adjust constraint for query

                match statement_form_object.statement_form(knowledge) {
                    StatementForm {
                        subject: ObjectForm::Specific(unevaluated_subject),
                        tag: ObjectForm::Specific(unevaluated_tag),
                        value: ObjectForm::Any,
                    } => {
                        // We query for values.

                        let subject = unevaluated_subject.eval(knowledge, context).into_set();
                        let tag = unevaluated_tag.eval(knowledge, context).into_set();

                        query::values(knowledge, &subject, tag, context).into()
                    }
                    StatementForm {
                        subject: ObjectForm::Specific(unevaluated_subject),
                        tag: ObjectForm::Specific(unevaluated_tag),
                        value: ObjectForm::Specific(unevaluated_value),
                    } => {
                        // We check if the statement exists.

                        let subject = unevaluated_subject.eval(knowledge, context).into_set();
                        let tag = unevaluated_tag.eval(knowledge, context).into_set();
                        let value = unevaluated_value.eval(knowledge, context).into_set();

                        ObjectOrAxiomaticQueryValues::Object(
                            Structure::new_bool(query::exists(
                                knowledge, &subject, tag, value, context,
                            ))
                            .into(),
                        )
                    }
                    _ => todo!(),
                }
            }
            Some(NodeType::Literal(literal)) => literal.into(),
            Some(NodeType::Computed(_)) => self.capture(knowledge, 0, context).into(),
            Some(NodeType::Parameter(depth)) => context.parameter_value(depth as usize).into(),
            Some(NodeType::Equal(BinaryNode { left, right })) => {
                Object::Structure(Structure::new_bool(
                    left.eval(knowledge, context).into_set()
                        == right.eval(knowledge, context).into_set(),
                ))
                .into()
            }
            Some(NodeType::Or(BinaryNode { left, right })) => {
                Object::Structure(if left.eval(knowledge, context).is_truthy(knowledge) {
                    // Short circuit (if the left side is "true")
                    Structure::new_bool(true)
                } else {
                    Structure::new_bool(right.eval(knowledge, context).is_truthy(knowledge)).into()
                })
                .into()
            }
            Some(NodeType::And(BinaryNode { left, right })) => {
                Object::Structure(Structure::new_bool(
                    left.eval(knowledge, context).is_truthy(knowledge)
                        && right.eval(knowledge, context).is_truthy(knowledge),
                ))
                .into()
            }
            Some(NodeType::Not(expression)) => Object::Structure(Structure::new_bool(
                !expression.eval(knowledge, context).is_truthy(knowledge),
            ))
            .into(),
            Some(NodeType::Add(BinaryNode { left, right })) => {
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let left = left.eval(knowledge, context).into_set();
                let right = right.eval(knowledge, context).into_set();

                left.add(knowledge, &right).into()
            }
            Some(ty) => todo!("{ty:?} not impl"),
            None if let Object::Structure(Structure::Any(any_structure)) = self => any_structure
                .properties()
                .map(|property| {
                    let value = property.value.eval(knowledge, context).into_set();

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
                .map(|mut properties| Object::Structure(Structure::new(&mut properties)).into())
                .unwrap_or_else(|(o, error)| {
                    warn!("invalid object {o:?} with error {error:?}; replacing with {{}}");

                    ObjectOrAxiomaticQueryValues::Object(Object::Structure(Structure::Empty))
                }),
            None => self.clone().into(),
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
            && let Some(NodeType::Computed(body)) = self.node_type(knowledge)
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
            query::values_axiomatically(knowledge, self, Abstract::SUCCESSOR_OF.into())
                .next_and_last()
        {
            predecessor
                .to_integer(knowledge)
                .map(|n| n.checked_add(1).expect("yo shi too big"))
        } else if let Some(successor) =
            query::values_axiomatically(knowledge, self, Abstract::PREDECESSOR_OF.into())
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
