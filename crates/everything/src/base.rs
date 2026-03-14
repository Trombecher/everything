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

pub static AXIOMATIC_AXIOMATIC_CONSTRAINT: LazyLock<Object> = LazyLock::new(|| {
    Structure::new_node_function(
        Structure::new_node_equal([
            Structure::new_node_count(
                Structure::new_node_query(
                    Structure::new(&mut [
                        Property {
                            tag: Object::STATEMENT_SUBJECT,
                            value: Structure::new_node_parameter(Object::ZERO).into(),
                        },
                        Property {
                            tag: Object::STATEMENT_TAG,
                            value: Object::AXIOMATIC,
                        },
                    ])
                    .into(),
                )
                .into(),
            )
            .into(),
            Object::natural_number(1),
        ])
        .into(),
    )
    .into()
});

fn unique_constraint_for(tag: Object) -> Object {
    Structure::new_node_function(
        Structure::new_node_equal([
            Object::natural_number(1),
            Structure::new_node_count(
                Structure::new_node_query(
                    Structure::new(&mut [
                        Property {
                            tag: Object::STATEMENT_SUBJECT,
                            value: Structure::new_node_parameter(Object::ZERO).into(),
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
            Structure::new_node_function(
                Structure::new_node_function(
                    Structure::new_node_and([
                        Structure::new_node_exists(
                            Structure::new(&mut [
                                Property {
                                    tag: Object::STATEMENT_SUBJECT,
                                    value: Structure::new_node_parameter(Object::natural_number(0))
                                        .into(),
                                },
                                Property {
                                    tag: Object::STATEMENT_TAG,
                                    value: Object::IS_NATURAL_NUMBER,
                                },
                            ])
                            .into(),
                        )
                        .into(),
                        Structure::new_node_equal([
                            Structure::new_node_count(
                                Structure::new_node_query(
                                    Structure::new(&mut [
                                        Property {
                                            tag: Object::STATEMENT_SUBJECT,
                                            value: Structure::new_node_parameter(
                                                Object::natural_number(1),
                                            )
                                            .into(),
                                        },
                                        Property {
                                            tag: Object::STATEMENT_TAG,
                                            value: Object::SUCCESSOR_OF,
                                        },
                                    ])
                                    .into(),
                                )
                                .into(),
                            )
                            .into(),
                            Object::natural_number(1),
                        ])
                        .into(),
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
            Object::NODE_FUNCTION_BODY,
            Object::AXIOMATIC,
            unique_constraint_for(Object::NODE_FUNCTION_BODY),
        ),
    ])
});
