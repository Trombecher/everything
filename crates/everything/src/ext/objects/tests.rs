use everything_structures::{Abstract, Object, Property, Structure};

use crate::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, NodeType, ObjectExt, PropertyExt, StructureExt},
};

#[test]
fn natural_number() {
    assert_eq!(
        Object::new_natural_number(0),
        Object::Abstract(Abstract::ZERO)
    );

    assert_eq!(
        Object::new_natural_number(2),
        Structure::new(&mut [Property {
            tag: Object::Abstract(Abstract::SUCCESSOR_OF),
            value: Structure::new(&mut [Property {
                tag: Object::Abstract(Abstract::SUCCESSOR_OF),
                value: Object::Abstract(Abstract::ZERO)
            }])
            .into()
        }])
        .into()
    )
}

#[test]
fn node_type() {
    let knowledge = &BASE;

    // None
    assert_eq!(
        Object::Structure(Structure::Empty).node_type(knowledge),
        None
    );

    // Single
    assert_eq!(
        Object::Structure(Structure::new_computed(Object::Abstract(Abstract::ZERO)))
            .node_type(knowledge),
        Some(NodeType::Computed)
    );

    // Multiple
    assert_eq!(
        Object::Structure(Structure::new_node_and([
            Object::Abstract(Abstract::ZERO),
            Object::Abstract(Abstract::KNOWLEDGE)
        ]))
        .node_type(knowledge),
        Some(NodeType::And)
    );
}

#[test]
fn call() {
    let f: Object = Structure::new_computed(Structure::new_node_parameter(0).into()).into();

    assert_eq!(
        f.call(
            &BASE,
            &[Object::Abstract(Abstract::ZERO)],
            &mut Default::default()
        ),
        Object::Abstract(Abstract::ZERO)
    );
}

#[test]
fn eval_and() {
    assert!(
        !Object::Structure(Structure::new_node_and([
            Structure::new_bool(false).into(),
            Structure::new_bool(true).into()
        ]))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    );

    assert!(
        Object::Structure(Structure::new_node_and([
            Structure::new_bool(true).into(),
            Structure::new_bool(true).into()
        ]))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    )
}

#[test]
fn eval_or() {
    assert!(
        !Object::Structure(Structure::new_node_or([
            Structure::new_bool(false).into(),
            Structure::new_bool(false).into()
        ]))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    );

    assert!(
        Object::Structure(Structure::new_node_or([
            Structure::new_bool(true).into(),
            Structure::new_bool(false).into()
        ]))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    )
}

#[test]
fn eval_literal() {
    assert_eq!(
        Object::Structure(Structure::new_node_literal(Object::Abstract(
            Abstract::ZERO
        )))
        .eval(&BASE, &mut Default::default()),
        Object::Abstract(Abstract::ZERO)
    );
}

#[test]
fn eval_count() {
    assert_eq!(
        Object::Structure(Structure::new_node_count(Structure::Empty.into()))
            .eval(&BASE, &mut Default::default()),
        Object::new_natural_number(0)
    );

    assert_eq!(
        Object::Structure(Structure::new_node_count(
            Structure::new_node_literal(
                Structure::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::KNOWLEDGE.into()),
                ])
                .into()
            )
            .into()
        ))
        .eval(&BASE, &mut Default::default()),
        Object::new_natural_number(2)
    );
}

#[test]
fn eval_query() {
    assert_eq!(
        Object::Structure(Structure::new_node_query_values(
            Structure::new(&mut [
                Property::new_contains(Abstract::ZERO.into()),
                Property::new_contains(Abstract::KNOWLEDGE.into()),
                Property::successor_of(0),
            ])
            .into(),
            Structure::new_node_literal(Abstract::CONTAINS.into()).into()
        ))
        .eval(&BASE, &mut Default::default()),
        Structure::new_set([Abstract::KNOWLEDGE.into(), Object::Abstract(Abstract::ZERO)]).into(),
    );
}

#[test]
fn eval_set_items() {
    let f: Object = Structure::new_computed(
        Structure::new_computed(
            Structure::new_set([
                Structure::new_node_parameter(0).into(),
                Structure::new_node_parameter(1).into(),
            ])
            .into(),
        )
        .into(),
    )
    .into();

    assert_eq!(
        f.call(
            &BASE,
            &[
                Object::Abstract(Abstract(1337)),
                Object::Abstract(Abstract(1338))
            ],
            &mut EvaluationContext::default(),
        ),
        Structure::new_set([
            Object::Abstract(Abstract(1337)),
            Object::Abstract(Abstract(1338))
        ])
        .into()
    );
}
