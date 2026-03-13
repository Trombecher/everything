use std::sync::LazyLock;

use everything_structures::{Object, Property, Structure};

use crate::objects::{self, ObjectExt, StructureExt};

fn stmt_to_prop(subject: Object, tag: Object, value: Object) -> Property {
    Property {
        tag: objects::CONTAINS,
        value: Structure::new(&mut [
            Property {
                tag: objects::STATEMENT_SUBJECT,
                value: subject,
            },
            Property {
                tag: objects::STATEMENT_TAG,
                value: tag,
            },
            Property {
                tag: objects::STATEMENT_VALUE,
                value: value,
            },
        ])
        .into(),
    }
}

pub static BASE: LazyLock<Structure> = LazyLock::new(|| {
    Structure::new(&mut [
        stmt_to_prop(
            objects::CONTAINS,
            objects::AXIOMATIC,
            Structure::new(&mut [Property {
                tag: objects::CONTAINS,
                value: Structure::EMPTY.into(),
            }])
            .into(),
        ),
        stmt_to_prop(
            objects::SUCCESSOR_OF,
            objects::AXIOMATIC,
            Structure::node_function(
                Structure::node_function(
                    Structure::node_and([
                        Structure::node_exists(
                            Structure::new(&mut [
                                Property {
                                    tag: objects::STATEMENT_SUBJECT,
                                    value: Structure::node_parameter(Object::natural_number(0))
                                        .into(),
                                },
                                Property {
                                    tag: objects::STATEMENT_TAG,
                                    value: objects::IS_NATURAL_NUMBER,
                                },
                            ])
                            .into(),
                        )
                        .into(),
                        Structure::node_equal([
                            Structure::node_count(
                                Structure::node_query(
                                    Structure::new(&mut [
                                        Property {
                                            tag: objects::STATEMENT_SUBJECT,
                                            value: Structure::node_parameter(
                                                Object::natural_number(1),
                                            )
                                            .into(),
                                        },
                                        Property {
                                            tag: objects::STATEMENT_TAG,
                                            value: objects::SUCCESSOR_OF,
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
    ])
});
