use std::sync::Arc;

use crate::{
    compute::{EvaluationContext, MaybeUndefined, Node},
    model::Atom,
};

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn capture(
    node: &Node,
    additional_depth: usize,
    evaluation_context: &EvaluationContext,
) -> Node {
    if evaluation_context.is_empty() {
        // Nothing to capture.
        return node.clone();
    }

    match node {
        Node::Function { body } => Node::Function {
            body: Arc::new(capture(body, additional_depth + 1, evaluation_context)),
        },
        Node::Parameter { depth } => {
            if let Some(offset_depth) = (*depth as usize).checked_sub(additional_depth) {
                // The min additional depth is 1.
                // So when the parameter depth is 1 it will refer to
                // captured parameters at an additional depth of 1.

                // We also need to escape the parameter because it may contain
                // a node (which is already evaluated because parameters
                // are always evaluated before the function).

                Node::Literal(match evaluation_context.parameter_value(offset_depth) {
                    MaybeUndefined::Defined(value) => value.into_object(),
                    MaybeUndefined::Undefined => Atom::UNDEFINED.into(),
                })
            } else {
                // This parameter refers to some inner, bound function,
                // so keep it.

                node.clone()
            }
        }
        Node::Literal(object) => Node::Literal(object.clone()),
        Node::And { first, second } => Node::And {
            first: Arc::new(capture(first, additional_depth, evaluation_context)),
            second: Arc::new(capture(second, additional_depth, evaluation_context)),
        },
        Node::FunctionSelf { depth } => {
            if let Some(offset_depth) = (*depth as usize).checked_sub(additional_depth) {
                // Same as for parameters: the function referenced by this
                // node is outside the captured function and will not be
                // available anymore once the capture is called.

                // Functions in the evaluation context are already captured,
                // so we can just close over them. Escaping the function is
                // necessary because it must not be evaluated again.

                Node::Literal(match evaluation_context.function_self(offset_depth) {
                    MaybeUndefined::Defined(function) => function.into_object(),
                    MaybeUndefined::Undefined => Atom::UNDEFINED.into(),
                })
            } else {
                // This function refers to some inner, bound function,
                // so keep it.

                node.clone()
            }
        }
        Node::Count(inner) => {
            Node::Count(capture(inner, additional_depth, evaluation_context).into())
        }
        Node::QueryAtomsWithProperty { property } => Node::QueryAtomsWithProperty {
            property: capture(property, additional_depth, evaluation_context).into(),
        },
        Node::QueryConstructionValues {
            set,
            property_constructor,
        } => Node::QueryConstructionValues {
            set: capture(set, additional_depth, evaluation_context).into(),
            property_constructor: capture(
                property_constructor,
                additional_depth,
                evaluation_context,
            )
            .into(),
        },
        Node::Contains { element, set } => Node::Contains {
            element: capture(element, additional_depth, evaluation_context).into(),
            set: capture(set, additional_depth, evaluation_context).into(),
        },
        Node::Equal { first, second } => Node::Equal {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::Or { first, second } => Node::Or {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::Xor { first, second } => Node::Xor {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::Not(inner) => Node::Not(capture(inner, additional_depth, evaluation_context).into()),
        Node::Add { first, second } => Node::Add {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::Union { first, second } => Node::Union {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::Map {
            set,
            mapper_function,
        } => Node::Map {
            set: capture(set, additional_depth, evaluation_context).into(),
            mapper_function: capture(mapper_function, additional_depth, evaluation_context).into(),
        },
        Node::Filter {
            set,
            filter_function,
        } => Node::Filter {
            set: capture(set, additional_depth, evaluation_context).into(),
            filter_function: capture(filter_function, additional_depth, evaluation_context).into(),
        },
        Node::Less { first, second } => Node::Less {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::If {
            condition,
            then,
            otherwise,
        } => Node::If {
            condition: capture(condition, additional_depth, evaluation_context).into(),
            then: capture(then, additional_depth, evaluation_context).into(),
            otherwise: capture(otherwise, additional_depth, evaluation_context).into(),
        },
        Node::UnwrapOr { set, default } => Node::UnwrapOr {
            set: capture(set, additional_depth, evaluation_context).into(),
            default: capture(default, additional_depth, evaluation_context).into(),
        },
        Node::Multiply { first, second } => Node::Multiply {
            first: capture(first, additional_depth, evaluation_context).into(),
            second: capture(second, additional_depth, evaluation_context).into(),
        },
        Node::Call { callee, with } => Node::Call {
            callee: capture(callee, additional_depth, evaluation_context).into(),
            with: capture(with, additional_depth, evaluation_context).into(),
        },
        Node::IsAtom(inner) => {
            Node::IsAtom(capture(inner, additional_depth, evaluation_context).into())
        }
        Node::IsConstruction(inner) => {
            Node::IsConstruction(capture(inner, additional_depth, evaluation_context).into())
        }
        Node::IsSet(inner) => {
            Node::IsSet(capture(inner, additional_depth, evaluation_context).into())
        }
        Node::Every { set, predicate } => Node::Every {
            set: capture(set, additional_depth, evaluation_context).into(),
            predicate: capture(predicate, additional_depth, evaluation_context).into(),
        },
        Node::Any { set, predicate } => Node::Any {
            set: capture(set, additional_depth, evaluation_context).into(),
            predicate: capture(predicate, additional_depth, evaluation_context).into(),
        },
        Node::Fold {
            set,
            folder,
            initial,
        } => Node::Fold {
            set: capture(set, additional_depth, evaluation_context).into(),
            folder: capture(folder, additional_depth, evaluation_context).into(),
            initial: capture(initial, additional_depth, evaluation_context).into(),
        },
        Node::NewConstruction { constructor, value } => Node::NewConstruction {
            constructor: capture(constructor, additional_depth, evaluation_context).into(),
            value: capture(value, additional_depth, evaluation_context).into(),
        },
        Node::ConstructionConstructor { construction } => Node::ConstructionConstructor {
            construction: capture(construction, additional_depth, evaluation_context).into(),
        },
        Node::ConstructionValue { construction } => Node::ConstructionValue {
            construction: capture(construction, additional_depth, evaluation_context).into(),
        },
    }
}
