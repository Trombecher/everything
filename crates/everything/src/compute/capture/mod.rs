#[cfg(test)]
mod tests;

use std::{collections::BTreeSet, sync::Arc};

use crate::{
    compute::{EvaluationContext, MaybeUndefined, Node, nodes::source_set},
    knowledge::Knowledge,
    model::{Atom, Object, Set},
};

/// Rebuilds the source object of a captured node.
///
/// The members of the original source object are preserved, except for the
/// properties that belong to the node kind. Those are taken from the captured
/// node, so that substituted children are reflected in the materialised
/// object. Annotations and unknown properties therefore survive capture.
fn source_with_captured_properties(
    source: &Object,
    original: &Node,
    captured: &Node,
    knowledge: &Knowledge,
) -> Object {
    let captured_object = captured.into_object();

    let Object::Set(captured_set) = captured_object else {
        return captured_object;
    };

    let source_set = match source {
        Object::Set(set) => set.clone(),
        Object::Atom(_) => source_set(source, knowledge),
        Object::Construction(_) => return Object::Set(captured_set),
    };

    let property_constructors = match original.into_object() {
        Object::Set(set) => set
            .into_iter()
            .filter_map(|value| match value {
                Object::Construction(construction) => Some(*construction.constructor),
                _ => None,
            })
            .collect::<BTreeSet<_>>(),
        _ => BTreeSet::new(),
    };

    let mut values = source_set
        .into_iter()
        .filter(|value| {
            !matches!(
                value,
                Object::Construction(construction)
                    if property_constructors.contains(construction.constructor.as_ref())
            )
        })
        .collect::<Vec<_>>();

    values.extend(captured_set);

    Set::from(values.as_mut_slice()).into()
}

/// Captures the evaluation context in the node tree.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn capture(
    node: &Node,
    additional_depth: usize,
    evaluation_context: &EvaluationContext,
    knowledge: &Knowledge,
) -> Node {
    if evaluation_context.is_empty() {
        // Nothing to capture.
        return node.clone();
    }

    match node {
        Node::WithSource {
            node: inner,
            source,
        } => {
            let captured = capture(inner, additional_depth, evaluation_context, knowledge);
            let source = source_with_captured_properties(source, inner, &captured, knowledge);

            Node::WithSource {
                node: Arc::new(captured),
                source: Arc::new(source),
            }
        }
        Node::Parameter { depth } => {
            if let Some(offset_depth) = (*depth as usize).checked_sub(additional_depth) {
                // The min additional depth is 1.
                // So when the parameter depth is 1 it will refer to
                // captured parameters at an additional depth of 1.

                // We also need to escape the parameter because it may contain
                // a node (which is already evaluated because parameters
                // are always evaluated before the function).

                Node::Literal(match evaluation_context.parameter_value(offset_depth) {
                    MaybeUndefined::Defined(value) => value.into_object(knowledge),
                    MaybeUndefined::Undefined => Atom::UNDEFINED.into(),
                })
            } else {
                // This parameter refers to some inner, bound function,
                // so keep it.

                node.clone()
            }
        }
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
        Node::Function { body, object } => Node::Function {
            body: capture(body, additional_depth + 1, evaluation_context, knowledge).into(),
            object: object.clone(),
        },
        Node::Literal(object) => Node::Literal(object.clone()),
        Node::And { first, second } => Node::And {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Count(inner) => {
            Node::Count(capture(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::ValuesOf(inner) => {
            Node::ValuesOf(capture(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::HasValue(inner) => {
            Node::HasValue(capture(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::QueryAtomsWithProperty { property } => Node::QueryAtomsWithProperty {
            property: capture(property, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::QueryAtomsWithConstructor { constructor } => Node::QueryAtomsWithConstructor {
            constructor: capture(constructor, additional_depth, evaluation_context, knowledge)
                .into(),
        },
        Node::QueryConstructionValues {
            object,
            property_constructor,
        } => Node::QueryConstructionValues {
            object: capture(object, additional_depth, evaluation_context, knowledge).into(),
            property_constructor: capture(
                property_constructor,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::QueryConstructions { constructor } => Node::QueryConstructions {
            constructor: capture(constructor, additional_depth, evaluation_context, knowledge)
                .into(),
        },
        Node::Contains { element, set } => Node::Contains {
            element: capture(element, additional_depth, evaluation_context, knowledge).into(),
            set: capture(set, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Equal { first, second } => Node::Equal {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Or { first, second } => Node::Or {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Xor { first, second } => Node::Xor {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Not(inner) => {
            Node::Not(capture(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::Add { first, second } => Node::Add {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Union { first, second } => Node::Union {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Intersection { first, second } => Node::Intersection {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Difference { first, second } => Node::Difference {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Map {
            set,
            mapper_function,
        } => Node::Map {
            set: capture(set, additional_depth, evaluation_context, knowledge).into(),
            mapper_function: capture(
                mapper_function,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::Filter {
            set,
            filter_function,
        } => Node::Filter {
            set: capture(set, additional_depth, evaluation_context, knowledge).into(),
            filter_function: capture(
                filter_function,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::Less { first, second } => Node::Less {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::If {
            condition,
            then,
            otherwise,
        } => Node::If {
            condition: capture(condition, additional_depth, evaluation_context, knowledge).into(),
            then: capture(then, additional_depth, evaluation_context, knowledge).into(),
            otherwise: capture(otherwise, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Unwrap(set) => {
            Node::Unwrap(capture(set, additional_depth, evaluation_context, knowledge).into())
        }
        Node::Multiply { first, second } => Node::Multiply {
            first: capture(first, additional_depth, evaluation_context, knowledge).into(),
            second: capture(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Call { callee, with } => Node::Call {
            callee: capture(callee, additional_depth, evaluation_context, knowledge).into(),
            with: capture(with, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::IsAtom(inner) => {
            Node::IsAtom(capture(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::IsConstruction(inner) => Node::IsConstruction(
            capture(inner, additional_depth, evaluation_context, knowledge).into(),
        ),
        Node::IsSet(inner) => {
            Node::IsSet(capture(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::Every { set, predicate } => Node::Every {
            set: capture(set, additional_depth, evaluation_context, knowledge).into(),
            predicate: capture(predicate, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Any { set, predicate } => Node::Any {
            set: capture(set, additional_depth, evaluation_context, knowledge).into(),
            predicate: capture(predicate, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Fold {
            set,
            folder,
            initial,
        } => Node::Fold {
            set: capture(set, additional_depth, evaluation_context, knowledge).into(),
            folder: capture(folder, additional_depth, evaluation_context, knowledge).into(),
            initial: capture(initial, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::NewConstruction { constructor, value } => Node::NewConstruction {
            constructor: capture(constructor, additional_depth, evaluation_context, knowledge)
                .into(),
            value: capture(value, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::ConstructionConstructor { construction } => Node::ConstructionConstructor {
            construction: capture(
                construction,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::ConstructionValue { construction } => Node::ConstructionValue {
            construction: capture(
                construction,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
    }
}
