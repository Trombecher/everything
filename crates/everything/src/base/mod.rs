#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use everything_structures::{Abstract, Object, Structure};

use crate::ext::{AbstractExt, StructureExt};

fn common_unique_constraint_expression(tag: Object, parameter_depth: usize) -> Object {
    Structure::new_node_equal(
        Object::new_integer(1),
        Structure::new_node_count(
            Structure::new_node_query_values(
                Structure::new_node_parameter(parameter_depth).into(),
                tag,
            )
            .into(),
        )
        .into(),
    )
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

pub static AXIOMATIC_AXIOMATIC_CONSTRAINT: LazyLock<Object> =
    LazyLock::new(|| unique_constraint_for(Abstract::AXIOMATIC.into(), 0));

/// A function that computes whether the object (passed in as the
/// parameter) is a natural number.
pub static IS_NATURAL_NUMBER: LazyLock<Object> = LazyLock::new(|| {
    Structure::new_computed(
        Structure::new_node_or(
            Structure::new_node_equal(
                Structure::new_node_parameter(0).into(),
                Abstract::ZERO.into(),
            )
            .into(),
            Structure::new_node_query_values(
                Structure::new_node_parameter(0).into(),
                Abstract::SUCCESSOR_OF.into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
});

pub static BASE: LazyLock<Structure> = LazyLock::new(|| {
    Structure::new_set([
        Structure::new_statement(
            Abstract::CONTAINS.into(),
            Abstract::AXIOMATIC.into(),
            Structure::new_bool(true).into(),
        )
        .into(),
        Structure::new_statement(
            Abstract::SUCCESSOR_OF.into(),
            Abstract::AXIOMATIC.into(),
            Structure::new_computed(
                Structure::new_computed(
                    Structure::new_node_and(
                        Structure::new_node_query_values(
                            Structure::new_node_parameter(0).into(),
                            IS_NATURAL_NUMBER.clone(),
                        )
                        .into(),
                        common_unique_constraint_expression(Abstract::SUCCESSOR_OF.into(), 1),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
        Structure::new_statement(
            Abstract::AXIOMATIC.into(),
            Abstract::AXIOMATIC.into(),
            AXIOMATIC_AXIOMATIC_CONSTRAINT.clone(),
        )
        .into(),
        Structure::new_statement(
            Abstract::COMPUTED.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::COMPUTED.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::STATEMENT_SUBJECT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_SUBJECT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::STATEMENT_TAG.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_TAG.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::STATEMENT_VALUE.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::STATEMENT_VALUE.into(), 0),
        )
        .into(),
        // TODO: knowledge

        // --------------------- NODES ---------------------
        Structure::new_statement(
            Abstract::NODE_LITERAL.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_LITERAL.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_COUNT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_COUNT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_PARAMETER.into(),
            Abstract::AXIOMATIC.into(),
            Structure::new_computed(
                Structure::new_computed(
                    Structure::new_node_and(
                        common_unique_constraint_expression(Abstract::NODE_PARAMETER.into(), 1),
                        // maybe hard code "parameter == zero or has succ"
                        Structure::new_node_query_values(
                            Structure::new_node_parameter(0).into(),
                            IS_NATURAL_NUMBER.clone(),
                        )
                        .into(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_AND_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_AND_LEFT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_AND_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_AND_RIGHT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_OR_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_OR_LEFT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_OR_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_OR_RIGHT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_XOR_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_XOR_LEFT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_XOR_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_XOR_RIGHT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_EQUAL_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EQUAL_LEFT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_EQUAL_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EQUAL_RIGHT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_QUERY.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_QUERY.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_NOT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_NOT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_ADD_LEFT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_ADD_LEFT.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_ADD_RIGHT.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_ADD_RIGHT.into(), 0),
        )
        .into(),
    ])
});
