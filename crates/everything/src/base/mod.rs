#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use everything_structures::{Abstract, Object, Property, Structure};

use crate::ext::{AbstractExt, PropertyExt, StructureExt};

fn common_unique_constraint_expression(tag: Object, parameter_depth: usize) -> Object {
    Structure::new_node_equal([
        Object::new_natural_number(1),
        Structure::new_node_count(
            Structure::new_node_query_values(
                Structure::new_node_parameter(parameter_depth).into(),
                tag,
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
    LazyLock::new(|| unique_constraint_for(Abstract::AXIOMATIC.into(), 0));

pub static IS_NATURAL_NUMBER: LazyLock<Object> = LazyLock::new(|| {
    Structure::new_computed(
        Structure::new_node_or([
            Structure::new_node_equal([
                Structure::new_node_parameter(0).into(),
                Abstract::ZERO.into(),
            ])
            .into(),
            Structure::new_node_exists(
                Structure::new(&mut [
                    Property::new_statement_subject(Structure::new_node_parameter(0).into()),
                    Property::new_statement_tag(Abstract::SUCCESSOR_OF.into()),
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
                    Structure::new_node_and([
                        Structure::new_node_exists(
                            Structure::new(&mut [
                                Property::new_statement_subject(
                                    Structure::new_node_parameter(0).into(),
                                ),
                                Property::new_statement_tag(IS_NATURAL_NUMBER.clone()),
                            ])
                            .into(),
                        )
                        .into(),
                        common_unique_constraint_expression(Abstract::SUCCESSOR_OF.into(), 1),
                    ])
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
        // TODO: statement, knowledge, tag

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
                    Structure::new_node_and([
                        common_unique_constraint_expression(Abstract::NODE_PARAMETER.into(), 1),
                        // maybe hard code "parameter == zero or has succ"
                        Structure::new_node_exists(
                            Structure::new(&mut [
                                Property::new_statement_subject(
                                    Structure::new_node_parameter(0).into(),
                                ),
                                Property::new_statement_tag(IS_NATURAL_NUMBER.clone()),
                            ])
                            .into(),
                        )
                        .into(),
                    ])
                    .into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_AND.into(),
            Abstract::AXIOMATIC.into(),
            more_than_1_constraint_for(Abstract::NODE_AND.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_OR.into(),
            Abstract::AXIOMATIC.into(),
            more_than_1_constraint_for(Abstract::NODE_OR.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_XOR.into(),
            Abstract::AXIOMATIC.into(),
            more_than_1_constraint_for(Abstract::NODE_XOR.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_EQUAL.into(),
            Abstract::AXIOMATIC.into(),
            more_than_1_constraint_for(Abstract::NODE_EQUAL.into(), 0),
        )
        .into(),
        Structure::new_statement(
            Abstract::NODE_EXISTS.into(),
            Abstract::AXIOMATIC.into(),
            unique_constraint_for(Abstract::NODE_EXISTS.into(), 0),
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
