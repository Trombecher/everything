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
                value: value,
            },
        ])
        .into(),
    }
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
    Structure::new_computed(
        Structure::new_node_equal([
            Object::natural_number(1),
            Structure::new_node_count(
                Structure::new_node_query(
                    Structure::new(&mut [
                        Property {
                            tag: Object::STATEMENT_SUBJECT,
                            value: Structure::new_node_parameter(Object::natural_number(
                                parameter_depth,
                            ))
                            .into(),
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
        .into(),
    )
    .into()
}

pub static AXIOMATIC_AXIOMATIC_CONSTRAINT: LazyLock<Object> =
    LazyLock::new(|| unique_constraint_for(Object::AXIOMATIC, 1));

pub static IS_NATURAL_NUMBER: LazyLock<Object> = LazyLock::new(|| {
    Structure::new_computed(
        Structure::new_node_or([
            Structure::new_node_equal([
                Structure::new_node_parameter(Object::ZERO).into(),
                Object::ZERO,
            ])
            .into(),
            Structure::new_node_exists(
                Structure::new(&mut [
                    Property {
                        tag: Object::STATEMENT_SUBJECT,
                        value: Structure::new_node_parameter(Object::ZERO).into(),
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
                                    value: Structure::new_node_parameter(Object::ZERO).into(),
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
            unique_constraint_for(Object::COMPUTED, 1),
        ),
        stmt_to_prop(
            Object::STATEMENT_SUBJECT,
            Object::AXIOMATIC,
            unique_constraint_for(Object::STATEMENT, 1),
        ),
        stmt_to_prop(
            Object::STATEMENT_TAG,
            Object::AXIOMATIC,
            unique_constraint_for(Object::STATEMENT_TAG, 1),
        ),
        stmt_to_prop(
            Object::STATEMENT_VALUE,
            Object::AXIOMATIC,
            unique_constraint_for(Object::STATEMENT_VALUE, 1),
        ),
        stmt_to_prop(
            Object::NODE_COUNT,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_COUNT, 1),
        ),
    ])
});
