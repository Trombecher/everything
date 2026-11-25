use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

fn construct(constructor: Node, value: Node) -> Object {
    evaluate(
        &Node::NewConstruction {
            constructor: constructor.into(),
            value: value.into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

#[test]
fn new_construction() {
    assert_eq!(
        construct(Node::Literal(Atom(1).into()), Node::Literal(Atom(2).into())),
        Construction::new(Atom(1).into(), Atom(2).into()).into()
    );
}

#[test]
fn children_are_evaluated() {
    let constructor = Node::Add {
        first: Node::Literal(integer(1)).into(),
        second: Node::Literal(integer(2)).into(),
    };
    let value = Node::Multiply {
        first: Node::Literal(integer(2)).into(),
        second: Node::Literal(integer(3)).into(),
    };

    assert_eq!(
        construct(constructor, value),
        Construction::new(integer(3), integer(6)).into()
    );
}

#[test]
fn nested_constructions() {
    let inner = Node::NewConstruction {
        constructor: Node::Literal(Atom(1).into()).into(),
        value: Node::Literal(Atom(2).into()).into(),
    };

    assert_eq!(
        construct(Node::Literal(Atom(3).into()), inner),
        Construction::new(
            Atom(3).into(),
            Construction::new(Atom(1).into(), Atom(2).into()).into()
        )
        .into()
    );
}

#[test]
fn undefined_is_not_special() {
    assert_eq!(
        construct(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(integer(1)),
        ),
        Construction::new(Atom::UNDEFINED.into(), integer(1)).into()
    );
    assert_eq!(
        construct(
            Node::Literal(Atom(1).into()),
            Node::Literal(Atom::UNDEFINED.into()),
        ),
        Construction::new(Atom(1).into(), Atom::UNDEFINED.into()).into()
    );
}

#[test]
fn deferred_children_are_materialized() {
    let union = Node::Union {
        first: Node::Literal(Object::Composite(Composite::from([integer(1)]))).into(),
        second: Node::Literal(Object::Composite(Composite::from([integer(2)]))).into(),
    };

    assert_eq!(
        construct(Node::Literal(Atom(1).into()), union),
        Construction::new(
            Atom(1).into(),
            Object::Composite(Composite::from([integer(1), integer(2)]))
        )
        .into()
    );
}
