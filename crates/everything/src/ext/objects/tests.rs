use everything_structures::{Abstract, Object, Property, Structure};

use crate::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt, PropertyExt, StructureExt},
    nodes::{BinaryNode, Node},
};

#[test]
fn natural_number() {
    assert_eq!(Object::new_integer(0), Object::Abstract(Abstract::ZERO));

    assert_eq!(
        Object::new_integer(2),
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

/*
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
 */

#[test]
fn call() {
    let f: Object = Structure::new_node(Node::Function(
        Structure::new_node(Node::Parameter(0)).into(),
    ))
    .into();

    assert_eq!(
        f.call(
            &BASE,
            &[Object::Abstract(Abstract::ZERO)],
            &mut Default::default()
        )
        .into_object(),
        Object::Abstract(Abstract::ZERO)
    );
}

#[test]
fn eval_and() {
    assert!(
        !Object::Structure(Structure::new_node(Node::And(BinaryNode {
            left: Structure::new_bool(false).into(),
            right: Structure::new_bool(true).into()
        })))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    );

    assert!(
        Object::Structure(Structure::new_node(Node::And(BinaryNode {
            left: Structure::new_bool(true).into(),
            right: Structure::new_bool(true).into()
        })))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    )
}

#[test]
fn eval_or() {
    assert!(
        !Object::Structure(Structure::new_node(Node::Or(BinaryNode {
            left: Structure::new_bool(false).into(),
            right: Structure::new_bool(false).into()
        })))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    );

    assert!(
        Object::Structure(Structure::new_node(Node::Or(BinaryNode {
            left: Structure::new_bool(true).into(),
            right: Structure::new_bool(false).into()
        })))
        .eval(&BASE, &mut Default::default())
        .is_truthy(&BASE)
    )
}

#[test]
fn eval_literal() {
    assert_eq!(
        Object::Structure(Structure::new_node(Node::Literal(Abstract::ZERO.into())))
            .eval(&BASE, &mut Default::default())
            .into_object(),
        Object::Abstract(Abstract::ZERO)
    );
}

#[test]
fn eval_count() {
    assert_eq!(
        Object::Structure(Structure::new_node(Node::Count(Structure::Empty.into())))
            .eval(&BASE, &mut Default::default())
            .into_object(),
        Object::new_integer(0)
    );

    assert_eq!(
        Object::Structure(Structure::new_node(Node::Count(
            Structure::new_node(Node::Literal(
                Structure::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::KNOWLEDGE.into()),
                ])
                .into()
            ))
            .into()
        )))
        .eval(&BASE, &mut Default::default())
        .into_object(),
        Object::new_integer(2)
    );
}

#[test]
fn eval_query() {
    assert_eq!(
        Object::Structure(Structure::new_node_query_values(
            Structure::new(&mut [
                Property::new_contains(Abstract::ZERO.into()),
                Property::new_contains(Abstract::KNOWLEDGE.into()),
                Property::new_successor_of(Object::new_integer(0)),
            ])
            .into(),
            Structure::new_node(Node::Literal(Abstract::CONTAINS.into())).into()
        ))
        .eval(&BASE, &mut Default::default())
        .into_object(),
        Structure::new_set([Abstract::KNOWLEDGE.into(), Object::Abstract(Abstract::ZERO)]).into(),
    );
}

#[test]
fn eval_set_items() {
    let f: Object = Structure::new_node(Node::Function(
        Structure::new_node(Node::Function(
            Structure::new_set([
                Structure::new_node(Node::Parameter(0)).into(),
                Structure::new_node(Node::Parameter(1)).into(),
            ])
            .into(),
        ))
        .into(),
    ))
    .into();

    assert_eq!(
        f.call(
            &BASE,
            &[
                Object::Abstract(Abstract(1337)),
                Object::Abstract(Abstract(1338))
            ],
            &mut EvaluationContext::default(),
        )
        .into_object(),
        Structure::new_set([
            Object::Abstract(Abstract(1337)),
            Object::Abstract(Abstract(1338))
        ])
        .into()
    );
}
