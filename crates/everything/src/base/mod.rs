#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use everything_objects::{Abstract, Composite, Object};

use crate::{
    ext::{AbstractExt, CompositeExt, ObjectExt},
    nodes::{BinaryNode, CallNode, Node, QueryValuesNode},
    statements::{Statement, Statements},
};

fn common_unique_constraint_expression(tag: Object, parameter_depth: u64) -> Object {
    Object::new_node(Node::Equal(BinaryNode {
        left: Object::new_integer(1),
        right: Object::new_node(Node::Count(Object::new_node(Node::QueryValues(
            QueryValuesNode {
                subject: Object::new_node(Node::Parameter(parameter_depth)),
                tag,
            },
        )))),
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
        right: Object::new_node(Node::QueryValues(QueryValuesNode {
            subject: Object::new_node(Node::Parameter(0)),
            tag: Abstract::SUCCESSOR_OF.into(),
        })),
    }))))
});

/*
pub static IS_INTEGER: LazyLock<Object> = LazyLock::new(|| {
    Object::new_node(Node::Function(Object::new_node(Node::Or(BinaryNode {
        left: Object::new_node(Node::Equal(BinaryNode {
            left: Object::new_node(Node::Parameter(0)),
            right: Abstract::ZERO.into(),
        })),
        right: Object::new_node(Node::Add(BinaryNode {
            left: Object::new_node(Node::Not(Object::new_node(Node::IsAbstract(
                Object::new_node(Node::Parameter(0)),
            )))),
            right: Object::new_node(Node::And(BinaryNode {
                left: Object::new_node(Node::Xor(BinaryNode {
                    left: Object::new_node(Node::Query()),
                    right: (),
                })),
                right: (),
            })),
        })),
    }))))
});
 */

fn bit_slot_statement(slot: Abstract) -> Statement {
    Statement::new(
        slot,
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
                right: common_unique_constraint_expression(slot.into(), 1),
            })),
        )))),
    )
}

pub static BASE: LazyLock<Statements> = LazyLock::new(|| {
    Statements::from([
        Statement::new(
            Abstract::CONTAINS,
            Abstract::AXIOMATIC.into(),
            Composite::new_bool(true).into(),
        ),
        Statement::new(
            Abstract::AXIOMATIC,
            Abstract::AXIOMATIC.into(),
            AXIOMATIC_AXIOMATIC_CONSTRAINT.clone(),
        ),
        Statement::new(
            Abstract::FUNCTION,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::FUNCTION.into(), 0),
        ),
        Statement::new(
            Abstract::STATEMENT_SUBJECT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_SUBJECT.into(), 0),
        ),
        Statement::new(
            Abstract::STATEMENT_TAG,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_TAG.into(), 0),
        ),
        Statement::new(
            Abstract::STATEMENT_VALUE,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_VALUE.into(), 0),
        ),
        // ---- Primitives -------------------------
        Statement::new(
            Abstract::SUCCESSOR_OF,
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
        ),
        Statement::new(
            Abstract::PREDECESSOR_OF,
            Abstract::AXIOMATIC.into(),
            Object::new_node(Node::Function(Object::new_node(Node::Function(
                Object::new_node(Node::And(BinaryNode {
                    left: Object::new_node(Node::Or(BinaryNode {
                        left: Object::new_node(Node::Equal(BinaryNode {
                            left: Object::new_node(Node::Parameter(0)),
                            right: Abstract::ZERO.into(),
                        })),
                        right: Object::new_node(Node::QueryValues(QueryValuesNode {
                            subject: Object::new_node(Node::Parameter(0)),
                            tag: Abstract::PREDECESSOR_OF.into(),
                        })),
                    })),
                    right: common_unique_constraint_expression(Abstract::PREDECESSOR_OF.into(), 1),
                })),
            )))),
        ),
        Statement::new(
            Abstract::CODE_POINT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::CODE_POINT.into(), 0),
        ),
        Statement::new(
            Abstract::LIST_ITEM,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::LIST_ITEM.into(), 0),
        ),
        Statement::new(
            Abstract::LIST_TAIL,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::LIST_TAIL.into(), 0),
        ),
        bit_slot_statement(Abstract::BIT_SLOT_0),
        bit_slot_statement(Abstract::BIT_SLOT_1),
        bit_slot_statement(Abstract::BIT_SLOT_2),
        bit_slot_statement(Abstract::BIT_SLOT_3),
        bit_slot_statement(Abstract::BIT_SLOT_4),
        bit_slot_statement(Abstract::BIT_SLOT_5),
        bit_slot_statement(Abstract::BIT_SLOT_6),
        bit_slot_statement(Abstract::BIT_SLOT_7),
        /*
        Statement::new(
            Abstract::KNOWLEDGE,
            Abstract::FUNCTION.into(),
            // A function that calls itself. In theory this would loop forever
            // but the implementation is hard-coded.
            Object::new_node(Node::Call(CallNode {
                callee: Object::new_node(Node::FunctionSelf(0)),
                with: Object::new_node(Node::Parameter(0)),
            })),
        )
         */
        // --------------------- NODES ---------------------
        Statement::new(
            Abstract::NODE_LITERAL,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LITERAL.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_COUNT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_COUNT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_PARAMETER,
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
        ),
        Statement::new(
            Abstract::NODE_FUNCTION_SELF,
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
        ),
        Statement::new(
            Abstract::NODE_AND_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_AND_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_AND_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_AND_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_OR_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_OR_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_OR_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_OR_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_XOR_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_XOR_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_XOR_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_XOR_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_EQUAL_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EQUAL_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_EQUAL_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EQUAL_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_QUERY_SUBJECT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_QUERY_SUBJECT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_QUERY_TAG,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_QUERY_TAG.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_QUERY_VALUE,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_QUERY_VALUE.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_NOT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_NOT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_ADD_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_ADD_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_ADD_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_ADD_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_UNION_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNION_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_UNION_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNION_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_MAP_MAPPER,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MAP_MAPPER.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_MAP_SET,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MAP_SET.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_FILTER_FILTER,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_FILTER_FILTER.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_FILTER_SET,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_FILTER_SET.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_LESS_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LESS_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_LESS_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LESS_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_IF_CONDITION,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_IF_CONDITION.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_IF_THEN,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_IF_THEN.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_IF_ELSE,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_IF_ELSE.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_UNWRAP_OR_SET,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNWRAP_OR_SET.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_UNWRAP_OR_DEFAULT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_UNWRAP_OR_DEFAULT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_MULTIPLY_LEFT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MULTIPLY_LEFT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_MULTIPLY_RIGHT,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_MULTIPLY_RIGHT.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_CALL_CALLEE,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_CALL_CALLEE.into(), 0),
        ),
        Statement::new(
            Abstract::NODE_CALL_WITH,
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_CALL_WITH.into(), 0),
        ),
    ])
});
