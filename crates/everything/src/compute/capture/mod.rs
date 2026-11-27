#[cfg(test)]
mod tests;

use std::{collections::BTreeSet, sync::Arc};

use crate::{
    compute::{Argument, EvaluationContext, MaybeUndefined, Node, nodes::source_composite},
    knowledge::Knowledge,
    model::{Atom, Composite, Object},
};

#[derive(Clone, Copy)]
enum CaptureMode {
    /// Capture a function body. References bound inside the function are
    /// preserved, and an empty outer context leaves the body unchanged.
    Function,
    /// Close a call argument. References that are not bound inside the
    /// expression are free and become `undefined`; they must not be rebound by
    /// the callee after substitution.
    Argument,
}

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

    let Object::Composite(captured_composite) = captured_object else {
        return captured_object;
    };

    let source_composite = match source {
        Object::Composite(set) => set.clone(),
        Object::Atom(_) => source_composite(source, knowledge),
        Object::Construction(_) => return Object::Composite(captured_composite),
    };

    let property_constructors = match original.into_object() {
        Object::Composite(set) => set
            .into_iter()
            .filter_map(|value| match value {
                Object::Construction(construction) => Some(*construction.constructor),
                _ => None,
            })
            .collect::<BTreeSet<_>>(),
        _ => BTreeSet::new(),
    };

    let mut values = source_composite
        .into_iter()
        .filter(|value| {
            !matches!(
                value,
                Object::Construction(construction)
                    if property_constructors.contains(construction.constructor.as_ref())
            )
        })
        .collect::<Vec<_>>();

    values.extend(captured_composite);

    Composite::from(values.as_mut_slice()).into()
}

/// Captures the evaluation context in the node tree.
#[must_use]
pub fn capture(
    node: &Node,
    additional_depth: usize,
    evaluation_context: &EvaluationContext,
    knowledge: &Knowledge,
) -> Node {
    capture_with_mode(
        node,
        additional_depth,
        evaluation_context,
        knowledge,
        CaptureMode::Function,
    )
}

/// Closes a call argument in the caller's context.
///
/// In contrast to [`capture`], this also turns free parameter and function
/// self-references into `undefined`. A free reference in an argument must not
/// be captured by the callee after the argument has been substituted into the
/// callee's body.
#[must_use]
pub fn capture_argument(
    node: &Node,
    evaluation_context: &EvaluationContext,
    knowledge: &Knowledge,
) -> Node {
    capture_with_mode(
        node,
        0,
        evaluation_context,
        knowledge,
        CaptureMode::Argument,
    )
}

#[must_use]
#[allow(clippy::too_many_lines)]
fn capture_with_mode(
    node: &Node,
    additional_depth: usize,
    evaluation_context: &EvaluationContext,
    knowledge: &Knowledge,
    mode: CaptureMode,
) -> Node {
    if matches!(mode, CaptureMode::Function) && evaluation_context.is_empty() {
        // Nothing to capture.
        return node.clone();
    }

    macro_rules! recurse {
        ($node:expr, $additional_depth:expr, $context:expr, $knowledge:expr $(,)?) => {
            capture_with_mode($node, $additional_depth, $context, $knowledge, mode)
        };
    }

    match node {
        Node::WithSource {
            node: inner,
            source,
        } => {
            let captured = recurse!(inner, additional_depth, evaluation_context, knowledge);
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

                match evaluation_context.parameter(offset_depth) {
                    MaybeUndefined::Defined(Argument::Value(value)) => {
                        // An evaluated value must not be reinterpreted as code.
                        Node::Literal(value.into_object(knowledge))
                    }
                    MaybeUndefined::Defined(Argument::Unevaluated(thunk)) => {
                        if let Some(value) = thunk.value() {
                            // Reuse the cached result instead of copying the
                            // expression. The value must not be reinterpreted
                            // as code.
                            Node::Literal(value.clone().into_object(knowledge))
                        } else {
                            // The argument is already closed in the caller's
                            // context. Share the thunk instead of copying its
                            // expression, so that every reference in the
                            // captured code shares one evaluation.
                            Node::Thunk(Arc::clone(&thunk))
                        }
                    }
                    MaybeUndefined::Undefined => Node::Literal(Atom::UNDEFINED.into()),
                }
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
            body: recurse!(body, additional_depth + 1, evaluation_context, knowledge).into(),
            object: object.clone(),
        },
        Node::Literal(object) => Node::Literal(object.clone()),
        Node::Thunk(thunk) => {
            if let Some(value) = thunk.value() {
                Node::Literal(value.clone().into_object(knowledge))
            } else {
                // The thunk is already closed, so capturing does not change it.
                Node::Thunk(Arc::clone(thunk))
            }
        }
        Node::And { first, second } => Node::And {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetCount(inner) => {
            Node::SetCount(recurse!(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::PropertiesOf(inner) => Node::PropertiesOf(
            recurse!(inner, additional_depth, evaluation_context, knowledge).into(),
        ),
        Node::HasASetItem(inner) => Node::HasASetItem(
            recurse!(inner, additional_depth, evaluation_context, knowledge).into(),
        ),
        Node::HasProperty { subject, property } => Node::HasProperty {
            subject: recurse!(subject, additional_depth, evaluation_context, knowledge).into(),
            property: recurse!(property, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::QueryAtomsWithProperty { property } => Node::QueryAtomsWithProperty {
            property: recurse!(property, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::QueryAtomsWithConstructor { constructor } => Node::QueryAtomsWithConstructor {
            constructor: recurse!(constructor, additional_depth, evaluation_context, knowledge)
                .into(),
        },
        Node::QueryConstructionValues {
            object,
            property_constructor,
        } => Node::QueryConstructionValues {
            object: recurse!(object, additional_depth, evaluation_context, knowledge).into(),
            property_constructor: recurse!(
                property_constructor,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::QueryConstructions { constructor } => Node::QueryConstructions {
            constructor: recurse!(constructor, additional_depth, evaluation_context, knowledge)
                .into(),
        },
        Node::SetContains { element, set } => Node::SetContains {
            element: recurse!(element, additional_depth, evaluation_context, knowledge).into(),
            set: recurse!(set, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Equal { first, second } => Node::Equal {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Or { first, second } => Node::Or {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Xor { first, second } => Node::Xor {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Not(inner) => {
            Node::Not(recurse!(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::Add { first, second } => Node::Add {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetUnion { first, second } => Node::SetUnion {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetIntersection { first, second } => Node::SetIntersection {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetDifference { first, second } => Node::SetDifference {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetMap {
            set,
            mapper_function,
        } => Node::SetMap {
            set: recurse!(set, additional_depth, evaluation_context, knowledge).into(),
            mapper_function: recurse!(
                mapper_function,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::SetFilter {
            set,
            filter_function,
        } => Node::SetFilter {
            set: recurse!(set, additional_depth, evaluation_context, knowledge).into(),
            filter_function: recurse!(
                filter_function,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::Less { first, second } => Node::Less {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::If {
            condition,
            then,
            otherwise,
        } => Node::If {
            condition: recurse!(condition, additional_depth, evaluation_context, knowledge).into(),
            then: recurse!(then, additional_depth, evaluation_context, knowledge).into(),
            otherwise: recurse!(otherwise, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetUnwrap(set) => {
            Node::SetUnwrap(recurse!(set, additional_depth, evaluation_context, knowledge).into())
        }
        Node::Multiply { first, second } => Node::Multiply {
            first: recurse!(first, additional_depth, evaluation_context, knowledge).into(),
            second: recurse!(second, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::Call { callee, with } => Node::Call {
            callee: recurse!(callee, additional_depth, evaluation_context, knowledge).into(),
            with: recurse!(with, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::IsAtom(inner) => {
            Node::IsAtom(recurse!(inner, additional_depth, evaluation_context, knowledge).into())
        }
        Node::IsConstruction(inner) => Node::IsConstruction(
            recurse!(inner, additional_depth, evaluation_context, knowledge).into(),
        ),
        Node::IsComposite(inner) => Node::IsComposite(
            recurse!(inner, additional_depth, evaluation_context, knowledge).into(),
        ),
        Node::SetEvery { set, predicate } => Node::SetEvery {
            set: recurse!(set, additional_depth, evaluation_context, knowledge).into(),
            predicate: recurse!(predicate, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetAny { set, predicate } => Node::SetAny {
            set: recurse!(set, additional_depth, evaluation_context, knowledge).into(),
            predicate: recurse!(predicate, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::SetFold {
            set,
            folder,
            initial,
        } => Node::SetFold {
            set: recurse!(set, additional_depth, evaluation_context, knowledge).into(),
            folder: recurse!(folder, additional_depth, evaluation_context, knowledge).into(),
            initial: recurse!(initial, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::NewConstruction { constructor, value } => Node::NewConstruction {
            constructor: recurse!(constructor, additional_depth, evaluation_context, knowledge)
                .into(),
            value: recurse!(value, additional_depth, evaluation_context, knowledge).into(),
        },
        Node::NewSet { items } => Node::NewSet {
            items: items
                .iter()
                .map(|value| {
                    recurse!(value, additional_depth, evaluation_context, knowledge).into()
                })
                .collect(),
        },
        Node::NewComposite { properties } => Node::NewComposite {
            properties: properties
                .iter()
                .map(|value| {
                    recurse!(value, additional_depth, evaluation_context, knowledge).into()
                })
                .collect(),
        },
        Node::ConstructionConstructor { construction } => Node::ConstructionConstructor {
            construction: recurse!(
                construction,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
        Node::ConstructionValue { construction } => Node::ConstructionValue {
            construction: recurse!(
                construction,
                additional_depth,
                evaluation_context,
                knowledge,
            )
            .into(),
        },
    }
}
