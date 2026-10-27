use std::sync::Arc;

use crate::{
    compute::{EvaluationContext, MaybeUndefined, Node},
    model::Atom,
};

#[must_use]
#[allow(unused_variables)]
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
        Node::And { first, second } => todo!(),
        Node::FunctionSelf { depth } => todo!(),
        Node::Count(node) => todo!(),
        Node::QueryAtomsWithProperty { property } => todo!(),
        Node::QueryConstructionValues {
            set,
            property_constructor,
        } => todo!(),
        Node::Contains { element, set } => todo!(),
        Node::Equal { first, second } => todo!(),
        Node::Or { first, second } => todo!(),
        Node::Xor { first, second } => todo!(),
        Node::Not(node) => todo!(),
        Node::Add { first, second } => todo!(),
        Node::Union { first, second } => todo!(),
        Node::Map {
            set,
            mapper_function,
        } => todo!(),
        Node::Filter {
            set,
            filter_function,
        } => todo!(),
        Node::Less { first, second } => todo!(),
        Node::If {
            condition,
            then,
            otherwise,
        } => todo!(),
        Node::UnwrapOr { set, default } => todo!(),
        Node::Multiply { first, second } => todo!(),
        Node::Call { callee, with } => todo!(),
        Node::IsAtom(node) => todo!(),
        Node::IsConstruction(node) => todo!(),
        Node::IsSet(node) => todo!(),
        Node::Every { set, predicate } => todo!(),
        Node::Any { set, predicate } => todo!(),
        Node::Fold {
            set,
            folder,
            initial,
        } => todo!(),
        Node::NewConstruction { constructor, value } => todo!(),
        Node::ConstructionConstructor { construction } => todo!(),
        Node::ConstructionValue { construction } => todo!(),
    }
}
