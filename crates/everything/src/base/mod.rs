#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use everything_objects::{Abstract, Composite, Object};

use crate::{
    ext::{AbstractExt, CompositeExt, ObjectExt},
    nodes::{BinaryNode, CallNode, Node},
};

fn common_unique_constraint_expression(tag: Object, parameter_depth: u64) -> Object {
    Object::new_node(Node::Equal(BinaryNode {
        left: Object::new_integer(1),
        right: Object::new_node(Node::Count(Object::new_node_query_values(
            Object::new_node(Node::Parameter(parameter_depth)),
            tag,
        ))),
    }))
}

/// Creates a function object that validates that any
/// subject associated axiomatically with the given tag
/// has at most one association with this tag.
///
/// More specifically, it creates this function for the given tag:
///
/// ```plain
/// ... |-> count query {(@4, $parameter_at_depth), (@5, tag)} == 1
/// ```
fn unique_constraint_for(tag: Object, parameter_depth: u64) -> Object {
    Object::new_node(Node::Function(common_unique_constraint_expression(
        tag,
        parameter_depth,
    )))
}

pub static AXIOMATIC_AXIOMATIC_CONSTRAINT: LazyLock<Object> =
    LazyLock::new(|| unique_constraint_for(Abstract::AXIOMATIC.into(), 0));

/// A function that computes whether the object (passed in as the
/// parameter) is a natural number.
pub static IS_NATURAL_NUMBER: LazyLock<Object> = LazyLock::new(|| {
    Object::new_node(Node::Function(Object::new_node(Node::Or(BinaryNode {
        left: Object::new_node(Node::Equal(BinaryNode {
            left: Object::new_node(Node::Parameter(0)),
            right: Abstract::ZERO.into(),
        })),
        right: Object::new_node_query_values(
            Object::new_node(Node::Parameter(0)),
            Abstract::SUCCESSOR_OF.into(),
        ),
    }))))
});

fn bit_slot_statement(slot: Object) -> Object {
    Composite::new_statement(
        slot.clone(),
        Abstract::AXIOMATIC.into(),
        Object::new_node(Node::Function(Object::new_node(Node::Function(
            Object::new_node(Node::And(BinaryNode {
                left: Object::new_node(Node::Or(BinaryNode {
                    left: Object::new_node(Node::Equal(BinaryNode {
                        left: Object::new_node(Node::Parameter(0)),
                        right: Abstract::BIT_0.into(),
                    })),
                    right: Object::new_node(Node::Equal(BinaryNode {
                        left: Object::new_node(Node::Parameter(0)),
                        right: Abstract::BIT_1.into(),
                    })),
                })),
                right: common_unique_constraint_expression(slot, 1),
            })),
        )))),
    )
    .into()
}

pub static BASE: LazyLock<Composite> = LazyLock::new(|| {
    Composite::new_set([
        Composite::new_statement(
            Abstract::CONTAINS.into(),
            Abstract::AXIOMATIC.into(),
            Composite::new_bool(true).into(),
        )
        .into(),
        Composite::new_statement(
            Abstract::AXIOMATIC.into(),
            Abstract::AXIOMATIC.into(),
            AXIOMATIC_AXIOMATIC_CONSTRAINT.clone(),
        )
        .into(),
        Composite::new_statement(
            Abstract::FUNCTION.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::FUNCTION.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::STATEMENT_SUBJECT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_SUBJECT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::STATEMENT_TAG.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_TAG.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::STATEMENT_VALUE.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_VALUE.into(), 0),
        )
        .into(),
        // ---- Primitives -------------------------
        Composite::new_statement(
            Abstract::SUCCESSOR_OF.into(),
            Abstract::AXIOMATIC.into(),
            Object::new_node(Node::Function(Object::new_node(Node::Function(
                Object::new_node(Node::And(BinaryNode {
                    left: Object::new_node(Node::Call(CallNode {
                        callee: IS_NATURAL_NUMBER.clone(),
                        with: Object::new_node(Node::Parameter(0)),
                    })),
                    right: common_unique_constraint_expression(Abstract::SUCCESSOR_OF.into(), 1),
                })),
            )))),
        )
        .into(),
        Composite::new_statement(
            Abstract::PREDECESSOR_OF.into(),
            Abstract::AXIOMATIC.into(),
            Object::new_node(Node::Function(Object::new_node(Node::Function(
                Object::new_node(Node::And(BinaryNode {
                    left: Object::new_node(Node::Or(BinaryNode {
                        left: Object::new_node(Node::Equal(BinaryNode {
                            left: Object::new_node(Node::Parameter(0)),
                            right: Abstract::ZERO.into(),
                        })),
                        right: Object::new_node_query_values(
                            Object::new_node(Node::Parameter(0)),
                            Abstract::PREDECESSOR_OF.into(),
                        ),
                    })),
                    right: common_unique_constraint_expression(Abstract::PREDECESSOR_OF.into(), 1),
                })),
            )))),
        )
        .into(),
        Composite::new_statement(
            Abstract::CODE_POINT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::CODE_POINT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::LIST_ITEM.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::LIST_ITEM.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::LIST_TAIL.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::LIST_TAIL.into(), 0),
        )
        .into(),
        bit_slot_statement(Abstract::BIT_SLOT_0.into()),
        bit_slot_statement(Abstract::BIT_SLOT_1.into()),
        bit_slot_statement(Abstract::BIT_SLOT_2.into()),
        bit_slot_statement(Abstract::BIT_SLOT_3.into()),
        bit_slot_statement(Abstract::BIT_SLOT_4.into()),
        bit_slot_statement(Abstract::BIT_SLOT_5.into()),
        bit_slot_statement(Abstract::BIT_SLOT_6.into()),
        bit_slot_statement(Abstract::BIT_SLOT_7.into()),
        Composite::new_statement(
            Abstract::KNOWLEDGE.into(),
            Abstract::FUNCTION.into(),
            // A function that calls itself. In theory this would loop forever
            // but the implementation is hard-coded.
            Object::new_node(Node::Call(CallNode {
                callee: Object::new_node(Node::FunctionSelf(0)),
                with: Object::new_node(Node::Parameter(0)),
            })),
        )
        .into(),
        // --------------------- NODES ---------------------
        Composite::new_statement(
            Abstract::NODE_LITERAL.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LITERAL.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_COUNT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_COUNT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_PARAMETER.into(),
            Abstract::AXIOMATIC.into(),
            Object::new_node(Node::Function(Object::new_node(Node::Function(
                Object::new_node(Node::And(BinaryNode {
                    left: common_unique_constraint_expression(Abstract::NODE_PARAMETER.into(), 1),
                    // maybe hard code "parameter == zero or has succ"
                    right: Object::new_node(Node::Call(CallNode {
                        callee: IS_NATURAL_NUMBER.clone(),
                        with: Object::new_node(Node::Parameter(0)),
                    })),
                })),
            )))),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_FUNCTION_SELF.into(),
            Abstract::AXIOMATIC.into(),
            Object::new_node(Node::Function(Object::new_node(Node::Function(
                Object::new_node(Node::And(BinaryNode {
                    left: common_unique_constraint_expression(
                        Abstract::NODE_FUNCTION_SELF.into(),
                        1,
                    ),
                    // maybe hard code "parameter == zero or has succ"
                    right: Object::new_node(Node::Call(CallNode {
                        callee: IS_NATURAL_NUMBER.clone(),
                        with: Object::new_node(Node::Parameter(0)),
                    })),
                })),
            )))),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_AND_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_AND_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_AND_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_AND_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_OR_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_OR_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_OR_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_OR_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_XOR_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_XOR_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_XOR_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_XOR_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_EQUAL_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EQUAL_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_EQUAL_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EQUAL_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_QUERY.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_QUERY.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_NOT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_NOT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_ADD_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_ADD_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_ADD_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_ADD_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_UNION_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNION_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_UNION_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNION_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_MAP_MAPPER.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MAP_MAPPER.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_MAP_SET.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MAP_SET.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_FILTER_FILTER.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_FILTER_FILTER.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_FILTER_SET.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_FILTER_SET.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_LESS_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LESS_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_LESS_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LESS_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_IF_CONDITION.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_IF_CONDITION.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_IF_THEN.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_IF_THEN.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_IF_ELSE.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_IF_ELSE.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_UNWRAP_OR_SET.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNWRAP_OR_SET.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_UNWRAP_OR_DEFAULT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNWRAP_OR_DEFAULT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_MULTIPLY_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MULTIPLY_LEFT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_MULTIPLY_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MULTIPLY_RIGHT.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_CALL_CALLEE.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_CALL_CALLEE.into(), 0),
        )
        .into(),
        Composite::new_statement(
            Abstract::NODE_CALL_WITH.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_CALL_WITH.into(), 0),
        )
        .into(),
    ])
});
