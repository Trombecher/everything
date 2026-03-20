#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};
use tracing::instrument;

use crate::{
    ctx::{EvaluationContext, FunctionContext},
    ext::{NodeType, ObjectForm, StatementForm, StructureExt},
    query::query_values,
};

macro_rules! define_abstract {
    ($($id:ident = $n:literal),* $(,)?) => {
        $(const $id: Object = Object::Abstract($n);)*
    };
}

pub trait ObjectExt {
    fn node_exists(&self, knowledge: &Structure) -> Option<Object>;
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
        ZERO = 9,
        SUCCESSOR_OF = 10,
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

    /// Extracts the first [Self::NODE_COUNT] from `self`.
    fn node_count(&self, knowledge: &Structure) -> Option<Object>;

    /// Extracts the first [Self::COMPUTED] from `self`.
    fn computed_body(&self, knowledge: &Structure) -> Option<Object>;

    fn capture(
        &self,
        knowledge: &Structure,
        additional_depth: usize,
        ctx: &EvaluationContext,
    ) -> Object;

    fn eval(&self, knowledge: &Structure, ctx: &mut EvaluationContext) -> Object;

    fn node_type(&self, knowledge: &Structure) -> Option<NodeType>;

    fn is_only_natural_number(&self) -> bool;

    /// Constructs a natural number object using
    /// repeated succ.
    fn natural_number(n: usize) -> Self;

    /// Returns the number of properties this object has.
    /// For abstract objects, this returns zero.
    fn property_count(&self) -> usize;

    /// Converts a boolean to an object.
    ///
    /// ```plain
    /// true |-> {(@1, {})}
    /// false |-> {}
    /// ```
    fn from_bool(b: bool) -> Object;

    /// Constructs a new set containing only `self`.
    fn to_set_of_self(self) -> Structure;

    fn structure(&self) -> Option<&Structure>;

    fn is_truthy(&self) -> bool;

    /// Calls `self` with a list of parameters.
    /// If none are provided, `self` will just be evaluated.
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[Object],
        ctx: &mut EvaluationContext,
    ) -> Object;

    fn to_natural_number(&self, knowledge: &Structure) -> Option<usize>;

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<usize>;

    fn node_literal(&self, knowledge: &Structure) -> Option<Object>;

    fn statement_form(&self, knowledge: &Structure) -> StatementForm;

    fn node_query(&self, knowledge: &Structure) -> Option<Object>;
}

impl ObjectExt for Object {
    fn is_only_natural_number(&self) -> bool {
        match self {
            &Object::ZERO => true,
            Object::Structure(s) => {
                if let [
                    Property {
                        tag: Object::SUCCESSOR_OF,
                        value,
                    },
                ] = s.as_ref()
                {
                    value.is_only_natural_number()
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn natural_number(n: usize) -> Self {
        if n == 0 {
            Object::ZERO
        } else {
            Structure::new(&mut [Property {
                tag: Object::SUCCESSOR_OF,
                value: Self::natural_number(n - 1),
            }])
            .into()
        }
    }

    fn property_count(&self) -> usize {
        match self {
            Object::Abstract(_) => 0,
            Object::Structure(structure) => structure.as_ref().len(),
        }
    }

    fn from_bool(b: bool) -> Self {
        if b {
            Self::to_set_of_self(Structure::EMPTY.into())
        } else {
            Structure::EMPTY
        }
        .into()
    }

    fn to_set_of_self(self) -> Structure {
        Structure::new(&mut [Property {
            tag: Object::CONTAINS,
            value: self,
        }])
    }

    fn structure(&self) -> Option<&Structure> {
        match self {
            Self::Abstract(_) => None,
            Self::Structure(structure) => Some(structure),
        }
    }

    // TODO: discuss abstract objects
    fn is_truthy(&self) -> bool {
        match self {
            Self::Abstract(_) => false,
            Self::Structure(structure) => !structure.as_ref().is_empty(),
        }
    }

    fn node_type(&self, knowledge: &Structure) -> Option<NodeType> {
        let mut current_pick = None;

        for node_type in NodeType::ALL {
            let node_type_object: Object = node_type.into();

            // We do not need an evaluation context because all node types are axiomatic.
            let query = query_values(knowledge, self, node_type_object, &mut Default::default());
            let there_are_values = query.iter().next().is_some();

            if there_are_values {
                if current_pick.is_some() {
                    // multiple node types apply
                    return None;
                }

                current_pick = Some(node_type);
            }
        }

        current_pick
    }

    fn node_count(&self, knowledge: &Structure) -> Option<Object> {
        let qr = query_values(
            knowledge,
            self,
            Object::NODE_COUNT,
            &mut EvaluationContext::default(),
        );

        qr.iter().next().cloned()
    }

    fn computed_body(&self, knowledge: &Structure) -> Option<Object> {
        let qr = query_values(
            knowledge,
            self,
            Object::COMPUTED,
            // We can pass in an empty evaluation context because it won't be used
            // (COMPUTED is axiomatic and therefore won't need the compuation
            // pipeline).
            &mut EvaluationContext::default(),
        );

        qr.iter().next().cloned()
    }

    fn node_parameter_depth(&self, knowledge: &Structure) -> Option<usize> {
        let depth_qr = query_values(
            knowledge,
            self,
            Object::NODE_PARAMETER,
            // We won't need that.
            &mut EvaluationContext::default(),
        );

        depth_qr
            .iter()
            .next()
            .and_then(|depth| depth.to_natural_number(knowledge))
    }

    fn node_literal(&self, knowledge: &Structure) -> Option<Object> {
        let qr = query_values(
            knowledge,
            self,
            Object::NODE_LITERAL,
            &mut EvaluationContext::default(),
        );

        qr.iter().next().cloned()
    }

    fn statement_form(&self, knowledge: &Structure) -> StatementForm {
        let subject_qr = query_values(
            knowledge,
            self,
            Object::STATEMENT_SUBJECT,
            &mut EvaluationContext::default(),
        );
        let subject: ObjectForm = subject_qr.iter().next().cloned().into();

        let tag_qr = query_values(
            knowledge,
            self,
            Object::STATEMENT_TAG,
            &mut EvaluationContext::default(),
        );
        let tag: ObjectForm = tag_qr.iter().next().cloned().into();

        let value_qr = query_values(
            knowledge,
            self,
            Object::STATEMENT_VALUE,
            &mut EvaluationContext::default(),
        );
        let value: ObjectForm = value_qr.iter().next().cloned().into();

        StatementForm {
            subject,
            tag,
            value,
        }
    }

    fn node_query(&self, knowledge: &Structure) -> Option<Object> {
        let qr = query_values(
            knowledge,
            self,
            Object::NODE_QUERY,
            &mut EvaluationContext::default(),
        );
        qr.iter().next().cloned()
    }

    fn node_exists(&self, knowledge: &Structure) -> Option<Object> {
        let qr = query_values(
            knowledge,
            self,
            Object::NODE_EXISTS,
            &mut EvaluationContext::default(),
        );
        qr.iter().next().cloned()
    }

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

                if depth >= additional_depth {
                    // The min additional depth is 1.
                    // So when the parameter depth is 1 it will refer to
                    // captured parameters at an additional depth of 1.

                    ctx.parameter_value(depth)
                } else {
                    // This parameter refers to some inner, bound function,
                    // so keep it.

                    self.clone()
                }
            }
            Some(NodeType::Count) => Structure::new_node_count(
                self.node_count(knowledge)
                    .unwrap()
                    .capture(knowledge, additional_depth, ctx),
            )
            .into(),
            Some(ty) => todo!("Cannot capture {ty:?}"),
            None => self.clone(),
        }
    }

    #[instrument(skip(knowledge))]
    fn eval(&self, knowledge: &Structure, ctx: &mut EvaluationContext) -> Object {
        // TODO: better panic msgs

        match self.node_type(knowledge) {
            Some(NodeType::Count) => Object::natural_number(
                self.node_count(knowledge)
                    .expect("NodeType::Count asserts that this exists")
                    .eval(knowledge, ctx)
                    .property_count(),
            ),
            Some(NodeType::Query) => {
                // TODO: adjust constraint for query

                let statement_form = self
                    .node_query(knowledge)
                    .expect("Node::Query expects this")
                    .statement_form(knowledge);

                let subject = Option::<Object>::from(statement_form.subject)
                    .expect("cannot query with no subject")
                    .eval(knowledge, ctx);

                let tag = Option::<Object>::from(statement_form.tag)
                    .expect("cannot query with no tag")
                    .eval(knowledge, ctx);

                let value =
                    Option::<Object>::from(statement_form.value).map(|c| c.eval(knowledge, ctx));

                let actual_qr = query_values(knowledge, &subject, tag, ctx);

                if let Some(value) = value {
                    // This is just equal to `NODE_EXISTS`.

                    for item in actual_qr.iter() {
                        if item == &value {
                            return Object::from_bool(true);
                        }
                    }

                    Object::from_bool(false)
                } else {
                    // Collect all values into a set.
                    actual_qr.collect_to_set()
                }
            }
            Some(NodeType::Literal) => self.node_literal(knowledge).unwrap(),
            Some(NodeType::Computed) => self.capture(knowledge, 0, ctx),
            Some(NodeType::Parameter) => {
                ctx.parameter_value(self.node_parameter_depth(knowledge).unwrap())
            }
            Some(NodeType::Equal) => {
                let qr = query_values(knowledge, self, Object::NODE_EQUAL, ctx);
                let mut expressions = qr.iter().map(|value| value.eval(knowledge, ctx));

                let first = expressions.next().unwrap();
                let equal = expressions.all(|object| object == first);

                Object::from_bool(equal)
            }
            Some(NodeType::Or) => {
                let qr = query_values(knowledge, self, Object::NODE_OR, ctx);
                let mut values = qr.iter().map(|value| value.eval(knowledge, ctx));

                Object::from_bool(values.any(|o| o.is_truthy()))
            }
            Some(NodeType::And) => {
                let qr = query_values(knowledge, self, Object::NODE_AND, ctx);
                let mut values = qr.iter().map(|value| value.eval(knowledge, ctx));

                Object::from_bool(values.all(|value| value.is_truthy()))
            }
            Some(NodeType::Exists) => {
                // TODO: discuss exists form

                let StatementForm {
                    subject,
                    tag,
                    value,
                } = self
                    .node_exists(knowledge)
                    .expect("NodeType::Exists asserts this")
                    .statement_form(knowledge);

                let subject = Option::<Object>::from(subject)
                    .expect("cannot query exists with no subject (not yet)")
                    .eval(knowledge, ctx);

                let tag = Option::<Object>::from(tag).map(|tag| tag.eval(knowledge, ctx));
                let value = Option::<Object>::from(value).map(|tag| tag.eval(knowledge, ctx));

                match (tag, value) {
                    // I think this should be fine.
                    (None, None) => Object::from_bool(true),
                    (Some(tag), None) => {
                        // Now we only need to check if one value exists.

                        let values_qr = query_values(knowledge, &subject, tag, ctx);

                        Object::from_bool(values_qr.iter().next().is_some())
                    }
                    // TODO: ?
                    (None, Some(_)) => Object::from_bool(true),
                    (Some(tag), Some(value)) => {
                        // TODO: perf

                        let values_qr = query_values(knowledge, &subject, tag, ctx);
                        Object::from_bool(values_qr.iter().find(|v| **v == value).is_some())
                    }
                }
            }
            Some(ty) => todo!("{ty:?} not impl"),
            None => self.clone(),
        }
    }

    #[instrument(skip(knowledge))]
    fn call(
        &self,
        knowledge: &Structure,
        parameters: &[Object],
        ctx: &mut EvaluationContext,
    ) -> Object {
        if let Some((parameter, next_parameters)) = parameters.split_first() {
            if let Some(NodeType::Computed) = self.node_type(knowledge) {
                let parameter = parameter.eval(knowledge, ctx);

                let body_qr =
                    query_values(knowledge, self, Object::COMPUTED, &mut Default::default());
                let body = body_qr.iter().next().unwrap();

                ctx.push(FunctionContext {
                    function: self.clone(),
                    parameter,
                });

                let result = body.call(knowledge, next_parameters, ctx);

                ctx.pop();

                result
            } else {
                // Ignore parameter and eval `self`.
                self.eval(knowledge, ctx)
            }
        } else {
            self.eval(knowledge, ctx)
        }
    }

    fn to_natural_number(&self, knowledge: &Structure) -> Option<usize> {
        if self == &Object::ZERO {
            Some(0)
        } else {
            let qr = query_values(
                knowledge,
                self,
                Object::SUCCESSOR_OF,
                &mut Default::default(),
            );

            // TODO: maybe validate that there is only one.
            if let Some(successor_of) = qr.iter().next() {
                successor_of.to_natural_number(knowledge).map(|n| n + 1)
            } else {
                None
            }
        }
    }
}
