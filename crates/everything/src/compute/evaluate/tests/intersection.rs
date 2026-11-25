use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn intersection() {
    let result = evaluate(
        &Node::Intersection {
            first: Node::Literal(Object::Composite(integer_composite())).into(),
            second: Node::Literal(Object::Composite(Composite::from([
                integer(2),
                integer(3),
                integer(4),
            ])))
            .into(),
        },
        &BASE,
    );

    assert!(result.is_composite());
    assert_eq!(result.size(&BASE), 2);
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(2), integer(3)]
    );
    assert!(result.has(&integer(2), &BASE));
    assert!(!result.has(&integer(4), &BASE));
}

#[test]
fn intersection_of_deferred_composites() {
    let union = Node::Union {
        first: Node::Literal(Object::Composite(Composite::from([integer(1), integer(2)]))).into(),
        second: Node::Literal(Object::Composite(Composite::from([integer(3)]))).into(),
    };

    let result = evaluate(
        &Node::Intersection {
            first: union.into(),
            second: Node::Literal(Object::Composite(Composite::from([integer(2), integer(3)])))
                .into(),
        },
        &BASE,
    );

    assert_eq!(result.size(&BASE), 2);
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(2), integer(3)]
    );
}

#[test]
fn intersection_of_disjoint_composites_is_empty() {
    let result = evaluate(
        &Node::Intersection {
            first: Node::Literal(Object::Composite(Composite::from([integer(1)]))).into(),
            second: Node::Literal(Object::Composite(Composite::from([integer(2)]))).into(),
        },
        &BASE,
    )
    .into_object(&BASE);

    assert_eq!(result, Object::Composite(Composite::empty()));
}

#[test]
fn intersection_requires_composites() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let invalid = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
    ];

    for node in invalid {
        assert_eq!(
            evaluate(
                &Node::Intersection {
                    first: node.into(),
                    second: empty_composite.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }
}
