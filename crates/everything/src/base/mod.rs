#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use everything_structures::{Object, Property, Structure};

use crate::ext::{ObjectExt, StructureExt};

fn stmt_to_prop(subject: Object, tag: Object, value: Object) -> Property {
    Property {
        tag: Object::CONTAINS,
        value: Structure::new(&mut [
            Property {
                tag: Object::STATEMENT_SUBJECT,
                value: subject,
            },
            Property {
                tag: Object::STATEMENT_TAG,
                value: tag,
            },
            Property {
                tag: Object::STATEMENT_VALUE,
                value,
            },
        ])
        .into(),
    }
}

fn common_unique_constraint_expression(tag: Object, parameter_depth: usize) -> Object {
    Structure::new_node_equal([
        Object::natural_number(1),
        Structure::new_node_count(
            Structure::new_node_query(
                Structure::new(&mut [
                    Property {
                        tag: Object::STATEMENT_SUBJECT,
                        value: Structure::new_node_parameter(parameter_depth).into(),
                    },
                    Property {
                        tag: Object::STATEMENT_TAG,
                        value: tag,
                    },
                ])
                .into(),
            )
            .into(),
        )
        .into(),
    ])
    .into()
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
fn unique_constraint_for(tag: Object, parameter_depth: usize) -> Object {
    Structure::new_computed(common_unique_constraint_expression(tag, parameter_depth)).into()
}

fn more_than_1_constraint_for(tag: Object, parameter_depth: usize) -> Object {
    Structure::new_computed(
        Structure::new_node_not(common_unique_constraint_expression(tag, parameter_depth)).into(),
    )
    .into()
}

pub static AXIOMATIC_AXIOMATIC_CONSTRAINT: LazyLock<Object> =
    LazyLock::new(|| unique_constraint_for(Object::AXIOMATIC, 0));

pub static IS_NATURAL_NUMBER: LazyLock<Object> = LazyLock::new(|| {
    Structure::new_computed(
        Structure::new_node_or([
            Structure::new_node_equal([Structure::new_node_parameter(0).into(), Object::ZERO])
                .into(),
            Structure::new_node_exists(
                Structure::new(&mut [
                    Property {
                        tag: Object::STATEMENT_SUBJECT,
                        value: Structure::new_node_parameter(0).into(),
                    },
                    Property {
                        tag: Object::STATEMENT_TAG,
                        value: Object::SUCCESSOR_OF,
                    },
                ])
                .into(),
            )
            .into(),
        ])
        .into(),
    )
    .into()
});

pub static BASE: LazyLock<Structure> = LazyLock::new(|| {
    Structure::new(&mut [
        stmt_to_prop(
            Object::CONTAINS,
            Object::AXIOMATIC,
            Structure::new(&mut [Property {
                tag: Object::CONTAINS,
                value: Structure::EMPTY.into(),
            }])
            .into(),
        ),
        stmt_to_prop(
            Object::SUCCESSOR_OF,
            Object::AXIOMATIC,
            Structure::new_computed(
                Structure::new_computed(
                    Structure::new_node_and([
                        Structure::new_node_exists(
                            Structure::new(&mut [
                                Property {
                                    tag: Object::STATEMENT_SUBJECT,
                                    value: Structure::new_node_parameter(0).into(),
                                },
                                Property {
                                    tag: Object::STATEMENT_TAG,
                                    value: IS_NATURAL_NUMBER.clone(),
                                },
                            ])
                            .into(),
                        )
                        .into(),
                        unique_constraint_for(Object::SUCCESSOR_OF, 1),
                    ])
                    .into(),
                )
                .into(),
            )
            .into(),
        ),
        stmt_to_prop(
            Object::AXIOMATIC,
            Object::AXIOMATIC,
            AXIOMATIC_AXIOMATIC_CONSTRAINT.clone(),
        ),
        stmt_to_prop(
            Object::COMPUTED,
            Object::AXIOMATIC,
            unique_constraint_for(Object::COMPUTED, 0),
        ),
        stmt_to_prop(
            Object::STATEMENT_SUBJECT,
            Object::AXIOMATIC,
            unique_constraint_for(Object::STATEMENT_SUBJECT, 0),
        ),
        stmt_to_prop(
            Object::STATEMENT_TAG,
            Object::AXIOMATIC,
            unique_constraint_for(Object::STATEMENT_TAG, 0),
        ),
        stmt_to_prop(
            Object::STATEMENT_VALUE,
            Object::AXIOMATIC,
            unique_constraint_for(Object::STATEMENT_VALUE, 0),
        ),
        // TODO: statement, knowledge, tag

        // --------------------- NODES ---------------------
        stmt_to_prop(
            Object::NODE_LITERAL,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_LITERAL, 0),
        ),
        stmt_to_prop(
            Object::NODE_COUNT,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_COUNT, 0),
        ),
        stmt_to_prop(
            Object::NODE_PARAMETER,
            Object::AXIOMATIC,
            Structure::new_computed(
                Structure::new_node_and([
                    common_unique_constraint_expression(Object::NODE_PARAMETER, 0),
                    // maybe hard code "parameter == zero or has succ"
                    Structure::new_node_exists(
                        Structure::new(&mut [
                            Property {
                                tag: Object::STATEMENT_SUBJECT,
                                value: Structure::new_node_parameter(0).into(),
                            },
                            Property {
                                tag: Object::STATEMENT_TAG,
                                value: IS_NATURAL_NUMBER.clone(),
                            },
                        ])
                        .into(),
                    )
                    .into(),
                ])
                .into(),
            )
            .into(),
        ),
        stmt_to_prop(
            Object::NODE_AND,
            Object::AXIOMATIC,
            more_than_1_constraint_for(Object::NODE_AND, 0),
        ),
        stmt_to_prop(
            Object::NODE_OR,
            Object::AXIOMATIC,
            more_than_1_constraint_for(Object::NODE_OR, 0),
        ),
        stmt_to_prop(
            Object::NODE_XOR,
            Object::AXIOMATIC,
            more_than_1_constraint_for(Object::NODE_XOR, 0),
        ),
        stmt_to_prop(
            Object::NODE_EQUAL,
            Object::AXIOMATIC,
            more_than_1_constraint_for(Object::NODE_EQUAL, 0),
        ),
        stmt_to_prop(
            Object::NODE_EXISTS,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_EXISTS, 0),
        ),
        stmt_to_prop(
            Object::NODE_QUERY,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_QUERY, 0),
        ),
        stmt_to_prop(
            Object::NODE_NOT,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_NOT, 0),
        ),
        // TODO: node
    ])
});
