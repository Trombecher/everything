use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn intersection() {
    let result = evaluate(
        &Node::Intersection {
            first: Node::Literal(Object::Set(integer_set())).into(),
            second: Node::Literal(Object::Set(Set::from([integer(2), integer(3), integer(4)])))
                .into(),
        },
        &BASE,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&BASE), 2);
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(2), integer(3)]
    );
    assert!(result.has(&integer(2), &BASE));
    assert!(!result.has(&integer(4), &BASE));
}

#[test]
fn intersection_of_deferred_sets() {
    let union = Node::Union {
        first: Node::Literal(Object::Set(Set::from([integer(1), integer(2)]))).into(),
        second: Node::Literal(Object::Set(Set::from([integer(3)]))).into(),
    };

    let result = evaluate(
        &Node::Intersection {
            first: union.into(),
            second: Node::Literal(Object::Set(Set::from([integer(2), integer(3)]))).into(),
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
fn intersection_of_disjoint_sets_is_empty() {
    let result = evaluate(
        &Node::Intersection {
            first: Node::Literal(Object::Set(Set::from([integer(1)]))).into(),
            second: Node::Literal(Object::Set(Set::from([integer(2)]))).into(),
        },
        &BASE,
    )
    .into_object(&BASE);

    assert_eq!(result, Object::Set(Set::empty()));
}

#[test]
fn intersection_requires_sets() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
    ];

    for node in invalid {
        assert_eq!(
            evaluate(
                &Node::Intersection {
                    first: node.into(),
                    second: empty_set.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }
}
